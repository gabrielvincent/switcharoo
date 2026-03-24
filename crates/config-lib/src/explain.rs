use crate::Config;
use std::fmt::Write;
use std::path::Path;

const BOLD: &str = "\x1b[1m";
const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

#[must_use]
pub fn explain(config: &Config, config_file: Option<&Path>, enable_color: bool) -> String {
    let (bold, blue, green, reset) = if enable_color {
        (BOLD, BLUE, GREEN, RESET)
    } else {
        ("", "", "", "")
    };

    let mut builder = config_file.map_or_else(String::new, |config_file| {
        let config_file_display = config_file.display();
        format!(
            "{bold}{green}Config is valid{reset} ({config_file_display})\n{bold}Explanation{reset} ({blue}blue{reset} are keys, {bold}{blue}bold blue{reset} keys can be configured in config):{reset}\n",
        )
    });

    if let Some(windows) = &config.windows {
        if let Some(switch) = &windows.switch {
            let _ = builder.write_str(&format!(
                "Press {bold}{blue}{}{reset} + {blue}tab{reset} and hold {bold}{blue}{}{reset} to view recently used applications. Press {blue}tab{reset} and {blue}grave{reset} / {blue}shift{reset} + {blue}tab{reset} to select a different window, release {bold}{blue}{}{reset} to close the window.\n",
                switch.modifier,
                switch.modifier,
                switch.modifier,
            ));
        } else {
            let _ = builder.write_str("<Switch mode disabled>\n");
        }
    } else {
        let _ = builder.write_str("<Windows disabled>\n");
    }

    builder
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::*;
    use std::path::PathBuf;

    fn create_test_config() -> Config {
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
    fn test_explain_with_switch() {
        const CONFIG: &str = r"Config is valid (/test/config.ron)
Explanation (blue are keys, bold blue keys can be configured in config):
Press Alt + tab and hold Alt to view recently used applications. Press tab and grave / shift + tab to select a different window, release Alt to close the window.
";
        let config = create_test_config();
        let path = PathBuf::from("/test/config.ron");
        let result = explain(&config, Some(&path), false);
        assert_eq!(result, CONFIG);
    }

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn test_explain_without_switch() {
        const CONFIG: &str = r"Config is valid (/test/config.ron)
Explanation (blue are keys, bold blue keys can be configured in config):
<Switch mode disabled>
";
        let mut config = create_test_config();
        config
            .windows
            .as_mut()
            .expect("config option missing")
            .switch = None;
        let path = PathBuf::from("/test/config.ron");
        let result = explain(&config, Some(&path), false);
        assert_eq!(result, CONFIG);
    }
}
