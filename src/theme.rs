use crate::{
    create_templates::create_template,
    reload::reload,
    utils::{get_config_folder, run, warning},
};
use dirs_next::config_dir;
use std::{
    fs::{create_dir_all, read_dir, read_to_string},
    path::Path,
};

pub fn collect_themes(subdir: &str) -> Vec<String> {
    let binding = get_config_folder().unwrap();
    let base = Path::new(&binding);
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

pub fn set_theme(theme_name: String, send: bool) {
    let (dark, light) = (collect_themes("dark"), collect_themes("light"));
    let mut theme: Vec<String> = dark.into_iter().chain(light).collect();
    if theme.is_empty() {
        let dis = config_dir().unwrap().join("wal").join("colorschemes");
        create_dir_all(&dis).unwrap();
        run(&format!(
            "cp -r /etc/walrs/colorschemes/* {}/walrs/colorschemes",
            get_config_folder().unwrap()
        ));
    }
    theme.sort();
    theme.dedup();
    if theme.contains(&theme_name) {
        let base = config_dir().unwrap();

        let file: Vec<String> = [
            base.join("wal/colorschemes/dark").join(&theme_name),
            base.join("wal/colorschemes/light").join(&theme_name),
            base.join("walrs/colorschemes/dark").join(&theme_name),
            base.join("walrs/colorschemes/light").join(&theme_name),
        ]
        .into_iter()
        .find_map(|p| read_to_string(p).ok())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

        let rgb_colors = file
            .iter()
            .map(|h| {
                u32::from_str_radix(&h[1..], 16)
                    .map(|v| ((v >> 16) as u8, (v >> 8 & 0xFF) as u8, (v & 0xFF) as u8))
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        create_template((rgb_colors, 100), "None");
        reload(send, false);
    } else {
        warning("Theme", "Can't find theme", send);
    }
}
