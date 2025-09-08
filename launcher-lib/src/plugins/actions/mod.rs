#[allow(clippy::module_inception)]
mod actions;
mod plugin;

pub use plugin::get_actions_options;
pub use plugin::run_action;
