use crate::utils::run;
use crate::utils::{get_absolute_path, info, warning};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::{exit, Command, Stdio};

fn spawn(cmd: &str) {
    let _ = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}

fn detect_de() -> String {
    for key in ["XDG_CURRENT_DESKTOP", "DESKTOP_SESSION"] {
        if let Ok(v) = env::var(key) {
            if !v.is_empty() {
                return v.to_lowercase();
            }
        }
    }
    String::new()
}

fn is_wayland() -> bool {
    env::var("WAYLAND_DISPLAY").is_ok()
}

fn de_commands(de: &str, path: &str) -> Option<Vec<String>> {
    let map: HashMap<&str, Vec<String>> = HashMap::from([
        ("gnome", vec![
            format!("gsettings set org.gnome.desktop.background picture-uri 'file://{path}'"),
            format!("gsettings set org.gnome.desktop.background picture-uri-dark 'file://{path}'"),
        ]),
        ("unity", vec![format!("gsettings set org.gnome.desktop.background picture-uri 'file://{path}'")]),
        ("budgie", vec![format!("gsettings set org.gnome.desktop.background picture-uri 'file://{path}'")]),
        ("mate", vec![format!("gsettings set org.mate.background picture-filename '{path}'")]),
        ("cinnamon", vec![format!("gsettings set org.cinnamon.desktop.background picture-uri 'file://{path}'")]),
        ("deepin", vec![format!("gsettings set com.deepin.wrap.gnome.desktop.background picture-uri 'file://{path}'")]),
        ("xfce", vec![format!("xfconf-query --channel xfce4-desktop --property /backdrop/screen0/monitor0/workspace0/last-image --set '{path}'")]),
        ("lxqt", vec![format!("pcmanfm-qt --set-wallpaper='{path}'")]),
        ("lxde", vec![format!("pcmanfm --set-wallpaper='{path}'")]),
        ("kde", vec![format!(
            r#"qdbus org.kde.plasmashell /PlasmaShell org.kde.PlasmaShell.evaluateScript "var d=desktops();for(i=0;i<d.length;i++){{d[i].wallpaperPlugin='org.kde.image';d[i].currentConfigGroup=Array('Wallpaper','org.kde.image','General');d[i].writeConfig('Image','{path}');}}""#
        )]),
    ]);

    for (key, cmds) in &map {
        if de.contains(key) {
            return Some(cmds.clone());
        }
    }
    None
}

fn generic_tools(wayland: bool, path: &str) -> Vec<(&'static str, String)> {
    if wayland {
        vec![
            ("swww", format!("swww-daemon >/dev/null 2>&1 & sleep 0.3; swww img '{path}'")),
            ("swaybg", format!("pkill swaybg; swaybg -i '{path}' -m fill &")),
            ("hyprpaper", format!("hyprpaper '{path}'")),
            ("wbg", format!("pkill wbg; wbg '{path}' &")),
        ]
    } else {
        vec![
            ("xwallpaper", format!("xwallpaper --zoom '{path}'")),
            ("feh", format!("feh --no-fehbg --bg-fill '{path}'")),
            ("hsetroot", format!("hsetroot -fill '{path}'")),
            ("nitrogen", format!("nitrogen --set-zoom-fill --save '{path}'")),
        ]
    }
}

pub fn change_wallpaper(img: &str, send: bool) {
    if !Path::new(img).is_file() {
        warning("Wallpaper", "invalid image path", send);
        exit(1);
    }

    let path = get_absolute_path(img).unwrap_or_else(|| img.to_string());
    let de = detect_de();

    if let Some(cmds) = de_commands(&de, &path) {
        for c in cmds {
            spawn(&c);
        }
        info("Wallpaper", &format!("wallpaper set for {de}"), send);
        return;
    }

    for (bin, cmd) in generic_tools(is_wayland(), &path) {
        if run(&format!("which {bin}")) {
            spawn(&cmd);
            info("Wallpaper", &format!("wallpaper set with {bin}"), send);
            return;
        }
    }

    warning("Wallpaper", "no suitable wallpaper tool found", send);
    exit(1);
}
