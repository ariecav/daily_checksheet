#!/usr/bin/env bash
set -euo pipefail

echo "== daily_checksheet setup =="

if command -v apt >/dev/null 2>&1; then
  sudo apt update
  sudo apt install -y build-essential pkg-config libsqlite3-dev curl
else
  echo "apt not found - install build-essential, pkg-config, libsqlite3-dev manually" >&2
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "Installing Rust via rustup..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
fi

cargo build

echo "== setup complete =="
