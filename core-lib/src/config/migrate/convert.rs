use crate::config;
use crate::config::migrate::old_structs;

impl From<old_structs::Config> for config::Config {
    fn from(value: old_structs::Config) -> Self {
        Self {
            layerrules: value.layerrules,
            launcher: value.launcher.map(Into::into),
            windows: value.windows.map(Into::into),
        }
    }
}

impl From<old_structs::Windows> for config::Windows {
    fn from(value: old_structs::Windows) -> Self {
        // actual migration, ignoring size_factor
        Self {
            scale: value.scale,
            workspaces_per_row: value.workspaces_per_row,
            strip_html_from_workspace_title: value.strip_html_from_workspace_title,
            overview: value.overview.map(Into::into),
            switch: value.switch.map(Into::into),
        }
    }
}

impl From<old_structs::Launcher> for config::Launcher {
    fn from(value: old_structs::Launcher) -> Self {
        Self {
            max_items: value.max_items,
            width: value.width,
            animate_launch_ms: value.animate_launch_ms,
            default_terminal: value.default_terminal,
            plugins: value.plugins.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<old_structs::Plugin> for config::Plugin {
    fn from(value: old_structs::Plugin) -> Self {
        match value {
            old_structs::Plugin::Applications(options) => {
                config::Plugin::Applications(options.into())
            }
            old_structs::Plugin::Terminal() => config::Plugin::Terminal(),
            old_structs::Plugin::Shell() => config::Plugin::Shell(),
            old_structs::Plugin::WebSearch(engines) => {
                config::Plugin::WebSearch(engines.into_iter().map(Into::into).collect())
            }
            old_structs::Plugin::Calc() => config::Plugin::Calc(),
        }
    }
}

impl From<old_structs::SearchEngine> for config::SearchEngine {
    fn from(value: old_structs::SearchEngine) -> Self {
        Self {
            name: value.name,
            url: value.url,
            // actual migration
            key: value.key.chars().next().unwrap_or('?'),
        }
    }
}
