use crate::get_daemon_socket_path_buff;
use crate::transfer::TransferType;
use anyhow::Context;
use std::io::Write;
use std::os::unix::net::UnixStream;

pub fn send_to_socket(transfer_type: &TransferType) -> anyhow::Result<()> {
    let path = get_daemon_socket_path_buff();
    let mut socket = UnixStream::connect(&path)
        .with_context(|| format!("Can't connect to daemon socket {path:?}"))?;
    serde_json::to_writer(&mut socket, &transfer_type)
        .context("Can't serialize/send transfer type")?;
    Ok(())
}

pub fn send_raw_to_socket(data: &str) -> anyhow::Result<()> {
    let path = get_daemon_socket_path_buff();
    let mut socket = UnixStream::connect(&path)
        .with_context(|| format!("Can't connect to daemon socket {path:?}"))?;
    socket
        .write_all(data.as_bytes())
        .context("Can't send data to socket")?;
    Ok(())
}
