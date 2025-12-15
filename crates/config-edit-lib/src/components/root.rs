use crate::components::changes::{Changes, ChangesInit, ChangesInput, generate_items};
use crate::components::json_preview::{JSONPreview, JSONPreviewInit};
use crate::components::launcher::{Launcher, LauncherInit, LauncherInput, LauncherOutput};
use crate::components::switch::SwitchOutput;
use crate::components::windows::{Windows, WindowsInit, WindowsInput, WindowsOutput};
use crate::components::windows_overview::WindowsOverviewOutput;
use crate::footer::{Footer, FooterOutput};
use relm4::ComponentController;
use relm4::adw;
use relm4::adw::gtk;
use relm4::adw::gtk::{Align, ListBox, SelectionMode};
use relm4::adw::prelude::*;
use relm4::{Component, ComponentParts, ComponentSender, Controller, SimpleComponent};
use relm4_components::alert::{Alert, AlertMsg, AlertResponse, AlertSettings};
use std::path::Path;
use tracing::info;

#[derive(Debug)]
pub enum Msg {
    Reset,
    CloseRequest,
    Close,
    Ignore,
    Save(bool),
    LauncherTabVisible(bool),
    Windows(WindowsOutput),
    Launcher(LauncherOutput),
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
    pub alert_dialog_changes_list: ListBox,
}

