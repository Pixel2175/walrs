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
use std::path::Path;
use std::process::exit;
use theme::{collect_themes, set_theme};
use utils::*;

#[derive(FromArgs)]
#[argh(description = "walrs - Generate colorscheme from image")]
struct Arg {
    #[argh(
        option,
        short = 'i',
        description = "path/to/your/wal.png | path/to/your/wallpapers/"
    )]
    image: Option<String>,

    #[argh(
        option,
        short = 'k',
        long = "backend",
        description = "change the colors backend (walrs -k backends)"
    )]
    backend: Option<String>,

    #[argh(
        switch,
        short = 'r',
        description = "reload without changing the wallpaper"
    )]
    reload: bool,

    #[argh(
        switch,
        short = 'R',
        long = "reload-nowal",
        description = "reload with changing the wallpaper"
    )]
    reload_nowal: bool,

    #[argh(
        option,
        short = 't',
        long = "theme",
        description = "use external theme file"
    )]
    theme: Option<String>,

    #[argh(
        option,
        short = 'g',
        long = "generate",
        description = "generate theme in themes folder (.cache/wal/colorschemes)"
    )]
    generate: Option<String>,

    #[argh(
        option,
        short = 's',
        long = "saturation",
        description = "specify the saturation value -128 => 127"
    )]
    saturation: Option<i8>,

    #[argh(
        option,
        short = 'b',
        long = "brightness",
        description = "specify the brightness value -128 => 127"
    )]
    brightness: Option<i8>,

    #[argh(
        switch,
        short = 'q',
        long = "quit",
        description = "set quit mode (no output)"
    )]
    quit: bool,
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

fn print_themes(send: bool) {
    let (dark, light) = (collect_themes("dark", send), collect_themes("light", send));

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
    let arg: Arg = argh::from_env();
    let backend = match arg.backend {
        Some(v) => {
            if v == "backends" {
                println!(
                    "┌──────────────────────┬───────────────────────┐  
│ Method               │ Description           │  
├──────────────────────┼───────────────────────┤  
│ kmeans               │ best colors, slow     │  
│ color_thief          │ balanced              │  
│ palette_extract      │ fast, weak colors     │  
│ all                  │ use all methods       │  
└──────────────────────┴───────────────────────┘"
                );
                exit(0)
            } else {
                v
            }
        }
        None => "kmeans".to_string(),
    };

    if arg.reload_nowal {
        reload(!arg.quit, true);
        exit(0);
    }

    if arg.reload {
        reload(!arg.quit, false);
        exit(0);
    }

    if let Some(v) = arg.theme {
        if v == "themes" {
            print_themes(!arg.quit);
        } else {
            if get_config(!arg.quit)
                .join("wal")
                .join("colorscheme")
                .exists()
                || get_config(!arg.quit)
                    .join("walrs")
                    .join("colorscheme")
                    .exists()
            {
                set_theme(v, !arg.quit);
            } else {
                create_dir_all(&format!(
                    "{}/walrs/colorschemes",
                    get_config(!arg.quit).to_string_lossy().to_string()
                ))
                .unwrap();
                run(&format!(
                    "cp -r /etc/walrs/colorschemes/* {}/walrs/colorschemes",
                    get_config(!arg.quit).to_string_lossy().to_string()
                ));
                set_theme(v, !arg.quit);
            };
        }
        exit(0);
    }

    if let Some(v) = arg.generate {
        let dis = get_config(!arg.quit).join("walrs").join("colorschemes");
        create_dir_all(&dis.join("dark")).unwrap();
        copy(
            get_config(!arg.quit).join("wal").join("colors"),
            dis.join("dark").join(v),
        )
        .unwrap();
        info("Generate", "generate colors", !arg.quit);
        exit(0);
    };

    if arg.image.is_some() {
        let image_path = image_path(arg.image, !arg.quit);

        let palette = get_colors(
            &image_path,
            &backend,
            !arg.quit,
            arg.brightness,
            arg.saturation,
        );
        info("Generate", "generate colors", !arg.quit);

        create_template(palette, &image_path, !arg.quit);
        info("Template", "create templates", !arg.quit);

        reload(!arg.quit, true);
        print_colors(!arg.quit);
    };
}
