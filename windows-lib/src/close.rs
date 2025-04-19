use core_lib::{FindByFirst, Warn};
use exec_lib::switch::{switch_client, switch_workspace};
use exec_lib::{activate_submap, to_client_address};
use gtk::prelude::{ApplicationWindowExt, WidgetExt};
use tracing::{debug, span, trace, Level};
use crate::WindowsGlobal;

pub async fn close_overview(kill: bool, global: &WindowsGlobal) {
    let _span = span!(Level::TRACE, "close_overview").entered();

    activate_submap("reset").warn("Failed to reset submap");

    let active = {
        let mut data1 = global.data.borrow_mut();
        for (window, monitor_data) in &mut data1.monitor_list.iter_mut() {
            while let Some(child) = monitor_data.workspaces_flow.first_child() {
                monitor_data.workspaces_flow.remove(&child);
            }
            trace!("Hiding window {:?}", window.id());
            window.set_visible(false);
        }
        data1.active
    };

    if !kill {
        match (active.client, active.workspace) {
            (Some(cid), _) => {
                {
                    let data = global.data.borrow();
                    debug!(
                        "Switching to client {}",
                        data.hypr_data
                            .clients
                            .find_by_first(&cid)
                            .map(|c| c.title.clone())
                            .unwrap_or_else(|| "<Unknown>".to_string())
                    );
                }
                switch_client(&to_client_address(cid))
                    .await
                    .warn(&format!("Failed to execute with id {cid:?}"));
            }
            (_, wid) => {
                {
                    let data = global.data.borrow();
                    debug!(
                        "Switching to workspace {}",
                        data.hypr_data
                            .workspaces
                            .find_by_first(&wid)
                            .map(|c| c.name.clone())
                            .unwrap_or_else(|| "<Unknown>".to_string())
                    );
                }
                switch_workspace(wid).await.warn(&format!(
                    "Failed to execute switch workspace with id {wid:?}"
                ));
            }
        };
    }
}
