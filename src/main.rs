mod create_templates;
mod get_colors;
mod reload;
mod utils;
mod wallpaper;

use clap::{ArgAction, CommandFactory, Parser};
use create_templates::create_template;
use get_colors::get_colors;
use reload::reload;
use std::path::Path;
use std::process::exit;
use utils::*;

#[derive(Parser, Debug)]
#[command(
    name = "walrs",
    version = "v1.0.6",
    about = "walrs - Generate colorscheme from image"
)]
struct Arg {
    /// path/to/your/wal.png
    #[arg(short = 'i')]
    image: Option<String>,

    /// generate colors and save it into .cache/wal
    #[arg(short = 'g')]
    generate: Option<String>,

    /// reload Templates from cache file and set the wallpaper
    #[arg(short = 'r', action = ArgAction::SetTrue)]
    reload_nowal: bool,

    /// reload Templates from cache file without set the wallpaper
    #[arg(short = 'R', action = ArgAction::SetTrue)]
    reload: bool,

    /// set quit mode (no output)
    #[arg(short = 'q', action = ArgAction::SetTrue)]
    quit: bool,

    /// specify the saturation value -128 => 127
    #[arg(short = 's', allow_hyphen_values = true)]
    saturation: Option<i8>,

    /// specify the brightness value -128 => 127
    #[arg(short = 'b', allow_hyphen_values = true)]
    brightness: Option<i8>,
}

fn image_path(image: Option<String>, send: bool) -> String {
    match image {
        Some(ref v) if Path::new(v).exists() => match get_absolute_path(v) {
            Some(p) => {
                if Path::new(&p).is_file() {
                    p
                } else {
                    std::str::from_utf8(
                        &std::process::Command::new("sh")
                            .arg("-c")
                            .arg(format!("find \"{}\" -type f | sort -R | head -n1", p))
                            .output()
                            .unwrap()
                            .stdout,
                    )
                    .unwrap()
                    .trim()
                    .to_string()
                }
            }
            None => {
                warning("Wallpaper", "Can't find wallpaper absolute path!", send);
                exit(1);
            }
        },
        Some(_) => {
            warning("Image", "Image does not exist", send);
            exit(1);
        }
        None => {
            warning("Image", "Can't find Image", send);
            exit(1);
        }
    }
}

fn main() {
    let arg = Arg::parse();

    if arg.reload_nowal {
        reload(!arg.quit, true);
        exit(0);
    }

    if arg.reload {
        reload(!arg.quit, false);
        exit(0);
    }

    if arg.image.is_none() && arg.generate.is_none() {
        let mut cmd = Arg::command();
        let _ = cmd.print_help();
        exit(1);
    }

    if arg.generate.is_some() {
        let image_path = image_path(arg.generate, !arg.quit);
        let palette = get_colors(&image_path, !arg.quit, arg.brightness, arg.saturation);
        info("Generate", "generate colors", !arg.quit);

        create_template(palette, &image_path);
        info("Template", "create templates", !arg.quit);
        exit(0)
    };

    if arg.image.is_some() {
        let image_path = image_path(arg.image, !arg.quit);

        let palette = get_colors(&image_path, !arg.quit, arg.brightness, arg.saturation);
        info("Generate", "generate colors", !arg.quit);

        create_template(palette, &image_path);
        info("Template", "create templates", !arg.quit);

        reload(!arg.quit, true);
        print_colors(!arg.quit);
    };
}
