use crate::start::Globals;
use crate::util::reload_desktop_data;
use async_channel::Receiver;
use core_lib::transfer::{
    CloseConfig, OpenOverview, OpenSwitch, SwitchConfig, TransferType, WindowsOverride,
};
use core_lib::{IdOverride, Warn, collect_desktop_files};
use gtk::prelude::EntryExt;
use tracing::{debug, warn};

pub async fn event_handler(globals: Globals, event_receiver: Receiver<TransferType>) {
    loop {
        if let Ok(transfer) = event_receiver.recv().await {
            let close_socket = matches!(transfer, TransferType::Restart);
            match transfer {
                TransferType::OpenOverview(config) => open_overview(&globals, config),
                TransferType::OpenSwitch(config) => open_switch(&globals, config),
                TransferType::Switch(config) => switch(&globals, config),
                TransferType::Exit => exit(&globals),
                TransferType::Type(text) => r#type(&globals, text),
                TransferType::Close(config) => close(&globals, config),
                TransferType::Restart => restart(&globals),
            }
            if close_socket {
                return;
            }
        }
    }
}

macro_rules! if_some {
    ($global:expr, $func:path $(, $args:expr)*) => {
        if let Some(_gl) = &$global {
            $func(_gl $(, $args)*)
        }
    };
}

fn open_overview(global: &Globals, config: OpenOverview) {
    if let Some(global) = &global.windows {
        if let Some(mut overview) = &global.overview {
            windows_lib::open_overview(&mut overview, config)
                .warn("Failed to open overview window");
            launcher_lib::open_launcher(&mut overview.launcher)
        } else {
            warn!("Window overview not active");
        }
    } else {
        warn!("Windows not active");
    };
}

fn open_switch(global: &Globals, config: OpenSwitch) {
    if let Some(global) = &global.windows {
        if let Some(mut switch) = &global.switch {
            windows_lib::open_switch(&mut switch, config).warn("Failed to open switch window");
        } else {
            warn!("Window switch not active");
        }
    } else {
        warn!("Windows not active");
    }
}

fn switch_switch(global: &Globals, config: SwitchConfig) {
    if let Some(global) = &global.windows {
        if let Some(mut switch) = &global.switch {
            windows_lib::update_switch(&mut switch, config)
        } else {
            warn!("Window switch not active");
        }
    } else {
        warn!("Windows not active");
    }
}
fn switch_overview(global: &Globals, config: SwitchConfig) {
    if let Some(global) = &global.windows {
        if let Some(mut overview) = &global.overview {
            // don't switch selected window if launcher is active
            let launch = overview.launcher.entry.text_length() > 0;
            if !launch {
                windows_lib::update_overview(&mut overview, config);
            }
        } else {
            warn!("Window switch not active");
        }
    } else {
        warn!("Windows not active");
    }
}
fn exit(global: &Globals) {
    if_some!(global.launcher, launcher_lib::close_launcher_press, None);
    if_some!(global.windows, windows_lib::close_windows, None);
    reload_desktop_data();
}

fn r#type(global: &Globals, text: String) {
    if_some!(global.launcher, launcher_lib::update_launcher, text);
}

fn close(global: &Globals, config: CloseConfig) {
    match config {
        CloseConfig::None => {
            let (launcher_active, launcher_empty) = global
                .launcher
                .as_ref()
                .and_then(|l| {
                    l.data.as_ref().map(|d| {
                        let b = d.borrow();
                        (b.entry.text_length() > 0, b.sorted_matches.is_empty())
                    })
                })
                .unwrap_or((false, false));

            if launcher_active {
                if !launcher_empty {
                    // kill overview, close launcher
                    if_some!(
                        global.launcher,
                        launcher_lib::close_launcher_press,
                        Some('0')
                    );
                    if_some!(global.windows, windows_lib::close_windows, None);
                } else {
                    debug!("Launcher is empty, not closing");
                }
            } else {
                // close overview, kill launcher
                if_some!(global.launcher, launcher_lib::close_launcher_press, None);
                if_some!(global.windows, windows_lib::close_windows, Some(None));
            };
        }
        CloseConfig::LauncherClick(iden) => {
            // kill overview, close launcher
            if_some!(global.launcher, launcher_lib::close_launcher_click, iden);
            if_some!(global.windows, windows_lib::close_windows, None);
        }
        CloseConfig::LauncherPress(char) => {
            // kill overview, close launcher
            if_some!(
                global.launcher,
                launcher_lib::close_launcher_press,
                Some(char)
            );
            if_some!(global.windows, windows_lib::close_windows, None);
        }
        CloseConfig::Windows(WindowsOverride::ClientId(client_id)) => {
            // close overview, kill launcher
            if_some!(global.launcher, launcher_lib::close_launcher_press, None);
            if_some!(
                global.windows,
                windows_lib::close_windows,
                Some(Some(IdOverride::ClientId(client_id)))
            );
        }
        CloseConfig::Windows(WindowsOverride::WorkspaceID(workspace_id)) => {
            // close overview, kill launcher
            if_some!(global.launcher, launcher_lib::close_launcher_press, None);
            if_some!(
                global.windows,
                windows_lib::close_windows,
                Some(Some(IdOverride::WorkspaceID(workspace_id)))
            );
        }
    }
    reload_desktop_data();
}

fn restart(global: &Globals) {
    if_some!(&global.windows, windows_lib::stop_overview);
    if_some!(&global.launcher, launcher_lib::stop_launcher);
}
