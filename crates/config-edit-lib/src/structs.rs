use adw::gdk::Display;
use relm4::gtk::prelude::DisplayExtManual;
use std::fmt;
use std::fmt::Formatter;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct Config {
    pub windows: Windows,
}

#[derive(Debug, Clone)]
pub struct Windows {
    pub enabled: bool,
    pub scale: f64,
    pub items_per_row: u8,
    pub overview: Overview,
    pub switch: Switch,
    pub switch_2: Switch,
}

#[derive(Debug, Clone)]
pub struct Overview {
    pub enabled: bool,
    pub launcher: Launcher,
    pub key: String,
    pub modifier: ConfigModifier,
    pub same_class: bool,
    pub current_workspace: bool,
    pub current_monitor: bool,
}

#[derive(Debug, Clone)]
pub struct Launcher {
    pub default_terminal: Option<String>,
    pub launch_modifier: ConfigModifier,
    pub width: u32,
    pub max_items: u8,
    pub show_when_empty: bool,
    pub plugins: Plugins,
}

#[derive(Debug, Clone)]
pub struct Plugins {
    pub applications: ApplicationsPluginConfig,
    pub terminal: EmptyConfig,
    pub shell: EmptyConfig,
    pub websearch: WebSearchConfig,
    pub calc: EmptyConfig,
    pub path: EmptyConfig,
    pub actions: ActionsPluginConfig,
}

#[derive(Debug, Clone)]
pub struct WebSearchConfig {
    pub enabled: bool,
    pub engines: Vec<config_lib::SearchEngine>,
}

#[derive(Debug, Clone)]
pub struct EmptyConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct ActionsPluginConfig {
    pub enabled: bool,
    pub actions: Vec<config_lib::ActionsPluginAction>,
}

#[derive(Debug, Clone)]
pub struct ApplicationsPluginConfig {
    pub enabled: bool,
    pub run_cache_weeks: u8,
    pub show_execs: bool,
    pub show_actions_submenu: bool,
}

#[derive(Debug, Clone)]
pub struct Switch {
    pub enabled: bool,
    pub modifier: ConfigModifier,
    pub key: String,
    pub same_class: bool,
    pub current_workspace: bool,
    pub current_monitor: bool,
    pub switch_workspaces: bool,
}

#[instrument(level = "trace", ret(level = "trace"))]
pub fn to_accelerator(modifier: ConfigModifier, key: &str) -> Option<String> {
    let key = key_to_name(key)?;
    if modifier == ConfigModifier::None {
        Some(key)
    } else {
        Some(format!("<{}>{}", modifier, key))
    }
}

pub fn key_to_name(key: &str) -> Option<String> {
    // key is keycode
    if key.starts_with("code:") {
        let key_id = key.split(':').nth(1)?;
        let code = key_id.parse::<u32>().ok()?;
        let display = &Display::default()?;
        let data = display.map_keycode(code)?;
        let (_, key) = data.iter().find(|(m, k)| m.level() == 0)?;
        Some(key.name()?.to_string())
    } else {
        Some(key.to_string())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConfigModifier {
    None,
    Alt,
    Super,
    Ctrl,
}

impl ConfigModifier {
    pub fn strings() -> &'static [&'static str] {
        &["Alt", "Super", "Ctrl"]
    }
    pub fn strings_with_none() -> &'static [&'static str] {
        &["Alt", "Super", "Ctrl", "None"]
    }
}

impl fmt::Display for ConfigModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigModifier::None => write!(f, ""),
            ConfigModifier::Alt => write!(f, "Alt"),
            ConfigModifier::Super => write!(f, "Super"),
            ConfigModifier::Ctrl => write!(f, "Ctrl"),
        }
    }
}

impl TryFrom<u32> for ConfigModifier {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Alt),
            1 => Ok(Self::Super),
            2 => Ok(Self::Ctrl),
            3 => Ok(Self::None),
            _ => Err(()),
        }
    }
}

impl Into<u32> for ConfigModifier {
    fn into(self) -> u32 {
        match self {
            ConfigModifier::Alt => 0,
            ConfigModifier::Super => 1,
            ConfigModifier::Ctrl => 2,
            ConfigModifier::None => 3,
        }
    }
}

