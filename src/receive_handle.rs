use crate::start::Globals;
use async_channel::{Receiver, Sender};
use core_lib::WarnWithDetails;
use core_lib::transfer::{Direction, OpenSwitch, SwitchSwitchConfig, TransferType};
use relm4::adw::gtk::prelude::ApplicationExt;
use relm4::adw::gtk::glib;
use std::thread;
use std::time::Duration;
use tracing::{debug, info, instrument, trace, warn};

#[allow(clippy::future_not_send)]
#[instrument(level = "trace", skip_all)]
pub async fn event_handler(
    mut globals: Globals,
    event_receiver: Receiver<TransferType>,
    event_sender: Sender<TransferType>,
) {
    loop {
        if let Ok(transfer) = event_receiver.recv().await {
            let close_socket = matches!(transfer, TransferType::Restart);
            trace!("handling event: {transfer:?}");
            if !globals.active
                && transfer != TransferType::SetActive
                && transfer != TransferType::Restart
            {
                debug!("Application is not active, ignoring event");
                continue;
            }
            match transfer {
                TransferType::OpenSwitch(config) => open_switch(&mut globals, &config),
                TransferType::SwitchSwitch(config) => switch_switch(&mut globals, &config),
                TransferType::CloseClientSwitch => close_client_switch(&mut globals, &event_sender),
                TransferType::CloseSwitch => close_switch(&mut globals),
                TransferType::CloseAll => close_all(&mut globals),
                TransferType::Restart => restart(&mut globals),
                TransferType::SetActive => {
                    globals.active = true;
                    info!("Application is now active");
                }
            }
            if close_socket {
                return;
            }
        }
    }
}

fn open_switch(global: &mut Globals, config: &OpenSwitch) {
    if let Some(windows) = &mut global.windows {
        if let Some(switch) = &mut windows.switch {
            if !windows_lib::switch::switch_already_open(switch) {
                windows_lib::switch::open_switch(switch, config)
                    .warn_details("Failed to open switch window");
            } else {
                debug!("Switch already open, converting to SwitchSwitch");
                windows_lib::switch::switch_to_next(
                    switch,
                    &SwitchSwitchConfig {
                        direction: if config.reverse {
                            Direction::Left
                        } else {
                            Direction::Right
                        },
                    },
                );
            }
        } else {
            warn!("Window switch not active");
        }
    } else {
        warn!("Windows not active");
    }
}

fn switch_switch(global: &mut Globals, config: &SwitchSwitchConfig) {
    if let Some(windows) = &mut global.windows {
        if let Some(switch) = &mut windows.switch {
            windows_lib::switch::switch_to_next(switch, config);
        } else {
            warn!("Window switch not active");
        }
    } else {
        warn!("Windows not active");
    }
}

fn close_client_switch(global: &mut Globals, _event_sender: &Sender<TransferType>) {
    if let Some(windows) = &mut global.windows {
        if let Some(switch) = &mut windows.switch {
            let success = windows_lib::switch::close_item(switch)
                .warn_details("Failed to close switch item")
                .unwrap_or(false);
            if success {
                windows_lib::switch::switch_to_next(
                    switch,
                    &SwitchSwitchConfig {
                        direction: Direction::Right,
                    },
                );
                windows_lib::switch::update_data(switch)
                    .warn_details("unable to update switch data");
            } else {
                windows_lib::switch::close_switch(switch, true);
            }
        } else {
            warn!("Window switch not active");
        }
    } else {
        warn!("Windows not active");
    }
}

fn close_all(global: &mut Globals) {
    if let Some(windows) = &mut global.windows {
        if let Some(switch) = &mut windows.switch {
            windows_lib::switch::close_switch(switch, false);
        }
    }
}

fn close_switch(global: &mut Globals) {
    if let Some(windows) = &mut global.windows
        && let Some(switch) = &mut windows.switch
    {
        if windows_lib::switch::switch_already_hidden(switch) {
            debug!("Switch is already closed");
            return;
        }
        windows_lib::switch::close_switch(switch, true);
    }
}

fn restart(global: &mut Globals) {
    global.active = false;
    if let Some(windows) = &global.windows {
        if let Some(switch) = &windows.switch {
            windows_lib::switch::stop_switch(switch);
        }
    }
    let app = global.app.clone();
    thread::sleep(Duration::from_millis(500));
    glib::idle_add_local_once(move || {
        app.quit();
        debug!("application closed");
    });
}
