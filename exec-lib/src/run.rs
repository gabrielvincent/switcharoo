use core_lib::{Warn, TERMINALS};
use std::os::unix::prelude::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, io, thread};
use tracing::{debug, error, info, trace};

pub fn run_program(
    run: &str,
    path: Option<Box<Path>>,
    terminal: bool,
    default_terminal: &Option<Box<str>>,
) {
    debug!("Running: {run}");
    if terminal {
        if let Some(term) = default_terminal {
            let command = format!("{term} -e {run}");
            run_command(&command, &path).warn("Failed to run command");
        } else {
            info!("No default terminal found, trying to find one. (configure default_terminal in config to set a default terminal)");
            for term in TERMINALS {
                // TODO fix this, command successfull if terminal not exists
                let command = format!("{term} -e {run}");
                if run_command(&command, &path).is_ok() {
                    trace!("Found terminal: {term}");
                    return;
                }
            }
            error!("Failed to find a terminal to run the command");
        }
    } else {
        run_command(run, &path).warn("Failed to run command");
    }
}

fn get_command(command: &str) -> Command {
    // if run as systemd unit all programs exit when not run outside the units cgroup
    if env::var("INVOCATION_ID").is_ok() {
        let mut cmd = Command::new("systemd-run");
        cmd.args(["--user", "--scope", "--collect", "sh", "-c", command]);
        cmd
    } else {
        let mut cmd = Command::new("sh");
        cmd.args(["-c", command]);
        cmd
    }
}

fn run_command(run: &str, path: &Option<Box<Path>>) -> io::Result<()> {
    trace!("Original command: {:?}", run);
    let mut cmd = get_command(run);
    cmd.process_group(0);
    if let Some(path) = path {
        cmd.current_dir(path.as_ref());
    }

    debug!("Running command: {:?}", cmd);
    let _out = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;

    thread::spawn(move || {
        let output = _out.wait_with_output();
        if env::var_os("HYPRSHELL_SHOW_OUTPUT").is_some() {
            if let Ok(output) = output {
                if !output.stdout.is_empty() || !output.stderr.is_empty() {
                    debug!("Output: {:?}", output);
                }
            }
        }
    });
    Ok(())
}
