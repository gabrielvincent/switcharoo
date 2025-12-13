// mod bind;
// mod footer;
// mod start;
mod components;
mod footer;
mod startv;
mod structs;
// mod update;
// mod update_changes_view;
// mod views;

// pub use start::start;

pub const APPLICATION_EDIT_ID: &str = "com.github.h3rmt.hyprshell-edit";
pub use startv::start;

pub(crate) use structs::*;

pub trait SetTextIfDifferent {
    fn set_text_if_different(&self, text: &str);
}

impl SetTextIfDifferent for relm4::gtk::Entry {
    fn set_text_if_different(&self, text: &str) {
        use relm4::adw::prelude::EditableExt;
        if self.text() != text {
            self.set_text(text);
        }
    }
}
