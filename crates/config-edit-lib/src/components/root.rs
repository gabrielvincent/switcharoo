use crate::components::changes::{
    Changes, ChangesInit, ChangesInput, ChangesOutput, generate_items,
};
use crate::components::json_preview::{JSONPreview, JSONPreviewInit};
use crate::components::launcher::{Launcher, LauncherInit, LauncherInput, LauncherOutput};
use crate::components::style::{Style, StyleInit, StyleInput, StyleOutput};
use crate::components::switch::SwitchOutput;
use crate::components::windows::{Windows, WindowsInit, WindowsInput, WindowsOutput};
use crate::components::windows_overview::WindowsOverviewOutput;
use crate::footer::{Footer, FooterInput, FooterOutput};
use crate::structs;
use crate::util::default_config;
use adw::AlertDialog;
use relm4::ComponentController;
use relm4::adw;
use relm4::adw::gtk;
use relm4::adw::gtk::{Align, ListBox, SelectionMode};
use relm4::adw::prelude::*;
use relm4::gtk::glib;
use relm4::{Component, ComponentParts, ComponentSender, Controller, SimpleComponent};
use relm4_components::alert::{Alert, AlertMsg, AlertResponse, AlertSettings};
use std::path::Path;
use tracing::{debug, info, trace, warn};

#[derive(Debug)]
pub enum Msg {
    Reset,
    CloseRequest,
    Close,
    Ignore,
    Save(bool),
    SetConfig(crate::Config),
    Windows(WindowsOutput),
    Launcher(LauncherOutput),
    Style(StyleOutput),
    Changes(ChangesOutput),
}

pub struct Root {
    config: crate::Config,
    prev_config: crate::Config,
    footer: Controller<Footer>,
    alert_dialog: Controller<Alert>,
    launcher: Controller<Launcher>,
    windows: Controller<Windows>,
    view_stack: adw::ViewStack,
    changes: Controller<Changes>,
    alert_dialog_changes_list: ListBox,
    pub style: Controller<Style>,
}

pub struct InitRoot {
    pub config_path: Box<Path>,
}

