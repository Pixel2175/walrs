use std::fs::{create_dir_all, OpenOptions};
use std::fs::{read_dir, read_to_string};
use std::io::Write;

use crate::utils::{get_cache, get_home, info, run, warning};
use crate::wallpaper;

fn colors(colors: Vec<String>, send: bool) {
    for i in read_dir("/dev/pts/").expect("Can't load terminals") {
        let file = i.unwrap().file_name().into_string().unwrap();
        if file != "ptmx" && file.parse::<i32>().is_ok() {
            let special_index = [(10, 7), (11, 0), (12, 15), (708, 5)];
            let term = file.parse::<i32>().unwrap();
            for (i, value) in colors.iter().enumerate() {
                let sequence = format!("\x1b]4;{};{}\x1b\\", i, value);

                if let Ok(mut file) = OpenOptions::new()
                    .write(true)
                    .open(format!("/dev/pts/{}", term))
                {
                    let _ = file.write_all(sequence.as_bytes());
                };
            }
            if let Ok(mut file) = OpenOptions::new()
                .write(true)
                .open(format!("/dev/pts/{}", term))
            {
                for (i, v) in special_index {
                    let sequence = format!("\x1b]{};{}\x1b\\", i, colors[v]);

                    let _ = file.write_all(sequence.as_bytes());
                }
            }
        }
    }
    info("Terminal", "terminal colorscheme set", send);
}

pub fn reload(send: bool, set_wal: bool) {
    let cache = get_cache(send).to_string_lossy().to_string();
    let file_path = format!("{}/wal/colors", cache);

    let lines: Vec<String> = std::fs::read_to_string(&file_path)
        .expect("Can't load colors")
        .lines()
        .map(|line| line.to_string())
        .collect();

    if set_wal {
        let wallpaper = read_to_string(format!("{}/wal/wal", cache))
            .expect("run 'cp /etc/walrs/templates/wal ~/.config/walrs/templates/' and restart app")
            .lines()
            .next()
            .unwrap()
            .trim()
            .to_string();
        wallpaper::change_wallpaper(wallpaper.as_str(), send);
    }
    colors(lines, send);

    let scripts_dir = get_home(send).join(".config").join("walrs").join("scripts");

    if !scripts_dir.exists() {
        match create_dir_all(&scripts_dir) {
            Ok(_) => {
                run(&format!(
                    "cp /etc/walrs/scripts/* {}",
                    scripts_dir.to_string_lossy().to_string()
                ));
            }
            Err(_) => return,
        }
    }

    match read_dir(scripts_dir) {
        Ok(v) => {
            for scr in v {
                let script = scr.unwrap().path();
                if !script.is_file() {
                    continue;
                };
                if !run(&format!(
                    "bash {}",
                    &script.canonicalize().unwrap().to_string_lossy()
                )) {
                    warning(
                        "Script",
                        &format!(
                            "can't run {}",
                            script.file_name().unwrap().to_string_lossy()
                        ),
                        send,
                    );
                }
            }
            info("Scripts", "scripts runs successfully", send);
        }
        _ => return,
    }

    info("Colors", "colorscheme applied successfully", send);
}
