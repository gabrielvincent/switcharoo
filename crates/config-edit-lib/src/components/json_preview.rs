use relm4::adw::gtk;
use relm4::adw::prelude::*;
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

#[derive(Debug)]
pub struct JSONPreview {}

#[derive(Debug)]
pub enum JSONPreviewInput {}

#[derive(Debug)]
pub struct JSONPreviewInit {}

#[derive(Debug)]
pub enum JSONPreviewOutput {}

#[relm4::component(pub)]
impl SimpleComponent for JSONPreview {
    type Init = JSONPreviewInit;
    type Input = JSONPreviewInput;
    type Output = JSONPreviewOutput;

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
        let model = JSONPreview {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, _message: Self::Input, _sender: ComponentSender<Self>) {
        todo!()
    }
}
