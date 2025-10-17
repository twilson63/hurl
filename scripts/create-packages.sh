#!/usr/bin/env bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "=== HURL Package Creation ==="
echo ""

BUILD_DIR="${PROJECT_ROOT}/build"
mkdir -p "$BUILD_DIR"

VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
echo "Package version: $VERSION"
echo ""

echo "=== Building Debian Package ==="
if command -v dpkg-buildpackage &> /dev/null; then
  dpkg-buildpackage -us -uc -b
  echo "✓ Debian package created"
else
  echo "Warning: dpkg-buildpackage not found, skipping Debian package"
fi
echo ""

echo "=== Building Chocolatey Package ==="
if command -v choco &> /dev/null; then
  choco pack hurl.nuspec
  if [ -f "hurl.$VERSION.nupkg" ]; then
    mv "hurl.$VERSION.nupkg" "$BUILD_DIR/"
    echo "✓ Chocolatey package created: $BUILD_DIR/hurl.$VERSION.nupkg"
  fi
else
  echo "Note: Chocolatey CLI not found"
  echo "Manual Chocolatey package creation: choco pack hurl.nuspec"
fi
echo ""

echo "=== Building Homebrew Formula ==="
if [ -f "Formula/hurl.rb" ]; then
  cp "Formula/hurl.rb" "$BUILD_DIR/hurl-$VERSION.rb"
  echo "✓ Homebrew formula copied: $BUILD_DIR/hurl-$VERSION.rb"
else
  echo "Warning: Homebrew formula not found"
fi
echo ""

echo "=== Building Docker Image ==="
if command -v docker &> /dev/null; then
  docker build -t "hurl:$VERSION" .
  docker tag "hurl:$VERSION" "hurl:latest"
  echo "✓ Docker image built: hurl:$VERSION"
  echo "  Run with: docker run -it hurl:$VERSION"
else
  echo "Warning: Docker not found, skipping Docker build"
fi
echo ""

echo "=== Creating RPM Package (optional) ==="
if command -v rpmbuild &> /dev/null; then
  echo "Note: RPM package building not yet implemented"
  echo "Would require: .spec file and rpm build directory"
else
  echo "Note: rpmbuild not found, skipping RPM package"
fi
echo ""

echo "=== Package Summary ==="
echo "Packages created in: $BUILD_DIR"
ls -lah "$BUILD_DIR" || echo "Build directory empty"
