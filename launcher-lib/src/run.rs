use core_lib::{Warn, TERMINALS};
use std::io;
use std::os::unix::prelude::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use tracing::debug;

pub fn run_program(
    run: &str,
    path: &Option<Box<Path>>,
    terminal: bool,
    default_terminal: &Option<String>,
) {
    if terminal {
        if let Some(term) = default_terminal {
            let mut process = Command::new(term);
            process.arg("-e");
            run_command(&mut process, run, path).warn("Failed to run command");
        } else {
            debug!("No default terminal found, trying to find one. (configure default_terminal in config to set a default terminal)");
            for term in TERMINALS {
                let mut process = Command::new(term);
                process.arg("-e");
                if run_command(&mut process, run, path).is_ok() {
                    break;
                }
            }
        }
    } else {
        let mut process = Command::new("sh");
        process.arg("-c");
        run_command(&mut process, run, path).warn("Failed to run command");
    }
}

fn run_command(command: &mut Command, run: &str, path: &Option<Box<Path>>) -> io::Result<()> {
    command.arg::<&str>(run.as_ref());
    command.process_group(0);

    if let Some(path) = path {
        command.current_dir(path.as_ref());
    }
    debug!("Running command: {:?}", command);
    let _out = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    Ok(())
}
