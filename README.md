# alco

### Usage
```
alco 0.1.0
Saecki <tobiasschmitz2001@gmail.com>
Update the colorscheme of alacritty.

USAGE:
    alco [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help             Prints help information
    -a, --reload-all       Reload all additional colorschemes
    -m, --reload-cmus      Also reload cmus by sourcing a configuration file
    -n, --reload-neovim    Also reload neovim by sourcing a configuration file
    -t, --reload-tmux      Also reload tmux by sourcing a configuration file
    -V, --version          Prints version information

OPTIONS:
        --cmus-selector <file>
            The cmus selector file which contains a coloscheme mapping [default:
            ~/.config/alco/cmus-selector.yml]

    -C, --colorscheme-dir <directory>
            The direcotry that contains colorscheme configurations [default:
            ~/.config/alacritty/colors/]

    -c, --config-file <file>
            Alacritty's configuration file in which values are replaced [default:
            ~/.config/alacritty/alacritty.yml]

    -g, --generate-completion <shell>
            Generates a completion script for the specified shell [possible values: bash, zsh, fish,
            elvish, powershell]

        --neovim-file <file>
            The neovim configuration file which will be sourced [default: ~/.config/nvim/colors.vim]

        --tmux-file <file>
            The tmux configuration file which will be overwritten and sourced [default:
            ~/.config/tmux/colors/current.conf]

        --tmux-selector <file>
            The tmux selector file which contains a coloscheme mapping [default:
            ~/.config/alco/tmux-selector.yml]


SUBCOMMANDS:
    apply     Apply a colorscheme
    help      Prints this message or the help of the given subcommand(s)
    list      List available colorschemes
    status    Print the current status
    toggle    Toggle the colorscheme between available options
```

### Selector files
Selctor files contain a mapping from the alacritty colorscheme filename to the respective colorscheme for the specific application. In some cases this might be a path in other cases just a name.

| Application | Type   |
|-------------|--------|
| tmux        | `path` |
| cmus        | `name` |

__Example__
A `tmux-selector.yml` file
```
my-dark-theme.yml: "~/.config/tmux/colors/my-dark-theme.conf"
my-light-theme.yml: "~/.config/tmux/colors/my-light-theme.conf"
else: "~/.config/tmux/colors/my-dark-theme.conf" # default to a dark theme
```
