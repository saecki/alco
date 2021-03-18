use anyhow::bail;
use shellexpand::tilde;
use yaml_rust::{Yaml, YamlLoader};

use std::fs;
use std::path::Path;
use std::process::Command;

pub async fn reload_tmux(
    tmux_file: impl AsRef<Path>,
    selector: impl AsRef<Path>,
    scheme_file: impl AsRef<str>,
) -> anyhow::Result<()> {
    let selector_str = fs::read_to_string(selector.as_ref())?;
    let selector = YamlLoader::load_from_str(&selector_str)?.remove(0);

    match mapping(&selector, scheme_file.as_ref()) {
        Some(s) => {
            fs::copy(tilde(s).as_ref(), tmux_file.as_ref())?;
            Command::new("tmux")
                .arg("source-file")
                .arg(tmux_file.as_ref())
                .output()?;

            Ok(())
        }
        None => bail!("Error parsing selector.yml"),
    }
}

fn mapping<'a>(selector: &'a Yaml, key: &'_ str) -> Option<&'a str> {
    let map = selector.as_hash()?;
    let mut default = None;

    for (k, v) in map.iter() {
        if k.as_str()? == key {
            return v.as_str();
        } else if k.as_str()? == "else" {
            default = v.as_str();
        }
    }

    default
}
