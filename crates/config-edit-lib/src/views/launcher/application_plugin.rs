use crate::structs::GTKApplications;
use relm4::adw::gdk::Cursor;
use relm4::adw::gtk::{Adjustment, Align, Label, Orientation, SpinButton, Switch};
use relm4::adw::prelude::*;
use relm4::adw::{ExpanderRow, gtk};

pub fn create_plugins_applications_view(row: &gtk::Box) -> GTKApplications {
    let hide_row = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .halign(Align::Start)
        .spacing(10)
        .build();
    hide_row.append(&Label::new(Some("Applications")));
    let info_icon = gtk::Image::from_icon_name("dialog-information-symbolic");
    info_icon.set_tooltip_text(Some("Show installed applications in the launcher, filed by the input, sorted by how often they are used"));
    info_icon.set_cursor(Cursor::from_name("help", None).as_ref());
    hide_row.append(&info_icon);

    let erow = ExpanderRow::builder()
        .title_selectable(false)
        .show_enable_switch(true)
        .hexpand(true)
        .css_classes(["enable-frame"])
        .build();
    erow.add_prefix(&hide_row);
    row.append(&erow);

    let row_1 = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .css_classes(["frame-row"])
        .spacing(30)
        .build();
    erow.add_row(&row_1);

    let cache_weeks = run_cache_weeks(&row_1);
    let show_exec = show_execs(&row_1);
    let submenu = application_submenu(&row_1);

    GTKApplications {
        row: erow,
        cache_weeks,
        submenu,
        show_exec,
    }
}

fn show_execs(windows_box: &gtk::Box) -> Switch {
    let hide_row = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .build();
    hide_row.append(&Label::new(Some("Show execs")));
    let info_icon = gtk::Image::from_icon_name("dialog-information-symbolic");
    info_icon.set_tooltip_text(Some("Show the exec line from the Desktop file. In the case of Flatpaks and PWAs these get shortened to the name of the app"));
    info_icon.set_cursor(Cursor::from_name("help", None).as_ref());
    hide_row.append(&info_icon);
    let switch_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Start)
        .valign(Align::Center)
        .build();
    let hide_switch = Switch::builder().build();
    switch_box.append(&hide_switch);
    hide_row.append(&switch_box);
    windows_box.append(&hide_row);
    hide_switch
}

fn application_submenu(windows_box: &gtk::Box) -> Switch {
    let hide_row = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .build();
    hide_row.append(&Label::new(Some("Show Actions")));
    let info_icon = gtk::Image::from_icon_name("dialog-information-symbolic");
    info_icon.set_tooltip_text(Some("Show a dropdown menu with all the desktop actions specified in the `.desktop` files of the applications, like `new private window`, etc."));
    info_icon.set_cursor(Cursor::from_name("help", None).as_ref());
    hide_row.append(&info_icon);
    let switch_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Start)
        .valign(Align::Center)
        .build();
    let hide_switch = Switch::builder().build();
    switch_box.append(&hide_switch);
    hide_row.append(&switch_box);
    windows_box.append(&hide_row);
    hide_switch
}

fn run_cache_weeks(windows_box: &gtk::Box) -> SpinButton {
    let gbox = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .build();
    gbox.append(&Label::new(Some("Run Cache period (weeks)")));
    let info_icon = gtk::Image::from_icon_name("dialog-information-symbolic");
    info_icon.set_tooltip_text(Some(
        "Number of weeks to retain run history; used to rank applications by usage",
    ));
    info_icon.set_cursor(Cursor::from_name("help", None).as_ref());
    gbox.append(&info_icon);
    let spin = SpinButton::builder()
        .adjustment(&Adjustment::new(0.0, 0.0, 52.0, 1.0, 8.0, 0.0))
        .hexpand(true)
        .digits(0)
        .build();
    gbox.append(&spin);
    windows_box.append(&gbox);
    spin
}
