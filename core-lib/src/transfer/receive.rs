use crate::transfer::TransferType;
use anyhow::{Context, bail};
use tracing::{debug, trace};

pub fn receive_from_buffer(buffer: &[u8]) -> anyhow::Result<TransferType> {
    if buffer.is_empty() {
        bail!("Received empty buffer");
    }
    let str = serde_json::from_str(&String::from_utf8_lossy(&buffer))
        .with_context(|| format!("Failed to deserialize str: {str:?}"))?;
    let transfer: TransferType =
        serde_json::from_str(str).with_context(|| format!("Failed to deserialize str: {str:?}"))?;
    debug!("Received command: {transfer:?}");
}
