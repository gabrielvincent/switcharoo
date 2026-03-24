use crate::Modifier;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(SmartDefault, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(not(feature = "ci_no_default_config_values"), serde(default))]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[default(crate::CURRENT_CONFIG_VERSION)]
    pub version: u16,
    #[default(None)]
    pub windows: Option<Windows>,
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(not(feature = "ci_no_default_config_values"), serde(default))]
#[serde(deny_unknown_fields)]
pub struct Windows {
    #[default = 8.5]
    pub scale: f64,
    #[default = 5]
    pub items_per_row: u8,
    #[default(None)]
    pub switch: Option<Switch>,
    #[default(None)]
    pub switch_2: Option<Switch>,
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(not(feature = "ci_no_default_config_values"), serde(default))]
#[serde(deny_unknown_fields)]
pub struct Switch {
    #[default(Modifier::Alt)]
    pub modifier: Modifier,
    #[default = "Tab"]
    pub key: Box<str>,
    #[default(vec![FilterBy::CurrentMonitor])]
    pub filter_by: Vec<FilterBy>,
    #[default = false]
    pub switch_workspaces: bool,
    #[default = ""]
    pub exclude_workspaces: Box<str>,
    #[default = true]
    pub show_workspace_number: bool,
    #[default = 'q']
    pub kill_key: char,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterBy {
    SameClass,
    CurrentWorkspace,
    CurrentMonitor,
}
