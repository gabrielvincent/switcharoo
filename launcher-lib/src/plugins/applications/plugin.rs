use crate::plugins::applications::data::{get_stored_runs, save_run};
use crate::plugins::applications::map::{DesktopEntry, get_all_desktop_files};
use crate::plugins::{DetailsMenuItem, Identifier, PluginNames, SortableLaunchOption};
use core_lib::{ExecType, Warn, analyse_exec};
use exec_lib::run::run_program;
use std::collections::HashMap;
use std::path::Path;
use tracing::{trace, warn};

#[derive(Debug, Clone, Copy)]
enum MatchType {
    AppType = 1,
    Keyword = 4,
    ExecName = 10,
    ExecExact = 15,
    Name = 16,
    Exact = 21,
}

impl SortableLaunchOption {
    fn from_desktop_entry(
        entry: &DesktopEntry,
        r#match: MatchType,
        runs: &HashMap<Box<Path>, u64>,
        show_execs: bool,
        show_actions_submenu: bool,
    ) -> Self {
        let (details, details_long) = if show_execs {
            match analyse_exec(&entry.exec) {
                ExecType::Flatpak(a, b) => (format!("[Flatpak] {a}").into_boxed_str(), Some(b)),
                ExecType::PWA(a, b) => (format!("[PWA] {a}").into_boxed_str(), Some(b)),
                ExecType::FlatpakPWA(a, b) => {
                    (format!("[Flatpak-PWA] {a}").into_boxed_str(), Some(b))
                }
                ExecType::AppImage(a, b) => (format!("[AppImage] {a}").into_boxed_str(), Some(b)),
                ExecType::Absolute(a, b) => (a, Some(b)),
                ExecType::Relative(a) => (a, None),
            }
        } else {
            (Box::from(""), None)
        };

        let runs = runs.get(&entry.source).unwrap_or(&0);
        // trace!(
        //     "entry {} has match:{:?}({}) and runs:{} => {}",
        //     entry.name,
        //     r#match,
        //     r#match as u64,
        //     runs,
        //     r#match as u64 + runs
        // );
        SortableLaunchOption {
            name: entry.name.clone(),
            icon: entry.icon.clone(),
            details,
            details_long,
            score: r#match as u64 + runs,
            iden: Identifier {
                identifier: Some(Box::from(entry.source.to_string_lossy())),
                plugin: PluginNames::Applications,
            },
            details_menu: if show_actions_submenu {
                entry
                    .other
                    .iter()
                    .map(|action| DetailsMenuItem {
                        text: action.name.clone(),
                        exec: action.exec.clone(),
                        iden: Identifier {
                            identifier: Some(Box::from(format!(
                                "{}|{}",
                                entry.source.to_string_lossy(),
                                action.id
                            ))),
                            plugin: PluginNames::Applications,
                        },
                    })
                    .collect()
            } else {
                vec![]
            },
        }
    }
}

pub fn get_sortable_options(
    matches: &mut Vec<SortableLaunchOption>,
    text: &str,
    run_cache_weeks: u8,
    show_execs: bool,
    show_actions_submenu: bool,
    data_dir: &Path,
) {
    let entries = get_all_desktop_files();
    let runs = get_stored_runs(run_cache_weeks, data_dir);

    if text.is_empty() {
        for entry in entries.iter() {
            matches.push(SortableLaunchOption::from_desktop_entry(
                entry,
                MatchType::Exact,
                &runs,
                show_execs,
                show_actions_submenu,
            ));
        }
        return;
    }

    let lower_text = text.to_ascii_lowercase();
    for entry in entries.iter() {
        let opt = if entry.name.to_ascii_lowercase().contains(&lower_text) {
            if entry.name.to_ascii_lowercase().starts_with(&lower_text) {
                Some(SortableLaunchOption::from_desktop_entry(
                    entry,
                    MatchType::Exact,
                    &runs,
                    show_execs,
                    show_actions_submenu,
                ))
            } else {
                Some(SortableLaunchOption::from_desktop_entry(
                    entry,
                    MatchType::Name,
                    &runs,
                    show_execs,
                    show_actions_submenu,
                ))
            }
        } else if entry.exec_search.to_ascii_lowercase().contains(&lower_text) {
            if entry
                .exec_search
                .to_ascii_lowercase()
                .starts_with(&lower_text)
            {
                Some(SortableLaunchOption::from_desktop_entry(
                    entry,
                    MatchType::ExecExact,
                    &runs,
                    show_execs,
                    show_actions_submenu,
                ))
            } else {
                Some(SortableLaunchOption::from_desktop_entry(
                    entry,
                    MatchType::ExecName,
                    &runs,
                    show_execs,
                    show_actions_submenu,
                ))
            }
        } else if entry
            .keywords
            .iter()
            .any(|k| k.to_ascii_lowercase().starts_with(&lower_text))
        {
            Some(SortableLaunchOption::from_desktop_entry(
                entry,
                MatchType::Keyword,
                &runs,
                show_execs,
                show_actions_submenu,
            ))
        } else if entry.type_search.eq(&lower_text) {
            Some(SortableLaunchOption::from_desktop_entry(
                entry,
                MatchType::AppType,
                &runs,
                show_execs,
                show_actions_submenu,
            ))
        } else {
            None
        };

        // push only if not already in matches
        if let Some(opt) = opt {
            if !matches.iter().any(|m| {
                m.name == opt.name && m.details == opt.details && m.details_long == opt.details_long
            }) {
                matches.push(opt);
            }
        }
    }
}
pub fn launch_option(
    iden: &Option<Box<str>>,
    default_terminal: &Option<Box<str>>,
    data_dir: &Path,
) -> bool {
    let entries = get_all_desktop_files();
    if let Some(iden) = iden {
        let (source, section) = if iden.contains('|') {
            // split identifier for desktop action
            let parts: Vec<&str> = iden.split('|').collect();
            if parts.len() != 2 {
                warn!("Invalid identifier format: {:?}", iden);
                return false;
            }
            (parts[0], Some(parts[1]))
        } else {
            (iden.as_ref(), None)
        };
        let entry = entries
            .iter()
            .find(|entry| source == entry.source.to_string_lossy());
        if let Some(entry) = entry {
            let exec = if let Some(section) = section {
                // find desktop action
                if let Some(action) = entry.other.iter().find(|a| *a.id == *section) {
                    action.exec.clone()
                } else {
                    warn!(
                        "Failed to find action {:?} in entry {:?}",
                        section, entry.name
                    );
                    return false;
                }
            } else {
                entry.exec.clone()
            };
            run_program(
                &exec,
                entry.exec_path.clone(),
                entry.terminal,
                default_terminal,
            )
            .warn("Failed to run program");
            trace!("Saving run: {:?}", entry.source);
            save_run(&entry.source, data_dir).warn("Failed to cache run");
            return true;
        } else {
            warn!("Failed to find entry for {:?}", iden);
        };
    };
    false
}
