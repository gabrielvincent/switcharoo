use crate::generate::tui::{WEB_SEARCH_ENGINES, configurable_launcher_plugins};
use crate::structs::{Switch, Windows};
use crate::{Config, Launcher, Modifier, Overview, Plugins, WebSearchConfig};
use std::path::Path;

pub fn get_overrides(force: &[String]) -> (bool, bool) {
    // force contains "config" or "css" or "all"
    let mut override_config = false;
    let mut override_css = false;
    for item in force {
        match item.as_str() {
            "config" => override_config = true,
            "css" => override_css = true,
            "all" => {
                override_config = true;
                override_css = true;
            }
            _ => {}
        }
    }
    (override_config, override_css)
}

pub fn check_file_exist(
    config_path: &Path,
    css_path: &Path,
    override_config: bool,
    override_css: bool,
) -> anyhow::Result<()> {
    if !override_config && config_path.exists() {
        eprintln!(
            "\x1b[1mConfig file {config_path:?} already exists, use -f to override all or -f config to override only the config file\x1b[0m"
        )
    }
    if !override_css && css_path.exists() {
        eprintln!(
            "\x1b[1mCSS file {css_path:?} already exists, use -f to override all or -f css to override only the css file\x1b[0m"
        )
    }
    Ok(())
}

#[derive(Debug)]
pub struct ConfigData {
    pub default_terminal: Option<Box<str>>,
    pub overview: Option<(Modifier, Box<str>)>,
    pub switch: (Option<Modifier>, bool),
    pub launcher_plugins: Vec<Box<str>>,
    pub launcher_engines: Vec<Box<str>>,
}

pub fn generate_config(data: ConfigData) -> Config {
    Config {
        windows: Some(Windows {
            overview: if let Some(overview) = data.overview {
                Some(Overview {
                    modifier: overview.0,
                    key: overview.1,
                    launcher: Launcher {
                        default_terminal: data.default_terminal,
                        plugins: Plugins {
                            applications: data
                                .launcher_plugins
                                .iter()
                                .find(|pl| {
                                    pl.as_ref().eq(configurable_launcher_plugins::APPLICATIONS)
                                })
                                .map(|_| Default::default()),
                            terminal: data
                                .launcher_plugins
                                .iter()
                                .find(|pl| pl.as_ref().eq(configurable_launcher_plugins::TERMINAL))
                                .map(|_| Default::default()),
                            shell: data
                                .launcher_plugins
                                .iter()
                                .find(|pl| pl.as_ref().eq(configurable_launcher_plugins::SHELL))
                                .map(|_| Default::default()),
                            websearch: data
                                .launcher_plugins
                                .iter()
                                .find(|pl| {
                                    pl.as_ref().eq(configurable_launcher_plugins::WEB_SEARCH)
                                })
                                .map(|_| WebSearchConfig {
                                    engines: data
                                        .launcher_engines
                                        .iter()
                                        .filter_map(|engine| {
                                            WEB_SEARCH_ENGINES
                                                .iter()
                                                .find(|(name, _)| *name == engine.as_ref())
                                                .map(|(_, constructor)| constructor())
                                        })
                                        .collect(),
                                }),
                            calc: data
                                .launcher_plugins
                                .iter()
                                .find(|pl| pl.as_ref().eq(configurable_launcher_plugins::CALC))
                                .map(|_| Default::default()),
                            path: Some(Default::default()),
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
            } else {
                None
            },
            switch: data.switch.0.map(|switch_mod| Switch {
                modifier: switch_mod,
                show_workspaces: data.switch.1,
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}
