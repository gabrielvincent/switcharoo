use anyhow::Context;
use core_lib::Warn;
use core_lib::binds::generate_bind_kill;
use core_lib::config::Config;
use exec_lib::binds::{apply_exec_bind, apply_layerrules};
use tracing::{Level, span};

pub fn create_binds(config: &Config) -> anyhow::Result<()> {
    let _span = span!(Level::DEBUG, "create_binds").entered();
    if config.layerrules {
        apply_layerrules().warn("Failed to apply layerrules");
    }
    generate_bind_kill(&config.kill_bind)
        .and_then(|bind| apply_exec_bind(&bind))
        .context("Failed to apply kill bind")?;

    if let Some(windows) = &config.windows {
        for bind in windows_lib::generate_open_keybinds(windows) {
            apply_exec_bind(&bind).context("Failed to apply open keybinds for windows")?;
        }
    };
    Ok(())
}
