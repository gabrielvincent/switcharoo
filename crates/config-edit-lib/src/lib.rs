// mod bind;
// mod footer;
// mod start;
mod components;
mod footer;
mod shortcut_dialog;
mod start;
mod structs;
mod util;
// mod update;
// mod update_changes_view;
// mod views;

// pub use start::start;

pub const APPLICATION_EDIT_ID: &str = "com.github.h3rmt.hyprshell-edit";
pub use start::start;

pub(crate) use structs::*;
