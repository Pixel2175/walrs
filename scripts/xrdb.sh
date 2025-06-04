#!/bin/env bash 

xrdb -merge -quiet "$HOME/.cache/wal/colors.Xresources"
echo "Xrdb: xrdb colorscheme set"

