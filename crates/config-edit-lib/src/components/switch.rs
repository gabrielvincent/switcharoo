use crate::SetTextIfDifferent;
use crate::structs::{ConfigModifier, to_accelerator};
use adw::gdk::{Key, ModifierType};
use adw::gtk::Align;
use relm4::ComponentController;
use relm4::adw::gtk;
use relm4::adw::prelude::*;
use relm4::gtk::EventControllerKey;
use relm4::{Component, Controller, adw};
use relm4::{ComponentParts, ComponentSender, SimpleComponent};
use relm4_components::alert::{Alert, AlertMsg, AlertResponse, AlertSettings};

#[derive(Debug)]
pub struct Switch {
    config: crate::Switch,
    prev_config: crate::Switch,
    get_keyboard_shortcut: bool,
    name: &'static str,
    keyboard_dialog: Controller<Alert>,
}

#[derive(Debug)]
pub enum SwitchInput {
    SetSwitch(crate::Switch),
    ToggleGetKeyboardShortcut,
}

#[derive(Debug)]
pub struct SwitchInit {
    pub config: crate::Switch,
    pub name: &'static str,
}

#[derive(Debug)]
pub enum SwitchOutput {
    Enabled(bool),
    Key(String),
    Modifier(ConfigModifier),
    FilterSameClass(bool),
    FilterWorkspace(bool),
    FilterMonitor(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for Switch {
    type Init = SwitchInit;
    type Input = SwitchInput;
    type Output = SwitchOutput;

    view! {
        #[root]
        adw::ExpanderRow {
            set_title_selectable: true,
            set_show_enable_switch: true,
            set_hexpand: true,
            set_css_classes: &["enable-frame"],
            add_prefix = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_halign: Align::Fill,
                set_valign: Align::Center,
                set_spacing: 25,
                gtk::Label {
                    set_label: model.name,
                },
                gtk::Button {
                    set_icon_name: "keyboard-layout",
                    #[watch]
                    set_css_classes: if model.get_keyboard_shortcut { &["active"] } else { &["not-active"] },
                    connect_clicked[sender] => move |_| sender.input(SwitchInput::ToggleGetKeyboardShortcut),
                },
                adw::ShortcutLabel::new(&to_accelerator(model.config.modifier, &model.config.key).unwrap_or_default()) {
                    #[watch]
                    set_accelerator: &to_accelerator(model.config.modifier, &model.config.key).unwrap_or_default(),
                    #[watch]
                    set_css_classes: if to_accelerator(model.config.modifier, &model.config.key) == to_accelerator(model.prev_config.modifier, &model.prev_config.key)  { &[] } else { &["blue-label"]  },
                },
            },
            connect_enable_expansion_notify[sender] => move |e| {sender.output(SwitchOutput::Enabled(e.enables_expansion())).unwrap()},
            #[watch]
            set_enable_expansion: model.config.enabled,
            #[watch]
            set_expanded: model.config.enabled,
            add_row = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_css_classes: &["frame-row"],
                set_spacing: 30,
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 10,
                    gtk::Label {
                        #[watch]
                        set_css_classes: if model.config.same_class == model.prev_config.same_class &&
                            model.config.current_workspace == model.prev_config.current_workspace &&
                            model.config.current_monitor == model.prev_config.current_monitor { &[] } else { &["blue-label"]  },
                        set_label: "Filter",
                    },
                    gtk::Image::from_icon_name("dialog-information-symbolic") {
                        set_tooltip_text: Some("Filter the shown windows by the provided filters")
                    },
                    adw::ExpanderRow {
                        set_title: "Filter",
                        set_hexpand: true,
                        set_title_lines: 2,
                        set_css_classes: &["item-expander"],
                        add_row = &adw::SwitchRow {
                            connect_active_notify[sender] => move |c| { sender.output(SwitchOutput::FilterSameClass(c.is_active())).unwrap() },
                            #[watch]
                            set_active: model.config.same_class,
                            set_title: "Same class",
                        },
                        add_row = &adw::SwitchRow {
                            connect_active_notify[sender] => move |c| { sender.output(SwitchOutput::FilterWorkspace(c.is_active())).unwrap() },
                            #[watch]
                            set_active: model.config.current_workspace,
                            set_title: "Current workspace",
                        },
                        add_row = &adw::SwitchRow {
                            connect_active_notify[sender] => move |c| { sender.output(SwitchOutput::FilterMonitor(c.is_active())).unwrap() },
                            #[watch]
                            set_active: model.config.current_monitor,
                            set_title: "Current monitor",
                        }
                    }
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 10,
                    gtk::Label {
                        set_label: "Exclude special workspaces",
                    },
                    gtk::Image::from_icon_name("dialog-information-symbolic") {
                        set_tooltip_text: Some("Exclude special workspaces by regex (hyprctl workspaces -j | jq \".[].name\")")
                    },
                    gtk::Entry {
                        set_input_purpose: gtk::InputPurpose::FreeForm,
                        set_placeholder_text: Some("special:(monitor|second)"),
                        set_hexpand: true,
                        set_valign: Align::Center
                    }
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let entry = gtk::Label::new(None);

        let keyboard_dialog = Alert::builder()
            .transient_for(&root)
            .launch(AlertSettings {
                text: Some("Press Keyboard shortcut".to_string()),
                secondary_text: None,
                confirm_label: Some("Use".to_string()),
                cancel_label: Some("Cancel".to_string()),
                option_label: None,
                is_modal: true,
                destructive_accept: false,
                extra_child: Some(entry.clone().into()),
            })
            .forward(sender.input_sender(), |res| match res {
                AlertResponse::Confirm => SwitchInput::ToggleGetKeyboardShortcut,
                AlertResponse::Cancel => SwitchInput::ToggleGetKeyboardShortcut,
                AlertResponse::Option => unreachable!("no option button in alert dialog"),
            });

        // Attach an EventControllerKey to the alert dialog's window to print raw key events.
        let key_controller = EventControllerKey::new();
        let entry = entry.clone();
        let window = keyboard_dialog.widgets().gtk_window_12.clone();
        let send = sender.clone();
        key_controller.connect_key_pressed(move |_, val, id, state| {
            tracing::debug!("Raw key event - val: {}, state: {:?}", val, state);
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
                    send.output(SwitchOutput::Key(format!("code:{id}")))
                        .unwrap();
                    send.output(SwitchOutput::Modifier(modifier)).unwrap();
                    if modifier == ConfigModifier::None {
                        entry.set_label(&key_name);
                    } else {
                        entry.set_label(&format!("{modifier} + {key_name}"));
                    };
                } else {
                    entry.set_label("---");
                }
            } else {
                entry.set_label("---");
            }
            gtk::glib::Propagation::Stop
        });
        window.add_controller(key_controller);

        let model = Switch {
            config: init.config.clone(),
            prev_config: init.config,
            get_keyboard_shortcut: false,
            keyboard_dialog,
            name: init.name,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            SwitchInput::SetSwitch(config) => {
                self.config = config;
            }
            SwitchInput::ToggleGetKeyboardShortcut => {
                self.get_keyboard_shortcut = !self.get_keyboard_shortcut;
                if self.get_keyboard_shortcut {
                    self.keyboard_dialog.emit(AlertMsg::Show);
                }
                self.keyboard_dialog.widgets().gtk_window_12.set_modal(true);
            }
        }
    }
}
