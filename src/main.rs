mod completions;
mod create_templates;
mod get_colors;
mod reload;
mod theme;
mod utils;
mod wallpaper;

use clap::{ArgAction, CommandFactory, Parser};
use create_templates::create_template;
use dirs_next::{cache_dir, config_dir};
use get_colors::get_colors;
use reload::reload;
use std::fs::{copy, create_dir_all};
use std::path::Path;
use std::process::exit;
use theme::{collect_themes, set_theme};
use utils::*;

#[derive(Parser, Debug)]
#[command(
    name = "walrs",
    version = env!("CARGO_PKG_VERSION"),
    about = "walrs - Generate colorscheme from image",
)]
struct Arg {
    /// path/to/your/wal.png | for random image: path/to/your/wallpapers/
    #[arg(short = 'i')]
    image: Option<String>,

    /// reload Templates with setting the wallpaper
    #[arg(short = 'r',long="reload-nowal", action = ArgAction::SetTrue)]
    reload: bool,

    /// reload Templates without setting the wallpaper
    #[arg(short = 'R',long="reload" ,action = ArgAction::SetTrue)]
    reload_nowal: bool,

    /// use external theme file  
    #[arg(short = 't', long = "theme")]
    theme: Option<String>,

    /// generate theme and save it in themes folder (.cache/wal/colorschemes)
    #[arg(short = 'g', long = "generate")]
    generate: Option<String>,

    /// specify the saturation value -128 => 127
    #[arg(short = 's', long = "saturation", allow_hyphen_values = true)]
    saturation: Option<i8>,

    /// specify the brightness value -128 => 127
    #[arg(short = 'b', long = "brightness", allow_hyphen_values = true)]
    brightness: Option<i8>,

    /// set quit mode (no output)
    #[arg(long="quit",short = 'q', action = ArgAction::SetTrue)]
    quit: bool,

    /// Install completions for the current shell
    #[arg(long = "install-completions")]
    install_completions: bool,
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

fn print_themes() {
    let (dark, light) = (collect_themes("dark"), collect_themes("light"));

    println!("[\x1b[33mDark\x1b[0m]");
    for theme in dark {
        println!("    -{theme}")
    }

    println!("[\x1b[33mLight\x1b[0m]");
    for theme in light {
        println!("    -{theme}")
    }
}

fn main() {
    let arg = Arg::parse();

    if arg.install_completions {
        if completions::install_completions().is_err() {
            warning("Completions", "Failed to install completions", !arg.quit);
            exit(1);
        }
        info(
            "Completions",
            "Completions installed successfully!",
            !arg.quit,
        );
        exit(0)
    }
    if arg.reload_nowal {
        reload(!arg.quit, true);
        exit(0);
    }

    if arg.reload {
        reload(!arg.quit, false);
        exit(0);
    }

    if arg.image.is_none() && arg.theme.is_none() && arg.generate.is_none() {
        let mut cmd = Arg::command();
        cmd.print_help().unwrap();
        exit(1);
    }

    if let Some(v) = arg.theme {
        if v == "themes" {
            print_themes();
        } else {
            if config_dir()
                .unwrap()
                .join("wal")
                .join("colorscheme")
                .exists()
                || config_dir()
                    .unwrap()
                    .join("walrs")
                    .join("colorscheme")
                    .exists()
            {
                set_theme(v, !arg.quit);
            } else {
                create_dir_all(&format!(
                    "{}/walrs/colorschemes",
                    get_config_folder().unwrap()
                ))
                .unwrap();
                run(&format!(
                    "cp -r /etc/walrs/colorschemes/* {}/walrs/colorschemes",
                    get_config_folder().unwrap()
                ));
                set_theme(v, !arg.quit);
            };
        }
        exit(0);
    }

    if let Some(v) = arg.generate {
        let dis = config_dir().unwrap().join("walrs").join("colorschemes");
        create_dir_all(&dis.join("dark")).unwrap();
        copy(
            cache_dir().unwrap().join("wal").join("colors"),
            dis.join("dark").join(v),
        )
        .unwrap();
        info("Generate", "generate colors", !arg.quit);
        exit(0);
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
