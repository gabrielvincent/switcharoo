use crate::config::structs::ToKey;
use crate::config::{load_config, Config, Reverse};
use std::path::Path;

pub fn check_config(config_path: &Path) -> anyhow::Result<()> {
    let config = load_config(config_path)
        .inspect_err(|_| eprintln!("\x1b[31mConfig is invalid\x1b[0m\n"))?;
    let info = generate_info(config);
    println!("{info}");
    Ok(())
}

pub fn generate_info(config: Config) -> String {
    let mut builder = String::from("\x1b[32mConfig is valid\x1b[0m\n\n");

    if let Some(windows) = &config.windows {
        if let Some(overview) = &windows.overview {
            builder.push_str(&format!(
                "Use \x1b[1m\x1b[34m{} + {}\x1b[0m to open the Overview. Use \x1b[1m\x1b[34m{}\x1b[0m and \x1b[1m\x1b[34m{}\x1b[0m to select a different window, press \x1b[34mreturn\x1b[0m to switch\n\
                You can also use the \x1b[34marrow keys\x1b[0m to navigate the workspaces. Use \x1b[34mEsc\x1b[0m to close the overview.\n",
                overview.open.modifier,
                overview.open.key.to_key(),
                overview.navigate.forward,
                match &overview.navigate.reverse {
                    Reverse::Key(k) => k.clone(),
                    Reverse::Mod(m) => format!("{} + {}", m, overview.navigate.forward),
                }
            ));
        } else {
            builder.push_str("<Overview disabled>\n");
        };
    };

    if let Some(_launcher) = &config.launcher {
        builder.push_str("After opening the Overview, start typing to search through applications (sorted by how often they were opened).\n\
                    Press \x1b[34mreturn\x1b[0m to launch the first app, use \x1b[34mCtrl + 1/2/3/...\x1b[0m to open the second, third, etc.\n");
    } else {
        builder.push_str("<Launcher disabled>\n");
    }

    builder.push('\n');

    if let Some(windows) = &config.windows {
        if let Some(switch) = &windows.switch {
            builder.push_str(&format!(
                "Press \x1b[1m\x1b[34m{} + {}\x1b[0m and hold \x1b[1m\x1b[34m{}\x1b[0m to view recently used applications. Press/hold \x1b[1m\x1b[34m{}\x1b[0m and \x1b[1m\x1b[34m{}\x1b[0m to select a different window,\n\
                release \x1b[1m\x1b[34m{}\x1b[0m to close the window. You can also use the \x1b[34marrow keys\x1b[0m to navigate the clients.\n",
                switch.navigate.forward,
                switch.open.modifier,
                switch.open.modifier,
                switch.navigate.forward,
                match &switch.navigate.reverse {
                    Reverse::Key(k) => k.clone(),
                    Reverse::Mod(m) => format!("{} + {}", m, switch.navigate.forward),
                },
                switch.open.modifier,
            ));
        } else {
            builder.push_str("<Recent Apps disabled>\n");
        };
    }

    builder
}
