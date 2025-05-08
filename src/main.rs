use anyhow::bail;
use clap::Parser;
use core_lib::{
    check_version, daemon_running, get_default_config_path, get_default_css_path,
    get_default_data_dir,
};
use exec_lib::get_version;
use std::env;
use tracing::{debug, warn};

mod cli;
mod keybinds;
mod receive;
mod recive_handle;
mod start;

fn main() -> anyhow::Result<()> {
    malloc::limit_mmap_threshold();
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
            check_version(get_version()).unwrap_or_else(|e| {
                warn!("Unable to check hyprland version, continuing anyway: {e}")
            });
            start::start(config_path, css_path, data_dir)?;
        }
        #[cfg(feature = "generate_config_command")]
        cli::Command::Config { command } => match command {
            cli::ConfigCommand::Generate { force, no_systemd } => {
                use core_lib::Warn;
                let (config_data, css_data) = core_lib::config::generate::prompt_config()?;
                let config = core_lib::config::generate::generate_config(config_data);
                core_lib::config::generate::write_config(&config_path, config, force)
                    .warn("create");
                core_lib::config::generate::write_css(css_path, force, css_data).warn("create");
                if !no_systemd {
                    core_lib::config::generate::write_systemd_unit(
                        force,
                        opts.config_file.as_ref(),
                        opts.css_file.as_ref(),
                        opts.data_dir.as_ref(),
                    )
                    .warn("create");
                }
                core_lib::config::explain::check_config(&config_path).warn("check");
            }
            cli::ConfigCommand::Check {} => {
                use core_lib::Warn;
                core_lib::config::explain::check_config(&config_path).warn("check");
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
    debug!(
        "FEATURES: TOML support: {}, Bar: {}, Config command: {}, Debug command: {}",
        cfg!(feature = "toml_config"),
        cfg!(feature = "bar"),
        cfg!(feature = "generate_config_command"),
        cfg!(feature = "debug_command"),
    );
}
