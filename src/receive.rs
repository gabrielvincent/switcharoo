use crate::recive_handle::{close, exit, open_overview, open_switch, restart, switch, r#type};
use anyhow::Context;
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

pub struct Globals {
    pub window: Option<windows_lib::WindowsGlobal>,
    pub launcher: Option<launcher_lib::LauncherGlobal>,
}

pub async fn socket_handler(global: Globals) {
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
                let exit = handle_client(
                    conn.input_stream(),
                    conn.socket().available_bytes(),
                    &global,
                )
                .context("Failed to handle client")
                .unwrap_or_else(|e| {
                    toast(&format!("Failed to handle connection {:?}", e));
                    false
                });
                if exit {
                    debug!("Exiting socket handler");
                    break;
                }
            }
            Err(e) => {
                error!("Failed to accept connection: {e}");
            }
        };
    }
}

fn handle_client(stream: InputStream, size: isize, global: &Globals) -> anyhow::Result<bool> {
    let now = Instant::now();
    let rand_id = rand::rng().random_range(100..=255);
    let _span = span!(Level::TRACE, "handle_client", id = rand_id).entered();

    let mut buffer = vec![0; size as usize];
    stream
        .read(&mut buffer, None::<&Cancellable>)
        .context("Failed to read data from buffer")?;

    let transfer = transfer::receive_from_buffer(&buffer)?;
    let exit = handle_client_transfer(transfer, global)?;

    trace!("Handled client in {:?}", now.elapsed());
    Ok(exit)
}

fn handle_client_transfer(transfer: TransferType, global: &Globals) -> anyhow::Result<bool> {
    let close_socket = matches!(transfer, TransferType::Restart);
    match transfer {
        TransferType::OpenOverview(config) => open_overview(global, config),
        TransferType::OpenSwitch(config) => open_switch(global, config),
        TransferType::Switch(config) => switch(global, config),
        TransferType::Exit => exit(global),
        TransferType::Type(text) => r#type(global, text),
        TransferType::Close(config) => close(global, config),
        TransferType::Restart => restart(global),
    }
    Ok(close_socket)
}
