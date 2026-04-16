#!/usr/bin/env bash
set -euo pipefail

node bin/parser.js
cp -r hand-written/src/* bybit-rust-sdk/src/
cp hand-written/Cargo.toml bybit-rust-sdk/Cargo.toml
cp hand-written/Cargo.lock bybit-rust-sdk/Cargo.lock
cp hand-written/rustfmt.toml bybit-rust-sdk/rustfmt.toml
if [ -d hand-written/examples ]; then
  mkdir -p bybit-rust-sdk/examples
  cp -r hand-written/examples/* bybit-rust-sdk/examples/
fi
if [ -f hand-written/README.md ]; then
  cp hand-written/README.md bybit-rust-sdk/README.md
fi
if [ -d hand-written/tests ]; then
  mkdir -p bybit-rust-sdk/tests
  cp -r hand-written/tests/* bybit-rust-sdk/tests/
fi
