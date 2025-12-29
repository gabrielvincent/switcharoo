use crate::components::launcher_plugins::{
    LauncherPlugins, LauncherPluginsInit, LauncherPluginsInput, LauncherPluginsOutput,
};
use crate::structs::ConfigModifier;
use crate::util::{SetCursor, SetTextIfDifferent};
use relm4::ComponentController;
use relm4::adw::gtk;
use relm4::adw::prelude::*;
use relm4::{
    Component, ComponentParts, ComponentSender, Controller, RelmWidgetExt, SimpleComponent,
};
use tracing::trace;

#[derive(Debug)]
pub struct Launcher {
    config: crate::Launcher,
    prev_config: crate::Launcher,
    launcher_plugins: Controller<LauncherPlugins>,
}

#[derive(Debug)]
pub enum LauncherInput {
    SetLauncher(crate::Launcher),
    SetPrevLauncher(crate::Launcher),
    ResetLauncher,
}

#[derive(Debug)]
pub struct LauncherInit {
    pub config: crate::Launcher,
}

#[derive(Debug)]
pub enum LauncherOutput {
    Modifier(ConfigModifier),
    Width(u32),
    MaxItems(u8),
    DefaultTerminal(Option<String>),
    LauncherPlugins(LauncherPluginsOutput),
}

#[relm4::component(pub)]
impl SimpleComponent for Launcher {
    type Init = LauncherInit;
    type Input = LauncherInput;
    type Output = LauncherOutput;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_margin_all: 10,
            adw::ExpanderRow {
                set_title_selectable: true,
                set_show_enable_switch: false,
                set_hexpand: true,
                set_css_classes: &["enable-frame"],
                set_title: "Launcher",
                set_expanded: true,
                add_row = &gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_css_classes: &["frame-row"],
                    set_spacing: 30,
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,
                        gtk::Label {
                            #[watch]
                            set_css_classes: if model.config.launch_modifier == model.prev_config.launch_modifier { &[] } else { &["blue-label"]  },
                            set_label: "Modifier",
                        },
                        gtk::Image::from_icon_name("dialog-information-symbolic") {
                            set_cursor_by_name: "help",
                            set_tooltip_text: Some("The modifier used to select items in the launcher, pressing `<Mod> + 1` to open second entry, `<Mod> + t` to run in terminal, etc.")
                        },
                        gtk::DropDown::from_strings(ConfigModifier::strings()) {
                            #[watch]
                            #[block_signal(h_1)]
                            set_selected: model.config.launch_modifier.into(),
                            connect_selected_notify[sender] => move |e| {sender.output(LauncherOutput::Modifier(e.selected().try_into().expect("invalid modifier"))).unwrap() } @ h_1,
                            set_hexpand: true,
                        }
                    },
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,
                        gtk::Label {
                            #[watch]
                            set_css_classes: if model.config.width == model.prev_config.width { &[] } else { &["blue-label"]  },
                            set_label: "Width",
                        },
                        gtk::Image::from_icon_name("dialog-information-symbolic") {
                            set_cursor_by_name: "help",
                            set_tooltip_text: Some("The width of the launcher in pixels.")
                        },
                        gtk::SpinButton {
                            set_adjustment: &gtk::Adjustment::new(0.0, 0.0, 2000.0, 50.0, 100.0, 0.0),
                            set_hexpand: true,
                            set_digits: 0,
                            #[watch]
                            #[block_signal(h_2)]
                            set_value: model.config.width as f64,
                            connect_value_changed[sender] => move |e| { sender.output(LauncherOutput::Width(e.value() as u32)).unwrap() } @h_2,
                        }
                    },
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,
                        gtk::Label {
                            #[watch]
                            set_css_classes: if model.config.max_items == model.prev_config.max_items { &[] } else { &["blue-label"]  },
                            set_label: "Max items",
                        },
                        gtk::Image::from_icon_name("dialog-information-symbolic") {
                            set_cursor_by_name: "help",
                            set_tooltip_text: Some("Sets the maximum number of items to show in the launcher.")
                        },
                        gtk::SpinButton {
                            set_adjustment: &gtk::Adjustment::new(0.0, 0.0, 10.0, 1.0, 2.0, 0.0),
                            set_hexpand: true,
                            set_digits: 0,
                            #[watch]
                            #[block_signal(h_3)]
                            set_value: model.config.max_items as f64,
                            connect_value_changed[sender] => move |e| { sender.output(LauncherOutput::MaxItems(e.value() as u8)).unwrap() } @h_3,
                        }
                    }
                },
                add_row = &gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_css_classes: &["frame-row"],
                    set_spacing: 30,
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,
                        gtk::Label {
                            #[watch]
                            set_css_classes: if model.config.default_terminal == model.prev_config.default_terminal { &[] } else { &["blue-label"]  },
                            set_label: "Autodetect Terminal",
                        },
                        gtk::Switch {
                            #[watch]
                            #[block_signal(h_4)]
                            set_active: !model.config.default_terminal.is_some(),
                            set_valign: gtk::Align::Center,
                            connect_active_notify[sender] => move |e| { sender.output(LauncherOutput::DefaultTerminal(if e.is_active() { None } else { Some("".to_string()) })).unwrap() } @h_4,
                        },
                        gtk::Image::from_icon_name("dialog-information-symbolic") {
                            set_cursor_by_name: "help",
                            set_tooltip_text: Some("name/path of the default terminal to use. This value is optional, if unset a list of default terminals is used to find a default terminal. Will be used to launch terminal apps and for the terminal plugin.")
                        },
                        gtk::Entry {
                            #[watch]
                            set_sensitive: model.config.default_terminal.is_some(),
                            #[watch]
                            #[block_signal(h_5)]
                            set_text_if_different: &model.config.default_terminal.as_ref().unwrap_or(&"".to_string()),
                            connect_changed[sender] => move |e| { sender.output(LauncherOutput::DefaultTerminal(Some(e.text().into()))).unwrap()} @h_5,
                            set_input_purpose: gtk::InputPurpose::FreeForm,
                            set_placeholder_text: Some("kitty"),
                            set_hexpand: true,
                        }
                    },
                },
                add_row = model.launcher_plugins.widget(),
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let launcher_plugins = LauncherPlugins::builder()
            .launch(LauncherPluginsInit {
                config: init.config.plugins.clone(),
            })
            .forward(sender.output_sender(), LauncherOutput::LauncherPlugins);

        let model = Launcher {
            config: init.config.clone(),
            prev_config: init.config,
            launcher_plugins,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        trace!("launcher::update: {message:?}");
        match message {
            LauncherInput::SetLauncher(config) => {
                self.config = config;
                self.launcher_plugins
                    .emit(LauncherPluginsInput::SetLauncherPlugins(
                        self.config.plugins.clone(),
                    ));
            }
            LauncherInput::SetPrevLauncher(config) => {
                self.prev_config = config;
                self.launcher_plugins
                    .emit(LauncherPluginsInput::SetPrevLauncherPlugins(
                        self.prev_config.plugins.clone(),
                    ));
            }
            LauncherInput::ResetLauncher => {
                self.config = self.prev_config.clone();
                self.launcher_plugins
                    .emit(LauncherPluginsInput::ResetLauncherPlugins)
            }
        }
    }
}
