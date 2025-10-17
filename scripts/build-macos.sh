#!/usr/bin/env bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "=== HURL macOS Build ==="
echo ""

TARGETS=(
  "x86_64-apple-darwin"
  "aarch64-apple-darwin"
)

BUILD_DIR="${PROJECT_ROOT}/build"
mkdir -p "$BUILD_DIR"

for target in "${TARGETS[@]}"; do
  echo "Building for $target..."
  
  rustup target add "$target" 2>/dev/null || true
  cargo build --release --target "$target" --bin hurl
  
  binary_path="target/$target/release/hurl"
  if [ -f "$binary_path" ]; then
    cp "$binary_path" "$BUILD_DIR/hurl-$target"
    strip "$BUILD_DIR/hurl-$target"
    echo "✓ Built and stripped: $BUILD_DIR/hurl-$target"
  fi
  echo ""
done

echo "=== Creating macOS universal binary ==="
lipo -create \
  "$BUILD_DIR/hurl-x86_64-apple-darwin" \
  "$BUILD_DIR/hurl-aarch64-apple-darwin" \
  -output "$BUILD_DIR/hurl-universal-apple-darwin"

echo "✓ Created universal binary"
echo ""

echo "=== Creating macOS tarballs ==="
for target in "${TARGETS[@]}"; do
  arch="${target%-*}"
  tarball="$BUILD_DIR/hurl-macos-$arch.tar.gz"
  
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
echo "macOS builds complete in: $BUILD_DIR"
