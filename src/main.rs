use anyhow::bail;
use clap::Parser;
use core_lib::config::{get_default_config_path, get_default_css_path, get_default_data_dir};
use core_lib::{check_version, daemon_running, Warn};
use exec_lib::get_version;
use std::env;
use tracing::{debug, warn};

mod cli;
mod receive;
mod start;

fn main() -> anyhow::Result<()> {
    malloc::limit_mmap_threshold();
    let cli = cli::App::parse();

    let level = if cli.global_opts.quiet {
        "off"
    } else {
        match cli.global_opts.verbose {
            0 => "info",
            1 => "debug",
            2.. => "trace",
        }
    };
    let subscriber = tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_target(
            env::var("LOG_MODULE_PATH")
                .map(|s| s.parse().unwrap_or(false))
                .unwrap_or(false),
        )
        .with_env_filter(format!(
            "hyprshell={level},core_lib={level},exec_lib={level},launcher_lib={level},windows_lib={level}",
        ))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .unwrap_or_else(|e| warn!("Unable to initialize logging: {e}"));

    check_features();
    check_version(get_version())
        .unwrap_or_else(|e| warn!("Unable to check hyprland version, continuing anyway: {e}"));

    let opts = cli.global_opts.clone();
    let config_path = cli
        .global_opts
        .config_file
        .unwrap_or(get_default_config_path());
    let css_path = cli.global_opts.css_file.unwrap_or(get_default_css_path());
    let data_dir = cli.global_opts.data_dir.unwrap_or(get_default_data_dir());

    match cli.command {
        cli::Command::Run {} => {
            if daemon_running() {
                bail!("Daemon already running");
            }
            start::start(config_path, css_path, data_dir)?;
        }
        #[cfg(feature = "generate_config_command")]
        cli::Command::Config { command } => match command {
            cli::ConfigCommand::Generate { force, no_systemd } => {
                let config_data = core_lib::config::generate::prompt_config()?;
                let config = core_lib::config::generate::generate_config(config_data);
                core_lib::config::generate::write_config(&config_path, config, force)
                    .warn("create");
                core_lib::config::generate::write_css(css_path, force).warn("create");
                core_lib::config::check::check_config(&config_path).warn("check");
                if !no_systemd {
                    core_lib::config::generate::write_systemd_unit(
                        force,
                        opts.config_file.as_ref(),
                        opts.css_file.as_ref(),
                        opts.data_dir.as_ref(),
                    )
                    .warn("create");
                }
            }
            cli::ConfigCommand::Check {} => {
                core_lib::config::check::check_config(&config_path).warn("check");
            }
        },
        #[cfg(feature = "debug_command")]
        cli::Command::Debug { command } => {
            println!("use with -vv ... to see full logs!");
            match command {
                cli::DebugCommand::Search { class } => {
                    let _ = class;
                    todo!("Config command not implemented")
                }
                cli::DebugCommand::List => {
                    todo!("Config command not implemented")
                }
                cli::DebugCommand::DesktopFiles => {
                    todo!("Config command not implemented")
                }
                cli::DebugCommand::LaunchCache => {
                    todo!("Config command not implemented")
                }
            };
        }
    }
    Ok(())
}

pub mod malloc {
    use std::os::raw::c_int;
    const M_MMAP_THRESHOLD: c_int = -3;

    extern "C" {
        fn mallopt(param: c_int, value: c_int) -> c_int;
    }

    /// Prevents glibc from hoarding memory via memory fragmentation.
    pub fn limit_mmap_threshold() {
        unsafe {
            mallopt(M_MMAP_THRESHOLD, 65536);
        }
    }
}

fn check_features() {
    debug!("FEATURES: JSON config: {}, TOML support: {}, Bar: {}, Config command: {}, Debug command: {}",
        cfg!(feature = "json_config"),
        cfg!(feature = "toml_config"),
        cfg!(feature = "bar"),
        cfg!(feature = "generate_config_command"),
        cfg!(feature = "debug_command"),
    );
}
