use crate::global::LauncherData;
use crate::{LauncherData, update_launcher};
use async_channel::Sender;
use core_lib::config::Launcher;
use core_lib::transfer::{CloseConfig, Direction, SwitchConfig, TransferType};
use core_lib::{LAUNCHER_NAMESPACE, Warn};
use gtk::Orientation;
use gtk::gdk::Key;
use gtk::glib::Propagation;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Entry, EventControllerKey, ListBox, SelectionMode};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::cell::RefCell;
use std::collections::HashMap;
use tracing::{Level, debug, span, warn};

pub fn create_windows_overview_launcher_window(
    app: &Application,
    launcher: &Launcher,
    event_sender: Sender<TransferType>,
) -> anyhow::Result<LauncherData> {
    let _span = span!(Level::TRACE, "create_windows_overview_launcher_window").entered();

    let main_vbox = ListBox::builder()
        .css_classes(["launcher"])
        .width_request(launcher.width as i32)
        .selection_mode(SelectionMode::None)
        .build();

    let entry = Entry::builder().css_classes(["launcher-input"]).build();
    entry.connect_changed(|e| {
        // trace!("Launcher entry changed: {}", e.text());
        event_sender
            .send_blocking(TransferType::Type(e.text().to_string()))
            .warn("unable to send");
    });
    let key_controller = EventControllerKey::new();
    key_controller.connect_key_pressed(move |_, k, _, _| match k {
        Key::Tab => Propagation::Stop,
        Key::ISO_Left_Tab => Propagation::Stop,
        _ => Propagation::Proceed,
    });
    entry.add_controller(key_controller);
    main_vbox.append(&entry);

    let results = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .css_classes(["launcher-results"])
        .spacing(3)
        .build();
    main_vbox.append(&results);

    let plugin_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .css_classes(["launcher-plugins"])
        .spacing(5)
        .build();
    main_vbox.append(&plugin_box);

    let event_controller = EventControllerKey::new();
    event_controller.connect_key_pressed(|_, key, _, _| handle_key(key, event_sender));

    let window = ApplicationWindow::builder()
        .css_classes(["window"])
        .startup_id("hyprshell") // TODO
        .application(app)
        .child(&main_vbox)
        .default_height(10)
        .default_width(10)
        .build();
    window.init_layer_shell();
    window.set_namespace(Some(LAUNCHER_NAMESPACE));
    window.set_layer(Layer::Overlay);
    window.set_anchor(Edge::Top, true);
    window.set_margin(Edge::Top, 15);
    window.present();
    window.set_visible(false);
    window.add_controller(event_controller);

    debug!("Created launcher window ({})", window.id());

    // initial update TODO
    // update_launcher(global, "".to_string());

    Ok(LauncherData {
        window,
        entry,
        results,
        plugin_box,
        sorted_matches: vec![],
        static_matches: HashMap::new(),
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
        Key::Tab => {
            event_sender
                .send_blocking(TransferType::Switch(SwitchConfig {
                    workspace: false,
                    direction: Direction::Right,
                }))
                .warn("unable to send");
            Propagation::Stop
        }
        Key::ISO_Left_Tab => {
            event_sender
                .send_blocking(TransferType::Switch(SwitchConfig {
                    workspace: false,
                    direction: Direction::Left,
                }))
                .warn("unable to send");
            Propagation::Stop
        }
        Key::Up => {
            event_sender
                .send_blocking(TransferType::Switch(SwitchConfig {
                    workspace: true,
                    direction: Direction::Up,
                }))
                .warn("unable to send");
            Propagation::Stop
        }
        Key::Down => {
            event_sender
                .send_blocking(TransferType::Switch(SwitchConfig {
                    workspace: true,
                    direction: Direction::Down,
                }))
                .warn("unable to send");
            Propagation::Stop
        }
        Key::Left => {
            event_sender
                .send_blocking(TransferType::Switch(SwitchConfig {
                    workspace: true,
                    direction: Direction::Left,
                }))
                .warn("unable to send");
            Propagation::Stop
        }
        Key::Right => {
            event_sender
                .send_blocking(TransferType::Switch(SwitchConfig {
                    workspace: true,
                    direction: Direction::Right,
                }))
                .warn("unable to send");
            Propagation::Stop
        }
        Key::Return => {
            event_sender
                .send_blocking(TransferType::Close(CloseConfig::None))
                .warn("unable to send");
            Propagation::Stop
        }
        _ => Propagation::Proceed,
    }
}
