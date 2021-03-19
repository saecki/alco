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
    -v, --reload-neovim    Also reload neovim by sourcing a configuration file
    -t, --reload-tmux      Also reload tmux by sourcing a configuration file
    -V, --version          Prints version information

OPTIONS:
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
            ~/.config/tmux/colors/selector.yml]


SUBCOMMANDS:
    apply     Apply a colorscheme
    help      Prints this message or the help of the given subcommand(s)
    list      List available colorschemes
    status    Print the current status
    toggle    Toggle the colorscheme between available options
```
