use crate::{IniFile, get_config_dirs, get_config_home};
use std::fs::{DirEntry, read_to_string};
use tracing::{debug, warn};

pub fn get_default_desktop_file(mime: &str) -> Option<Box<str>> {
    for entry in get_mimeapps() {
        // parse the mimeapps.list file
        if let Ok(str) = read_to_string(entry.path()) {
            let ini = IniFile::parse(&str);
            let d_file = ini.get_section("Default Applications")?.get_boxed(mime)?;
            debug!("{mime} from mimeapps.list: {d_file}");
            return Some(d_file);
        } else {
            warn!("Failed to read file: {:?}", entry.path());
        }
    }
    None
}

fn get_mimeapps() -> Vec<DirEntry> {
    let mut res = Vec::new();
    let mut dirs = get_config_dirs();
    dirs.push(get_config_home());
    for dir in dirs {
        if !dir.exists() {
            continue;
        }
        match dir.read_dir() {
            Ok(dir) => {
                for entry in dir.flatten() {
                    let path = entry.path();
                    if path.is_file()
                        && path
                            .file_name()
                            .is_some_and(|e| e.to_string_lossy().ends_with("mimeapps.list"))
                    {
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
    debug!("found {} mimeapps lists", res.len());
    res
}
