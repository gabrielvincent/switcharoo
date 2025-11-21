use relm4::adw::gtk;
use relm4::adw::prelude::*;
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

#[derive(Debug)]
pub struct JsonPreview {}

#[derive(Debug)]
pub enum JsonPreviewMsg {}

#[derive(Debug)]
pub enum JsonPreviewOutput {}

#[relm4::component(pub)]
impl SimpleComponent for JsonPreview {
    type Init = ();
    type Input = ();
    type Output = JsonPreviewOutput;

    view! {
        #[root]
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
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = JsonPreview {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
