#!/bin/env bash 

kitty() {
    if command -v kitty >/dev/null && pgrep kitty >/dev/null; then
        kitty @ set-colors --all "$1/wal/colors-kitty.conf"
        echo "Kitty: kitty colorscheme set"
    fi
}

kitty "$HOME/.cache"
