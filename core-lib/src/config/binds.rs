use crate::config::structs::{Config, FilterBy, Overview, Reverse, Switch, ToKey};
use crate::config::Launcher;
use crate::transfer::{
    Direction, OpenOverview, OpenSwitch, ReturnConfig, SwitchConfig, TransferType,
};
use crate::util::{get_daemon_socket_path_buff, LAUNCHER_NAMESPACE, OVERVIEW_NAMESPACE};
use anyhow::Context;
use ron::extensions::Extensions;
use std::env;
use std::path::PathBuf;
use tracing::{span, Level};

pub fn create_binds_and_submaps<'a>(config: &Config) -> anyhow::Result<Vec<(&'a str, String)>> {
    let _span = span!(Level::DEBUG, "create_binds_and_submaps").entered();
    let ron_options = ron::Options::default()
        .with_default_extension(Extensions::IMPLICIT_SOME)
        .with_default_extension(Extensions::UNWRAP_VARIANT_NEWTYPES)
        .with_default_extension(Extensions::EXPLICIT_STRUCT_NAMES);

    let socat_path = get_socat_path()?;

    let mut keyword_list = Vec::<(&str, String)>::new();

    if config.layerrules {
        keyword_list.push(("layerrule", format!("noanim, {LAUNCHER_NAMESPACE}")));
        keyword_list.push(("layerrule", format!("noanim, {OVERVIEW_NAMESPACE}")));
        keyword_list.push(("layerrule", format!("dimaround, {OVERVIEW_NAMESPACE}")));
    }

    if let Some(windows) = &config.windows {
        if let Some(overview) = &windows.overview {
            let workspaces_per_row = windows.workspaces_per_row;
            let submap_name = "hyprshell-overview";
            generate_overview(
                &mut keyword_list,
                &ron_options,
                overview,
                submap_name,
                workspaces_per_row,
                &config.launcher,
                &socat_path,
            )
            .context("Failed to generate overview")?;
        }
        if let Some(switch) = &windows.switch {
            let workspaces_per_row = windows.workspaces_per_row;
            let submap_name = "hyprshell-switch";
            generate_switch(
                &mut keyword_list,
                &ron_options,
                switch,
                submap_name,
                workspaces_per_row,
                &socat_path,
            )
            .context("Failed to generate overview")?;
        }
    }

    Ok(keyword_list)
}

fn get_socat_path() -> anyhow::Result<String> {
    env::var("HYPRSHELL_SOCAT_PATH")
        .or_else(|_| which::which("socat").map(|path| path.to_string_lossy().to_string()))
        .context("`socat` command not found. Please ensure it is installed and available in PATH.")
}

fn generate_socat(echo: &str, path: PathBuf, socat_path: &str) -> String {
    format!(
        r#"echo '{}' | {} - UNIX-CONNECT:{}"#,
        echo,
        socat_path,
        path.as_path().to_string_lossy()
    )
}

fn generate_close(ron_options: &ron::Options, socat_path: &str) -> anyhow::Result<String> {
    let config = TransferType::Close;
    let config_str = ron_options
        .to_string(&config)
        .context("Failed to serialize config")?;
    Ok(generate_socat(
        &config_str,
        get_daemon_socket_path_buff(),
        socat_path,
    ))
}

fn generate_restart(ron_options: &ron::Options, socat_path: &str) -> anyhow::Result<String> {
    let config = TransferType::Restart;
    let config_str = ron_options
        .to_string(&config)
        .context("Failed to serialize config")?;
    Ok(generate_socat(
        &config_str,
        get_daemon_socket_path_buff(),
        socat_path,
    ))
}

fn generate_return(
    ron_options: &ron::Options,
    offset: u8,
    socat_path: &str,
) -> anyhow::Result<String> {
    let config = TransferType::Return(ReturnConfig { offset });
    let config_str = ron_options
        .to_string(&config)
        .context("Failed to serialize config")?;
    Ok(generate_socat(
        &config_str,
        get_daemon_socket_path_buff(),
        socat_path,
    ))
}

fn generate_switch_press(
    ron_options: &ron::Options,
    direction: Direction,
    workspace: bool,
    socat_path: &str,
) -> anyhow::Result<String> {
    let config = TransferType::Switch(SwitchConfig {
        direction,
        workspace,
    });
    let config_str = ron_options
        .to_string(&config)
        .context("Failed to serialize config")?;
    Ok(generate_socat(
        &config_str,
        get_daemon_socket_path_buff(),
        socat_path,
    ))
}

