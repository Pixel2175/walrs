mod get_colors;
mod create_templates;
mod reload;
mod wallpaper;
mod utils;


use reload::reload;
use create_templates::create_template;
use get_colors::get_colors;
use clap::{ArgAction, CommandFactory, Parser};
use std::path::Path;
use std::process::exit;
use utils::*;

#[derive(Parser, Debug)]
#[command(name = "walrs",version="v1.0.4-2",about= "walrs - Generate colorscheme from image")]
struct Arg {
    /// path/to/your/wal.png
    #[arg(short = 'i')]
    image: Option<String>,

    
    /// generate colors and save it into .cache/wal 
    #[arg(short = 'g')]
    generate: Option<String>,

    /// reload Templates from cache file without set the wallpaper
    #[arg(short = 'r', action = ArgAction::SetTrue)]
    reload_nowal: bool,

    /// reload Templates from cache file and set the wallpaper
    #[arg(short = 'R', action = ArgAction::SetTrue)]
    reload: bool,

    /// set quit mode (no output)
    #[arg(short = 'q', action = ArgAction::SetTrue)]
    quit: bool,


    /// specify the saturation value 
    #[arg(short = 's')]
    saturation: Option<i8>,


    /// specify the brightness value
    #[arg(short = 'b')]
    brightness: Option<i8>,




}

fn main() {
    let arg = Arg::parse();


    if arg.reload_nowal {
        reload(!arg.quit,true);
        exit(0);
    }

    if arg.reload {
        reload(!arg.quit,false);
        exit(0);
    }


    if arg.image.is_none() && arg.generate.is_none() {
        let mut cmd = Arg::command();
        let _ = cmd.print_help();
        exit(1);
    }

    match arg.generate {
        Some(ref v) if Path::new(v).exists() => match get_absolute_path(v) {
            Some(p) => {
                let palette = get_colors(&p, !arg.quit,arg.brightness,arg.saturation);
                info("Generate", "generate colors", !arg.quit);

                create_template(palette, &p);
                info("Template", "create templates", !arg.quit);
                exit(0)
            },
            None => {
                warning("Wallpaper", "Can't find wallpaper absolute path!", !arg.quit);
                exit(1);
            }
        },
        Some(_) => {
            warning("Image", "Image does not exist", !arg.quit);
            exit(1);
        }
        None => {}
    }

    let image_path = match arg.image {
        Some(ref v) if Path::new(v).exists() => match get_absolute_path(v) {
            Some(p) => p,
            None => {
                warning("Wallpaper", "Can't find wallpaper absolute path!", !arg.quit);
                exit(1);
            }
        },
        Some(_) => {
            warning("Image", "Image does not exist", !arg.quit);
            exit(1);
        }
        None => {
            warning("Image", "Can't find Image", !arg.quit);
            exit(1);
        }
    };

    let palette = get_colors(&image_path, !arg.quit,arg.brightness,arg.saturation);
    info("Generate", "generate colors", !arg.quit);

    create_template(palette, &image_path);
    info("Template", "create templates", !arg.quit);

    reload(!arg.quit,true);

    print_colors(!arg.quit);
}

