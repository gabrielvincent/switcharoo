use crate::data::get_cached_runs;
use crate::plugins::applications::maps::{get_all_desktop_files, DesktopEntry};
use crate::plugins::{Identifier, SortableLaunchOption};
use core_lib::Warn;
use exec_lib::run::run_program;
use std::collections::HashMap;
use std::path::Path;
use tracing::warn;

const PLUGIN_NAME: &str = "applications";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MatchType {
    Other = 1,
    Keyword = 5,
    Name = 10,
    Exact = 15,
}

impl SortableLaunchOption {
    fn from(
        entry: &DesktopEntry,
        r#match: MatchType,
        runs: &HashMap<Box<Path>, u64>,
        show_execs: bool,
    ) -> Self {
        let (details, details_long) = if show_execs {
            if let Some(short_exec) = entry.exec.as_ref().rsplit('/').find(|s| !s.is_empty()) {
                (Box::from(short_exec), Some(entry.exec.clone()))
            } else {
                (entry.exec.clone(), None)
            }
        } else {
            (Box::from(""), None)
        };
        let score = r#match as u64 + runs.get(&entry.source).unwrap_or(&0);
        SortableLaunchOption {
            name: entry.name.clone(),
            icon: entry.icon.clone(),
            details,
            details_long,
            score,
            data: Identifier {
                identifier: Some(Box::from(entry.source.to_string_lossy())),
                plugin: PLUGIN_NAME,
            },
        }
    }
}

pub fn get_sortable_options(
    text: &str,
    run_cache_weeks: u8,
    show_execs: bool,
    data_dir: &Path,
) -> Vec<SortableLaunchOption> {
    let mut matches = Vec::new();
    let entries = get_all_desktop_files();
    let runs = get_cached_runs(run_cache_weeks, data_dir);

    let lower_text = text.to_ascii_lowercase();
    for entry in entries.iter() {
        if entry.name.to_ascii_lowercase().contains(&lower_text)
            || entry.exec.to_ascii_lowercase().contains(&lower_text)
        {
            if entry.name.to_ascii_lowercase().starts_with(&lower_text)
                || entry.exec.to_ascii_lowercase().starts_with(&lower_text)
            {
                matches.push(SortableLaunchOption::from(
                    entry,
                    MatchType::Exact,
                    &runs,
                    show_execs,
                ));
            } else {
                matches.push(SortableLaunchOption::from(
                    entry,
                    MatchType::Name,
                    &runs,
                    show_execs,
                ));
            }
        } else if entry
            .keywords
            .iter()
            .any(|k| k.to_ascii_lowercase().starts_with(&lower_text))
        {
            matches.push(SortableLaunchOption::from(
                entry,
                MatchType::Keyword,
                &runs,
                show_execs,
            ));
        }
    }
    matches
}
pub fn launch_option(r#match: SortableLaunchOption, default_terminal: Option<String>) {
    let entries = get_all_desktop_files();
    let entry = entries
        .iter().find(|entry| r#match.data.identifier.as_deref() == Some(&*entry.source.to_string_lossy()));
    if let Some(entry) = entry {
        let mut exec = entry.exec.clone();
        run_program(
            &*exec,
            entry.exec_path.clone(),
            entry.terminal,
            default_terminal,
        );
    } else {
        warn!("Failed to find entry for {:?}", r#match.data.identifier);
    }
}

pub mod maps {
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
}
