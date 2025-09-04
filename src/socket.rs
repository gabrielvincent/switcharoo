use anyhow::{Context, bail};
use async_channel::Sender;
use core_lib::transfer::TransferType;
use core_lib::{get_daemon_socket_path_buff, transfer};
use std::fs::remove_file;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net;
use std::os::unix::net::UnixStream;
use tracing::{debug_span, info, warn};

pub fn socket_handler(event_sender: &Sender<TransferType>) {
    let _span = debug_span!("socket_handler").entered();
    let buf = get_daemon_socket_path_buff();
    let path = buf.as_path();
    let listener = {
        // remove old PATH
        if path.exists() {
            remove_file(path).expect("Unable to remove old socket file");
        }
        net::UnixListener::bind(path)
            .unwrap_or_else(|_| panic!("Failed to bind to socket {}", path.display()))
    };
    info!("Starting socket on {path:?}");

    loop {
        let path = listener.accept();
        match path {
            Ok((conn, _)) => {
                handle_client(conn, event_sender)
                    .context("Failed to handle client")
                    .unwrap_or_else(|e| {
                        warn!("Failed to handle connection {e:?}");
                    });
            }
            Err(e) => {
                warn!("Failed to accept connection: {e:?}");
            }
        }
    }
}

fn handle_client(
    mut stream: UnixStream,
    event_sender: &Sender<TransferType>,
) -> anyhow::Result<()> {
    let _span = debug_span!("handle_client").entered();
    let mut buffer = vec![];
    let mut reader = BufReader::new(&mut stream);
    reader
        .read_until(b'\0', &mut buffer)
        .context("Can't read data from socket")?;
    if buffer.is_empty() {
        return Ok(());
    }
    match transfer::receive_from_buffer(buffer) {
        Ok(transfer) => {
            event_sender
                .send_blocking(transfer)
                .context("Failed to send transfer")?;
            let _ = stream
                .write_all(b"OK")
                .and_then(|()| stream.write_all(b"\0"));
        }
        Err(err) => {
            let _ = stream
                .write_all(b"ERR")
                .and_then(|()| stream.write_all(b"\0"));
            bail!("Invalid transfer received.\n{err:?}");
        }
    }

    Ok(())
}
