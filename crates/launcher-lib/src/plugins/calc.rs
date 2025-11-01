use crate::plugins::{Identifier, PluginNames, SortableLaunchOption};
use adw::gtk::gdk::Display;
use adw::gtk::prelude::DisplayExt;
use core_lib::WarnWithDetails;
use std::path::Path;
use std::sync::{OnceLock, RwLock};
use tracing::{debug, trace};

fn get_context() -> Option<&'static RwLock<rink_core::Context>> {
    static MAP_LOCK: OnceLock<Option<RwLock<rink_core::Context>>> = OnceLock::new();
    MAP_LOCK
        .get_or_init(|| {
            rink_core::simple_context()
                .warn_details("unable to create calc context")
                .map(RwLock::new)
        })
        .as_ref()
}

pub fn init_context() {
    get_context();
}

pub fn get_calc_options(matches: &mut Vec<SortableLaunchOption>, text: &str) {
    if text.is_empty() {
        return;
    }

    let Some(context_lock) = get_context() else {
        return;
    };
    let Ok(mut context) = context_lock.write() else {
        return;
    };
    let eval = rink_core::one_line(&mut context, text);
    // let mut context: calc::Context<f64> = calc::Context::default();
    // let eval = context.evaluate(text);

    if let Ok(eval) = eval {
        let (title, desc) = parse_result(eval);
        trace!("Added calc option: {title}");
        matches.push(SortableLaunchOption {
            icon: Some(Box::from(Path::new("accessories-calculator"))),
            name: title.clone(),
            details: Box::from("Copy to clipboard"),
            details_long: desc,
            score: 0,
            grayed: false,
            iden: Identifier::data(PluginNames::Calc, title),
            details_menu: vec![],
        });
    } else {
        trace!("No option added: expression error: {eval:?}");
    }
}

pub fn copy_result(data: Option<&str>) -> bool {
    if let Some(data) = data {
        if let Some(clipboard) = Display::default().map(|display| display.clipboard()) {
            debug!("Copying result to clipboard: {}", data);
            clipboard.set_text(data.as_ref());
        }
    }
    false
}

#[allow(clippy::map_unwrap_or)]
fn parse_result(result: String) -> (Box<str>, Option<Box<str>>) {
    result
        .split_once(" (")
        .map(|(title, desc)| {
            (
                Box::from(title),
                Some(Box::from(desc.trim_end_matches(')'))),
            )
        })
        .unwrap_or_else(|| (result.into_boxed_str(), None))
}
