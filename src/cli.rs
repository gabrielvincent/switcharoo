use std::fmt::Debug;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "hyprshell is a Rust-based GUI designed to enhance window management in hyprland")]
pub struct App {
    #[clap(flatten)]
    pub global_opts: GlobalOpts,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Args, Debug, Clone)]
pub struct GlobalOpts {
    /// Increase the verbosity level (-v: debug, -vv: trace)
    #[arg(short = 'v', global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Turn off all output
    #[arg(short = 'q', long, global = true)]
    pub quiet: bool,

    /// Path to config [default: $XDG_CONFIG_HOME/hyprshell/config.ron],
    /// allowed file types: ron, toml, json
    #[arg(short = 'c', long, global = true)]
    pub config_file: Option<std::path::PathBuf>,

    /// Path to css [default: $XDG_CONFIG_HOME/hyprshell/style.css]
    #[arg(long, short = 's', global = true)]
    pub css_file: Option<std::path::PathBuf>,

    /// Path to css [default: $XDG_DATA_HOME/hyprshell]
    #[arg(long, short = 'd', global = true)]
    pub data_dir: Option<std::path::PathBuf>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Run {},
    #[cfg(feature = "generate_config_command")]
    /// Generate the config file
    Config {
        #[clap(subcommand)]
        command: ConfigCommand,
    },

    #[cfg(feature = "debug_command")]
    /// Debug command to debug finding icons for the GUI, desktop files, etc.
    Debug {
        #[clap(subcommand)]
        command: DebugCommand,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigCommand {
    /// Generate a default config file
    Generate {
        /// Turn off all output
        #[arg(short = 'f', long)]
        force: bool,
    },
    /// Check the config file for errors and explain how to use the program based on the config
    Check {},
}

#[derive(Subcommand, Debug, Clone)]
pub enum DebugCommand {
    /// Search for an icon with a window class
    Search {
        /// The class (from `hyprctl clients -j | jq -e ".[] | {title, class}"`) of a window to find an icon for
        #[arg(long)]
        class: String,
    },

    /// List all icons in the theme
    List,

    /// List all desktop files
    DesktopFiles,

    /// List cache for launched apps
    LaunchCache,
}
