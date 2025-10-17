#!/usr/bin/env bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "=== HURL Windows Build ==="
echo ""

if [[ "$OSTYPE" != "msys" && "$OSTYPE" != "win32" && "$OSTYPE" != "cygwin" ]]; then
  echo "Note: Building Windows binaries from non-Windows system"
  echo "Install cross for cross-compilation:"
  echo "  cargo install cross"
  echo ""
fi

TARGETS=(
  "x86_64-pc-windows-msvc"
  "i686-pc-windows-msvc"
)

BUILD_DIR="${PROJECT_ROOT}/build"
mkdir -p "$BUILD_DIR"

for target in "${TARGETS[@]}"; do
  echo "Building for $target..."
  
  if command -v cross &> /dev/null; then
    cross build --release --target "$target" --bin hurl
  else
    if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" || "$OSTYPE" == "cygwin" ]]; then
      cargo build --release --target "$target" --bin hurl
    else
      echo "Error: cross not installed and not on Windows"
      echo "Install cross: cargo install cross"
      exit 1
    fi
  fi
  
  binary_path="target/$target/release/hurl.exe"
  if [ -f "$binary_path" ]; then
    cp "$binary_path" "$BUILD_DIR/hurl-$target.exe"
    echo "✓ Built: $BUILD_DIR/hurl-$target.exe"
  fi
  echo ""
done

echo "=== Creating Windows ZIP archives ==="
for target in "${TARGETS[@]}"; do
  arch="${target%-*}"
  
  if command -v zip &> /dev/null; then
    zip -j "$BUILD_DIR/hurl-windows-$arch.zip" \
      "$BUILD_DIR/hurl-$target.exe" \
      README.md LICENSE-MIT
    echo "✓ Created $BUILD_DIR/hurl-windows-$arch.zip"
  else
    echo "Warning: zip not found, creating tar.gz instead"
    tar -czf "$BUILD_DIR/hurl-windows-$arch.tar.gz" \
      -C "$BUILD_DIR" "hurl-$target.exe" && \
      tar -rzf "$BUILD_DIR/hurl-windows-$arch.tar.gz" README.md LICENSE-MIT
    echo "✓ Created $BUILD_DIR/hurl-windows-$arch.tar.gz"
  fi
done

echo ""
echo "Windows builds complete in: $BUILD_DIR"
