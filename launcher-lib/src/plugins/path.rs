use crate::plugins::{Identifier, PluginNames, SortableLaunchOption};
use core_lib::{IniFile, WarnWithDetails, get_default_desktop_file};
use exec_lib::run::run_program;
use std::fs::{DirEntry, read_to_string};
use std::path::Path;
use std::sync::{Mutex, MutexGuard, OnceLock};
use tracing::{Level, debug, span, trace, warn};

pub(crate) fn get_path_options(matches: &mut Vec<SortableLaunchOption>, text: &str) {
    if text.starts_with("/") || text.starts_with("~") {
        let file_manager = get_file_manager_info();
        matches.push(SortableLaunchOption {
            icon: file_manager.icon.clone(),
            name: format!("open in {}", file_manager.name).into_boxed_str(),
            details: Box::from(""),
            details_long: None,
            score: 100,
            iden: Identifier {
                plugin: PluginNames::Path,
                identifier: None,
            },
            details_menu: vec![],
        });
    }
}
// inode/directory
pub fn launch_option(text: &str) -> bool {
    if text.is_empty() {
        debug!("No text to search for");
        return false;
    }

    debug!("Opening folder: {}", text);
    let file_manager = get_file_manager_info();
    let cmdline = if ["%u", "%U", "%f", "%F"]
        .iter()
        .any(|repl| file_manager.exec.contains(repl))
    {
        let mut exec = file_manager.exec.to_string();
        for repl in ["%u", "%U", "%f", "%F"] {
            exec = exec.replace(repl, text);
        }
        exec
    } else {
        format!("{} {}", file_manager.exec, text)
    };
    debug!("Launching file-manger: {}", cmdline);
    run_program(&cmdline, None, false, &None).warn_details("Failed to run program");
    true
}

pub struct FilemanagerData {
    pub exec: Box<str>,
    pub name: Box<str>,
    pub icon: Option<Box<Path>>,
}

pub(super) fn get_file_manager_info<'a>() -> MutexGuard<'a, FilemanagerData> {
    FILE_MANAGER_DATA
        .get()
        .expect("file-manager exec no initialized")
        .lock()
        .expect("Failed to lock file-manager exec")
}

static FILE_MANAGER_DATA: OnceLock<Mutex<FilemanagerData>> = OnceLock::new();

pub fn reload_default_file_manager(files: &[DirEntry], mime_apps: &[DirEntry]) {
    let _span = span!(Level::TRACE, "reload_default_file_manager").entered();
    let default_file_manager = get_default_desktop_file("inode/directory", mime_apps);

    for entry in files {
        if entry.file_name() == default_file_manager.as_deref().unwrap_or_default() {
            if let Ok(str) = read_to_string(entry.path()) {
                let ini = IniFile::parse(&str);
                if let Some(section) = ini.get_section("Desktop Entry") {
                    let exec = section.get("Exec");
                    let icon = section.get("Icon");
                    let name = section.get_boxed("Name").unwrap_or_default();
                    trace!("Found exec: {:?}, icon: {:?}", exec, icon);
                    if let Some(exec) = exec {
                        trace!(
                            "Found default file-manager file: {:?} with exec: {:?}",
                            entry.path(),
                            exec,
                        );
                        let _ = FILE_MANAGER_DATA.set(Mutex::new(FilemanagerData {
                            exec: Box::from(exec),
                            icon: icon.map(Path::new).map(Box::from),
                            name,
                        }));
                        return;
                    }
                } else {
                    warn!(
                        "Failed to find section 'Desktop Entry' in file: {:?}",
                        entry.path()
                    );
                }
            } else {
                warn!("Failed to read file: {:?}", entry.path());
            }
        }
    }
    warn!("No default file-manager found! (using nautilus)");
    let _ = FILE_MANAGER_DATA.set(Mutex::new(FilemanagerData {
        exec: Box::from(r#"nautilus --new-window %U"#),
        icon: Some(Box::from(Path::new("org.gnome.Nautilus"))),
        name: Box::from(r#"Nautilus"#),
    }));
}
