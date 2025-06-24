mod create_templates;
mod get_colors;
mod reload;
mod theme;
mod utils;
mod wallpaper;

use argh::FromArgs;
use create_templates::create_template;
use get_colors::get_colors;
use reload::reload;
use std::fs::{copy, create_dir_all};
use std::process::exit;
use theme::{print_themes, set_theme, theme_exists};
use utils::*;

#[derive(FromArgs)]
#[argh(description = "walrs - Generate colorscheme from image")]
struct Arg {
    #[argh(option, short = 'i', description = "path to image or directory")]
    image: Option<String>,

    #[argh(
        switch,
        short = 'r',
        description = "reload without changing the wallpaper"
    )]
    reload: bool,

    #[argh(
        switch,
        short = 'R',
        long = "reload-no",
        description = "will be removed in the next update; use -w instead"
    )]
    reload_no: bool,

    #[argh(
        option,
        short = 't',
        long = "theme",
        description = "use external theme file from .config/walrs/colorschemes"
    )]
    theme: Option<String>,

    #[argh(
        option,
        short = 'g',
        long = "generate",
        description = "generate & save theme to .config/walrs/colorschemes"
    )]
    generate: Option<String>,

    #[argh(
        option,
        short = 's',
        long = "saturation",
        description = "set saturation value (-128 to 127)"
    )]
    saturation: Option<i16>,

    #[argh(
        option,
        short = 'b',
        long = "brightness",
        description = "set brightness value (-128 to 127)"
    )]
    brightness: Option<i16>,

    #[argh(
        switch,
        short = 'S',
        long = "scripts",
        description = "skip running scripts in ~/.config/walrs/scripts/"
    )]
    run_scripts: Option<bool>,

    #[argh(
        switch,
        short = 'W',
        long = "walless",
        description = "skip changing the wallpaper"
    )]
    walless: Option<bool>,

    #[argh(
        switch,
        short = 'q',
        long = "quiet",
        description = "set quit mode (no output)"
    )]
    quit: bool,

    #[argh(switch, short = 'v', long = "version", description = "show version")]
    version: bool,
}

fn main() {
    // get and load args from user
    let arg: Arg = argh::from_env();

    // save the quit status
    let send = !arg.quit;

    // print the version
    if arg.version {
        info("Version", env!("CARGO_PKG_VERSION"), send);
        exit(0);
    }

    // this will be removed next update
    if arg.reload_no {
        warning(
            "Reload",
            "this will be removed in the next update, use -W instead",
            send,
        );
        reload(send, true, arg.run_scripts.unwrap_or(false));
        exit(0);
    }

    // reload colors with setting wallpaper
    if arg.reload {
        reload(
            send,
            arg.walless.unwrap_or(false),
            arg.run_scripts.unwrap_or(false),
        );
        exit(0);
    }

    // if user didn't type any thing
    if arg.image.is_none() && arg.theme.is_none() && arg.generate.is_none() {
        warning("Args", "run: walrs --help", send);
        exit(1);
    }

    // show or set theme from user
    if let Some(v) = arg.theme {
        let config = get_config(send);
        if v == "themes" {
            print_themes(send);
        } else if theme_exists(&config) {
            set_theme(v, send, arg.run_scripts.unwrap_or(false));
        } else {
            let colorschemes_dir = config.join("walrs").join("colorschemes");
            create_dir_all(&colorschemes_dir).unwrap();
            let walrs_cache = share_files();
            if !theme_exists(walrs_cache.parent().unwrap()) {
                warning("theme", "Can't find configuration directory", send);
                exit(1)
            }
            run(&format!(
                "cp -r {}/* {}",
                walrs_cache.join("colorschemes").display(),
                colorschemes_dir.display()
            ));
            set_theme(v, send, arg.run_scripts.unwrap_or(false));
        }
        exit(0);
    }

    // generate a new theme from current colors
    if let Some(v) = arg.generate {
        let dis = get_config(send).join("walrs").join("colorschemes");
        create_dir_all(dis.join("dark")).unwrap();
        copy(
            get_cache(send).join("wal").join("colors"),
            dis.join("dark").join(v),
        )
        .unwrap();
        info("Generate", "generate colors", send);
        exit(0);
    };

    // analyze the image and generate the palette
    if arg.image.is_some() {
        let image_path = image_path(arg.image, send);
        let palette = get_colors(&image_path, send, arg.brightness, arg.saturation);
        info("Generate", "generate colors", send);

        create_template(palette, &image_path, send);
        info("Template", "create templates", send);

        reload(
            send,
            arg.walless.unwrap_or(false),
            arg.run_scripts.unwrap_or(false),
        );
        print_colors(send);
    };
}
