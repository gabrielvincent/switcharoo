use crate::util::to_client_address;
use anyhow::Context;
use core_lib::ClientId;
use hyprland::data::Clients;
use hyprland::dispatch::{Dispatch, DispatchType, WindowIdentifier};
use hyprland::prelude::HyprData;
use std::thread;
use std::time::Duration;
use tracing::instrument;

/// Sends a close window request to hyprland and waits for the client to be killed
///
/// Returns true if the client was killed successfully, false if close was interrupted
#[instrument(level = "debug", ret(level = "trace"))]
pub fn kill_client_blocking(address: ClientId, timeout: Duration) -> anyhow::Result<bool> {
    Dispatch::call(DispatchType::CloseWindow(WindowIdentifier::Address(
        to_client_address(address),
    )))?;
    thread::sleep(timeout);
    let client = Clients::get()
        .context("get clients failed")?
        .into_iter()
        .find(|c| c.address == to_client_address(address));

    Ok(client.is_none())
}
