use crate::util::SetCursor;
use relm4::adw::prelude::*;
use relm4::gtk::Align;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};
use relm4::{adw, gtk};
use tracing::trace;

#[derive(Debug)]
pub struct SimplePlugin {
    config: crate::EmptyConfig,
    prev_config: crate::EmptyConfig,
    todo: bool,
}

#[derive(Debug)]
pub enum SimplePluginInput {
    SetSimplePluginConfig(crate::EmptyConfig),
    SetPrevSimplePluginConfig(crate::EmptyConfig),
    ResetSimplePluginConfig,
}

#[derive(Debug)]
pub struct SimplePluginInit {
    pub name: &'static str,
    pub description: &'static str,
    pub todo: bool,
    pub config: crate::EmptyConfig,
}

#[derive(Debug)]
pub enum SimplePluginOutput {
    Enabled(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for SimplePlugin {
    type Init = SimplePluginInit;
    type Input = SimplePluginInput;
    type Output = SimplePluginOutput;

    view! {
        #[root]
        adw::ExpanderRow {
            set_title_selectable: true,
            set_show_enable_switch: !model.todo,
            set_hexpand: true,
            set_css_classes: &["enable-frame"],
            add_prefix = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_halign: Align::Fill,
                set_valign: Align::Center,
                set_spacing: 15,
                gtk::Label {
                    set_label: init.name,
                    set_css_classes: if init.todo { &["gray-label"] } else { &[] },
                },
                gtk::Image::from_icon_name("dialog-information-symbolic") {
                    set_cursor_by_name: "help",
                    set_tooltip_text: Some(init.description)
                },
            },
            #[watch]
            #[block_signal(h)]
            set_enable_expansion: !model.todo && model.config.enabled,
            connect_enable_expansion_notify[sender] => move |e| {sender.output(SimplePluginOutput::Enabled(e.enables_expansion())).unwrap()} @h,
            #[watch]
            set_expanded: !model.todo && model.config.enabled,
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SimplePlugin {
            config: init.config.clone(),
            prev_config: init.config,
            todo: init.todo,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        trace!("launcher_plugins::simple::update: {message:?}");
        match message {
            SimplePluginInput::SetSimplePluginConfig(config) => {
                self.config = config;
            }
            SimplePluginInput::SetPrevSimplePluginConfig(config) => {
                self.prev_config = config;
            }
            SimplePluginInput::ResetSimplePluginConfig => {
                self.config = self.prev_config.clone();
            }
        }
    }
}