fn generate_overview_open(
    ron_options: &ron::Options,
    submap_name: &str,
    overview: &Overview,
    workspaces_per_row: u8,
    socat_path: &str,
) -> anyhow::Result<String> {
    let config = TransferType::OpenOverview(OpenOverview {
        hide_filtered: overview.other.hide_filtered,
        filter_current_workspace: overview
            .other
            .filter_by
            .iter()
            .any(|f| f == &FilterBy::CurrentWorkspace),
        filter_current_monitor: overview
            .other
            .filter_by
            .iter()
            .any(|f| f == &FilterBy::CurrentMonitor),
        filter_same_class: overview
            .other
            .filter_by
            .iter()
            .any(|f| f == &FilterBy::SameClass),
        submap_name: submap_name.to_string(),
        workspaces_per_row,
    });
    let config_str = ron_options
        .to_string(&config)
        .context("Failed to serialize config")?;
    Ok(generate_socat(
        &config_str,
        get_daemon_socket_path_buff(),
        socat_path,
    ))
}

fn generate_overview(
    keyword_list: &mut Vec<(&str, String)>,
    ron_options: &ron::Options,
    overview: &Overview,
    submap_name: &str,
    workspaces_per_row: u8,
    launcher: &Option<Launcher>,
    socat_path: &str,
) -> anyhow::Result<()> {
    keyword_list.push((
        "bind",
        format!(
            "{}, {}, exec, {}",
            overview.open.modifier,
            overview.open.key.to_key(),
            generate_overview_open(
                ron_options,
                submap_name,
                overview,
                workspaces_per_row,
                socat_path
            )?,
        ),
    ));

    keyword_list.push(("submap", submap_name.to_string()));
    keyword_list.push((
        "bind",
        format!(
            ", escape, exec, {}",
            generate_close(ron_options, socat_path)?
        ),
    ));
    keyword_list.push((
        "bind",
        format!(
            "{}, {}, exec, {}",
            overview.open.modifier,
            overview.open.key.to_key(),
            generate_close(ron_options, socat_path)?
        ),
    ));
    keyword_list.push((
        "bind",
        format!(
            ", return, exec, {}",
            generate_return(ron_options, 0, socat_path)?
        ),
    ));

    if let Some(_launcher) = launcher {
        // add index keys launcher run
        for i in 1..=9 {
            keyword_list.push((
                "bind",
                format!(
                    "ctrl, {}, exec, {}",
                    i,
                    generate_return(ron_options, i, socat_path)?
                ),
            ));
        }
    }

    keyword_list.push((
        "binde",
        format!(
            ", right, exec, {}",
            generate_switch_press(ron_options, Direction::Right, true, socat_path)?
        ),
    ));
    keyword_list.push((
        "binde",
        format!(
            ", left, exec, {}",
            generate_switch_press(ron_options, Direction::Left, true, socat_path)?
        ),
    ));
    keyword_list.push((
        "binde",
        format!(
            ", down, exec, {}",
            generate_switch_press(ron_options, Direction::Down, true, socat_path)?
        ),
    ));
    keyword_list.push((
        "binde",
        format!(
            ", up, exec, {}",
            generate_switch_press(ron_options, Direction::Up, true, socat_path)?
        ),
    ));

    keyword_list.push((
        "binde",
        format!(
            ", {}, exec, {}",
            overview.navigate.forward,
            generate_switch_press(ron_options, Direction::Right, false, socat_path)?
        ),
    ));
    match &overview.navigate.reverse {
        Reverse::Key(key) => keyword_list.push((
            "binde",
            format!(
                ", {}, exec, {}",
                key,
                generate_switch_press(ron_options, Direction::Left, false, socat_path)?
            ),
        )),
        Reverse::Mod(modk) => keyword_list.push((
            "binde",
            format!(
                "{}, {}, exec, {}",
                modk,
                overview.navigate.forward,
                generate_switch_press(ron_options, Direction::Left, false, socat_path)?
            ),
        )),
    }

    // if poisoned lock
    keyword_list.push((
        "bind",
        "ctrl, k, exec, pkill hyprshell; hyprctl dispatch submap reset".to_string(),
    ));

    // restart demon (like config reload or monitor change)
    keyword_list.push((
        "bind",
        format!(
            "ctrl, r, exec, {}",
            generate_restart(ron_options, socat_path)?
        ),
    ));
    keyword_list.push(("submap", "reset".to_string()));
    Ok(())
}

fn generate_switch_open(
    ron_options: &ron::Options,
    submap_name: &str,
    switch: &Switch,
    workspaces_per_row: u8,
    direction: Direction,
    socat_path: &str,
) -> anyhow::Result<String> {
    let config = TransferType::OpenSwitch(OpenSwitch {
        hide_filtered: switch.other.hide_filtered,
        filter_current_workspace: switch
            .other
            .filter_by
            .iter()
            .any(|f| f == &FilterBy::CurrentWorkspace),
        filter_current_monitor: switch
            .other
            .filter_by
            .iter()
            .any(|f| f == &FilterBy::CurrentMonitor),
        filter_same_class: switch
            .other
            .filter_by
            .iter()
            .any(|f| f == &FilterBy::SameClass),
        submap_name: submap_name.to_string(),
        workspaces_per_row,
        direction,
    });
    let config_str = ron_options
        .to_string(&config)
        .context("Failed to serialize config")?;
    Ok(generate_socat(
        &config_str,
        get_daemon_socket_path_buff(),
        socat_path,
    ))
}

