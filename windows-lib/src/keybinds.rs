use core_lib::binds::{ExecBind, generate_transfer_socat};
use core_lib::config::{FilterBy, Mod, Windows};
use core_lib::transfer::{CloseSwitchConfig, OpenOverview, OpenSwitch, TransferType};

pub fn generate_open_keybinds(windows: &Windows) -> Vec<ExecBind> {
    let mut binds = Vec::new();
    if let Some(overview) = &windows.overview {
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
            items_per_row: windows.items_per_row,
            scale: windows.scale,
        });

        binds.push(ExecBind {
            mods: vec![Mod::Super],
            key: Box::from("super_r"),
            on_release: false,
            exec: generate_transfer_socat(&config).into_boxed_str(),
        });
        binds.push(ExecBind {
            mods: vec![Mod::Super],
            key: Box::from("super_l"),
            on_release: false,
            exec: generate_transfer_socat(&config).into_boxed_str(),
        });
    }
    if let Some(switch) = &windows.switch {
        let config = TransferType::OpenSwitch(OpenSwitch {
            filter_current_workspace: switch
                .other
                .filter_by
                .iter()
                .any(|f| f == &FilterBy::CurrentWorkspace),
            filter_current_monitor: switch
                .other
                .filter_by
                .iter()
                .any(|f| f == &FilterBy::CurrentMonitor),
            filter_same_class: switch
                .other
                .filter_by
                .iter()
                .any(|f| f == &FilterBy::SameClass),
            items_per_row: windows.items_per_row,
            scale: windows.scale,
            reverse: false,
        });
        binds.push(ExecBind {
            mods: vec![Mod::Alt],
            key: Box::from("tab"),
            on_release: false,
            exec: generate_transfer_socat(&config).into_boxed_str(),
        });

        let config_r = TransferType::OpenSwitch(OpenSwitch {
            filter_current_workspace: switch
                .other
                .filter_by
                .iter()
                .any(|f| f == &FilterBy::CurrentWorkspace),
            filter_current_monitor: switch
                .other
                .filter_by
                .iter()
                .any(|f| f == &FilterBy::CurrentMonitor),
            filter_same_class: switch
                .other
                .filter_by
                .iter()
                .any(|f| f == &FilterBy::SameClass),
            items_per_row: windows.items_per_row,
            scale: windows.scale,
            reverse: true,
        });
        binds.push(ExecBind {
            mods: vec![Mod::Alt],
            key: Box::from("grave"),
            on_release: false,
            exec: generate_transfer_socat(&config_r).into_boxed_str(),
        });
        binds.push(ExecBind {
            mods: vec![Mod::Alt, Mod::Shift],
            key: Box::from("tab"),
            on_release: false,
            exec: generate_transfer_socat(&config_r).into_boxed_str(),
        });

        let config_close = TransferType::CloseSwitch(CloseSwitchConfig::None);

        // release
        binds.push(ExecBind {
            mods: vec![Mod::Alt],
            key: Box::from("alt_l"),
            on_release: true,
            exec: generate_transfer_socat(&config_close).into_boxed_str(),
        });
        // binds.push(ExecBind {
        //     mods: vec![switch.open.modifier],
        //     key: Box::from("grave"),
        //     on_release: true,
        //     exec: generate_transfer_socat(&config_close).into_boxed_str(),
        // });
        // binds.push(ExecBind {
        //     mods: vec![switch.open.modifier],
        //     key: Box::from("tab"),
        //     on_release: true,
        //     exec: generate_transfer_socat(&config_close).into_boxed_str(),
        // });
        // binds.push(ExecBind {
        //     mods: vec![switch.open.modifier, Mod::Shift],
        //     key: Box::from("tab"),
        //     on_release: true,
        //     exec: generate_transfer_socat(&config_close).into_boxed_str(),
        // });
    }

    binds
}
