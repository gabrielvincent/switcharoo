use crate::shortcut_dialog::{
    KeyboardShortcut, KeyboardShortcutInit, KeyboardShortcutInput, KeyboardShortcutOutput,
};
use crate::structs::ConfigModifier;
use crate::util::{SetTextIfDifferent, mod_key_to_accelerator, mod_key_to_string};
use adw::gtk;
use relm4::adw::prelude::CheckButtonExt;
use relm4::gtk::prelude::{BoxExt, ButtonExt, EditableExt, EntryExt, OrientableExt, WidgetExt};
use relm4::gtk::{Align, Justification, gio};
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmWidgetExt,
    SimpleComponent, WidgetRef,
};
use std::path::Path;
use tracing::trace;

#[derive(Debug)]
pub struct Generate {
    system_data_dir: Box<Path>,
    step: u8,
    step0: Option<(ConfigModifier, String)>,
    step0_keyboard_shortcut: Controller<KeyboardShortcut>,
    step1: Option<String>,
    step1_entry: gtk::Entry,
}

#[derive(Debug)]
pub enum GenerateInput {
    Start,
    Advance,
    Back,
    Step0OpenKeyboardShortcut,
    SetStep0(Option<(ConfigModifier, String)>),
    SetStep1(Option<String>),
}

#[derive(Debug)]
pub struct GenerateInit {
    pub system_data_dir: Box<Path>,
}

#[derive(Debug)]
pub enum GenerateOutput {
    Finish(Out),
}

#[derive(Debug)]
pub struct Out {
    overview: Option<(ConfigModifier, String)>,
    default_terminal: Option<String>,
}

const MAX_STEP: u8 = 2;

