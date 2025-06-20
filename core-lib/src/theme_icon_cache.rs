// https://github.com/H3rmt/hyprshell/discussions/137#discussioncomment-12078216

use std::collections::BTreeSet;
use std::env;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::Instant;
use tracing::{Level, debug, span};

fn get_icon_map() -> &'static Mutex<BTreeSet<Box<str>>> {
    static MAP_LOCK: OnceLock<Mutex<BTreeSet<Box<str>>>> = OnceLock::new();
    MAP_LOCK.get_or_init(|| Mutex::new(BTreeSet::new()))
}

pub fn init_icon_map(icon_names: Vec<String>, search_path: Vec<PathBuf>, in_background: bool) {
    let _span = span!(Level::TRACE, "init_icon_map").entered();
    let mut map = get_icon_map().lock().expect("Failed to lock icon map");
    let instant = Instant::now();

    debug!("found {} icons from theme", icon_names.len());
    for icon in icon_names {
        map.insert(icon.into_boxed_str());
    }
    drop(map);

    // gtk4 only reports 500 icons for candy-theme, scan through the filesystem
    if env::var_os("HYPRSHELL_NO_ALL_ICONS").is_none() {
        for path in search_path {
            if path.exists() {
                if in_background {
                    std::thread::spawn(move || {
                        let paths = collect_unique_files_recursive(&path);
                        debug!(
                            "found {} icons from filesystem in {path:?} paths (in background)",
                            paths.len()
                        );
                        let mut map2 = get_icon_map().lock().expect("Failed to lock icon map");
                        map2.extend(paths);
                        drop(map2)
                    });
                } else {
                    let paths = collect_unique_files_recursive(&path);
                    debug!(
                        "found {} icons from filesystem in {path:?} paths",
                        paths.len()
                    );
                    let mut map2 = get_icon_map().lock().expect("Failed to lock icon map");
                    map2.extend(paths);
                    drop(map2)
                }
            }
        }
    }
    debug!("icon map filled in {:?}", instant.elapsed());
}

pub fn get_all_icons<'a>() -> MutexGuard<'a, BTreeSet<Box<str>>> {
    get_icon_map().lock().expect("Failed to lock icon map")
}

pub fn theme_has_icon_name(name: &str) -> bool {
    let map = get_icon_map().lock().expect("Failed to lock icon map");
    map.contains(&Box::from(name))
}
fn collect_unique_files_recursive(dir: &Path) -> BTreeSet<Box<str>> {
    let mut names = BTreeSet::new();
    let mut dirs_to_visit = vec![dir.to_path_buf()];

    while let Some(current_dir) = dirs_to_visit.pop() {
        if current_dir.is_dir() {
            if let Ok(entries) = read_dir(&current_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        dirs_to_visit.push(path);
                    } else if let Some(name_osstr) = path.file_stem() {
                        // Avoid allocation unless needed
                        let name = name_osstr.to_string_lossy();
                        if !name.is_empty() && !names.contains(&*name) {
                            names.insert(name.into_owned().into_boxed_str());
                        }
                    }
                }
            }
        }
    }
    names
}
