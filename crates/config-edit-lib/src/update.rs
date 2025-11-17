use crate::structs::{
    GTKConfig, GTKLauncher, GTKOverview, GTKPlugins, GTKSwitch, GTKWebsearch, GTKWindowsFilter,
};
use crate::update_changes_view::update_changes_view;
use crate::views::launcher;
use adw::ViewStack;
use adw::prelude::{ButtonExt, EditableExt, ExpanderRowExt, PreferencesRowExt, WidgetExt};
use config_lib::{Config, FilterBy, Launcher, Modifier, Overview, Plugins, Switch};
use std::cell::RefCell;
use std::rc::Rc;
use tracing::trace;

pub fn update_config(gtk_config: &mut GTKConfig, config_rc: &Rc<RefCell<Config>>) {
    let config = &config_rc.borrow();
    // let view_stack = &gtk_config.view_stack;
    if let Some(windows) = &config.windows {
        gtk_config.windows.row.set_enable_expansion(true);
        gtk_config.windows.row.set_expanded(true);
        #[allow(clippy::cast_sign_loss)]
        if gtk_config.windows.scale.value() as u8 != windows.scale as u8 {
            gtk_config.windows.scale.set_value(windows.scale);
        }
        #[allow(clippy::cast_sign_loss)]
        if gtk_config.windows.items_per_row.value() as u8 != windows.items_per_row {
            gtk_config
                .windows
                .items_per_row
                .set_value(f64::from(windows.items_per_row));
        }
        update_overview(gtk_config, windows.overview.as_ref(), config_rc);
        update_switch(&gtk_config.windows.switch, windows.switch.as_ref());
    } else {
        gtk_config.windows.row.set_enable_expansion(false);
        update_overview(gtk_config, None, config_rc);
        update_switch(&gtk_config.windows.switch, None);
    }

    let changes_exist = update_changes_view(
        &gtk_config.changes,
        &gtk_config.how_to_use,
        config,
        &gtk_config.path,
    );
    match (changes_exist, config_lib::check(config)) {
        (true, Ok(())) => {
            gtk_config.save.set_sensitive(true);
            gtk_config.save.set_tooltip_text(Some("Save changes"));
        }
        (_, Err(err)) => {
            gtk_config.save.set_sensitive(false);
            gtk_config.save.set_tooltip_text(Some(&err.to_string()));
        }
        (false, _) => {
            gtk_config.save.set_sensitive(false);
            gtk_config.save.set_tooltip_text(Some("No changes to save"));
        }
    }
}

fn update_overview(
    gtk_config: &mut GTKConfig,
    overview: Option<&Overview>,
    config_rc: &Rc<RefCell<Config>>,
) {
    let g_overview = &mut gtk_config.windows.overview;

    if let Some(overview) = overview {
        g_overview.row.set_enable_expansion(true);
        g_overview.row.set_expanded(true);
        if g_overview.key.text() != *overview.key {
            g_overview.key.set_text(&overview.key);
        }
        let desired_modifier = match overview.modifier {
            Modifier::Alt => 0,
            Modifier::Ctrl => 1,
            Modifier::Super => 2,
        };
        if g_overview.modifier.selected() != desired_modifier {
            g_overview.modifier.set_selected(desired_modifier);
        }
        update_windows_filter(&g_overview.filter, &overview.filter_by);
        update_launcher(gtk_config, Some(&overview.launcher), config_rc);
    } else {
        g_overview.row.set_enable_expansion(false);
        update_launcher(gtk_config, None, config_rc);
    }
}

fn update_windows_filter(g_filter: &GTKWindowsFilter, filter: &[FilterBy]) {
    if g_filter.same_class.is_active() != filter.contains(&FilterBy::SameClass) {
        g_filter
            .same_class
            .set_active(filter.contains(&FilterBy::SameClass));
    }
    if g_filter.workspace.is_active() != filter.contains(&FilterBy::CurrentWorkspace) {
        g_filter
            .workspace
            .set_active(filter.contains(&FilterBy::CurrentWorkspace));
    }
    if g_filter.monitor.is_active() != filter.contains(&FilterBy::CurrentMonitor) {
        g_filter
            .monitor
            .set_active(filter.contains(&FilterBy::CurrentMonitor));
    }
    g_filter.row.set_title(&if filter.is_empty() {
        String::from("Filter")
    } else if filter.len() == 1 {
        format!("Filter: {:?}", filter[0])
    } else if filter.len() == 2 {
        format!("Filter: {:?} + {:?}", filter[0], filter[1])
    } else {
        // should not be possible, maybe if loaded from config
        format!(
            "Filter: {:?} + {:?} + {:?}",
            filter[0], filter[1], filter[2]
        )
    });
}

fn update_switch(g_swich: &GTKSwitch, switch: Option<&Switch>) {
    match switch {
        Some(switch) => {
            g_swich.row.set_enable_expansion(true);
            g_swich.row.set_expanded(true);
            let desired_modifier = match switch.modifier {
                Modifier::Alt => 0,
                Modifier::Ctrl => 1,
                Modifier::Super => 2,
            };
            if g_swich.modifier.selected() != desired_modifier {
                g_swich.modifier.set_selected(desired_modifier);
            }
            update_windows_filter(&g_swich.filter, &switch.filter_by);
            if g_swich.switch_workspaces.is_active() != switch.switch_workspaces {
                g_swich
                    .switch_workspaces
                    .set_active(switch.switch_workspaces);
            }
        }
        None => {
            g_swich.row.set_enable_expansion(false);
        }
    }
}