struct AppWidgets {
    label: gtk::Label,
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
            set_content = &adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Raised,
                set_bottom_bar_style: adw::ToolbarStyle::Flat,
                set_reveal_bottom_bars: true,
                set_reveal_top_bars: true,
                add_bottom_bar = footer.widget(),
                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    #[name = "view_stack"]
                    adw::ViewStack {
                    }
                },
                add_top_bar = &adw::HeaderBar {
                    set_show_end_title_buttons: true,
                    set_show_start_title_buttons: true,
                    set_show_back_button: true,
                    #[wrap(Some)]
                    set_title_widget: view_stack_switcher = &adw::ViewSwitcherBar {
                        set_reveal: true,
                    },
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
        let config = if !init.config_path.exists() {
            warn!("Config file does not exist, create it using `hyprshell config generate`");
            let dialog = AlertDialog::builder()
                .heading("Config doesnt exist")
                .body("create it using `hyprshell config generate`")
                .close_response("close")
                .build();
            dialog.add_responses(&[("close", "Close")]);
            glib::spawn_future_local(async move {
                let res = dialog
                    .choose_future(relm4::main_application().windows().first())
                    .await;
                debug!("Dialog closed: {res:?}");
                relm4::main_application().quit();
            });
            default_config()
        } else {
            match config_lib::load_and_migrate_config(&init.config_path, true) {
                Ok(c) => c,
                Err(err) => {
                    warn!("Failed to load config: {err:?}");
                    let dialog = AlertDialog::builder()
                        .heading("Failed to load config")
                        .body(format!("{err:#}"))
                        .close_response("close")
                        .build();
                    dialog.add_responses(&[("close", "Close")]);
                    glib::spawn_future_local(async move {
                        let res = dialog
                            .choose_future(relm4::main_application().windows().first())
                            .await;
                        debug!("Dialog closed: {res:?}");
                        relm4::main_application().quit();
                    });
                    default_config()
                }
            }
        };
        let config = structs::Config::from(config);

        let footer: Controller<Footer> =
            Footer::builder()
                .launch(init.config_path)
                .forward(sender.input_sender(), |msg| match msg {
                    FooterOutput::Reset => Msg::Reset,
                    FooterOutput::Close => Msg::CloseRequest,
                    FooterOutput::Save => Msg::Save(false),
                });

        let changes_list = ListBox::builder()
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
            .launch(StyleInit {})
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
        let json_preview = JSONPreview::builder().launch(JSONPreviewInit {}).detach();
        let changes = Changes::builder()
            .launch(ChangesInit {
                config: config.clone(),
            })
            .forward(sender.input_sender(), Msg::Changes);

        let widgets = view_output!();
        widgets.view_stack.add_titled_with_icon(
            style.widget(),
            Some("style"),
            "Style",
            "viewimage",
        );
        widgets.view_stack.add_titled_with_icon(
            changes.widget(),
            None,
            "Changes",
            "document-edit-symbolic",
        );
        widgets.view_stack.add_titled_with_icon(
            json_preview.widget(),
            None,
            "Json Preview",
            "preview",
        );
        widgets.view_stack.add_titled_with_icon(
            windows.widget(),
            Some("overview"),
            "Windows",
            "configure",
        );
        if config.windows.overview.enabled {
            widgets.view_stack.add_titled_with_icon(
                launcher.widget(),
                Some("launcher"),
                "Launcher",
                "configure",
            );
        }
        widgets
            .view_stack_switcher
            .set_stack(Some(&widgets.view_stack));
        widgets.view_stack.set_visible_child_name("overview");

        let model = Root {
            config: config.clone(),
            prev_config: config.clone(),
            footer,
            windows,
            launcher,
            changes,
            style,
            alert_dialog,
            alert_dialog_changes_list: changes_list,
            view_stack: widgets.view_stack.clone(),
        };
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        trace!("update: {msg:?}");
        match msg {
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
            Msg::SetConfig(config) => {
                self.config = config;

                self.windows
                    .emit(WindowsInput::SetWindows(self.config.windows.clone()));
                self.launcher.emit(LauncherInput::SetLauncher(
                    self.config.windows.overview.launcher.clone(),
                ));
                self.changes
                    .emit(ChangesInput::SetConfig(self.config.clone()));
            }
            Msg::Save(close) => {
                info!("save");
                // TODO
                if close {
                    sender.input(Msg::Close);
                }
                self.prev_config = self.config.clone();
                self.windows.emit(WindowsInput::SetPrevWindows(
                    self.prev_config.windows.clone(),
                ));
                self.launcher.emit(LauncherInput::SetPrevLauncher(
                    self.prev_config.windows.overview.launcher.clone(),
                ));
                self.changes
                    .emit(ChangesInput::SetPrevConfig(self.prev_config.clone()));
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
            Msg::Style(msg) => {}
            Msg::Launcher(msg) => {
                let mut r#ref = &mut self.config.windows.overview.launcher;
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
                }
                // propagate event back
                self.launcher
                    .emit(LauncherInput::SetLauncher(r#ref.clone()));
                self.changes
                    .emit(ChangesInput::SetConfig(self.config.clone()));
            }
            Msg::Windows(msg) => {
                let mut r#ref = &mut self.config.windows;
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
                            if enabled {
                                trace!("Adding launcher tab");
                                if self.view_stack.child_by_name("launcher").is_none() {
                                    self.view_stack.add_titled_with_icon(
                                        self.launcher.widget(),
                                        Some("launcher"),
                                        "Launcher",
                                        "configure",
                                    );
                                } else {
                                    warn!("Launcher tab already exists");
                                }
                            } else {
                                self.view_stack.child_by_name("launcher");
                            }
                            self.launcher
                                .emit(LauncherInput::SetLauncher(r#ref.overview.launcher.clone()));
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
                self.windows.emit(WindowsInput::SetWindows(r#ref.clone()));
                self.changes
                    .emit(ChangesInput::SetConfig(self.config.clone()));
            }
        }
    }
}
