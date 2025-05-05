use core_lib::config::Plugins;
use std::path::Path;

mod applications;
mod shell;
mod terminal;
mod search;

pub use applications::reload_desktop_map as reload_applications_desktop_map;
pub use search::reload_default_browser as reload_search_default_browser;

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
    pub text: Box<str>,
    pub details: Box<str>,
    pub icon: Option<Box<Path>>,
    pub key: char,
    pub data: Identifier,
}

#[derive(Debug)]
pub enum PluginNames {
    Applications,
    Shell,
    Terminal,
    WebSearch,
}

#[derive(Debug)]
pub struct Identifier {
    /// identify the plugin that created this option.
    pub plugin: PluginNames,
    pub identifier: Option<Box<str>>,
}

pub fn get_sortable_launch_options(
    plugins: &[Plugins],
    text: &str,
    data_dir: &Path,
) -> Vec<SortableLaunchOption> {
    let mut matches = Vec::new();

    for plugins in plugins {
        #[allow(clippy::single_match)]
        match plugins {
            Plugins::Applications(config) => {
                applications::get_sortable_options(
                    &mut matches,
                    text,
                    config.run_cache_weeks,
                    config.show_execs,
                    data_dir,
                );
            }
            _ => {}
        }
    }

    // sort in reverse
    matches.sort_by(|a, b| b.score.cmp(&a.score));
    // trace!("matches: {:?}", matches);
    matches
}

pub fn get_static_launch_options(
    plugins: &[Plugins],
    default_terminal: &Option<Box<str>>,
) -> Vec<StaticLaunchOption> {
    let mut matches = Vec::new();

    for plugins in plugins {
        match plugins {
            Plugins::Shell() => {
                shell::get_static_options(&mut matches);
            }
            Plugins::Terminal() => {
                terminal::get_static_options(&mut matches, default_terminal);
            }
            Plugins::WebSearch(config) => {
                search::get_static_options(&mut matches, config);
            }
            _ => {}
        }
    }

    matches
}

pub fn launch(iden: &Identifier, text: &str, default_terminal: &Option<Box<str>>, data_dir: &Path) {
    match iden.plugin {
        PluginNames::Applications => {
            applications::launch_option(&iden.identifier, default_terminal, data_dir)
        }
        PluginNames::Shell => shell::launch_option(text, default_terminal),
        PluginNames::Terminal => terminal::launch_option(text, default_terminal),
        PluginNames::WebSearch => search::launch_option(&iden.identifier, text),
    }
}

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
            Plugins::WebSearch(config) => {
                chars.append(&mut search::get_chars(config));
            }
            _ => {}
        }
    }
    chars
}
