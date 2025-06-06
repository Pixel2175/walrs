use std::env;
use std::path::{Path, PathBuf};
use std::process::{exit, Stdio};
use std::{fs, process::Command};

pub fn image_path(image: Option<String>, send: bool) -> String {
    match image {
        Some(ref v) if Path::new(v).exists() => match get_absolute_path(v) {
            Some(p) => {
                if Path::new(&p).is_file() {
                    p
                } else {
                    std::str::from_utf8(
                        &std::process::Command::new("sh")
                            .arg("-c")
                            .arg(format!("find \"{p}\" -type f | sort -R | head -n1"))
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

pub fn run(command: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn print_colors(send: bool) {
    if send {
        if let Ok(output) = Command::new("bash")
            .arg("-c")
            .arg(r#"for i in {30..37} 90; do echo -en "\033[0;${i}m●\033[0m "; done; echo"#)
            .output()
        {
            if output.status.success() {
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }
        }
    }
}

pub fn warning(title: &str, message: &str, send: bool) {
    if send {
        println!("[\x1b[33mW\x1b[0m] \x1b[31m{title}:\x1b[0m {message}.");
    }
}

pub fn info(title: &str, message: &str, send: bool) {
    if send {
        println!("[\x1b[32mI\x1b[0m] \x1b[31m{title}:\x1b[0m {message}.");
    }
}

pub fn get_home(send: bool) -> PathBuf {
    env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| {
        warning("Home", "can't find the home dir", send);
        exit(1)
    })
}

pub fn get_config(send: bool) -> PathBuf {
    env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| get_home(send).join(".config"))
}
pub fn get_cache(send: bool) -> PathBuf {
    env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| get_home(send).join(".cache"))
}

pub fn get_absolute_path(path_str: &str) -> Option<String> {
    fs::canonicalize(Path::new(path_str))
        .ok()?
        .to_str()
        .map(|s| s.to_string())
}
