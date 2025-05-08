use crate::plugins::{Identifier, PluginNames, SortableLaunchOption};
use std::path::Path;
use tracing::info;

pub(crate) fn get_calc_options(matches: &mut Vec<SortableLaunchOption>, text: &str) {
    let mut context: calc::Context<u64> = Default::default();
    let eval = context.evaluate(text);
    if let Ok(eval) = eval {
        matches.push(SortableLaunchOption {
            icon: Some(Box::from(Path::new("accessories-calculator"))),
            name: format!("{eval}").into_boxed_str(),
            details: Box::from(""),
            details_long: None,
            score: 0,
            data: Identifier {
                plugin: PluginNames::Calc,
                identifier: Some(format!("{eval}").into_boxed_str()),
            },
        });
    } else {
        // trace!("expression error: {eval:?}");
    }
}

pub fn copy_result(iden: &Option<Box<str>>) -> bool {
    if let Some(iden) = iden {
        // TODO COPY in clipboard
        info!("TODO copy to clipboard: {}", iden);
    }
    false
}
