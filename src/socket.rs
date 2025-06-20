use crate::recive_handle::{close, exit, open_overview, open_switch, restart, switch, r#type};
use crate::start::Globals;
use anyhow::Context;
use async_channel::{Receiver, Sender};
use core_lib::transfer::TransferType;
use core_lib::{get_daemon_socket_path_buff, transfer};
use exec_lib::toast;
use gtk::gio::{Cancellable, InputStream, SocketListener, UnixSocketAddress};
use gtk::prelude::*;
use gtk::{gio, glib};
use rand::Rng;
use std::fs::remove_file;
use std::time::Instant;
use tracing::{Level, debug, error, info, span, trace};

pub async fn socket_handler(event_sender: Sender<TransferType>) {
    let buf = get_daemon_socket_path_buff();
    let path = buf.as_path();
    let listener = {
        // remove old PATH
        if path.exists() {
            remove_file(path).expect("Unable to remove old socket file");
        }
        let socket = SocketListener::new();
        socket
            .add_address(
                &UnixSocketAddress::new(path),
                gio::SocketType::Stream,
                gio::SocketProtocol::Default,
                None::<&glib::Object>,
            )
            .unwrap_or_else(|_| panic!("Failed to bind to socket {path:?}"));
        socket
    };
    info!("Starting socket on {path:?}");

    loop {
        let path = listener.accept_future().await;
        match path {
            Ok((conn, _)) => {
                handle_client(
                    conn.input_stream(),
                    conn.socket().available_bytes(),
                    &event_sender,
                )
                .context("Failed to handle client")
                .unwrap_or_else(|e| {
                    toast(&format!("Failed to handle connection {:?}", e));
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {e}");
            }
        };
    }
}

fn handle_client(
    stream: InputStream,
    size: isize,
    event_sender: &Sender<TransferType>,
) -> anyhow::Result<()> {
    let mut buffer = vec![0; size as usize];
    stream
        .read(&mut buffer, None::<&Cancellable>)
        .context("Failed to read data from buffer")?;
    let transfer =
        transfer::receive_from_buffer(&buffer).context("Failed to receive from buffer")?;
    event_sender
        .send_blocking(transfer)
        .context("Failed to send transfer")?;
    Ok(())
}
