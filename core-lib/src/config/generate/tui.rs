use crate::config::generate::autocomplete::StringAutoCompleter;
use crate::config::generate::write::ConfigData;
use crate::config::structs::{KeyMaybeMod, Mod};
use crate::util::TERMINALS;
use anyhow::bail;
use inquire::{Confirm, Text};

pub fn prompt_config() -> anyhow::Result<ConfigData> {
    let open_overview = {
        let open_overview = Text::new("Key combination to open the overview (and launcher)")
            .with_autocomplete(StringAutoCompleter::from(vec![
                "Super",
                "Super + Tab",
                "Ctrl",
                "Ctrl + Tab",
                "Alt",
                "Alt + Tab",
            ]))
            .with_help_message("Shows all apps in a miniature view, allows to switch using arrow keys or tab. Leave blank to disable]\n[↑↓ to move, tab to autocomplete, enter to submit")
            .prompt()?;
        if open_overview.trim().is_empty() {
            None
        } else {
            get_mod(&open_overview).ok()
        }
    };
    let launcher = if open_overview.is_some() {
        let enable_launcher = Confirm::new("Enable Application Launcher?")
            .with_default(true)
            .with_help_message("Used to start applications, opens with overview")
            .prompt()?;

        let default_terminal = if enable_launcher {
            Text::new("Default Terminal")
                .with_autocomplete(StringAutoCompleter::from(Box::from(TERMINALS)))
                .with_help_message("used to open terminal applications (htop), leave empty to chose from installed terminals]\n[↑↓ to move, tab to autocomplete, enter to submit")
                .prompt()
                .map_or(None, |term| if term.trim().is_empty() { None } else { Some(term) })
        } else {
            None
        };
        Some((enable_launcher, default_terminal))
    } else {
        None
    };

    let open_switch = {
        let open_switch = Text::new("Modifier to open the switch (<mod> + tab)")
            .with_autocomplete(StringAutoCompleter::from(vec!["Alt", "Ctrl", "Super"]))
            .with_help_message("Shows clients in a list sorted by recently accessed. Please use something different from the Overview modifier. Leave blank to disable]\n[↑↓ to move, tab to autocomplete, enter to submit")
            .prompt()?;
        if open_switch.trim().is_empty() {
            None
        } else {
            get_mod(&open_switch)
                .ok()
                .map(|(enable_switch, _)| enable_switch)
        }
    };

    let grave_reverse =
        Confirm::new("Use Grave key (`) to select in reverse instead of Shift + Tab?")
            .with_default(false)
            .prompt()?;

    Ok(ConfigData {
        enable_launcher: launcher.as_ref().map(|l| l.0).unwrap_or(false),
        default_terminal: launcher.and_then(|l| l.1),
        overview: open_overview.map(|o| (o.0, KeyMaybeMod::from(&*o.1))),
        switch: open_switch,
        grave_reverse,
    })
}

fn get_mod(modifier: &str) -> anyhow::Result<(Mod, String)> {
    let split = modifier.split('+').collect::<Vec<_>>();
    let modd = match &*split[0].trim().to_ascii_lowercase() {
        "super" => Mod::Super,
        "ctrl" => Mod::Ctrl,
        "alt" => Mod::Alt,
        "shift" => Mod::Shift,
        _ => bail!("Unknown modifier: {}", split[0]),
    };
    Ok((
        modd,
        split
            .get(1)
            .map_or_else(|| modd.to_string(), |s| s.trim().to_string()),
    ))
}
