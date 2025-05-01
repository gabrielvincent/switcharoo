fn main() {
    let path = option_env!("HYPRSHELL_SOCAT_PATH")
        .and_then(|path| Some(path.to_string()))
        .or_else(|| {
            which::which("socat")
                .map(|path| path.to_string_lossy().to_string())
                .ok()
        })
        .expect("`socat` command not found. Please ensure it is installed and available in PATH or set it using HYPRSHELL_SOCAT_PATH.");

    println!("cargo:rustc-env=SOCAT_PATH={}", path);
}
