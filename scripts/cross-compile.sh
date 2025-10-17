#!/usr/bin/env bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "=== HURL Cross-Compilation Setup ==="
echo ""

if ! command -v cargo &> /dev/null; then
  echo "Error: Rust/Cargo not installed"
  echo "Install from: https://rustup.rs/"
  exit 1
fi

echo "Available targets:"
rustup target list | grep installed

echo ""
echo "=== Installing Cross-Compilation Tools ==="

TARGETS=(
  "x86_64-unknown-linux-gnu"
  "aarch64-unknown-linux-gnu"
  "armv7-unknown-linux-gnueabihf"
  "arm-unknown-linux-gnueabihf"
  "x86_64-apple-darwin"
  "aarch64-apple-darwin"
  "x86_64-pc-windows-gnu"
  "x86_64-pc-windows-msvc"
  "i686-pc-windows-msvc"
)

for target in "${TARGETS[@]}"; do
  if rustup target list | grep -q "^$target"; then
    if ! rustup target list | grep -q "^$target.*installed"; then
      echo "Installing $target..."
      rustup target add "$target"
    else
      echo "✓ $target already installed"
    fi
  fi
done

echo ""
echo "=== Installing cross utility ==="
if ! command -v cross &> /dev/null; then
  echo "Installing cross for easier cross-compilation..."
  cargo install cross
  echo "✓ cross installed"
else
  echo "✓ cross already installed"
fi

echo ""
echo "=== Build Example ==="
echo "Build for Linux ARM64:"
echo "  cross build --release --target aarch64-unknown-linux-gnu --bin hurl"
echo ""
echo "Build for macOS Intel:"
echo "  rustup target add x86_64-apple-darwin"
echo "  cargo build --release --target x86_64-apple-darwin --bin hurl"
echo ""
echo "Build for Windows:"
echo "  cargo build --release --target x86_64-pc-windows-msvc --bin hurl"
echo ""
echo "Or use the platform-specific build scripts:"
echo "  bash scripts/build-linux.sh"
echo "  bash scripts/build-macos.sh"
echo "  bash scripts/build-windows.sh"
