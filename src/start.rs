use crate::keybinds::create_binds;
use crate::receive::{Globals, socket_handler};
use async_channel::{Receiver, Sender};
use core_lib::theme_icon_cache::init_icon_map;
use core_lib::transfer::TransferType;
use core_lib::{
    Warn, collect_desktop_files, config, hyprshell_config_block, hyprshell_config_listener,
    hyprshell_css_listener,
};
use exec_lib::listener::{hyprland_config_listener, monitor_listener};
use exec_lib::{reload_config, reset_remain_focused, reset_submap, toast};
use gtk::gdk::Display;
use gtk::glib::ControlFlow;
use gtk::prelude::*;
use gtk::{
    Application, CssProvider, IconTheme, STYLE_PROVIDER_PRIORITY_APPLICATION,
    STYLE_PROVIDER_PRIORITY_USER, Settings, glib, style_context_add_provider_for_display,
};
use launcher_lib::{LauncherGlobal, create_launcher_window};
use std::env;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};
use tracing::{Level, debug, info, span, trace, warn};
use windows_lib::{WindowsGlobal, create_windows_window};

const APPLICATION_ID: &str = "com.github.h3rmt.hyprshell";
const SIGTERM: i32 = 15; // Signal number for SIGTERM

pub fn start(config_path: PathBuf, css_path: PathBuf, data_dir: PathBuf) -> anyhow::Result<()> {
    let _span = span!(Level::TRACE, "start").entered();
    let config_path = Rc::new(config_path);
    let css_path = Rc::new(css_path);
    let data_dir = Rc::new(data_dir);

    fill_icon_map(true);
    check_themes();
    gtk_handle_sigterm();

    let desktop_files = collect_desktop_files();
    windows_lib::reload_desktop_map(&desktop_files);
    launcher_lib::reload_applications_desktop_map(&desktop_files);
    launcher_lib::reload_search_default_browser(&desktop_files);

    let (event_sender, event_receiver) = async_channel::unbounded();
    let (restart_sender, restart_receiver) = async_channel::bounded(1);

    if env::var_os("HYPRSHELL_NO_LISTENERS").is_none() {
        // delay for 1 second to allow the config to be reloaded before listening for reload
        let config_path = config_path.clone();
        let css_path = css_path.clone();
        let delay = env::var("HYPRSHELL_RELOAD_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1500);
        glib::timeout_add_local_once(Duration::from_millis(delay), move || {
            setup_restart_listener(&config_path, &css_path, restart_sender);
        });
        glib::spawn_future_local(async move {
            let mut last_send = Instant::now();
            loop {
                let cause = restart_receiver.recv().await.unwrap_or_default();
                let now = Instant::now();
                if now.duration_since(last_send) < Duration::from_millis(delay) {
                    debug!("Ignoring restart request ({cause}) too soon after last send");
                    continue;
                }
                info!("Restarting gui ({cause})");
                event_sender
                    .send(&TransferType::Restart)
                    .warn("unable to send restart");
                last_send = now;
            }
        });
    }

    loop {
        let application = Application::builder()
            .application_id(APPLICATION_ID.to_string())
            .build();

        let config_path = config_path.clone();
        let css_path = css_path.clone();
        let data_dir = data_dir.clone();
        let event_sender = event_sender.clone();
        let event_receiver = event_receiver.clone();
        application.connect_activate(move |app| {
            activate(
                app,
                &config_path,
                &css_path,
                &data_dir,
                event_sender.clone(),
                event_receiver.clone(),
            )
        });
        application.run_with_args::<String>(&[]);
    }
}
fn activate(
    app: &Application,
    config_path: &Path,
    css_path: &Path,
    data_dir: &Path,
    event_sender: Sender<TransferType>,
    event_receiver: Receiver<TransferType>,
) {
    let _span = span!(Level::TRACE, "activate").entered();

    let config = match config::load_config(config_path) {
        Ok(config) => config,
        Err(err) => {
            toast(&format!("Failed to load config: {:?}", err));
            hyprshell_config_block(config_path);
            return; // return needed to exit the application
        }
    };

    if let Err(err) = create_binds(&config) {
        warn!("Failed to create keybinds: {err}");
        toast(&format!("Failed to create keybinds: {err}"));
        return; // return needed to exit the application
    }

    apply_css(css_path);

    let windows_data: Option<WindowsGlobal> = config.windows.map(WindowsGlobal::new);
    let mut launcher_data: Option<LauncherGlobal> =
        config.launcher.map(LauncherGlobal::new(data_dir));

    if let Some(windows_data) = &windows_data {
        create_windows_window(app, windows_data)
            .unwrap_or_else(|e| toast(&format!("Failed to create windows window(s): {e}")));
    } else {
        debug!("Overview is disabled");
    }

    if let Some(launcher_data) = &mut launcher_data {
        create_launcher_window(app, launcher_data)
            .unwrap_or_else(|e| toast(&format!("Failed to create launcher window: {e}")));
    } else {
        debug!("Launcher is disabled");
    }

    let globals = Globals {
        window: windows_data,
        launcher: launcher_data,
    };
    glib::spawn_future_local(async move {
        socket_handler(globals).await;
    });

    debug!("Application initialized");
}
static WATCHERS: OnceLock<Mutex<Vec<Box<dyn std::any::Any + Send>>>> = OnceLock::new();

fn setup_restart_listener(
    config_path: &Path,
    css_path: &Path,
    restart_tx: async_channel::Sender<&'static str>,
) {
    let tx = restart_tx.clone();
    if let Some(watcher) = hyprshell_config_listener(config_path, move |mess| {
        let _ = tx.send_blocking(mess);
    }) {
        WATCHERS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("Failed to lock watchers")
            .push(Box::new(watcher));
    };
    let tx = restart_tx.clone();
    if let Some(watcher) = hyprshell_css_listener(css_path, move |mess| {
        let _ = tx.send_blocking(mess);
    }) {
        WATCHERS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("Failed to lock watchers")
            .push(Box::new(watcher));
    };

    let tx = restart_tx.clone();
    glib::spawn_future_local(async move {
        monitor_listener(move |mess| {
            let _ = tx.send_blocking(mess);
        })
        .await;
    });
    let tx = restart_tx.clone();
    glib::spawn_future_local(async move {
        hyprland_config_listener(move |mess| {
            let _ = tx.send_blocking(mess);
        })
        .await;
    });
}

fn apply_css(custom_css: &Path) {
    let provider_app = CssProvider::new();
    provider_app.load_from_bytes(&glib::Bytes::from_static(include_bytes!(
        "default-styles.css"
    )));
    style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider_app,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    windows_lib::get_css();
    launcher_lib::get_css();

    if !custom_css.exists() {
        info!("Custom css file {custom_css:?} does not exist");
    } else {
        debug!("Loading custom css file {custom_css:?}");
        let provider_user = CssProvider::new();
        provider_user.load_from_path(custom_css);
        style_context_add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider_user,
            STYLE_PROVIDER_PRIORITY_USER,
        );
    }
}

