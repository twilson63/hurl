# Release Notes Template

## HURL v0.1.0 - Initial Release

### Overview
HURL is a blazingly fast, user-friendly HTTP client written in Rust. This initial release includes full HTTP/1.1 and HTTP/2 support with modern features for API testing and HTTP interactions.

### Features
- ✨ **HTTP/1.1 & HTTP/2 Support** - Full protocol compliance
- 🚀 **Blazingly Fast** - Written in Rust for maximum performance
- 🎯 **Intuitive CLI** - Simple and user-friendly command interface
- 🔐 **Security First** - Built-in SSL/TLS support with mutual TLS
- 📦 **Data Format Support** - Native JSON and XML parsing
- 🎨 **Beautiful Output** - Colorized and formatted responses
- 💾 **Request History** - Automatic tracking of recent requests
- ✔️ **Built-in Testing** - Assertions and response validation
- 🍪 **Cookie Management** - Persistent cookies and session handling
- 📦 **Compression** - Support for gzip, deflate, brotli
- 🔑 **Authentication** - Basic, digest, bearer, and OAuth2 support

### Installation

#### macOS (Homebrew)
```bash
brew install hurl
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get install hurl
```

#### Windows (Chocolatey)
```powershell
choco install hurl
```

#### Docker
```bash
docker run hurl get https://httpbin.org/get
```

#### From Source
```bash
git clone https://github.com/hurl/hurl
cd hurl
cargo install --path crates/hurl-cli
```

### Quick Start

#### Basic GET request
```bash
hurl get https://api.example.com/users
```

#### POST with JSON data
```bash
hurl post https://api.example.com/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John", "email": "john@example.com"}'
```

#### With authentication
```bash
hurl get https://api.example.com/protected -u username:password
```

#### Save response to file
```bash
hurl get https://api.example.com/data -o response.json
```

### Breaking Changes
None - This is the initial release.

### Bug Fixes
- Initial implementation with comprehensive testing

### Known Issues
- Docker image size can be large (multi-stage builds help)
- Some older SSL/TLS configurations may not be supported
- Windows completions require manual installation

### Dependencies
- Rust 1.70 or later
- OpenSSL (libssl-dev on Linux)
- Standard C library

### Contributors
Thanks to all contributors who helped make this release possible!

### Downloads
- [Linux x86_64](https://github.com/hurl/hurl/releases/download/v0.1.0/hurl-0.1.0-x86_64-unknown-linux-gnu.tar.gz)
- [Linux ARM64](https://github.com/hurl/hurl/releases/download/v0.1.0/hurl-0.1.0-aarch64-unknown-linux-gnu.tar.gz)
- [macOS x86_64](https://github.com/hurl/hurl/releases/download/v0.1.0/hurl-0.1.0-x86_64-apple-darwin.tar.gz)
- [macOS ARM64](https://github.com/hurl/hurl/releases/download/v0.1.0/hurl-0.1.0-aarch64-apple-darwin.tar.gz)
- [Windows x86_64](https://github.com/hurl/hurl/releases/download/v0.1.0/hurl-0.1.0-x86_64-pc-windows-msvc.zip)

### Getting Help
- [Documentation](https://github.com/hurl/hurl#readme)
- [GitHub Issues](https://github.com/hurl/hurl/issues)
- [Discussion Forum](https://github.com/hurl/hurl/discussions)

### License
Licensed under either Apache License, Version 2.0 or MIT license at your option.
