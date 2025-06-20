mod r#const;
mod exec;
mod helpers;
mod version;

pub use r#const::*;
pub use exec::*;
pub use helpers::*;
pub use version::*;

pub fn get_daemon_socket_path_buff() -> std::path::PathBuf {
    let mut buf = if let Some(runtime_path) = std::env::var_os("XDG_RUNTIME_DIR") {
        std::path::PathBuf::from(runtime_path)
    } else if let Ok(uid) = std::env::var("UID") {
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
        tracing::debug!("Checking if daemon is running");
        std::os::unix::net::UnixStream::connect(buf).is_ok()
    } else {
        tracing::debug!("Daemon not running");
        false
    }
}
