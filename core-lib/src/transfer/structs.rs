use crate::{ClientId, WorkspaceId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TransferType {
    /// send from the keybind to open the overview
    OpenOverview(OpenOverview),
    /// send from the keybind to open the switch
    OpenSwitch(OpenSwitch),
    /// send from the keybinds like arrow keys or tab on overview
    SwitchOverview(SwitchOverviewConfig),
    /// send from the keybinds like arrow keys or tab on switch
    SwitchSwitch(SwitchSwitchConfig),
    /// send by pressing enter / ctrl + <n> / or from the gui itself to close the overview / switch
    CloseOverview(CloseOverviewConfig),
    /// send by pressing enter / ctrl + <n> / or from the gui itself to close the overview / switch
    CloseSwitch(CloseSwitchConfig),
    /// send from the gui itself when typing the launcher
    Type(String),
    /// send from pressing ESC
    Exit,
    /// send from the app itself when new monitor / config changes detected
    Restart,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenSwitch {
    pub scale: f64,
    pub items_per_row: u8,
    pub filter_current_workspace: bool,
    pub filter_current_monitor: bool,
    pub filter_same_class: bool,
    pub reverse: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenOverview {
    pub hide_filtered: bool,
    pub scale: f64,
    pub items_per_row: u8,
    pub filter_current_workspace: bool,
    pub filter_current_monitor: bool,
    pub filter_same_class: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwitchOverviewConfig {
    pub direction: Direction,
    pub workspace: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwitchSwitchConfig {
    pub reverse: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CloseOverviewConfig {
    LauncherClick(Identifier),
    LauncherPress(char),
    Windows(WindowsOverride),
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CloseSwitchConfig {
    Windows(ClientId),
    None,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PluginNames {
    Applications,
    Shell,
    Terminal,
    WebSearch,
    Calc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub plugin: PluginNames,
    pub identifier: Option<Box<str>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WindowsOverride {
    ClientId(ClientId),
    WorkspaceID(WorkspaceId),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}
