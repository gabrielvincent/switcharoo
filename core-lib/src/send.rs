use crate::transfer::TransferType;
use crate::get_daemon_socket_path_buff;
use anyhow::Context;
use std::os::unix::net::UnixStream;

pub fn send_to_socket(transfer_type: &TransferType) -> anyhow::Result<()> {
    let path = get_daemon_socket_path_buff();
    let mut socket = UnixStream::connect(&path)
        .with_context(|| format!("Can't connect to daemon socket {path:?}"))?;
    serde_json::to_writer(&mut socket, &transfer_type)
        .context("Can't serialize/send transfer type")?;
    Ok(())
}
