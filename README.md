# HURL - Modern HTTP CLI

A blazingly fast, user-friendly HTTP client written in Rust. HURL combines the simplicity of cURL with the power of postman, all while delivering blazing-fast performance through Rust's zero-cost abstractions.

## Table of Contents

- [Features](#features)
- [Value Proposition](#value-proposition)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Basic Usage Examples](#basic-usage-examples)
- [Advanced Usage Guide](#advanced-usage-guide)
- [Testing with Assertions](#testing-with-assertions)
- [Request Chaining](#request-chaining)
- [Batch Processing](#batch-processing)
- [Configuration Guide](#configuration-guide)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)

## Features

- 🚀 **Blazingly Fast** - Written in Rust for maximum performance with zero-copy operations
- 🎯 **Simple & Intuitive** - Minimalist command-line interface that feels natural
- 🔌 **HTTP/1.1 & HTTP/2** - Full protocol support with automatic negotiation
- 🔐 **Secure** - Built-in SSL/TLS support with certificate verification
- 📦 **Multiple Formats** - Native JSON, XML, CSV, and Table output formats
- 🎨 **Colorized Output** - Beautiful syntax-highlighted responses
- 💾 **Request History** - Automatic tracking of recent requests for reference
- ✨ **Built-in Testing** - Request/response assertions without external test frameworks
- 🔗 **Request Chaining** - Extract data from responses and use in subsequent requests
- 📝 **Batch Processing** - Execute multiple requests from a single file
- 🔐 **Authentication** - Support for Basic, Bearer, Digest, and OAuth2
- 📊 **Performance Metrics** - Built-in timing and analysis tools
- 🔄 **Compression** - Automatic gzip, brotli, and deflate support
- 🍪 **Cookie Management** - Persistent and session cookie handling

## Value Proposition

HURL solves the modern API developer's workflow challenges:

**For API Development:**
- Faster than cURL, simpler than Postman
- Perfect for quick API exploration and debugging
- Zero configuration required - just run and go

**For Testing & CI/CD:**
- Replace cURL scripts with a more powerful, readable tool
- Built-in assertions eliminate the need for external test frameworks
- Chainable requests for complex test scenarios

**For DevOps & Automation:**
- Lightweight binary with no runtime dependencies
- Perfect for server environments and containers
- Superior performance for load testing and batch operations

**For Learning & Documentation:**
- Clear, readable syntax that serves as API documentation
- Self-documenting request examples
- Easy to share and version control

## Quick Start

Get your first API request working in 5 minutes.

### Installation (Latest)

```bash
cargo install hurl-cli
```

### Your First Request

```bash
hurl get https://api.example.com/users
```

### With Headers

```bash
hurl get https://api.example.com/users \
  -H "Authorization: Bearer token123" \
  -H "Accept: application/json"
```

### POST with JSON Data

```bash
hurl post https://api.example.com/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com"}'
```

### Save to File

```bash
hurl get https://api.example.com/users -o users.json
```

### With Authentication

```bash
hurl get https://api.example.com/protected -u username:password
```

### With Timeout

```bash
hurl get https://api.example.com/slow --timeout 30
```

## Installation

### Build from Source

```bash
git clone https://github.com/hurl/hurl
cd hurl
cargo install --path crates/hurl-cli
```

### Verify Installation

```bash
hurl --version
```

### Build Commands

```bash
make build           # Debug build
make build-release   # Optimized release build
```

## Basic Usage Examples

### Example 1: Simple GET Request

```bash
hurl get https://httpbin.org/get
```

### Example 2: GET with Query Parameters

```bash
hurl get "https://api.example.com/users?page=1&limit=10"
```

### Example 3: GET with Multiple Headers

```bash
hurl get https://api.example.com/users \
  -H "Authorization: Bearer token" \
  -H "User-Agent: MyApp/1.0" \
  -H "Accept-Language: en-US"
```

### Example 4: POST with JSON Body

```bash
hurl post https://api.example.com/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Alice Johnson",
    "email": "alice@example.com",
    "role": "admin"
  }'
```

### Example 5: POST with Form Data

```bash
hurl post https://api.example.com/form \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=alice&password=secret&remember=true"
```

### Example 6: PUT Request

```bash
hurl put https://api.example.com/users/123 \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice Smith", "status": "active"}'
```

### Example 7: DELETE Request

```bash
hurl delete https://api.example.com/users/123
```

### Example 8: PATCH Request

```bash
hurl patch https://api.example.com/users/123 \
  -H "Content-Type: application/json" \
  -d '{"status": "inactive"}'
```

### Example 9: HEAD Request

```bash
hurl head https://api.example.com/users/123
```

### Example 10: OPTIONS Request

```bash
hurl options https://api.example.com/users
```

### Example 11: Basic Authentication

```bash
hurl get https://api.example.com/secure -u admin:password123
```

### Example 12: Bearer Token Authentication

```bash
hurl get https://api.example.com/data \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIs..."
```

### Example 13: Save Response to File

```bash
hurl get https://api.example.com/data -o response.json
```

### Example 14: Large Response Handling

```bash
hurl get https://api.example.com/large-dataset --timeout 60
```

### Example 15: Request with Retry

```bash
# Retry logic can be scripted or handled at shell level
for i in {1..3}; do
  hurl get https://api.example.com/data && break || sleep 2
done
```

## Advanced Usage Guide

### Request Chaining

Extract data from one request and use it in the next:

```bash
# First request to get a token
TOKEN=$(hurl post https://api.example.com/auth/login \
  -d '{"username":"admin","password":"secret"}' \
  | jq -r '.token')

# Use token in subsequent request
hurl get https://api.example.com/users \
  -H "Authorization: Bearer $TOKEN"
```

### Custom Headers and Authentication

```bash
hurl get https://api.example.com/data \
  -H "X-API-Key: sk_live_xyz123" \
  -H "X-Request-ID: $(uuidgen)" \
  -H "User-Agent: MyApp/2.0"
```

### Handling Different Response Formats

**JSON:**
```bash
hurl get https://api.example.com/users
```

**XML:**
```bash
hurl get https://api.example.com/data \
  -H "Accept: application/xml"
```

### Working with Different HTTP Status Codes

```bash
# Expect 201 Created
hurl post https://api.example.com/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Bob"}'

# Expect 404 Not Found
hurl get https://api.example.com/users/999999
```

### Timeout Configuration

```bash
# 5 second timeout
hurl get https://api.example.com/slow --timeout 5

# 60 second timeout for large operations
hurl post https://api.example.com/upload --timeout 60
```

### Verbose Output for Debugging

```bash
hurl --verbose get https://api.example.com/users
```

### Quiet Mode (Errors Only)

```bash
hurl --quiet get https://api.example.com/users
```

### Using Configuration Files

Create `.hurl/config.toml`:

```toml
[default]
timeout = 30
default_headers = [
  "User-Agent: HURL/0.1.0",
  "Accept: application/json"
]
```

Load configuration:

```bash
hurl --config .hurl/config.toml get https://api.example.com/users
```

## Testing with Assertions

HURL includes built-in testing capabilities:

### Assert Status Code

```bash
hurl get https://api.example.com/users --assert-status 200
```

### Assert Header Presence

```bash
hurl get https://api.example.com/data \
  --assert-header "Content-Type:application/json"
```

### Assert Response Body Contains Text

```bash
hurl get https://api.example.com/users \
  --assert-body-contains "alice@example.com"
```

### Complex Assertions with JSON

```bash
hurl post https://api.example.com/users \
  -d '{"name":"Charlie"}' \
  --assert-status 201
```

### Multiple Assertions

```bash
hurl get https://api.example.com/users \
  --assert-status 200 \
  --assert-header "Content-Type:application/json" \
  --assert-body-contains "users"
```

## Request Chaining

Execute multiple related requests:

```bash
# Create a user
USER_ID=$(hurl post https://api.example.com/users \
  -d '{"name":"David"}' \
  | jq -r '.id')

# Get the user
hurl get https://api.example.com/users/$USER_ID

# Update the user
hurl put https://api.example.com/users/$USER_ID \
  -d '{"status":"verified"}'

# Delete the user
hurl delete https://api.example.com/users/$USER_ID
```

## Batch Processing

Process multiple requests from a file:

Create `requests.txt`:

```
GET https://api.example.com/users
GET https://api.example.com/posts
GET https://api.example.com/comments
POST https://api.example.com/users -d {"name":"Eve"}
```

Execute batch:

```bash
hurl batch requests.txt
```

Or via stdin:

```bash
cat requests.txt | hurl batch -
```

## Configuration Guide

### Environment Variables

HURL respects these environment variables:

- `HURL_CONFIG` - Path to configuration file
- `HURL_TIMEOUT` - Default timeout in seconds
- `HURL_PROXY` - HTTP proxy URL
- `HURL_NO_COLOR` - Disable colored output
- `HURL_VERBOSE` - Enable verbose output

### Configuration File Format

Create `~/.hurl/config.toml`:

```toml
[defaults]
timeout = 30
verify_ssl = true
follow_redirects = true
max_redirects = 5

[output]
format = "json"
colors = true
pretty_print = true

[headers]
"User-Agent" = "HURL/0.1.0"
"Accept" = "application/json"

[auth]
default_auth = "basic"
```

### Per-Request Configuration

```bash
hurl --timeout 60 \
     --no-verify-ssl \
     --follow-redirects 10 \
     get https://api.example.com/data
```

## Troubleshooting

### Connection Refused

**Problem:** `Connection refused` error

**Solution:** Verify the URL is correct and the service is running:
```bash
hurl get https://localhost:8080/api
```

### Timeout Errors

**Problem:** Request times out

**Solution:** Increase timeout value:
```bash
hurl get https://api.example.com/slow --timeout 60
```

### SSL Certificate Errors

**Problem:** `SSL certificate verification failed`

**Solution:** Use `--no-verify-ssl` (use with caution):
```bash
hurl --no-verify-ssl get https://api.example.com/data
```

### Authentication Failures

**Problem:** `401 Unauthorized`

**Solution:** Verify credentials:
```bash
hurl get https://api.example.com/secure -u username:password
```

### Proxy Issues

**Problem:** Cannot reach URL through proxy

**Solution:** Configure proxy:
```bash
hurl get https://api.example.com/data --proxy http://proxy.example.com:8080
```

For more troubleshooting, see [docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md).

## Contributing

We welcome contributions! Here's how to get started:

### Development Setup

```bash
git clone https://github.com/hurl/hurl
cd hurl
cargo build
cargo test
```

### Code Guidelines

1. Follow Rust naming conventions
2. Write tests for new features
3. Run `make fmt` and `make lint` before submitting
4. Update documentation for API changes

### Commit Message Format

```
type(scope): description

feat(auth): add OAuth2 support
fix(http): handle redirects properly
docs(readme): update examples
```

### Submitting Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -am 'Add feature'`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Create a Pull Request

### Running Tests

```bash
make test          # Run all tests
make test-unit     # Run unit tests
make test-cli      # Run CLI tests
```

### Building Release

```bash
make build-release
./target/release/hurl --version
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

## Quick Reference

| Command | Description |
|---------|-------------|
| `hurl get <URL>` | Send GET request |
| `hurl post <URL>` | Send POST request |
| `hurl put <URL>` | Send PUT request |
| `hurl delete <URL>` | Send DELETE request |
| `hurl patch <URL>` | Send PATCH request |
| `hurl head <URL>` | Send HEAD request |
| `hurl options <URL>` | Send OPTIONS request |

## More Resources

- [Getting Started Guide](docs/GETTING_STARTED.md)
- [API Reference](docs/API_REFERENCE.md)
- [Examples](docs/EXAMPLES.md)
- [Architecture Guide](docs/ARCHITECTURE.md)
- [Performance Guide](docs/PERFORMANCE.md)
- [Troubleshooting Guide](docs/TROUBLESHOOTING.md)

## Support

- 📖 [Documentation](docs/)
- 🐛 [Issue Tracker](https://github.com/hurl/hurl/issues)
- 💬 [Discussions](https://github.com/hurl/hurl/discussions)
- 📧 [Email Support](mailto:support@hurl.dev)
