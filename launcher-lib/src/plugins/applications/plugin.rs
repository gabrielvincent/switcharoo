use crate::plugins::applications::cache::{get_cached_runs, save_run};
use crate::plugins::applications::map::{get_all_desktop_files, DesktopEntry};
use crate::plugins::{Identifier, PluginNames, SortableLaunchOption};
use core_lib::Warn;
use exec_lib::run::run_program;
use std::collections::HashMap;
use std::path::Path;
use tracing::{trace, warn};

enum MatchType {
    Keyword = 2,
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
            get_exec_labels(&entry.exec)
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
                plugin: PluginNames::Applications,
            },
        }
    }
}

pub fn get_sortable_options(
    matches: &mut Vec<SortableLaunchOption>,
    text: &str,
    run_cache_weeks: u8,
    show_execs: bool,
    data_dir: &Path,
) {
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
}
pub fn launch_option(
    iden: &Option<Box<str>>,
    default_terminal: &Option<Box<str>>,
    data_dir: &Path,
) {
    let entries = get_all_desktop_files();
    let entry = entries
        .iter()
        .find(|entry| iden.as_deref() == Some(&*entry.source.to_string_lossy()));
    if let Some(entry) = entry {
        let exec = entry.exec.clone();
        run_program(
            &exec,
            entry.exec_path.clone(),
            entry.terminal,
            default_terminal,
        );
        trace!("Saving run: {:?}", entry.source);
        save_run(&entry.source, data_dir).warn("Failed to cache run");
    } else {
        warn!("Failed to find entry for {:?}", iden);
    }
}

fn get_exec_labels(exec: &str) -> (Box<str>, Option<Box<str>>) {
    let exec_trim = exec.replace("'", "").replace("\"", "");
    // pwa detection
    if exec.contains("--app-id=") && exec.contains("--profile-directory=") {
        // "flatpak 'run'" = pwa from browser inside flatpak
        if exec.contains("flatpak run") || exec.contains("flatpak 'run'") {
            (
                format!(
                    "[Flatpak + PWA] {}",
                    exec_trim
                        .split_whitespace()
                        .find(|s| s.contains("--command="))
                        .and_then(|s| s
                            .split('=')
                            .next_back()
                            .and_then(|s| s.split('/').next_back()))
                        .unwrap_or_default()
                )
                .into_boxed_str(),
                exec_trim
                    .split_whitespace()
                    .skip(2)
                    .find(|arg| !arg.starts_with("--"))
                    .map(Box::from),
            )
        } else {
            // normal PWA
            (
                format!(
                    "[PWA] {}",
                    exec.split_whitespace()
                        .next()
                        .and_then(|s| s.split('/').next_back())
                        .unwrap_or_default()
                )
                .into_boxed_str(),
                exec.split_whitespace().next().map(Box::from),
            )
        }
        // flatpak detection
    } else if exec.contains("flatpak run") || exec.contains("flatpak 'run'") {
        (
            format!(
                "[Flatpak] {}",
                exec_trim
                    .split(' ')
                    .find(|s| s.contains("--command="))
                    .and_then(|s| s
                        .split('=')
                        .next_back()
                        .and_then(|s| s.split('/').next_back()))
                    .unwrap_or_default()
            )
            .into_boxed_str(),
            exec_trim
                .split_whitespace()
                .skip(2)
                .find(|arg| !arg.starts_with("--"))
                .map(Box::from),
        )
    } else if exec_trim.starts_with("/") {
        (
            Box::from(
                exec_trim
                    .rsplit('/')
                    .find(|s| !s.is_empty())
                    .unwrap_or_default(),
            ),
            Some(Box::from(exec)),
        )
    } else {
        (Box::from(exec_trim), None)
    }
}
