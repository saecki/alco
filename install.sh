#!/bin/sh

cargo install --path .

case "$SHELL" in
    *zsh)
    echo "creating a completion script for zsh"
    ~/.cargo/bin/alco -g "zsh" > ~/.config/zsh/functions/_alco
    ;;
    *)
    echo "create a completion script for your shell manually by running 'alco --generate-completion <shell>'"
    ;;
esac

