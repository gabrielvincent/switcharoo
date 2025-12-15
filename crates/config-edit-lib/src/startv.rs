use crate::APPLICATION_EDIT_ID;
use crate::components::root::InitRoot;
use config_lib::{Config, FilterBy, Modifier, Overview, Windows};
use relm4::RelmApp;
use relm4::adw::gdk::Display;
use relm4::adw::gtk::{
    CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, style_context_add_provider_for_display,
};
use std::path::PathBuf;

pub fn start(config_path: PathBuf, _css_path: PathBuf) {
    let relm = RelmApp::new(&format!(
        "{}{}",
        APPLICATION_EDIT_ID,
        if cfg!(debug_assertions) { "-test" } else { "" }
    ))
    .with_args(vec![]);

    let provider_app = CssProvider::new();
    provider_app.load_from_string(include_str!("styles.css"));
    style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider_app,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let mut config = Config::default();
    let mut windows = Windows::default();
    let mut overview = Overview::default();
    overview.filter_by = vec![FilterBy::CurrentMonitor];
    overview.modifier = Modifier::Super;
    overview.key = "Super_L".into();
    windows.overview = Some(overview);
    windows.scale = 5.5;
    config.windows = Some(windows);

    relm.run::<crate::components::root::Root>(InitRoot {
        config: config.into(),
        config_path: config_path.into_boxed_path(),
    });
}
