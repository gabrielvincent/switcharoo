use crate::flags_csv;
use crate::structs::{Config, Plugins};
use crate::util::key_to_name;
use relm4::adw::ActionRow;
use relm4::adw::gtk::SelectionMode;
use relm4::adw::prelude::*;
use relm4::gtk::ListBox;
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};
use relm4::{adw, gtk};
use tracing::trace;

#[derive(Debug)]
pub struct Changes {
    config: Config,
    prev_config: Config,
    list: ListBox,
}

#[derive(Debug)]
pub enum ChangesInput {
    SetConfig(Config),
    SetPrevConfig(Config),
}

#[derive(Debug)]
pub struct ChangesInit {
    pub config: Config,
}

#[derive(Debug)]
pub enum ChangesOutput {
    ChangesExist(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for Changes {
    type Init = ChangesInit;
    type Input = ChangesInput;
    type Output = ChangesOutput;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_margin_all: 10,
            #[name="list"]
            ListBox {
                set_show_separators: false,
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::Start,
                set_hexpand: true,
                set_selection_mode: SelectionMode::None,
                set_css_classes: &["items-list", "boxed-list"]
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        #[allow(unused_assignments)]
        let widgets = view_output!();
        let model = Changes {
            config: init.config.clone(),
            prev_config: init.config,
            list: widgets.list.clone(),
        };
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        trace!("changes::update: {message:?}");
        match message {
            ChangesInput::SetConfig(config) => {
                self.config = config;
            }
            ChangesInput::SetPrevConfig(config) => {
                self.prev_config = config;
            }
        }
        let changes = generate_items(
            &self.list,
            // TODO
            &self.config,
            &self.prev_config,
            // TODO
        );
        sender.output(ChangesOutput::ChangesExist(changes)).unwrap();
    }
}

pub fn generate_items(
    changes: &ListBox,
    // how_to_use: &TextView,
    config: &Config,
    prev_config: &Config,
    // path: &Path,
) -> bool {
    while let Some(child) = changes.first_child() {
        changes.remove(&child);
    }

    match (prev_config.windows.enabled, config.windows.enabled) {
        (false, false) => {}
        (true, false) => {
            add_info(changes, "Disabled Windows");
        }
        (_, true) => {
            if !prev_config.windows.enabled {
                add_info(changes, "Enabled Windows");
            }

            #[allow(clippy::cast_sign_loss)]
            if (prev_config.windows.scale - config.windows.scale).abs() > 0.001 {
                add_info_subtitle(
                    changes,
                    "Changed windows scale",
                    format!("{} -> {}", prev_config.windows.scale, config.windows.scale),
                );
            }
            if prev_config.windows.items_per_row != config.windows.items_per_row {
                add_info_subtitle(
                    changes,
                    "Changed windows items per row",
                    format!(
                        "{} -> {}",
                        prev_config.windows.items_per_row, config.windows.items_per_row
                    ),
                );
            }
            match (
                prev_config.windows.overview.enabled,
                config.windows.overview.enabled,
            ) {
                (false, false) => {}
                (true, false) => {
                    add_info(changes, "Disabled Overview");
                }
                (_, true) => {
                    if !prev_config.windows.overview.enabled {
                        add_info(changes, "Enabled Overview");
                    }
                    if prev_config.windows.overview.modifier != config.windows.overview.modifier {
                        add_info_subtitle(
                            changes,
                            "Changed overview modifier",
                            format!(
                                "{} -> {}",
                                prev_config.windows.overview.modifier,
                                config.windows.overview.modifier
                            ),
                        );
                    }
                    if prev_config.windows.overview.key != config.windows.overview.key {
                        add_info_subtitle(
                            changes,
                            "Changed overview key",
                            format!(
                                "{} ({}) -> {} ({})",
                                prev_config.windows.overview.key,
                                key_to_name(&prev_config.windows.overview.key)
                                    .unwrap_or_else(|| String::from("---")),
                                config.windows.overview.key,
                                key_to_name(&config.windows.overview.key)
                                    .unwrap_or_else(|| String::from("---")),
                            ),
                        );
                    }
                    if prev_config.windows.overview.same_class != config.windows.overview.same_class
                        || prev_config.windows.overview.current_monitor
                            != config.windows.overview.current_monitor
                        || prev_config.windows.overview.current_workspace
                            != config.windows.overview.current_workspace
                    {
                        add_info_subtitle(
                            changes,
                            "Changed overview filter by",
                            format!(
                                "{} -> {}",
                                flags_csv!(
                                    prev_config.windows.overview,
                                    same_class,
                                    current_monitor,
                                    current_workspace
                                ),
                                flags_csv!(
                                    config.windows.overview,
                                    same_class,
                                    current_monitor,
                                    current_workspace
                                ),
                            ),
                        );
                    }
                    if prev_config.windows.overview.launcher.launch_modifier
                        != config.windows.overview.launcher.launch_modifier
                    {
                        add_info_subtitle(
                            changes,
                            "Changed overview launcher launch modifier",
                            format!(
                                "{} -> {}",
                                prev_config.windows.overview.launcher.launch_modifier,
                                config.windows.overview.launcher.launch_modifier
                            ),
                        );
                    }
                    if prev_config.windows.overview.launcher.max_items
                        != config.windows.overview.launcher.max_items
                    {
                        add_info_subtitle(
                            changes,
                            "Changed overview launcher max items",
                            format!(
                                "{} -> {}",
                                prev_config.windows.overview.launcher.max_items,
                                config.windows.overview.launcher.max_items
                            ),
                        );
                    }
                    if prev_config.windows.overview.launcher.show_when_empty
                        != config.windows.overview.launcher.show_when_empty
                    {
                        add_info_subtitle(
                            changes,
                            "Changed overview launcher show when empty",
                            format!(
                                "{} -> {}",
                                prev_config.windows.overview.launcher.show_when_empty,
                                config.windows.overview.launcher.show_when_empty
                            ),
                        );
                    }
                    if prev_config.windows.overview.launcher.width
                        != config.windows.overview.launcher.width
                    {
                        add_info_subtitle(
                            changes,
                            "Changed overview launcher width",
                            format!(
                                "{} -> {}",
                                prev_config.windows.overview.launcher.width,
                                config.windows.overview.launcher.width
                            ),
                        );
                    }
                    match (
                        &prev_config.windows.overview.launcher.default_terminal,
                        &config.windows.overview.launcher.default_terminal,
                    ) {
                        (None, None) => {}
                        (Some(_), None) => {
                            add_info(changes, "Disabled overview launcher default terminal");
                        }
                        (None, Some(dt)) => {
                            add_info_subtitle(
                                changes,
                                "Enabled overview launcher default terminal",
                                format!("{dt:?}"),
                            );
                        }
                        (Some(pdt), Some(cdt)) => {
                            if pdt != cdt {
                                add_info_subtitle(
                                    changes,
                                    "Changed overview launcher default terminal",
                                    format!("{pdt:?} -> {cdt:?}"),
                                );
                            }
                        }
                    }
                    add_plugin_changes(
                        changes,
                        &prev_config.windows.overview.launcher.plugins,
                        &config.windows.overview.launcher.plugins,
                    );
                }
            }
            match (
                &prev_config.windows.switch.enabled,
                &config.windows.switch.enabled,
            ) {
                (false, false) => {}
                (true, false) => {
                    add_info(changes, "Disabled Switch view");
                }
                (_, true) => {
                    if !prev_config.windows.switch.enabled {
                        add_info(changes, "Enabled Switch view");
                    }

                    if prev_config.windows.switch.modifier != config.windows.switch.modifier {
                        add_info_subtitle(
                            changes,
                            "Changed switch modifier",
                            format!(
                                "{} -> {}",
                                prev_config.windows.switch.modifier, config.windows.switch.modifier
                            ),
                        );
                    }
                    if prev_config.windows.switch.same_class != config.windows.switch.same_class
                        || prev_config.windows.switch.current_monitor
                            != config.windows.switch.current_monitor
                        || prev_config.windows.switch.current_workspace
                            != config.windows.switch.current_workspace
                    {
                        add_info_subtitle(
                            changes,
                            "Changed switch filter by",
                            format!(
                                "{} -> {}",
                                flags_csv!(
                                    prev_config.windows.switch,
                                    same_class,
                                    current_monitor,
                                    current_workspace
                                ),
                                flags_csv!(
                                    config.windows.switch,
                                    same_class,
                                    current_monitor,
                                    current_workspace
                                ),
                            ),
                        );
                    }
                    if prev_config.windows.switch.switch_workspaces
                        != config.windows.switch.switch_workspaces
                    {
                        add_info_subtitle(
                            changes,
                            "Changed switch switch workspaces",
                            format!(
                                "{} -> {}",
                                prev_config.windows.switch.switch_workspaces,
                                config.windows.switch.switch_workspaces
                            ),
                        );
                    }
                }
            }
            match (
                &prev_config.windows.switch_2.enabled,
                &config.windows.switch_2.enabled,
            ) {
                (false, false) => {}
                (true, false) => {
                    add_info(changes, "Disabled Switch 2 view");
                }
                (_, true) => {
                    if !prev_config.windows.switch_2.enabled {
                        add_info(changes, "Enabled Switch 2 view");
                    }

                    if prev_config.windows.switch_2.modifier != config.windows.switch_2.modifier {
                        add_info_subtitle(
                            changes,
                            "Changed switch 2 modifier",
                            format!(
                                "{} -> {}",
                                prev_config.windows.switch_2.modifier,
                                config.windows.switch_2.modifier
                            ),
                        );
                    }
                    if prev_config.windows.switch_2.same_class != config.windows.switch_2.same_class
                        || prev_config.windows.switch_2.current_monitor
                            != config.windows.switch_2.current_monitor
                        || prev_config.windows.switch_2.current_workspace
                            != config.windows.switch_2.current_workspace
                    {
                        add_info_subtitle(
                            changes,
                            "Changed switch 2 filter by",
                            format!(
                                "{} -> {}",
                                flags_csv!(
                                    prev_config.windows.switch_2,
                                    same_class,
                                    current_monitor,
                                    current_workspace
                                ),
                                flags_csv!(
                                    config.windows.switch_2,
                                    same_class,
                                    current_monitor,
                                    current_workspace
                                ),
                            ),
                        );
                    }
                    if prev_config.windows.switch_2.switch_workspaces
                        != config.windows.switch_2.switch_workspaces
                    {
                        add_info_subtitle(
                            changes,
                            "Changed switch 2 switch workspaces",
                            format!(
                                "{} -> {}",
                                prev_config.windows.switch_2.switch_workspaces,
                                config.windows.switch_2.switch_workspaces
                            ),
                        );
                    }
                }
            }
        }
    }

    let changes_exist = if changes.first_child().is_none() {
        add_info(changes, "No changes");
        false
    } else {
        true
    };

    // let text = config_lib::explain(config, path, false, false);
    // how_to_use.buffer().set_text(&text);

    changes_exist
}

fn add_plugin_changes(changes: &ListBox, prev: &Plugins, current: &Plugins) {
    match (&prev.applications.enabled, &current.applications.enabled) {
        (false, false) => {}
        (true, false) => {
            add_info(changes, "Disabled Application Plugin");
        }
        (_, true) => {
            if !prev.applications.enabled {
                add_info(changes, "Enabled Application Plugin");
            }

            if prev.applications.show_execs != current.applications.show_execs {
                add_info_subtitle(
                    changes,
                    "Changed application plugin show execs",
                    format!(
                        "{} -> {}",
                        prev.applications.show_execs, current.applications.show_execs
                    ),
                );
            }

            if prev.applications.show_actions_submenu != current.applications.show_actions_submenu {
                add_info_subtitle(
                    changes,
                    "Changed application plugin show actions",
                    format!(
                        "{} -> {}",
                        prev.applications.show_actions_submenu,
                        current.applications.show_actions_submenu
                    ),
                );
            }

            if prev.applications.run_cache_weeks != current.applications.run_cache_weeks {
                add_info_subtitle(
                    changes,
                    "Changed application plugin run cache weeks",
                    format!(
                        "{} -> {}",
                        prev.applications.run_cache_weeks, current.applications.run_cache_weeks
                    ),
                );
            }
        }
    }

    match (&prev.terminal.enabled, &current.terminal.enabled) {
        (true, false) => {
            add_info(changes, "Disabled Terminal Plugin");
        }
        (false, true) => {
            add_info(changes, "Enabled Terminal Plugin");
        }
        _ => {}
    }

    match (&prev.shell.enabled, &current.shell.enabled) {
        (true, false) => {
            add_info(changes, "Disabled Shell Plugin");
        }
        (false, true) => {
            add_info(changes, "Enabled Shell Plugin");
        }
        _ => {}
    }
}

fn add_info(changes: &ListBox, text: &str) {
    let label = ActionRow::builder().title(text).build();
    changes.append(&label);
}

fn add_info_subtitle(changes: &ListBox, text: &str, subtitle: String) {
    let label = ActionRow::builder().title(text).subtitle(subtitle).build();
    changes.append(&label);
}
