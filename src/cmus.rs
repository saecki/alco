use anyhow::bail;
use shellexpand::tilde;
use yaml_rust::YamlLoader;

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

const CMUS_AUTOSAVE_FILE: &str = "~/.config/cmus/autosave";

pub fn reload_cmus(selector: impl AsRef<Path>, scheme_file: impl AsRef<str>) -> anyhow::Result<()> {
    let selector_str = fs::read_to_string(selector.as_ref())?;
    let selector = YamlLoader::load_from_str(&selector_str)?.remove(0);

    match super::selector(&selector, scheme_file.as_ref()) {
        Some(s) => {
            Command::new("cmus-remote")
                .arg("-C")
                .arg(&format!("colorscheme {}", s))
                .output()?;

            write_autosave(s)?;

            Ok(())
        }
        None => bail!("Error parsing selector.yml"),
    }
}

fn write_autosave(cmus_scheme: impl AsRef<str>) -> anyhow::Result<()> {
    let autosave_path = tilde(CMUS_AUTOSAVE_FILE);
    let original_autosave_str = fs::read_to_string(autosave_path.as_ref())?;
    let new_scheme_str: String;

    let local_path =
        PathBuf::from(tilde(&format!("~/.config/cmus/{}.theme", cmus_scheme.as_ref())).to_string());
    let global_path = PathBuf::from(format!("/usr/share/cmus/{}.theme", cmus_scheme.as_ref()));
    if local_path.exists() {
        new_scheme_str = fs::read_to_string(local_path)?;
    } else if global_path.exists() {
        new_scheme_str = fs::read_to_string(global_path)?;
    } else {
        bail!("Cmus colorscheme not found");
    }

    let new_scheme: Vec<_> = new_scheme_str
        .lines()
        .filter(|&s| s.starts_with("set color_"))
        .collect();

    let mut new_autosave_str = String::with_capacity(original_autosave_str.capacity());
    for l in original_autosave_str.lines() {
        if l.starts_with("set color_") {
            println!("line: {}", l);
            if let Some(var) = l.split('=').next() {
                println!("var: {}", var);
                let new = new_scheme
                    .iter()
                    .find(|&s| s.starts_with(var))
                    .and_then(|s| s.split('=').skip(1).next());
                println!("new: {:?}", new);

                let new_val = match new {
                    Some(val) => val.trim(),
                    None => "default",
                };
                println!("new_val: {:?}", new_val);

                new_autosave_str.push_str(var);
                new_autosave_str.push('=');
                new_autosave_str.push_str(new_val);
                new_autosave_str.push('\n');
            }
        } else {
            new_autosave_str.push_str(l);
            new_autosave_str.push('\n');
        }
    }

    fs::write(autosave_path.as_ref(), new_autosave_str)?;

    Ok(())
}
