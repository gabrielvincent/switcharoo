use tracing::trace;

pub fn test() -> anyhow::Result<()> {
    let path = hyprland_plugin::extract_plugin()?;
    trace!("plugin extracted to path: {path:?}");
    let out = hyprland_plugin::build(path);
    println!("{:?}", out);
    // hyprland::ctl::plugin::load()
    Ok(())
}
