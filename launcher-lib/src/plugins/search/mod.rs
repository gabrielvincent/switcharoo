mod default;

use crate::plugins::search::default::get_browser_exec;
use crate::plugins::{Identifier, PluginNames, StaticLaunchOption};
use core_lib::config::SearchEngine;
pub use default::reload_default_browser;
use exec_lib::run::run_program;
use exec_lib::toast;
use tracing::debug;

pub fn get_static_options(matches: &mut Vec<StaticLaunchOption>, config: &[SearchEngine]) {
    for engine in config.iter() {
        if let Some(key) = engine.key.chars().next() {
            matches.push(StaticLaunchOption {
                text: engine.name.clone().into_boxed_str(),
                details: format!("Search with {}", engine.name).into_boxed_str(),
                icon: None,
                key,
                data: Identifier {
                    plugin: PluginNames::WebSearch,
                    identifier: Some(engine.url.clone().into_boxed_str()),
                },
            });
        } else {
            toast(&format!("Plugin {} has no valid key set", engine.name));
        }
    }
}

pub fn launch_option(iden: &Option<Box<str>>, text: &str) {
    if let Some(iden) = iden {
        let url = iden.replace("{}", text);
        debug!("Launching URL: {}", url);
        let browser = get_browser_exec();
        for repl in ["%u", "%U", "%f", "%F"] {
            if browser.contains(repl) {
                let browser = browser.replace(repl, &format!("'{url}'"));
                debug!("Launching browser: {}", browser);
                run_program(&browser, None, false, &None);
                return;
            }
        }
        run_program(&format!("{browser} '{url}'"), None, false, &None);
        // TODO maybe try to focus the browser if it is already open
    }
}

pub(crate) fn get_chars(config: &[SearchEngine]) -> Vec<char> {
    config
        .iter()
        .flat_map(|engine| {
            engine.key.chars().next().map_or_else(
                || {
                    toast(&format!("Plugin {} has no valid key set", engine.name));
                    None
                },
                Some,
            )
        })
        .collect()
}
