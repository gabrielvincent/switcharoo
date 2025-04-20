# hyprshell

[![crates.io](https://img.shields.io/crates/v/hyprshell.svg)](https://crates.io/crates/hyprshell) [![Docs](https://docs.rs/built/badge.svg)](https://docs.rs/hyprshell)

![image.png](imgs/swappy-20250420_000613.png)

## Overview

hyprshell is a Rust-based GUI designed to enhance window management in [Hyprland](https://github.com/hyprwm/Hyprland).
It provides a powerful and customizable interface for switching between windows using keyboard shortcuts and GUI.
The application also includes a launcher for running applications directly from the GUI.

## Features

- **Window Switching**: Switch between windows using keyboard shortcuts in a GUI.
- **Customizable Keybindings**: Define your own keybindings for window switching and GUI interactions.
- **Config**: Interactive config file generation for easy setup.
- **Launcher Integration**: Launch applications directly from the GUI, sorted by usage frequency.
- **Sorting and Filtering**: windows sorted by position, can be filtered by class, workspace, or monitor.
- **Theming**: Customize the GUI appearance using CSS.
- **Dynamic Configuration**: Automatically reloads configuration/style changes without restarting the application.

## Installation

### From Source

gtk4 and [gtk4-layer-shell](https://github.com/wmww/gtk4-layer-shell) must be installed

```bash
cargo install hyprshell
```

### Arch Linux (TODO Add)

```bash
paru -S hyprshell
# or
yay -S hyprshell
```

### Nixos

- add ``hyprshell.url = "github:h3rmt/hyprshell/release";`` to flake inputs
- add `specialArgs = { inherit inputs; };` to `nixpkgs.lib.nixosSystem`
- add `inputs.hyprshell.packages.x86_64-linux.default` to your `environment.systemPackages`
- available systems: `aarch64-linux`, `i686-linux`, `riscv32-linux`, `riscv64-linux`, `x86_64-linux`

## Usage

Run ``hyprshell --help`` to see available commands and options.

### Config generation

To generate a default configuration file, run:

```bash
hyprshell config generate
```

This launches an interactive prompt to set up your configuration.
The generated file will be located at `~/.config/hypr/hyprshell.conf`.

### Config validation

To validate your configuration file, run:

```bash 
hyprshell config check
```

This checks for any syntax errors or issues in your configuration file andshows a `explanation` of how to use hyprshell.

### Initialization

Add the following to your Hyprland configuration (`~/.config/hypr/hyprland.conf`):

```ini
exec-once = hyprshell run &
```

Or enable the systemd service (generated with `hyprshell config generate`):

```bash
systemctl --user enable --now hyprshell.service
```

![image.png](imgs/swappy-20250420_000818.png)
![image.png](imgs/swappy-20250420_000857.png)