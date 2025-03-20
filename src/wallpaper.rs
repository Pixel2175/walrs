use std::env;
use std::path::Path;
use std::process::{Command, Stdio};
use crate::utils::{info, warning};

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

fn run_with_output(command: &str) -> Option<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .output()
        .ok()?;
    
    if output.status.success() {
        String::from_utf8(output.stdout).ok()
    } else {
        None
    }
}

fn spawn(command: &str) {
    let _ = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn();
}

fn get_desktop_env() -> Option<String> {
    let keys = [
        "XDG_CURRENT_DESKTOP",
        "DESKTOP_SESSION",
        "GNOME_DESKTOP_SESSION_ID",
        "MATE_DESKTOP_SESSION_ID",
        "SWAYSOCK",
        "HYPRLAND_INSTANCE_SIGNATURE",
        "DESKTOP_STARTUP_ID",
        "I3SOCK",
        "WAYLAND_DISPLAY",
    ];
    
    // Check specifically for Sway
    if env::var("SWAYSOCK").is_ok() || run("pgrep -x sway") {
        return Some("SWAY".to_string());
    }
    
    // Check if Hyprland is running
    if run("pgrep -x Hyprland") || env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
        return Some("HYPRLAND".to_string());
    }
    
    // Check if i3 is running
    if run("pgrep -x i3") || env::var("I3SOCK").is_ok() {
        return Some("I3".to_string());
    }

    // Check if bspwm is running
    if run("pgrep -x bspwm") {
        return Some("BSPWM".to_string());
    }

    // Check if qtile is running
    if run("pgrep -x qtile") {
        return Some("QTILE".to_string());
    }

    // Check for other environment variables
    for key in keys.iter() {
        if let Ok(val) = env::var(key) {
            if !val.is_empty() {
                if *key == "DESKTOP_STARTUP_ID" && val.contains("awesome") {
                    return Some("AWESOME".to_string());
                }
                if *key == "WAYLAND_DISPLAY" && env::var("XDG_CURRENT_DESKTOP").is_err() {
                    // Generic Wayland session without specific DE identifier
                    return Some("WAYLAND".to_string());
                }
                // Don't return SWAYSOCK here, as we've already checked for it
                if *key != "SWAYSOCK" {
                    return Some(val);
                }
            }
        }
    }
    None
}

fn set_wm_wallpaper(img: &str, send: bool) {
    if run("which feh") {
        spawn(&format!("feh --no-fehbg --bg-fill '{}'", img));
        info("Wallpaper", "wallpaper set with feh", send);
    } else if run("which xwallpaper") {
        spawn(&format!("xwallpaper --zoom '{}'", img));
        info("Wallpaper", "wallpaper set with xwallpaper", send);
    } else if run("which hsetroot") {
        spawn(&format!("hsetroot -fill '{}'", img));
        info("Wallpaper", "wallpaper set with hsetroot", send);
    } else if run("which nitrogen") {
        spawn(&format!("nitrogen --set-zoom-fill --save '{}'", img));
        info("Wallpaper", "wallpaper set with nitrogen", send);
    } else if run("which xsetroot") {
        warning("Wallpaper", "using xsetroot, but it does not support images properly", send);
        spawn(&format!("xsetroot -solid '#000000'"));
        info("Wallpaper", "set solid background with xsetroot", send);
    } else {
        warning("Wallpaper", "can't find any app to set wallpaper", send);
    }
}

