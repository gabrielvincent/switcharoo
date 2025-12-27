use anyhow::{Context, bail};
use std::fs;
use std::path::PathBuf;
use tracing::{debug, warn};

#[derive(Debug)]
pub struct Theme {
    pub name: String,
    pub path: PathBuf,
    pub style: String,
    pub image_path: Option<PathBuf>,
}

pub fn list_themes(path: PathBuf) -> anyhow::Result<(Vec<Theme>, Vec<anyhow::Error>)> {
    let mut themes = Vec::new();
    if !path.exists() {
        bail!("Themes directory does not exist: {}", path.display());
    }

    let mut errors = Vec::new();
    for entry in fs::read_dir(&path)
        .with_context(|| format!("Failed to read themes directory ({})", path.display()))?
    {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                warn!("Failed to read theme directory in: {}", path.display());
                errors.push(err.into());
                continue;
            }
        };

        let file_type = match entry.file_type() {
            Ok(ft) => ft,
            Err(err) => {
                warn!(
                    "Failed to get file type for theme directory: {}",
                    entry.path().display()
                );
                errors.push(err.into());
                continue;
            }
        };
        if !file_type.is_dir() {
            warn!("Invalid theme directory: {}", entry.path().display());
            errors.push(anyhow::anyhow!(
                "Invalid theme directory: {}",
                entry.path().display()
            ));
            continue;
        }
        let dir_path = entry.path();
        let style_path = dir_path.join("style.css");
        if !style_path.is_file() {
            warn!("Invalid theme directory: {}", dir_path.display());
            errors.push(anyhow::anyhow!(
                "Invalid theme directory: {}",
                dir_path.display()
            ));
        } else {
            if let Some(name) = dir_path.file_name().and_then(|n| n.to_str()) {
                let Ok(theme_content) = fs::read_to_string(&style_path) else {
                    warn!("Failed to read theme file: {}", style_path.display());
                    errors.push(anyhow::anyhow!(
                        "Failed to read theme file: {}",
                        style_path.display()
                    ));
                    continue;
                };
                let image_path = dir_path.join("image.jpg");
                themes.push(Theme {
                    name: name.to_string(),
                    path: dir_path.clone(),
                    style: theme_content,
                    image_path: if image_path.exists() {
                        Some(image_path)
                    } else {
                        debug!("Image file not found: {}", image_path.display());
                        None
                    },
                });
            }
        }
    }

    Ok((themes, errors))
}
