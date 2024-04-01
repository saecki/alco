use anyhow::{anyhow, bail};
use serde::{Deserialize, Serialize};
use yaml_rust::Yaml;

use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};

pub use alacritty::reload_alacritty;
pub use bat::reload_bat;
pub use cmus::reload_cmus;
pub use delta::reload_delta;
pub use kitty::reload_kitty;
pub use nvim::reload_neovim;
pub use starship::reload_starship;
pub use tmux::reload_tmux;

pub const DEFAULT_COLORSCHEME_FILE: &str = "~/.config/alco/colors.yml";
pub const DEFAULT_CONFIG_FILE: &str = "~/.config/alco/alco.yml";

pub const DEFAULT_ALACRITTY_FILE: &str = "~/.config/alacritty/alacritty.yml";
pub const DEFAULT_ALACRITTY_IN_FILE: &str = "~/.config/alacritty/alacritty.yml.in";
pub const DEFAULT_ALACRITTY_SELECTOR: &str = "~/.config/alco/alacritty-selector.yml";

pub const DEFAULT_KITTY_FILE: &str = "~/.config/kitty/colors/current.conf";
pub const DEFAULT_KITTY_SELECTOR: &str = "~/.config/alco/kitty-selector.yml";
pub const DEFAULT_KITTY_SOCKET: &str = "/tmp/kitty";

pub const DEFAULT_TMUX_FILE: &str = "~/.config/tmux/colors/current.conf";
pub const DEFAULT_TMUX_SELECTOR: &str = "~/.config/alco/tmux-selector.yml";

pub const DEFAULT_NEOVIM_FILE: &str = "~/.config/nvim/colors.vim";
pub const DEFAULT_NEOVIM_COMMAND: &str = "require('colors').reload()";

pub const DEFAULT_STARSHIP_FILE: &str = "~/.config/starship.toml";
pub const DEFAULT_STARSHIP_IN_FILE: &str = "~/.config/starship/starship.toml.in";
pub const DEFAULT_STARSHIP_SELECTOR: &str = "~/.config/alco/starship-selector.yml";

pub const DEFAULT_BAT_FILE: &str = "~/.config/bat/config";
pub const DEFAULT_BAT_IN_FILE: &str = "~/.config/bat/config.in";
pub const DEFAULT_BAT_SELECTOR: &str = "~/.config/alco/bat-selector.yml";

pub const DEFAULT_DELTA_FILE: &str = "~/.config/delta/colors/current.gitconfig";
pub const DEFAULT_DELTA_SELECTOR: &str = "~/.config/alco/delta-selector.yml";

pub const DEFAULT_CMUS_SELECTOR: &str = "~/.config/alco/cmus-selector.yml";

#[cfg(feature = "alacritty")]
mod alacritty;
#[cfg(not(feature = "alacritty"))]
mod alacritty {
    use anyhow::bail;
    use std::path::Path;

    pub fn reload_alacritty(
        _: impl AsRef<Path>,
        _: impl AsRef<Path>,
        _: impl AsRef<str>,
    ) -> anyhow::Result<()> {
        bail!("alco was compiled without the alacritty feature flag")
    }
}

#[cfg(feature = "kitty")]
mod kitty;
#[cfg(not(feature = "kitty"))]
mod kitty {
    use anyhow::bail;
    use std::path::Path;

    pub fn reload_kitty(
        _: impl AsRef<Path>,
        _: impl AsRef<Path>,
        _: impl AsRef<Path>,
        _: impl AsRef<str>,
    ) -> anyhow::Result<()> {
        bail!("alco was compiled without the kitty feature flag")
    }
}

#[cfg(feature = "tmux")]
mod tmux;
#[cfg(not(feature = "tmux"))]
mod tmux {
    use anyhow::bail;
    use std::path::Path;

    pub fn reload_tmux(
        _: impl AsRef<Path>,
        _: impl AsRef<Path>,
        _: impl AsRef<str>,
    ) -> anyhow::Result<()> {
        bail!("alco was compiled without the tmux feature flag")
    }
}

#[cfg(feature = "neovim")]
mod nvim;
#[cfg(not(feature = "neovim"))]
mod nvim {
    use anyhow::bail;
    use std::path::Path;

    pub async fn reload_neovim(_: impl AsRef<str>) -> anyhow::Result<()> {
        bail!("alco was compiled without the neovim feature flag")
    }
}

#[cfg(feature = "starship")]
mod starship;
#[cfg(not(feature = "starship"))]
mod starship {
    use anyhow::bail;
    use std::path::Path;

    pub fn reload_starship(
        _: impl AsRef<Path>,
        _: impl AsRef<Path>,
        _: impl AsRef<Path>,
        _: impl AsRef<str>,
    ) -> anyhow::Result<()> {
        bail!("alco was compiled without the starship feature flag")
    }
}

#[cfg(feature = "bat")]
mod bat;
#[cfg(not(feature = "bat"))]
mod bat {
    use anyhow::bail;
    use std::path::Path;

