use crate::get_daemon_socket_path_buff;
use anyhow::Context;
use std::io::Write;
use std::os::unix::net::UnixStream;

pub fn send_raw_to_socket(data: &str) -> anyhow::Result<()> {
    let path = get_daemon_socket_path_buff();
    let mut socket = UnixStream::connect(&path)
        .with_context(|| format!("Can't connect to daemon socket {path:?}"))?;
    socket
        .write_all(data.as_bytes())
        .context("Can't send data to socket")?;
    Ok(())
}
