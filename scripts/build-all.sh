#!/usr/bin/env bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "=== HURL Build All Platforms ==="
echo ""

TARGETS=(
  "x86_64-unknown-linux-gnu"
  "aarch64-unknown-linux-gnu"
  "armv7-unknown-linux-gnueabihf"
  "x86_64-apple-darwin"
  "aarch64-apple-darwin"
  "x86_64-pc-windows-msvc"
  "i686-pc-windows-msvc"
)

BUILD_DIR="${PROJECT_ROOT}/build"
mkdir -p "$BUILD_DIR"

failed_targets=()

for target in "${TARGETS[@]}"; do
  echo "Building for $target..."
  
  if cargo build --release --target "$target" --bin hurl 2>/dev/null; then
    echo "✓ $target built successfully"
    
    binary_path="target/$target/release/hurl"
    if [[ "$target" == *"windows"* ]]; then
      binary_path="${binary_path}.exe"
    fi
    
    if [ -f "$binary_path" ]; then
      cp "$binary_path" "$BUILD_DIR/hurl-$target"
      echo "  Copied to $BUILD_DIR/hurl-$target"
    fi
  else
    echo "✗ $target build failed (target may not be installed)"
    failed_targets+=("$target")
  fi
  echo ""
done

echo "=== Build Summary ==="
successful=$((${#TARGETS[@]} - ${#failed_targets[@]}))
echo "Successful: $successful/${#TARGETS[@]}"

if [ ${#failed_targets[@]} -gt 0 ]; then
  echo "Failed targets:"
  for target in "${failed_targets[@]}"; do
    echo "  - $target"
  done
  echo ""
  echo "Note: Some targets may not be available on this system."
  echo "Install missing targets with: rustup target add <target>"
fi

echo ""
echo "Build artifacts are in: $BUILD_DIR"
