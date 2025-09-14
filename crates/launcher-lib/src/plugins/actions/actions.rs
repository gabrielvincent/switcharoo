#![allow(clippy::unnecessary_wraps)]

#[cfg(not(debug_assertions))]
use anyhow::Context;
use anyhow::Result;
use tracing::debug;

pub fn lock_session() -> Result<()> {
    debug!("loginctl lock-session");
    #[cfg(not(debug_assertions))]
    exec_lib::run::run_program("loginctl lock-session", None, false, None)
        .context("Failed to lock session")?;
    Ok(())
}

pub fn hibernate() -> Result<()> {
    debug!("systemctl hibernate");
    #[cfg(not(debug_assertions))]
    exec_lib::run::run_program("systemctl hibernate", None, false, None)
        .context("Failed to hibernate")?;
    Ok(())
}

pub fn shutdown() -> Result<()> {
    debug!("systemctl poweroff");
    #[cfg(not(debug_assertions))]
    exec_lib::run::run_program("systemctl poweroff", None, false, None)
        .context("Failed to shutdown")?;
    Ok(())
}

pub fn reboot() -> Result<()> {
    debug!("systemctl reboot");
    #[cfg(not(debug_assertions))]
    exec_lib::run::run_program("systemctl reboot", None, false, None)
        .context("Failed to reboot")?;
    Ok(())
}

pub fn logout() -> Result<()> {
    debug!("loginctl terminate-user $USER");
    #[cfg(not(debug_assertions))]
    exec_lib::run::run_program("loginctl terminate-user $USER", None, false, None)
        .context("Failed to logout")?;
    Ok(())
}

pub fn suspend() -> Result<()> {
    debug!("systemctl suspend");
    #[cfg(not(debug_assertions))]
    exec_lib::run::run_program("systemctl suspend", None, false, None)
        .context("Failed to suspend")?;
    Ok(())
}