impl From<config_lib::Config> for Config {
    fn from(value: config_lib::Config) -> Self {
        Self {
            windows: value.windows.into(),
        }
    }
}

impl From<config_lib::Modifier> for ConfigModifier {
    fn from(value: config_lib::Modifier) -> Self {
        match value {
            config_lib::Modifier::Alt => Self::Alt,
            config_lib::Modifier::Super => Self::Super,
            config_lib::Modifier::Ctrl => Self::Ctrl,
            config_lib::Modifier::None => Self::None,
        }
    }
}

impl From<Option<config_lib::Switch>> for Switch {
    fn from(value: Option<config_lib::Switch>) -> Self {
        let enabled = value.is_some();
        let v = value.unwrap_or_default();
        Self {
            enabled,
            modifier: v.modifier.into(),
            key: "tab".to_string(),
            same_class: v.filter_by.contains(&config_lib::FilterBy::SameClass),
            current_workspace: v
                .filter_by
                .contains(&config_lib::FilterBy::CurrentWorkspace),
            current_monitor: v.filter_by.contains(&config_lib::FilterBy::CurrentMonitor),
            switch_workspaces: v.switch_workspaces,
        }
    }
}

impl From<Option<config_lib::Overview>> for Overview {
    fn from(value: Option<config_lib::Overview>) -> Self {
        let enabled = value.is_some();
        let v = value.unwrap_or_default();
        Self {
            enabled,
            launcher: v.launcher.into(),
            key: v.key.to_string(),
            modifier: v.modifier.into(),
            same_class: v.filter_by.contains(&config_lib::FilterBy::SameClass),
            current_workspace: v
                .filter_by
                .contains(&config_lib::FilterBy::CurrentWorkspace),
            current_monitor: v.filter_by.contains(&config_lib::FilterBy::CurrentMonitor),
        }
    }
}

impl From<config_lib::Launcher> for Launcher {
    fn from(value: config_lib::Launcher) -> Self {
        Self {
            default_terminal: value.default_terminal.map(|s| s.to_string()),
            launch_modifier: value.launch_modifier.into(),
            width: value.width,
            max_items: value.max_items,
            show_when_empty: value.show_when_empty,
            plugins: value.plugins.into(),
        }
    }
}

impl From<Option<config_lib::EmptyConfig>> for EmptyConfig {
    fn from(value: Option<config_lib::EmptyConfig>) -> Self {
        let enabled = value.is_some();
        Self { enabled }
    }
}

impl From<Option<config_lib::ActionsPluginConfig>> for ActionsPluginConfig {
    fn from(value: Option<config_lib::ActionsPluginConfig>) -> Self {
        let enabled = value.is_some();
        let v = value.unwrap_or_default();
        Self {
            enabled,
            actions: v.actions,
        }
    }
}

impl From<Option<config_lib::ApplicationsPluginConfig>> for ApplicationsPluginConfig {
    fn from(value: Option<config_lib::ApplicationsPluginConfig>) -> Self {
        let enabled = value.is_some();
        let v = value.unwrap_or_default();
        Self {
            enabled,
            run_cache_weeks: v.run_cache_weeks,
            show_execs: v.show_execs,
            show_actions_submenu: v.show_actions_submenu,
        }
    }
}

impl From<Option<config_lib::WebSearchConfig>> for WebSearchConfig {
    fn from(value: Option<config_lib::WebSearchConfig>) -> Self {
        let enabled = value.is_some();
        let v = value.unwrap_or_default();
        Self {
            enabled,
            engines: v.engines,
        }
    }
}

impl From<config_lib::Plugins> for Plugins {
    fn from(value: config_lib::Plugins) -> Self {
        Self {
            applications: value.applications.into(),
            terminal: value.terminal.into(),
            shell: value.shell.into(),
            websearch: value.websearch.into(),
            calc: value.calc.into(),
            path: value.path.into(),
            actions: value.actions.into(),
        }
    }
}

impl From<Option<config_lib::Windows>> for Windows {
    fn from(value: Option<config_lib::Windows>) -> Self {
        let enabled = value.is_some();
        let v = value.unwrap_or_default();
        Self {
            enabled,
            scale: v.scale,
            items_per_row: v.items_per_row,
            overview: v.overview.into(),
            switch: v.switch.into(),
            switch_2: v.switch_2.into(),
        }
    }
}
