use relm4::adw::gtk;
use relm4::adw::gtk::Orientation;
use relm4::adw::prelude::*;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};
use std::path::Path;

pub struct Footer {
    config_path: Box<Path>,
    changes: bool,
}

#[derive(Debug)]
pub enum FooterOutput {
    Close,
    Save,
    Reset,
}

#[derive(Debug)]
pub enum FooterInput {
    ChangesExist(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for Footer {
    type Init = Box<Path>;
    type Input = FooterInput;
    type Output = FooterOutput;

    view! {
        gtk::ActionBar  {
            #[wrap(Some)]
            set_center_widget = &gtk::Box {
                set_spacing: 20,
                set_hexpand: true,
                set_css_classes: &["footer"],
                set_orientation: Orientation::Horizontal,
                gtk::Label {
                    set_label: &format!("Hyprshell v{}", env!("CARGO_PKG_VERSION")),
                },
                gtk::Box {
                    set_spacing: 10,
                    set_hexpand: true,
                    set_halign: gtk::Align::End,
                    set_orientation: Orientation::Horizontal,
                    gtk::Button {
                        set_label: "Reset",
                        #[watch]
                        set_sensitive: model.changes,
                        set_css_classes: &["destructive-action"],
                        connect_clicked[sender] => move |_| sender.output(FooterOutput::Reset).unwrap(),
                    },
                    gtk::Button {
                        set_label: "Save Changes",
                        #[watch]
                        set_sensitive: model.changes,
                        set_css_classes: &["suggested-action"],
                        set_tooltip_text: Some(&format!("Config file: {}", model.config_path.display())),
                        connect_clicked[sender] => move |_| sender.output(FooterOutput::Save).unwrap(),
                    },
                    gtk::Button {
                        set_label: "Close",
                        set_css_classes: &["destructive-action"],
                        connect_clicked[sender] => move |_| sender.output(FooterOutput::Close).unwrap(),
                    }
                }
            }
        }
    }

    fn init(
        config_path: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Footer {
            config_path,
            changes: false,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            FooterInput::ChangesExist(changes) => {
                self.changes = changes;
            }
        }
    }
}
