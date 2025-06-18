use anyhow::Context;
use core_lib::binds::ExecBind;
use core_lib::config::Mod;
use core_lib::{LAUNCHER_NAMESPACE, OVERVIEW_NAMESPACE};
use hyprland::config::binds;
use hyprland::config::binds::{Binder, Binding};
use hyprland::dispatch::DispatchType;
use hyprland::keyword::Keyword;

pub fn apply_layerrules() -> anyhow::Result<()> {
    Keyword::set("layerrule", format!("noanim, {LAUNCHER_NAMESPACE}"))?;
    Keyword::set("layerrule", format!("noanim, {OVERVIEW_NAMESPACE}"))?;
    Keyword::set("layerrule", format!("dimaround, {OVERVIEW_NAMESPACE}"))?;
    Keyword::set("layerrule", format!("ignorezero, {OVERVIEW_NAMESPACE}"))?;
    Keyword::set("layerrule", format!("ignorezero, {LAUNCHER_NAMESPACE}"))?;
    Ok(())
}

// ctrl+shift+alt, h
// hyprland::bind!(d e | SUPER, Key, "a" => Exec, "pkill hyprshell");
pub fn apply_exec_bind(bind: &ExecBind) -> anyhow::Result<()> {
    Binder::bind(Binding {
        mods: bind
            .mods
            .iter()
            .map(|m| match m {
                Mod::Alt => binds::Mod::ALT,
                Mod::Ctrl => binds::Mod::CTRL,
                Mod::Super => binds::Mod::SUPER,
                Mod::Shift => binds::Mod::SHIFT,
            })
            .collect(),
        key: binds::Key::Key(&bind.key.to_key()),
        flags: vec![],
        dispatcher: DispatchType::Exec(&bind.exec),
    })
    .with_context(|| format!("binding exec failed: {bind:?}"))?;
    Ok(())
}