fn generate_switch(
    keyword_list: &mut Vec<(&str, String)>,
    ron_options: &ron::Options,
    switch: &Switch,
    submap_name: &str,
    workspaces_per_row: u8,
    socat_path: &str,
) -> anyhow::Result<()> {
    keyword_list.push((
        "bind",
        format!(
            "{}, {}, exec, {}",
            switch.open.modifier,
            switch.navigate.forward,
            generate_switch_open(
                ron_options,
                submap_name,
                switch,
                workspaces_per_row,
                Direction::Right,
                socat_path
            )?,
        ),
    ));
    match &switch.navigate.reverse {
        Reverse::Key(key) => keyword_list.push((
            "bind",
            format!(
                "{}, {}, exec, {}",
                switch.open.modifier,
                key,
                generate_switch_open(
                    ron_options,
                    submap_name,
                    switch,
                    workspaces_per_row,
                    Direction::Left,
                    socat_path
                )?,
            ),
        )),
        Reverse::Mod(modk) => keyword_list.push((
            "bind",
            format!(
                "{} {}, {}, exec, {}",
                switch.open.modifier,
                modk,
                switch.navigate.forward,
                generate_switch_open(
                    ron_options,
                    submap_name,
                    switch,
                    workspaces_per_row,
                    Direction::Left,
                    socat_path
                )?,
            ),
        )),
    }

    keyword_list.push(("submap", submap_name.to_string()));
    keyword_list.push((
        "bind",
        format!(
            ", escape, exec, {}",
            generate_close(ron_options, socat_path)?
        ),
    ));
    keyword_list.push((
        "bindrt",
        format!(
            "{}, {}, exec, {}",
            switch.open.modifier,
            switch.open.modifier.to_key(),
            generate_return(ron_options, 0, socat_path)?
        ),
    ));
    // second keybinding to close of mod + reverse mod is released
    if let Reverse::Mod(modk) = &switch.navigate.reverse {
        keyword_list.push((
            "bindrt",
            format!(
                "{} {}, {}, exec, {}",
                switch.open.modifier,
                modk,
                switch.open.modifier.to_key(),
                generate_return(ron_options, 0, socat_path)?,
            ),
        ));
    }

    keyword_list.push((
        "bind",
        format!(
            "{}, right, exec, {}",
            switch.open.modifier,
            generate_switch_press(ron_options, Direction::Right, false, socat_path)?
        ),
    ));
    keyword_list.push((
        "binde",
        format!(
            "{}, left, exec, {}",
            switch.open.modifier,
            generate_switch_press(ron_options, Direction::Left, false, socat_path)?
        ),
    ));
    keyword_list.push((
        "binde",
        format!(
            "{}, down, exec, {}",
            switch.open.modifier,
            generate_switch_press(ron_options, Direction::Down, false, socat_path)?
        ),
    ));
    keyword_list.push((
        "binde",
        format!(
            "{}, up, exec, {}",
            switch.open.modifier,
            generate_switch_press(ron_options, Direction::Up, false, socat_path)?
        ),
    ));

    keyword_list.push((
        "binde",
        format!(
            "{}, {}, exec, {}",
            switch.open.modifier,
            switch.navigate.forward,
            generate_switch_press(ron_options, Direction::Right, false, socat_path)?
        ),
    ));
    match &switch.navigate.reverse {
        Reverse::Key(key) => keyword_list.push((
            "binde",
            format!(
                "{}, {}, exec, {}",
                switch.open.modifier,
                key,
                generate_switch_press(ron_options, Direction::Left, false, socat_path)?
            ),
        )),
        Reverse::Mod(modk) => keyword_list.push((
            "binde",
            format!(
                "{} {}, {}, exec, {}",
                switch.open.modifier,
                modk,
                switch.navigate.forward,
                generate_switch_press(ron_options, Direction::Left, false, socat_path)?
            ),
        )),
    }

    // if poisoned lock
    keyword_list.push((
        "bind",
        "ctrl, k, exec, pkill hyprshell; hyprctl dispatch submap reset".to_string(),
    ));

    // restart demon (like config reload or monitor change)
    keyword_list.push((
        "bind",
        format!(
            "ctrl, r, exec, {}",
            generate_restart(ron_options, socat_path)?
        ),
    ));
    keyword_list.push(("submap", "reset".to_string()));
    Ok(())
}
