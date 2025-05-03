use clap::CommandFactory;
use clap_complete::{Shell, generate};
use dirs_next::home_dir;
use std::fs;
use std::path::PathBuf;

pub fn install_completions() -> std::io::Result<()> {
    let shell = detect_shell();
    let mut cmd = crate::Arg::command();
    let app_name = cmd.get_name().to_string();

    let (target_dir, completion_file) = match shell {
        Shell::Bash => (
            home_dir().unwrap().join(".bash_completion.d"),
            PathBuf::from(app_name.clone()),
        ),
        Shell::Zsh => (
            home_dir().unwrap().join(".zsh/completions"),
            PathBuf::from(format!("_{}", app_name)),
        ),
        Shell::Fish => (
            home_dir().unwrap().join(".config/fish/completions"),
            PathBuf::from(format!("{}.fish", app_name)),
        ),
        Shell::PowerShell => (
            home_dir().unwrap().join("Documents/PowerShell/Modules"),
            PathBuf::from(format!("{}.ps1", app_name)),
        ),
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Unsupported shell",
            ));
        }
    };

    fs::create_dir_all(&target_dir)?;

    let mut file = fs::File::create(target_dir.join(completion_file))?;
    generate(shell, &mut cmd, app_name, &mut file);

    Ok(())
}

fn detect_shell() -> Shell {
    std::env::var("SHELL")
        .ok()
        .and_then(|shell| {
            if shell.contains("zsh") {
                Some(Shell::Zsh)
            } else if shell.contains("bash") {
                Some(Shell::Bash)
            } else if shell.contains("fish") {
                Some(Shell::Fish)
            } else {
                None
            }
        })
        .unwrap_or_else(|| {
            if cfg!(windows) {
                Shell::PowerShell
            } else {
                // Default to bash if detection fails
                Shell::Bash
            }
        })
}
