use crate::utils::{get_cache, get_config, info, run, share_files, warning};
use crate::wallpaper::change_wallpaper;
use std::fs::{create_dir_all, OpenOptions};
use std::fs::{read_dir, read_to_string};
use std::io::Write;
use std::path::Path;
use std::process::exit;

fn colors(colors: Vec<String>, send: bool) {
    for i in read_dir("/dev/pts/").unwrap_or_else(|_| {
        warning("Terminal", "Unable to find open terminals", send);
        exit(1)
    }) {
        let file = i.unwrap().file_name().into_string().unwrap();
        if file != "ptmx" && file.parse::<i32>().is_ok() {
            let special_index = [(10, 7), (11, 0), (12, 15), (708, 5)];
            let term = file.parse::<i32>().unwrap();
            for (i, value) in colors.iter().enumerate() {
                let sequence = format!("\x1b]4;{i};{value}\x1b\\");
                if let Ok(mut file) = OpenOptions::new()
                    .write(true)
                    .open(format!("/dev/pts/{term}"))
                {
                    file.write_all(sequence.as_bytes()).unwrap_or_else(|_| {
                        warning("Colors", "Can't apply terminal colors", send);
                    });
                };
            }
            if let Ok(mut file) = OpenOptions::new()
                .write(true)
                .open(format!("/dev/pts/{term}"))
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

// read ~/.cache/wal/wal file and return the wallpaper path
pub fn get_wallpaper(cache: &Path, send: bool) -> String {
    read_to_string(cache.join("wal"))
        .unwrap_or_else(|_| {
            warning("Wallpaper", "Can't find the wallpaper", send);
            exit(1)
        })
        .lines()
        .next()
        .unwrap()
        .trim()
        .to_string()
}

pub fn reload(send: bool, set_wal: bool, run_scripts: bool) {
    let cache = get_cache(send).join("wal");
    let walrs_cache = share_files();
    let file_path = cache.join("colors");

    // read the colors file and load all the colors
    let lines: Vec<String> = std::fs::read_to_string(&file_path)
        .unwrap_or_else(|_| {
            warning("Colors", "can't read colors", send);
            exit(1)
        })
        .lines()
        .map(|line| line.to_string())
        .collect();

    // applie the wallpaper
    if !set_wal {
        change_wallpaper(&get_wallpaper(&cache, send), send);
    }

    // change terminal colors
    colors(lines, send);

    if !run_scripts {
        // initial scripts files
        let scripts_dir = get_config(send).join("walrs").join("scripts");
        if !scripts_dir.exists() {
            match create_dir_all(&scripts_dir) {
                Ok(_) => {
                    run(&format!(
                        "cp -r {}/* {}",
                        walrs_cache.join("scripts").display(),
                        scripts_dir.display()
                    ));
                }
                Err(_) => return,
            }
        }

        // read the scripts directory and run them
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
    }
    info("Colors", "colorscheme applied successfully", send);
}
