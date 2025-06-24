use crate::config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Config {
    pub(super) layerrules: bool,
    pub(super) kill_bind: String,
    pub(super) launcher: Option<config::Launcher>,
    pub(super) windows: Option<Windows>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Windows {
    pub(super) scale: f64,
    pub(super) workspaces_per_row: u8,
    pub(super) strip_html_from_workspace_title: bool,
    pub(super) overview: Option<Overview>,
    pub(super) switch: Option<Switch>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Overview {
    pub(super) open: OpenOverview,
    pub(super) navigate: Navigate,
    pub(super) other: OtherOverview,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct OtherOverview {
    pub(super) filter_by: Vec<config::FilterBy>,
    pub(super) hide_filtered: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct OpenOverview {
    pub(super) key: KeyMaybeMod,
    pub(super) modifier: config::Modifier,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Navigate {
    forward: String,
    reverse: Reverse,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Switch {
    pub(super) open: OpenSwitch,
    pub(super) navigate: Navigate,
    pub(super) other: OtherSwitch,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct OpenSwitch {
    pub(super) modifier: config::Modifier,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct OtherSwitch {
    pub(super) filter_by: Vec<config::FilterBy>,
    pub(super) hide_filtered: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum Reverse {
    Key(String),
    Mod(config::Modifier),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct KeyMaybeMod(String);
impl From<&str> for KeyMaybeMod {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

// https://wiki.hyprland.org/Configuring/Variables/#variable-types
// SHIFT CAPS CTRL/CONTROL ALT MOD2 MOD3 SUPER/WIN/LOGO/MOD4 MOD5
impl KeyMaybeMod {
    pub(super) fn to_key(&self) -> String {
        match &*self.0.to_ascii_lowercase() {
            "alt" => "alt_l".to_string(),
            "ctrl" => "ctrl_l".to_string(),
            "super" => "super_l".to_string(),
            "shift" => "shift_l".to_string(),
            a => a.to_string(),
        }
    }
}
