#!/usr/bin/env bash
set -euxo pipefail

rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu