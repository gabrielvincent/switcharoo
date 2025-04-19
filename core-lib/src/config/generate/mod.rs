mod autocomplete;
mod write;
mod tui;
mod css;

pub use write::{generate_config, write_config};
pub use tui::prompt_config;
pub use css::write_css;
