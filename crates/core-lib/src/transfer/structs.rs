use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransferType {
    /// send from the keybind to open the switch
    OpenSwitch(OpenSwitch),
    /// send from the keybinds like arrow keys or tab on switch
    SwitchSwitch(SwitchSwitchConfig),
    /// send from the gui itself when closing the switch
    CloseSwitch,
    /// send from the gui itself when closing a client (Blocking)
    CloseClientSwitch,
    /// send from pressing ESC
    CloseAll,
    /// send from the app itself when new monitor / config changes detected
    Restart,
    /// send from the app itself, 500ms after starting
    SetActive,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct OpenSwitch {
    pub reverse: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SwitchSwitchConfig {
    pub direction: Direction,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}
