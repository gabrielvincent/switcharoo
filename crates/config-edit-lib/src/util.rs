use crate::structs::ConfigModifier;
use relm4::gtk;
use relm4::gtk::gdk::{Cursor, Display, Key, ModifierType};
use relm4::gtk::prelude::{Cast, DisplayExtManual, WidgetExt};
// use relm4::tokio::time::sleep;
use tracing::instrument;

pub trait SetTextIfDifferent {
    fn set_text_if_different(&self, text: &str);
}

impl SetTextIfDifferent for gtk::Entry {
    fn set_text_if_different(&self, text: &str) {
        use relm4::adw::prelude::EditableExt;
        if self.text() != text {
            self.set_text(text);
        }
    }
}

pub trait SetCursor {
    fn set_cursor_by_name(&self, name: &str);
}

impl SetCursor for gtk::Image {
    fn set_cursor_by_name(&self, name: &str) {
        use relm4::adw::prelude::WidgetExt;
        self.set_cursor(Cursor::from_name(name, None).as_ref());
    }
}

pub trait ScrollToPosition {
    fn scroll_to_pos(&self, pos: usize, animate: bool);
}

impl ScrollToPosition for adw::Carousel {
    fn scroll_to_pos(&self, pos: usize, animate: bool) {
        if let Some(wdg) = self.observe_children().into_iter().flatten().nth(pos) {
            if let Ok(widget) = wdg.downcast::<gtk::Widget>() {
                let s2 = self.clone();
                // scuffed method to select new widget (else it doesnt work on first render)
                gtk::glib::idle_add_local(move || {
                    s2.scroll_to(&widget, animate);
                    if s2.position() as usize == pos {
                        gtk::glib::ControlFlow::Break
                    } else {
                        gtk::glib::ControlFlow::Continue
                    }
                });
            }
        }
    }
}

pub fn handle_key(
    val: Key,
    state: ModifierType,
    id: u32,
) -> Option<(String, ConfigModifier, String)> {
    if let Some(key_name) = val.name() {
        if let Some(modifier) = match val {
            Key::Alt_L | Key::Alt_R => Some(ConfigModifier::Alt),
            Key::Control_L | Key::Control_R => Some(ConfigModifier::Ctrl),
            Key::Super_L | Key::Super_R => Some(ConfigModifier::Super),
            _ => match state {
                ModifierType::NO_MODIFIER_MASK => Some(ConfigModifier::None),
                ModifierType::ALT_MASK => Some(ConfigModifier::Alt),
                ModifierType::CONTROL_MASK => Some(ConfigModifier::Ctrl),
                ModifierType::SUPER_MASK => Some(ConfigModifier::Super),
                _ => None,
            },
        } {
            let label = if modifier == ConfigModifier::None {
                key_name.to_string()
            } else {
                format!("{modifier} + {key_name}")
            };
            Some((format!("code:{id}"), modifier, label))
        } else {
            None
        }
    } else {
        None
    }
}

pub fn default_config() -> config_lib::Config {
    let mut conf = config_lib::Config::default();
    conf.windows = Some(config_lib::Windows::default());
    // conf.windows.as_mut().unwrap().overview = Some(config_lib::Overview::default());
    conf
}

#[instrument(level = "trace", ret(level = "trace"))]
pub fn to_accelerator(modifier: ConfigModifier, key: &str) -> Option<String> {
    let key = key_to_name(key)?;
    if modifier == ConfigModifier::None {
        Some(key)
    } else {
        Some(format!("<{modifier}>{key}"))
    }
}

pub fn key_to_name(key: &str) -> Option<String> {
    // key is keycode
    if key.starts_with("code:") {
        let key_id = key.split(':').nth(1)?;
        let code = key_id.parse::<u32>().ok()?;
        let display = &Display::default()?;
        let data = display.map_keycode(code)?;
        let (_, key) = data.iter().find(|(m, _k)| m.level() == 0)?;
        Some(key.name()?.to_string())
    } else {
        Some(key.to_string())
    }
}

#[macro_export]
macro_rules! flags_csv {
    ($s:expr, $($field:ident),+ $(,)?) => {{
        [$( (stringify!($field), $s.$field) ),+]
            .into_iter()
            .filter(|(_, v)| *v)
            .map(|(k, _)| k)
            .collect::<Vec<&str>>()
            .join(", ")
    }};
}
