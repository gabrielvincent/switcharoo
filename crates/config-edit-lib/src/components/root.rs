use crate::components::changes::{
    Changes, ChangesInit, ChangesInput, ChangesOutput, generate_items,
};
use crate::components::generate::{Generate, GenerateInit, GenerateInput, GenerateOutput};
use crate::components::launcher::{Launcher, LauncherInit, LauncherInput, LauncherOutput};
use crate::components::launcher_plugins::LauncherPluginsOutput;
use crate::components::launcher_plugins::applications::ApplicationsOutput;
use crate::components::launcher_plugins::simple::SimplePluginOutput;
use crate::components::nix_preview::{NixPreview, NixPreviewInit};
use crate::components::style::{Style, StyleInit, StyleInput, StyleOutput};
use crate::components::switch::SwitchOutput;
use crate::components::windows::{Windows, WindowsInit, WindowsInput, WindowsOutput};
use crate::components::windows_overview::WindowsOverviewOutput;
use crate::footer::{Footer, FooterInput, FooterOutput};
use crate::structs;
use crate::util::default_config;
use adw::StyleManager;
use relm4::ComponentController;
use relm4::abstractions::Toaster;
use relm4::adw;
use relm4::adw::gtk;
use relm4::adw::gtk::{Align, SelectionMode};
use relm4::adw::prelude::*;
use relm4::gtk::glib;
use relm4::{Component, ComponentParts, ComponentSender, Controller, SimpleComponent};
use relm4_components::alert::{Alert, AlertMsg, AlertResponse, AlertSettings};
use std::path::Path;
use tracing::{debug, error, info, trace, warn};

#[derive(Debug)]
pub enum Msg {
    Ignore,
    Reload(bool),
    CloseRequest,
    Close,
    Save(bool),
    Regenerate,
    AbortGenerate,

    Reset,
    SetConfig(crate::Config),
    SetPrevConfig(crate::Config),

    Windows(WindowsOutput),
    Launcher(LauncherOutput),
    Style(StyleOutput),
    Changes(ChangesOutput),
    Generate(GenerateOutput),
}

pub struct Root {
    config_path: Box<Path>,
    css_path: Box<Path>,

    is_generate_mode: bool,

    config: crate::Config,
    prev_config: crate::Config,

    footer: Controller<Footer>,
    alert_dialog: Controller<Alert>,
    launcher: Controller<Launcher>,
    windows: Controller<Windows>,
    changes: Controller<Changes>,
    nix_preview: Controller<NixPreview>,
    style: Controller<Style>,
    generate: Controller<Generate>,

    view_stack: adw::ViewStack,
    alert_dialog_changes_list: gtk::ListBox,
    toaster: Toaster,
}

pub struct InitRoot {
    pub config_path: Box<Path>,
    pub system_data_dir: Box<Path>,
    pub css_path: Box<Path>,
}

#[relm4::component(pub)]
impl SimpleComponent for Root {
    type Input = Msg;
    type Output = ();
    type Init = InitRoot;