fn check_themes() {
    if let Some(settings) = Settings::default() {
        let theme_name = settings.gtk_theme_name();
        let icon_theme_name = settings.gtk_icon_theme_name();
        info!(
            "Using theme: {theme_name:?} and icon theme: {icon_theme_name:?}, please make sure both exist, else weird icon or graphical issues may occur"
        );
    } else {
        warn!("Unable to check default settings for icon theme");
    }
}

pub fn fill_icon_map(threads: bool) {
    gtk::init().expect("Failed to initialize GTK");
    let icon_theme = IconTheme::new();
    let gtk_icons = icon_theme
        .icon_names()
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let search_path = icon_theme
        .search_path()
        .into_iter()
        .filter(|path| path.exists())
        .collect::<Vec<_>>();
    trace!("Icon theme search path: {search_path:?}");
    init_icon_map(gtk_icons, search_path, threads);
}

pub fn gtk_handle_sigterm() {
    glib::unix_signal_add(SIGTERM, || {
        info!("Received SIGTERM, exiting gracefully");
        reset_submap().warn("Failed to reset submap on SIGTERM");
        reset_remain_focused().warn("Failed to reset follow mouse on SIGTERM");
        // Continue with the default SIGTERM handler after cleanup
        unsafe {
            libc::signal(SIGTERM, libc::SIG_DFL);
            libc::raise(SIGTERM);
        }
        ControlFlow::Continue
    });
}
