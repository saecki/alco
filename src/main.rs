use clap::{crate_authors, crate_version, value_parser, Arg, ColorChoice, Command, ValueHint};
use clap_complete::generate;
use clap_complete::shells::{Bash, Elvish, Fish, PowerShell, Zsh};
use shellexpand::tilde;

use std::future::Future;
use std::path::Path;
use std::process::exit;
use std::str::FromStr;
use std::time::Duration;

const BIN_NAME: &str = "alco";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shell {
    Bash,
    Elvish,
    Fish,
    Pwrsh,
    Zsh,
}

impl FromStr for Shell {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bash" => Ok(Shell::Bash),
            "elvish" => Ok(Shell::Elvish),
            "fish" => Ok(Shell::Fish),
            "powershell" => Ok(Shell::Pwrsh),
            "zsh" => Ok(Shell::Zsh),
            _ => Err("Unknown shell"),
        }
    }
}

struct Options {
    alacritty: AlacrittyOptions,
    kitty: KittyOptions,
    tmux: TmuxOptions,
    neovim: NeovimOptions,
    starship: StarshipOptions,
    delta: DeltaOptions,
    cmus: CmusOptions,
}

struct AlacrittyOptions {
    reload: bool,
    file: String,
    in_file: String,
    selector: String,
}

struct KittyOptions {
    reload: bool,
    file: String,
    socket: String,
    selector: String,
}

struct TmuxOptions {
    reload: bool,
    file: String,
    selector: String,
}

struct NeovimOptions {
    reload: bool,
    command: String,
}

struct StarshipOptions {
    reload: bool,
    file: String,
    in_file: String,
    selector: String,
}

struct DeltaOptions {
    reload: bool,
    file: String,
    selector: String,
}

struct CmusOptions {
    reload: bool,
    selector: String,
}

