project_dir := justfile_directory()

default:
    @just --list --justfile {{ justfile() }}

[group('security')]
audit:
    #!/usr/bin/env bash
    if ! command -v cargo-audit >/dev/null 2>&1; then
        echo "cargo-audit not found, installing..."
        if ! command -v cargo binstall >/dev/null 2>&1; then
          cargo install --locked cargo-audit
        else
          echo "installing with cargo binstall"
          cargo binstall cargo-outdated
        fi
    fi
    cargo audit

[group('security')]
outdated:
    #!/usr/bin/env bash
    if ! command -v cargo-outdated >/dev/null 2>&1; then
        echo "cargo-outdated not found, installing..."
        if ! command -v cargo binstall >/dev/null 2>&1; then
          cargo install --locked cargo-outdated
        else
          echo "installing with cargo binstall"
          cargo binstall cargo-outdated
        fi
    fi
    cargo outdated

[group('security')]
shear:
    #!/usr/bin/env bash
    if ! command -v cargo-shear >/dev/null 2>&1; then
        echo "cargo-shear not found, installing..."
        if ! command -v cargo binstall >/dev/null 2>&1; then
          cargo install --locked cargo-shear
        else
          echo "installing with cargo binstall"
          cargo binstall cargo-shear
        fi
    fi
    cargo shear

[group('develop')]
format:
    cargo +nightly fmt --all

[group('develop')]
fix:
    cargo fix --allow-dirty --workspace --exclude hyprshell-hyprland --exclude hyprshell-wl-clipboard-rs

[group('develop')]
build profile="dev":
    cargo build --profile {{ profile }} --locked

[group('checks')]
lint profile="dev":
    cargo +nightly fmt --all -- --check
    cargo clippy --profile {{ profile }} --all-targets --workspace --exclude hyprshell-hyprland --exclude hyprshell-wl-clipboard-rs -- --deny warnings

[group('checks')]
test profile="dev":
    cargo nextest run --cargo-profile {{ profile }} --all-targets --workspace --exclude hyprshell-hyprland --exclude hyprshell-wl-clipboard-rs

[group('checks')]
check-feature-combinations:
    bash {{ project_dir }}/scripts/check-all-feature-combinations.sh

[group('checks')]
check-default-nix-features:
    nix build '.#checks.x86_64-linux.hyprshell-check-nix-configs' -L

check target: (build target) (lint target) (test target) check-default-nix-features check-feature-combinations

pre-release: (check "release")

[group('run')]
run *args="":
    cargo run -- {{ args }}

[group('run')]
run-run *args="-vv": (run "run" args)

[group('run')]
run-edit-config *args="-vv": (run "config edit" args)

[group('run')]
run-explain-config *args="-vv": (run "config explain" args)

[group('run')]
run-debug *args="": (run "debug" args)
