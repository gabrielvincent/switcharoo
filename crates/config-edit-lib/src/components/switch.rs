use crate::flags_csv;
use crate::shortcut_dialog::{
    KeyboardShortcut, KeyboardShortcutInit, KeyboardShortcutInput, KeyboardShortcutOutput,
};
use crate::structs::ConfigModifier;
use crate::util::{SetCursor, SetTextIfDifferent, to_accelerator};
use adw::gtk::Align;
use relm4::ComponentController;
use relm4::adw::gtk;
use relm4::adw::prelude::*;
use relm4::{Component, Controller, adw};
use relm4::{ComponentParts, ComponentSender, SimpleComponent};
use tracing::trace;

#[derive(Debug)]
pub struct Switch {
    config: crate::Switch,
    prev_config: crate::Switch,
    name: &'static str,
    keyboard_shortcut: Controller<KeyboardShortcut>,
}

#[derive(Debug)]
pub enum SwitchInput {
    SetSwitch(crate::Switch),
    SetPrevSwitch(crate::Switch),
    ResetSwitch,
    OpenKeyboardShortcut,
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
    SwitchWorkspaces(bool),
    ExcludeSpecialWorkspaces(String),
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
                model.keyboard_shortcut.widget().clone() -> gtk::Button {
                    #[watch]
                    set_sensitive: model.config.enabled,
                },
                adw::ShortcutLabel::new(&to_accelerator(model.config.modifier, &model.config.key).unwrap_or_default()) {
                    #[watch]
                    set_accelerator: &to_accelerator(model.config.modifier, &model.config.key).unwrap_or_default(),
                    #[watch]
                    set_css_classes: if !model.config.enabled {
                        &["gray-label"]
                    } else {
                        if to_accelerator(model.config.modifier, &model.config.key) == to_accelerator(model.prev_config.modifier, &model.prev_config.key)
                            { &[] }
                        else
                            { &["blue-label"] }
                    },
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
                        set_cursor_by_name: "help",
                        set_tooltip_text: Some("Filter the shown windows by the provided filters")
                    },
                    adw::ExpanderRow {
                        #[watch]
                        set_title: &flags_csv!(model.config,same_class,current_monitor,current_workspace),
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
                        set_label: "Switch Workspaces",
                        #[watch]
                        set_css_classes: if model.config.switch_workspaces == model.prev_config.switch_workspaces { &[] } else { &["blue-label"] },
                    },
                    gtk::Image::from_icon_name("dialog-information-symbolic") {
                        set_cursor_by_name: "help",
                        set_tooltip_text: Some("Switch between workspaces in the Switch mode instead of windows")
                    },
                    gtk::Switch {
                        #[watch]
                        set_active: model.config.switch_workspaces,
                        set_valign: Align::Center,
                        connect_active_notify[sender] => move |e| { sender.output(SwitchOutput::SwitchWorkspaces(e.is_active())).unwrap() },
                    },
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 10,
                    gtk::Label {
                        set_label: "Exclude special workspaces (TODO)",
                        #[watch]
                        set_css_classes: if model.config.exclude_special_workspaces == model.prev_config.exclude_special_workspaces { &[] } else { &["blue-label"] },
                    },
                    gtk::Image::from_icon_name("dialog-information-symbolic") {
                        set_cursor_by_name: "help",
                        set_tooltip_text: Some("Exclude special workspaces by regex \n(hyprctl workspaces -j | jq \".[].name\")")
                    },
                    gtk::Entry {
                        set_input_purpose: gtk::InputPurpose::FreeForm,
                        set_placeholder_text: Some("special:(monitor|second)"),
                        set_hexpand: true,
                        set_valign: Align::Center,
                        connect_changed[sender] => move |e| { sender.output(SwitchOutput::ExcludeSpecialWorkspaces(e.text().into())).unwrap() } @h,
                        #[watch]
                        #[block_signal(h)]
                        set_text_if_different: &model.config.exclude_special_workspaces,
                    }
                },
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let outs = sender.output_sender().clone();
        let ins = sender.input_sender().clone();
        let keyboard_shortcut = KeyboardShortcut::builder()
            .launch(KeyboardShortcutInit {
                key: init.config.key.clone(),
                modifier: init.config.modifier.clone(),
            })
            .connect_receiver(move |_, out| match out {
                KeyboardShortcutOutput::SetKey(key, r#mod) => {
                    outs.emit(SwitchOutput::Key(key));
                    outs.emit(SwitchOutput::Modifier(r#mod));
                }
                KeyboardShortcutOutput::OpenRequest => {
                    ins.emit(SwitchInput::OpenKeyboardShortcut);
                }
                _ => {}
            });

        let model = Switch {
            name: init.name,
            config: init.config.clone(),
            prev_config: init.config,
            keyboard_shortcut,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        trace!("switch::update: {message:?}");
        match message {
            SwitchInput::SetSwitch(config) => {
                self.config = config;
            }
            SwitchInput::SetPrevSwitch(config) => {
                self.prev_config = config;
            }
            SwitchInput::ResetSwitch => {
                self.config = self.prev_config.clone();
            }
            SwitchInput::OpenKeyboardShortcut => {
                self.keyboard_shortcut
                    .emit(KeyboardShortcutInput::ShowKeyboardShortcut(
                        self.config.key.clone(),
                        self.config.modifier,
                    ));
            }
        }
    }
}
