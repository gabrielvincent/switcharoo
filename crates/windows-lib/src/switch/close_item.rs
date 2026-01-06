use crate::global::WindowsSwitchData;
use anyhow::{Context, Ok, anyhow};
use core_lib::FindByFirst;
use std::time::Duration;
use tracing::debug;

const TIMEOUT: Duration = Duration::from_millis(100);

pub fn close_item(data: &WindowsSwitchData) -> anyhow::Result<bool> {
    if data.config.switch_workspaces {
        kill_switch_workspace(data)
    } else {
        kill_switch_client(data)
    }
}

fn kill_switch_client(data: &WindowsSwitchData) -> anyhow::Result<bool> {
    if let Some(id) = data.active.client {
        return exec_lib::kill::kill_client_blocking(id, TIMEOUT)
            .context("failed to kill active client");
    }
    Err(anyhow!("no active client"))
}

fn kill_switch_workspace(data: &WindowsSwitchData) -> anyhow::Result<bool> {
    let workspace_id = data.active.workspace;
    debug!(
        "Killing all clients in workspace {}",
        data.hypr_data
            .workspaces
            .find_by_first(&workspace_id)
            .map_or_else(|| workspace_id.to_string(), |w| w.name.clone())
    );

    let clients_to_kill: Vec<_> = data
        .hypr_data
        .clients
        .iter()
        .filter(|(_, client)| client.workspace == workspace_id && client.enabled)
        .map(|(id, _)| *id)
        .collect();

    for client_id in clients_to_kill {
        if !exec_lib::kill::kill_client_blocking(client_id, TIMEOUT)
            .context("failed to kill client while killing workspace")?
        {
            debug!("failed to kill client {}", client_id);
            return Ok(false);
        }
    }

    Ok(true)
}
