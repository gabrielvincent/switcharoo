use ashpd::WindowIdentifier;
use ashpd::desktop::global_shortcuts;
use futures_lite::stream::StreamExt;
use gtk4::gdk::Key;
use gtk4::glib::Propagation;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, ButtonExt, GtkWindowExt, WidgetExt};
use gtk4::{Application, ApplicationWindow, Button, EventControllerKey};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Application::builder()
        .application_id("org.example.GlobalShortcuts")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(300)
            .default_height(200)
            .title("Global Shortcuts")
            .build();

        let button = Button::with_label("Click me!");
        window.set_child(Some(&button));

        window.init_layer_shell();
        window.set_namespace(Some("aaaaaaaaaaaaaaaaaaaaaaaa"));
        window.set_layer(Layer::Top);
        window.set_margin(Edge::Top, 80);
        window.set_keyboard_mode(KeyboardMode::OnDemand);

        window.present();
        // window.set_visible(false);
        window.set_opacity(0.5);

        let key_controller = EventControllerKey::new();
        key_controller.connect_key_pressed(|_, key, _, _| handle_key(key));
        window.add_controller(key_controller);

        button.connect_clicked(move |_| {
            let value = window.clone();
            gtk4::glib::spawn_future_local(async move {
                let iden = WindowIdentifier::from_native(&value).await.unwrap();
                let portal = global_shortcuts::GlobalShortcuts::new().await.unwrap();

                let session_handle = portal.create_session().await.unwrap();
                println!("Created session: {:?}", session_handle);

                let mut stream = portal.receive_activated().await.unwrap();
                gtk4::glib::spawn_future_local(async move {
                    println!("Waiting for activation");
                    let a = stream.next().await.unwrap();
                    println!("Activated: {:?}", a);
                });

                portal
                    .bind_shortcuts(
                        &session_handle,
                        &[global_shortcuts::NewShortcut::new("open", "Open Overview")
                            .preferred_trigger("LOGO + a")],
                        Some(&iden),
                    )
                    .await
                    .unwrap();
                println!("Bound shortcuts");

                let mut signals = portal.receive_all_signals().await.unwrap();
                while let Some(signal) = StreamExt::next(&mut signals).await {
                    println!("Signal: {:?}", signal);
                }
            });
        });
    });

    app.run_with_args::<String>(&[]);
    Ok(())
}

fn handle_key(p0: Key) -> Propagation {
    // println!("Key pressed: {:?}", p0);
    match p0 {
        Key::Escape => {
            println!("Escape");
            Propagation::Stop
        }
        Key::Up => {
            println!("Up");
            Propagation::Stop
        }
        _ => Propagation::Proceed,
    }
}
