# Walrs
Like Pywal, but written in Rust

## Features
- Uses the same method as Pywal
- Fast and minimal (10× faster than Pywal)
- Better than Wallust in color accuracy, speed, and simplicity
- Allows customization of brightness and saturation

## Usage
```bash
walrs - Generate a colorscheme from an image

Usage: walrs [OPTIONS]

Options:
  -i <IMAGE>                     Path to your wal.png | For random image: path to your wallpapers/
  -r, --reload-nowal             Reload templates and set the wallpaper
  -R, --reload                   Reload templates without setting the wallpaper
  -t, --theme <THEME>            Use an external theme file
  -g, --generate <GENERATE>      Generate theme and save it in the themes folder (.cache/wal/colorschemes)
  -s, --saturation <SATURATION>  Specify saturation value (-128 to 127)
  -b, --brightness <BRIGHTNESS>  Specify brightness value (-128 to 127)
  -q, --quit                     Enable quit mode (no output)
      --install-completions      Install completions for the current shell
  -h, --help                     Print help
  -V, --version                  Print version

```

## Installation
- AUR
```bash
  yay -S walrs
```
- Source
```bash
git clone https://github.com/pixel2175/walrs && make install
```

## Performance
- Memory: ~3 MB
- Time: ~290 ms for a 1.5 MB 1080P image
- Dependencies: Requires an external app to set the wallpaper (feh, xwallpaper, etc.)

### Benchmark
```bash

time walrs -i ~/.config/wallpaper/
[I] Generate: generate colors.
[I] Template: create templates.
[I] Wallpaper: wallpaper set with feh.
[I] Terminal: terminal colorscheme set.
[I] Xrdb: xrdb colorscheme set.
[I] Colors: colorscheme applied successfully.
● ● ● ● ● ● ● ● ●

________________________________________________________
Executed in  376.01 millis    fish           external
   usr time  236.90 millis    2.05 millis  234.85 millis
   sys time  132.21 millis    1.04 millis  131.17 millis
```

If you need any help, reach me on Discord @pi66

