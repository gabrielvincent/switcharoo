use crate::structs::ConfigModifier;
use crate::util::{handle_key, key_to_name};
use adw::gtk;
use adw::prelude::*;
use relm4::gtk::{EventControllerKey, Label};
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, SimpleComponent,
};
use relm4_components::alert::{Alert, AlertMsg, AlertResponse, AlertSettings};
use tracing::debug;

#[derive(Debug)]
pub struct KeyboardShortcut {
    key: String,
    modifier: ConfigModifier,
    is_visible: bool,
    dialog: Controller<Alert>,
    pub entry: Label,
}

#[derive(Debug)]
pub enum KeyboardShortcutInput {
    UpdateKey(String),
    UpdateModifier(ConfigModifier),
    HideKeyboardShortcut(bool),
    ShowKeyboardShortcut(String, ConfigModifier),
}

#[derive(Debug)]
pub struct KeyboardShortcutInit {
    pub key: String,
    pub modifier: ConfigModifier,
}

#[derive(Debug)]
pub enum KeyboardShortcutOutput {
    SetKey(String, ConfigModifier),
    Abort,
    OpenRequest,
}

#[relm4::component(pub)]
impl SimpleComponent for KeyboardShortcut {
    type Init = KeyboardShortcutInit;
    type Input = KeyboardShortcutInput;
    type Output = KeyboardShortcutOutput;

    view! {
        #[root]
            gtk::Button {
            set_icon_name: "keyboard-layout",
            #[watch]
            set_css_classes: if model.is_visible { &["active"] } else { &["not-active"] },
            connect_clicked[sender] => move |_| { sender.output(KeyboardShortcutOutput::OpenRequest).unwrap(); },
        },
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        use relm4::ComponentController;

        let entry = gtk::Label::new(None);
        let dialog = Alert::builder()
            .transient_for(&root)
            .launch(AlertSettings {
                text: Some("Press Keyboard shortcut".to_string()),
                secondary_text: None,
                confirm_label: Some("Use".to_string()),
                cancel_label: Some("Cancel".to_string()),
                option_label: None,
                is_modal: true,
                destructive_accept: false,
                extra_child: Some(entry.clone().into()),
            })
            .forward(sender.input_sender(), |res| match res {
                AlertResponse::Confirm => KeyboardShortcutInput::HideKeyboardShortcut(true),
                AlertResponse::Cancel => KeyboardShortcutInput::HideKeyboardShortcut(false),
                AlertResponse::Option => unreachable!("no option button in alert dialog"),
            });

        // Attach an EventControllerKey to the alert dialog's window to print raw key events.
        let key_controller = EventControllerKey::new();
        let entry = entry.clone();
        let entry_2 = entry.clone();
        let window = dialog.widgets().gtk_window_12.clone();
        let send = sender.clone();
        key_controller.connect_key_pressed(move |_, val, id, state| {
            debug!("Raw key event - val: {}, state: {:?}", val, state);
            match handle_key(val, state, id) {
                Some((key, r#mod, label)) => {
                    entry.set_text(&label);
                    send.input(KeyboardShortcutInput::UpdateKey(key));
                    send.input(KeyboardShortcutInput::UpdateModifier(r#mod));
                }
                None => {
                    entry.set_text("---");
                }
            }
            gtk::glib::Propagation::Stop
        });
        window.add_controller(key_controller);

        let model = KeyboardShortcut {
            key: init.key,
            modifier: init.modifier,
            is_visible: false,
            entry: entry_2,
            dialog,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            KeyboardShortcutInput::UpdateKey(key) => {
                self.key = key;
            }
            KeyboardShortcutInput::UpdateModifier(modifier) => {
                self.modifier = modifier;
            }
            KeyboardShortcutInput::HideKeyboardShortcut(confirm) => {
                self.is_visible = false;
                if confirm {
                    sender
                        .output(KeyboardShortcutOutput::SetKey(
                            self.key.clone(),
                            self.modifier.clone(),
                        ))
                        .unwrap();
                } else {
                    sender.output(KeyboardShortcutOutput::Abort).unwrap();
                }
            }
            KeyboardShortcutInput::ShowKeyboardShortcut(key, r#mod) => {
                self.is_visible = true;
                self.key = key;
                self.modifier = r#mod;
                let label = if self.modifier == ConfigModifier::None {
                    key_to_name(&self.key).unwrap_or("---".to_string())
                } else {
                    format!(
                        "{} + {}",
                        self.modifier,
                        key_to_name(&self.key).unwrap_or("---".to_string())
                    )
                };
                self.entry.set_text(&label);
                self.dialog.emit(AlertMsg::Show);
            }
        }
    }
}
