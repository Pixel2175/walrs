use crate::utils::run;
use crate::utils::{get_absolute_path, info, warning};
use std::env;
use std::path::Path;
use std::process::{exit, Command, Stdio};

fn run_with_output(command: &str) -> Option<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stderr(Stdio::null())
        .output() // ✅ Remove stdout(Stdio::null()) to capture output
        .ok()?;

    if output.status.success() {
        String::from_utf8(output.stdout)
            .ok()
            .map(|s| s.trim().to_string())
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
    // Check specifically for Sway first
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

    // Check environment variables
    let keys = [
        "XDG_CURRENT_DESKTOP",
        "DESKTOP_SESSION",
        "GNOME_DESKTOP_SESSION_ID",
        "MATE_DESKTOP_SESSION_ID",
        "DESKTOP_STARTUP_ID",
        "WAYLAND_DISPLAY",
    ];

    for key in keys.iter() {
        if let Ok(val) = env::var(key) {
            if !val.is_empty() {
                if *key == "DESKTOP_STARTUP_ID" && val.contains("awesome") {
                    return Some("AWESOME".to_string());
                }

                if *key == "WAYLAND_DISPLAY" {
                    // Try to identify specific Wayland compositor
                    if let Ok(desktop) = env::var("XDG_CURRENT_DESKTOP") {
                        return Some(desktop);
                    } else {
                        return Some("WAYLAND".to_string());
                    }
                }

                return Some(val);
            }
        }
    }
    None
}

fn set_wm_wallpaper(img: &str, send: bool) {
    if run("which xwallpaper") {
        spawn(&format!("xwallpaper --zoom '{img}'"));
        info("Wallpaper", "wallpaper set with xwallpaper", send);
    } else if run("which feh") {
        spawn(&format!("feh --no-fehbg --bg-fill '{img}'"));
        info("Wallpaper", "wallpaper set with feh", send);
    } else if run("which hsetroot") {
        spawn(&format!("hsetroot -fill '{img}'"));
        info("Wallpaper", "wallpaper set with hsetroot", send);
    } else if run("which nitrogen") {
        spawn(&format!("nitrogen --set-zoom-fill --save '{img}'"));
        info("Wallpaper", "wallpaper set with nitrogen", send);
    } else if run("which xsetroot") {
        warning(
            "Wallpaper",
            "using xsetroot, but it does not support images properly",
            send,
        );
        spawn("xsetroot -solid '#000000'");
        info("Wallpaper", "set solid background with xsetroot", send);
    } else {
        warning("Wallpaper", "can't find any app to set wallpaper", send);
        exit(1)
    }
}

