use crate::data::{collect_data, SortConfig};
use crate::icon::set_icon;
use crate::next::find_next;
use crate::WindowsGlobal;
use anyhow::Context;
use core_lib::transfer::{CloseConfig, OpenSwitch, TransferType, WindowsOverride};
use core_lib::{send_to_socket, ClientData, ClientId, FindByFirst, Warn};
use exec_lib::activate_submap;
use gtk::prelude::*;
use gtk::{pango, EventSequenceState, Frame, GestureClick, Image, Label, Overflow, Overlay};
use std::cmp::min;
use tracing::{debug, span, trace, Level};

fn scale(value: i16, size_factor: f64) -> i32 {
    (value as f64 / 31.0 * size_factor) as i32
}
pub fn open_switch(global: &WindowsGlobal, config: OpenSwitch) -> anyhow::Result<()> {
    let _span = span!(Level::TRACE, "open_switch").entered();
    let (clients_data, active) = collect_data(&SortConfig {
        filter_current_monitor: config.filter_current_monitor,
        filter_current_workspace: config.filter_current_workspace,
        filter_same_class: config.filter_same_class,
        sort_recent: true,
    })
    .context("Failed to collect data")?;
    let active = find_next(
        &config.direction,
        false,
        &clients_data,
        active,
        global.workspaces_per_row as usize,
    );

    activate_submap(&config.submap_name)?;

    let mut data = global.data.borrow_mut();
    for (window, monitor_data) in &mut data.monitor_list.iter_mut() {
        let monitor = if let Some(monitor) = clients_data.monitors.find_by_first(&monitor_data.id) {
            monitor
        } else {
            continue;
        };
        if config.hide_filtered && !monitor.enabled {
            continue;
        }

        let clients: Vec<&(ClientId, ClientData)> = {
            let mut clients = clients_data
                .clients
                .iter()
                .filter(|(_, client)| client.monitor == monitor_data.id)
                .collect::<Vec<_>>();
            clients.sort_by(|(_, a), (_, b)| {
                // prefer smaller windows
                if a.floating && b.floating {
                    (b.width * b.height).cmp(&(a.width * a.height))
                } else {
                    a.floating.cmp(&b.floating)
                }
            });
            clients
        };
        for (address, client) in clients {
            if config.hide_filtered && !client.enabled {
                continue;
            }
            let client_overlay = {
                let title = if !client.title.trim().is_empty() {
                    &client.title
                } else {
                    &client.class
                };
                let client_label = Label::builder()
                    .label(title)
                    .overflow(Overflow::Visible)
                    .margin_start(6)
                    .ellipsize(pango::EllipsizeMode::End)
                    .build();
                let client_frame = Frame::builder()
                    .label_xalign(0.5)
                    .label_widget(&client_label)
                    .build();

                // hide picture if client so small
                // 2 => > infinity
                // 2.1  > 800
                // 3 => > 800
                // 3.9  > 800
                // 4 => > 538
                // 5 => > 473
                // 6 => > 408
                // 7 => > 343
                // 8 => > 278
                // 9 => > 250
                if match global.size_factor {
                    ..2.5 => false,
                    2.5..3.9 => monitor.height > 800,
                    _ => {
                        monitor.height as i16
                            > 700 - min(((global.size_factor - 1.5) * 65.0) as i16, 450)
                    }
                } {
                    let image = Image::builder()
                        .css_classes(["client-image"])
                        .pixel_size(
                            (scale(monitor.height as i16, global.size_factor).clamp(50, 200) as f64
                                / 1.5) as i32
                                - 20,
                        )
                        .build();
                    if !client.enabled {
                        image.add_css_class("monochrome");
                    }
                    set_icon(&client.class, client.pid, &image);
                    client_frame.set_child(Some(&image));
                }

                let client_overlay = Overlay::builder()
                    .css_classes(["client"])
                    .overflow(Overflow::Hidden)
                    .child(&client_frame)
                    .width_request(scale(monitor.width as i16, global.size_factor))
                    .height_request(scale(monitor.height as i16, global.size_factor))
                    .build();

                // add border around initial active client
                if active.client == Some(*address) {
                    client_overlay.add_css_class("active");
                }

                client_overlay.add_controller(click_client(*address));
                client_overlay
            };
            monitor_data.workspaces_flow.insert(&client_overlay, -1);
            monitor_data.client_refs.insert(*address, client_overlay);
        }

        trace!("Showing window {:?}", window.id());
        window.set_visible(true);

        trace!("Refresh window {:?}", window.id());
    }

    data.active = active;
    data.hypr_data = clients_data;
    drop(data);
    Ok(())
}

fn click_client(client_id: ClientId) -> GestureClick {
    let gesture = GestureClick::new();
    gesture.connect_pressed(move |gesture, _, _, _| {
        gesture.set_state(EventSequenceState::Claimed);
        debug!("Exiting on click of client box");
        send_to_socket(&TransferType::Close(CloseConfig::Windows(
            WindowsOverride::ClientId(client_id),
        )))
        .warn("unable send return to socket");
    });
    gesture
}
