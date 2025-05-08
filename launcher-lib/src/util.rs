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
