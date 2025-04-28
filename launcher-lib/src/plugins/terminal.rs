use crate::plugins::{Identifier, StaticLaunchOption, StaticLaunchOptionDisplay};
use exec_lib::run::run_program;

const PLUGIN_NAME: &str = "shell";

pub fn get_static_options() -> Vec<StaticLaunchOption> {
    let mut matches = Vec::new();
    matches.push(StaticLaunchOption {
        data: Identifier {
            plugin: PLUGIN_NAME,
            identifier: None,
        },
        key: 't',
        display: StaticLaunchOptionDisplay::Text("Terminal".into()),
    });
    matches
}

pub fn launch_static_options(text: &str, default_terminal: Option<String>) {
    run_program(
        &format!("$SHELL -c \"{text}\""),
        None,
        true,
        default_terminal,
    );
}

pub(crate) fn get_chars() -> Vec<char> {
    vec!['t']
}
