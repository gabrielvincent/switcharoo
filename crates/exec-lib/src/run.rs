use anyhow::Context;
use std::os::unix::prelude::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, thread};
use tracing::{debug, trace};

pub fn run_program(
    run: &str,
    path: Option<&Path>,
) -> anyhow::Result<()> {
    debug!("Running: {run}");
    let home_path_buf = env::var_os("HOME").map(PathBuf::from);
    let path = path.map_or(home_path_buf.as_deref(), Some);
    run_command(run, path).context("Failed to run command")?;
    Ok(())
}

fn get_command(command: &str) -> Command {
    // replace common exec placeholders
    let mut command = command.to_string();
    for replacement in ["%f", "%F", "%u", "%U"] {
        command = command.replace(replacement, "");
    }
    // if run as systemd unit all programs exit when not run outside the units cgroup
    if env::var_os("INVOCATION_ID").is_some() {
        let mut cmd = Command::new("systemd-run");
        cmd.args([
            "--user",
            "--scope",
            "--collect",
            "/usr/bin/env",
            "bash",
            "-c",
            &command,
        ]);
        cmd
    } else {
        let mut cmd = Command::new("/usr/bin/env");
        cmd.args(["bash", "-c", &command]);
        cmd
    }
}

fn run_command(run: &str, path: Option<&Path>) -> anyhow::Result<()> {
    trace!("Original command: {run:?}");
    let mut cmd = get_command(run);
    cmd.process_group(0);
    if let Some(path) = path {
        cmd.current_dir(path);
    }

    debug!("Running command: {cmd:?}");
    let out = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    thread::spawn(move || {
        let start = std::time::Instant::now();
        let output = out.wait_with_output();
        trace!("Command [{cmd:?}] finished");
        if let Ok(output) = output
            && start.elapsed().as_secs() < 2
            && (!output.stdout.is_empty() || !output.stderr.is_empty())
        {
            trace!("Output from [{cmd:?}]: {output:?}");
        }
    });
    Ok(())
}
