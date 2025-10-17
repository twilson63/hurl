# HURL Installation Guide

Complete installation instructions for HURL across all supported platforms and methods.

## Table of Contents

1. [System Requirements](#system-requirements)
2. [Quick Install](#quick-install)
3. [macOS Installation](#macos-installation)
4. [Linux Installation](#linux-installation)
5. [Windows Installation](#windows-installation)
6. [Docker Installation](#docker-installation)
7. [Building from Source](#building-from-source)
8. [Cross-Compilation](#cross-compilation)
9. [Post-Installation](#post-installation)
10. [Uninstallation](#uninstallation)
11. [Troubleshooting](#troubleshooting)

## System Requirements

### Minimum Requirements
- **CPU**: Any 64-bit processor (or 32-bit with `i686` target)
- **RAM**: 256 MB minimum, 512 MB recommended
- **Disk**: 50 MB for binary + dependencies
- **Network**: Internet access for downloads (not required for offline use)

### Platform Support

| Platform | Architecture | Status | Note |
|----------|-------------|--------|------|
| macOS | x86_64 | Supported | 10.12+, Intel |
| macOS | ARM64 | Supported | 11+, Apple Silicon |
| Linux | x86_64 | Supported | Most distributions |
| Linux | ARM64 | Supported | Raspberry Pi 4, Jetson |
| Linux | armv7 | Supported | Older ARM devices |
| Windows | x86_64 | Supported | Windows 7+ |
| Windows | i686 | Supported | 32-bit systems |
| FreeBSD | x86_64 | Partial | Community builds |

### Dependencies

#### macOS
- Xcode Command Line Tools (for building from source)
- OpenSSL 1.1 or later

#### Linux
- `libssl-dev` or `openssl-dev` (depending on distro)
- `ca-certificates` for SSL/TLS
- glibc 2.17+

#### Windows
- Visual C++ Redistributable (included in Windows 10+)
- .NET Framework 4.7+ (optional, for certain features)

## Quick Install

### macOS (Homebrew)
```bash
brew install hurl
hurl --version
```

### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install hurl
hurl --version
```

### Windows (Chocolatey)
```powershell
choco install hurl
hurl --version
```

### Docker
```bash
docker run hurl --version
```

### Any Platform (Cargo)
```bash
cargo install --path crates/hurl-cli
```

## macOS Installation

### Method 1: Homebrew (Recommended)

The easiest way to install on macOS. Homebrew handles updates automatically.

```bash
# Install
brew install hurl

# Verify installation
hurl --version

# Check man page
man hurl

# Update to latest version
brew upgrade hurl
```

**Uninstall:**
```bash
brew uninstall hurl
```

**Note on M1/M2 (Apple Silicon):** Homebrew automatically detects your architecture and installs the correct binary. No manual steps needed.

### Method 2: macOS Binary Download

Download pre-built binaries from GitHub releases.

```bash
# Download latest release
VERSION="0.1.0"
ARCH="x86_64"  # or "aarch64" for Apple Silicon
URL="https://github.com/hurl/hurl/releases/download/v${VERSION}/hurl-${VERSION}-${ARCH}-apple-darwin.tar.gz"

# Download and extract
wget "$URL" -O hurl.tar.gz
tar xzf hurl.tar.gz
cd hurl-${VERSION}-${ARCH}-apple-darwin/

# Install globally
sudo cp hurl /usr/local/bin/
sudo cp hurl.1 /usr/local/share/man/man1/

# Verify
hurl --version
man hurl
```

**Note:** You can also use `~/bin/hurl` for user-level installation (add `~/bin` to `$PATH` if not already done).

### Method 3: Install from Source

Build HURL from source code.

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/hurl/hurl.git
cd hurl

# Build and install
cargo install --path crates/hurl-cli

# Verify
hurl --version
which hurl  # Should show ~/.cargo/bin/hurl
```

**Build Options:**
```bash
# Release build (optimized, recommended)
cargo install --path crates/hurl-cli --release

# Debug build (faster compilation, slower execution)
cargo install --path crates/hurl-cli

# Install to custom location
cargo install --path crates/hurl-cli --root ~/.local
```

### Method 4: MacPorts

For MacPorts users:

```bash
sudo port install hurl
sudo port upgrade outdated
```

## Linux Installation

### Debian/Ubuntu (apt)

**Ubuntu 20.04 LTS and later:**

```bash
# Add HURL repository (if available)
curl -fsSL https://hurl.dev/install.sh | sudo bash

# Or install from standard repositories
sudo apt-get update
sudo apt-get install hurl

# Verify installation
hurl --version

# View man page
man hurl

# View completions
sudo update-bash-completion
```

**Building from Debian package manually:**

```bash
sudo apt-get install build-essential cargo rustc pkg-config libssl-dev

git clone https://github.com/hurl/hurl.git
cd hurl

dpkg-buildpackage -us -uc
sudo dpkg -i ../hurl_*.deb
```

### Red Hat/Fedora (DNF/RPM)

HURL is available in some repositories:

```bash
# Fedora
sudo dnf install hurl

# RHEL (may require EPEL)
sudo yum install epel-release
sudo yum install hurl

# Or build from source with cargo
sudo dnf install cargo rustc openssl-devel
cargo install --path crates/hurl-cli
```

### Arch Linux (pacman)

```bash
# From official repository (if available)
sudo pacman -S hurl

# Or build from source
git clone https://github.com/hurl/hurl.git
cd hurl
cargo install --path crates/hurl-cli
```

### Alpine Linux (apk)

```bash
# Build dependencies
sudo apk add --no-cache cargo rustc openssl-dev

# Build and install
git clone https://github.com/hurl/hurl.git
cd hurl
cargo install --path crates/hurl-cli
```

### Generic Linux Binary Installation

For any Linux distribution:

```bash
VERSION="0.1.0"
ARCH="x86_64"

# Download binary
wget https://github.com/hurl/hurl/releases/download/v${VERSION}/hurl-${VERSION}-${ARCH}-unknown-linux-gnu.tar.gz

# Extract and install
tar xzf hurl-${VERSION}-${ARCH}-unknown-linux-gnu.tar.gz
cd hurl-${VERSION}-${ARCH}-unknown-linux-gnu

sudo cp hurl /usr/local/bin/
sudo cp hurl.1 /usr/local/share/man/man1/

# For Bash completion (optional)
sudo cp completions/hurl.bash /usr/share/bash-completion/completions/hurl

# Verify
hurl --version
```

### Snap (Ubuntu and others)

```bash
sudo snap install hurl

# Verify
hurl --version

# Update automatically with snap
```

### Building from Source on Linux

```bash
# Install build dependencies (Ubuntu/Debian)
sudo apt-get install build-essential cargo rustc pkg-config libssl-dev

# Or for Fedora
sudo dnf install cargo rustc openssl-devel

# Clone and build
git clone https://github.com/hurl/hurl.git
cd hurl

cargo build --release

# Binary is at: target/release/hurl
./target/release/hurl --version

# Install system-wide
sudo cp target/release/hurl /usr/local/bin/
sudo cp man/hurl.1 /usr/local/share/man/man1/
```

## Windows Installation

### Method 1: Chocolatey (Recommended)

Windows package manager method:

```powershell
# Install Chocolatey (if not already installed)
# From PowerShell as Administrator:
# Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
# (Invoke-WebRequest -Uri https://chocolatey.org/install.ps1).Content | Invoke-Expression

# Install HURL
choco install hurl

# Verify
hurl --version

# Update
choco upgrade hurl
```

### Method 2: Scoop

Alternative Windows package manager:

```powershell
# Install Scoop (if not already installed)
iwr -useb get.scoop.sh | iex

# Install HURL
scoop install hurl

# Verify
hurl --version
```

### Method 3: Windows Binary Download

Direct download from GitHub:

```powershell
# Set version
$VERSION = "0.1.0"

# Download
$url = "https://github.com/hurl/hurl/releases/download/v$VERSION/hurl-$VERSION-x86_64-pc-windows-msvc.zip"
Invoke-WebRequest -Uri $url -OutFile hurl.zip

# Extract
Expand-Archive -Path hurl.zip -DestinationPath hurl
cd hurl

# Install to Program Files
New-Item -Type Directory "C:\Program Files\HURL" -Force
Copy-Item hurl.exe "C:\Program Files\HURL\"

# Add to PATH (requires Administrator PowerShell)
$env:Path += ";C:\Program Files\HURL"
[Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::Machine)

# Verify
hurl --version
```

### Method 4: Windows Terminal (winget)

Windows 11+ package manager:

```powershell
winget install hurl

hurl --version
```

### Method 5: Build from Source

```powershell
# Install Rust
# Download from https://rustup.rs/
# Or use: irm https://astro.build/install.ps1 | iex

# Clone repository
git clone https://github.com/hurl/hurl.git
cd hurl

# Build
cargo build --release

# Binary at: target\release\hurl.exe
.\target\release\hurl.exe --version

# Add to PATH for system-wide access
```

### Shell Completion for Windows

**PowerShell:**
```powershell
hurl --generate-completion powershell | Add-Content $PROFILE
```

**cmd.exe:**
Manual completion not directly supported, but can use `doskey` aliases:
```cmd
doskey hurl=C:\Program Files\HURL\hurl.exe $*
```

## Docker Installation

### Quick Start

```bash
# Run directly
docker run hurl get https://httpbin.org/get

# Show version
docker run hurl --version

# Interactive mode
docker run -it hurl /bin/bash
```

### Build Custom Image

```bash
# Clone repository
git clone https://github.com/hurl/hurl.git
cd hurl

# Build image
docker build -t hurl:latest .

# Test
docker run hurl --version
```

### Docker Compose

Using docker-compose for development or testing:

```yaml
version: '3.8'
services:
  hurl:
    image: hurl:latest
    volumes:
      - ./requests:/workspace/requests
    working_dir: /workspace
```

Run with:
```bash
docker-compose run --rm hurl get https://httpbin.org/get
```

### Docker Advanced Usage

**Mount local request files:**
```bash
docker run -v ~/requests:/workspace hurl \
  -f /workspace/api-test.hurl
```

**Use environment variables:**
```bash
docker run -e API_URL=https://api.example.com \
  hurl get $API_URL/users
```

**Keep container running:**
```bash
docker run -it --entrypoint /bin/bash hurl
# Inside container:
hurl --version
```

## Building from Source

### Prerequisites

- **Rust 1.70+**: Install from https://rustup.rs/
- **Cargo**: Included with Rust
- **Git**: Version control
- **C Compiler**: GCC, Clang, or MSVC
- **OpenSSL development files**

### Step-by-Step Build

```bash
# 1. Install Rust (if not already done)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 2. Clone repository
git clone https://github.com/hurl/hurl.git
cd hurl

# 3. Build release binary
cargo build --release

# 4. Binary location
ls -lah target/release/hurl

# 5. Test binary
./target/release/hurl --version

# 6. Install globally (optional)
cargo install --path crates/hurl-cli
```

### Build Options

```bash
# Fast debug build (testing during development)
cargo build

# Optimized release build
cargo build --release

# Build specific CLI tool
cargo build --release --bin hurl

# Build with specific features
cargo build --release --features "full"

# Build all packages
cargo build --release --all
```

### Verify Build

```bash
# Check binary works
./target/release/hurl --version

# Run tests
cargo test --release

# Check dependencies
cargo tree

# Verify code quality
cargo clippy --release
```

## Cross-Compilation

### Setup Cross-Compilation Environment

```bash
# Install cross utility
cargo install cross

# Install target toolchain
rustup target add aarch64-unknown-linux-gnu
rustup target add armv7-unknown-linux-gnueabihf
rustup target add x86_64-apple-darwin
rustup target add x86_64-pc-windows-gnu
```

### Build for Different Targets

```bash
# Linux ARM64
cross build --release --target aarch64-unknown-linux-gnu

# Linux ARMv7
cross build --release --target armv7-unknown-linux-gnueabihf

# macOS Intel (from macOS)
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# macOS ARM64 (from macOS)
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Windows (from Windows)
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target i686-pc-windows-msvc

# Windows (from Linux, requires MinGW)
cross build --release --target x86_64-pc-windows-gnu
```

### Using Build Scripts

Automated build scripts are provided:

```bash
# Build all platforms
bash scripts/build-all.sh

# Build Linux targets
bash scripts/build-linux.sh

# Build macOS targets
bash scripts/build-macos.sh

# Build Windows targets
bash scripts/build-windows.sh

# Create distribution packages
bash scripts/create-packages.sh
```

## Post-Installation

### Verify Installation

```bash
# Check version
hurl --version

# Check installation location
which hurl

# Run basic test
hurl get https://httpbin.org/get

# Test with headers
hurl get https://httpbin.org/headers \
  -H "User-Agent: HURL-Client"
```

### Shell Completions

**Bash:**
```bash
# Ubuntu/Debian (auto-installed)
source /usr/share/bash-completion/completions/hurl

# Or manually
source completions/hurl.bash
```

**Zsh:**
```bash
# Add to ~/.zshrc
fpath=(~/completions $fpath)
autoload -U compinit && compinit
source completions/hurl.zsh
```

**Fish:**
```bash
# Copy to Fish completions directory
cp completions/hurl.fish ~/.config/fish/completions/

# Or for system-wide
sudo cp completions/hurl.fish /usr/share/fish/vendor_completions.d/
```

### Man Page Access

```bash
# View man page
man hurl

# Search in man page
man hurl | grep -i "option"

# View specific section
man hurl | sed -n '/^EXAMPLES/,/^[A-Z]/p'
```

### Create Aliases (Optional)

```bash
# Add to ~/.bashrc or ~/.zshrc
alias h='hurl get'
alias hp='hurl post'
alias hurl-test='hurl --verbose'
```

## Uninstallation

### macOS

**Homebrew:**
```bash
brew uninstall hurl
brew cleanup
```

**MacPorts:**
```bash
sudo port uninstall hurl
```

**Manual:**
```bash
sudo rm /usr/local/bin/hurl
sudo rm /usr/local/share/man/man1/hurl.1
```

### Linux

**Ubuntu/Debian:**
```bash
sudo apt-get remove hurl
sudo apt-get autoremove
```

**Fedora/RHEL:**
```bash
sudo dnf remove hurl
```

**Snap:**
```bash
sudo snap remove hurl
```

**Manual binary:**
```bash
sudo rm /usr/local/bin/hurl
sudo rm /usr/local/share/man/man1/hurl.1
```

### Windows

**Chocolatey:**
```powershell
choco uninstall hurl
```

**Scoop:**
```powershell
scoop uninstall hurl
```

**Manual:**
```powershell
Remove-Item "C:\Program Files\HURL\hurl.exe"
# Remove from PATH manually via System Settings
```

### Cargo Installation

```bash
cargo uninstall hurl
```

### Clean Up

```bash
# Remove user configuration
rm -rf ~/.hurl

# Remove cached builds
rm -rf ~/Labs/rust-lua/hurl/target
```

## Troubleshooting

### Common Issues

#### Command not found
```bash
# Check if in PATH
echo $PATH

# Add to PATH manually
export PATH="/usr/local/bin:$PATH"

# Add to ~/.bashrc for persistence
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
```

#### SSL/TLS Certificate Errors
```bash
# Update CA certificates
# Ubuntu/Debian
sudo apt-get install ca-certificates
sudo update-ca-certificates

# macOS
brew install ca-certificates

# Use custom CA
hurl --cacert /path/to/ca.crt get https://example.com
```

#### Permission Denied
```bash
# Make binary executable
chmod +x /usr/local/bin/hurl

# Or reinstall with proper permissions
sudo cp target/release/hurl /usr/local/bin/
sudo chmod 755 /usr/local/bin/hurl
```

#### Building from Source Fails
```bash
# Update Rust toolchain
rustup update

# Update Cargo
cargo install --upgrade cargo

# Clean build artifacts
cargo clean

# Rebuild
cargo build --release
```

#### Port Already in Use (Docker)
```bash
# Use different port
docker run -p 9000:8080 hurl

# Or check what's using the port
lsof -i :8080  # macOS/Linux
netstat -ano | findstr :8080  # Windows
```

### Getting Help

- **GitHub Issues**: https://github.com/hurl/hurl/issues
- **Documentation**: https://github.com/hurl/hurl#readme
- **Man Page**: `man hurl`
- **Help Flag**: `hurl --help`

### Version Information

To help with troubleshooting, provide:

```bash
hurl --version
uname -a
echo $SHELL
rustc --version  # If built from source
```

---

**Last Updated**: October 2025  
**HURL Version**: 0.1.0+
