#!/bin/env bash 

polybar() {
    if command -v polybar >/dev/null && pgrep polybar >/dev/null; then
        pkill -USR1 polybar
        echo "Polybar: polybar colorscheme set"
    fi
}

polybar
