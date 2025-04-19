use anyhow::Context;
use chrono::{DateTime, Datelike, Utc};
use serde_json::from_reader;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use tracing::warn;

pub fn cache_run(desktop_file: &Box<Path>, cache_path: &Path) -> anyhow::Result<()> {
    let cache_path = get_current_week(cache_path);
    let mut cache_data = if cache_path.exists() {
        let file = OpenOptions::new()
            .read(true)
            .open(&cache_path)
            .context("Failed to open cache file")?;
        from_reader(file).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        // create the file and folder
        std::fs::create_dir_all(cache_path.parent().unwrap())
            .context("Failed to create cache directory")?;
        serde_json::json!({})
    };

    cache_data[&*desktop_file.to_string_lossy()] = serde_json::json!(cache_data
        .get(&*desktop_file.to_string_lossy())
        .map(|v| v.as_i64().unwrap_or(0) + 1)
        .unwrap_or(1));

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&cache_path)
        .context("Failed to open cache file for writing")?;
    serde_json::to_writer_pretty(file, &cache_data).context("Failed to write to cache file")?;
    Ok(())
}

fn get_current_week(cache_path: &Path) -> PathBuf {
    let mut path = PathBuf::from(cache_path);
    path.push("runs");
    path.push(get_name_from_timestamp(0));
    path
}

fn get_all_weeks(run_cache_weeks: u8, cache_path: &Path) -> Vec<Box<Path>> {
    let mut weeks = Vec::new();
    for week in 0..run_cache_weeks {
        let mut path = PathBuf::from(cache_path);
        path.push("runs");
        path.push(get_name_from_timestamp(week));
        weeks.push(path.into_boxed_path());
    }
    weeks
}

fn get_name_from_timestamp(week: u8) -> Box<Path> {
    let timestamp = Utc::now().timestamp() - (week as i64 * 7 * 24 * 60 * 60);
    let datetime = DateTime::from_timestamp(timestamp, 0).expect("Invalid timestamp");
    Box::from(Path::new(&format!(
        "{}_{}.json",
        datetime.year(),
        datetime.iso_week().week()
    )))
}

pub fn get_cached_runs(run_cache_weeks: u8, cache_path: &Path) -> HashMap<Box<Path>, i64> {
    let mut runs = HashMap::new();

    for week in get_all_weeks(run_cache_weeks, cache_path) {
        let cache_data = if week.exists() {
            match OpenOptions::new().read(true).open(&week) {
                Ok(file) => from_reader(file).unwrap_or_else(|_| serde_json::json!({})),
                Err(_) => serde_json::json!({}),
            }
        } else {
            serde_json::json!({})
        };
        if let Some(obj) = cache_data.as_object() {
            for (path, runs_count) in obj {
                runs.insert(
                    PathBuf::from(path).into_boxed_path(),
                    runs_count.as_i64().unwrap_or(0),
                );
            }
        } else {
            warn!("Cache data is not an object");
        }
    }
    runs
}
