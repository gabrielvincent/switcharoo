use anyhow::Context;
use semver::Version;
use tracing::info;

pub const MIN_VERSION: Version = Version::new(0, 42, 0);

pub fn check_version(version: anyhow::Result<String>) -> anyhow::Result<()> {
    if let Ok(version) = version {
        info!(
            "Starting hyprshell {} in {} mode on hyprland {}",
            env!("CARGO_PKG_VERSION"),
            if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            },
            version,
        );
        let parsed_version =
            Version::parse(&version).context("Unable to parse hyprland Version")?;
        if parsed_version.lt(&MIN_VERSION) {
            Err(anyhow::anyhow!(
                "hyprland version {} is too old or unknown, please update to at least {}",
                parsed_version,
                MIN_VERSION
            ))
        } else {
            Ok(())
        }
    } else {
        Err(anyhow::anyhow!("Unable to get hyprland version"))
    }
}
