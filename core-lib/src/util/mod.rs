mod exec;
mod helpers;
mod transfer;
mod version;

pub use exec::*;
pub use helpers::*;
use std::env;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use tracing::debug;
pub use transfer::*;
pub use version::*;

pub const OVERVIEW_NAMESPACE: &str = "hyprshell_overview";
pub const LAUNCHER_NAMESPACE: &str = "hyprshell_launcher";

// from https://github.com/i3/i3/blob/next/i3-sensible-terminal
// shorted to only the most common ones that I know support -e option
pub const TERMINALS: [&str; 9] = [
    "alacritty",
    "kitty",
    "wezterm",
    "foot",
    "qterminal",
    "lilyterm",
    "tilix",
    "terminix",
    "konsole",
];

pub fn get_daemon_socket_path_buff() -> PathBuf {
    let mut buf = if let Some(runtime_path) = env::var_os("XDG_RUNTIME_DIR") {
        std::path::PathBuf::from(runtime_path)
    } else if let Ok(uid) = env::var("UID") {
        std::path::PathBuf::from("/run/user/".to_owned() + &uid)
    } else {
        std::path::PathBuf::from("/tmp")
    };
    #[cfg(debug_assertions)]
    buf.push("hyprshell.debug.sock");
    #[cfg(not(debug_assertions))]
    buf.push("hyprshell.sock");
    buf
}

pub fn daemon_running() -> bool {
    // check if socket exists and socket is open
    let buf = get_daemon_socket_path_buff();
    if buf.exists() {
        debug!("Checking if daemon is running");
        UnixStream::connect(buf).is_ok()
    } else {
        debug!("Daemon not running");
        false
    }
}
