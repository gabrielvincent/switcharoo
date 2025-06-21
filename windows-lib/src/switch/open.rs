use crate::data::{SortConfig, collect_data};
use crate::global::WindowsSwitchData;
use crate::icon::set_icon;
use crate::next::find_next;
use anyhow::Context;
use async_channel::Sender;
use core_lib::transfer::{
    CloseOverviewConfig, Direction, OpenSwitch, TransferType, WindowsOverride,
};
use core_lib::{ClientData, ClientId, WarnWithDetails};
use exec_lib::{get_current_monitor, set_remain_focused};
use gtk::gdk::Cursor;
use gtk::prelude::*;
use gtk::{Button, Frame, Image, Label, Overflow, Overlay, pango};
use tracing::{Level, debug, span, trace};

fn scale(value: i16, scale: f64) -> i32 {
    (value as f64 / (15f64 - scale)) as i32
}

pub fn open_switch(
    data: &mut WindowsSwitchData,
    config: OpenSwitch,
    event_sender: Sender<TransferType>,
) -> anyhow::Result<()> {
    let _span = span!(Level::TRACE, "open_switch").entered();
    // check if already open
    if data.window.get_visible() {
        return Ok(());
    }

    set_remain_focused().warn("Failed to set no follow mouse");

    let (clients_data, active_prev) = collect_data(&SortConfig {
        filter_current_monitor: config.filter_current_monitor,
        filter_current_workspace: config.filter_current_workspace,
        filter_same_class: config.filter_same_class,
        sort_recent: true,
    })
    .context("Failed to collect data")?;
    let active = find_next(
        &if config.reverse {
            Direction::Left
        } else {
            Direction::Right
        },
        false,
        &clients_data,
        active_prev,
        config.items_per_row as usize,
    );

    trace!("Showing window {:?}", data.window.id());
    data.window.set_visible(true);

    let current_monitor = get_current_monitor().context("Failed to get current monitor")?;

    let clients: &Vec<(ClientId, ClientData)> = &clients_data.clients;
    for (address, client) in clients {
        if !client.enabled {
            continue;
        }
        let client_button = {
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
            let client_h_w = scale(current_monitor.height as i16, config.scale)
                .min(scale(current_monitor.width as i16, config.scale));
            if client_h_w > 70 {
                let image = Image::builder()
                    .css_classes(["client-image"])
                    .pixel_size((client_h_w.clamp(50, 600) as f64 / 1.5) as i32 - 20)
                    .build();
                if !client.enabled {
                    image.add_css_class("monochrome");
                }
                set_icon(&client.class, client.pid, &image);
                client_frame.set_child(Some(&image));
            }

            let client_overlay = Overlay::builder()
                .overflow(Overflow::Hidden)
                .child(&client_frame)
                .build();
            let button = Button::builder()
                .child(&client_overlay)
                .css_classes(["client"])
                .width_request(scale(current_monitor.width as i16, config.scale))
                .height_request(scale(current_monitor.height as i16, config.scale))
                .build();
            button.set_cursor(Cursor::from_name("pointer", None).as_ref());

            // add border around initial active client
            if active.client == Some(*address) {
                button.add_css_class("active");
            }

            click_client(&button, *address, event_sender.clone());
            button
        };
        data.clients_flow.insert(&client_button, -1);
        data.clients.insert(*address, client_button);
    }

    data.active = active;
    data.hypr_data = clients_data;
    Ok(())
}

fn click_client(button: &Button, client_id: ClientId, event_sender: Sender<TransferType>) {
    button.connect_clicked(move |_| {
        debug!("Exiting on click of launcher details entry");
        event_sender
            .send_blocking(TransferType::CloseOverview(CloseOverviewConfig::Windows(
                WindowsOverride::ClientId(client_id),
            )))
            .warn("unable to send");
    });
}
