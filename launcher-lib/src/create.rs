use crate::global::LauncherGlobalData;
use crate::LauncherGlobal;
use core_lib::theme_icon_cache::theme_has_icon_name;
use core_lib::transfer::{CloseConfig, TransferType};
use core_lib::{send_to_socket, Warn, LAUNCHER_NAMESPACE};
use gtk::gdk::Key;
use gtk::glib::{clone, Propagation};
use gtk::pango::EllipsizeMode;
use gtk::prelude::*;
use gtk::{glib, EventSequenceState, GestureClick, IconSize, Image, ListBoxRow};
use gtk::{
    Align, Application, ApplicationWindow, Entry, EventControllerKey, Label, ListBox, Orientation,
    SelectionMode,
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use tracing::{debug, span, trace, warn, Level};

pub fn create_launcher_window(
    app: &Application,
    global: &mut LauncherGlobal,
    data_dir: &Path,
) -> anyhow::Result<()> {
    let _span = span!(Level::TRACE, "create_launcher_window").entered();

    let main_vbox = ListBox::builder()
        .css_classes(vec!["launcher"])
        .width_request(global.width as i32)
        .selection_mode(SelectionMode::None)
        .build();

    let results = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(vec!["launcher-results"])
        .build();

    let entry = Entry::builder().css_classes(vec!["launcher-input"]).build();
    entry.connect_changed(clone!(
        #[weak]
        results,
        move |e| {
            trace!("Entry changed: {}", e.text());
            send_to_socket(&TransferType::Type(e.text().to_string()))
                .warn("unable send return to socket");
        }
    ));
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(move |_, k, _, m| match (k, m) {
        (Key::Tab, _) => Propagation::Stop,
        (Key::ISO_Left_Tab, _) => Propagation::Stop,
        _ => Propagation::Proceed,
    });
    entry.add_controller(controller);
    main_vbox.append(&entry);
    main_vbox.append(&results);

    let window = ApplicationWindow::builder()
        .css_classes(vec!["window"])
        .application(app)
        .child(&main_vbox)
        .default_height(10)
        .default_width(10)
        .build();
    window.init_layer_shell();
    window.set_namespace(Some(LAUNCHER_NAMESPACE));
    window.set_layer(Layer::Overlay);
    window.set_anchor(Edge::Top, true);
    window.set_margin(Edge::Top, 20);
    window.present();
    window.set_visible(false);

    debug!("Created launcher window ({})", window.id());
    global.data = Some(RefCell::new(LauncherGlobalData {
        window,
        entry,
        results,
        matches: vec![],
    }));

    Ok(())
}
