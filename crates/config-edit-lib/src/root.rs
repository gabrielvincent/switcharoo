use crate::components::json_preview::JsonPreview;
use crate::footer::{Footer, FooterOutput};
use adw::gtk::{Align, ListBox, SelectionMode};
use adw::prelude::*;
use adw::{ActionRow, gtk};
use config_lib::{
    ActionsPluginAction, ActionsPluginConfig, ApplicationsPluginConfig, Config, EmptyConfig,
    Modifier, Overview, SearchEngine, Switch, WebSearchConfig, Windows,
};
use relm4::ComponentController;
use relm4::component::Connector;
use relm4::{Component, ComponentParts, ComponentSender, Controller, SimpleComponent};
use relm4_components::alert::{Alert, AlertMsg, AlertResponse, AlertSettings};
use std::path::Path;
use tracing::info;

#[derive(Debug)]
pub enum Msg {
    CloseRequest,
    Close,
    Ignore,
    Save,
    SaveAndClose,
    SetWindows(Option<Windows>),
    SetWindowsScale(f64),
    SetWindowsItemsPerRow(u8),
    SetWindowsOverview(Option<Overview>),
    SetWindowsOverviewKey(Box<str>),
    SetWindowsOverviewModifier(Modifier),
    SetWindowsOverviewFilterSameClass(bool),
    SetWindowsOverviewFilterWorkspace(bool),
    SetWindowsOverviewFilterMonitor(bool),
    SetWindowsOverviewLauncherDefaultTerminal(Option<Box<str>>),
    SetWindowsOverviewLauncherModifier(Modifier),
    SetWindowsOverviewLauncherWidth(u32),
    SetWindowsOverviewLauncherMaxItems(u8),
    SetWindowsOverviewLauncherShowWhenEmpty(bool),
    SetWindowsOverviewLauncherPluginsApplicationsOption(ApplicationsPluginConfig),
    SetWindowsOverviewLauncherPluginsApplicationsRunCacheWeeks(u8),
    SetWindowsOverviewLauncherPluginsApplicationsShowExecs(bool),
    SetWindowsOverviewLauncherPluginsApplicationsShowActionsSubmenu(bool),
    SetWindowsOverviewLauncherPluginsTerminal(Option<EmptyConfig>),
    SetWindowsOverviewLauncherPluginsShell(Option<EmptyConfig>),
    SetWindowsOverviewLauncherPluginsWebSearch(Option<WebSearchConfig>),
    SetWindowsOverviewLauncherPluginsWebSearchAdd(SearchEngine),
    SetWindowsOverviewLauncherPluginsWebSearchRemove(SearchEngine),
    SetWindowsOverviewLauncherPluginsCalc(Option<EmptyConfig>),
    SetWindowsOverviewLauncherPluginsPath(Option<EmptyConfig>),
    SetWindowsOverviewLauncherPluginsActions(Option<ActionsPluginConfig>),
    SetWindowsOverviewLauncherPluginsActionsAdd(ActionsPluginAction),
    SetWindowsOverviewLauncherPluginsActionsRemove(ActionsPluginAction),
    SetWindowsSwitch(Option<Switch>),
    SetWindowsSwitchModifier(Modifier),
    SetWindowsSwitchFilterSameClass(bool),
    SetWindowsSwitchFilterWorkspace(bool),
    SetWindowsSwitchFilterMonitor(bool),
    SetWindowsSwitchSwitchWorkspaces(bool),
}

pub struct Root {
    config: Config,
    prev_config: Config,
    footer: Controller<Footer>,
    alert_dialog: Controller<Alert>,
}

pub struct InitRoot {
    pub config_path: Box<Path>,
    pub config: Config,
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
                add_top_bar = &adw::HeaderBar {
                    set_show_end_title_buttons: true,
                    set_show_start_title_buttons: true,
                    set_show_back_button: true,
                },
                add_bottom_bar = model.footer.widget(),
                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    #[name = "view_stack"]
                    adw::ViewStack {
                    }
                }
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
                    FooterOutput::Close => Msg::CloseRequest,
                });

        let json_preview = JsonPreview::builder().launch(()).detach();

        let list = ListBox::builder()
            .css_classes(["items-list", "boxed-list"])
            .selection_mode(SelectionMode::None)
            .show_separators(false)
            .halign(Align::Center)
            .valign(Align::Start)
            .hexpand(true)
            .build();
        list.append(&ActionRow::builder().title("TODO add changes").build());
        list.append(&ActionRow::builder().title("TODO add changes").build());

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
                AlertResponse::Option => Msg::SaveAndClose,
                AlertResponse::Cancel => Msg::Ignore,
            });

        let model = Root {
            config: init.config.clone(),
            prev_config: init.config,
            footer,
            alert_dialog,
        };
        let widgets = view_output!();

        widgets.view_stack.add_titled_with_icon(
            json_preview.widget(),
            None,
            "Json Preview",
            "preview",
        );
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::CloseRequest => {
                self.alert_dialog.emit(AlertMsg::Show);
            }
            Msg::Close => {
                relm4::main_application().quit();
            }
            Msg::Ignore => (),
            Msg::SaveAndClose => {
                info!("save");
                relm4::main_application().quit();
            }
            _ => {}
        }
    }
}
