use crate::utils::{get_cache_folder, get_config_folder};
use std::{
    fs::{self, create_dir_all, read_to_string, write},
    path::Path,
};

fn change(template: &str, colors: (u8, u8, u8), alpha: u8) -> String {
    let (r, g, b) = colors;

    if template.contains(".strip") {
        format!("{:02x}{:02x}{:02x}", r, g, b)
    } else if template.contains(".xrgba") {
        format!("{:02x}/{:02x}/{:02x}/{:02x}", r, g, b, alpha)
    } else if template.contains(".rgba") {
        format!("{},{},{},{}", r, g, b, alpha)
    } else if template.contains(".rgb") {
        format!("{},{},{}", r, g, b)
    } else if template.contains(".alpha_per") {
        format!("{:.1}", (alpha / 255) * 100)
    } else if template.contains(".alpha") {
        format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, alpha)
    } else {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }
}

fn fill_template(
    template_name: &str,
    template: &str,
    colors: &(Vec<(u8, u8, u8)>, u8),
    wallpaper: &str,
    output_dir: String,
) {
    let output_path = if output_dir == "None" {
        format!(
            "{}/wal/{}",
            get_cache_folder().expect("Can't find cache folder"),
            template_name
        )
    } else {
        Path::new(&output_dir)
            .join(template_name)
            .to_string_lossy()
            .to_string()
    };
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
            format!("{{color{}.strip}}", i),
            format!("{{color{}.xrgba}}", i),
            format!("{{color{}.rgba}}", i),
            format!("{{color{}.rgb}}", i),
            format!("{{color{}.alpha_dec}}", i),
            format!("{{color{}.alpha}}", i),
            format!("{{color{}}}", i),
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
            .map(|(r, g, b)| format!("{:02X}{:02X}{:02X}", r, g, b))
            .collect::<Vec<String>>()
            .join("");
        result = result.replace("{checksum}", &checksum);
    }

    write(output_path, result).expect("Failed to write filled template");
}

pub fn create_template(colors: (Vec<(u8, u8, u8)>, u8), wallpaper: &str, output_dir: String) {
    let system_template_path = "/etc/walrs/templates/";
    let user_template_path = format!("{}/walrs/templates/", get_config_folder().unwrap());
    let cache_path = format!("{}/wal/", get_cache_folder().unwrap());
    let _ = create_dir_all(&cache_path);

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
                fill_template(&name, &content, &colors, wallpaper, output_dir.clone());
            }
        }
    }

    // If no user templates, copy from system templates
    if !has_user_templates {
        let _ = create_dir_all(&user_template_path);

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
                    let user_file_path = format!("{}{}", user_template_path, name);
                    let _ = write(&user_file_path, &content);

                    fill_template(&name, &content, &colors, wallpaper, output_dir.clone());
                }
            }
        }
    }
}
