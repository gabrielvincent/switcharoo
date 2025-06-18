use crate::transfer::TransferType;
use std::env;
use std::env::split_paths;
use std::ffi::OsString;

pub fn get_hyprshell_path() -> String {
    env::current_exe()
        .expect("Current executable not found")
        .display()
        .to_string()
        .replace("(deleted)", "")
}

pub fn get_hyprctl_path() -> String {
    let path =
        env::var_os("PATH").unwrap_or_else(|| OsString::from("/usr/bin:/bin:/usr/local/bin"));

    split_paths(&path)
        .find_map(|dir| {
            let path = dir.join("hyprctl");
            if path.exists() {
                Some(path.display().to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| String::from("hyprctl"))
}

pub fn generate_transfer_socat(transfer: &TransferType) -> String {
    format!(
        r#"{} socat '{}'"#,
        get_hyprshell_path(),
        serde_json::to_string(transfer).expect("serialize transfer")
    )
}
