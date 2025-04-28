use core_lib::config::Launcher;
use core_lib::transfer::{CloseConfig, TransferType};
use core_lib::{generate_socat, to_ron_string};

fn generate_launcher_return(config: CloseConfig) -> String {
    let config = TransferType::Close(config);
    let config_str = to_ron_string(&config).expect("Failed to serialize config");
    generate_socat(&config_str)
}

pub fn generate_keybinds(keyword_list: &mut Vec<(&str, String)>, launcher: &Launcher) {
    // add index keys launcher run
    for i in 1..=9 {
        keyword_list.push((
            "bind",
            format!(
                "ctrl, {}, exec, {}",
                i,
                generate_launcher_return(CloseConfig::Launcher(
                    char::from_u32(i).expect("Failed to convert u32 to char")
                ))
            ),
        ));
    }
    // TODO extend this
    if true {
        keyword_list.push((
            "bind",
            format!(
                "ctrl, r, exec, {}",
                generate_launcher_return(CloseConfig::Launcher('r'))
            ),
        ));
        keyword_list.push((
            "bind",
            format!(
                "ctrl, t, exec, {}",
                generate_launcher_return(CloseConfig::Launcher('t'))
            ),
        ));
    }
}
