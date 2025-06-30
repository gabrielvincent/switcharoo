use crate::Config;
use anyhow::bail;

pub fn check(config: &Config) -> anyhow::Result<()> {
    if config
        .windows
        .as_ref()
        .map(|w| w.scale >= 15f64 || w.scale <= 0f64)
        .unwrap_or(false)
    {
        bail!("Scale factor must be less than 15 and greater than 0");
    }

    if config
        .windows
        .as_ref()
        .and_then(|w| w.overview.as_ref())
        .map(|o| o.launcher.launch_modifier == o.modifier)
        .unwrap_or(false)
    {
        bail!(
            "Launcher modifier cannot be the same as overview open modifier. (pressing the modifier will just close the overview instead of launching an app)"
        );
    }

    if config
        .windows
        .as_ref()
        .and_then(|w| w.overview.as_ref())
        .map(|o| matches!(&*o.key, "super" | "alt" | "shift" | "control" | "ctrl"))
        .unwrap_or(false)
    {
        bail!(
            "If a modifier key is used to open it must include _l or _r at the end. (e.g. super_l, alt_r, etc)\nctrl_l / _r is NOT a valid modifier key, only control_l / _r is"
        );
    }

    if let Some(l) = &config
        .windows
        .as_ref()
        .and_then(|w| w.overview.as_ref().map(|o| &o.launcher))
    {
        let mut used: Vec<char> = vec![];
        for engine in l
            .plugins
            .websearch
            .as_ref()
            .map(|ws| &ws.engines)
            .unwrap_or(&vec![])
        {
            if engine.url.is_empty() {
                bail!("Search engine url cannot be empty");
            }
            if engine.name.is_empty() {
                bail!("Search engine name cannot be empty");
            }
            if used.contains(&engine.key) {
                bail!("Duplicate search engine key: {}", engine.key);
            } else {
                used.push(engine.key);
            }
        }
        if l.plugins.calc.is_some() {
            #[cfg(not(feature = "launcher_calc_plugin"))]
            {
                bail!("Calc Plugin enabled but not compiled in, please enable the calc feature");
            }
        }
    };

    Ok(())
}
