use crate::utils::{get_cache, get_config, warning};
use std::{
    fs::{self, create_dir_all, read_to_string, write},
    process::exit,
};

fn change(template: &str, colors: (u8, u8, u8), alpha: u8) -> String {
    let (r, g, b) = colors;

    if template.contains(".strip") {
        format!("{r:02x}{g:02x}{b:02x}")
    } else if template.contains(".xrgba") {
        format!("{r:02x}/{g:02x}/{b:02x}/{alpha:02x}")
    } else if template.contains(".rgba") {
        format!("{r},{g},{b},{alpha}")
    } else if template.contains(".rgb") {
        format!("{r},{g},{b}")
    } else if template.contains(".alpha_per") {
        format!("{:.1}", (alpha / 255) * 100)
    } else if template.contains(".alpha") {
        format!("#{r:02x}{g:02x}{b:02x}{alpha:02x}")
    } else {
        format!("#{r:02x}{g:02x}{b:02x}")
    }
}

fn fill_template(
    template_name: &str,
    template: &str,
    colors: &(Vec<(u8, u8, u8)>, u8),
    wallpaper: &str,
    send: bool,
) {
    let output_path = format!(
        "{}/wal/{}",
        get_cache(send).to_string_lossy(),
        template_name
    );
    let alpha = colors.1;

    let mut result = template
        .replace("{wallpaper}", wallpaper)
        .replace("{alpha}", &format!("{}", (alpha / 255) * 100));

    if !colors.0.is_empty() {
        let background_patterns = [
            "{background.strip}",
            "{background.xrgba}",
            "{background.rgba}",
            "{background.rgb}",
            "{background.alpha}",
            "{background}",
        ];

        for pattern in background_patterns {
            let replacement = change(pattern, colors.0[0], alpha);
            result = result.replace(pattern, &replacement);
        }
    }

    if colors.0.len() > 7 {
        let foreground_patterns = [
            "{foreground.strip}",
            "{foreground.xrgba}",
            "{foreground.rgba}",
            "{foreground.rgb}",
            "{foreground.alpha}",
            "{foreground}",
        ];

        for pattern in foreground_patterns {
            let replacement = change(pattern, colors.0[7], alpha);
            result = result.replace(pattern, &replacement);
        }

        let cursor_patterns = [
            "{cursor.strip}",
            "{cursor.xrgba}",
            "{cursor.rgba}",
            "{cursor.rgb}",
            "{cursor.alpha}",
            "{cursor}",
        ];

        for pattern in cursor_patterns {
            let replacement = change(pattern, colors.0[7], alpha);
            result = result.replace(pattern, &replacement);
        }
    }

    for i in 0..colors.0.len().min(16) {
        let color = colors.0[i];

        let patterns = [
            format!("{{color{i}.strip}}"),
            format!("{{color{i}.xrgba}}"),
            format!("{{color{i}.rgba}}"),
            format!("{{color{i}.rgb}}"),
            format!("{{color{i}.alpha_dec}}"),
            format!("{{color{i}.alpha}}"),
            format!("{{color{i}}}"),
        ];

        for pattern in patterns {
            let replacement = change(&pattern, color, alpha);
            result = result.replace(&pattern, &replacement);
        }
    }

    if result.contains("{checksum}") {
        let checksum = colors
            .0
            .iter()
            .map(|(r, g, b)| format!("{r:02X}{g:02X}{b:02X}"))
            .collect::<Vec<String>>()
            .join("");
        result = result.replace("{checksum}", &checksum);
    }

    write(output_path, result).expect("Failed to write filled template");
}

pub fn create_template(colors: (Vec<(u8, u8, u8)>, u8), wallpaper: &str, send: bool) {
    let system_template_path = get_cache(send).join("walrs").join("templates");
    let user_template_path = format!("{}/walrs/templates/", get_config(send).to_string_lossy());
    let cache_path = format!("{}/wal/", get_cache(send).to_string_lossy());
    create_dir_all(&cache_path).unwrap_or_else(|_| {
        warning("Create", "can't create the cache folder", send);
        exit(1)
    });

    // Check if user templates directory exists and has templates
    let mut has_user_templates = false;
    if let Ok(entries) = fs::read_dir(&user_template_path) {
        for entry in entries.flatten() {
            if entry.path().is_file() {
                has_user_templates = true;
                let Ok(content) = read_to_string(entry.path()) else {
                    continue;
                };
                let Some(name) = entry.file_name().into_string().ok() else {
                    continue;
                };
                fill_template(&name, &content, &colors, wallpaper, send);
            }
        }
    }

    // If no user templates, copy from system templates
    if !has_user_templates {
        create_dir_all(&user_template_path).unwrap_or_else(|_| {
            warning("Create", "can't create user template path", send);
            exit(1)
        });

        if let Ok(entries) = fs::read_dir(system_template_path) {
            for entry in entries.flatten() {
                if entry.path().is_file() {
                    let Ok(content) = read_to_string(entry.path()) else {
                        continue;
                    };
                    let Some(name) = entry.file_name().into_string().ok() else {
                        continue;
                    };

                    // Copy template to user directory
                    let user_file_path = format!("{user_template_path}{name}");
                    let _ = write(&user_file_path, &content);

                    fill_template(&name, &content, &colors, wallpaper, send);
                }
            }
        }
    }
}
