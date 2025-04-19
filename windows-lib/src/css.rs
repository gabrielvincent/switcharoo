use gtk::gdk::Display;
use gtk::{
    style_context_add_provider_for_display, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION,
};

pub fn get_css() {
    let provider_app = CssProvider::new();
    provider_app.load_from_data(include_str!("styles.css"));
    style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider_app,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
