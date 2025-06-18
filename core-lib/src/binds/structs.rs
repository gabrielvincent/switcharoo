use crate::config::{KeyMaybeMod, Mod};

#[derive(Debug)]
pub struct ExecBind {
    pub mods: Vec<Mod>,
    pub key: KeyMaybeMod,
    pub flags: Vec<Flag>,
    pub exec: Box<str>,
}

#[derive(Debug)]
pub enum Flag {
    AllowRepeat,
    DontConsume,
}
