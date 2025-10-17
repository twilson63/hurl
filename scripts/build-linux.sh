#!/usr/bin/env bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "=== HURL Linux Build ==="
echo ""

TARGETS=(
  "x86_64-unknown-linux-gnu"
  "aarch64-unknown-linux-gnu"
  "armv7-unknown-linux-gnueabihf"
)

BUILD_DIR="${PROJECT_ROOT}/build"
mkdir -p "$BUILD_DIR"

for target in "${TARGETS[@]}"; do
  echo "Building for $target..."
  
  if command -v cross &> /dev/null; then
    cross build --release --target "$target" --bin hurl
  else
    echo "Installing cross for cross-compilation..."
    cargo install cross
    cross build --release --target "$target" --bin hurl
  fi
  
  binary_path="target/$target/release/hurl"
  if [ -f "$binary_path" ]; then
    cp "$binary_path" "$BUILD_DIR/hurl-$target"
    strip "$BUILD_DIR/hurl-$target"
    echo "✓ Built and stripped: $BUILD_DIR/hurl-$target"
  fi
  echo ""
done

echo "=== Creating Linux tarballs ==="
for target in "${TARGETS[@]}"; do
  dirname="${target%-*}"
  tarball="$BUILD_DIR/hurl-linux-$dirname.tar.gz"
  
  mkdir -p "$BUILD_DIR/hurl-$target-tmp"
  cp "$BUILD_DIR/hurl-$target" "$BUILD_DIR/hurl-$target-tmp/hurl"
  cp man/hurl.1 "$BUILD_DIR/hurl-$target-tmp/"
  cp README.md "$BUILD_DIR/hurl-$target-tmp/"
  cp LICENSE-MIT "$BUILD_DIR/hurl-$target-tmp/"
  
  tar -czf "$tarball" -C "$BUILD_DIR" "hurl-$target-tmp"
  rm -rf "$BUILD_DIR/hurl-$target-tmp"
  
  echo "✓ Created $tarball"
done

echo ""
echo "Linux builds complete in: $BUILD_DIR"
