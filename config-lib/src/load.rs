use crate::Config;
use crate::migrate::check_migration_needed;
use anyhow::{Context, bail};
use ron::Options;
use ron::extensions::Extensions;
use std::ffi::OsStr;
use std::path::Path;
use tracing::{Level, debug, info, span, trace, warn};

pub fn load_and_migrate_config(config_path: &Path) -> anyhow::Result<Config> {
    let _span = span!(Level::TRACE, "load_config", path =? config_path).entered();
    if !config_path.exists() {
        bail!("Config file does not exist, create it using `hyprshell config generate`");
    }

    if check_migration_needed(config_path)
        .inspect_err(|e| warn!("Failed to check migration needed: {e:?}"))
        .unwrap_or(false)
    {
        info!("Config needs migration");
        let migrated = crate::migrate::migrate(config_path);
        match migrated {
            Ok(config) => {
                info!("Config migrated successfully");
                crate::check(&config)?;
                return Ok(config);
            }
            Err(err) => {
                warn!("Migration failed: {err:?}");
                bail!("Failed to load config and migration failed");
            }
        }
    } else {
        trace!("No migration needed")
    }

    let config = match config_path.extension().and_then(OsStr::to_str) {
        None | Some("ron") => {
            let options = Options::default()
                .with_default_extension(Extensions::IMPLICIT_SOME)
                .with_default_extension(Extensions::UNWRAP_NEWTYPES)
                .with_default_extension(Extensions::UNWRAP_VARIANT_NEWTYPES);
            let file = std::fs::File::open(config_path)
                .with_context(|| format!("Failed to open config at ({config_path:?})"))?;
            options
                .from_reader(file)
                .context("Failed to read ron config")
        }
        #[cfg(not(feature = "json5_config"))]
        Some("json") => {
            let file = std::fs::File::open(config_path)
                .with_context(|| format!("Failed to open config at ({config_path:?})"))?;
            serde_json::from_reader(file).context("Failed to read json config")
        }
        #[cfg(feature = "json5_config")]
        Some("json5") | Some("json") => {
            let file = std::fs::File::open(config_path)
                .with_context(|| format!("Failed to open config at ({config_path:?})"))?;
            serde_json5::from_reader(file).context("Failed to read json config")
        }
        Some(ext) => bail!(
            "Invalid config file extension: {} (run with -vv and check `FEATURES: ` debug log to see enabled extensions)",
            ext
        ),
    }.context("Failed to deserialize config")?;
    debug!("Loaded config");

    crate::check(&config)?;

    Ok(config)
}
