use std::env;
use std::path::PathBuf;

pub fn get_default_config_path() -> PathBuf {
    let mut path = get_config_dir();
    path.push("hyprshell/config.ron");
    path
}

pub fn get_default_css_path() -> PathBuf {
    let mut path = get_config_dir();
    path.push("hyprshell/styles.css");
    path
}

pub fn get_default_data_dir() -> PathBuf {
    let mut path = get_data_dir();
    path.push("hyprshell");
    path
}

pub fn get_data_dir() -> PathBuf {
    env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            env::var_os("HOME")
                .map(|home| PathBuf::from(format!("{}/.local/share", home.to_string_lossy())))
        })
        .expect("Failed to get config dir (XDG_DATA_HOME or HOME not set)")
}

fn get_config_dir() -> PathBuf {
    env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            env::var_os("HOME")
                .map(|home| PathBuf::from(format!("{}/.config", home.to_string_lossy())))
        })
        .expect("Failed to get config dir (XDG_CONFIG_HOME or HOME not set)")
}
