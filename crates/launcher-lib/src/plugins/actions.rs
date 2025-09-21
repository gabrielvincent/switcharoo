use crate::plugins::{Identifier, PluginNames, PluginReturn, SortableLaunchOption};
use config_lib::ActionsPluginConfig;
use core_lib::WarnWithDetails;
use std::path::{Path, PathBuf};
use tracing::{info, trace};

struct Action {
    names: Vec<Box<str>>,
    details: Box<str>,
    command: Box<str>,
    icon: Box<Path>,
}

pub fn get_actions_options(
    matches: &mut Vec<SortableLaunchOption>,
    text: &str,
    config: &ActionsPluginConfig,
) {
    if text.is_empty() {
        return;
    }
    let lower_text = text.to_ascii_lowercase();

    let actions = config.0.iter().map(get_action).collect::<Vec<Action>>();

    for action in actions {
        if action.names.iter().any(|name| {
            name.to_ascii_lowercase().contains(&lower_text)
                || lower_text.contains(&name.to_ascii_lowercase())
        }) || text.eq("actions")
        {
            let name = action
                .names
                .iter()
                .find(|name| {
                    name.to_ascii_lowercase().contains(&lower_text)
                        || lower_text.contains(&name.to_ascii_lowercase())
                })
                .unwrap_or(&action.names[0])
                .to_ascii_lowercase()
                .into_boxed_str();
            let match_type = if name.to_ascii_lowercase().starts_with(&lower_text)
                || lower_text.contains(&name.to_ascii_lowercase())
            {
                "Exact"
            } else {
                "Partial"
            };
            let mut command = action.command.clone();
            if command.contains("{}") {
                trace!(
                    "Action command contains '{{}}', replacing <{text}> with stripped ({name}) text"
                );
                let stripped_text = text.trim_start_matches(&*name).trim();
                command = Box::from(command.replace("{}", stripped_text));
            }
            trace!("Added action option: {} ({})", command, match_type);
            matches.push(SortableLaunchOption {
                icon: Some(action.icon),
                name: name.clone(),
                details: action.details,
                details_long: Some(command.clone()),
                score: match match_type {
                    "Exact" => 70,
                    _ => 15,
                },
                grayed: false,
                iden: Identifier::data(PluginNames::Actions, command),
                details_menu: vec![],
            });
        }
    }
}

pub fn run_action(data: Option<&str>) -> PluginReturn {
    if let Some(data) = data {
        if cfg!(debug_assertions) && std::env::var("HYPRSHELL_RUN_ACTIONS_IN_DEBUG").is_err() {
            info!("Not running action: {} (debug mode)", data);
        } else {
            info!("Running action: {}", data);
            exec_lib::run::run_program(data, None, false, None)
                .warn_details("Failed to run command");
        }
    }

    PluginReturn {
        show_animation: true,
    }
}

fn get_action(action: &config_lib::ActionsPluginAction) -> Action {
    match action {
        config_lib::ActionsPluginAction::LockScreen => Action {
            names: vec![Box::from("Lock Screen")],
            details: Box::from("Lock the screen"),
            command: Box::from("loginctl lock-session"),
            icon: PathBuf::from("system-lock-screen").into_boxed_path(),
        },
        config_lib::ActionsPluginAction::Hibernate => Action {
            names: vec![Box::from("Hibernate")],
            details: Box::from("Hibernate the computer"),
            command: Box::from("systemctl hibernate"),
            icon: PathBuf::from("system-hibernate").into_boxed_path(),
        },
        config_lib::ActionsPluginAction::Logout => Action {
            names: vec![Box::from("Log Out")],
            details: Box::from("Log out of the session"),
            command: Box::from("loginctl terminate-session self"),
            icon: PathBuf::from("system-log-out").into_boxed_path(),
        },
        config_lib::ActionsPluginAction::Reboot => Action {
            names: vec![Box::from("Reboot"), Box::from("Restart")],
            details: Box::from("Reboot the computer"),
            command: Box::from("systemctl reboot"),
            icon: PathBuf::from("system-reboot").into_boxed_path(),
        },
        config_lib::ActionsPluginAction::Shutdown => Action {
            names: vec![Box::from("Shut Down"), Box::from("Power off")],
            details: Box::from("Shut down the computer"),
            command: Box::from("systemctl poweroff"),
            icon: PathBuf::from("system-shutdown").into_boxed_path(),
        },
        config_lib::ActionsPluginAction::Suspend => Action {
            names: vec![Box::from("Sleep"), Box::from("Suspend")],
            details: Box::from("Put the computer to sleep"),
            command: Box::from("systemctl suspend"),
            icon: PathBuf::from("system-suspend").into_boxed_path(),
        },
        config_lib::ActionsPluginAction::Custom(custom) => Action {
            names: custom.names.clone(),
            details: custom.details.clone(),
            command: custom.command.clone(),
            icon: Box::from(Path::new(&*custom.icon)),
        },
    }
}
