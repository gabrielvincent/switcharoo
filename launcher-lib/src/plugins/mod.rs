use std::path::Path;

mod applications;
mod shell;
mod terminal;

pub use applications::maps::reload_desktop_map;
use core_lib::config::Plugins;

#[derive(Debug)]
pub enum MatchData {
    Sortable(Identifier),
    Static(Identifier),
}

#[derive(Debug)]
pub struct SortableLaunchOption {
    pub name: Box<str>,
    pub icon: Option<Box<Path>>,
    pub details: Box<str>,
    pub details_long: Option<Box<str>>,
    pub score: u64,
    pub data: Identifier,
}

#[derive(Debug)]
pub struct StaticLaunchOption {
    pub display: StaticLaunchOptionDisplay,
    pub key: char,
    pub data: Identifier,
}

#[derive(Debug)]
pub enum StaticLaunchOptionDisplay {
    Icon(Box<Path>),
    Text(Box<str>),
}

#[derive(Debug)]
pub struct Identifier {
    /// identify the plugin that created this option.
    pub plugin: &'static str,
    pub identifier: Option<Box<str>>,
}

pub fn get_sortable_launch_options(
    plugins: &Vec<Plugins>,
    text: &str,
    data_dir: &Path,
) -> Vec<SortableLaunchOption> {
    let mut matches = Vec::new();

    for plugins in plugins {
        match plugins {
            Plugins::Applications(config) => {
                matches.append(&mut applications::get_sortable_options(
                    text,
                    config.run_cache_weeks,
                    config.show_execs,
                    data_dir,
                ));
            }
            _ => {}
        }
    }

    matches.sort_by(|a, b| a.score.cmp(&b.score));
    matches
}
pub fn get_static_launch_options(plugins: &Vec<Plugins>) -> Vec<StaticLaunchOption> {
    let mut matches = Vec::new();

    for plugins in plugins {
        match plugins {
            Plugins::Shell() => {
                matches.append(&mut shell::get_static_options());
            }
            Plugins::Terminal() => {
                matches.append(&mut terminal::get_static_options());
            }
            _ => {}
        }
    }

    matches
}

pub fn launch_static_options(key: &str, default_terminal: Option<String>) {}

pub fn launch_sortable_options(text: &str, offset: u8) {}

pub fn get_static_options_chars(plugins: &Vec<Plugins>) -> Vec<char> {
    let mut chars = Vec::new();

    for plugins in plugins {
        match plugins {
            Plugins::Shell() => {
                chars.append(&mut shell::get_chars());
            }
            Plugins::Terminal() => {
                chars.append(&mut terminal::get_chars());
            }
            _ => {}
        }
    }
    chars
}
