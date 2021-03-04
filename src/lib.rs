use serde::{Deserialize, Serialize};
use yaml_rust::parser::{Event, MarkedEventReceiver, Parser};
use yaml_rust::scanner::Marker;

use std::fs;
use std::io;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct Config {
    colors: Colors,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct Colors {
    cursor: Option<Cursor>,
    primary: Option<Primary>,
    normal: Option<List>,
    bright: Option<List>,
    dim: Option<List>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct Cursor {
    text: Option<String>,
    cursor: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct Primary {
    foreground: Option<String>,
    background: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct List {
    black: Option<String>,
    red: Option<String>,
    green: Option<String>,
    yellow: Option<String>,
    blue: Option<String>,
    magenta: Option<String>,
    cyan: Option<String>,
    white: Option<String>,
}

struct ColorEventReceiver<T> {
    listener: T,
}

impl<T: Fn(Event, Marker)> ColorEventReceiver<T> {
    fn new(listener: T) -> Self {
        Self { listener }
    }
}

impl<T: Fn(Event, Marker)> MarkedEventReceiver for ColorEventReceiver<T> {
    fn on_event(&mut self, event: Event, marker: Marker) {
        (self.listener)(event, marker);
    }
}

pub fn apply(
    config_file: impl AsRef<Path>,
    scheme_dir: impl AsRef<Path>,
    scheme_file: &str,
) -> anyhow::Result<()> {
    let colors = parse_colors(scheme_dir.as_ref().join(scheme_file))?;

    let config_str = fs::read_to_string(config_file.as_ref())?;
    let new_config_str = config_str.clone();

    let mut parser = Parser::new(config_str.chars());
    let recv = |event, marker| match event {
        Event::MappingStart(anchor_id) => {}
        Event::MappingEnd => {}
        _ => (),
    };
    let mut receiver = ColorEventReceiver::new(recv);
    parser.load(&mut receiver, false)?;

    fs::write(config_file.as_ref(), new_config_str)?;

    Ok(())
}

pub fn toggle(config_file: impl AsRef<Path>, scheme_dir: impl AsRef<Path>, reverse: bool) {}

pub fn list(dir: impl AsRef<Path>) -> Result<Vec<String>, io::Error> {
    fs::read_dir(dir.as_ref()).map(|read_dir| {
        read_dir
            .filter_map(Result::ok)
            .filter(|d| d.metadata().map(|m| m.is_file()).unwrap_or(false))
            .filter_map(|d| d.file_name().to_str().map(str::to_owned))
            .collect()
    })
}

pub fn status(config_file: impl AsRef<Path>, scheme_dir: impl AsRef<Path>) {
    let options = list(scheme_dir);
}

fn parse_colors(file: impl AsRef<Path>) -> anyhow::Result<Colors> {
    let config_str = std::fs::read_to_string(file)?;
    let config: Config = serde_yaml::from_str(&config_str)?;

    Ok(config.colors)
}
