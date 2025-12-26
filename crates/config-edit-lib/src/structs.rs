use std::fmt;
use std::fmt::Formatter;

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
    pub exclude_special_workspaces: String,
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
    pub exclude_special_workspaces: String,
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
        &["None", "Alt", "Super", "Ctrl"]
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
            0 => Ok(Self::None),
            1 => Ok(Self::Super),
            2 => Ok(Self::Ctrl),
            3 => Ok(Self::Alt),
            _ => Err(()),
        }
    }
}

impl Into<u32> for ConfigModifier {
    fn into(self) -> u32 {
        match self {
            ConfigModifier::None => 0,
            ConfigModifier::Super => 1,
            ConfigModifier::Ctrl => 2,
            ConfigModifier::Alt => 3,
        }
    }
}

impl From<config_lib::Modifier> for ConfigModifier {
    fn from(value: config_lib::Modifier) -> Self {
        match value {
            config_lib::Modifier::None => Self::None,
            config_lib::Modifier::Alt => Self::Alt,
            config_lib::Modifier::Super => Self::Super,
            config_lib::Modifier::Ctrl => Self::Ctrl,
        }
    }
}
impl Into<config_lib::Modifier> for ConfigModifier {
    fn into(self) -> config_lib::Modifier {
        match self {
            Self::None => config_lib::Modifier::None,
            Self::Alt => config_lib::Modifier::Alt,
            Self::Super => config_lib::Modifier::Super,
            Self::Ctrl => config_lib::Modifier::Ctrl,
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
impl Into<Option<config_lib::Windows>> for Windows {
    fn into(self) -> Option<config_lib::Windows> {
        if self.enabled {
            Some(config_lib::Windows {
                scale: self.scale,
                items_per_row: self.items_per_row,
                overview: self.overview.into(),
                switch: self.switch.into(),
                switch_2: self.switch_2.into(),
            })
        } else {
            None
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
            key: v.key.to_string(),
            same_class: v.filter_by.contains(&config_lib::FilterBy::SameClass),
            current_workspace: v
                .filter_by
                .contains(&config_lib::FilterBy::CurrentWorkspace),
            current_monitor: v.filter_by.contains(&config_lib::FilterBy::CurrentMonitor),
            switch_workspaces: v.switch_workspaces,
            // TODO
            exclude_special_workspaces: "".to_string(),
        }
    }
}
impl Into<Option<config_lib::Switch>> for Switch {
    fn into(self) -> Option<config_lib::Switch> {
        if self.enabled {
            let mut vec = vec![];
            if self.same_class {
                vec.push(config_lib::FilterBy::SameClass);
            }
            if self.current_workspace {
                vec.push(config_lib::FilterBy::CurrentWorkspace);
            }
            if self.current_monitor {
                vec.push(config_lib::FilterBy::CurrentMonitor);
            }
            Some(config_lib::Switch {
                modifier: self.modifier.into(),
                key: Box::from(self.key),
                filter_by: vec,
                switch_workspaces: self.switch_workspaces,
                // TODO exclude_special_workspaces
            })
        } else {
            None
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
            // TODO
            exclude_special_workspaces: "".to_string(),
        }
    }
}
impl Into<Option<config_lib::Overview>> for Overview {
    fn into(self) -> Option<config_lib::Overview> {
        if self.enabled {
            let mut vec = vec![];
            if self.same_class {
                vec.push(config_lib::FilterBy::SameClass);
            }
            if self.current_workspace {
                vec.push(config_lib::FilterBy::CurrentWorkspace);
            }
            if self.current_monitor {
                vec.push(config_lib::FilterBy::CurrentMonitor);
            }
            Some(config_lib::Overview {
                launcher: self.launcher.into(),
                key: Box::from(self.key),
                modifier: self.modifier.into(),
                filter_by: vec,
                hide_filtered: false,
                // TODO exclude_special_workspaces
            })
        } else {
            None
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
impl Into<config_lib::Launcher> for Launcher {
    fn into(self) -> config_lib::Launcher {
        config_lib::Launcher {
            default_terminal: self.default_terminal.map(Box::from),
            launch_modifier: self.launch_modifier.into(),
            width: self.width,
            max_items: self.max_items,
            show_when_empty: self.show_when_empty,
            plugins: self.plugins.into(),
        }
    }
}

impl From<Option<config_lib::EmptyConfig>> for EmptyConfig {
    fn from(value: Option<config_lib::EmptyConfig>) -> Self {
        let enabled = value.is_some();
        Self { enabled }
    }
}
impl Into<Option<config_lib::EmptyConfig>> for EmptyConfig {
    fn into(self) -> Option<config_lib::EmptyConfig> {
        if self.enabled {
            Some(config_lib::EmptyConfig {})
        } else {
            None
        }
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
impl Into<Option<config_lib::ActionsPluginConfig>> for ActionsPluginConfig {
    fn into(self) -> Option<config_lib::ActionsPluginConfig> {
        if self.enabled {
            Some(config_lib::ActionsPluginConfig {
                actions: self.actions,
            })
        } else {
            None
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
impl Into<Option<config_lib::ApplicationsPluginConfig>> for ApplicationsPluginConfig {
    fn into(self) -> Option<config_lib::ApplicationsPluginConfig> {
        if self.enabled {
            Some(config_lib::ApplicationsPluginConfig {
                run_cache_weeks: self.run_cache_weeks,
                show_execs: self.show_execs,
                show_actions_submenu: self.show_actions_submenu,
            })
        } else {
            None
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
impl Into<Option<config_lib::WebSearchConfig>> for WebSearchConfig {
    fn into(self) -> Option<config_lib::WebSearchConfig> {
        if self.enabled {
            Some(config_lib::WebSearchConfig {
                engines: self.engines,
            })
        } else {
            None
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
impl Into<config_lib::Plugins> for Plugins {
    fn into(self) -> config_lib::Plugins {
        config_lib::Plugins {
            applications: self.applications.into(),
            terminal: self.terminal.into(),
            shell: self.shell.into(),
            websearch: self.websearch.into(),
            calc: self.calc.into(),
            path: self.path.into(),
            actions: self.actions.into(),
        }
    }
}
