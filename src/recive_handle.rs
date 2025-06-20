use crate::start::Globals;
use crate::util::reload_desktop_data;
use async_channel::{Receiver, Sender};
use core_lib::transfer::{
    CloseOverviewConfig, CloseSwitchConfig, OpenOverview, OpenSwitch, SwitchOverviewConfig,
    SwitchSwitchConfig, TransferType, WindowsOverride,
};
use core_lib::{Warn, collect_desktop_files};
use gtk::prelude::EntryExt;
use tracing::{debug, warn};

pub async fn event_handler(
    globals: Globals,
    event_receiver: Receiver<TransferType>,
    event_sender: Sender<TransferType>,
) {
    loop {
        if let Ok(transfer) = event_receiver.recv().await {
            let close_socket = matches!(transfer, TransferType::Restart);
            match transfer {
                TransferType::OpenOverview(config) => {
                    open_overview(&globals, config, event_sender.clone())
                }
                TransferType::OpenSwitch(config) => {
                    open_switch(&globals, config, event_sender.clone())
                }
                TransferType::SwitchOverview(config) => switch_overview(&globals, config),
                TransferType::SwitchSwitch(config) => switch_switch(&globals, config),
                TransferType::Exit => exit(&globals),
                TransferType::Type(text) => r#type(&globals, text, event_sender.clone()),
                TransferType::CloseOverview(config) => close_overview(&globals, config),
                TransferType::CloseSwitch(config) => close_switch(&globals, config),
                TransferType::Restart => restart(&globals),
            }
            if close_socket {
                return;
            }
        }
    }
}

fn r#type(global: &Globals, text: String, event_sender: Sender<TransferType>) {
    if let Some(windows) = &global.windows {
        if let Some(overview) = &windows.overview {
            launcher_lib::update_launcher(overview.launcher, text, event_sender)
        }
    }
}

fn open_overview(global: &Globals, config: OpenOverview, event_sender: Sender<TransferType>) {
    if let Some(windows) = &global.windows {
        if let Some(overview) = &windows.overview {
            windows_lib::open_overview(overview, config, event_sender)
                .warn("Failed to open overview window");
            launcher_lib::open_launcher(overview.launcher)
        } else {
            warn!("Window overview not active");
        }
    } else {
        warn!("Windows not active");
    };
}

fn open_switch(global: &Globals, config: OpenSwitch, event_sender: Sender<TransferType>) {
    if let Some(windows) = &global.windows {
        if let Some(switch) = &windows.switch {
            windows_lib::open_switch(switch, config, event_sender)
                .warn("Failed to open switch window");
        } else {
            warn!("Window switch not active");
        }
    } else {
        warn!("Windows not active");
    }
}

fn switch_switch(global: &Globals, config: SwitchSwitchConfig) {
    if let Some(windows) = &global.windows {
        if let Some(switch) = &windows.switch {
            windows_lib::update_switch(switch, config)
        } else {
            warn!("Window switch not active");
        }
    } else {
        warn!("Windows not active");
    }
}
fn switch_overview(global: &Globals, config: SwitchOverviewConfig) {
    if let Some(windows) = &global.windows {
        if let Some(overview) = &windows.overview {
            // don't switch selected window if launcher is active
            let launch = overview.launcher.entry.text_length() > 0;
            if !launch {
                windows_lib::update_overview(overview, config);
            }
        } else {
            warn!("Window switch not active");
        }
    } else {
        warn!("Windows not active");
    }
}
fn exit(global: &Globals) {
    if let Some(windows) = &global.windows {
        if let Some(overview) = &windows.overview {
            windows_lib::close_overview(overview, None);
            launcher_lib::close_launcher_by_char(overview.launcher, None); // this will never open a program and need the default terminal
        };
        if let Some(switch) = &windows.switch {
            windows_lib::close_switch(switch, None);
        };
    }
    reload_desktop_data();
}

fn close_overview(global: &Globals, config: CloseOverviewConfig) {
    if let Some(windows) = &global.windows {
        if let Some(overview) = &windows.overview {
            match config {
                // return (focus active)
                CloseOverviewConfig::None => {
                    let launcher_empty = overview.launcher.entry.text_length() > 0;
                    let launcher_has_items = overview.launcher.sorted_matches.is_empty();
                    if launcher_empty {
                        // close overview, kill launcher
                        windows_lib::close_overview(overview, Some(None));
                        launcher_lib::close_launcher_by_char(overview.launcher, None);
                    } else {
                        if launcher_has_items {
                            // kill overview, close launcher
                            windows_lib::close_overview(overview, None);
                            launcher_lib::close_launcher_by_char(overview.launcher, Some('0'));
                        } else {
                            debug!("Launcher is empty, not closing");
                        }
                    };
                }
                // clicked on launcher item
                CloseOverviewConfig::LauncherClick(iden) => {
                    windows_lib::close_overview(overview, None);
                    launcher_lib::close_launcher_by_iden(overview.launcher, &iden);
                }
                // typed a character in launcher
                CloseOverviewConfig::LauncherPress(iden) => {
                    windows_lib::close_overview(overview, None);
                    launcher_lib::close_launcher_by_char(overview.launcher, Some(iden));
                }
                // clicked on window
                CloseOverviewConfig::Windows(iden) => {
                    windows_lib::close_overview(overview, Some(Some(iden)));
                    launcher_lib::close_launcher_by_char(overview.launcher, None);
                }
            }
        }
    }
    reload_desktop_data()
}

fn close_switch(global: &Globals, config: CloseSwitchConfig) {
    if let Some(windows) = &global.windows {
        if let Some(switch) = &windows.switch {
            match config {
                CloseSwitchConfig::Windows(iden) => {
                    windows_lib::close_switch(switch, Some(iden));
                }
                CloseSwitchConfig::None => {
                    windows_lib::close_switch(switch, None);
                }
            }
        }
    }
    reload_desktop_data()
}

fn restart(global: &Globals) {
    if let Some(windows) = &global.windows {
        if let Some(overview) = &windows.overview {
            windows_lib::stop_overview(overview);
            launcher_lib::stop_launcher(&overview.launcher);
        };
        if let Some(switch) = &windows.switch {
            windows_lib::stop_switch(switch);
        };
    }
}
