use core_lib::Warn;
use std::collections::HashMap;
use std::fs::DirEntry;
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;
use tracing::{span, trace, Level};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Source {
    DesktopFileName,
    DesktopFileStartupWmClass,
    DesktopFileExecName,
    ByPidExec,
}
type IconPathMap = HashMap<(Box<str>, Source), (Box<Path>, Box<Path>)>;

pub fn reload_desktop_map(files: &Vec<DirEntry>) {
    let mut map = get_icon_path_map().lock().expect("Failed to lock icon map");
    map.clear();
    fill_desktop_file_map(&mut map, files).warn("Failed to fill desktop file map");
}

fn fill_desktop_file_map(map: &mut IconPathMap, files: &Vec<DirEntry>) -> anyhow::Result<()> {
    let _span = span!(Level::TRACE, "fill_desktop_file_map").entered();

    let now = Instant::now();
    for entry in files {
        std::fs::read_to_string(entry.path())
            .map(|content| {
                let lines: Vec<&str> = content.lines().collect();
                let icon = lines
                    .iter()
                    .find(|l| l.starts_with("Icon="))
                    .map(|l| l.trim_start_matches("Icon="));

                let name = lines
                    .iter()
                    .find(|l| l.starts_with("Name="))
                    .map(|l| l.trim_start_matches("Name="));
                let exec_name = lines
                    .iter()
                    .find(|l| l.starts_with("Exec="))
                    .map(|l| l.trim_start_matches("Exec="))
                    .map(extract_exec_name)
                    .and_then(|l| l.split_whitespace().next())
                    .and_then(|l| l.split('/').next_back())
                    .map(|n| n.replace('"', ""));
                let startup_wm_class = lines
                    .iter()
                    .find(|l| l.starts_with("StartupWMClass="))
                    .map(|l| l.trim_start_matches("StartupWMClass="));

                if let (Some(name), Some(icon)) = (name, icon) {
                    map.insert(
                        (Box::from(name.to_lowercase()), Source::DesktopFileName),
                        (Box::from(Path::new(icon)), entry.path().into_boxed_path()),
                    );
                }
                if let (Some(startup_wm_class), Some(icon)) = (startup_wm_class, icon) {
                    map.insert(
                        (
                            Box::from(startup_wm_class.to_lowercase()),
                            Source::DesktopFileStartupWmClass,
                        ),
                        (Box::from(Path::new(icon)), entry.path().into_boxed_path()),
                    );
                }
                if let (Some(exec_name), Some(icon)) = (exec_name, icon) {
                    map.insert(
                        (
                            Box::from(exec_name.to_lowercase()),
                            Source::DesktopFileExecName,
                        ),
                        (Box::from(Path::new(icon)), entry.path().into_boxed_path()),
                    );
                }
            })
            .warn(&format!("Failed to read file: {:?}", entry.path()));
    }
    trace!("filled icon map in {}ms", now.elapsed().as_millis());
    Ok(())
}

fn extract_exec_name(l: &str) -> &str {
    // is a flatpak and isn't a PWA
    // (PWAs work out of the box by using the class = to the icon-name)
    // else chromium/chrome/etc would be detected as exec
    if l.contains("flatpak") && l.contains("--command") && !l.contains("--app-id") {
        // trim all text until --command
        l.split("--command=").last().unwrap_or(l)
    } else {
        l
    }
}

fn get_icon_path_map() -> &'static Mutex<IconPathMap> {
    static MAP_LOCK: OnceLock<Mutex<IconPathMap>> = OnceLock::new();
    MAP_LOCK.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn add_path_for_icon_by_pid_exec(class: &str, path: Box<Path>) {
    let mut map = get_icon_path_map().lock().expect("Failed to lock icon map");
    map.insert(
        (Box::from(class.to_ascii_lowercase()), Source::ByPidExec),
        (path, Box::from(Path::new(""))),
    );
}

pub fn get_icon_name_by_name(name: &str) -> Option<(Box<Path>, Box<Path>, Source)> {
    let map = get_icon_path_map().lock().expect("Failed to lock icon map");
    // prio: name by pid-exec, desktop file name, startup wm class, exec name
    map.get(&(Box::from(name.to_ascii_lowercase()), Source::ByPidExec))
        .map(|s| (s.0.clone(), s.1.clone(), Source::ByPidExec))
        .or_else(|| {
            map.get(&(
                Box::from(name.to_ascii_lowercase()),
                Source::DesktopFileName,
            ))
            .map(|s| (s.0.clone(), s.1.clone(), Source::DesktopFileName))
        })
        .or_else(|| {
            map.get(&(
                Box::from(name.to_ascii_lowercase()),
                Source::DesktopFileStartupWmClass,
            ))
            .map(|s| (s.0.clone(), s.1.clone(), Source::DesktopFileStartupWmClass))
        })
        .or_else(|| {
            map.get(&(
                Box::from(name.to_ascii_lowercase()),
                Source::DesktopFileExecName,
            ))
            .map(|s| (s.0.clone(), s.1.clone(), Source::DesktopFileExecName))
        })
}
