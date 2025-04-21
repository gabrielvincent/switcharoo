use crate::data::{collect_data, SortConfig};
use crate::icon::set_icon;
use crate::WindowsGlobal;
use anyhow::Context;
use core_lib::transfer::{OpenOverview, Override, ReturnConfig, TransferType};
use core_lib::{send_to_socket, ClientData, ClientId, FindByFirst, Warn, WorkspaceId};
use exec_lib::activate_submap;
use gtk::prelude::*;
use gtk::{pango, EventSequenceState, Fixed, Frame, GestureClick, Image, Label, Overflow, Overlay};
use std::borrow::Cow;
use std::cmp::min;
use tracing::{debug, span, trace, Level};

fn scale(value: i16, size_factor: f64) -> i32 {
    (value as f64 / 30.0 * size_factor) as i32
}
pub fn open_overview(config: OpenOverview, global: &WindowsGlobal) -> anyhow::Result<()> {
    let _span = span!(Level::TRACE, "open_overview").entered();
    let (clients_data, active) = collect_data(&SortConfig {
        filter_current_monitor: config.filter_current_monitor,
        filter_current_workspace: config.filter_current_workspace,
        filter_same_class: config.filter_same_class,
        sort_recent: false,
    })
    .context("Failed to collect data")?;

    activate_submap(&config.submap_name)?;

    let regex = regex::Regex::new(r"<[^>]*>").context("Invalid regex")?;

    let mut data = global.data.borrow_mut();
    for (window, monitor_data) in data.monitor_list.iter_mut() {
        if config.hide_filtered
            && clients_data
                .monitors
                .find_by_first(&monitor_data.id)
                .map(|m| !m.enabled)
                .unwrap_or(false)
        {
            continue;
        }

        let workspaces = {
            let mut workspaces = clients_data
                .workspaces
                .iter()
                .filter(|(_, v)| v.monitor == monitor_data.id)
                .collect::<Vec<_>>();
            workspaces.sort_by(|(a, _), (b, _)| a.cmp(b));
            workspaces
        };

        for (wid, workspace) in workspaces {
            trace!(
                "Creating workspace {:?} with ({}x{})",
                wid,
                scale(workspace.width as i16, global.size_factor),
                scale(workspace.height as i16, global.size_factor)
            );
            let workspace_fixed = Fixed::builder()
                .width_request(scale(workspace.width as i16, global.size_factor))
                .height_request(scale(workspace.height as i16, global.size_factor))
                .build();

            let id_string = wid.to_string();
            let title = if !workspace.name.trim().is_empty() {
                if global.strip_html_from_workspace_title {
                    regex.replace_all(&workspace.name, "")
                } else {
                    Cow::from(&workspace.name)
                }
            } else {
                Cow::from(&id_string)
            };

            let workspace_frame = Frame::builder()
                .label(title)
                .label_xalign(0.5)
                .child(&workspace_fixed)
                .build();

            let workspace_overlay = {
                let workspace_overlay = Overlay::builder()
                    .css_classes(vec!["workspace"])
                    .child(&workspace_frame)
                    .build();
                workspace_overlay.add_controller(click_workspace(*wid));
                monitor_data.workspaces_flow.insert(&workspace_overlay, -1);
                workspace_overlay
            };
            monitor_data.workspace_refs.insert(*wid, workspace_overlay);

            let clients: Vec<&(ClientId, ClientData)> = {
                let mut clients = clients_data
                    .clients
                    .iter()
                    .filter(|(_, client)| client.workspace == *wid)
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
                        2.5..3.9 => client.height > 800,
                        _ => {
                            client.height
                                > 700 - min(((global.size_factor - 1.5) * 65.0) as i16, 450)
                        }
                    } {
                        let image = Image::builder()
                            .css_classes(vec!["client-image"])
                            .pixel_size(
                                (scale(client.height, global.size_factor).clamp(50, 200) as f64
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
                        .css_classes(vec!["client"])
                        .overflow(Overflow::Hidden)
                        .child(&client_frame)
                        .width_request(scale(client.width, global.size_factor))
                        .height_request(scale(client.height, global.size_factor))
                        .build();

                    // add initial border around initial active client
                    if active.client == Some(*address) {
                        client_overlay.add_css_class("active");
                    }

                    client_overlay.add_controller(click_client(*address));
                    client_overlay
                };
                trace!(
                    "Creating Client {:?} with ({}x{}) at ({}x{})",
                    address,
                    scale(client.width, global.size_factor),
                    scale(client.height, global.size_factor),
                    scale(client.x - workspace.x as i16, global.size_factor) as f64,
                    scale(client.y - workspace.y as i16, global.size_factor) as f64
                );
                workspace_fixed.put(
                    &client_overlay,
                    scale(client.x - workspace.x as i16, global.size_factor) as f64,
                    scale(client.y - workspace.y as i16, global.size_factor) as f64,
                );
                monitor_data.client_refs.insert(*address, client_overlay);
            }
        }

        trace!("Showing window {:?}", window.id());
        window.set_visible(true);
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
        send_to_socket(&TransferType::Return(ReturnConfig {
            r#override: Some(Override::ClientId(client_id)),
        }))
        .warn("unable send return to socket");
    });
    gesture
}

fn click_workspace(workspace_id: WorkspaceId) -> GestureClick {
    let gesture = GestureClick::new();
    gesture.connect_pressed(move |gesture, _, _, _| {
        gesture.set_state(EventSequenceState::Claimed);
        debug!("Exiting on click of workspace box");
        send_to_socket(&TransferType::Return(ReturnConfig {
            r#override: Some(Override::WorkspaceID(workspace_id)),
        }))
        .warn("unable send return to socket");
    });
    gesture
}
