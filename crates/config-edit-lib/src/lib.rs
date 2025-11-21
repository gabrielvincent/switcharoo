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
