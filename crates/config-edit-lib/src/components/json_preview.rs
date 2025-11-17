use adw::gtk;
use adw::prelude::*;
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

#[derive(Debug)]
pub struct JsonPreview {}

#[derive(Debug)]
pub enum JsonPreviewMsg {}

#[derive(Debug)]
pub enum FooterOutput {}

#[relm4::component(pub)]
impl SimpleComponent for JsonPreview {
    type Init = ();
    type Input = ();
    type Output = FooterOutput;

    view! {
        gtk::Box {
           set_orientation: gtk::Orientation::Vertical,
            set_margin_all: 10,
            gtk::Label {
                set_label:  "Json preview for nix users (TODO)"
            }
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = JsonPreview {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
