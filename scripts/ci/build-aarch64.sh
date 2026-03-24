#!/usr/bin/env bash
set -euxo pipefail

sudo apt update && sudo apt -y install --no-install-recommends \
  libgraphene-1.0-dev:arm64 zstd

export PKG_CONFIG_ALLOW_CROSS=1
export PKG_CONFIG_PATH=/usr/lib/aarch64-linux-gnu/pkgconfig:/usr/aarch64-linux-gnu/lib/pkgconfig

rustup default stable && rustup target add aarch64-unknown-linux-gnu
CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc cargo build --release --target aarch64-unknown-linux-gnu
ls -lh target/aarch64-unknown-linux-gnu/release/switcharoo

tar --zstd -cf /tmp/switcharoo-aarch64.tar.zst --transform 's,.*/,,' LICENSE README.md docs/CONFIGURE.md docs/DEBUG.md target/aarch64-unknown-linux-gnu/release/switcharoo packaging/switcharoo.service packaging/switcharoo-settings.png packaging/switcharoo-settings.desktop packaging/usr-share.tar
ls -lh /tmp/switcharoo-aarch64.tar.zst

CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc cargo build --release --target aarch64-unknown-linux-gnu --no-default-features --features slim
ls -lh target/aarch64-unknown-linux-gnu/release/switcharoo

tar --zstd -cf /tmp/switcharoo-slim-aarch64.tar.zst --transform 's,.*/,,' LICENSE README.md docs/CONFIGURE.md docs/DEBUG.md target/aarch64-unknown-linux-gnu/release/switcharoo packaging/switcharoo.service packaging/switcharoo-settings.png packaging/switcharoo-settings.desktop packaging/usr-share.tar
ls -lh /tmp/switcharoo-slim-aarch64.tar.zst