fn main() {
    let mut app = Command::new(BIN_NAME)
        .color(ColorChoice::Auto)
        .bin_name(BIN_NAME)
        .version(crate_version!())
        .author(crate_authors!())
        .about("Update terminal colorschemes on the fly")
        .arg(
            Arg::new("configuration file")
                .long("config-file")
                .short('c')
                .default_value(alco::DEFAULT_CONFIG_FILE)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .help("Alco's configuration file"),
        )
        .arg(
            Arg::new("colorscheme file")
                .long("colorscheme-file")
                .short('C')
                .default_value(alco::DEFAULT_COLORSCHEME_FILE)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .help("The file that contains a list of colorschemes"),
        )
        .arg(
            Arg::new("reload all")
                .long("reload-all")
                .short('a')
                .num_args(0)
                .help("Reload all additional colorschemes"),
        )
        .arg(
            Arg::new("reload alacritty")
                .long("reload-alacritty")
                .short('A')
                .num_args(0)
                .conflicts_with("reload all")
                .help("Also reload alacritty updating the configuration file"),
        )
        .arg(
            Arg::new("alacritty file")
                .long("alacritty-file")
                .default_value(alco::DEFAULT_ALACRITTY_FILE)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .help("The alacritty configuration file which will updated"),
        )
        .arg(
            Arg::new("alacritty in file")
                .long("alacritty-in-file")
                .default_value(alco::DEFAULT_ALACRITTY_IN_FILE)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .help("The alacritty configuration file which will updated"),
        )
        .arg(
            Arg::new("alacritty selector")
                .long("alacritty-selector")
                .default_value(alco::DEFAULT_ALACRITTY_SELECTOR)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .help("The alacritty selector file which contains a colorscheme mapping"),
        )
        .arg(
            Arg::new("reload kitty")
                .long("reload-kitty")
                .short('k')
                .num_args(0)
                .conflicts_with("reload all")
                .help("Also reload kitty by sourcing a configuration file"),
        )
        .arg(
            Arg::new("kitty file")
                .long("kitty-file")
                .default_value(alco::DEFAULT_KITTY_FILE)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .help("The kitty configuration file which will be overwritten and sourced"),
        )
        .arg(
            Arg::new("kitty selector")
                .long("kitty-selector")
                .default_value(alco::DEFAULT_KITTY_SELECTOR)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .help("The kitty selector file which contains a colorscheme mapping"),
        )
        .arg(
            Arg::new("kitty socket")
                .long("kitty-socket")
                .default_value(alco::DEFAULT_KITTY_SOCKET)
                .value_name("socket")
                .value_hint(ValueHint::FilePath)
                .help("The unix socket on which kitty is listening for remote control"),
        )
        .arg(
            Arg::new("reload tmux")
                .long("reload-tmux")
                .short('t')
                .num_args(0)
                .conflicts_with("reload all")
                .help("Also reload tmux by sourcing a configuration file"),
        )
        .arg(
            Arg::new("tmux file")
                .long("tmux-file")
                .default_value(alco::DEFAULT_TMUX_FILE)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .help("The tmux configuration file which will be overwritten and sourced"),
        )
        .arg(
            Arg::new("tmux selector")
                .long("tmux-selector")
                .default_value(alco::DEFAULT_TMUX_SELECTOR)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .help("The tmux selector file which contains a colorscheme mapping"),
        )
        .arg(
            Arg::new("reload neovim")
                .long("reload-neovim")
                .short('n')
                .num_args(0)
                .conflicts_with("reload all")
                .help("Also reload neovim by sourcing a configuration file"),
        )
        .arg(
            Arg::new("neovim command")
                .long("neovim-command")
                .default_value(alco::DEFAULT_NEOVIM_COMMAND)
                .value_name("command")
                .value_hint(ValueHint::FilePath)
                .help("The neovim lua codde that will be executed to update the colorscheme"),
        )
        .arg(
            Arg::new("reload starship")
                .long("reload-starship")
                .short('s')
                .num_args(0)
                .conflicts_with("reload all")
                .help("Also reload starship by updating the configuration file"),
        )
        .arg(
            Arg::new("starship file")
                .long("starship-file")
                .default_value(alco::DEFAULT_STARSHIP_FILE)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .help("The starship configuration file which will be overwritten"),
        )
        .arg(
            Arg::new("starship in file")
                .long("starship-in-file")
                .default_value(alco::DEFAULT_STARSHIP_IN_FILE)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .help("The starship in file which will be read"),
        )
        .arg(
            Arg::new("starship selector")
                .long("starship-selector")
                .default_value(alco::DEFAULT_STARSHIP_SELECTOR)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .help("The starship selector file which contains a colorscheme mapping"),
        )
        .arg(
            Arg::new("reload delta")
                .long("reload-delta")
                .short('d')
                .num_args(0)
                .conflicts_with("reload all")
                .help("Also reload delta by updating the configuration file"),
        )
        .arg(
            Arg::new("delta file")
                .long("delta-file")
                .default_value(alco::DEFAULT_DELTA_FILE)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .help("The delta configuration file which will be overwritten"),
        )
        .arg(
            Arg::new("delta selector")
                .long("delta-selector")
                .default_value(alco::DEFAULT_DELTA_SELECTOR)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .help("The delta selector file which contains a colorscheme mapping"),
        )
        .arg(
            Arg::new("reload cmus")
                .long("reload-cmus")
                .short('m')
                .num_args(0)
                .conflicts_with("reload all")
                .help("Also reload cmus by sourcing a configuration file"),
        )
        .arg(
            Arg::new("cmus selector")
                .long("cmus-selector")
                .default_value(alco::DEFAULT_CMUS_SELECTOR)
                .value_name("file")
                .value_hint(ValueHint::FilePath)
                .help("The cmus selector file which contains a colorscheme mapping"),
        )
        .arg(
            Arg::new("generate completion")
                .long("generate-completion")
                .short('g')
                .value_name("shell")
                .value_parser(value_parser!(Shell))
                .help("Generates a completion script for the specified shell"),
        )
        .subcommands(vec![
            Command::new("apply")
                .bin_name("alco-apply")
                .about("Apply a colorscheme")
                .arg(Arg::new("colorscheme").index(1).value_name("colorscheme").required(true)),
            Command::new("toggle")
                .bin_name("alco-toggle")
                .about("Toggle the colorscheme between available options")
                .arg(
                    Arg::new("reverse")
                        .long("reverse")
                        .short('r')
                        .num_args(0)
                        .help("Toggle in reverse order between available colorschemes"),
                ),
            Command::new("list").bin_name("alco-list").about("List available colorschemes"),
            Command::new("status").bin_name("alco-status").about("Print the current status").arg(
                Arg::new("time")
                    .long("time")
                    .short('t')
                    .num_args(0)
                    .help("Print the duration since the last change"),
            ),
        ]);

    let app_m = app.clone().get_matches();

    let generate_completion = app_m.get_one("generate completion");
    if let Some(shell) = generate_completion {
        let mut stdout = std::io::stdout();
        match shell {
            Shell::Bash => generate(Bash, &mut app, BIN_NAME, &mut stdout),
            Shell::Elvish => generate(Elvish, &mut app, BIN_NAME, &mut stdout),
            Shell::Fish => generate(Fish, &mut app, BIN_NAME, &mut stdout),
            Shell::Zsh => generate(Zsh, &mut app, BIN_NAME, &mut stdout),
            Shell::Pwrsh => generate(PowerShell, &mut app, BIN_NAME, &mut stdout),
        }

        exit(0);
    }

    let colors_file = tilde(app_m.get_one::<String>("colorscheme file").unwrap()).into_owned();
    let config_file = tilde(app_m.get_one::<String>("configuration file").unwrap()).into_owned();
    let reload_all = app_m.get_flag("reload all");

    let alacritty = AlacrittyOptions {
        reload: app_m.get_flag("reload alacritty") | reload_all,
        file: tilde(app_m.get_one::<String>("alacritty file").unwrap()).into_owned(),
        in_file: tilde(app_m.get_one::<String>("alacritty in file").unwrap()).into_owned(),
        selector: tilde(app_m.get_one::<String>("alacritty selector").unwrap()).into_owned(),
    };
    let kitty = KittyOptions {
        reload: app_m.get_flag("reload kitty") | reload_all,
        file: tilde(app_m.get_one::<String>("kitty file").unwrap()).into_owned(),
        socket: tilde(app_m.get_one::<String>("kitty socket").unwrap()).into_owned(),
        selector: tilde(app_m.get_one::<String>("kitty selector").unwrap()).into_owned(),
    };
    let tmux = TmuxOptions {
        reload: app_m.get_flag("reload tmux") | reload_all,
        file: tilde(app_m.get_one::<String>("tmux file").unwrap()).into_owned(),
        selector: tilde(app_m.get_one::<String>("tmux selector").unwrap()).into_owned(),
    };
    let neovim = NeovimOptions {
        reload: app_m.get_flag("reload neovim") | reload_all,
        command: app_m.get_one::<String>("neovim command").unwrap().to_owned(),
    };
    let starship = StarshipOptions {
        reload: app_m.get_flag("reload starship") | reload_all,
        file: tilde(app_m.get_one::<String>("starship file").unwrap()).into_owned(),
        in_file: tilde(app_m.get_one::<String>("starship in file").unwrap()).into_owned(),
        selector: tilde(app_m.get_one::<String>("starship selector").unwrap()).into_owned(),
    };
    let delta = DeltaOptions {
        reload: app_m.get_flag("reload delta") | reload_all,
        file: tilde(app_m.get_one::<String>("delta file").unwrap()).into_owned(),
        selector: tilde(app_m.get_one::<String>("delta selector").unwrap()).into_owned(),
    };
    let cmus = CmusOptions {
        reload: app_m.get_flag("reload cmus") | reload_all,
        selector: tilde(app_m.get_one::<String>("cmus selector").unwrap()).into_owned(),
    };

    let opts = Options {
        alacritty,
        kitty,
        tmux,
        neovim,
        starship,
        delta,
        cmus,
    };

    match app_m.subcommand() {
        Some(("apply", sub_m)) => {
            let colorscheme = sub_m.get_one::<String>("colorscheme").unwrap();
            apply(colors_file, config_file, colorscheme, opts);
        }
        Some(("toggle", sub_m)) => {
            let reverse = sub_m.get_flag("reverse");
            toggle(colors_file, config_file, reverse, opts);
        }
        Some(("list", _)) => list(colors_file),
        Some(("status", sub_m)) => {
            let time = sub_m.get_flag("time");
            status(config_file, time);
        }
        _ => {
            app.print_help().ok();
        }
    }
}

