use crate::plugins::{Identifier, StaticLaunchOption, StaticLaunchOptionDisplay};
use exec_lib::run::run_program;
use std::path::PathBuf;

const PLUGIN_NAME: &str = "shell";

pub fn get_static_options() -> Vec<StaticLaunchOption> {
    let mut matches = Vec::new();
    matches.push(StaticLaunchOption {
        data: Identifier {
            plugin: PLUGIN_NAME,
            identifier: None,
        },
        key: 'r',
        display: StaticLaunchOptionDisplay::Icon(PathBuf::from("system-run").into_boxed_path()),
    });
    matches
}

pub fn launch_static_options(text: &str, default_terminal: Option<String>) {
    run_program(text, None, false, default_terminal);
}

pub(crate) fn get_chars() -> Vec<char> {
    vec!['r']
}
