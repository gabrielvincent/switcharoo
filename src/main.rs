use anyhow::Context;
use clap::Parser;
use core_lib::{
    check_version, daemon_running, get_default_config_path, get_default_css_path,
    get_default_data_dir,
};
use std::env;

mod cli;
mod data;
mod keybinds;
mod receive_handle;
mod socket;
mod start;
mod util;

#[cfg(feature = "debug_command")]
mod debug;

fn main() -> anyhow::Result<()> {
    let cli = cli::App::parse();
    let opts = cli.global_opts.clone();

    let level = if opts.quiet {
        "off"
    } else {
        match opts.verbose {
            0 => "info",
            1 => "debug",
            2.. => "trace",
        }
    };
    let subscriber = tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_target(
            env::var("HYPRSHELL_LOG_MODULE_PATH")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(false),
        )
        .with_env_filter(format!(
            "hyprshell={level},core_lib={level},exec_lib={level},launcher_lib={level},windows_lib={level}",
        ))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .unwrap_or_else(|e| tracing::warn!("Unable to initialize logging: {e}"));

    check_features();
    check_env();

    let data_dir = cli.global_opts.data_dir;
    let css_file = cli.global_opts.css_file;
    let config_path = cli.global_opts.config_file;

    match cli.command {
        cli::Command::Run {} => {
            if daemon_running() {
                anyhow::bail!("Daemon already running");
            }
            check_version(exec_lib::get_version()).unwrap_or_else(|e| {
                tracing::warn!("Unable to check hyprland version, continuing anyway: {e}")
            });
            start::start(
                config_path.unwrap_or(get_default_config_path()),
                css_file.unwrap_or(get_default_css_path()),
                data_dir.unwrap_or(get_default_data_dir()),
            )?;
        }
        #[cfg(feature = "generate_config_command")]
        cli::Command::Config { command } => match command {
            cli::ConfigCommand::Generate { force, no_systemd } => {
                use core_lib::Warn;
                let config_path = config_path.unwrap_or(get_default_config_path());
                let css_path = css_file.unwrap_or(get_default_css_path());

                let (override_config, override_css) =
                    core_lib::config::generate::get_overrides(&force);
                core_lib::config::generate::check_file_exist(
                    &config_path,
                    &css_path,
                    override_config,
                    override_css,
                )?;

                let (config_data, css_data) = core_lib::config::generate::prompt_config()?;
                let config = core_lib::config::generate::generate_config(config_data);
                core_lib::config::write_config(&config_path, &config, override_config).warn();
                core_lib::config::generate::write_css(&css_path, &css_data, override_css).warn();
                if !no_systemd {
                    core_lib::config::generate::write_systemd_unit(
                        opts.config_file.as_ref(),
                        opts.css_file.as_ref(),
                        opts.data_dir.as_ref(),
                    )
                    .warn();
                }
                core_lib::config::explain::explain_config(&config_path).warn();
            }
            cli::ConfigCommand::Explain {} => {
                use core_lib::Warn;
                core_lib::config::explain::explain_config(
                    &config_path.unwrap_or(get_default_config_path()),
                )
                .warn();
            }
            cli::ConfigCommand::Check {} => {
                if let Err(err) = core_lib::config::load_and_migrate_config(
                    &config_path.unwrap_or(get_default_config_path()),
                ) {
                    tracing::warn!("Failed to load config: {err}");
                    std::process::exit(1);
                }
            }
            #[cfg(feature = "config_check_is_default")]
            cli::ConfigCommand::CheckIfDefault {} => {
                if let Ok(config) = core_lib::config::load_and_migrate_config(
                    &config_path.unwrap_or(get_default_config_path()),
                ) {
                    let config_default = core_lib::config::Config::default();
                    if config != config_default {
                        tracing::warn!("Current config does not match the default configuration");
                        tracing::info!("Default config: {:#?}", config_default);
                        tracing::info!("Current config: {:#?}", config);
                        std::process::exit(1);
                    } else {
                        tracing::info!("Current config matches the default configuration");
                    }
                }
            }
        },
        #[cfg(feature = "debug_command")]
        cli::Command::Debug { command } => {
            tracing::info!("use with -vv ... to see full logs!");
            match command {
                cli::DebugCommand::CheckClass { class } => {
                    debug::check_class(class);
                }
                cli::DebugCommand::ListIcons => {
                    debug::list_icons();
                }
                cli::DebugCommand::ListDesktopFiles => {
                    debug::list_desktop_files();
                }
                cli::DebugCommand::Search { text, all } => {
                    debug::search(
                        &text,
                        all,
                        &config_path.unwrap_or(get_default_config_path()),
                        &data_dir.unwrap_or(get_default_data_dir()),
                    );
                }
            };
        }
        cli::Command::Data { command } => match command {
            cli::DataCommand::LaunchHistory { run_cache_weeks } => {
                data::launch_history(
                    run_cache_weeks,
                    &config_path.unwrap_or(get_default_config_path()),
                    &data_dir.unwrap_or(get_default_data_dir()),
                    opts.verbose,
                );
            }
        },
        cli::Command::Socat { json } => core_lib::transfer::send_raw_to_socket(&json)
            .context("Failed to send JSON to socket: is hyprshell running?")?,
    }
    Ok(())
}

fn check_features() {
    tracing::debug!(
        "FEATURES: TOML support: {}, Bar: {}, Config command: {}, Debug command: {}, Launcher calc: {}",
        cfg!(feature = "toml_config"),
        cfg!(feature = "bar"),
        cfg!(feature = "generate_config_command"),
        cfg!(feature = "debug_command"),
        cfg!(feature = "launcher_calc"),
    );
}

fn check_env() {
    tracing::trace!(
        "ENV: HYPRSHELL_NO_LISTENERS: {}, HYPRSHELL_NO_ALL_ICONS: {}, HYPRSHELL_RELOAD_TIMEOUT: {}, HYPRSHELL_LOG_MODULE_PATH: {}",
        env::var("HYPRSHELL_NO_LISTENERS").unwrap_or_else(|_| "-None-".to_string()),
        env::var("HYPRSHELL_NO_ALL_ICONS").unwrap_or_else(|_| "-None-".to_string()),
        env::var("HYPRSHELL_RELOAD_TIMEOUT").unwrap_or_else(|_| "-None-".to_string()),
        env::var("HYPRSHELL_LOG_MODULE_PATH").unwrap_or_else(|_| "-None-".to_string()),
    )
}
