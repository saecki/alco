use anyhow::anyhow;
use anyhow::bail;

use shellexpand::tilde;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn reload_starship(
    config_file: impl AsRef<Path>,
    in_file: impl AsRef<Path>,
    selector: impl AsRef<Path>,
    scheme_file: impl AsRef<str>,
) -> anyhow::Result<()> {
    let selector_str = fs::read_to_string(selector.as_ref())
        .map_err(|_| anyhow!("Error reading cmus selector"))?;
    let selector = YamlLoader::load_from_str(&selector_str)?.remove(0);

    match super::selector(&selector, scheme_file.as_ref()) {
        Some(s) => {
            Command::new("cmus-remote").arg("-C").arg(&format!("colorscheme {}", s)).output()?;

            write_config(config_file, in_file, s)?;

            Ok(())
        }
        None => bail!("Missing mapping in cmus selector"),
    }
}

fn write_config(
    config_file: impl AsRef<Path>,
    in_file: impl AsRef<Path>,
    scheme_file: impl AsRef<str>,
) -> anyhow::Result<()> {
    let scheme_str = fs::read_to_string(tilde(scheme_file.as_ref()).as_ref())
        .map_err(|_| anyhow!("Starship colorscheme file not found"))?;
    let input_str = fs::read_to_string(in_file.as_ref())
        .map_err(|_| anyhow!("Starship input file not found"))?;

    let colorscheme = parse_colorscheme(&scheme_str)?;

    let mut new_config = input_str;
    for (k, v) in colorscheme.iter() {
        new_config = new_config.replace(&format!("<{}>", k), v);
    }

    fs::write(config_file.as_ref(), new_config)?;

    Ok(())
}

fn parse_colorscheme(str: &str) -> anyhow::Result<HashMap<String, String>> {
    let yaml = YamlLoader::load_from_str(str)?;
    match yaml.into_iter().next() {
        Some(Yaml::Hash(h)) => {
            let mut map = HashMap::new();
            for e in h.into_iter() {
                if let (Yaml::String(k), Yaml::String(v)) = e {
                    map.insert(k, v);
                }
            }
            Ok(map)
        }
        _ => bail!("Error parsing starship colorscheme"),
    }
}
