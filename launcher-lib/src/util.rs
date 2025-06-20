use gtk::gdk::Key;
use gtk::prelude::ObjectExt;

pub trait DataInWidget {
    fn get_iden_data(&self) -> Option<&str>;
    fn set_iden_data(&self, data: String);
}

impl<T: gtk::prelude::ObjectType> DataInWidget for T {
    fn get_iden_data(&self) -> Option<&str> {
        if let Some(data) = unsafe { self.data::<String>("hyprshell-identifier") } {
            let data = unsafe { data.as_ref() };
            Some(data)
        } else {
            None
        }
    }

    fn set_iden_data(&self, data: String) {
        unsafe { self.set_data("hyprshell-identifier", data) };
    }
}

pub fn convert_to_key(char: char) -> Option<Key> {
    match char {
        'a' => Some(Key::a),
        'b' => Some(Key::b),
        'c' => Some(Key::c),
        'd' => Some(Key::d),
        'e' => Some(Key::e),
        'f' => Some(Key::f),
        'g' => Some(Key::g),
        'h' => Some(Key::h),
        'i' => Some(Key::i),
        'j' => Some(Key::j),
        'k' => Some(Key::k),
        'l' => Some(Key::l),
        'm' => Some(Key::m),
        'n' => Some(Key::n),
        'o' => Some(Key::o),
        'p' => Some(Key::p),
        'q' => Some(Key::q),
        'r' => Some(Key::r),
        's' => Some(Key::s),
        't' => Some(Key::t),
        'u' => Some(Key::u),
        'v' => Some(Key::v),
        'w' => Some(Key::w),
        'x' => Some(Key::x),
        'y' => Some(Key::y),
        'z' => Some(Key::z),
        _ => None,
    }
}
