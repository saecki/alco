use clap::{crate_version, App, Arg, ValueHint};
use std::path::Path;
use std::time::Duration;

use alacritty_colorscheme as lib;

fn main() {
    let base_dirs = directories_next::BaseDirs::new().unwrap();
    let alacritty_dir = base_dirs.config_dir().join("alacritty");

    let default_config_file = alacritty_dir.join("alacritty.yml");
    let default_colorscheme_dir = alacritty_dir.join("colors");

    let mut app = App::new("alacritty colorscheme")
        .version(crate_version!())
        .bin_name("alco")
        .arg(
            Arg::new("configuration file")
                .long("config_file")
                .short('c')
                .default_value(default_config_file.to_str().unwrap())
                .value_name("file")
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("colorscheme directory")
                .long("colorscheme_dir")
                .short('C')
                .default_value(default_colorscheme_dir.to_str().unwrap())
                .value_name("directory")
                .value_hint(ValueHint::DirPath),
        )
        .subcommands(vec![
            App::new("apply").arg(
                Arg::new("colorscheme")
                    .index(1)
                    .value_name("schemefile")
                    .required(true),
            ),
            App::new("toggle").arg(
                Arg::new("reverse")
                    .long("reverse")
                    .short('r')
                    .takes_value(false),
            ),
            App::new("list"),
            App::new("status").arg(
                Arg::new("time")
                    .long("time")
                    .short('t')
                    .about("print the duration since the last change")
                    .takes_value(false),
            ),
        ]);

    let app_m = app.get_matches_mut();

    let config_file = app_m.value_of("configuration file").unwrap();
    let scheme_dir = app_m.value_of("colorscheme directory").unwrap();

    match app_m.subcommand() {
        Some(("apply", sub_m)) => {
            let scheme_file = sub_m.value_of("colorscheme").unwrap();
            apply(config_file, scheme_dir, scheme_file);
        }
        Some(("toggle", sub_m)) => {
            let reverse = sub_m.is_present("reverse");
            toggle(config_file, scheme_dir, reverse);
        }
        Some(("list", _)) => list(scheme_dir),
        Some(("status", sub_m)) => {
            let time = sub_m.is_present("time");
            status(scheme_dir, time);
        }
        _ => {
            app.print_help().ok();
        }
    }
}

fn apply(config_file: impl AsRef<Path>, scheme_dir: impl AsRef<Path>, scheme_file: &str) {
    if let Err(e) = lib::apply(config_file, scheme_dir, scheme_file) {
        println!("Error applying colorscheme {}:\n{:?}", scheme_file, e);
    }
}
fn toggle(config_file: impl AsRef<Path>, scheme_dir: impl AsRef<Path>, reverse: bool) {
    match lib::toggle(config_file, scheme_dir, reverse) {
        Ok(c) => println!("{}", c),
        Err(_) => println!("Error toggling colorscheme"),
    }
}

fn list(dir: impl AsRef<Path>) {
    match lib::list(dir.as_ref()) {
        Ok(files) => {
            for f in files {
                println!("{}", f);
            }
        }
        Err(_) => {
            println!("Error listing files in dir: {}", dir.as_ref().display());
            std::process::exit(1);
        }
    }
}

fn status(scheme_dir: impl AsRef<Path>, time: bool) {
    match lib::status(scheme_dir) {
        Ok(s) => {
            if time {
                let seconds = Duration::from_secs(s.duration.as_secs());
                println!(
                    "{} changed {} ago",
                    s.file_name,
                    humantime::format_duration(seconds),
                );
            } else {
                println!("{}", s.file_name);
            }
        }
        Err(e) => println!("Error getting current colorscheme:\n{}", e),
    }
}
