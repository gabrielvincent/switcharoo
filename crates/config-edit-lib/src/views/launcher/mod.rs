mod application_plugin;
#[allow(clippy::module_inception)]
pub mod launcher;
mod plugins;
mod web_search;

pub use web_search::{generate_row, open_edit_dialog};
