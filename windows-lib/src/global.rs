use core_lib::{Active, ClientId, HyprlandData, MonitorId, WorkspaceId};
use gtk::{ApplicationWindow, Button, FlowBox};
use launcher_lib::LauncherData;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct WindowsGlobal {
    pub overview: Option<WindowsOverviewData>,
    pub switch: Option<WindowsSwitchData>,
}

#[derive(Debug)]
pub struct WindowsOverviewData {
    pub window_list: HashMap<ApplicationWindow, WindowsOverviewMonitorData>,
    pub active: Active,
    pub hypr_data: HyprlandData,
    pub launcher: LauncherData,
}

#[derive(Debug)]
pub struct WindowsSwitchData {
    pub window: ApplicationWindow,
    pub clients_flow: FlowBox,
    pub clients: HashMap<ClientId, Button>,
    pub active: Active,
    pub hypr_data: HyprlandData,
}

#[derive(Debug)]
pub struct WindowsOverviewMonitorData {
    pub id: MonitorId,
    // used to store a ref to the FlowBox containing the workspaces
    pub workspaces_flow: FlowBox,
    // used to store refs to the Overlays over the workspace Frames
    pub workspace_refs: HashMap<WorkspaceId, Button>,
    // used to store refs to the Overlays containing the clients
    pub client_refs: HashMap<ClientId, Button>,
}

impl WindowsOverviewMonitorData {
    pub fn new(id: MonitorId, workspaces_flow: FlowBox) -> Self {
        Self {
            id,
            workspaces_flow,
            workspace_refs: HashMap::new(),
            client_refs: HashMap::new(),
        }
    }
}
