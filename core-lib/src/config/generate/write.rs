use crate::config::structs::{
    Config, KeyMaybeMod, Launcher, Mod, Navigate, OpenOverview, OpenSwitch, Overview, Reverse,
    Switch, Windows,
};
use anyhow::{bail, Context};
use ron::extensions::Extensions;
use ron::ser::PrettyConfig;
use ron::Options;
use std::ffi::OsStr;
use std::fs::{create_dir_all, write, File};
use std::path::Path;
use tracing::{info, span, Level};

#[derive(Debug)]
pub struct ConfigData {
    pub enable_launcher: bool,
    pub default_terminal: Option<String>,
    pub overview: Option<(Mod, KeyMaybeMod)>,
    pub switch: Option<Mod>,
    pub grave_reverse: bool,
}

pub fn generate_config(data: ConfigData) -> Config {
    Config {
        launcher: if data.enable_launcher {
            Some(Launcher {
                default_terminal: data.default_terminal,
                ..Default::default()
            })
        } else {
            None
        },
        windows: Some(Windows {
            overview: if let Some(overview) = data.overview {
                Some(Overview {
                    open: OpenOverview {
                        modifier: overview.0,
                        key: overview.1,
                    },
                    navigate: Navigate {
                        reverse: if data.grave_reverse {
                            Reverse::Key("grave".to_string())
                        } else {
                            Reverse::Mod(Mod::Shift)
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
            } else {
                None
            },
            switch: if let Some(switch_mod) = data.switch {
                Some(Switch {
                    open: OpenSwitch {
                        modifier: switch_mod,
                    },
                    navigate: Navigate {
                        reverse: if data.grave_reverse {
                            Reverse::Key("grave".to_string())
                        } else {
                            Reverse::Mod(Mod::Shift)
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
            } else {
                None
            },
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub fn write_config(config_path: &Path, config: Config, override_file: bool) -> anyhow::Result<()> {
    let _span = span!(Level::TRACE, "write_config").entered();

    if config_path.exists() && !override_file {
        bail!("Config file already exists, delete it before generating a new one or use -f to override");
    }
    if let Some(parent) = config_path.parent() {
        create_dir_all(parent)
            .with_context(|| format!("Failed to create config dir at ({parent:?})"))?;
    }
    match config_path.extension().and_then(OsStr::to_str) {
        None | Some("ron") => {
            let file = File::create(config_path)
                .with_context(|| format!("Failed to create config at ({config_path:?})"))?;
            Options::default()
                .with_default_extension(Extensions::IMPLICIT_SOME)
                .with_default_extension(Extensions::UNWRAP_NEWTYPES)
                .with_default_extension(Extensions::UNWRAP_VARIANT_NEWTYPES)
                .to_io_writer_pretty(file, &config, PrettyConfig::default())
                .context("Failed to write ron config")?;
        }
        #[cfg(feature = "json_config")]
        Some("json") => {
            let file = File::create(config_path)
                .with_context(|| format!("Failed to create config at ({config_path:?})"))?;
            serde_json::to_writer_pretty(file, &config).context("Failed to write json config")?
        }
        #[cfg(feature = "toml_config")]
        Some("toml") => {
            let str = toml::to_string_pretty(&config).context("Failed to write toml config")?;
            write(config_path, str)
                .with_context(|| format!("Failed to create config at ({config_path:?})"))?;
        }
        Some(ext) => bail!("Invalid config file extension: {}", ext),
    };

    info!("Config file generated successfully");
    Ok(())
}
