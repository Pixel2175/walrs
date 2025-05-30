use std::fs::OpenOptions;
use std::fs::{read_dir, read_to_string};
use std::io::Write;
use std::process::{Command, Stdio};

use crate::wallpaper;

use crate::utils::{get_cache, info};

fn run(command: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn tty(cache: &str) {
    let path = format!("{}/wal/colors-tty.sh", cache);
    if Command::new("tty")
        .arg("-s")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        run(&format!("chmod +x {}", path));
        run(&path);
    }
}

fn xrdb(cache: &str, send: bool) {
    if run("which xrdb") {
        run(&format!(
            "xrdb -merge -quiet {}/wal/colors.Xresources",
            cache
        ));
        info("Xrdb", "xrdb colorscheme set", send);
    }
}

fn i3(send: bool) {
    if run("pgrep -x i3") {
        run("i3-msg reload");
        info("i3", "i3 colorscheme set", send);
    }
}

fn bspwm(send: bool) {
    if run("pgrep -x bspwm") {
        run("bspc wm -r");
        info("Bspwm", "bspwm colorscheme set", send);
    }
}

fn kitty(cache: &str, send: bool) {
    if run("which kitty") && run("pgrep kitty") {
        run(&format!(
            "kitty @ set-colors --all {}/wal/colors-kitty.conf",
            cache
        ));
        info("Kitty", "kitty colorscheme set", send);
    }
}

fn polybar(send: bool) {
    if run("which polybar") && run("pgrep polybar") {
        run("pkill -USR1 polybar");
        info("Polybar", "polybar colorscheme set", send);
    }
}

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

    // Spawn threads
    let cache = get_cache(send).to_string_lossy().to_string();

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
    xrdb(&cache, send);
    kitty(&cache, send);
    i3(send);
    bspwm(send);
    polybar(send);
    tty(&cache);
    info("Colors", "colorscheme applied successfully", send);
}
