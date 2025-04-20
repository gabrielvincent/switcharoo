use crate::cache::get_cached_runs;
use crate::desktop_map::{get_all_desktop_files, DesktopEntry};
use crate::global::LauncherGlobalData;
use crate::LauncherGlobal;
use core_lib::theme_icon_cache::theme_has_icon_name;
use core_lib::LAUNCHER_NAMESPACE;
use gtk::gdk::Key;
use gtk::glib::{clone, Propagation};
use gtk::pango::EllipsizeMode;
use gtk::prelude::{ApplicationWindowExt, BoxExt, EditableExt, GtkWindowExt, WidgetExt};
use gtk::{glib, IconSize, Image, ListBoxRow};
use gtk::{
    Align, Application, ApplicationWindow, Entry, EventControllerKey, Label, ListBox, Orientation,
    SelectionMode,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, span, trace, warn, Level};

pub fn create_launcher_window(
    app: &Application,
    global: &mut LauncherGlobal,
    cache_path: &Path,
) -> anyhow::Result<()> {
    let _span = span!(Level::TRACE, "create_launcher_window").entered();

    let main_vbox = ListBox::builder()
        .css_classes(vec!["launcher"])
        .width_request(global.width as i32)
        .selection_mode(SelectionMode::None)
        .build();

    let results = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(vec!["launcher-results"])
        .build();

    let max = global.max_items as usize;
    let show_execs = global.show_execs;
    let run_cache_weeks = global.run_cache_weeks;
    let cache_path = PathBuf::from(cache_path);
    let entry = Entry::builder().css_classes(vec!["launcher-input"]).build();
    entry.connect_changed(clone!(
        #[weak]
        results,
        move |e| {
            trace!("Entry changed: {}", e.text());
            update(
                &e.text(),
                &results,
                max,
                run_cache_weeks,
                show_execs,
                cache_path.clone(),
            );
        }
    ));
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(move |_, k, _, m| match (k, m) {
        (Key::Tab, _) => Propagation::Stop,
        (Key::ISO_Left_Tab, _) => Propagation::Stop,
        _ => Propagation::Proceed,
    });
    entry.add_controller(controller);
    main_vbox.append(&entry);
    main_vbox.append(&results);

    let window = ApplicationWindow::builder()
        .css_classes(vec!["window"])
        .application(app)
        .child(&main_vbox)
        .default_height(10)
        .default_width(10)
        .build();
    window.init_layer_shell();
    window.set_namespace(Some(LAUNCHER_NAMESPACE));
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::Exclusive);
    window.set_anchor(Edge::Top, true);
    window.set_margin(Edge::Top, 20);
    window.present();
    window.set_visible(false);

    debug!("Created launcher window ({})", window.id());
    global.data = Some(RefCell::new(LauncherGlobalData {
        window,
        entry,
        results,
    }));

    Ok(())
}

fn update(
    text: &str,
    list: &ListBox,
    launcher_max_items: usize,
    run_cache_weeks: u8,
    show_launcher_execs: bool,
    cache_path: PathBuf,
) {
    while let Some(child) = list.first_child() {
        list.remove(&child);
    }
    if text.is_empty() {
        return;
    }

    let matches = get_matches(text, launcher_max_items, run_cache_weeks, &cache_path);

    for (index, (_, entry)) in matches.into_iter().take(launcher_max_items).enumerate() {
        let hbox = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .hexpand(true)
            .vexpand(true)
            .build();

        let icon = Image::builder().css_classes(vec!["launcher-icon"]).icon_size(IconSize::Large).build();
        if let Some(icon_path) = entry.icon {
            if icon_path.is_absolute() {
                if let Some(icon_name) = icon_path.file_stem() {
                    if !theme_has_icon_name(&icon_name.to_string_lossy()) {
                        icon.set_from_file(Some(Path::new(&*icon_path)));
                    } else {
                        icon.set_icon_name(Some(&icon_name.to_string_lossy()));
                    }
                } else {
                    warn!("invalid icon name: {icon_path:?}");
                }
            } else {
                icon.set_icon_name(icon_path.file_name().and_then(|name| name.to_str()));
            }
        }
        hbox.append(&icon);

        let title = Label::builder()
            .halign(Align::Start)
            .valign(Align::Center)
            .label(entry.name)
            .build();
        hbox.append(&title);

        if show_launcher_execs {
            let exec = Label::builder()
                .halign(Align::Start)
                .valign(Align::Center)
                .hexpand(true)
                .css_classes(vec!["launcher-exec"])
                .ellipsize(EllipsizeMode::End)
                .label(get_exec_label(&entry.exec))
                .build();
            hbox.append(&exec);
        } else {
            title.set_hexpand(true);
        }

        if let Some(label) = match index {
            0 => Some("Return".to_string()),
            i if i <= 9 => Some(format!("Ctrl+{}", i)),
            _ => None,
        } {
            let index_label = Label::builder()
                .halign(Align::End)
                .valign(Align::Center)
                .label(label)
                .build();
            hbox.append(&index_label);
        }

        let list2 = ListBoxRow::builder()
            .css_classes(vec!["launcher-item"])
            .height_request(45)
            .hexpand(true)
            .vexpand(true)
            .child(&hbox)
            .build();
        // TODO
        // list.add_controller(click_entry(&share, raw_index));

        list.append(&list2);
    }
}

