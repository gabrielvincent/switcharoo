use crate::global::WindowsSwitchData;
use crate::icon::set_icon;
use anyhow::Context;
use core_lib::{Active, HyprlandData};
use exec_lib::collect::get_current_monitor;
use relm4::adw::prelude::FixedExt;
use relm4::gtk::gdk::Cursor;
use relm4::gtk::prelude::{BoxExt, WidgetExt};
use relm4::gtk::{Box, Button, Fixed, Frame, Image, Label, Orientation, Overflow, Overlay, pango};
use std::borrow::Cow;
use tracing::debug_span;

fn scale<T: Into<f64>>(value: T, scale: f64) -> i32 {
    (value.into() / (15f64 - scale)) as i32
}

#[allow(clippy::too_many_lines)]
pub fn render_switch(
    data: &mut WindowsSwitchData,
    clients_data: HyprlandData,
    active: Active,
    remove_html: &regex::Regex,
) -> anyhow::Result<()> {
    let _span = debug_span!("render_switch").entered();

    // Clear existing widgets
    while let Some(child) = data.main_flow.first_child() {
        data.main_flow.remove(&child);
    }
    data.workspaces.clear();
    data.clients.clear();

    if data.config.switch_workspaces {
        for (wid, workspace) in &clients_data.workspaces {
            let clients: Vec<_> = {
                let mut clients = clients_data
                    .clients
                    .iter()
                    .filter(|(_, client)| client.workspace == *wid && client.enabled)
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
            if clients.is_empty() {
                continue;
            }
            let workspace_fixed = Fixed::builder()
                .width_request(scale(workspace.width, data.config.scale))
                .height_request(scale(workspace.height, data.config.scale))
                .build();
            let id_string = wid.to_string();
            let title = if workspace.name.trim().is_empty() {
                Cow::from(&id_string)
            } else {
                remove_html.replace_all(&workspace.name, "")
            };
            let workspace_frame = {
                let mut builder = Frame::builder()
                    .label_xalign(0.5)
                    .child(&workspace_fixed);
                if data.config.show_workspace_number {
                    builder = builder.label(title);
                }
                builder.build()
            };

            let workspace_button = {
                let workspace_overlay = Overlay::builder().child(&workspace_frame).build();
                let button = Button::builder()
                    .child(&workspace_overlay)
                    .css_classes(["workspace", "no-hover"])
                    .build();
                button.set_cursor(Cursor::from_name("pointer", None).as_ref());
                if active.workspace == *wid {
                    button.add_css_class("active");
                }
                button
            };
            if *wid < 0 {
                workspace_button.add_css_class("special");
            }
            data.main_flow.insert(&workspace_button, -1);
            data.workspaces.insert(*wid, workspace_button);

            for (address, client) in clients {
                if !client.enabled {
                    continue;
                }

                let client_button = {
                    let title_str = if client.title.trim().is_empty() {
                        &client.class
                    } else {
                        &client.title
                    };
                    let client_label = Label::builder()
                        .label(title_str)
                        .overflow(Overflow::Hidden)
                        .css_classes(["client-label"])
                        .ellipsize(pango::EllipsizeMode::End)
                        .max_width_chars(15)
                        .build();

                    // hide picture if client so small
                    let client_h_w = scale(client.height, data.config.scale)
                        .min(scale(client.width, data.config.scale));
                    
                    let image = Image::builder()
                        .css_classes(["client-image"])
                        .pixel_size((f64::from(client_h_w.clamp(50, 600)) / 1.8) as i32 - 20)
                        .build();
                    if !client.enabled {
                        image.add_css_class("monochrome");
                    }
                    set_icon(&client.class, client.pid, &image);

                    let client_box = Box::builder()
                        .orientation(Orientation::Vertical)
                        .valign(relm4::gtk::Align::Center)
                        .halign(relm4::gtk::Align::Center)
                        .spacing(4)
                        .build();
                    client_box.append(&image);
                    client_box.append(&client_label);

                    let button = Button::builder()
                        .child(&client_box)
                        .css_classes(["client", "no-hover"])
                        .width_request(scale(client.width, data.config.scale))
                        .height_request(scale(client.height, data.config.scale))
                        .build();
                    button.set_cursor(Cursor::from_name("pointer", None).as_ref());

                    // add initial border around initial active client
                    if active.client == Some(*address) {
                        button.add_css_class("active");
                    }
                    button
                };
                workspace_fixed.put(
                    &client_button,
                    f64::from(scale(client.x, data.config.scale)),
                    f64::from(scale(client.y, data.config.scale)),
                );
                data.clients.insert(*address, client_button);
            }
        }
    } else {
        let current_monitor =
            get_current_monitor().context("current_monitor required for non-workspace mode")?;
        for (address, client) in &clients_data.clients {
            if !client.enabled {
                continue;
            }
            let client_button = {
                let title_str = if client.title.trim().is_empty() {
                    &client.class
                } else {
                    &client.title
                };
                let client_label = Label::builder()
                    .label(title_str)
                    .overflow(Overflow::Hidden)
                    .css_classes(["client-label"])
                    .ellipsize(pango::EllipsizeMode::End)
                    .max_width_chars(25)
                    .build();

                // hide picture if client so small
                let client_h_w = scale(i16::try_from(current_monitor.height)?, data.config.scale)
                    .min(scale(
                        (f32::from(current_monitor.height) / current_monitor.scale) as i16,
                        data.config.scale,
                    ));
                
                let image = Image::builder()
                    .css_classes(["client-image"])
                    .pixel_size((f64::from(client_h_w.clamp(50, 600)) / 1.8) as i32 - 20)
                    .build();
                if !client.enabled {
                    image.add_css_class("monochrome");
                }
                set_icon(&client.class, client.pid, &image);

                let client_box = Box::builder()
                    .orientation(Orientation::Vertical)
                    .valign(relm4::gtk::Align::Center)
                    .halign(relm4::gtk::Align::Center)
                    .spacing(8)
                    .build();
                client_box.append(&image);
                client_box.append(&client_label);

                let button = Button::builder()
                    .child(&client_box)
                    .css_classes(["client", "no-hover"])
                    .width_request(scale(
                        (f32::from(current_monitor.width) / current_monitor.scale) as i16,
                        data.config.scale,
                    ))
                    .height_request(scale(
                        (f32::from(current_monitor.height) / current_monitor.scale) as i16,
                        data.config.scale,
                    ))
                    .build();
                button.set_cursor(Cursor::from_name("pointer", None).as_ref());

                // add border around initial active client
                if active.client == Some(*address) {
                    button.add_css_class("active");
                }
                button
            };
            data.main_flow.insert(&client_button, -1);
            data.clients.insert(*address, client_button);
        }
    }

    data.active = active;
    data.hypr_data = clients_data;

    // Force GTK to update the layout
    data.main_flow.queue_resize();
    data.main_flow.queue_draw();

    Ok(())
}
