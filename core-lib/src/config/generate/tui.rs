use crate::config::generate::autocomplete::StringAutoCompleter;
use crate::config::generate::config::ConfigData;
use crate::config::structs::{KeyMaybeMod, Mod};
use crate::config::Plugins;
use crate::util::TERMINALS;
use anyhow::bail;
use inquire::formatter::MultiOptionFormatter;
use inquire::{Confirm, MultiSelect, Text};

pub const CONFIGURABLE_LAUNCHER_PLUGINS: &[(&str, fn() -> Plugins)] = &[
    ("Open Applications", || {
        Plugins::Applications(Default::default())
    }),
    ("Run in shell", || Plugins::Shell()),
    ("Run in terminal", || Plugins::Terminal()),
    ("Web search", || Plugins::WebSearch(Default::default())),
];
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

        let (default_terminal, launcher_plugins) = if enable_launcher {
            let default_terminal = Text::new("Default Terminal")
                .with_autocomplete(StringAutoCompleter::from(Box::from(TERMINALS)))
                .with_help_message("used to open terminal applications (htop), leave empty to chose from installed terminals]\n[↑↓ to move, tab to autocomplete, enter to submit")
                .prompt()
                .map_or(None, |term| if term.trim().is_empty() { None } else { Some(term.into_boxed_str()) });

            let formatter: MultiOptionFormatter<'_, &str> =
                &|a| format!("{} plugins active", a.len());
            let plugins = MultiSelect::new(
                "Plugins for launcher",
                CONFIGURABLE_LAUNCHER_PLUGINS
                    .iter()
                    .map(|(name, _)| *name)
                    .collect(),
            )
            .with_all_selected_by_default()
            .with_formatter(formatter)
            .prompt()
            .map_or(vec![], |selected| {
                selected.into_iter().map(Box::from).collect()
            });

            (default_terminal, plugins)
        } else {
            (None, vec![])
        };
        Some((enable_launcher, default_terminal, launcher_plugins))
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
        default_terminal: launcher.as_ref().and_then(|l| l.1.clone()),
        overview: open_overview.map(|o| (o.0, KeyMaybeMod::from(&*o.1))),
        switch: open_switch,
        launcher_plugins: launcher.map(|l| l.2).unwrap_or_default(),
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
