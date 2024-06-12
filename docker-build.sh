#!/bin/sh
set -e
case "$1" in
    linux/amd64)
        cargo install --path lichess --target x86_64-unknown-linux-gnu
        ;;
    linux/arm64)
        rustup target add aarch64-unknown-linux-gnu
        apt-get update -y
        apt-get install -yq gcc-aarch64-linux-gnu
        export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/aarch64-linux-gnu-gcc
        cargo install --path lichess --target aarch64-unknown-linux-gnu
        ;;
    *)
        echo "unknown target '$1'"
        exit 1
        ;;
esac
