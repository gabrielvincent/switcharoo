use crate::structs::ConfigModifier;
use crate::util::{handle_key, mod_key_to_string};
use relm4::adw::prelude::*;
use relm4::gtk::{EventControllerKey, Label};
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmWidgetExt,
    SimpleComponent,
};
use relm4::{adw, gtk};
use relm4_components::alert::{Alert, AlertMsg, AlertResponse, AlertSettings};
use tracing::{debug, warn};

#[derive(Debug)]
pub struct KeyboardShortcut {
    key: Option<String>,
    modifier: Option<ConfigModifier>,
    is_visible: bool,
    dialog: Controller<Alert>,
    entry: Label,
    label: Option<String>,
}

#[derive(Debug)]
pub enum KeyboardShortcutInput {
    UpdateKey(String),
    UpdateModifier(ConfigModifier),
    HideKeyboardShortcut(bool),
    ShowKeyboardShortcutDialog(Option<(ConfigModifier, String)>, Option<gtk::Widget>),
    SetLabelText(Option<String>),
}

#[derive(Debug)]
pub struct KeyboardShortcutInit {
    pub label: Option<String>,
    pub icon: Option<String>,
    pub init: Option<(ConfigModifier, String)>,
}

#[derive(Debug)]
pub enum KeyboardShortcutOutput {
    SetKey(ConfigModifier, String),
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
            set_icon_name?: &init.icon,
            #[watch]
            set_label?: &model.label,
            #[watch]
            set_css_classes: if model.is_visible { &["active"] } else { &["not-active"] },
            connect_clicked[sender] => move |_b| { sender.output(KeyboardShortcutOutput::OpenRequest).unwrap(); },
        },
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        use relm4::ComponentController;

        let entry = Label::new(None);
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
            key: init.init.as_ref().map(|(_, key)| key.clone()),
            modifier: init.init.map(|(r#mod, _)| r#mod.clone()),
            is_visible: false,
            entry: entry_2,
            dialog,
            label: init.label.clone(),
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            KeyboardShortcutInput::UpdateKey(key) => {
                self.key = Some(key);
            }
            KeyboardShortcutInput::UpdateModifier(modifier) => {
                self.modifier = Some(modifier);
            }
            KeyboardShortcutInput::HideKeyboardShortcut(confirm) => {
                self.is_visible = false;
                if confirm {
                    if let (Some(key), Some(r#mod)) = (&self.key, &self.modifier) {
                        sender
                            .output(KeyboardShortcutOutput::SetKey(r#mod.clone(), key.clone()))
                            .unwrap();
                    } else {
                        warn!("Tried to hide keyboard shortcut dialog without key or modifier");
                        sender.output(KeyboardShortcutOutput::Abort).unwrap();
                    }
                } else {
                    sender.output(KeyboardShortcutOutput::Abort).unwrap();
                }
            }
            KeyboardShortcutInput::ShowKeyboardShortcutDialog(initial, root) => {
                if let Some(root) = root {
                    self.dialog
                        .widget()
                        .set_transient_for(root.toplevel_window().as_ref());
                }
                self.is_visible = true;
                self.key = initial.as_ref().map(|(_, key)| key.clone());
                self.modifier = initial.map(|(r#mod, _)| r#mod.clone());
                if let (Some(r#mod), Some(key)) = (&self.modifier, &self.key) {
                    self.entry.set_text(&mod_key_to_string(r#mod, key));
                } else {
                    self.entry.set_text("");
                }
                self.dialog.emit(AlertMsg::Show);
                self.dialog.widgets().gtk_window_12.set_modal(true); // TODO remove if https://github.com/Relm4/Relm4/issues/837 fixed
            }
            KeyboardShortcutInput::SetLabelText(text) => {
                self.label = text;
            }
        }
    }
}
