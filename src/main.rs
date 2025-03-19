mod get_colors;
mod create_templates;
mod reload;
mod wallpaper;
mod utils;

use wallpaper::change_wallpaper;
use reload::reload;
use create_templates::create_template;
use get_colors::get_colors;
use clap::{ArgAction, CommandFactory, Parser};
use std::path::Path;
use std::process::exit;
use utils::*;

#[derive(Parser, Debug)]
#[command(name = "walrs",version="v1.0.1",about= "walrs - Generate colorscheme from image")]

struct Arg {
    /// path/to/your/wal.png
    #[arg(short = 'i')]
    image: Option<String>,

    /// reload Templates from cache file
    #[arg(short = 'R', action = ArgAction::SetTrue)]
    reload: bool,

    /// set quit mode (no output)
    #[arg(short = 'q', action = ArgAction::SetTrue)]
    quit: bool,
}

fn main() {
    let arg = Arg::parse();

    if !arg.reload && arg.image.is_none() {
        let mut cmd = Arg::command();
        let _ = cmd.print_help();
        
        std::process::exit(1);
    }

    if arg.reload {
        reload(!arg.quit);
        exit(0);
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

    let palette = get_colors(image_path.clone(), !arg.quit);
    info("Generate", "generate colors", !arg.quit);

    create_template(palette, image_path.clone());
    info("Template", "create templates", !arg.quit);

    change_wallpaper(&image_path.to_string(),!arg.quit);

    reload(!arg.quit);

    print_colors(!arg.quit);
}

