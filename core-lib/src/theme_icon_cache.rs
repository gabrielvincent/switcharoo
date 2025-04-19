// https://github.com/H3rmt/hyprshell/discussions/137#discussioncomment-12078216
use std::collections::BTreeSet;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use tracing::debug;

fn get_icon_map() -> &'static Mutex<BTreeSet<Box<str>>> {
    static MAP_LOCK: OnceLock<Mutex<BTreeSet<Box<str>>>> = OnceLock::new();
    MAP_LOCK.get_or_init(|| Mutex::new(BTreeSet::new()))
}

pub fn init_icon_map(icon_names: Vec<String>, search_path: Option<Vec<PathBuf>>) {
    let mut map = get_icon_map().lock().expect("Failed to lock icon map");

    debug!("found {} icons from theme", icon_names.len());
    for icon in icon_names {
        map.insert(icon.into_boxed_str());
    }

    if let Some(search_path) = search_path {
        // gtk4 only reports 500 icons for candy-theme, scan through the filesystem
        for path in search_path {
            if path.exists() {
                let paths = collect_files_recursive(&path);
                debug!("found {} icons from filesystem in {path:?}", paths.len());
                for icon in paths {
                    map.insert(icon);
                }
                return;
            }
        }
    }
}

pub fn theme_has_icon_name(name: &str) -> bool {
    let map = get_icon_map().lock().expect("Failed to lock icon map");
    map.contains(&Box::from(name))
}
fn collect_files_recursive(dir: &Path) -> Vec<Box<str>> {
    let mut files = Vec::new();
    let mut dirs_to_visit = vec![dir.to_path_buf()];

    while let Some(current_dir) = dirs_to_visit.pop() {
        if current_dir.is_dir() {
            if let Ok(entries) = read_dir(&current_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        dirs_to_visit.push(path);
                    } else {
                        files.push(
                            path.file_stem()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .into_owned()
                                .into_boxed_str(),
                        );
                    }
                }
            }
        }
    }

    files
}
