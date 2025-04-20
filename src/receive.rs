use anyhow::Context;
use core_lib::transfer::{from_ron_string, TransferType};
use core_lib::{get_daemon_socket_path_buff, Warn};
use exec_lib::toast;
use gtk::gio::{Cancellable, InputStream, SocketListener, UnixSocketAddress};
use gtk::prelude::{
    EntryExt, IOStreamExt, InputStreamExtManual, SocketConnectionExt, SocketExt, SocketListenerExt,
    WidgetExt,
};
use gtk::{gio, glib};
use rand::Rng;
use std::fs::remove_file;
use std::path::Path;
use std::time::Instant;
use tracing::{debug, error, info, span, trace, Level};
use windows_lib::{
    close_overview, open_overview, open_switch, stop_overview, update_overview, WindowsGlobal,
};

pub struct Globals {
    pub window: Option<WindowsGlobal>,
    #[cfg(feature = "launcher")]
    pub launcher: Option<launcher_lib::LauncherGlobal>,
    #[cfg(any(feature = "launcher", feature = "bar"))]
    pub cache_path: Box<Path>,
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
                handle_client(
                    conn.input_stream(),
                    conn.socket().available_bytes(),
                    &global,
                )
                .await
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

async fn handle_client(stream: InputStream, size: isize, global: &Globals) -> anyhow::Result<()> {
    let now = Instant::now();
    let rand_id = rand::rng().random_range(100..=255);
    let _span = span!(Level::TRACE, "handle_client", id = rand_id).entered();

    let mut buffer = vec![0; size as usize];
    stream
        .read(&mut buffer, None::<&Cancellable>)
        .context("Failed to read data from buffer")?;

    // client checked if socket is OK
    if buffer.is_empty() {
        trace!("Received empty buffer");
        return Ok(());
    }
    let str = String::from_utf8_lossy(&buffer);
    handle_client_transfer(&str, global).await?;
    trace!("Handled client in {:?}", now.elapsed());
    Ok(())
}

async fn handle_client_transfer(buffer: &str, global: &Globals) -> anyhow::Result<()> {
    let transfer: TransferType = from_ron_string(buffer)
        .with_context(|| format!("Failed to deserialize buffer: {buffer:?}"))?;
    debug!("Received command: {transfer:?}");

    handle_transfer(transfer, global)
        .await
        .warn("Failed to handle transfer");
    Ok(())
}

async fn handle_transfer(transfer: TransferType, global: &Globals) -> anyhow::Result<()> {
    match transfer {
        TransferType::OpenOverview(config) => {
            if let Some(global) = &global.window {
                open_overview(config, global).await?;
            } else {
                return Err(anyhow::anyhow!("No window global data available"));
            };
            #[cfg(feature = "launcher")]
            if let Some(global) = &global.launcher {
                launcher_lib::open_launcher(global).await?;
            }
        }
        TransferType::OpenSwitch(config) => {
            if let Some(global) = &global.window {
                open_switch(config, global).await?;
            } else {
                return Err(anyhow::anyhow!("No window global data available"));
            }
        }
        TransferType::Switch(config) => {
            if let Some(global) = &global.window {
                update_overview(config, global).await?;
            } else {
                return Err(anyhow::anyhow!("No window global data available"));
            }
        }
        TransferType::Close => {
            if let Some(global) = &global.window {
                close_overview(true, global).await;
            }
            #[cfg(feature = "launcher")]
            if let Some(l_global) = &global.launcher {
                launcher_lib::close_launcher(None, l_global, &global.cache_path).await;
            }
        }
        TransferType::Return(config) => {
            #[cfg(not(feature = "launcher"))]
            if let Some(global) = &global.window {
                close_overview(false, global).await;
            }
            #[cfg(feature = "launcher")]
            {
                if let Some(l_global) = &global.launcher {
                    let launch = l_global
                        .data
                        .as_ref()
                        .map(|d| {
                            let b = d.borrow();
                            b.entry.text_length() > 0 && b.results.first_child().is_some()
                        })
                        .unwrap_or(false);

                    if launch {
                        // kill overview, launch program
                        if let Some(global) = &global.window {
                            close_overview(true, global).await;
                        }
                        launcher_lib::close_launcher(
                            Some(config.offset),
                            l_global,
                            &global.cache_path,
                        )
                        .await;
                    } else {
                        // close overview, kill launcher
                        if let Some(global) = &global.window {
                            close_overview(false, global).await;
                        }
                        launcher_lib::close_launcher(None, l_global, &global.cache_path).await;
                    };
                } else if let Some(global) = &global.window {
                    close_overview(false, global).await;
                }
            }
        }
        TransferType::Restart => {
            if let Some(global) = &global.window {
                stop_overview(global).await;
            }
            #[cfg(feature = "launcher")]
            if let Some(global) = &global.launcher {
                launcher_lib::stop_launcher(global).await;
            }
        }
    }
    Ok(())
}
