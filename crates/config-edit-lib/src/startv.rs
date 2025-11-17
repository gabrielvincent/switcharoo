use crate::APPLICATION_EDIT_ID;
use crate::root::InitRoot;
use adw::gdk::Display;
use adw::gtk::{
    CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, style_context_add_provider_for_display,
};
use config_lib::Config;
use relm4::RelmApp;
use std::path::PathBuf;

pub fn start(config_path: PathBuf, _css_path: PathBuf) {
    let relm = RelmApp::new(&format!(
        "{}{}",
        APPLICATION_EDIT_ID,
        if cfg!(debug_assertions) { "-test" } else { "" }
    ))
    .with_args(vec![]);

    let provider_app = CssProvider::new();
    provider_app.load_from_data(include_str!("styles.css"));
    style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider_app,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    relm.run::<crate::root::Root>(InitRoot {
        config: Config::default(),
        config_path: config_path.into_boxed_path(),
    });
}
