use crate::plugins::{Identifier, PluginNames, StaticLaunchOption};
use exec_lib::run::run_program;
use std::path::PathBuf;

pub fn get_static_options() -> Vec<StaticLaunchOption> {
    let mut matches = Vec::new();
    matches.push(StaticLaunchOption {
        data: Identifier {
            plugin: PluginNames::Shell,
            identifier: None,
        },
        key: 'r',
        text: Box::from("Shell"),
        icon: Some(PathBuf::from("bash").into_boxed_path()),
    });
    matches
}

pub fn launch_option(text: &str, default_terminal: &Option<Box<str>>) {
    run_program(text, None, false, default_terminal);
}

pub(crate) fn get_chars() -> Vec<char> {
    vec!['r']
}
