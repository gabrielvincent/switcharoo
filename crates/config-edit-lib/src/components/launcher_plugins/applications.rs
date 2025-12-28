use crate::util::SetCursor;
use adw::gtk;
use adw::gtk::{Adjustment, Align};
use adw::prelude::*;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};
use tracing::trace;

#[derive(Debug)]
pub struct Applications {
    config: crate::ApplicationsPluginConfig,
    prev_config: crate::ApplicationsPluginConfig,
}

#[derive(Debug)]
pub enum ApplicationsInput {
    SetApplicationsPluginConfig(crate::ApplicationsPluginConfig),
    SetPrevApplicationsPluginConfig(crate::ApplicationsPluginConfig),
    ResetApplicationsPluginConfig,
}

#[derive(Debug)]
pub struct ApplicationsInit {
    pub config: crate::ApplicationsPluginConfig,
}

#[derive(Debug)]
pub enum ApplicationsOutput {
    Enabled(bool),
    ShowExecs(bool),
    ShowActions(bool),
    CacheWeeks(u8),
}

#[relm4::component(pub)]
impl SimpleComponent for Applications {
    type Init = ApplicationsInit;
    type Input = ApplicationsInput;
    type Output = ApplicationsOutput;

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
                set_spacing: 15,
                gtk::Label {
                    set_label: "Applications",
                },
                gtk::Image::from_icon_name("dialog-information-symbolic") {
                    set_cursor_by_name: "help",
                    set_tooltip_text: Some("TODO")
                },
            },
            connect_enable_expansion_notify[sender] => move |e| {sender.output(ApplicationsOutput::Enabled(e.enables_expansion())).unwrap()},
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
                        set_css_classes: if model.config.run_cache_weeks == model.prev_config.run_cache_weeks { &[] } else { &["blue-label"]  },
                        set_label: "Run cache period (weeks)",
                    },
                    gtk::Image::from_icon_name("dialog-information-symbolic") {
                        set_cursor_by_name: "help",
                        set_tooltip_text: Some("TODO")
                    },
                    gtk::SpinButton {
                        set_adjustment: &Adjustment::new(0.0, 0.0, 52.0, 1.0, 8.0, 0.0),
                        set_digits: 0,
                        set_hexpand: true,
                        connect_value_changed[sender] => move |e| { sender.output(ApplicationsOutput::CacheWeeks(e.value() as u8)).unwrap() } @h_3,
                        #[watch] // IMPORTANT: always call this last, else the initial value will not be set
                        #[block_signal(h_3)]
                        set_value: model.config.run_cache_weeks as f64,
                    }
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 10,
                    gtk::Label {
                        #[watch]
                        set_css_classes: if model.config.show_execs == model.prev_config.show_execs { &[] } else { &["blue-label"]  },
                        set_label: "Show Execs",
                    },
                    gtk::Image::from_icon_name("dialog-information-symbolic") {
                        set_cursor_by_name: "help",
                        set_tooltip_text: Some("TODO")
                    },
                    gtk::Switch {
                        set_valign: Align::Center,
                        connect_active_notify[sender] => move |e| { sender.output(ApplicationsOutput::ShowExecs(e.is_active())).unwrap() },
                        #[watch]
                        set_active: model.config.show_execs,
                    },
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 10,
                    gtk::Label {
                        #[watch]
                        set_css_classes: if model.config.show_actions_submenu == model.prev_config.show_actions_submenu { &[] } else { &["blue-label"]  },
                        set_label: "Show Actions",
                    },
                    gtk::Image::from_icon_name("dialog-information-symbolic") {
                        set_cursor_by_name: "help",
                        set_tooltip_text: Some("TODO")
                    },
                    gtk::Switch {
                        set_valign: Align::Center,
                        connect_active_notify[sender] => move |e| { sender.output(ApplicationsOutput::ShowActions(e.is_active())).unwrap() },
                        #[watch]
                        set_active: model.config.show_actions_submenu,
                    },
                },
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Applications {
            config: init.config.clone(),
            prev_config: init.config,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        trace!("launcher_plugins::application::update: {message:?}");
        match message {
            ApplicationsInput::SetApplicationsPluginConfig(config) => {
                self.config = config;
            }
            ApplicationsInput::SetPrevApplicationsPluginConfig(config) => {
                self.prev_config = config;
            }
            ApplicationsInput::ResetApplicationsPluginConfig => {
                self.config = self.prev_config.clone();
            }
        }
    }
}
