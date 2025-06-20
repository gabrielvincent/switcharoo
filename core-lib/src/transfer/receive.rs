use crate::transfer::TransferType;
use anyhow::{Context, bail};
use tracing::debug;

pub fn receive_from_buffer(buffer: &[u8]) -> anyhow::Result<TransferType> {
    if buffer.is_empty() {
        bail!("Received empty buffer");
    }
    let str =
        str::from_utf8(buffer).with_context(|| format!("Failed to convert buffer: {buffer:?}"))?;
    let transfer: TransferType =
        serde_json::from_str(str).with_context(|| format!("Failed to deserialize str: {str:?}"))?;
    debug!("Received command: {transfer:?}");
    Ok(transfer)
}
