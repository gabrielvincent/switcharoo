use clap_complete::{generate_to, shells};
use std::env;
use std::io::Error;

include!("src/cli.rs");
fn main() -> Result<(), Error> {
    use clap::CommandFactory;
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = App::command();
    let path = generate_to(shells::Bash, &mut cmd, env!("CARGO_PKG_NAME"), &outdir)?;
    println!("cargo:warning=completion file for bash is generated: {path:?}");
    let path = generate_to(shells::Zsh, &mut cmd, env!("CARGO_PKG_NAME"), &outdir)?;
    println!("cargo:warning=completion file for zsh is generated: {path:?}");
    let path = generate_to(shells::Fish, &mut cmd, env!("CARGO_PKG_NAME"), &outdir)?;
    println!("cargo:warning=completion file for fish is generated: {path:?}");
    Ok(())
}
