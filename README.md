# alco

### Usage
```
alco 0.2.0

Saecki <tobiasschmitz2001@gmail.com>

Update the terminal colorschemes on the fly.

USAGE:
    alco [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -a, --reload-all
            Reload all additional colorschemes

        --alacritty-file <file>
            The alacritty configuration file which will updated [default:
            ~/.config/alacritty/alacritty.yml]

        --alacritty-selector <file>
            The alacritty selector file which contains a colorscheme mapping [default:
            ~/.config/alco/alacritty-selector.yml]

    -c, --config-file <file>
            Alco's configuration file [default: ~/.config/alco/alco.yml]

    -C, --colorscheme-file <file>
            The file that contains a list of colorschemes [default: ~/.config/alco/colors.yml]

        --cmus-selector <file>
            The cmus selector file which contains a colorscheme mapping [default:
            ~/.config/alco/cmus-selector.yml]

    -d, --reload-delta
            Also reload delta by updating the configuration file

        --delta-file <file>
            The delta configuration file which will be overwritten [default:
            ~/.config/delta/colors/current.gitconfig]

        --delta-selector <file>
            The delta selector file which contains a colorscheme mapping [default:
            ~/.config/alco/delta-selector.yml]

    -g, --generate-completion <shell>
            Generates a completion script for the specified shell [possible values: bash, zsh, fish,
            elvish, powershell]

    -h, --help
            Print help information

    -k, --reload-kitty
            Also reload kitty by sourcing a configuration file

        --kitty-file <file>
            The kitty configuration file which will be overwritten and sourced [default:
            ~/.config/kitty/colors/current.conf]

        --kitty-selector <file>
            The kitty selector file which contains a colorscheme mapping [default:
            ~/.config/alco/kitty-selector.yml]

        --kitty-socket <socket>
            The unix socket on which kitty is listening for remote control [default: /tmp/kitty]

    -m, --reload-cmus
            Also reload cmus by sourcing a configuration file

    -n, --reload-neovim
            Also reload neovim by sourcing a configuration file

        --neovim-command <command>
            The neovim command that will be executed to update the colorscheme [default: "lua
            require('colors').reload()"]

    -t, --reload-tmux
            Also reload tmux by sourcing a configuration file

        --tmux-file <file>
            The tmux configuration file which will be overwritten and sourced [default:
            ~/.config/tmux/colors/current.conf]

        --tmux-selector <file>
            The tmux selector file which contains a colorscheme mapping [default:
            ~/.config/alco/tmux-selector.yml]

    -V, --version
            Print version information

SUBCOMMANDS:
    apply     Apply a colorscheme
    help      Print this message or the help of the given subcommand(s)
    list      List available colorschemes
    status    Print the current status
    toggle    Toggle the colorscheme between available options
```

### Selector files
Selctor files contain a mapping from the alco colorscheme names to the respective colorscheme for the specific application. In some cases this might be a path in other cases just a name.

| Application | Type   |
|-------------|--------|
| alacritty   | `path` |
| kitty       | `path` |
| tmux        | `path` |
| delta       | `path` |
| cmus        | `name` |

__Example__
A `tmux-selector.yml` file
```
my-dark-theme: "~/.config/tmux/colors/my-dark-theme.conf"
my-light-theme: "~/.config/tmux/colors/my-light-theme.conf"
else: "~/.config/tmux/colors/my-dark-theme.conf" # default to a dark theme
```
