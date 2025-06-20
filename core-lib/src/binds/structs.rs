use crate::config::Mod;

#[derive(Debug)]
pub struct ExecBind {
    pub mods: Vec<Mod>,
    pub key: Box<str>,
    // pub flags: Vec<Flag>,
    pub on_release: bool,
    pub exec: Box<str>,
    // hello from bene
}

#[derive(Debug)]
pub enum Flag {
    AllowRepeat,
    DontConsume,
}
