use crate::Config;
use anyhow::bail;

pub fn check(config: &Config) -> anyhow::Result<()> {
    if config
        .windows
        .as_ref()
        .is_some_and(|w| w.scale >= 15f64 || w.scale <= 0f64)
    {
        bail!("Scale factor must be less than 15 and greater than 0");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::*;

    fn full() -> Config {
        Config {
            windows: Some(Windows {
                switch: Some(Switch::default()),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn test_valid_config() {
        let config = full();
        assert!(check(&config).is_ok());
    }

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn test_invalid_scale() {
        let mut config = full();
        config
            .windows
            .as_mut()
            .expect("config option missing")
            .scale = 20.0;
        assert!(check(&config).is_err());
        config
            .windows
            .as_mut()
            .expect("config option missing")
            .scale = 0.0;
        assert!(check(&config).is_err());
    }
}
