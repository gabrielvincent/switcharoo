use core_lib::{IniFile, get_default_desktop_file};
use std::fs::{DirEntry, read_to_string};
use std::path::Path;
use std::sync::{Mutex, MutexGuard, OnceLock};
use tracing::{Level, span, trace, warn};

pub struct BrowserData {
    pub exec: Box<str>,
    pub startup_wm_class: Option<Box<str>>,
    pub icon: Option<Box<Path>>,
}

pub(super) fn get_browser_info<'a>() -> MutexGuard<'a, BrowserData> {
    BROWSER_EXEC
        .get()
        .expect("browser exec no initialized")
        .lock()
        .expect("Failed to lock browser exec")
}

static BROWSER_EXEC: OnceLock<Mutex<BrowserData>> = OnceLock::new();

pub fn reload_default_browser(files: &[DirEntry], mime_apps: &[DirEntry]) {
    let _span = span!(Level::TRACE, "reload_default_browser").entered();
    let default_browser = get_default_desktop_file("x-scheme-handler/https", mime_apps);

    for entry in files {
        if entry.file_name() == default_browser.as_deref().unwrap_or_default() {
            if let Ok(str) = read_to_string(entry.path()) {
                let ini = IniFile::parse(&str);
                if let Some(section) = ini.get_section("Desktop Entry") {
                    let exec = section.get("Exec");
                    let startup_wm_class = section.get("StartupWMClass");
                    let icon = section.get("Icon");
                    trace!(
                        "Found exec: {:?}, startup_wm_class: {:?}, icon: {:?}",
                        exec, startup_wm_class, icon
                    );
                    if let Some(exec) = exec {
                        trace!(
                            "Found default browser file: {:?} with exec: {:?} and startup_wm_class: {:?}",
                            entry.path(),
                            exec,
                            startup_wm_class
                        );
                        let _ = BROWSER_EXEC.set(Mutex::new(BrowserData {
                            exec: Box::from(exec),
                            startup_wm_class: startup_wm_class.map(Box::from),
                            icon: icon.map(Path::new).map(Box::from),
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
    warn!("No default browser found! (using firefox and gdbus to open)");
    let _ = BROWSER_EXEC.set(Mutex::new(BrowserData {
        exec: Box::from(r#"gdbus call --session --dest="org.freedesktop.portal.Desktop" --object-path=/org/freedesktop/portal/desktop --method=org.freedesktop.portal.OpenURI.OpenURI '' '%u' '{}'"#),
        startup_wm_class: Some(Box::from("firefox")),
        icon: Some(Box::from(Path::new("firefox"))),
    }));
}
