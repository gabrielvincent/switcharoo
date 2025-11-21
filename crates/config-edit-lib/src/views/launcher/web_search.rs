use crate::structs::GTKWebsearches;
use config_lib::{Modifier, SearchEngine};
use relm4::adw::gdk::Cursor;
use relm4::adw::gtk::{Align, Button, Label, ListBox, Orientation, SelectionMode};
use relm4::adw::prelude::{
    ActionRowExt, AdwDialogExt, BoxExt, ButtonExt, EditableExt, ExpanderRowExt, WidgetExt,
};
use relm4::adw::{ActionRow, Dialog, EntryRow, ExpanderRow, gtk};
use std::collections::BTreeMap;

pub fn create_plugins_websearch_view(row: &gtk::Box) -> GTKWebsearches {
    let hide_row = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .halign(Align::Start)
        .spacing(10)
        .build();
    hide_row.append(&Label::new(Some("Web search")));
    let info_icon = gtk::Image::from_icon_name("dialog-information-symbolic");
    info_icon.set_tooltip_text(Some(
        "Allows searching for the typed query in a web browser.",
    ));
    info_icon.set_cursor(Cursor::from_name("help", None).as_ref());
    hide_row.append(&info_icon);

    let button = Button::builder()
        .hexpand(true)
        .valign(Align::Center)
        .tooltip_text("Add")
        .build();
    button.set_cursor(Cursor::from_name("pointer", None).as_ref());
    let plus_icon = gtk::Image::from_icon_name("list-add-symbolic");
    plus_icon.set_pixel_size(20);
    button.set_child(Some(&plus_icon));
    hide_row.append(&button);

    let erow = ExpanderRow::builder()
        .title_selectable(false)
        .show_enable_switch(true)
        .hexpand(true)
        .css_classes(["enable-frame"])
        .build();
    erow.add_prefix(&hide_row);
    row.append(&erow);

    let list = ListBox::builder()
        .css_classes(["items-list", "boxed-list-separate"])
        .selection_mode(SelectionMode::None)
        .show_separators(false)
        .halign(Align::Fill)
        .valign(Align::Start)
        .hexpand(true)
        .vexpand(true)
        .build();
    erow.add_row(&list);

    GTKWebsearches {
        row: erow,
        list,
        create: button,
        items: BTreeMap::new(),
    }
}

pub fn generate_row(
    search_engine: &SearchEngine,
    modifier: Modifier,
) -> (ActionRow, Button, Button) {
    let row1 = ActionRow::builder()
        .title(format!(
            "{} ({} + {})",
            search_engine.name, modifier, search_engine.key
        ))
        .subtitle(search_engine.url.clone())
        .build();
    let delete_icon = gtk::Image::from_icon_name("delete");
    delete_icon.set_pixel_size(28);
    let delete = Button::builder()
        .hexpand(false)
        .valign(Align::Center)
        .tooltip_text("Delete")
        .child(&delete_icon)
        .build();
    delete.set_cursor(Cursor::from_name("pointer", None).as_ref());
    row1.add_suffix(&delete);
    let edit_icon = gtk::Image::from_icon_name("edit");
    edit_icon.set_pixel_size(28);
    let edit = Button::builder()
        .hexpand(false)
        .valign(Align::Center)
        .tooltip_text("Edit")
        .child(&edit_icon)
        .build();
    edit.set_cursor(Cursor::from_name("pointer", None).as_ref());
    row1.add_suffix(&edit);

    (row1, delete, edit)
}

pub fn open_edit_dialog(search_engine: &SearchEngine) -> (Dialog, EntryRow, EntryRow, EntryRow) {
    let row_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_bottom(10)
        .margin_end(10)
        .margin_start(10)
        .margin_top(10)
        .build();
    let dialog = Dialog::builder().child(&row_box).build();

    let list = ListBox::builder()
        .css_classes(["items-list", "boxed-list-separate"])
        .selection_mode(SelectionMode::None)
        .show_separators(false)
        .halign(Align::Center)
        .valign(Align::Start)
        .hexpand(true)
        .build();
    row_box.append(&list);

    let name = EntryRow::builder()
        .title("name")
        .text(search_engine.name.clone())
        .build();
    list.append(&name);
    let dialog_clone = dialog.clone();
    name.connect_text_notify(move |entry| {
        dialog_clone.set_can_close(entry.text().len() > 0);
    });

    let key = EntryRow::builder()
        .title("key")
        .text(search_engine.key.to_string())
        .build();
    list.append(&key);
    let dialog_clone = dialog.clone();
    name.connect_text_notify(move |entry| {
        dialog_clone.set_can_close(entry.text().len() == 1);
    });

    let url = EntryRow::builder()
        .title("url (must contain {})")
        .text(search_engine.url.clone())
        .build();
    list.append(&url);
    let dialog_clone = dialog.clone();
    name.connect_text_notify(move |entry| {
        dialog_clone.set_can_close(entry.text().len() > 0);
    });

    (dialog, name, key, url)
}
