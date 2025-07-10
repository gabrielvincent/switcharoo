use crate::IniFile;
use std::fs::{DirEntry, read_to_string};
use tracing::warn;

pub fn get_default_desktop_file(mime: &str, mime_apps: &[DirEntry]) -> Option<Box<str>> {
    for entry in mime_apps {
        if let Ok(str) = read_to_string(entry.path()) {
            let ini = IniFile::from_str(&str);
            let default = ini.get_section("Default Applications").and_then(|section| {
                section.get_first_as_boxed(mime).or_else(|| {
                    ini.get_section("Added Associations")
                        .and_then(|section| section.get_first_as_boxed(mime))
                })
            });
            return default;
        } else {
            warn!("Failed to read file: {:?}", entry.path());
        }
    }
    None
}
