use anyhow::{bail, Context};
use std::fs::create_dir_all;
use std::path::PathBuf;
use tracing::{info, span, Level};

const CSS_CONFIG: &str = include_str!("default.css");

pub fn write_css(css_path: PathBuf, override_file: bool) -> anyhow::Result<()> {
    let _span = span!(Level::TRACE, "write_css").entered();

    if css_path.exists() && !override_file {
        bail!(
            "CSS file at {css_path:?} already exists, delete it before generating a new one or use -f to override"
        );
    }
    if let Some(parent) = css_path.parent() {
        create_dir_all(parent)
            .with_context(|| format!("Failed to create config dir at ({parent:?})"))?;
    }

    std::fs::write(&css_path, CSS_CONFIG)
        .with_context(|| format!("Failed to write css file at ({css_path:?})"))?;

    info!("CSS file generated successfully at {css_path:?}");
    Ok(())
}
