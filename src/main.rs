use clap::{crate_version, App, Arg, ValueHint};
use std::path::Path;

use alacritty_colorscheme::{apply, list, status, toggle};

fn main() {
    let base_dirs = directories_next::BaseDirs::new().unwrap();
    let alacritty_dir = base_dirs.config_dir().join("alacritty");

    let default_config_file = alacritty_dir.join("alacritty.yml");
    let default_colorscheme_dir = alacritty_dir.join("colors");

    let app = App::new("alacritty colorscheme")
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
            App::new("status"),
        ]);

    let app_m = app.get_matches();

    let config_file = app_m.value_of("configuration file").unwrap();
    let scheme_dir = app_m.value_of("colorscheme directory").unwrap();

    match app_m.subcommand() {
        Some(("apply", sub_m)) => {
            let scheme_file = sub_m.value_of("colorscheme").unwrap();
            _apply(config_file, scheme_dir, scheme_file);
        }
        Some(("toggle", sub_m)) => {
            let reverse = sub_m.is_present("reverse");
            _toggle(config_file, scheme_dir, reverse);
        }
        Some(("list", _sub_m)) => _list(scheme_dir),
        Some(("status", _sub_m)) => _status(config_file, scheme_dir),
        _ => unreachable!(),
    }
}

fn _apply(config_file: impl AsRef<Path>, scheme_dir: impl AsRef<Path>, scheme_file: &str) {
    if let Err(e) = apply(config_file, scheme_dir, scheme_file) {
        println!("Error applying colorscheme {}:\n{:?}", scheme_file, e);
    }
}
fn _toggle(config_file: impl AsRef<Path>, scheme_dir: impl AsRef<Path>, reverse: bool) {
    let _ = toggle(config_file, scheme_dir, reverse);
}

fn _list(dir: impl AsRef<Path>) {
    match list(dir.as_ref()) {
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

fn _status(config_file: impl AsRef<Path>, scheme_dir: impl AsRef<Path>) {
    let _ = status(config_file, scheme_dir);
}
