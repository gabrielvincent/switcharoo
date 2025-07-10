use crate::cli;
use anyhow::{Context, bail};
use std::fs::{create_dir_all, write};
use std::path::PathBuf;
use tracing::info;

pub fn generate(shell: &str, bash_path: Option<PathBuf>) -> anyhow::Result<()> {
    use clap::CommandFactory;
    use clap_complete::aot::*;
    let cli = &mut cli::App::command();
    let mut buffer = Vec::new();
    match shell {
        "bash" => {
            let mut path = bash_path.unwrap_or("/usr/share/bash-completion/completions".into());
            create_dir_all(&path)
                .with_context(|| format!("failed to create directory: {}", path.display()))?;
            generate(Bash, cli, "hyprshell", &mut buffer);
            path.push("hyprshell.bash");
            write(&path, buffer)
                .with_context(|| format!("failed to write to file: {}", &path.display()))?;
            info!("Generated bash completion script at: {}", path.display());
        }
        "zsh" => {
            let mut path = bash_path.unwrap_or("/usr/share/zsh/site-functions".into());
            create_dir_all(&path)
                .with_context(|| format!("failed to create directory: {}", &path.display()))?;
            generate(Zsh, cli, "hyprshell", &mut buffer);
            path.push("_hyprshell");
            write(&path, buffer)
                .with_context(|| format!("failed to write to file: {}", &path.display()))?;
            info!("Generated zsh completion script at: {}", path.display());
        }
        "fish" => {
            let mut path = bash_path.unwrap_or("/usr/share/fish/vendor_completions.d".into());
            create_dir_all(&path)
                .with_context(|| format!("failed to create directory: {}", &path.display()))?;
            generate(Fish, cli, "hyprshell", &mut buffer);
            path.push("hyprshell.fish");
            write(&path, buffer)
                .with_context(|| format!("failed to write to file: {}", &path.display()))?;
            info!("Generated fish completion script at: {}", path.display());
        }
        _ => bail!("unknown shell: {}", shell),
    };
    Ok(())
}
