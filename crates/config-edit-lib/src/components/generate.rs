use adw::gtk;
use relm4::gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

#[derive(Debug)]
pub struct Generate {
    step: u8,
}

#[derive(Debug)]
pub enum GenerateInput {
    Advance,
    Back,
}

#[derive(Debug)]
pub struct GenerateInit {}

#[derive(Debug)]
pub enum GenerateOutput {}

const MAX_STEP: u8 = 2;

#[relm4::component(pub)]
impl SimpleComponent for Generate {
    type Init = GenerateInit;
    type Input = GenerateInput;
    type Output = GenerateOutput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 20,
            gtk::Box {
                set_expand: true,
                set_spacing: 20,
                set_valign: gtk::Align::Fill,
                set_halign: gtk::Align::Center,
                match model.step {
                    0 => {
                        gtk::Box {
                             gtk::Label::new(Some("Select a window to 0")) {},
                             gtk::Label::new(Some("Select a window to 0")) {}
                        }
                    },
                    1 => {
                        gtk::Box {
                             gtk::Label::new(Some("Select a window to 1")) {},
                             gtk::Label::new(Some("Select a window to 1")) {}
                        }
                    },
                    2 => {
                        gtk::Box {
                             gtk::Label::new(Some("Select a window to 2")) {},
                             gtk::Label::new(Some("Select a window to 2")) {}
                        }
                    },
                    _ => {
                        gtk::Label::new(Some("INVALID GENERATE STEP")) {}
                    },
                }
            },
            gtk::Box {
                set_spacing: 20,
                set_halign: gtk::Align::Center,
                gtk::Button {
                    set_label: "Back",
                    set_css_classes: &["suggested-action", "pill"],
                    connect_clicked[sender] => move |_| sender.input(GenerateInput::Back),
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Generate { step: 2 };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            GenerateInput::Advance => {
                if self.step < MAX_STEP {
                    self.step += 1;
                }
            }
            GenerateInput::Back => {
                if self.step > 0 {
                    self.step -= 1;
                }
            }
        }
    }
}