    pub fn reload_bat(
        _: impl AsRef<Path>,
        _: impl AsRef<Path>,
        _: impl AsRef<Path>,
        _: impl AsRef<str>,
    ) -> anyhow::Result<()> {
        bail!("alco was compiled without the bat feature flag")
    }
}

#[cfg(feature = "delta")]
mod delta;
#[cfg(not(feature = "delta"))]
mod delta {
    use anyhow::bail;
    use std::path::Path;

    pub fn reload_delta(
        _: impl AsRef<Path>,
        _: impl AsRef<Path>,
        _: impl AsRef<str>,
    ) -> anyhow::Result<()> {
        bail!("alco was compiled without the delta feature flag")
    }
}

#[cfg(feature = "cmus")]
mod cmus;
#[cfg(not(feature = "cmus"))]
mod cmus {
    use anyhow::bail;
    use std::path::Path;

    pub fn reload_cmus(_: impl AsRef<Path>, _: impl AsRef<str>) -> anyhow::Result<()> {
        bail!("alco was compiled without the tmux feature flag")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Colors {
    colors: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Config {
    current: String,
    changed: SystemTime,
}

impl Config {
    fn new(current: String, changed: SystemTime) -> Self {
        Self { current, changed }
    }

    fn now(current: String) -> Self {
        Self::new(current, SystemTime::now())
    }
}

pub struct Status {
    pub current: String,
    pub duration: Duration,
}

impl From<Config> for Status {
    fn from(config: Config) -> Self {
        let duration = SystemTime::now().duration_since(config.changed).unwrap_or_default();
        Self::new(config.current, duration)
    }
}

impl Status {
    pub const fn new(current: String, duration: Duration) -> Self {
        Status { current, duration }
    }
}

pub fn apply(
    colors_file: impl AsRef<Path>,
    config_file: impl AsRef<Path>,
    colorscheme: String,
) -> anyhow::Result<()> {
    let colors = list(colors_file)?;
    if colors.contains(&colorscheme) {
        write_config(config_file, &Config::now(colorscheme))?;
        Ok(())
    } else {
        bail!("No matching colorscheme")
    }
}

pub fn toggle(
    colors_file: impl AsRef<Path>,
    config_file: impl AsRef<Path>,
    reverse: bool,
) -> anyhow::Result<String> {
    let mut available_colors = list(colors_file)?;
    if available_colors.is_empty() {
        bail!("No colorschemes available");
    }

    let mut index = 0;
    if let Ok(c) = parse_config(config_file.as_ref()) {
        if let Some(i) = available_colors.iter().position(|f| f == &c.current) {
            index = if reverse {
                (available_colors.len() + i - 1) % available_colors.len()
            } else {
                (i + 1) % available_colors.len()
            };
        }
    }

    let new_scheme = available_colors.remove(index);
    let new_config = Config::now(new_scheme);

    write_config(config_file, &new_config)?;

    Ok(new_config.current)
}

pub fn reload(
    colors_file: impl AsRef<Path>,
    config_file: impl AsRef<Path>,
) -> anyhow::Result<String> {
    let mut available_colors = list(colors_file)?;
    if available_colors.is_empty() {
        bail!("No colorschemes available");
    }

    let mut index = 0;
    if let Ok(c) = parse_config(config_file.as_ref()) {
        if let Some(i) = available_colors.iter().position(|f| f == &c.current) {
            index = i;
        }
    }

    let new_scheme = available_colors.remove(index);
    let new_config = Config::now(new_scheme);

    write_config(config_file, &new_config)?;

    Ok(new_config.current)
}

pub fn list(colors_file: impl AsRef<Path>) -> anyhow::Result<Vec<String>> {
    parse_colors(colors_file).map(|c| c.colors)
}

pub fn status(config_file: impl AsRef<Path>) -> anyhow::Result<Status> {
    let config = parse_config(config_file)?;
    Ok(Status::from(config))
}

fn write_config(config_file: impl AsRef<Path>, config: &Config) -> anyhow::Result<()> {
    let config_str = serde_yaml::to_string(config)?;
    fs::write(config_file, config_str)?;
    Ok(())
}

fn parse_colors(colors_file: impl AsRef<Path>) -> anyhow::Result<Colors> {
    let colors_str = fs::read_to_string(colors_file.as_ref())
        .map_err(|_| anyhow!("Error reading colorscheme list file"))?;
    let colors = serde_yaml::from_str::<Colors>(&colors_str)
        .map_err(|_| anyhow!("Error parsing colorscheme list file"))?;
    Ok(colors)
}

fn parse_config(config_file: impl AsRef<Path>) -> anyhow::Result<Config> {
    let config_str = fs::read_to_string(config_file.as_ref())?;
    Ok(serde_yaml::from_str::<Config>(&config_str)?)
}

fn selector<'a>(selector: &'a Yaml, key: &'_ str) -> Option<&'a str> {
    let map = selector.as_hash()?;
    let mut default = None;

    for (k, v) in map.iter() {
        if k.as_str()? == key {
            return v.as_str();
        } else if k.as_str()? == "else" {
            default = v.as_str();
        }
    }

    default
}
