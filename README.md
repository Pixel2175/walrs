# Walrs
pywal is its written in rust

## Features 
- use the same as pywal do 
- fast and minimal (10x faster than pywal)
- better than wallust in colors, speed ,simplest
- you can edit the brightness and saturation as you want 

## Usage
```bash
walrs - Generate colorscheme from image

Usage: walrs [OPTIONS]

Options:
  -i <IMAGE>                     path/to/your/wal.png | for random image: path/to/your/wallpapers/
  -r, --reload-nowal             reload Templates with setting the wallpaper
  -R, --reload                   reload Templates without setting the wallpaper
  -t, --theme <THEME>            use external theme file
  -g, --generate <GENERATE>      generate theme and save it in themes folder (.cache/wal/colorschemes)
  -s, --saturation <SATURATION>  specify the saturation value -128 => 127
  -b, --brightness <BRIGHTNESS>  specify the brightness value -128 => 127
  -q, --quit                     set quit mode (no output)
      --install-completions      Install completions for the current shell
  -h, --help                     Print help
  -V, --version                  Print version
```

## Installation
- AUR
```bash yay -S walrs```
- Source
```git clone https://github.com/pixel2175/walrs && make install```

## Performance
- **Memory** : 3mb 
- **Time**   : 290ms for image 1.5mb 1080P image
- **Dep**    : this app just requires an external app to set wallpaper (feh,xwallpaper,etc...)

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


- if you want any help catch me on discord @pi66
