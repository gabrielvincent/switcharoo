use crate::IniFile;
use std::fs::{DirEntry, read_to_string};
use tracing::{trace, warn};

pub fn get_default_desktop_file(mime: &str, mime_apps: &[DirEntry]) -> Option<Box<str>> {
    for entry in mime_apps {
        if let Ok(str) = read_to_string(entry.path()) {
            let ini = IniFile::parse(&str);
            let d_file = ini.get_section("Default Applications")?.get_boxed(mime)?;
            trace!("{mime} from mimeapps.list: {d_file}");
            return Some(d_file);
        } else {
            warn!("Failed to read file: {:?}", entry.path());
        }
    }
    None
}
