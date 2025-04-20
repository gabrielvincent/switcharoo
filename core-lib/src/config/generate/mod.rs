mod autocomplete;
mod config;
mod tui;
mod css;
mod systemd;

pub use config::{generate_config, write_config};
pub use tui::prompt_config;
pub use css::write_css;
pub use systemd::write_systemd_unit;