pub struct InitRoot {
    pub config_path: Box<Path>,
    pub config: crate::Config,
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
                gtk::glib::Propagation::Stop
            }
        }
    }

    // Initialize the UI.
    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
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
                extra_child: Some((changes_list).clone().into()),
            })
            .forward(sender.input_sender(), |res| match res {
                AlertResponse::Confirm => Msg::Close,
                AlertResponse::Option => Msg::Save(true),
                AlertResponse::Cancel => Msg::Ignore,
            });

        let windows = Windows::builder()
            .launch(WindowsInit {
                config: init.config.windows.clone(),
            })
            .forward(sender.input_sender(), Msg::Windows);
        let launcher = Launcher::builder()
            .launch(LauncherInit {
                config: init.config.windows.overview.launcher.clone(),
            })
            .forward(sender.input_sender(), Msg::Launcher);

        let json_preview = JSONPreview::builder().launch(JSONPreviewInit {}).detach();
        let changes = Changes::builder()
            .launch(ChangesInit {
                config: init.config.clone(),
            })
            .detach();

        let widgets = view_output!();
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
        sender.input(Msg::LauncherTabVisible(
            init.config.windows.overview.enabled,
        ));
        widgets
            .view_stack_switcher
            .set_stack(Some(&widgets.view_stack));
        widgets.view_stack.set_visible_child_name("overview");

        let model = Root {
            config: init.config.clone(),
            prev_config: init.config,
            footer,
            windows,
            launcher,
            changes,
            alert_dialog,
            alert_dialog_changes_list: changes_list,
            view_stack: widgets.view_stack.clone(),
        };
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            Msg::Ignore => (),
            Msg::CloseRequest => {
                let changes = generate_items(
                    &self.alert_dialog_changes_list,
                    &self.config,
                    &self.prev_config,
                );
                if changes {
                    self.alert_dialog.emit(AlertMsg::Show);
                    self.alert_dialog.widgets().gtk_window_12.set_modal(true);
                } else {
                    sender.input(Msg::Close);
                }
            }
            Msg::Close => {
                relm4::main_application().quit();
            }
            Msg::Save(close) => {
                info!("save");
                dbg!(&self.config);
                if close {
                    sender.input(Msg::Close);
                }
                // TODO set prev config
            }
            Msg::Reset => {
                self.config = self.prev_config.clone();
                self.windows
                    .emit(WindowsInput::SetWindows(self.config.windows.clone()));
            }
            Msg::LauncherTabVisible(visible) => {
                if visible {
                    self.view_stack.add_titled_with_icon(
                        self.launcher.widget(),
                        Some("launcher"),
                        "Launcher",
                        "configure",
                    );
                } else {
                    self.view_stack.child_by_name("launcher");
                }
            }
            Msg::Launcher(msg) => {
                match msg {
                    LauncherOutput::Modifier(modifier) => {
                        self.config.windows.overview.launcher.launch_modifier = modifier;
                    }
                    LauncherOutput::Width(width) => {
                        self.config.windows.overview.launcher.width = width;
                    }
                    LauncherOutput::MaxItems(max_items) => {
                        self.config.windows.overview.launcher.max_items = max_items;
                    }
                    LauncherOutput::DefaultTerminal(default_terminal) => {
                        dbg!(&default_terminal);
                        match default_terminal {
                            None => {
                                self.config.windows.overview.launcher.default_terminal = None;
                            }
                            Some(val) => {
                                self.config.windows.overview.launcher.default_terminal = Some(val);
                            }
                        }
                    }
                }
                // propagate event back
                self.launcher.emit(LauncherInput::SetLauncher(
                    self.config.windows.overview.launcher.clone(),
                ));
            }
            Msg::Windows(msg) => {
                match msg {
                    WindowsOutput::Enabled(enabled) => {
                        self.config.windows.enabled = enabled;
                        sender.input(Msg::LauncherTabVisible(enabled));
                    }
                    WindowsOutput::Scale(scale) => {
                        self.config.windows.scale = scale;
                    }
                    WindowsOutput::ItemsPerRow(items_per_row) => {
                        self.config.windows.items_per_row = items_per_row;
                    }
                    WindowsOutput::Overview(msg) => match msg {
                        WindowsOverviewOutput::Enabled(enabled) => {
                            self.config.windows.overview.enabled = enabled;
                        }
                        WindowsOverviewOutput::Key(key) => self.config.windows.overview.key = key,
                        WindowsOverviewOutput::Modifier(modifier) => {
                            self.config.windows.overview.modifier = modifier;
                        }
                        WindowsOverviewOutput::FilterSameClass(enabled) => {
                            self.config.windows.overview.same_class = enabled;
                        }
                        WindowsOverviewOutput::FilterWorkspace(enabled) => {
                            self.config.windows.overview.current_workspace = enabled;
                        }
                        WindowsOverviewOutput::FilterMonitor(enabled) => {
                            self.config.windows.overview.current_monitor = enabled;
                        }
                    },
                    WindowsOutput::Switch(msg) => match msg {
                        SwitchOutput::Enabled(enabled) => {
                            self.config.windows.switch.enabled = enabled;
                        }
                        SwitchOutput::Key(key) => self.config.windows.switch.key = key,
                        SwitchOutput::Modifier(modifier) => {
                            self.config.windows.switch.modifier = modifier;
                        }
                        SwitchOutput::FilterSameClass(enabled) => {
                            self.config.windows.switch.same_class = enabled;
                        }
                        SwitchOutput::FilterWorkspace(enabled) => {
                            self.config.windows.switch.current_workspace = enabled;
                        }
                        SwitchOutput::FilterMonitor(enabled) => {
                            self.config.windows.switch.current_monitor = enabled;
                        }
                    },
                    WindowsOutput::Switch2(msg) => match msg {
                        SwitchOutput::Enabled(enabled) => {
                            self.config.windows.switch_2.enabled = enabled;
                        }
                        SwitchOutput::Key(key) => self.config.windows.switch_2.key = key,
                        SwitchOutput::Modifier(modifier) => {
                            self.config.windows.switch_2.modifier = modifier;
                        }
                        SwitchOutput::FilterSameClass(enabled) => {
                            self.config.windows.switch_2.same_class = enabled;
                        }
                        SwitchOutput::FilterWorkspace(enabled) => {
                            self.config.windows.switch_2.current_workspace = enabled;
                        }
                        SwitchOutput::FilterMonitor(enabled) => {
                            self.config.windows.switch_2.current_monitor = enabled;
                        }
                    },
                };
                // propagate event back
                self.windows
                    .emit(WindowsInput::SetWindows(self.config.windows.clone()));
            }
        }
        self.changes
            .emit(ChangesInput::SetConfig(self.config.clone()));
    }
}
