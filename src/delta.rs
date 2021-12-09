use anyhow::{anyhow, bail};
use shellexpand::tilde;
use yaml_rust::YamlLoader;

use std::fs;
use std::path::Path;

pub fn reload_delta(
    delta_file: impl AsRef<Path>,
    selector: impl AsRef<Path>,
    scheme_file: impl AsRef<str>,
) -> anyhow::Result<()> {
    let selector_str = fs::read_to_string(selector.as_ref())
        .map_err(|_| anyhow!("Error reading delta selector"))?;
    let selector = YamlLoader::load_from_str(&selector_str)?.remove(0);

    match super::selector(&selector, scheme_file.as_ref()) {
        Some(s) => {
            fs::copy(tilde(s).as_ref(), delta_file.as_ref())?;
            Ok(())
        }
        None => bail!("Missing mapping in delta selector"),
    }
}