fn set_desktop_wallpaper(desktop: &str, img: &str, send: bool) {
    let d = desktop.to_lowercase();
    
    // Convert img to absolute path
    let abs_path = if Path::new(img).is_absolute() {
        img.to_string()
    } else {
        match std::fs::canonicalize(img) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => {
                warning("Wallpaper", "failed to get absolute path", send);
                img.to_string()
            }
        }
    };
    
    if d.contains("xfce") || d.contains("xubuntu") {
        // Try to find the active monitor
        let monitors = run_with_output("xfconf-query -c xfce4-desktop -l | grep last-image").unwrap_or_default();
        if !monitors.is_empty() {
            for line in monitors.lines() {
                spawn(&format!(
                    "xfconf-query --channel xfce4-desktop --property {} --set '{}'",
                    line.trim(), abs_path
                ));
            }
        } else {
            // Fallback to default monitor
            spawn(&format!(
                "xfconf-query --channel xfce4-desktop --property /backdrop/screen0/monitor0/workspace0/last-image --set '{}'",
                abs_path
            ));
        }
        info("Wallpaper", "wallpaper set with XFCE settings", send);
    } else if d.contains("gnome") || d.contains("unity") || d.contains("ubuntu") {
        // Check for GNOME version for compatibility
        if run("gsettings get org.gnome.desktop.background picture-uri-dark") {
            // GNOME 42+ with light/dark mode support
            spawn(&format!(
                "gsettings set org.gnome.desktop.background picture-uri 'file://{}'",
                abs_path
            ));
            spawn(&format!(
                "gsettings set org.gnome.desktop.background picture-uri-dark 'file://{}'",
                abs_path
            ));
        } else {
            // Older GNOME versions
            spawn(&format!(
                "gsettings set org.gnome.desktop.background picture-uri 'file://{}'",
                abs_path
            ));
        }
        info("Wallpaper", "wallpaper set with GNOME settings", send);
    } else if d.contains("mate") {
        spawn(&format!(
            "gsettings set org.mate.background picture-filename '{}'",
            abs_path
        ));
        info("Wallpaper", "wallpaper set with MATE settings", send);
    } else if d.contains("cinnamon") {
        spawn(&format!(
            "gsettings set org.cinnamon.desktop.background picture-uri 'file://{}'",
            abs_path
        ));
        info("Wallpaper", "wallpaper set with Cinnamon settings", send);
    } else if d.contains("hyprland") {
        // Try to determine the monitor configuration
        let monitors = run_with_output("hyprctl monitors -j | jq -r '.[].name'").unwrap_or_default();
        if !monitors.is_empty() {
            // Set for each monitor
            for monitor in monitors.lines() {
                spawn(&format!("hyprctl hyprpaper wallpaper \"{},{}\"", monitor.trim(), abs_path));
            }
        } else {
            // No monitors detected, try generic approach
            if run("which hyprpaper") {
                // Using hyprpaper
                spawn(&format!("echo 'preload = {}\nwallpaper = ,{}' >> ~/.config/hypr/hyprpaper.conf", abs_path, abs_path));
                //spawn(&format!("echo 'wallpaper = ,{}' >> ~/.config/hypr/hyprpaper.conf", abs_path));
                spawn("killall hyprpaper; hyprpaper &");
                info("Wallpaper", "wallpaper set with hyprpaper", send);
            } else {
                // Using swaybg as fallback for Hyprland
                spawn(&format!("pkill swaybg; swaybg -i '{}' -m fill &", abs_path));
                info("Wallpaper", "wallpaper set with swaybg for Hyprland", send);
            }
        }
    } else if d == "sway" {
        // Using swww or swaybg for Sway
        if run("which swww") {
            // Initialize swww if not running
            if !run("pgrep -x swww-daemon") {
                spawn("swww init");
            }
            spawn(&format!("swww img '{}' --transition-type fade --transition-fps 60", abs_path));
            info("Wallpaper", "wallpaper set with swww for Sway", send);
        } else if run("which swaybg") {
            spawn(&format!("pkill swaybg; swaybg -i '{}' -m fill &", abs_path));
            info("Wallpaper", "wallpaper set with swaybg for Sway", send);
        } else {
            warning("Wallpaper", "no suitable wallpaper tool found for Sway", send);
        }
    } else if d.contains("awesome") {
        spawn(&format!(
            "awesome-client \"require('gears').wallpaper.maximized('{}')\"",
            abs_path
        ));
        info("Wallpaper", "wallpaper set with Awesome WM", send);
    } else if d.contains("kde") || d.contains("plasma") {
        let script = format!(
            r#"var allDesktops = desktops();for (i=0;i<allDesktops.length;i++){{d = allDesktops[i];d.wallpaperPlugin = "org.kde.image";d.currentConfigGroup = Array("Wallpaper", "org.kde.image", "General");d.writeConfig("Image", "{}");}}"#,
            abs_path
        );
        spawn(&format!(
            "qdbus org.kde.plasmashell /PlasmaShell org.kde.PlasmaShell.evaluateScript \"{}\"",
            script
        ));
        info("Wallpaper", "wallpaper set with KDE Plasma settings", send);
    } else if d.contains("i3") {
        // i3 support
        if run("which feh") {
            spawn(&format!("feh --no-fehbg --bg-fill '{}'", abs_path));
            info("Wallpaper", "wallpaper set with feh for i3", send);
        } else {
            set_wm_wallpaper(&abs_path, send);
        }
    } else if d.contains("bspwm") {
        // bspwm support
        if run("which feh") {
            spawn(&format!("feh --no-fehbg --bg-fill '{}'", abs_path));
            info("Wallpaper", "wallpaper set with feh for bspwm", send);
        } else {
            set_wm_wallpaper(&abs_path, send);
        }
    } else if d.contains("qtile") {
        // qtile support - uses typical WM wallpaper setters
        set_wm_wallpaper(&abs_path, send);
    } else if d.contains("wayland") {
        // Generic Wayland - try multiple approaches
        if run("which swaybg") {
            spawn(&format!("pkill swaybg; swaybg -i '{}' -m fill &", abs_path));
            info("Wallpaper", "wallpaper set with swaybg", send);
        } else if run("which wbg") {
            spawn(&format!("wbg '{}'", abs_path));
            info("Wallpaper", "wallpaper set with wbg", send);
        } else if run("which swww") {
            spawn(&format!("swww img '{}'", abs_path));
            info("Wallpaper", "wallpaper set with swww", send);
        } else {
            warning("Wallpaper", "no suitable Wayland wallpaper tool found", send);
        }
    } else if d.contains("wayfire") {
        // Wayfire compositor
        if run("which wbg") {
            spawn(&format!("wbg '{}'", abs_path));
            info("Wallpaper", "wallpaper set with wbg for Wayfire", send);
        } else {
            spawn(&format!("pkill swaybg; swaybg -i '{}' -m fill &", abs_path));
            info("Wallpaper", "wallpaper set with swaybg for Wayfire", send);
        }
    } else if d.contains("deepin") {
        spawn(&format!(
            "gsettings set com.deepin.wrap.gnome.desktop.background picture-uri 'file://{}'",
            abs_path
        ));
        info("Wallpaper", "wallpaper set with Deepin settings", send);
    } else if d.contains("lxqt") {
        // LXQt uses pcmanfm-qt for desktop management
        spawn(&format!(
            "pcmanfm-qt --set-wallpaper='{}'",
            abs_path
        ));
        info("Wallpaper", "wallpaper set with LXQt settings", send);
    } else if d.contains("lxde") {
        // LXDE uses pcmanfm for desktop management
        spawn(&format!(
            "pcmanfm --set-wallpaper='{}'",
            abs_path
        ));
        info("Wallpaper", "wallpaper set with LXDE settings", send);
    } else if d.contains("budgie") {
        // Budgie uses GNOME settings
        spawn(&format!(
            "gsettings set org.gnome.desktop.background picture-uri 'file://{}'",
            abs_path
        ));
        info("Wallpaper", "wallpaper set with Budgie (GNOME) settings", send);
    } else if d.contains("enlightenment") || d.contains("e17") || d.contains("e16") {
        // Enlightenment WM - try using enlightenment_remote
        if run("which enlightenment_remote") {
            spawn(&format!(
                "enlightenment_remote -desktop-bg-add 0 0 0 0 '{}'",
                abs_path
            ));
            info("Wallpaper", "wallpaper set with Enlightenment", send);
        } else {
            set_wm_wallpaper(&abs_path, send);
        }
    } else {
        // Default to the generic wallpaper setters
        set_wm_wallpaper(&abs_path, send);
    }
}

pub fn change_wallpaper(img: &str, send: bool) {
    if !Path::new(img).is_file() {
        warning("Wallpaper", "invalid image path", send);
        return;
    }
    
    match get_desktop_env() {
        Some(d) => {
            info("Wallpaper", &format!("detected environment: {}", d), send);
            set_desktop_wallpaper(&d, img, send);
        },
        None => {
            set_wm_wallpaper(img, send);
        },
    }
}
