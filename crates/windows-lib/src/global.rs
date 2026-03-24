use core_lib::{Active, ClientId, HyprlandData, WorkspaceId};
use relm4::adw::gtk::{ApplicationWindow, Button, FlowBox};
use std::collections::HashMap;

#[derive(Debug)]
pub struct WindowsSwitchData {
    pub config: WindowsSwitchConfig,
    pub window: ApplicationWindow,
    pub main_flow: FlowBox,
    pub workspaces: HashMap<WorkspaceId, Button>,
    pub clients: HashMap<ClientId, Button>,
    pub active: Active,
    pub hypr_data: HyprlandData,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug)]
pub struct WindowsSwitchConfig {
    pub items_per_row: u8,
    pub scale: f64,
    pub filter_current_workspace: bool,
    pub filter_current_monitor: bool,
    pub filter_same_class: bool,
    pub switch_workspaces: bool,
    pub show_workspace_number: bool,
    pub exclude_workspaces: Option<Box<str>>,
}
