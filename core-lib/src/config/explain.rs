use crate::config::{Config, load_and_migrate_config};
use crate::daemon_running;
use std::path::Path;
use tracing::error;

pub fn explain_config(config_path: &Path) -> anyhow::Result<()> {
    let config = load_and_migrate_config(config_path)
        .inspect_err(|err| error!("{BOLD}{RED}Config is invalid:{RESET} {err:?}\n"))?;
    let info = explain(config);
    println!("{info}");

    if daemon_running() {
        println!("Daemon {GREEN}running{RESET}")
    } else {
        println!(
            "Daemon {RED}not running{RESET}, start it with `hyprshell run` or `systemctl --user enable --now hyprshell`"
        );
    }
    Ok(())
}

const BOLD: &str = "\x1b[1m";
const BLUE: &str = "\x1b[34m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

pub fn explain(config: Config) -> String {
    let mut builder = format!(
        "{BOLD}{GREEN}Config is valid{RESET}\n{BOLD}Explanation{RESET} ({BLUE}blue{RESET} are keys, {BOLD}{BLUE}bold blue{RESET} keys can be configured in config):{RESET}\n\n"
    );

    if let Some(windows) = &config.windows {
        if let Some(overview) = &windows.overview {
            builder.push_str(&format!(
                "Use {BOLD}{BLUE}{} + {}{RESET} to open the Overview. Use {BLUE}tab{RESET} and {BLUE}grave{RESET} / {BLUE}shift + tab{RESET} to select a different window, press {BLUE}return{RESET} to switch\n\
                You can also use the {BLUE}arrow keys{RESET} to navigate the workspaces. Use {BLUE}Esc{RESET} to close the overview.\n",
                overview.modifier,
                overview.key,
            ));
            builder.push_str("After opening the Overview the launcher is available.\n");
            if let Some(_applications) = overview.launcher.plugins.applications.as_ref() {
                builder.push_str(&format!("Start typing to search through applications (sorted by how often they were opened).\n\
                    Press {BLUE}return{RESET} to launch the first app, use {BLUE}Ctrl + 1/2/3/...{RESET} to open the second, third, etc.\n"));
            }
            if overview.launcher.plugins.terminal.is_some() {
                builder.push_str(&format!(
                    "Press {BLUE}Ctrl + t{RESET} to run the typed command in a terminal.\n"
                ));
            }
            if overview.launcher.plugins.shell.is_some() {
                builder.push_str(&format!(
                    "Press {BLUE}Ctrl + r{RESET} to run the typed command in the background.\n",
                ));
            }
            if let Some(engines) = &overview.launcher.plugins.websearch {
                builder.push_str(&format!("Press {BLUE}Ctrl + {BOLD}{BLUE}<key>{RESET} to search the typed text in any of the configured SearchEngines: {}.\n",
                                              engines.engines.iter().map(|e| e.name.to_string()).collect::<Vec<_>>().join(", ")));
            }
            if overview.launcher.plugins.calc.is_some() {
                builder.push_str(
                    "Typing a mathematical expression will calculate it and display the result in the launcher.\n",
                );
            }
        } else {
            builder.push_str("<Overview disabled>\n");
        };
    };

    builder.push('\n');

    if let Some(windows) = &config.windows {
        if let Some(switch) = &windows.switch {
            builder.push_str(&format!(
                "Press {BOLD}{BLUE}{}{RESET} + {BLUE}tab{RESET} and hold {BOLD}{BLUE}{}{RESET} to view recently used applications. Press {BLUE}tab{RESET} and {BLUE}grave{RESET}/{BLUE}shift + tab{RESET} to select a different window,\n\
                release {BOLD}{BLUE}{}{RESET} to close the window. You can also use the {BLUE}arrow keys{RESET} to navigate the clients.\n",
                switch.modifier,
                switch.modifier,
                switch.modifier,
            ));
        } else {
            builder.push_str("<Recent Apps disabled>\n");
        };
    }

    builder
}