fn apply(
    colors_file: impl AsRef<Path>,
    config_file: impl AsRef<Path>,
    colorscheme: &str,
    opts: Options,
) {
    match alco::apply(colors_file, config_file, colorscheme.to_owned()) {
        Ok(_) => apply_colorscheme(colorscheme, opts),
        Err(e) => {
            println!("Error applying colorscheme {}:\n{:?}", colorscheme, e);
        }
    }
}

fn toggle(
    colors_file: impl AsRef<Path>,
    config_file: impl AsRef<Path>,
    reverse: bool,
    opts: Options,
) {
    match alco::toggle(&colors_file, &config_file, reverse) {
        Ok(colorscheme) => apply_colorscheme(&colorscheme, opts),
        Err(e) => println!("Error toggling colorscheme:\n{}", e),
    }
}

fn apply_colorscheme(colorscheme: &str, opts: Options) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_io()
        .build()
        .expect("tokio runtime failed to start");

    runtime.block_on(async move {
        #[rustfmt::skip]
        tokio::join!(
            spawn_if(opts.alacritty.reload, reload_alacritty(opts.alacritty, colorscheme.to_owned())),
            spawn_if(opts.kitty.reload, reload_kitty(opts.kitty, colorscheme.to_owned())),
            spawn_if(opts.tmux.reload, reload_tmux(opts.tmux, colorscheme.to_owned())),
            spawn_if(opts.neovim.reload, reload_neovim(opts.neovim.command)),
            spawn_if(opts.starship.reload, reload_starship(opts.starship, colorscheme.to_owned())),
            spawn_if(opts.delta.reload, reload_delta(opts.delta, colorscheme.to_owned())),
            spawn_if(opts.cmus.reload, reload_cmus(opts.cmus, colorscheme.to_owned())),
        );
    });
}

