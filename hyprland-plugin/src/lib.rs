use anyhow::{Context, Error};
use gag::Redirect;
use std::fs::File;
use std::io::{Cursor, Read, copy};
use std::path::{Path, PathBuf};
use tempfile::{TempDir, tempdir};
use tracing::{Level, span, trace, warn};
use zip::ZipArchive;

static ASSET_ZIP: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/plugin.zip"));

pub fn extract_plugin() -> Result<TempDir, Error> {
    let tmp_dir = tempdir().expect("create tempdir failed");
    let mut archive = ZipArchive::new(Cursor::new(ASSET_ZIP)).expect("failed to read zip");

    let mut counter = 0;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = tmp_dir.path().join(file.name());

        if let Some(p) = out_path.parent() {
            std::fs::create_dir_all(p)?;
        }
        let mut outfile = File::create(&out_path)?;
        copy(&mut file, &mut outfile)?;
        counter += 1;
    }

    trace!("extracted {} files", counter);
    Ok(tmp_dir)
}

pub fn build<P: AsRef<Path>>(path: P) -> PathBuf {
    let _span = span!(Level::TRACE, "build", path =? path.as_ref()).entered();
    let mut out = path.as_ref().to_path_buf();
    out.push("out");
    let mut logs = path.as_ref().to_path_buf();
    logs.push("log.log");
    let log_file =
        File::create(&logs).with_context(|| format!("unable to create log file: {logs:?}"));
    let mut errs = path.as_ref().to_path_buf();
    errs.push("log.err");
    let log_file_errs =
        File::create(&errs).with_context(|| format!("unable to create log file: {errs:?}"));

    // move to simple run command instead

    trace!("building plugin...");
    // this uses many println's...
    let stdout = log_file.and_then(|f| Redirect::stdout(f).context("unable to redirect stdout"));
    let stderr =
        log_file_errs.and_then(|f| Redirect::stderr(f).context("unable to redirect stderr"));
    cmake::Config::new(&path)
        .define("HYPRSHELL_PLUGIN_NAME", env!("CARGO_PKG_NAME"))
        .define("HYPRSHELL_PLUGIN_AUTHOR", env!("CARGO_PKG_AUTHORS"))
        .define(
            "HYPRSHELL_PLUGIN_DESCRIPTION",
            env!("CARGO_PKG_DESCRIPTION"),
        )
        .define("HYPRSHELL_PLUGIN_VERSION", env!("CARGO_PKG_VERSION"))
        .profile("Release")
        .host(env!("TARGET"))
        .target(env!("TARGET"))
        .out_dir(&out)
        .build();

    match stderr {
        Err(e) => warn!("unable to redirect stderr: {}", e),
        Ok(s) => drop(s),
    };
    match stdout {
        Err(e) => warn!("unable to redirect stdout: {}", e),
        Ok(s) => drop(s),
    };

    if let Ok(mut log_file) =
        File::open(&logs).with_context(|| format!("unable to ope log file: {logs:?}"))
    {
        let mut buf = String::new();
        let e = log_file.read_to_string(&mut buf);
        if let Err(e) = e {
            warn!("unable to read log file: {}", e);
        }
        trace!("cmake output ({}):", buf.lines().count());
        for line in buf.lines() {
            trace!("\t{}", line);
        }
    } else {
        warn!("unable to open log file");
    }
    if let Ok(mut log_file) =
        File::open(&errs).with_context(|| format!("unable to ope log file: {errs:?}"))
    {
        let mut buf = String::new();
        let e = log_file.read_to_string(&mut buf);
        if let Err(e) = e {
            warn!("unable to read log file: {}", e);
        }
        trace!("cmake err-output ({}):", buf.lines().count());
        for line in buf.lines() {
            trace!("\t{}", line);
        }
    } else {
        warn!("unable to open log file");
    }
    out
}
