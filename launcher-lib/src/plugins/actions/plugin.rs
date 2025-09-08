use crate::plugins::actions::actions::{
    hibernate, lock_session, logout, reboot, shutdown, suspend,
};
use crate::plugins::{Identifier, PluginNames, PluginReturn, SortableLaunchOption};
use core_lib::WarnWithDetails;
use std::path::Path;
use tracing::{info, trace, warn};

const ACTIONS: &[(&str, &str, &str, &str)] = &[
    (
        "Lock Screen",
        "Lock the screen",
        "command",
        "system-lock-screen",
    ),
    (
        "Hibernate",
        "Hibernate the computer",
        "command",
        "system-hibernate",
    ),
    (
        "Log Out",
        "Log out of the session",
        "command",
        "system-log-out",
    ),
    ("Reboot", "Reboot the computer", "command", "system-reboot"),
    (
        "Shut Down",
        "Shut down the computer",
        "command",
        "system-shutdown",
    ),
    (
        "Sleep",
        "Put the computer to sleep",
        "command",
        "system-suspend",
    ),
];

pub fn get_actions_options(matches: &mut Vec<SortableLaunchOption>, text: &str) {
    if text.is_empty() {
        return;
    }
    let lower_text = text.to_ascii_lowercase();

    for (name, details, command, icon) in ACTIONS {
        if name.to_ascii_lowercase().contains(&lower_text) || text.eq("actions") {
            let match_type = if name.to_ascii_lowercase().starts_with(&lower_text) {
                "Exact"
            } else {
                "Partial"
            };
            matches.push(SortableLaunchOption {
                icon: Some(Box::from(Path::new(icon))),
                name: Box::from(*name),
                details: Box::from(*details),
                details_long: Some(Box::from(*command)),
                score: match match_type {
                    "Exact" => 100,
                    _ => 30,
                },
                grayed: false,
                iden: Identifier::data(
                    PluginNames::Actions,
                    name.replace(' ', "-").to_ascii_lowercase().into_boxed_str(),
                ),
                details_menu: vec![],
            });
            trace!("Added action option: {} ({})", name, match_type);
        }
    }
}

pub fn run_action(data: Option<&str>) -> PluginReturn {
    if let Some(data) = data {
        info!("Running action: {}", data);
        match data {
            "lock-screen" => lock_session(),
            "hibernate" => hibernate(),
            "log-out" => logout(),
            "reboot" => reboot(),
            "shut-down" => shutdown(),
            "suspend" => suspend(),
            _ => {
                warn!("Unknown action: {}", data);
                return PluginReturn {
                    show_animation: false,
                };
            }
        }
        .warn_details("Failed to run program");
    }

    PluginReturn {
        show_animation: true,
    }
}