    view! {
        #[root]
        adw::ApplicationWindow {
            set_title: Some("Hyprshell Config Editor"),
            set_default_width: 900,
            set_default_height: 700,
            set_resizable: true,
            #[wrap(Some)]
            #[local_ref]
            set_content = toast_overlay -> adw::ToastOverlay {
                set_vexpand: true,
                adw::ToolbarView {
                    set_top_bar_style: adw::ToolbarStyle::Raised,
                    set_bottom_bar_style: adw::ToolbarStyle::Flat,
                    set_reveal_bottom_bars: true,
                    set_reveal_top_bars: true,
                    #[local_ref]
                    add_bottom_bar = footer -> gtk::ActionBar {},
                    #[wrap(Some)]
                    set_content = &adw::Clamp {
                        set_maximum_size: 1400,
                        gtk::ScrolledWindow {
                            #[local_ref]
                            view_stack -> adw::ViewStack {
                                set_enable_transitions: true,
                            }
                        }
                    },
                    add_top_bar = &adw::HeaderBar {
                        set_show_end_title_buttons: true,
                        set_show_start_title_buttons: true,
                        set_show_back_button: true,
                        #[wrap(Some)]
                        set_title_widget: view_stack_switcher = &adw::ViewSwitcherBar {
                            #[watch]
                            set_visible: !model.is_generate_mode,
                            set_reveal: true,
                        },
                        pack_start = &gtk::Button {
                            set_label: "Generate new",
                            set_css_classes: &["pill", "warning"],
                            #[watch]
                            set_visible: !model.is_generate_mode,
                            connect_clicked[sender] => move |_| {
                                sender.input(Msg::Regenerate)
                            }
                        }
                    }
                },
            },
            connect_close_request[sender] => move |_| {
                sender.input(Msg::CloseRequest);
                glib::Propagation::Stop
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let config = default_config();
        let config = structs::Config::from(config);

        let footer: Controller<Footer> = Footer::builder()
            .launch(init.config_path.clone())
            .forward(sender.input_sender(), |msg| match msg {
                FooterOutput::Reset => Msg::Reset,
                FooterOutput::Close => Msg::CloseRequest,
                FooterOutput::Save => Msg::Save(false),
                FooterOutput::Reload => Msg::Reload(false),
                FooterOutput::Abort => Msg::AbortGenerate,
            });

        let changes_list = gtk::ListBox::builder()
            .css_classes(["items-list", "boxed-list"])
            .selection_mode(SelectionMode::None)
            .show_separators(false)
            .halign(Align::Center)
            .valign(Align::Start)
            .hexpand(true)
            .build();
        let alert_dialog = Alert::builder()
            .transient_for(&root)
            .launch(AlertSettings {
                text: Some("Do you want to close before saving?".to_string()),
                secondary_text: None,
                // secondary_text: Some(String::from("All unsaved changes will be lost")),
                confirm_label: Some(String::from("Close without saving")),
                cancel_label: Some(String::from("Cancel")),
                option_label: Some(String::from("Save and quit")),
                is_modal: true,
                destructive_accept: true,
                extra_child: Some(changes_list.clone().into()),
            })
            .forward(sender.input_sender(), |res| match res {
                AlertResponse::Confirm => Msg::Close,
                AlertResponse::Option => Msg::Save(true),
                AlertResponse::Cancel => Msg::Ignore,
            });
        let style = Style::builder()
            .launch(StyleInit {
                system_data_dir: init.system_data_dir.clone(),
                css_path: init.css_path.clone(),
            })
            .forward(sender.input_sender(), Msg::Style);
        let windows = Windows::builder()
            .launch(WindowsInit {
                config: config.windows.clone(),
            })
            .forward(sender.input_sender(), Msg::Windows);
        let launcher = Launcher::builder()
            .launch(LauncherInit {
                config: config.windows.overview.launcher.clone(),
            })
            .forward(sender.input_sender(), Msg::Launcher);
        let nix_preview = NixPreview::builder().launch(NixPreviewInit {}).detach();
        let changes = Changes::builder()
            .launch(ChangesInit {
                config: config.clone(),
            })
            .forward(sender.input_sender(), Msg::Changes);
        let generate = Generate::builder()
            .launch(GenerateInit {
                system_data_dir: init.system_data_dir,
            })
            .forward(sender.input_sender(), Msg::Generate);

        let view_stack = adw::ViewStack::builder().build();
        let toaster = Toaster::default();
        let model = Root {
            config_path: init.config_path,
            css_path: init.css_path.clone(),
            config: config.clone(),
            prev_config: config.clone(),
            is_generate_mode: false,
            generate,
            footer,
            windows,
            launcher,
            changes,
            style,
            nix_preview,
            alert_dialog,
            toaster,
            alert_dialog_changes_list: changes_list,
            view_stack,
        };
        let toast_overlay = model.toaster.overlay_widget();
        let footer = model.footer.widget();
        let view_stack = &model.view_stack;
        let widgets = view_output!();

        model.view_stack.add_titled_with_icon(
            model.style.widget(),
            Some("style"),
            "Style",
            "viewimage",
        );
        model.view_stack.add_titled_with_icon(
            model.changes.widget(),
            None,
            "Changes",
            "document-edit-symbolic",
        );
        model.view_stack.add_titled_with_icon(
            model.nix_preview.widget(),
            None,
            "Nix Preview",
            "preview",
        );
        model.view_stack.add_titled_with_icon(
            model.windows.widget(),
            Some("overview"),
            "Windows",
            "configure",
        );
        if config.windows.overview.enabled {
            model.view_stack.add_titled_with_icon(
                model.launcher.widget(),
                Some("launcher"),
                "Launcher",
                "configure",
            );
        }
        model.view_stack.set_visible_child_name("overview");
        widgets
            .view_stack_switcher
            .set_stack(Some(&model.view_stack));
        sender.input(Msg::Reload(true));
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        trace!("root::update: {message:?}");
        match message {
            Msg::Ignore => (),
            Msg::CloseRequest => {
                debug!("close request");
                let changes = generate_items(
                    &self.alert_dialog_changes_list,
                    &self.config,
                    &self.prev_config,
                );
                if changes {
                    self.alert_dialog.emit(AlertMsg::Show);
                    // self.alert_dialog.widgets().gtk_window_12.set_modal(true);
                } else {
                    sender.input(Msg::Close);
                }
            }
            Msg::Close => {
                relm4::main_application().quit();
            }
            Msg::Regenerate => {
                // TODO check for changes. dont check for changes if window is being closed
                self.is_generate_mode = true;
                self.footer.emit(FooterInput::GenerateMode(true));
                self.generate.emit(GenerateInput::Start);
                trace!("Adding generate tab");
                if self.view_stack.child_by_name("generate").is_none() {
                    self.view_stack.add_titled_with_icon(
                        self.generate.widget(),
                        Some("generate"),
                        "Generate",
                        "configure",
                    );
                } else {
                    trace!("Generate tab already exists");
                }
                self.view_stack.set_visible_child_name("generate");
            }
            Msg::AbortGenerate => {
                self.is_generate_mode = false;
                self.footer.emit(FooterInput::GenerateMode(false));
                if let Some(ch) = self.view_stack.child_by_name("generate") {
                    self.view_stack.remove(&ch);
                }
                self.view_stack.set_visible_child_name("overview");
            }
            Msg::Generate(msg) => match msg {
                GenerateOutput::Finish(out) => {
                    self.is_generate_mode = false;
                    self.footer.emit(FooterInput::GenerateMode(false));
                    if let Some(ch) = self.view_stack.child_by_name("generate") {
                        self.view_stack.remove(&ch);
                    }
                    self.view_stack.set_visible_child_name("overview");
                    dbg!(out);
                    // TODO out
                }
            },
            Msg::Reload(initial) => {
                if !self.config_path.exists() {
                    if initial {
                        warn!("Config file doesnt exist");
                        sender.input(Msg::Regenerate)
                    } else {
                        warn!("Config file was deleted");
                        let button = adw::Toast::builder()
                            .title("Config file missing")
                            .button_label("Generate new")
                            .timeout(0)
                            .build();
                        let s = sender.clone();
                        button.connect_button_clicked(move |_| {
                            s.input(Msg::Regenerate);
                        });
                        self.toaster.add_toast(button);

                        let config = default_config();
                        let config = structs::Config::from(config);
                        sender.input(Msg::SetConfig(config.clone()));
                        sender.input(Msg::SetPrevConfig(config.clone()));
                    }
                } else {
                    match config_lib::load_and_migrate_config(&self.config_path, true) {
                        Ok(c) => {
                            let config = structs::Config::from(c);
                            sender.input(Msg::SetConfig(config.clone()));
                            sender.input(Msg::SetPrevConfig(config.clone()));
                        }
                        Err(err) => {
                            warn!("Failed to load config: {err:#}");
                            self.toaster.add_toast(
                                adw::Toast::builder()
                                    .title(err.to_string())
                                    .timeout(0)
                                    .build(),
                            );
                        }
                    }
                }
            }
            Msg::SetConfig(config) => {
                self.config = config;

                if self.config.windows.overview.enabled {
                    trace!("Adding launcher tab");
                    if self.view_stack.child_by_name("launcher").is_none() {
                        self.view_stack.add_titled_with_icon(
                            self.launcher.widget(),
                            Some("launcher"),
                            "Launcher",
                            "configure",
                        );
                    } else {
                        trace!("Launcher tab already exists");
                    }
                } else {
                    if let Some(ch) = self.view_stack.child_by_name("launcher") {
                        self.view_stack.remove(&ch);
                    }
                }

                self.windows
                    .emit(WindowsInput::SetWindows(self.config.windows.clone()));
                self.launcher.emit(LauncherInput::SetLauncher(
                    self.config.windows.overview.launcher.clone(),
                ));
                self.changes
                    .emit(ChangesInput::SetConfig(self.config.clone()));
            }
            Msg::SetPrevConfig(config) => {
                self.prev_config = config;

                self.windows.emit(WindowsInput::SetPrevWindows(
                    self.prev_config.windows.clone(),
                ));
                self.launcher.emit(LauncherInput::SetPrevLauncher(
                    self.prev_config.windows.overview.launcher.clone(),
                ));
                self.changes
                    .emit(ChangesInput::SetPrevConfig(self.prev_config.clone()));
            }
            Msg::Save(close) => {
                match config_lib::write_config(
                    &self.config_path,
                    &(self.config.clone().into()),
                    true,
                ) {
                    Ok(_) => {
                        info!("Saved config to {}", self.config_path.display());
                        self.toaster.add_toast(
                            adw::Toast::builder()
                                .title("Saved".to_string())
                                .timeout(2)
                                .build(),
                        );
                    }
                    Err(err) => {
                        error!("Failed to save config: {err:#}");
                        self.toaster.add_toast(
                            adw::Toast::builder()
                                .title(err.to_string())
                                .timeout(0)
                                .build(),
                        );
                    }
                }

                if close {
                    sender.input(Msg::Close);
                }
                sender.input(Msg::SetPrevConfig(self.config.clone()));
            }
            Msg::Reset => {
                self.config = self.prev_config.clone();

                self.windows.emit(WindowsInput::ResetWindows);
                self.launcher.emit(LauncherInput::ResetLauncher);
                self.changes
                    .emit(ChangesInput::SetConfig(self.config.clone()));
            }
            Msg::Changes(msg) => match msg {
                ChangesOutput::ChangesExist(changes_exist) => {
                    self.footer.emit(FooterInput::ChangesExist(changes_exist))
                }
            },
            Msg::Style(msg) => match msg {
                StyleOutput::Apply((name, content)) => {
                    match std::fs::write(&self.css_path, content) {
                        Ok(_) => {
                            info!("Saved css from {name} to {}", self.css_path.display());
                            self.toaster.add_toast(
                                adw::Toast::builder()
                                    .title("Saved".to_string())
                                    .timeout(2)
                                    .build(),
                            );
                        }
                        Err(err) => {
                            error!("Failed to save css from {name}: {err:#}");
                            self.toaster.add_toast(
                                adw::Toast::builder()
                                    .title(err.to_string())
                                    .timeout(0)
                                    .build(),
                            );
                        }
                    }
                    self.style.emit(StyleInput::Reload);
                }
            },
            Msg::Launcher(msg) => {
                let r#ref = &mut self.config.windows.overview.launcher;
                match msg {
                    LauncherOutput::Modifier(modifier) => {
                        r#ref.launch_modifier = modifier;
                    }
                    LauncherOutput::Width(width) => {
                        r#ref.width = width;
                    }
                    LauncherOutput::MaxItems(max_items) => {
                        r#ref.max_items = max_items;
                    }
                    LauncherOutput::DefaultTerminal(default_terminal) => match default_terminal {
                        None => {
                            r#ref.default_terminal = None;
                        }
                        Some(val) => {
                            r#ref.default_terminal = Some(val);
                        }
                    },
                    LauncherOutput::LauncherPlugins(msg) => match msg {
                        LauncherPluginsOutput::Applications(msg) => match msg {
                            ApplicationsOutput::Enabled(enabled) => {
                                r#ref.plugins.applications.enabled = enabled;
                            }
                            ApplicationsOutput::ShowExecs(enabled) => {
                                r#ref.plugins.applications.show_execs = enabled;
                            }
                            ApplicationsOutput::ShowActions(enabled) => {
                                r#ref.plugins.applications.show_actions_submenu = enabled;
                            }
                            ApplicationsOutput::CacheWeeks(weeks) => {
                                r#ref.plugins.applications.run_cache_weeks = weeks;
                            }
                        },
                        LauncherPluginsOutput::Terminal(msg) => match msg {
                            SimplePluginOutput::Enabled(enabled) => {
                                r#ref.plugins.terminal.enabled = enabled;
                            }
                        },
                        LauncherPluginsOutput::Shell(msg) => match msg {
                            SimplePluginOutput::Enabled(enabled) => {
                                r#ref.plugins.shell.enabled = enabled;
                            }
                        },
                        LauncherPluginsOutput::Calculator(msg) => match msg {
                            SimplePluginOutput::Enabled(enabled) => {
                                r#ref.plugins.calc.enabled = enabled;
                            }
                        },
                        LauncherPluginsOutput::FilePath(msg) => match msg {
                            SimplePluginOutput::Enabled(enabled) => {
                                r#ref.plugins.path.enabled = enabled;
                            }
                        },
                    },
                }
                // propagate event back
                sender.input(Msg::SetConfig(self.config.clone()))
            }
            Msg::Windows(msg) => {
                let r#ref = &mut self.config.windows;
                match msg {
                    WindowsOutput::Enabled(enabled) => {
                        r#ref.enabled = enabled;
                    }
                    WindowsOutput::Scale(scale) => {
                        r#ref.scale = scale;
                    }
                    WindowsOutput::ItemsPerRow(items_per_row) => {
                        r#ref.items_per_row = items_per_row;
                    }
                    WindowsOutput::Overview(msg) => match msg {
                        WindowsOverviewOutput::Enabled(enabled) => {
                            r#ref.overview.enabled = enabled;
                        }
                        WindowsOverviewOutput::Key(key) => r#ref.overview.key = key,
                        WindowsOverviewOutput::Modifier(modifier) => {
                            r#ref.overview.modifier = modifier;
                        }
                        WindowsOverviewOutput::FilterSameClass(enabled) => {
                            r#ref.overview.same_class = enabled;
                        }
                        WindowsOverviewOutput::FilterWorkspace(enabled) => {
                            r#ref.overview.current_workspace = enabled;
                            // current monitor and current workspace are incompatible
                            if enabled {
                                r#ref.overview.current_monitor = false;
                            }
                        }
                        WindowsOverviewOutput::FilterMonitor(enabled) => {
                            r#ref.overview.current_monitor = enabled;
                            // current monitor and current workspace are incompatible
                            if enabled {
                                r#ref.overview.current_workspace = false;
                            }
                        }
                        WindowsOverviewOutput::ExcludeSpecialWorkspaces(
                            exclude_special_workspaces,
                        ) => {
                            r#ref.overview.exclude_special_workspaces = exclude_special_workspaces;
                        }
                    },
                    WindowsOutput::Switch(msg) => match msg {
                        SwitchOutput::Enabled(enabled) => {
                            r#ref.switch.enabled = enabled;
                        }
                        SwitchOutput::Key(key) => r#ref.switch.key = key,
                        SwitchOutput::Modifier(modifier) => {
                            r#ref.switch.modifier = modifier;
                        }
                        SwitchOutput::FilterSameClass(enabled) => {
                            r#ref.switch.same_class = enabled;
                        }
                        SwitchOutput::FilterWorkspace(enabled) => {
                            r#ref.switch.current_workspace = enabled;
                            // current monitor and current workspace are incompatible
                            if enabled {
                                r#ref.switch.current_monitor = false;
                            }
                        }
                        SwitchOutput::FilterMonitor(enabled) => {
                            r#ref.switch.current_monitor = enabled;
                            // current monitor and current workspace are incompatible
                            if enabled {
                                r#ref.switch.current_workspace = false;
                            }
                        }
                        SwitchOutput::SwitchWorkspaces(enabled) => {
                            r#ref.switch.switch_workspaces = enabled;
                        }
                        SwitchOutput::ExcludeSpecialWorkspaces(exclude_special_workspaces) => {
                            r#ref.switch.exclude_special_workspaces = exclude_special_workspaces;
                        }
                    },
                    WindowsOutput::Switch2(msg) => match msg {
                        SwitchOutput::Enabled(enabled) => {
                            r#ref.switch_2.enabled = enabled;
                        }
                        SwitchOutput::Key(key) => r#ref.switch_2.key = key,
                        SwitchOutput::Modifier(modifier) => {
                            r#ref.switch_2.modifier = modifier;
                        }
                        SwitchOutput::FilterSameClass(enabled) => {
                            r#ref.switch_2.same_class = enabled;
                        }
                        SwitchOutput::FilterWorkspace(enabled) => {
                            r#ref.switch_2.current_workspace = enabled;
                            // current monitor and current workspace are incompatible
                            if enabled {
                                r#ref.switch_2.current_monitor = false;
                            }
                        }
                        SwitchOutput::FilterMonitor(enabled) => {
                            r#ref.switch_2.current_monitor = enabled;
                            // current monitor and current workspace are incompatible
                            if enabled {
                                r#ref.switch_2.current_workspace = false;
                            }
                        }
                        SwitchOutput::SwitchWorkspaces(enabled) => {
                            r#ref.switch_2.switch_workspaces = enabled;
                        }
                        SwitchOutput::ExcludeSpecialWorkspaces(exclude_special_workspaces) => {
                            r#ref.switch_2.exclude_special_workspaces = exclude_special_workspaces;
                        }
                    },
                };
                // propagate event back
                sender.input(Msg::SetConfig(self.config.clone()))
            }
        }
    }
}
