use crate::WindowsGlobal;
use crate::global::{WindowsOverviewMonitorData, WindowsSwitchData};
use anyhow::Context;
use async_channel::Sender;
use core_lib::config::{Switch, Windows};
use core_lib::transfer::TransferType;
use core_lib::{HyprlandData, OVERVIEW_NAMESPACE, Warn};
use exec_lib::{get_initial_active, get_monitors};
use gtk::gdk::{Display, Key, Monitor};
use gtk::glib::Propagation;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, EventControllerKey, FlowBox, Orientation, Overlay,
    SelectionMode, ShortcutController, ShortcutScope,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::RefCell;
use std::collections::HashMap;
use tracing::{Level, debug, span};

pub fn create_windows_switch_window(
    app: &Application,
    _switch: &Switch,
    windows: &Windows,
    event_sender: Sender<TransferType>,
) -> anyhow::Result<WindowsSwitchData> {
    let _span = span!(Level::TRACE, "create_windows_switch_window").entered();

    let clients_flow = FlowBox::builder()
        .selection_mode(SelectionMode::None)
        .orientation(Orientation::Horizontal)
        .max_children_per_line(windows.items_per_row as u32)
        .min_children_per_line(windows.items_per_row as u32)
        .build();

    let workspaces_flow_overlay = Overlay::builder()
        .child(&clients_flow)
        .css_classes(["monitor"])
        .build();

    let window = ApplicationWindow::builder()
        .css_classes(["window"])
        .application(app)
        .child(&workspaces_flow_overlay)
        .default_height(10)
        .default_width(10)
        .build();

    let key_controller = EventControllerKey::new();
    let event_sender_2 = event_sender.clone();
    key_controller.connect_key_pressed(move |_, key, _, _| handle_key(key, event_sender_2.clone()));
    window.add_controller(key_controller);

    window.init_layer_shell();
    window.set_namespace(Some(OVERVIEW_NAMESPACE));
    window.set_layer(Layer::Top);
    window.set_keyboard_mode(KeyboardMode::Exclusive);
    window.present();
    window.set_visible(false);

    debug!("Created switch window ({})", window.id(),);

    Ok(WindowsSwitchData {
        window,
        clients_flow: Default::default(),
        clients: HashMap::default(),
        active: get_initial_active()?,
        hypr_data: HyprlandData::default(),
    })
}

fn handle_key(key: Key, event_sender: Sender<TransferType>) -> Propagation {
    match key {
        Key::Escape => {
            event_sender
                .send_blocking(TransferType::Exit)
                .warn("unable to send");
            Propagation::Stop
        }
        _ => Propagation::Proceed,
    }
}
