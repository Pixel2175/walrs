use crate::{
    create_templates::create_template,
    reload::reload,
    utils::{get_config, run, warning},
};
use std::fs::{create_dir_all, read_dir, read_to_string};
use std::path::Path;
use std::process::exit;

pub fn theme_exists(dir: &Path) -> bool {
    dir.join("walrs").join("colorscheme").exists() || dir.join("wal").join("colorscheme").exists()
}

pub fn print_themes(send: bool) {
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

pub fn collect_themes(subdir: &str, send: bool) -> Vec<String> {
    let base = get_config(send);
    let mut themes = vec![];
    for folder in ["wal", "walrs"] {
        let path = base.join(folder).join("colorschemes").join(subdir);
        if let Ok(entries) = read_dir(path) {
            themes.extend(
                entries
                    .filter_map(|e| e.ok())
                    .filter_map(|e| e.file_name().into_string().ok()),
            );
        }
    }

    themes
}

fn hex_to_rgb(file: Vec<String>) -> Vec<(u8, u8, u8)> {
    file.iter()
        .map(|h| {
            u32::from_str_radix(&h[1..], 16)
                .map(|v| ((v >> 16) as u8, (v >> 8 & 0xFF) as u8, (v & 0xFF) as u8))
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

pub fn set_theme(theme_name: String, send: bool) {
    let base = get_config(send);
    let mut theme: Vec<String> = ["dark", "light"]
        .iter()
        .flat_map(|variant| collect_themes(variant, send))
        .collect();

    if !theme.is_empty() {
        let dis = get_config(send).join("wal").join("colorschemes");
        create_dir_all(&dis).unwrap();
        run(&format!(
            "cp -r /etc/walrs/colorschemes/* {}/walrs/colorschemes",
            base.display()
        ));
    }
    theme.sort();
    theme.dedup();
    if theme.contains(&theme_name) {
        let file: Vec<String> = [
            base.join("wal/colorschemes/dark"),
            base.join("wal/colorschemes/light"),
            base.join("walrs/colorschemes/dark"),
            base.join("walrs/colorschemes/light"),
        ]
        .into_iter()
        .find_map(|p| read_to_string(p.join(&theme_name)).ok())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

        let rgb_colors = hex_to_rgb(file);

        create_template((rgb_colors, 100), "None", send);
        reload(send, false);
    } else {
        warning("Theme", "Can't find theme", send);
        exit(1)
    }
}