async fn spawn_if<F>(condition: bool, f: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    if condition {
        if let Err(e) = tokio::spawn(f).await {
            println!("Error: {e}");
        }
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
                println!("{} changed {} ago", s.current, humantime::format_duration(seconds),);
            } else {
                println!("{}", s.current);
            }
        }
        Err(e) => println!("Error getting current colorscheme:\n{}", e),
    }
}

async fn reload_alacritty(opts: AlacrittyOptions, colorscheme: impl AsRef<str>) {
    if let Err(e) = alco::reload_alacritty(opts.file, opts.in_file, opts.selector, colorscheme) {
        println!("Error reloading alacritty colorscheme:\n{}", e);
    }
}

async fn reload_kitty(opts: KittyOptions, colorscheme: impl AsRef<str>) {
    if let Err(e) = alco::reload_kitty(opts.file, opts.socket, opts.selector, colorscheme) {
        println!("Error reloading kitty colorscheme:\n{}", e);
    }
}

async fn reload_tmux(opts: TmuxOptions, colorscheme: impl AsRef<str>) {
    if let Err(e) = alco::reload_tmux(opts.file, opts.selector, colorscheme) {
        println!("Error reloading tmux colorscheme:\n{}", e);
    }
}

async fn reload_neovim(command: impl AsRef<str>) {
    if let Err(e) = alco::reload_neovim(command).await {
        println!("Error reloading neovim colorscheme:\n{}", e);
    }
}

async fn reload_starship(opts: StarshipOptions, colorscheme: impl AsRef<str>) {
    if let Err(e) = alco::reload_starship(opts.file, opts.in_file, opts.selector, colorscheme) {
        println!("Error reloading starship colorscheme:\n{}", e);
    }
}

async fn reload_delta(opts: DeltaOptions, colorscheme: impl AsRef<str>) {
    if let Err(e) = alco::reload_delta(opts.file, opts.selector, colorscheme) {
        println!("Error reloading delta colorscheme:\n{}", e);
    }
}

async fn reload_cmus(opts: CmusOptions, colorscheme: impl AsRef<str>) {
    if let Err(e) = alco::reload_cmus(opts.selector, colorscheme) {
        println!("Error reloading cmus colorscheme:\n{}", e);
    }
}
