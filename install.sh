#!/bin/sh

cargo build --release
sudo cp target/release/alacritty-colorscheme /usr/local/bin/alco

case "$SHELL" in
    *bash)
	echo "creating a completion script for bash"
	/usr/local/bin/alco -g "bash" | sudo tee /etc/bash_completion.d/alco > /dev/null
	;;
    *zsh)
	echo "creating a completion script for zsh"
	/usr/local/bin/alco -g "zsh" | sudo tee /usr/share/zsh/site-functions/_alco > /dev/null
	;;
    *)
	echo "create a completion script for your shell manually by running 'alco --generate-completion <shell>'"
	;;
esac

