#!/usr/bin/env bash
set -euxo pipefail

rustup default stable && rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
ls -lh target/x86_64-unknown-linux-gnu/release/switcharoo

tar --zstd -cf /tmp/switcharoo-x86_64.tar.zst --transform 's,.*/,,' LICENSE README.md docs/CONFIGURE.md docs/DEBUG.md target/x86_64-unknown-linux-gnu/release/switcharoo packaging/switcharoo.service packaging/switcharoo-settings.png packaging/switcharoo-settings.desktop packaging/usr-share.tar
ls -lh /tmp/switcharoo-x86_64.tar.zst

cargo build --release --target x86_64-unknown-linux-gnu --no-default-features --features slim
ls -lh target/x86_64-unknown-linux-gnu/release/switcharoo

tar --zstd -cf /tmp/switcharoo-slim-x86_64.tar.zst --transform 's,.*/,,' LICENSE README.md docs/CONFIGURE.md docs/DEBUG.md target/x86_64-unknown-linux-gnu/release/switcharoo packaging/switcharoo.service packaging/switcharoo-settings.png packaging/switcharoo-settings.desktop packaging/usr-share.tar
ls -lh /tmp/switcharoo-slim-x86_64.tar.zst
