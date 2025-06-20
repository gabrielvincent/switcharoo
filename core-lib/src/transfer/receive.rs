use crate::transfer::TransferType;
use anyhow::{Context, bail};
use tracing::debug;

pub fn receive_from_buffer(buffer: Vec<u8>) -> anyhow::Result<TransferType> {
    if buffer.is_empty() {
        bail!("Received empty buffer");
    }
    // TODO use this if newer rust version
    // let str =
    //     str::from_utf8(buffer).with_context(|| format!("Failed to convert buffer: {buffer:?}"))?;
    let str = String::from_utf8(buffer).context("Failed to convert buffer")?;
    let transfer: TransferType = serde_json::from_str(&str)
        .with_context(|| format!("Failed to deserialize str: {str:?}"))?;
    debug!("Received command: {transfer:?}");
    Ok(transfer)
}
