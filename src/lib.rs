use serde::{Deserialize, Serialize};
use yaml_rust::parser::{Event, MarkedEventReceiver, Parser};
use yaml_rust::scanner::Marker;
use yaml_rust::YamlLoader;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct Config {
    colors: Colors,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct Colors {
    cursor: HashMap<String, String>,
    primary: HashMap<String, String>,
    normal: HashMap<String, String>,
    bright: HashMap<String, String>,
    dim: HashMap<String, String>,
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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct Bounds {
    line: usize,
    column: usize,
}

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

pub fn apply(
    config_file: impl AsRef<Path>,
    scheme_dir: impl AsRef<Path>,
    scheme_file: &str,
) -> anyhow::Result<()> {
    let new_colors = parse_colors(scheme_dir.as_ref().join(scheme_file))?;

    let config_str = fs::read_to_string(config_file.as_ref())?;
    let mut current_path: Vec<String> = Vec::new();
    let mut last_line = 0;
    let mut last_col = 0;

    let mut parser = Parser::new(config_str.chars());
    let mut receiver = ColorEventReceiver::new(move |event, mark| {
        match event {
            Event::Scalar(name, ts, _, tt) => {
                if mark.line() != last_line {
                    println!("key{:?}{:?}#{}", ts, tt, name);
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
                } else {
                    println!("val{:?}{:?}#{}", ts, tt, name);
                }
            }
            _ => (),
        }
        println!("{:?}{:?}", mark, current_path);
    });
    parser.load(&mut receiver, true)?;

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

fn parse_colors(file: impl AsRef<Path>) -> anyhow::Result<()> {
    let config_str = fs::read_to_string(file)?;
    let config = YamlLoader::load_from_str(&config_str)?;

    println!("{:?}", config);

    Ok(())
}