#[relm4::component(pub)]
impl SimpleComponent for Generate {
    type Init = GenerateInit;
    type Input = GenerateInput;
    type Output = GenerateOutput;

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let ins = sender.input_sender().clone();
        let step0_keyboard_shortcut = KeyboardShortcut::builder()
            .launch(KeyboardShortcutInit {
                label: Some("Custom".to_string()),
                icon: None,
                init: None,
            })
            .connect_receiver(move |send, out| match out {
                KeyboardShortcutOutput::SetKey(r#mod, key) => {
                    send.emit(KeyboardShortcutInput::SetLabelText(Some(
                        mod_key_to_string(&r#mod, &key),
                    )));
                    ins.emit(GenerateInput::SetStep0(Some((r#mod, key))));
                }
                KeyboardShortcutOutput::OpenRequest => {
                    ins.emit(GenerateInput::Step0OpenKeyboardShortcut);
                }
                _ => {}
            });
        let step1_entry = gtk::Entry::new();

        let model = Generate {
            step: 0,
            step0: None,
            step0_keyboard_shortcut,
            step1: None,
            step1_entry,
            system_data_dir: init.system_data_dir.join("setup_preview").into_boxed_path(),
        };

        let step0_keyboard_button = model.step0_keyboard_shortcut.widget().clone();
        let step1_entry = &model.step1_entry;
        let widgets = view_output!();

        widgets.step0_stack.set_transition_duration(500);
        ComponentParts { model, widgets }
    }

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 20,
            gtk::Box {
                set_expand: true,
                set_spacing: 20,
                set_margin_all: 20,
                #[transition = "SlideUpDown"]
                #[name="step0_stack"]
                match model.step {
                    0 => {
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_hexpand: true,
                            set_spacing: 40,
                            gtk::Label::new(Some("Key combination to open the overview and launcher")) {
                                set_css_classes: &["title-1"],
                                set_align: Align::Center,
                                set_justify: Justification::Center,
                            },
                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_align: Align::Center,
                                set_spacing: 5,
                                gtk::CheckButton {
                                    set_label: Some(" Super"),
                                    #[watch]
                                    #[block_signal(h_1)]
                                    set_active: model.step0 == Some((ConfigModifier::Super, "Super_L".to_string())),
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep0(Some((ConfigModifier::Super, "Super_L".to_string())))) } @h_1,
                                },
                                gtk::CheckButton {
                                    set_label: Some(" Super + Tab"),
                                    #[watch]
                                    #[block_signal(h_2)]
                                    set_active: model.step0 == Some((ConfigModifier::Super, "Tab".to_string())),
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep0(Some((ConfigModifier::Super, "Tab".to_string())))) } @h_2,
                                },
                                gtk::CheckButton {
                                    set_label: Some(" Ctrl"),
                                    #[watch]
                                    #[block_signal(h_3)]
                                    set_active: model.step0 == Some((ConfigModifier::Ctrl, "Ctrl_L".to_string())),
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep0(Some((ConfigModifier::Ctrl, "Ctrl_L".to_string())))) } @h_3,
                                },
                                gtk::CheckButton {
                                    set_label: Some(" Ctrl + Tab"),
                                    #[watch]
                                    #[block_signal(h_4)]
                                    set_active: model.step0 == Some((ConfigModifier::Ctrl, "Tab".to_string())),
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep0(Some((ConfigModifier::Ctrl, "Tab".to_string())))) } @h_4,
                                },
                                gtk::CheckButton {
                                    set_label: Some(" Alt"),
                                    #[watch]
                                    #[block_signal(h_5)]
                                    set_active: model.step0 == Some((ConfigModifier::Alt, "Alt_L".to_string())),
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep0(Some((ConfigModifier::Alt, "Alt_L".to_string())))) } @h_5,
                                },
                                gtk::CheckButton {
                                    set_label: Some(" Alt + Tab"),
                                    #[watch]
                                    #[block_signal(h_6)]
                                    set_active: model.step0 == Some((ConfigModifier::Alt, "Tab".to_string())),
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep0(Some((ConfigModifier::Alt, "Tab".to_string())))) } @h_6,
                                },
                                gtk::CheckButton {
                                    set_label: None,
                                    #[local_ref]
                                    #[wrap(Some)]
                                    set_child = &step0_keyboard_button -> gtk::Button {
                                        set_sensitive: true,
                                    },
                                    #[watch]
                                    #[block_signal(h_7)]
                                    set_active: model.step0 != None &&
                                                model.step0 != Some((ConfigModifier::Super, "Super_L".to_string())) &&
                                                model.step0 != Some((ConfigModifier::Super, "Tab".to_string())) &&
                                                model.step0 != Some((ConfigModifier::Ctrl, "Ctrl_L".to_string())) &&
                                                model.step0 != Some((ConfigModifier::Ctrl, "Tab".to_string())) &&
                                                model.step0 != Some((ConfigModifier::Alt, "Alt_L".to_string())) &&
                                                model.step0 != Some((ConfigModifier::Alt, "Tab".to_string())),
                                    connect_toggled[sender] => move |b| if b.is_active() {
                                        trace!("Generate: step0_keyboard_button toggled");
                                        sender.input(GenerateInput::Step0OpenKeyboardShortcut)
                                    } @h_7
                                },
                            },
                            gtk::Picture {
                                set_file: Some(&gio::File::for_path(model.system_data_dir.join("00_overview.png"))),
                                set_css_classes: &["theme-image"],
                                set_vexpand: true,
                                set_hexpand: false,
                                set_valign: Align::Fill,
                                set_halign: Align::Center,
                            },
                            gtk::Label::new(Some("similar to gnome's overview\nShows all apps in a miniature view, allows to switch using arrow keys or tab.")) {
                                set_css_classes: &["title-4"],
                                set_justify: Justification::Center,
                                set_vexpand: true,
                                set_valign: Align::End,
                            },
                        }
                    },
                    1 => {
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_hexpand: true,
                            set_spacing: 40,
                            gtk::Label::new(Some("Default Terminal")) {
                                set_css_classes: &["title-1"],
                                set_align: Align::Center,
                                set_justify: Justification::Center,
                            },
                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_align: Align::Center,
                                set_spacing: 5,
                                gtk::CheckButton {
                                    set_label: Some(" Autodetect"),
                                    #[watch]
                                    #[block_signal(h_10)]
                                    set_active: model.step1 == None,
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep1(None)) } @h_10,
                                },
                                gtk::CheckButton {
                                    set_label: Some(" Alacritty"),
                                    #[watch]
                                    #[block_signal(h_11)]
                                    set_active: model.step1 == Some("alacritty".to_string()),
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep1(Some("alacritty".to_string()))) } @h_11,
                                },
                                gtk::CheckButton {
                                    set_label: Some(" Kitty"),
                                    #[watch]
                                    #[block_signal(h_12)]
                                    set_active: model.step1 == Some("kitty".to_string()),
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep1(Some("kitty".to_string()))) } @h_12,
                                },
                                gtk::CheckButton {
                                    set_label: Some(" Wezterm"),
                                    #[watch]
                                    #[block_signal(h_13)]
                                    set_active: model.step1 == Some("wezterm".to_string()),
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep1(Some("wezterm".to_string()))) } @h_13,
                                },
                                gtk::CheckButton {
                                    set_label: Some(" Foot"),
                                    #[watch]
                                    #[block_signal(h_14)]
                                    set_active: model.step1 == Some("foot".to_string()),
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep1(Some("foot".to_string()))) } @h_14,
                                },
                                gtk::CheckButton {
                                    set_label: Some(" Qterminal"),
                                    #[watch]
                                    #[block_signal(h_15)]
                                    set_active: model.step1 == Some("qterminal".to_string()),
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep1(Some("qterminal".to_string()))) } @h_15,
                                },
                                gtk::CheckButton {
                                    set_label: Some(" Lilyterm"),
                                    #[watch]
                                    #[block_signal(h_16)]
                                    set_active: model.step1 == Some("lilyterm".to_string()),
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep1(Some("lilyterm".to_string()))) } @h_16,
                                },
                                gtk::CheckButton {
                                    set_label: Some(" Tilix"),
                                    #[watch]
                                    #[block_signal(h_17)]
                                    set_active: model.step1 == Some("tilix".to_string()),
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep1(Some("tilix".to_string()))) } @h_17,
                                },
                                gtk::CheckButton {
                                    set_label: Some(" Terminix"),
                                    #[watch]
                                    #[block_signal(h_18)]
                                    set_active: model.step1 == Some("terminix".to_string()),
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep1(Some("terminix".to_string()))) } @h_18,
                                },
                                gtk::CheckButton {
                                    set_label: Some(" konsole"),
                                    #[watch]
                                    #[block_signal(h_19)]
                                    set_active: model.step1 == Some("konsole".to_string()),
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep1(Some("konsole".to_string()))) } @h_19,
                                },
                                gtk::CheckButton {
                                    set_label: None,
                                    #[local_ref]
                                    #[wrap(Some)]
                                    set_child = step1_entry -> gtk::Entry {
                                        #[watch]
                                        #[block_signal(h_20)]
                                        set_text_if_different: if model.step1 != None &&
                                                model.step1 != Some("alacritty".to_string()) &&
                                                model.step1 != Some("kitty".to_string()) &&
                                                model.step1 != Some("wezterm".to_string()) &&
                                                model.step1 != Some("foot".to_string()) &&
                                                model.step1 != Some("qterminal".to_string()) &&
                                                model.step1 != Some("lilyterm".to_string()) &&
                                                model.step1 != Some("tilix".to_string()) &&
                                                model.step1 != Some("terminix".to_string()) &&
                                                model.step1 != Some("konsole".to_string()) { &model.step1.as_ref().map_or("", |v| v) } else { "" },
                                        connect_changed[sender] => move |e| { sender.input(GenerateInput::SetStep1(Some(e.text().into())))} @h_20,
                                        set_input_purpose: gtk::InputPurpose::FreeForm,
                                        set_placeholder_text: Some("/usr/bin/kitty"),
                                    },
                                    #[watch]
                                    #[block_signal(h_21)]
                                    set_active: model.step1 != None &&
                                                model.step1 != Some("alacritty".to_string()) &&
                                                model.step1 != Some("kitty".to_string()) &&
                                                model.step1 != Some("wezterm".to_string()) &&
                                                model.step1 != Some("foot".to_string()) &&
                                                model.step1 != Some("qterminal".to_string()) &&
                                                model.step1 != Some("lilyterm".to_string()) &&
                                                model.step1 != Some("tilix".to_string()) &&
                                                model.step1 != Some("terminix".to_string()) &&
                                                model.step1 != Some("konsole".to_string()),
                                    connect_toggled[sender] => move |b| if b.is_active() { sender.input(GenerateInput::SetStep1(Some("".to_string()))) } @h_21,
                                }
                            }
                        }
                    },
                    2 => {
                        gtk::Box {
                             gtk::Label::new(Some("TODO")) {}
                        }
                    },
                    _ => {
                        gtk::Label::new(Some("INVALID GENERATE STEP")) {}
                    },
                }
            },
            gtk::Box {
                set_spacing: 20,
                set_halign: Align::Center,
                gtk::Button {
                    set_label: "Back",
                    set_css_classes: &["suggested-action", "pill"],
                    connect_clicked[sender] => move |_| sender.input(GenerateInput::Back),
                },
                gtk::Button {
                    #[watch]
                    set_label: if model.step == MAX_STEP { "Finish" } else { "Next" },
                    set_css_classes: &["suggested-action", "pill"],
                    connect_clicked[sender] => move |_| sender.input(GenerateInput::Advance),
                }
            }
        }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        trace!("launcher::update: {message:?}");
        match message {
            GenerateInput::Advance => {
                if self.step < MAX_STEP {
                    self.step += 1;
                } else {
                    sender
                        .output(GenerateOutput::Finish(Out {
                            overview: self.step0.clone(),
                            default_terminal: self.step1.clone(),
                        }))
                        .unwrap();
                }
            }
            GenerateInput::Back => {
                if self.step > 0 {
                    self.step -= 1;
                }
            }
            GenerateInput::Step0OpenKeyboardShortcut => {
                self.step0_keyboard_shortcut
                    .sender()
                    .send(KeyboardShortcutInput::SetWidgetRef(
                        self.step0_keyboard_shortcut.widget().widget_ref().clone(),
                    ))
                    .unwrap();
                self.step0_keyboard_shortcut.emit(
                    KeyboardShortcutInput::ShowKeyboardShortcutDialog(self.step0.clone()),
                );
            }
            GenerateInput::SetStep0(step0) => {
                self.step0 = step0;
            }
            GenerateInput::Start => {
                self.step = 0;
                self.step0 = None;
            }
            GenerateInput::SetStep1(step1) => {
                self.step1 = step1;
            }
        }
    }
}
