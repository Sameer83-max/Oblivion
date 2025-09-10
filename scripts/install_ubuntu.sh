#!/usr/bin/env bash
set -euo pipefail

INCLUDE_SERVER=${INCLUDE_SERVER:-0}
INCLUDE_ISO=${INCLUDE_ISO:-1}

echo "== Secure Disk Erasure Tool - Ubuntu Setup =="

# 1) Rust
if ! command -v rustup >/dev/null 2>&1; then
  echo "Installing Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
fi

# 2) Build essentials & OpenSSL
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev curl wget git

# 3) Disk tools
sudo apt-get install -y hdparm nvme-cli sg3-utils lsblk parted gdisk shred

# 4) Node.js (use apt for simplicity; consider nvm for teams)
if ! command -v node >/dev/null 2>&1; then
  echo "Installing Node.js LTS..."
  curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
  sudo apt-get install -y nodejs
fi

# 5) Tauri CLI
npm install -g @tauri-apps/cli

# 6) ISO build (optional, default on)
if [ "$INCLUDE_ISO" = "1" ]; then
  sudo apt-get install -y live-build
fi

echo "\nDone. Restart your shell or 'source ~/.cargo/env' to use cargo."
