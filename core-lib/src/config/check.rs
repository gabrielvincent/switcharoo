use crate::config::{Config, Plugin};
use anyhow::bail;
use std::mem;

pub fn check(config: &Config) -> anyhow::Result<()> {
    if config
        .windows
        .as_ref()
        .map(|w| w.scale >= 15f64 || w.scale <= 0f64)
        .unwrap_or(false)
    {
        bail!("Scale factor must be less than 15 and greater than 0");
    }

    if let Some(l) = &config.launcher {
        let mut active_plugins: Vec<&Plugin> = vec![];
        for plugin in &l.plugins {
            if active_plugins
                .iter()
                .any(|p| mem::discriminant(*p) == mem::discriminant(plugin))
            {
                bail!("Duplicate plugin: {:?}", plugin);
            } else {
                active_plugins.push(plugin);
            }

            if let Plugin::WebSearch(config) = plugin {
                let mut used: Vec<char> = vec![];
                for engine in config {
                    if used.contains(&engine.key) {
                        bail!("Duplicate search engine key: {}", engine.key);
                    } else {
                        used.push(engine.key);
                    }
                }
            }
            if let Plugin::Calc(_) = plugin {
                #[cfg(not(feature = "calc"))]
                {
                    bail!(
                        "Calc Plugin enabled but not compiled in, please enable the calc feature"
                    );
                }
            }
        }
    };

    Ok(())
}
