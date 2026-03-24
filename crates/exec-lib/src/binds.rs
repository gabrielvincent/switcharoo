use anyhow::Context;
use core_lib::binds::ExecBind;
use core_lib::SWITCH_NAMESPACE;
use hyprland::config::binds;
use hyprland::config::binds::{Binder, Binding};
use hyprland::dispatch::DispatchType;
use hyprland::keyword::Keyword;
use tracing::{trace, warn};

pub fn apply_layerrules() -> anyhow::Result<()> {
    // Aggressively disable animations for the switcher namespace
    // We use a regex ^...$ to ensure exact match and avoid any ambiguity
    let target = format!("^({SWITCH_NAMESPACE})$");
    
    Keyword::set("layerrule", format!("noanim, {target}"))?;
    
    // Explicitly disable blur and xray which can sometimes cause visual artifacts during mapping
    Keyword::set("layerrule", format!("blur 0, {target}"))?;
    Keyword::set("layerrule", format!("xray 0, {target}"))?;
    
    // We remove dimaround as it often has its own hardcoded or configurable fade animation 
    // that might be what the user is perceiving as a "fade".
    // Keyword::set("layerrule", format!("dimaround, {target}"))?;
    
    // ignorealpha and ignorezero help with snappy transparency transitions
    Keyword::set("layerrule", format!("ignorealpha 0, {target}"))?;
    Keyword::set("layerrule", format!("ignorezero, {target}"))?;

    trace!("layerrules applied for {target}");
    Ok(())
}

// ctrl+shift+alt, h
// hyprland::bind!(d e | SUPER, Key, "a" => Exec, "pkill switcharoo");
pub fn apply_exec_bind(bind: &ExecBind) -> anyhow::Result<()> {
    let binding = Binding {
        mods: bind
            .mods
            .iter()
            .filter_map(|m| match m.to_lowercase().as_str() {
                "alt" => Some(binds::Mod::ALT),
                "control" | "ctrl" => Some(binds::Mod::CTRL),
                "super" | "win" => Some(binds::Mod::SUPER),
                "shift" => Some(binds::Mod::SHIFT),
                _ => {
                    warn!("unknown mod: {m}");
                    None
                }
            })
            .collect(),
        key: binds::Key::Key(&bind.key),
        flags: vec![],
        dispatcher: DispatchType::Exec(&bind.exec),
    };
    trace!("binding exec: {binding:?}");
    Binder::bind(binding).with_context(|| format!("binding exec failed: {bind:?}"))?;
    Ok(())
}
