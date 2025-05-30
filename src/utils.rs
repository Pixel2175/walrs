use std::env;
use std::path::{Path, PathBuf};
use std::process::{exit, Stdio};
use std::{fs, process::Command};

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
        exit(1)
    }
}

pub fn info(title: &str, message: &str, send: bool) {
    if send {
        println!("[\x1b[32mI\x1b[0m] \x1b[31m{title}:\x1b[0m {message}.");
    }
}

pub fn get_home(send: bool) -> PathBuf {
    env::home_dir().unwrap_or_else(|| {
        warning("Home", "can't find the home dir", send);
        exit(1)
    })
}
pub fn get_config(send: bool) -> PathBuf {
    get_home(send).join(".config")
}
pub fn get_cache(send: bool) -> PathBuf {
    get_home(send).join(".cache")
}

pub fn get_absolute_path(path_str: &str) -> Option<String> {
    let path = Path::new(path_str);
    fs::canonicalize(path).ok()?.to_str().map(|s| s.to_string())
}
