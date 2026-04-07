#!/usr/bin/env bash
set -euo pipefail

node bin/parser.js
cp -r hand-written/src/* bybit-rust-sdk/src/
cp hand-written/Cargo.toml bybit-rust-sdk/Cargo.toml
cp hand-written/Cargo.lock bybit-rust-sdk/Cargo.lock
cp hand-written/rustfmt.toml bybit-rust-sdk/rustfmt.toml
