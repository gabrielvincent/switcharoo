use crate::components::json_preview::JsonPreview;
use crate::components::windows::{Windows, WindowsInit, WindowsInput, WindowsOutput};
use crate::components::windows_overview::WindowsOverviewOutput;
use crate::footer::{Footer, FooterOutput};
use relm4::ComponentController;
use relm4::adw;
use relm4::adw::gtk::{Align, ListBox, SelectionMode};
use relm4::adw::prelude::*;
use relm4::adw::{ActionRow, gtk};
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
    Windows(WindowsOutput),
}

pub struct Root {
    config: crate::Config,
    prev_config: crate::Config,
    footer: Controller<Footer>,
    alert_dialog: Controller<Alert>,
    pub windows: Controller<Windows>,
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
                add_bottom_bar = model.footer.widget(),
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

        let list = ListBox::builder()
            .css_classes(["items-list", "boxed-list"])
            .selection_mode(SelectionMode::None)
            .show_separators(false)
            .halign(Align::Center)
            .valign(Align::Start)
            .hexpand(true)
            .build();
        list.append(
            &ActionRow::builder()
                .title("TODO add changes")
                .focusable(false)
                .build(),
        );
        let alert_dialog = relm4_components::alert::Alert::builder()
            .transient_for(&root)
            .launch(AlertSettings {
                text: Some("Do you want to close before saving?".to_string()),
                secondary_text: Some(String::from("All unsaved changes will be lost")),
                confirm_label: Some(String::from("Close without saving")),
                cancel_label: Some(String::from("Cancel")),
                option_label: Some(String::from("Save")),
                is_modal: true,
                destructive_accept: true,
                extra_child: Some(list.into()),
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

        let model = Root {
            config: init.config.clone(),
            prev_config: init.config,
            footer,
            windows,
            alert_dialog,
        };

        let json_preview = JsonPreview::builder().launch(()).detach();

        let widgets = view_output!();
        widgets.view_stack.add_titled_with_icon(
            json_preview.widget(),
            None,
            "Json Preview",
            "preview",
        );
        widgets.view_stack.add_titled_with_icon(
            model.windows.widget(),
            Some("overview"),
            "Windows",
            "configure",
        );

        widgets
            .view_stack_switcher
            .set_stack(Some(&widgets.view_stack));

        widgets.view_stack.set_visible_child_name("overview");
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            Msg::Ignore => (),
            Msg::CloseRequest => {
                self.alert_dialog.emit(AlertMsg::Show);
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
            }
            Msg::Reset => {
                self.config = self.prev_config.clone();
                self.windows
                    .emit(WindowsInput::SetWindows(self.config.windows.clone()));
            }
            Msg::Windows(msg) => {
                match msg {
                    WindowsOutput::Enabled(enabled) => {
                        self.config.windows.enabled = enabled;
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
                };

                self.windows
                    .emit(WindowsInput::SetWindows(self.config.windows.clone()));
            }
        }
    }
}
