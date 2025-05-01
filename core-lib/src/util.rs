use anyhow::Context;
use semver::Version;
use std::fs::DirEntry;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::{env, fmt};
use tracing::{debug, trace, warn};

pub const MIN_VERSION: Version = Version::new(0, 42, 0);

pub const OVERVIEW_NAMESPACE: &str = "hyprshell_overview";
pub const LAUNCHER_NAMESPACE: &str = "hyprshell_launcher";

pub trait Warn<A> {
    fn warn(self, msg: &str) -> Option<A>;
}

impl<A> Warn<A> for Option<A> {
    fn warn(self, msg: &str) -> Option<A> {
        match self {
            Some(o) => Some(o),
            None => {
                warn!("{}", msg);
                None
            }
        }
    }
}

impl<A, E: fmt::Debug + fmt::Display> Warn<A> for Result<A, E> {
    fn warn(self, msg: &str) -> Option<A> {
        match self {
            Ok(o) => Some(o),
            Err(e) => {
                warn!("{}: {}", msg, e);
                debug!("{e:?}");
                None
            }
        }
    }
}

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
    let mut buf = if let Ok(runtime_path) = env::var("XDG_RUNTIME_DIR") {
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

pub fn check_version(version: anyhow::Result<String>) -> anyhow::Result<()> {
    if let Ok(version) = version {
        let parsed_version =
            Version::parse(&version).context("Unable to parse hyprland Version")?;
        if parsed_version.lt(&MIN_VERSION) {
            Err(anyhow::anyhow!(
                "hyprland version {} is too old or unknown, please update to at least {}",
                parsed_version,
                MIN_VERSION
            ))
        } else {
            Ok(())
        }
    } else {
        Err(anyhow::anyhow!("Unable to get hyprland version"))
    }
}

pub fn collect_desktop_files() -> Vec<DirEntry> {
    let mut res = Vec::new();
    for dir in find_application_dirs() {
        if !dir.exists() {
            continue;
        }
        match dir.read_dir() {
            Ok(dir) => {
                for entry in dir.flatten() {
                    let path = entry.path();
                    if path.is_file() && path.extension().is_some_and(|e| e == "desktop") {
                        res.push(entry);
                    }
                }
            }
            Err(e) => {
                warn!("Failed to read dir {dir:?}: {e}");
                continue;
            }
        }
    }
    debug!("found {} desktop files", res.len());
    res
}

fn find_application_dirs() -> Vec<PathBuf> {
    let mut dirs = env::var_os("XDG_DATA_DIRS")
        .map(|val| env::split_paths(&val).collect())
        .unwrap_or_else(|| {
            vec![
                PathBuf::from("/usr/local/share"),
                PathBuf::from("/usr/share"),
            ]
        });

    if let Some(data_home) = env::var_os("XDG_DATA_HOME").map(PathBuf::from).map_or_else(
        || {
            env::var_os("HOME")
                .map(|p| PathBuf::from(p).join(".local/share"))
                .or_else(|| {
                    warn!("No XDG_DATA_HOME and HOME environment variable found");
                    None
                })
        },
        Some,
    ) {
        dirs.push(data_home)
    }

    let dirs = dirs
        .into_iter()
        .map(|dir| dir.join("applications"))
        .collect();
    trace!("searching for icons in dirs: {:?}", dirs);
    dirs
}

pub fn generate_socat(echo: &str) -> String {
    format!(
        r#"echo '{}' | {} - UNIX-CONNECT:{}"#,
        echo,
        env!("SOCAT_PATH"),
        get_daemon_socket_path_buff().to_string_lossy()
    )
}
