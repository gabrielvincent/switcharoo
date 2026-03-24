use crate::transfer::TransferType;
use std::env;

/// # Panics
/// if the current executable couldn't be found
#[must_use]
pub fn get_switcharoo_path() -> String {
    env::current_exe()
        .expect("Current executable not found")
        .display()
        .to_string()
        .replace("(deleted)", "")
}

/// # Panics
/// if the transfer could not be serialized into a string
#[must_use]
pub fn generate_transfer_socat(transfer: &TransferType) -> String {
    format!(
        r"{} socat '{}'",
        get_switcharoo_path(),
        generate_transfer(transfer)
    )
}

/// # Panics
/// if the transfer could not be serialized into a string
#[must_use]
pub fn generate_transfer(transfer: &TransferType) -> String {
    serde_json::to_string(transfer).expect("serialize transfer")
}
