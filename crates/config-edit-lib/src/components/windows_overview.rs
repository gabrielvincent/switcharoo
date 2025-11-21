use crate::structs::Modifier;
use relm4::adw;
use relm4::adw::gtk;
use relm4::adw::prelude::*;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};

#[derive(Debug)]
pub struct WindowsOverview {
    config: crate::Overview,
    prev_config: crate::Overview,
    key_buffer: gtk::EntryBuffer,
}

#[derive(Debug)]
pub enum WindowsOverviewInput {
    SetOverview(crate::Overview),
}

#[derive(Debug)]
pub struct WindowsOverviewInit {
    pub config: crate::Overview,
}

#[derive(Debug)]
pub enum WindowsOverviewOutput {
    Enabled(bool),
    Key(String),
    Modifier(Modifier),
    FilterSameClass(bool),
    FilterWorkspace(bool),
    FilterMonitor(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for WindowsOverview {
    type Init = WindowsOverviewInit;
    type Input = WindowsOverviewInput;
    type Output = WindowsOverviewOutput;

    view! {
        #[root]
        adw::ExpanderRow {
            set_title_selectable: true,
            set_show_enable_switch: true,
            set_hexpand: true,
            set_css_classes: &["enable-frame"],
            set_title: "Overview + Launcher",
            connect_enable_expansion_notify[sender] => move |e| {sender.output(WindowsOverviewOutput::Enabled(e.enables_expansion())).unwrap();},
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
                        set_css_classes: if model.config.key == model.prev_config.key { &[] } else { &["blue-label"]  },
                        set_label: "Key",
                    },
                    gtk::Image::from_icon_name("dialog-information-symbolic") {
                        set_tooltip_text: Some("The key to use to open the Overview mode (like `tab` or `alt_r`). If you want to only open using a modifier, set this to the modifier name like `super_l`")
                    },
                    #[name = "gtk_entry"]
                    gtk::Entry {
                        connect_changed[sender] => move |e| { sender.output(WindowsOverviewOutput::Key(e.text().into())).unwrap(); } @h_2,
                        set_buffer: &model.key_buffer,
                        set_input_purpose: gtk::InputPurpose::FreeForm,
                        set_placeholder_text: Some("super_l"),
                        set_hexpand: true,
                    }
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 10,
                    gtk::Label {
                        #[watch]
                        set_css_classes: if model.config.modifier == model.prev_config.modifier { &[] } else { &["blue-label"]  },
                        set_label: "Modifier",
                    },
                    gtk::Image::from_icon_name("dialog-information-symbolic") {
                        set_tooltip_text: Some("The modifier that must be pressed together with the key to open the Overview mode (like ctrl)")
                    },
                    gtk::DropDown::from_strings(Modifier::strings()) {
                        connect_selected_notify[sender] => move |e| {sender.output(WindowsOverviewOutput::Modifier(e.selected().try_into().expect("invalid modifier"))).unwrap(); },
                        #[watch]
                        set_selected: model.config.modifier.into(),
                        set_hexpand: true,
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
                            connect_active_notify[sender] => move |c| { sender.output(WindowsOverviewOutput::FilterSameClass(c.is_active())).unwrap(); },
                            #[watch]
                            set_active: model.config.same_class,
                            set_title: "Same class",
                        },
                        add_row = &adw::SwitchRow {
                            connect_active_notify[sender] => move |c| { sender.output(WindowsOverviewOutput::FilterWorkspace(c.is_active())).unwrap(); },
                            #[watch]
                            set_active: model.config.current_workspace,
                            set_title: "Current workspace",
                        },
                        add_row = &adw::SwitchRow {
                            connect_active_notify[sender] => move |c| { sender.output(WindowsOverviewOutput::FilterMonitor(c.is_active())).unwrap(); },
                            #[watch]
                            set_active: model.config.current_monitor,
                            set_title: "Current monitor",
                        }
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
        let model = WindowsOverview {
            key_buffer: gtk::EntryBuffer::new(Some(init.config.key.clone())),
            config: init.config.clone(),
            prev_config: init.config,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: WindowsOverviewInput, _sender: ComponentSender<Self>) {
        match msg {
            WindowsOverviewInput::SetOverview(config) => {
                self.config = config;
                if self.key_buffer.text() != self.config.key {
                    self.key_buffer.set_text(&self.config.key);
                }
            }
        }
    }
}
