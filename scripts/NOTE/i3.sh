#!/bin/env bash 

i3() {
    if pgrep -x i3 >/dev/null; then
        i3-msg reload
        echo "i3: i3 colorscheme set"
    fi
}

i3
