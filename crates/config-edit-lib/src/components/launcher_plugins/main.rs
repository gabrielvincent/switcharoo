use crate::components::launcher_plugins::applications::{
    Applications, ApplicationsInit, ApplicationsInput, ApplicationsOutput,
};
use crate::components::launcher_plugins::simple::{
    SimplePlugin, SimplePluginInit, SimplePluginInput, SimplePluginOutput,
};
use relm4::ComponentController;
use relm4::adw::prelude::*;
use relm4::{
    Component, ComponentParts, ComponentSender, Controller, RelmWidgetExt, SimpleComponent,
};
use relm4::{adw, gtk};
use tracing::trace;

#[derive(Debug)]
pub struct LauncherPlugins {
    config: crate::Plugins,
    prev_config: crate::Plugins,
    applications: Controller<Applications>,
    run_in_terminal: Controller<SimplePlugin>,
    run_in_shell: Controller<SimplePlugin>,
    calculator: Controller<SimplePlugin>,
    file_path: Controller<SimplePlugin>,
    web_search: Controller<SimplePlugin>,
    actions: Controller<SimplePlugin>,
}

#[derive(Debug)]
pub enum LauncherPluginsInput {
    Set(crate::Plugins),
    SetPrev(crate::Plugins),
    Reset,
}

#[derive(Debug)]
pub struct LauncherPluginsInit {
    pub config: crate::Plugins,
}

#[derive(Debug)]
pub enum LauncherPluginsOutput {
    Applications(ApplicationsOutput),
    Terminal(SimplePluginOutput),
    Shell(SimplePluginOutput),
    Calculator(SimplePluginOutput),
    FilePath(SimplePluginOutput),
}

#[relm4::component(pub)]
impl SimpleComponent for LauncherPlugins {
    type Init = LauncherPluginsInit;
    type Input = LauncherPluginsInput;
    type Output = LauncherPluginsOutput;

    view! {
        #[root]
        adw::ExpanderRow {
            set_title_selectable: true,
            set_show_enable_switch: false,
            set_hexpand: true,
            set_css_classes: &["enable-frame"],
            set_title: "Plugins",
            set_enable_expansion: true,
            set_expanded: true,
            add_row = &gtk::Box {
                set_margin_all: 10,
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 20,
                model.applications.widget(),
            },
            add_row = &gtk::Box {
                set_margin_all: 10,
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 20,
                model.run_in_terminal.widget(),
                model.run_in_shell.widget(),
            },
            add_row = &gtk::Box {
                set_margin_all: 10,
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 20,
                model.calculator.widget(),
                model.file_path.widget(),
            },
            add_row = &gtk::Box {
                set_margin_all: 10,
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 20,
                model.web_search.widget(),
                model.actions.widget(),
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let launcher_plugins = Applications::builder()
            .launch(ApplicationsInit {
                config: init.config.applications.clone(),
            })
            .forward(sender.output_sender(), LauncherPluginsOutput::Applications);
        let run_in_terminal = SimplePlugin::builder()
            .launch(SimplePluginInit {
                config: init.config.terminal.clone(),
                name: "Run in Terminal",
                description: "TODO",
                todo: false,
            })
            .forward(sender.output_sender(), LauncherPluginsOutput::Terminal);
        let run_in_shell = SimplePlugin::builder()
            .launch(SimplePluginInit {
                config: init.config.shell.clone(),
                name: "Run in Shell",
                description: "TODO",
                todo: false,
            })
            .forward(sender.output_sender(), LauncherPluginsOutput::Shell);
        let calculator = SimplePlugin::builder()
            .launch(SimplePluginInit {
                config: init.config.calc.clone(),
                name: "Calculator",
                description: "TODO",
                todo: false,
            })
            .forward(sender.output_sender(), LauncherPluginsOutput::Calculator);
        let file_path = SimplePlugin::builder()
            .launch(SimplePluginInit {
                config: init.config.path.clone(),
                name: "Open Filepath",
                description: "TODO",
                todo: false,
            })
            .forward(sender.output_sender(), LauncherPluginsOutput::FilePath);
        let web_search = SimplePlugin::builder()
            .launch(SimplePluginInit {
                config: init.config.path.clone(),
                name: "Websearch (TODO)",
                description: "TODO",
                todo: true,
            })
            .detach();
        let actions = SimplePlugin::builder()
            .launch(SimplePluginInit {
                config: init.config.path.clone(),
                name: "Actions (TODO)",
                description: "TODO",
                todo: true,
            })
            .detach();

        let model = Self {
            config: init.config.clone(),
            prev_config: init.config,
            applications: launcher_plugins,
            run_in_terminal,
            run_in_shell,
            calculator,
            file_path,
            web_search,
            actions,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        trace!("launcher_plugins::main::update: {message:?}");
        match message {
            LauncherPluginsInput::Set(config) => {
                self.config = config;
                self.applications
                    .emit(ApplicationsInput::Set(self.config.applications.clone()));
                self.run_in_terminal
                    .emit(SimplePluginInput::Set(self.config.terminal.clone()));
                self.run_in_shell
                    .emit(SimplePluginInput::Set(self.config.shell.clone()));
                self.calculator
                    .emit(SimplePluginInput::Set(self.config.calc.clone()));
                self.file_path
                    .emit(SimplePluginInput::Set(self.config.path.clone()));
            }
            LauncherPluginsInput::SetPrev(config) => {
                self.prev_config = config;
                self.applications.emit(ApplicationsInput::SetPrev(
                    self.prev_config.applications.clone(),
                ));
                self.run_in_terminal.emit(SimplePluginInput::SetPrev(
                    self.prev_config.terminal.clone(),
                ));
                self.run_in_shell
                    .emit(SimplePluginInput::SetPrev(self.prev_config.shell.clone()));
                self.calculator
                    .emit(SimplePluginInput::SetPrev(self.prev_config.calc.clone()));
                self.file_path
                    .emit(SimplePluginInput::SetPrev(self.prev_config.path.clone()));
            }
            LauncherPluginsInput::Reset => {
                self.config = self.prev_config.clone();
                self.applications.emit(ApplicationsInput::Reset);
                self.run_in_terminal.emit(SimplePluginInput::Reset);
                self.run_in_shell.emit(SimplePluginInput::Reset);
                self.calculator.emit(SimplePluginInput::Reset);
                self.file_path.emit(SimplePluginInput::Reset);
            }
        }
    }
}
