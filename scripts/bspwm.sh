#!/bin/env bash 

bspwm() {
    if pgrep -x bspwm >/dev/null; then
        bspc wm -r
        echo "Bspwm: bspwm colorscheme set"
    fi
}
bspwm
