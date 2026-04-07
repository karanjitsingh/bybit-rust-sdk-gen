#!/usr/bin/env bash
set -euo pipefail

cargo fmt --manifest-path bybit-rust-sdk/Cargo.toml
cargo clippy --fix --allow-dirty --allow-staged --manifest-path bybit-rust-sdk/Cargo.toml --all-targets --all-features