fn get_exec_label(exec: &str) -> String {
    let exec_trim = exec.replace("'", "").replace("\"", "");
    // pwa detection
    if exec.contains("--app-id=") && exec.contains("--profile-directory=") {
        // "flatpak 'run'" = pwa from browser inside flatpak
        if exec.contains("flatpak run") || exec.contains("flatpak 'run'") {
            format!(
                "[flatpak + PWA] ({})",
                exec_trim
                    .split(' ')
                    .find(|s| s.contains("--command="))
                    .and_then(|s| s.split('=').next_back().and_then(|s| s.split('/').next_back()))
                    .unwrap_or_default()
            )
        } else {
            // normal PWA
            format!(
                "[PWA] ({})",
                exec.split(' ')
                    .next()
                    .and_then(|s| s.split('/').next_back())
                    .unwrap_or_default()
            )
        }
        // flatpak detection
    } else if exec.contains("flatpak run") || exec.contains("flatpak 'run'") {
        format!(
            "[flatpak] ({})",
            exec_trim
                .split(' ')
                .find(|s| s.contains("--command="))
                .and_then(|s| s.split('=').next_back().and_then(|s| s.split('/').next_back()))
                .unwrap_or_default()
        )
    } else {
        format!("({})", exec_trim) // show full exec instead of only last part of /path/to/exec
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Match {
    Keyword = 1,
    Name = 10,
    Exact = 15,
}
fn compare_matches(
    runs: HashMap<Box<Path>, i64>,
) -> impl Fn(&(Match, DesktopEntry), &(Match, DesktopEntry)) -> Ordering {
    move |a: &(Match, DesktopEntry), b: &(Match, DesktopEntry)| {
        // sort in reverse order
        match (
            b.0 as i64 + runs.get(&b.1.path).unwrap_or(&0),
            a.0 as i64 + runs.get(&a.1.path).unwrap_or(&0),
        ) {
            (a1, b1) if a1 > b1 => Ordering::Greater,
            (a1, b1) if a1 < b1 => Ordering::Less,
            (a1, b1) if a1 == b1 => {
                // sort by name
                a.1.name.cmp(&b.1.name)
            }
            _ => unreachable!(),
        }
    }
}

pub fn get_matches(
    text: &str,
    launcher_max_items: usize,
    run_cache_weeks: u8,
    cache_path: &Path,
) -> Vec<(Match, DesktopEntry)> {
    let entries = get_all_desktop_files();
    let mut matches = HashMap::new();
    for entry in entries.iter() {
        if entry.keywords.iter().any(|k| {
            k.to_ascii_lowercase()
                .starts_with(&text.to_ascii_lowercase())
        }) {
            matches.insert(entry.path.clone(), (Match::Keyword, entry.clone()));
        }
    }
    // do name last to let them appear first
    for entry in entries.iter() {
        if entry
            .name
            .to_ascii_lowercase()
            .contains(&text.to_ascii_lowercase())
        {
            if entry
                .name
                .to_ascii_lowercase()
                .starts_with(&text.to_ascii_lowercase())
            {
                matches.insert(entry.path.clone(), (Match::Exact, entry.clone()));
            } else {
                matches.insert(entry.path.clone(), (Match::Name, entry.clone()));
            }
        }
    }
    let runs = get_cached_runs(run_cache_weeks, cache_path);

    // sort each of the sections by times run in the past
    let mut matches: Vec<_> = matches.into_values().collect();
    matches.sort_by(compare_matches(runs));
    let matches = matches
        .into_iter()
        .take(launcher_max_items)
        .collect::<Vec<_>>();
    trace!(
        "Matches: {:?}",
        matches
            .iter()
            .map(|(v, e)| format!("{:?}: {}", v, e.name))
            .collect::<Vec<_>>()
    );
    matches
}