fn set_desktop_wallpaper(d: &str, img: &str, send: bool) {
    let abs_path = get_absolute_path(img).unwrap_or_else(|| {
        warning("Wallpaper", "failed to get absolute path", send);
        img.to_string()
    });
    if d.contains("hyprland") {
        // Hyprland-specific wallpaper handling
        if run("which swww") {
            if !run("pgrep -x swww-daemon") {
                spawn("swww-daemon &");
                // Give daemon time to start
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            spawn(&format!(
                "swww img '{abs_path}' --transition-type fade --transition-fps 60"
            ));
            info("Wallpaper", "wallpaper set with swww for Hyprland", send);
        } else if run("which hyprpaper") {
            spawn(&format!("hyprpaper '{abs_path}'"));
            info("Wallpaper", "wallpaper set with hyprpaper", send);
        } else if run("which swaybg") {
            spawn(&format!("pkill swaybg; swaybg -i '{abs_path}' -m fill &"));
            info("Wallpaper", "wallpaper set with swaybg for Hyprland", send);
        } else if run("which wbg") {
            spawn(&format!("pkill wbg; wbg '{abs_path}' &"));
            info("Wallpaper", "wallpaper set with wbg for Hyprland", send);
        } else {
            warning(
                "Wallpaper",
                "no suitable wallpaper tool found for Hyprland (try installing swww, hyprpaper, swaybg, or wbg)",
                send,
            );
            exit(1);
        }
    } else if d.contains("xfce") || d.contains("xubuntu") {
        // Try to find the active monitor
        let monitors = run_with_output("xfconf-query -c xfce4-desktop -l | grep last-image")
            .unwrap_or_default();
        if !monitors.is_empty() {
            for line in monitors.lines() {
                spawn(&format!(
                    "xfconf-query --channel xfce4-desktop --property {} --set '{}'",
                    line.trim(),
                    abs_path
                ));
            }
        } else {
            // Fallback to default monitor
            spawn(&format!(
                "xfconf-query --channel xfce4-desktop --property /backdrop/screen0/monitor0/workspace0/last-image --set '{abs_path}'"
            ));
        }
        info("Wallpaper", "wallpaper set with XFCE settings", send);
    } else if d.contains("gnome") || d.contains("unity") || d.contains("ubuntu") {
        // Check for GNOME version for compatibility
        if run("gsettings get org.gnome.desktop.background picture-uri-dark") {
            // GNOME 42+ with light/dark mode support
            spawn(&format!(
                "gsettings set org.gnome.desktop.background picture-uri 'file://{abs_path}'"
            ));
            spawn(&format!(
                "gsettings set org.gnome.desktop.background picture-uri-dark 'file://{abs_path}'"
            ));
        } else {
            // Older GNOME versions
            spawn(&format!(
                "gsettings set org.gnome.desktop.background picture-uri 'file://{abs_path}'"
            ));
        }
        info("Wallpaper", "wallpaper set with GNOME settings", send);
    } else if d.contains("mate") {
        spawn(&format!(
            "gsettings set org.mate.background picture-filename '{abs_path}'"
        ));
        info("Wallpaper", "wallpaper set with MATE settings", send);
    } else if d.contains("cinnamon") {
        spawn(&format!(
            "gsettings set org.cinnamon.desktop.background picture-uri 'file://{abs_path}'"
        ));
        info("Wallpaper", "wallpaper set with Cinnamon settings", send);
    } else if d.contains("sway") {
        if run("which swww") {
            if !run("pgrep -x swww-daemon") {
                spawn("swww-daemon &");
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            spawn(&format!(
                "swww img '{abs_path}' --transition-type fade --transition-fps 60"
            ));
            info("Wallpaper", "wallpaper set with swww for Sway", send);
        } else if run("which swaybg") {
            spawn(&format!("pkill swaybg; swaybg -i '{abs_path}' -m fill &"));
            info("Wallpaper", "wallpaper set with swaybg for Sway", send);
        } else {
            warning(
                "Wallpaper",
                "no suitable wallpaper tool found for Sway (try installing swww or swaybg)",
                send,
            );
            exit(1)
        }
    } else if d.contains("awesome") {
        spawn(&format!(
            "awesome-client \"require('gears').wallpaper.maximized('{abs_path}')\""
        ));
        info("Wallpaper", "wallpaper set with Awesome WM", send);
    } else if d.contains("kde") || d.contains("plasma") {
        let script = format!(
            r#"var allDesktops = desktops();for (i=0;i<allDesktops.length;i++){{d = allDesktops[i];d.wallpaperPlugin = "org.kde.image";d.currentConfigGroup = Array("Wallpaper", "org.kde.image", "General");d.writeConfig("Image", "{abs_path}");}}"#
        );
        spawn(&format!(
            "qdbus org.kde.plasmashell /PlasmaShell org.kde.PlasmaShell.evaluateScript \"{script}\""
        ));
        info("Wallpaper", "wallpaper set with KDE Plasma settings", send);
    } else if d.contains("i3") || d.contains("bspwm") || d.contains("qtile") {
        set_wm_wallpaper(&abs_path, send);
    } else if d.contains("wayland") {
        // Generic Wayland - try multiple approaches
        if run("which swww") {
            if !run("pgrep -x swww-daemon") {
                spawn("swww-daemon &");
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            spawn(&format!("swww img '{abs_path}'"));
            info("Wallpaper", "wallpaper set with swww", send);
        } else if run("which swaybg") {
            spawn(&format!("pkill swaybg; swaybg -i '{abs_path}' -m fill &"));
            info("Wallpaper", "wallpaper set with swaybg", send);
        } else if run("which wbg") {
            spawn(&format!("pkill wbg; wbg '{abs_path}' &"));
            info("Wallpaper", "wallpaper set with wbg", send);
        } else {
            warning(
                "Wallpaper",
                "no suitable Wayland wallpaper tool found (try installing swww, swaybg, or wbg)",
                send,
            );
            exit(1);
        }
    } else if d.contains("wayfire") {
        if run("which wbg") {
            spawn(&format!("pkill wbg; wbg '{abs_path}' &"));
            info("Wallpaper", "wallpaper set with wbg for Wayfire", send);
        } else if run("which swaybg") {
            spawn(&format!("pkill swaybg; swaybg -i '{abs_path}' -m fill &"));
            info("Wallpaper", "wallpaper set with swaybg for Wayfire", send);
        } else {
            warning(
                "Wallpaper",
                "no suitable wallpaper tool found for Wayfire (try installing wbg or swaybg)",
                send,
            );
            exit(1);
        }
    } else if d.contains("river") {
        if run("which wbg") {
            spawn(&format!("pkill wbg; wbg '{abs_path}' &"));
            info("Wallpaper", "wallpaper set with wbg for River", send);
        } else if run("which swaybg") {
            spawn(&format!("pkill swaybg; swaybg -i '{abs_path}' -m fill &"));
            info("Wallpaper", "wallpaper set with swaybg for River", send);
        } else {
            warning(
                "Wallpaper",
                "no suitable wallpaper tool found for River (try installing wbg or swaybg)",
                send,
            );
            exit(1);
        }
    } else if d.contains("fht") || d.contains("fht-compositor") {
        // fht-compositor support - it's a Wayland compositor
        if run("which swaybg") {
            spawn(&format!("pkill swaybg; swaybg -i '{abs_path}' -m fill &"));
            info(
                "Wallpaper",
                "wallpaper set with swaybg for fht-compositor",
                send,
            );
        } else if run("which wbg") {
            spawn(&format!("pkill wbg; wbg '{abs_path}' &"));
            info(
                "Wallpaper",
                "wallpaper set with wbg for fht-compositor",
                send,
            );
        } else if run("which swww") {
            if !run("pgrep -x swww-daemon") {
                spawn("swww-daemon &");
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            spawn(&format!("swww img '{abs_path}'"));
            info(
                "Wallpaper",
                "wallpaper set with swww for fht-compositor",
                send,
            );
        } else {
            warning(
                "Wallpaper",
                "no suitable wallpaper tool found for fht-compositor (try installing swaybg, wbg, or swww)",
                send,
            );
            exit(1);
        }
    } else if d.contains("deepin") {
        spawn(&format!(
            "gsettings set com.deepin.wrap.gnome.desktop.background picture-uri 'file://{abs_path}'"
        ));
        info("Wallpaper", "wallpaper set with Deepin settings", send);
    } else if d.contains("lxqt") {
        spawn(&format!("pcmanfm-qt --set-wallpaper='{abs_path}'"));
        info("Wallpaper", "wallpaper set with LXQt settings", send);
    } else if d.contains("lxde") {
        spawn(&format!("pcmanfm --set-wallpaper='{abs_path}'"));
        info("Wallpaper", "wallpaper set with LXDE settings", send);
    } else if d.contains("budgie") {
        spawn(&format!(
            "gsettings set org.gnome.desktop.background picture-uri 'file://{abs_path}'"
        ));
        info(
            "Wallpaper",
            "wallpaper set with Budgie (GNOME) settings",
            send,
        );
    } else if d.contains("enlightenment") || d.contains("e17") || d.contains("e16") {
        if run("which enlightenment_remote") {
            spawn(&format!(
                "enlightenment_remote -desktop-bg-add 0 0 0 0 '{abs_path}'"
            ));
            info("Wallpaper", "wallpaper set with Enlightenment", send);
        } else {
            set_wm_wallpaper(&abs_path, send);
        }
    } else {
        // Default to the generic wallpaper setters
        info(
            "Wallpaper",
            &format!("Unknown desktop environment: {}, trying generic tools", d),
            send,
        );
        set_wm_wallpaper(&abs_path, send);
    }
}

pub fn change_wallpaper(img: &str, send: bool) {
    if !Path::new(img).is_file() {
        warning("Wallpaper", "invalid image path", send);
        exit(1)
    }

    match get_desktop_env() {
        Some(d) => {
            if send {
                info(
                    "Desktop",
                    &format!("Detected desktop environment: {}", d),
                    send,
                );
            }
            set_desktop_wallpaper(&d.to_lowercase(), img, send);
        }
        None => {
            if send {
                warning(
                    "Desktop",
                    "Could not detect desktop environment, using generic tools",
                    send,
                );
            }
            set_wm_wallpaper(img, send);
        }
    }
}
