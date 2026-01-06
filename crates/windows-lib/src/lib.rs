mod css;
mod data;
mod desktop_map;
mod global;
mod icon;
mod keybinds;
mod next;
pub mod overview;
mod sort;
pub mod switch;

pub use css::get_css;
pub use desktop_map::{get_icon_name_by_name_from_desktop_files, reload_class_to_icon_map};
pub use global::{WindowsOverviewData, WindowsSwitchData};
pub use keybinds::generate_open_keybinds;
