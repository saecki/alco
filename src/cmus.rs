use anyhow::bail;
use yaml_rust::YamlLoader;

use std::fs;
use std::path::Path;
use std::process::Command;

pub async fn reload_cmus(
    selector: impl AsRef<Path>,
    scheme_file: impl AsRef<str>,
) -> anyhow::Result<()> {
    let selector_str = fs::read_to_string(selector.as_ref())?;
    let selector = YamlLoader::load_from_str(&selector_str)?.remove(0);

    match super::selector(&selector, scheme_file.as_ref()) {
        Some(s) => {
            Command::new("cmus-remote")
                .arg("-C")
                .arg(&format!("colorscheme {}", s))
                .output()?;

            Ok(())
        }
        None => bail!("Error parsing selector.yml"),
    }
}
