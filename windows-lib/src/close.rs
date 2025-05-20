use crate::WindowsGlobal;
use core_lib::{FindByFirst, IdOverride, Warn};
use exec_lib::switch::{switch_client, switch_workspace};
use exec_lib::{activate_submap, to_client_address};
use gtk::prelude::*;
use tracing::{debug, span, trace, Level};

pub fn close_overview(global: &WindowsGlobal, ids: Option<Option<IdOverride>>) {
    let _span = span!(Level::TRACE, "close_overview").entered();

    activate_submap("reset").warn("Failed to reset submap");

    if let Some(ids) = ids {
        let ids = match ids {
            None => {
                let data1 = global.data.borrow();
                data1
                    .active
                    .client
                    .map(IdOverride::ClientId)
                    .unwrap_or_else(|| IdOverride::WorkspaceID(data1.active.workspace))
            }
            Some(IdOverride::ClientId(client_id)) => IdOverride::ClientId(client_id),
            Some(IdOverride::WorkspaceID(workspace_id)) => IdOverride::WorkspaceID(workspace_id),
        };
        match ids {
            IdOverride::ClientId(client_id) => {
                let data = global.data.borrow();
                debug!(
                    "Switching to client {}",
                    data.hypr_data
                        .clients
                        .find_by_first(&client_id)
                        .map(|c| c.title.clone())
                        .unwrap_or_else(|| "<Unknown>".to_string())
                );
                switch_client(to_client_address(client_id))
                    .warn(&format!("Failed to execute with id {client_id:?}"));
            }
            IdOverride::WorkspaceID(workspace_id) => {
                let data = global.data.borrow();
                debug!(
                    "Switching to workspace {}",
                    data.hypr_data
                        .workspaces
                        .find_by_first(&workspace_id)
                        .map(|c| c.name.clone())
                        .unwrap_or_else(|| "<Unknown>".to_string())
                );
                switch_workspace(workspace_id).warn(&format!(
                    "Failed to execute switch workspace with id {workspace_id:?}"
                ));
            }
        }
    }
    let mut data1 = global.data.borrow_mut();
    for (window, monitor_data) in &mut data1.monitor_list.iter_mut() {
        while let Some(child) = monitor_data.workspaces_flow.first_child() {
            monitor_data.workspaces_flow.remove(&child);
        }
        trace!("Hiding window (windows) {:?}", window.id());
        window.set_visible(false);
    }
}
