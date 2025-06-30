use crate::binds::structs::ExecBind;
use anyhow::bail;
use config_lib::Modifier;

pub fn generate_bind_kill(kill_bind: &str) -> anyhow::Result<ExecBind> {
    let a = kill_bind
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let mods = match a.first() {
        Some(s) => {
            let mut parsed_mods = Vec::new();
            for str in s.split('+') {
                parsed_mods.push(Modifier::try_from(str)?.to_str());
            }
            parsed_mods
        }
        None => bail!("No mods specified"),
    };
    let key = match a.get(1) {
        Some(s) => Box::from(*s),
        None => bail!("No key provided in bind: {}", kill_bind),
    };

    let bind = ExecBind {
        key,
        mods,
        on_release: true,
        exec: format!("kill {}", std::process::id()).into_boxed_str(),
    };
    Ok(bind)
}
