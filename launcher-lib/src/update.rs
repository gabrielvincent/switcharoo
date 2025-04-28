use crate::plugins::{get_sortable_launch_options, get_static_launch_options, StaticLaunchOption};
use crate::LauncherGlobal;
use core_lib::theme_icon_cache::theme_has_icon_name;
use core_lib::transfer::{CloseConfig, TransferType};
use core_lib::{send_to_socket, Warn};
use gtk::pango::EllipsizeMode;
use gtk::prelude::{BoxExt, GestureExt, WidgetExt};
use gtk::{
    glib, Align, EventSequenceState, GestureClick, IconSize, Image, Label, ListBoxRow, Orientation,
};
use std::path::Path;
use tracing::{debug, span, trace, warn, Level};

pub fn update_launcher(global: &LauncherGlobal, text: String) {
    let _span = span!(Level::TRACE, "update_launcher").entered();

    if let Some(data) = &global.data {
        let data1 = data.borrow();
        while let Some(child) = data1.results.first_child() {
            data1.results.remove(&child);
        }
        if text.is_empty() {
            return;
        }

        let sortable_launch_options =
            get_sortable_launch_options(&global.plugins, &text, &global.data_dir);
        let mut items = global.max_items;
        for (index, opt) in sortable_launch_options.into_iter().enumerate() {
            if items <= 0 {
                break;
            }
            items -= 1;
            let row = create_entry(
                match index {
                    0 => "Return".to_string(),
                    i if i <= 9 => format!("Ctrl+{}", i),
                    _ => "".to_string(),
                },
                opt.icon,
                &opt.name,
                opt.details,
                opt.details_long,
            );
            row.add_controller(click_entry(
                char::from_u32(index as u32).expect("Failed to convert u32 to char"),
            ));
            data1.results.append(&row);
        }

        let (static_row, static_box) = create_static_plugin_row();
        data1.results.append(&static_row);

        let static_launch_options = get_static_launch_options(&global.plugins);
        for opt in static_launch_options.into_iter() {
            let r#box = create_static_plugin_box(opt);
            static_box.append(&r#box);
        }
    }
}

fn create_static_plugin_box(opt: StaticLaunchOption) -> gtk::Box {
    let hbox = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    hbox
}

fn create_static_plugin_row() -> (ListBoxRow, gtk::Box) {
    let hbox = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .hexpand(true)
        .vexpand(true)
        .build();

    // TODO
    let row = ListBoxRow::builder()
        .css_classes(vec!["launcher-item"])
        .height_request(45)
        .hexpand(true)
        .vexpand(true)
        .child(&hbox)
        .build();
    (row, hbox)
}

fn create_entry(
    key: impl Into<glib::GString>,
    icon_path: Option<Box<Path>>,
    name: &str,
    details: Box<str>,
    details_long: Option<Box<str>>,
) -> ListBoxRow {
    let hbox = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .hexpand(true)
        .vexpand(true)
        .build();

    let icon = Image::builder()
        .css_classes(vec!["launcher-icon"])
        .icon_size(IconSize::Large)
        .build();
    if let Some(icon_path) = icon_path {
        if icon_path.is_absolute() {
            if let Some(icon_name) = icon_path.file_stem() {
                if !theme_has_icon_name(&icon_name.to_string_lossy()) {
                    icon.set_from_file(Some(Path::new(&*icon_path)));
                } else {
                    icon.set_icon_name(Some(&icon_name.to_string_lossy()));
                }
            } else {
                warn!("invalid icon name: {icon_path:?}");
            }
        } else {
            // use filename as some files are named org.gnome.file
            trace!(
                "using name: {:?}",
                icon_path.file_name().and_then(|name| name.to_str())
            );
            icon.set_icon_name(icon_path.file_name().and_then(|name| name.to_str()));
        }
    }
    hbox.append(&icon);

    let title = Label::builder()
        .halign(Align::Start)
        .valign(Align::Center)
        .label(name)
        .build();
    hbox.append(&title);

    // if let Some(exec) = exec {
    let exec = Label::builder()
        .halign(Align::Start)
        .valign(Align::Center)
        .hexpand(true)
        .css_classes(vec!["launcher-exec"])
        .ellipsize(EllipsizeMode::End)
        .label(details)
        .build();
    if let Some(details_long) = details_long {
        exec.set_tooltip_text(Some(&details_long));
        exec.add_css_class("underline")
    }
    hbox.append(&exec);

    let index_label = Label::builder()
        .halign(Align::End)
        .valign(Align::Center)
        .label(key)
        .build();
    hbox.append(&index_label);

    let row = ListBoxRow::builder()
        .css_classes(vec!["launcher-item"])
        .height_request(45)
        .hexpand(true)
        .vexpand(true)
        .child(&hbox)
        .build();
    row
}

fn click_entry(char: char) -> GestureClick {
    let gesture = GestureClick::new();
    gesture.connect_pressed(move |gesture, _, _, _| {
        gesture.set_state(EventSequenceState::Claimed);
        debug!("Exiting on click of launcher entry");
        send_to_socket(&TransferType::Close(CloseConfig::Launcher(char)))
            .warn("unable send return to socket");
    });
    gesture
}
