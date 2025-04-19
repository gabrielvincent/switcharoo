use anyhow::Context;
use hyprland::data::Workspace;
use hyprland::dispatch::{
    Dispatch, DispatchType, WindowIdentifier, WorkspaceIdentifierWithSpecial,
};
use hyprland::prelude::*;
use hyprland::shared::{Address, WorkspaceId};
use tracing::{trace, warn};

pub async fn switch_workspace(workspace_id: WorkspaceId) -> anyhow::Result<()> {
    // check if already on workspace (if so, don't switch because it throws an error `Previous workspace doesn't exist`)
    let current_workspace = Workspace::get_active_async().await;
    if let Ok(workspace) = current_workspace {
        if workspace_id == workspace.id {
            trace!("Already on workspace {}", workspace_id);
            return Ok(());
        }
    }

    if workspace_id < 0 {
        warn!(
            "Special workspace id detected, not switching to special workspace, TODO not supported"
        );
    } else {
        switch_normal_workspace(workspace_id)
            .await
            .with_context(|| {
                format!(
                    "Failed to execute switch workspace with id {}",
                    workspace_id
                )
            })?;
    }
    Ok(())
}

pub async fn switch_client(address: &Address) -> anyhow::Result<()> {
    trace!("execute switch to next_client: {}", address);
    Dispatch::call_async(DispatchType::FocusWindow(WindowIdentifier::Address(
        address.clone(),
    )))
    .await?;
    Dispatch::call_async(DispatchType::BringActiveToTop).await?;
    Ok(())
}

async fn switch_normal_workspace(workspace_id: WorkspaceId) -> anyhow::Result<()> {
    trace!("execute switch to workspace {workspace_id}");
    Dispatch::call_async(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(
        workspace_id,
    )))
    .await?;
    Ok(())
}
