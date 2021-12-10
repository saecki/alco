use anyhow::{anyhow, bail};
use shellexpand::tilde;
use yaml_rust::YamlLoader;

use std::fs;
use std::path::Path;
use std::process::Command;

pub fn reload_kitty(
    kitty_file: impl AsRef<Path>,
    selector: impl AsRef<Path>,
    socket_file: impl AsRef<Path>,
    scheme_file: impl AsRef<str>,
) -> anyhow::Result<()> {
    let selector_str = fs::read_to_string(selector.as_ref())
        .map_err(|_| anyhow!("Error reading kitty selector"))?;
    let selector = YamlLoader::load_from_str(&selector_str)?.remove(0);

    match super::selector(&selector, scheme_file.as_ref()) {
        Some(s) => {
            fs::copy(tilde(s).as_ref(), kitty_file.as_ref())?;
            let unix_socket = format!("unix:{}", socket_file.as_ref().display());
            if Path::exists(socket_file.as_ref()) {
                Command::new("kitty")
                    .arg("@")
                    .arg("--to")
                    .arg(&unix_socket)
                    .arg("set-colors")
                    .arg("-a")
                    .arg(kitty_file.as_ref())
                    .output()?;
            }

            Ok(())
        }
        None => bail!("Missing mapping in kitty selector"),
    }
}
