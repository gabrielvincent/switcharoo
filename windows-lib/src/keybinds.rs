use core_lib::binds::ExecBind;
use core_lib::config::{FilterBy, Windows};
use core_lib::generate_transfer_socat;
use core_lib::transfer::{OpenOverview, TransferType};

pub fn generate_open_keybinds(windows: &Windows) -> Vec<ExecBind> {
    let mut binds = Vec::new();
    if let Some(overview) = &windows.overview {
        let workspaces_per_row = windows.workspaces_per_row;

        let config = TransferType::OpenOverview(OpenOverview {
            hide_filtered: overview.other.hide_filtered,
            filter_current_workspace: overview
                .other
                .filter_by
                .iter()
                .any(|f| f == &FilterBy::CurrentWorkspace),
            filter_current_monitor: overview
                .other
                .filter_by
                .iter()
                .any(|f| f == &FilterBy::CurrentMonitor),
            filter_same_class: overview
                .other
                .filter_by
                .iter()
                .any(|f| f == &FilterBy::SameClass),
            workspaces_per_row,
        });

        binds.push(ExecBind {
            mods: vec![overview.open.modifier],
            key: overview.open.key.clone(),
            flags: vec![],
            exec: generate_transfer_socat(&config).into_boxed_str(),
        });
    }

    binds
}
