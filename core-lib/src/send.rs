use crate::get_daemon_socket_path_buff;
use crate::transfer::{to_ron_string, TransferType};
use anyhow::Context;
use std::io::Write;
use std::os::unix::net::UnixStream;

pub fn send_to_socket(transfer_type: &TransferType) -> anyhow::Result<()> {
    let path = get_daemon_socket_path_buff();
    let mut socket = UnixStream::connect(&path)
        .with_context(|| format!("Can't connect to daemon socket {path:?}"))?;
    socket
        .write_all(
            to_ron_string(transfer_type)
                .context("Can't serialize transfer type")?
                .as_bytes(),
        )
        .context("Can't write transfer type")?;
    Ok(())
}
