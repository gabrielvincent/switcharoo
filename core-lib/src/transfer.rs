use crate::{ClientId, WorkspaceId};
use anyhow::Context;
use ron::extensions::Extensions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TransferType {
    OpenOverview(OpenOverview),
    OpenSwitch(OpenSwitch),
    Switch(SwitchConfig),
    Return(ReturnConfig),
    Close,
    Restart,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenSwitch {
    pub submap_name: String,
    pub hide_filtered: bool,
    pub filter_current_workspace: bool,
    pub filter_current_monitor: bool,
    pub filter_same_class: bool,
    pub workspaces_per_row: u8,
    pub direction: Direction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenOverview {
    pub submap_name: String,
    pub hide_filtered: bool,
    pub filter_current_workspace: bool,
    pub filter_current_monitor: bool,
    pub filter_same_class: bool,
    pub workspaces_per_row: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwitchConfig {
    pub direction: Direction,
    pub workspace: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnConfig {
    pub r#override: Option<Override>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Override {
    Offset(u8),
    ClientId(ClientId),
    WorkspaceID(WorkspaceId),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub fn to_ron_string(transfer: &TransferType) -> anyhow::Result<String> {
    ron::Options::default()
        .with_default_extension(Extensions::IMPLICIT_SOME)
        .with_default_extension(Extensions::UNWRAP_VARIANT_NEWTYPES)
        .with_default_extension(Extensions::EXPLICIT_STRUCT_NAMES)
        .to_string(transfer)
        .context("Failed to serialize ron transfer data")
}

pub fn from_ron_string(transfer: &str) -> anyhow::Result<TransferType> {
    ron::Options::default()
        .with_default_extension(Extensions::IMPLICIT_SOME)
        .with_default_extension(Extensions::UNWRAP_VARIANT_NEWTYPES)
        .with_default_extension(Extensions::EXPLICIT_STRUCT_NAMES)
        .from_str(transfer)
        .context("Failed to deserialize ron transfer data")
}
