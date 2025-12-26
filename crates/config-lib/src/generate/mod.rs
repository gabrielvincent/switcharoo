mod autocomplete;
mod config;
mod css;
mod tui;

pub use config::check_file_exist;
pub use config::generate_config;
pub use config::get_overrides;
pub use css::write_css;
pub use tui::prompt_config;