fn update_launcher(
    gtk_config: &mut GTKConfig,
    config: Option<&Launcher>,
    config_rc: &Rc<RefCell<Config>>,
) {
    let g_launcher = &mut gtk_config.windows.overview.launcher;

    if let Some(launcher) = config {
        if gtk_config.view_stack.child_by_name("launcher").is_none() {
            trace!("Adding launcher view");
            gtk_config.view_stack.add_titled_with_icon(
                &g_launcher.view,
                Some("launcher"),
                "Launcher",
                "configure",
            );
        }
        g_launcher.row.set_enable_expansion(true);
        g_launcher.row.set_expanded(true);
        let desired_modifier = match launcher.launch_modifier {
            Modifier::Alt => 0,
            Modifier::Ctrl => 1,
            Modifier::Super => 2,
        };
        if g_launcher.modifier.selected() != desired_modifier {
            g_launcher.modifier.set_selected(desired_modifier);
        }
        #[allow(clippy::cast_sign_loss)]
        if g_launcher.width.value() as u32 != launcher.width {
            g_launcher.width.set_value(f64::from(launcher.width));
        }
        #[allow(clippy::cast_sign_loss)]
        if g_launcher.max_items.value() as u8 != launcher.max_items {
            g_launcher
                .max_items
                .set_value(f64::from(launcher.max_items));
        }
        if g_launcher.show_when_empty.is_active() != launcher.show_when_empty {
            g_launcher
                .show_when_empty
                .set_active(launcher.show_when_empty);
        }
        if let Some(terminal) = &launcher.default_terminal {
            if !g_launcher.dont_use_default_terminal.is_active() {
                g_launcher.dont_use_default_terminal.set_active(true);
            }
            if *g_launcher.terminal.text() != **terminal {
                g_launcher.terminal.set_text(terminal);
            }
            g_launcher.terminal.set_sensitive(true);
        } else {
            if g_launcher.dont_use_default_terminal.is_active() {
                g_launcher.dont_use_default_terminal.set_active(false);
            }
            if g_launcher.terminal.text() != "" {
                g_launcher.terminal.set_text("");
            }
            g_launcher.terminal.set_sensitive(false);
        }
        update_plugins(
            gtk_config,
            Some(&launcher.plugins),
            launcher.launch_modifier,
            config_rc,
        );
    } else {
        trace!("Removing launcher view");
        if gtk_config.view_stack.child_by_name("launcher").is_some() {
            gtk_config.view_stack.remove(&g_launcher.view);
        }
    }
}

fn update_plugins(
    gtk_config: &mut GTKConfig,
    config: Option<&Plugins>,
    launcher_modifier: Modifier,
    config_rc: &Rc<RefCell<Config>>,
) {
    let g_plugins = &mut gtk_config.windows.overview.launcher.plugins;

    if let Some(plugins) = config {
        g_plugins.row.set_enable_expansion(true);
        g_plugins.row.set_expanded(true);

        g_plugins.terminal.set_active(plugins.terminal.is_some());
        g_plugins.shell.set_active(plugins.shell.is_some());
        g_plugins.calc.set_active(plugins.calc.is_some());
        g_plugins.path.set_active(plugins.path.is_some());

        match &plugins.applications {
            Some(applications) => {
                g_plugins.applications.row.set_enable_expansion(true);
                g_plugins.applications.row.set_expanded(true);
                #[allow(clippy::cast_sign_loss)]
                if g_plugins.applications.cache_weeks.value() as u8 != applications.run_cache_weeks
                {
                    g_plugins
                        .applications
                        .cache_weeks
                        .set_value(f64::from(applications.run_cache_weeks));
                }
                if g_plugins.applications.submenu.is_active() != applications.show_actions_submenu {
                    g_plugins
                        .applications
                        .submenu
                        .set_active(applications.show_actions_submenu);
                }
                if g_plugins.applications.show_exec.is_active() != applications.show_execs {
                    g_plugins
                        .applications
                        .show_exec
                        .set_active(applications.show_execs);
                }
            }
            None => {
                g_plugins.applications.row.set_enable_expansion(false);
            }
        }

        match &plugins.websearch {
            Some(search) => {
                g_plugins.websearches.row.set_enable_expansion(true);
                g_plugins.websearches.row.set_expanded(true);

                while let Some(child) = g_plugins.websearches.list.first_child() {
                    g_plugins.websearches.list.remove(&child);
                }
                for search_engine in search.engines.iter() {
                    let (row, delete, edit) =
                        launcher::generate_row(search_engine, launcher_modifier);
                    edit.connect_clicked(move |_| {
                        crate::bind::websearch_handler(
                            search_engine.key,
                            &gtk_config.window,
                            config_rc,
                            gtk_config,
                        )
                    });
                    g_plugins.websearches.list.append(&row);
                    g_plugins
                        .websearches
                        .items
                        .insert(search_engine.key, GTKWebsearch { row, edit, delete });
                }
            }
            None => {
                g_plugins.websearches.row.set_enable_expansion(false);
            }
        }
    } else {
        g_plugins.row.set_enable_expansion(false);
        g_plugins.row.set_expanded(false);
    }
}
