use crate::WindowsOverviewData;
use anyhow::{Context, anyhow};
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_millis(100);

pub fn close_client(data: &WindowsOverviewData) -> anyhow::Result<bool> {
    if let Some(id) = data.active.client {
        return exec_lib::kill::kill_client_blocking(id, TIMEOUT)
            .context("failed to kill active client");
    }
    Err(anyhow!("no active client"))
}
