use std::fs;
use std::path::Path;

use anyhow::{anyhow, bail};
use shellexpand::tilde;
use yaml_rust::parser::{MarkedEventReceiver, Parser};
use yaml_rust::scanner::Marker;
use yaml_rust::{Event, Yaml, YamlLoader};

struct ColorEventReceiver<T> {
    listener: T,
}

impl<T: FnMut(Event, Marker)> ColorEventReceiver<T> {
    fn new(listener: T) -> Self {
        Self { listener }
    }
}

impl<T: FnMut(Event, Marker)> MarkedEventReceiver for ColorEventReceiver<T> {
    fn on_event(&mut self, event: Event, marker: Marker) {
        (self.listener)(event, marker);
    }
}

pub fn reload_alacritty(
    config_file: impl AsRef<Path>,
    selector: impl AsRef<Path>,
    colorscheme: impl AsRef<str>,
) -> anyhow::Result<()> {
    let selector_str = fs::read_to_string(selector.as_ref())
        .map_err(|_| anyhow!("Error reading alacritty selector"))?;
    let selector = YamlLoader::load_from_str(&selector_str)?.remove(0);

    match super::selector(&selector, colorscheme.as_ref()) {
        Some(s) => {
            apply(config_file, tilde(s).as_ref())?;
            Ok(())
        }
        None => bail!("Missing mapping in alacritty selector"),
    }
}

fn apply(config_file: impl AsRef<Path>, scheme_file: impl AsRef<str>) -> anyhow::Result<()> {
    let new_colors = parse_colors(scheme_file.as_ref())
        .map_err(|_| anyhow!("Error reading alacritty colorscheme file"))?;

    let config_str = fs::read_to_string(config_file.as_ref())?;
    let config_lines = config_str.lines().collect::<Vec<_>>();
    let mut new_config_str = String::new();
    let mut line_index = 0;

    let mut current_path: Vec<String> = Vec::new();
    let mut last_line = 0;
    let mut last_col = 0;

    let mut parser = Parser::new(config_str.chars());
    let mut receiver = ColorEventReceiver::new(|event, mark| {
        if let Event::Scalar(name, _, _, _) = event {
            if mark.line() != last_line {
                if mark.col() == last_col {
                    current_path.pop();
                    current_path.push(name);
                    last_line = mark.line();
                    last_col = mark.col();
                } else if mark.col() == last_col + 2 {
                    current_path.push(name);
                    last_line = mark.line();
                    last_col = mark.col();
                } else if mark.col() < last_col {
                    let indent = mark.col() / 2;
                    for _ in indent..current_path.len() {
                        current_path.pop();
                    }
                    current_path.push(name);
                    last_line = mark.line();
                    last_col = mark.col();
                }
            } else if let Some(v) = value(&new_colors, &current_path) {
                if let Some(stringified) = stringify(v) {
                    for i in line_index..mark.line() - 1 {
                        new_config_str.push_str(config_lines[i]);
                        new_config_str.push('\n');
                    }
                    new_config_str.push_str(&config_lines[mark.line() - 1][0..mark.col()]);
                    new_config_str.push_str(&stringified);
                    new_config_str.push('\n');
                    line_index = mark.line();
                }
            }
        }
    });
    parser.load(&mut receiver, true)?;

    for i in line_index..config_lines.len() {
        new_config_str.push_str(config_lines[i]);
        new_config_str.push('\n');
    }

    fs::write(config_file, new_config_str)?;

    Ok(())
}

fn parse_colors(file: impl AsRef<Path>) -> anyhow::Result<Yaml> {
    let config_str = fs::read_to_string(file)?;
    let config = YamlLoader::load_from_str(&config_str)?;

    if let Some(c) = config.into_iter().next() {
        return Ok(c);
    }

    bail!("Error parsing colors")
}

fn value<'a>(yaml: &'a Yaml, path: &[String]) -> Option<&'a Yaml> {
    let mut current = yaml;

    for key in path {
        if let Yaml::Hash(h) = current {
            let value = h.iter().find(|(k, _)| match k {
                Yaml::String(s) => s == key,
                _ => false,
            });

            current = value?.1;
        }
    }

    Some(current)
}

fn stringify(value: &Yaml) -> Option<String> {
    match value {
        Yaml::String(s) => Some(format!("'{}'", s)),
        Yaml::Integer(i) => Some(i.to_string()),
        Yaml::Boolean(b) => Some(b.to_string()),
        _ => None,
    }
}
