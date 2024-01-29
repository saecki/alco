use anyhow::anyhow;
use anyhow::bail;

use yaml_rust::YamlLoader;

use std::fs;
use std::path::Path;

pub fn reload_bat(
    config_file: impl AsRef<Path>,
    in_file: impl AsRef<Path>,
    selector: impl AsRef<Path>,
    colorscheme: impl AsRef<str>,
) -> anyhow::Result<()> {
    let selector_str =
        fs::read_to_string(selector.as_ref()).map_err(|_| anyhow!("Error reading bat selector"))?;
    let selector = YamlLoader::load_from_str(&selector_str)?.remove(0);

    match super::selector(&selector, colorscheme.as_ref()) {
        Some(s) => {
            write_config(config_file, in_file, s)?;

            Ok(())
        }
        None => bail!("Missing mapping in bat selector"),
    }
}

fn write_config(
    config_file: impl AsRef<Path>,
    in_file: impl AsRef<Path>,
    colorscheme: impl AsRef<str>,
) -> anyhow::Result<()> {
    let input_str = fs::read_to_string(in_file.as_ref()).map_err(|_| anyhow!("Bat input file not found"))?;

    let new_config = input_str.replace("<theme>", colorscheme.as_ref());

    fs::write(config_file.as_ref(), new_config)?;

    Ok(())
}
