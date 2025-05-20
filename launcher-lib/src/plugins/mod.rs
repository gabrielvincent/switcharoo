use core_lib::config::Plugin;
use std::path::Path;
use tracing::{span, Level};

mod applications;
mod search;
mod shell;
mod terminal;

#[cfg(feature = "calc")]
mod calc;

pub use applications::get_stored_runs as get_applications_stored_runs;
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

#[derive(Debug, Copy, Clone)]
pub enum PluginNames {
    Applications,
    Shell,
    Terminal,
    WebSearch,
    Calc,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    /// identify the plugin that created this option.
    pub plugin: PluginNames,
    pub identifier: Option<Box<str>>,
}

impl Identifier {
    pub fn str(&self) -> String {
        format!(
            "{}:{}",
            self.plugin as u8,
            self.identifier.as_deref().unwrap_or_default()
        )
    }
}

pub fn get_sortable_launch_options(
    plugins: &[Plugin],
    text: &str,
    data_dir: &Path,
) -> Vec<SortableLaunchOption> {
    let mut matches = Vec::new();

    for plugins in plugins {
        match plugins {
            Plugin::Applications(config) => {
                applications::get_sortable_options(
                    &mut matches,
                    text,
                    config.run_cache_weeks,
                    config.show_execs,
                    data_dir,
                );
            }
            Plugin::Calc() => {
                #[cfg(feature = "calc")]
                calc::get_calc_options(&mut matches, text);
                #[cfg(not(feature = "calc"))]
                tracing::warn!("calc plugin is not enabled");
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
    plugins: &[Plugin],
    default_terminal: &Option<Box<str>>,
) -> Vec<StaticLaunchOption> {
    let mut matches = Vec::new();

    for plugins in plugins {
        match plugins {
            Plugin::Shell() => {
                shell::get_static_options(&mut matches);
            }
            Plugin::Terminal() => {
                terminal::get_static_options(&mut matches, default_terminal);
            }
            Plugin::WebSearch(config) => {
                search::get_static_options(&mut matches, config);
            }
            _ => {}
        }
    }

    matches
}

pub fn launch(
    iden: &Identifier,
    text: &str,
    default_terminal: &Option<Box<str>>,
    data_dir: &Path,
) -> bool {
    let _span = span!(Level::TRACE, "launch_plugin").entered();

    match iden.plugin {
        PluginNames::Applications => {
            applications::launch_option(&iden.identifier, default_terminal, data_dir)
        }
        PluginNames::Shell => shell::launch_option(text, default_terminal),
        PluginNames::Terminal => terminal::launch_option(text, default_terminal),
        PluginNames::WebSearch => search::launch_option(&iden.identifier, text),
        PluginNames::Calc => {
            #[cfg(feature = "calc")]
            calc::copy_result(&iden.identifier);
            #[cfg(not(feature = "calc"))]
            tracing::warn!("calc plugin is not enabled");
            false
        }
    }
}

pub fn get_static_options_chars(plugins: &Vec<Plugin>) -> Vec<char> {
    let mut chars = Vec::new();

    for plugins in plugins {
        match plugins {
            Plugin::Shell() => {
                chars.append(&mut shell::get_chars());
            }
            Plugin::Terminal() => {
                chars.append(&mut terminal::get_chars());
            }
            Plugin::WebSearch(config) => {
                chars.append(&mut search::get_chars(config));
            }
            _ => {}
        }
    }
    chars
}
