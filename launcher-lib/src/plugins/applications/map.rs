use core_lib::Warn;
use std::fs::{read_to_string, DirEntry};
use std::path::Path;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::Instant;
use tracing::{span, trace, Level};

#[derive(Debug, Clone)]
pub(super) struct DesktopEntry {
    pub(super) name: Box<str>,
    pub(super) icon: Option<Box<Path>>,
    pub(crate) keywords: Vec<Box<str>>,
    pub(crate) exec: Box<str>,
    pub(super) exec_path: Option<Box<Path>>,
    pub(super) terminal: bool,
    pub(super) source: Box<Path>,
}

fn get_desktop_file_map() -> &'static Mutex<Vec<DesktopEntry>> {
    static MAP_LOCK: OnceLock<Mutex<Vec<DesktopEntry>>> = OnceLock::new();
    MAP_LOCK.get_or_init(|| Mutex::new(Vec::new()))
}

pub(super) fn get_all_desktop_files<'a>() -> MutexGuard<'a, Vec<DesktopEntry>> {
    let map = get_desktop_file_map()
        .lock()
        .expect("Failed to lock desktop file map");
    map
}

pub fn reload_desktop_map(files: &Vec<DirEntry>) {
    let mut map = get_desktop_file_map()
        .lock()
        .expect("Failed to lock desktop file map");
    map.clear();
    fill_desktop_file_map(&mut map, files).warn("Failed to fill desktop file map");
}

fn fill_desktop_file_map(
    map: &mut Vec<DesktopEntry>,
    files: &Vec<DirEntry>,
) -> anyhow::Result<()> {
    let _span = span!(Level::TRACE, "fill_desktop_file_map").entered();

    let now = Instant::now();
    for entry in files {
        read_to_string(entry.path())
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
                let r#type = lines
                    .iter()
                    .find(|l| l.starts_with("Type="))
                    .map(|l| l.trim_start_matches("Type="));
                let exec = lines
                    .iter()
                    .find(|l| l.starts_with("Exec="))
                    .map(|l| l.trim_start_matches("Exec="));
                let keywords = lines
                    .iter()
                    .find(|l| l.starts_with("Keywords="))
                    .map(|l| l.trim_start_matches("Keywords="));
                let no_display = lines
                    .iter()
                    .find(|l| l.starts_with("NoDisplay="))
                    .map(|l| l.trim_start_matches("NoDisplay="))
                    .map(|l| l == "true");
                let exec_path = lines
                    .iter()
                    .find(|l| l.starts_with("Path="))
                    .and_then(|l| l.split('=').nth(1));
                let terminal = lines
                    .iter()
                    .find(|l| l.starts_with("Terminal="))
                    .map(|l| l.trim_start_matches("Terminal="))
                    .map(|l| l == "true")
                    .unwrap_or(false);
                if r#type == Some("Application") && no_display.is_none_or(|n| !n) {
                    if let (Some(name), Some(exec)) = (name, exec) {
                        let mut exec = String::from(exec);
                        for repl in &["%f", "%F", "%u", "%U"] {
                            if exec.contains(repl) {
                                exec = exec.replace(repl, "");
                            }
                        }
                        map.push(DesktopEntry {
                            name: name.trim().into(),
                            icon: icon.map(|p| Box::from(Path::new(p))),
                            keywords: keywords
                                .map(|k| k.split(';').map(|k| k.trim().into()).collect())
                                .unwrap_or_else(Vec::new),
                            exec: exec.trim().into(),
                            exec_path: exec_path.map(|p| Box::from(Path::new(p))),
                            terminal,
                            source: entry.path().into_boxed_path(),
                        });
                    }
                }
            })
            .warn(&format!("Failed to read file: {:?}", entry.path()));
    }
    trace!("filled icon map in {}ms", now.elapsed().as_millis());
    Ok(())
}