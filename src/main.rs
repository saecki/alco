use async_std::task::{block_on, spawn};
use clap::{crate_authors, crate_version, App, Arg, ValueHint};
use clap_generate::generate;
use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
use shellexpand::tilde;

use std::path::Path;
use std::process::exit;
use std::time::Duration;

const BIN_NAME: &str = "alco";

const BASH: &str = "bash";
const ELVISH: &str = "elvish";
const FISH: &str = "fish";
const PWRSH: &str = "powershell";
const ZSH: &str = "zsh";

const DEFAULT_CONFIG_FILE: &str = "~/.config/alacritty/alacritty.yml";
const DEFAULT_COLORSCHEME_DIR: &str = "~/.config/alacritty/colors/";
const DEFAULT_NEOVIM_FILE: &str = "~/.config/nvim/colors.vim";
const DEFAULT_TMUX_FILE: &str = "~/.config/tmux/colors/current.conf";
const DEFAULT_TMUX_SELECTOR: &str = "~/.config/tmux/colors/selector.yml";

struct TmuxOptions {
    reload: bool,
    selector: String,
    file: String,
}

struct NeovimOptions {
    reload: bool,
    file: String,
}

fn main() {
    let mut app = App::new("alco")
        .bin_name(BIN_NAME)
        .version(crate_version!())
        .author(crate_authors!())
        .about("Update the colorscheme of alacritty.")
        .arg(
            Arg::new("configuration file")
                .long("config-file")
                .short('c')
                .default_value(DEFAULT_CONFIG_FILE)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .about("Alacritty's configuration file in which values are replaced"),
        )
        .arg(
            Arg::new("colorscheme directory")
                .long("colorscheme-dir")
                .short('C')
                .default_value(DEFAULT_COLORSCHEME_DIR)
                .value_name("directory")
                .value_hint(ValueHint::DirPath)
                .about("The direcotry that contains colorscheme configurations"),
        )
        .arg(
            Arg::new("reload neovim")
                .long("reload-neovim")
                .short('v')
                .takes_value(false)
                .about("Also reload neovim by sourcing a configuration file"),
        )
        .arg(
            Arg::new("neovim file")
                .long("neovim-file")
                .default_value(DEFAULT_NEOVIM_FILE)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .about("The neovim configuration file which will be sourced"),
        )
        .arg(
            Arg::new("reload tmux")
                .long("reload-tmux")
                .short('t')
                .takes_value(false)
                .about("Also reload tmux by sourcing a configuration file"),
        )
        .arg(
            Arg::new("tmux file")
                .long("tmux-file")
                .default_value(DEFAULT_TMUX_FILE)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .about("The tmux configuration file which will be overwritten and sourced"),
        )
        .arg(
            Arg::new("tmux selector")
                .long("tmux-selector")
                .default_value(DEFAULT_TMUX_SELECTOR)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .about("The tmux selector file which contains a coloscheme mapping"),
        )
        .arg(
            Arg::new("generate completion")
                .long("generate-completion")
                .short('g')
                .value_name("shell")
                .possible_values(&[BASH, ZSH, FISH, ELVISH, PWRSH])
                .about("Generates a completion script for the specified shell"),
        )
        .subcommands(vec![
            App::new("apply")
                .bin_name("alco-apply")
                .about("Apply a colorscheme")
                .arg(
                    Arg::new("colorscheme")
                        .index(1)
                        .value_name("schemefile")
                        .required(true),
                ),
            App::new("toggle")
                .bin_name("alco-toggle")
                .about("Toggle the colorscheme between available options")
                .arg(
                    Arg::new("reverse")
                        .long("reverse")
                        .short('r')
                        .takes_value(false)
                        .about("Toggle in reverse order between available colorschemes"),
                ),
            App::new("list")
                .bin_name("alco-list")
                .about("List available colorschemes"),
            App::new("status")
                .bin_name("alco-status")
                .about("Print the current status")
                .arg(
                    Arg::new("time")
                        .long("time")
                        .short('t')
                        .takes_value(false)
                        .about("Print the duration since the last change"),
                ),
        ]);

    let app_m = app.clone().get_matches();

    let generate_completion = app_m.value_of("generate completion");
    if let Some(shell) = generate_completion {
        let mut stdout = std::io::stdout();
        match shell {
            BASH => generate::<Bash, _>(&mut app, BIN_NAME, &mut stdout),
            ELVISH => generate::<Elvish, _>(&mut app, BIN_NAME, &mut stdout),
            FISH => generate::<Fish, _>(&mut app, BIN_NAME, &mut stdout),
            ZSH => generate::<Zsh, _>(&mut app, BIN_NAME, &mut stdout),
            PWRSH => generate::<PowerShell, _>(&mut app, BIN_NAME, &mut stdout),
            _ => unreachable!(),
        }

        exit(0);
    }

    let config_file = tilde(app_m.value_of("configuration file").unwrap()).into_owned();
    let scheme_dir = tilde(app_m.value_of("colorscheme directory").unwrap()).into_owned();

    let tmux = TmuxOptions {
        reload: app_m.is_present("reload tmux"),
        file: tilde(app_m.value_of("tmux file").unwrap()).into_owned(),
        selector: tilde(app_m.value_of("tmux selector").unwrap()).into_owned(),
    };

    let neovim = NeovimOptions {
        reload: app_m.is_present("reload neovim"),
        file: tilde(app_m.value_of("neovim file").unwrap()).into_owned(),
    };

    match app_m.subcommand() {
        Some(("apply", sub_m)) => {
            let scheme_file = sub_m.value_of("colorscheme").unwrap();
            apply(config_file, scheme_dir, scheme_file, tmux, neovim);
        }
        Some(("toggle", sub_m)) => {
            let reverse = sub_m.is_present("reverse");
            toggle(config_file, scheme_dir, reverse, tmux, neovim);
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

fn apply(
    config_file: impl AsRef<Path>,
    scheme_dir: impl AsRef<Path>,
    scheme_file: &str,
    tmux: TmuxOptions,
    neovim: NeovimOptions,
) {
    if let Err(e) = alco::apply(config_file, scheme_dir, scheme_file) {
        println!("Error applying colorscheme {}:\n{:?}", scheme_file, e);
    } else {
        block_on(async {
            if tmux.reload {
                reload_tmux(tmux.file, tmux.selector, scheme_file).await;
            }
            if neovim.reload {
                reload_neovim(neovim.file).await;
            }
        });
    }
}
fn toggle(
    config_file: impl AsRef<Path>,
    scheme_dir: impl AsRef<Path>,
    reverse: bool,
    tmux: TmuxOptions,
    neovim: NeovimOptions,
) {
    match alco::toggle(config_file, scheme_dir, reverse) {
        Ok(c) => {
            println!("{}", c);
            block_on(async move {
                let t = if tmux.reload {
                    Some(spawn(reload_tmux(tmux.file, tmux.selector, c)))
                } else {
                    None
                };
                let n = if neovim.reload {
                    Some(spawn(reload_neovim(neovim.file)))
                } else {
                    None
                };

                if let Some(t) = t {
                    t.await;
                }
                if let Some(n) = n {
                    n.await;
                }
            });
        }
        Err(_) => println!("Error toggling colorscheme"),
    }
}

fn list(dir: impl AsRef<Path>) {
    match alco::list(dir.as_ref()) {
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
    match alco::status(scheme_dir) {
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

async fn reload_tmux(
    tmux_file: impl AsRef<Path>,
    selector: impl AsRef<Path>,
    scheme_file: impl AsRef<str>,
) {
    if let Err(e) = alco::reload_tmux(tmux_file, selector, scheme_file).await {
        println!("Error reloading tmux sessions:\n{}", e);
    }
}

async fn reload_neovim(file: impl AsRef<Path>) {
    if let Err(e) = alco::reload_neovim(file).await {
        println!("Error reloading neovim instances:\n{}", e);
    }
}
