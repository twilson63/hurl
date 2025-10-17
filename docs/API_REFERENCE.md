# HURL API Reference

Complete reference documentation for all HURL commands, flags, and options.

## Table of Contents

- [Global Options](#global-options)
- [HTTP Methods](#http-methods)
- [Authentication Methods](#authentication-methods)
- [Output Formats](#output-formats)
- [Error Codes](#error-codes)
- [Exit Codes](#exit-codes)
- [Configuration Options](#configuration-options)

## Global Options

These options work with all commands:

### --version, -V

Display HURL version.

```bash
hurl --version
```

Output:
```
hurl 0.1.0
```

### --help, -h

Display help information.

```bash
hurl --help
hurl get --help
```

### --verbose, -v

Enable verbose output. Prints detailed request and response information.

```bash
hurl --verbose get https://api.example.com/users
```

Shows:
- Request method, URL, headers
- Response status, headers
- Connection timing
- Redirect chains

### --quiet, -q

Suppress all output except errors and results.

```bash
hurl --quiet get https://api.example.com/users
```

### --config

Path to configuration file (TOML format).

```bash
hurl --config ~/.hurl/config.toml get https://api.example.com/users
```

See [Configuration Guide](../README.md#configuration-guide) for file format.

## HTTP Methods

### GET

Send a GET request to retrieve data.

**Syntax:**
```bash
hurl get <URL> [OPTIONS]
```

**Arguments:**
- `<URL>` - Target URL (required)

**Options:**
- `-H, --header <HEADER>` - Add request header (can be used multiple times)
- `-u, --auth <AUTH>` - Basic authentication (user:password)
- `--timeout <SECONDS>` - Request timeout in seconds
- `-o, --output <FILE>` - Save response to file
- `--proxy <URL>` - Use HTTP proxy
- `--no-verify-ssl` - Disable SSL certificate verification
- `--follow-redirects <COUNT>` - Maximum redirects to follow (default: 5)

**Examples:**

```bash
# Simple GET
hurl get https://api.example.com/users

# With headers
hurl get https://api.example.com/users \
  -H "Authorization: Bearer token123" \
  -H "Accept: application/json"

# With authentication
hurl get https://api.example.com/secure -u admin:password

# With output file
hurl get https://api.example.com/data -o response.json

# With custom timeout
hurl get https://api.example.com/slow --timeout 60

# With proxy
hurl get https://api.example.com/data --proxy http://proxy.example.com:8080
```

### POST

Send a POST request with request body.

**Syntax:**
```bash
hurl post <URL> [OPTIONS]
```

**Arguments:**
- `<URL>` - Target URL (required)

**Options:**
- `-d, --data <DATA>` - Request body data (JSON or form-encoded)
- `-H, --header <HEADER>` - Add request header
- `-u, --auth <AUTH>` - Basic authentication
- `--timeout <SECONDS>` - Request timeout in seconds
- `-o, --output <FILE>` - Save response to file
- `--content-type <TYPE>` - Set Content-Type header (default: application/json)

**Body Formats:**

JSON:
```bash
hurl post https://api.example.com/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John", "email": "john@example.com"}'
```

Form Data:
```bash
hurl post https://api.example.com/form \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=john&password=secret"
```

Multipart Form:
```bash
hurl post https://api.example.com/upload \
  -H "Content-Type: multipart/form-data" \
  -d '{"file": "@/path/to/file.txt"}'
```

Raw Text:
```bash
hurl post https://api.example.com/logs \
  -H "Content-Type: text/plain" \
  -d "This is a log message"
```

**Examples:**

```bash
# Simple POST with JSON
hurl post https://api.example.com/users \
  -d '{"name": "Alice"}'

# POST with authentication
hurl post https://api.example.com/secure \
  -u admin:password \
  -d '{"data": "value"}'

# POST with multiple headers
hurl post https://api.example.com/users \
  -H "Authorization: Bearer token" \
  -H "X-Request-ID: 12345" \
  -d '{"name": "Bob"}'

# POST with custom timeout
hurl post https://api.example.com/process \
  --timeout 120 \
  -d '{"task": "process"}'
```

### PUT

Send a PUT request to replace entire resource.

**Syntax:**
```bash
hurl put <URL> [OPTIONS]
```

**Options:**
Same as POST command.

**Examples:**

```bash
# Replace entire resource
hurl put https://api.example.com/users/123 \
  -d '{"name": "Updated Name", "status": "active"}'

# With authentication
hurl put https://api.example.com/users/123 \
  -u admin:password \
  -d '{"name": "New Name"}'
```

### DELETE

Send a DELETE request to remove a resource.

**Syntax:**
```bash
hurl delete <URL> [OPTIONS]
```

**Options:**
- `-H, --header <HEADER>` - Add request header
- `-u, --auth <AUTH>` - Basic authentication
- `--timeout <SECONDS>` - Request timeout
- `-o, --output <FILE>` - Save response to file

**Examples:**

```bash
# Simple DELETE
hurl delete https://api.example.com/users/123

# DELETE with authentication
hurl delete https://api.example.com/users/123 -u admin:password

# DELETE with confirmation
hurl delete https://api.example.com/users/123 \
  -H "X-Confirm: true"
```

### PATCH

Send a PATCH request to partially update a resource.

**Syntax:**
```bash
hurl patch <URL> [OPTIONS]
```

**Options:**
Same as POST command.

**Examples:**

```bash
# Partial update
hurl patch https://api.example.com/users/123 \
  -d '{"status": "inactive"}'

# Update nested field
hurl patch https://api.example.com/users/123 \
  -d '{"profile": {"bio": "New bio"}}'
```

### HEAD

Send a HEAD request (GET without response body).

**Syntax:**
```bash
hurl head <URL> [OPTIONS]
```

**Options:**
- `-H, --header <HEADER>` - Add request header
- `-u, --auth <AUTH>` - Basic authentication
- `--timeout <SECONDS>` - Request timeout

**Examples:**

```bash
# Check resource existence
hurl head https://api.example.com/users/123

# Check headers only
hurl head https://api.example.com/data \
  -H "Authorization: Bearer token"
```

### OPTIONS

Send an OPTIONS request to get allowed methods.

**Syntax:**
```bash
hurl options <URL> [OPTIONS]
```

**Options:**
- `-H, --header <HEADER>` - Add request header
- `--timeout <SECONDS>` - Request timeout

**Examples:**

```bash
# Get allowed methods
hurl options https://api.example.com/users

# Check CORS headers
hurl options https://api.example.com/api \
  -H "Origin: https://example.com"
```

## Authentication Methods

### Basic Authentication

Use HTTP Basic Authentication (Base64-encoded credentials).

```bash
hurl get https://api.example.com/secure -u username:password
```

Equivalent to:
```bash
hurl get https://api.example.com/secure \
  -H "Authorization: Basic dXNlcm5hbWU6cGFzc3dvcmQ="
```

### Bearer Token

Use Bearer token in Authorization header.

```bash
hurl get https://api.example.com/data \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIs..."
```

Or save in variable:
```bash
TOKEN="eyJhbGciOiJIUzI1NiIs..."
hurl get https://api.example.com/data \
  -H "Authorization: Bearer $TOKEN"
```

### Digest Authentication

Use HTTP Digest Authentication.

```bash
hurl get https://api.example.com/secure \
  -H "Authorization: Digest username=\"user\", realm=\"realm\", nonce=\"nonce\", uri=\"/\", response=\"response\""
```

### OAuth2

Get token first, then use in requests.

```bash
# Get token
TOKEN=$(hurl post https://auth.example.com/token \
  -d '{"client_id":"id","client_secret":"secret","grant_type":"client_credentials"}' \
  | jq -r '.access_token')

# Use token
hurl get https://api.example.com/data \
  -H "Authorization: Bearer $TOKEN"
```

### API Key Authentication

Use API key in custom header.

```bash
hurl get https://api.example.com/data \
  -H "X-API-Key: sk_live_xyz123"
```

### Custom Headers

Use any custom authentication scheme.

```bash
hurl get https://api.example.com/data \
  -H "X-Custom-Auth: scheme credentials"

hurl get https://api.example.com/data \
  -H "Authorization: Custom token123"
```

### Multiple Authentication

Combine authentication methods.

```bash
hurl get https://api.example.com/data \
  -u admin:password \
  -H "X-API-Key: key123" \
  -H "Authorization: Bearer token456"
```

## Output Formats

### JSON (Default)

Output response as pretty-printed JSON.

```bash
hurl get https://api.example.com/users
```

Output:
```json
{
  "users": [
    {
      "id": 1,
      "name": "Alice",
      "email": "alice@example.com"
    }
  ]
}
```

### Compact JSON

Remove whitespace from JSON.

```bash
hurl get https://api.example.com/users --output-format json-compact
```

Output:
```json
{"users":[{"id":1,"name":"Alice","email":"alice@example.com"}]}
```

### Raw Output

Output response as raw text.

```bash
hurl get https://api.example.com/text --output-format raw
```

### XML Format

Output as formatted XML.

```bash
hurl get https://api.example.com/data --output-format xml
```

### CSV Format

Output as CSV.

```bash
hurl get https://api.example.com/users --output-format csv
```

Output:
```csv
id,name,email
1,Alice,alice@example.com
2,Bob,bob@example.com
```

### Table Format

Output as formatted table.

```bash
hurl get https://api.example.com/users --output-format table
```

Output:
```
┌─────┬───────┬──────────────────────┐
│ id  │ name  │ email                │
├─────┼───────┼──────────────────────┤
│ 1   │ Alice │ alice@example.com    │
│ 2   │ Bob   │ bob@example.com      │
└─────┴───────┴──────────────────────┘
```

### Headers Only

Output only response headers.

```bash
hurl --headers-only get https://api.example.com/data
```

Output:
```
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 1234
Date: Mon, 01 Jan 2024 12:00:00 GMT
```

## Error Codes

### HTTP Status Codes

| Code | Meaning | Action |
|------|---------|--------|
| 2xx | Success | Operation completed successfully |
| 200 | OK | Request successful |
| 201 | Created | Resource created |
| 204 | No Content | Success, no body |
| 3xx | Redirection | Follow redirect (automatic) |
| 301 | Moved Permanently | Resource moved (automatic redirect) |
| 302 | Found | Temporary redirect (automatic) |
| 304 | Not Modified | Resource not changed |
| 4xx | Client Error | Request problem |
| 400 | Bad Request | Invalid request format |
| 401 | Unauthorized | Authentication required |
| 403 | Forbidden | Permission denied |
| 404 | Not Found | Resource not found |
| 429 | Too Many Requests | Rate limited |
| 5xx | Server Error | Server problem |
| 500 | Internal Server Error | Server error |
| 502 | Bad Gateway | Invalid gateway response |
| 503 | Service Unavailable | Server temporarily unavailable |

### HURL Error Messages

| Error | Cause | Solution |
|-------|-------|----------|
| Connection refused | Server not running | Check server is running and URL is correct |
| Timeout | Request too slow | Increase timeout with `--timeout` |
| SSL certificate error | Invalid certificate | Use `--no-verify-ssl` (dev only) |
| Invalid URL | Malformed URL | Check URL format |
| Hostname not found | DNS resolution failed | Check domain name |
| No response | Server hung up | Check server status |
| Parse error | Invalid response format | Check Content-Type header |

## Exit Codes

HURL returns the following exit codes:

| Code | Meaning |
|------|---------|
| 0 | Success - request completed |
| 1 | General error |
| 2 | Connection error |
| 3 | Timeout error |
| 4 | Invalid arguments |
| 5 | Authentication failed |
| 6 | Assertion failed |
| 127 | Command not found |

**Usage in Scripts:**

```bash
hurl get https://api.example.com/users
if [ $? -eq 0 ]; then
  echo "Success"
else
  echo "Failed with exit code $?"
fi
```

## Configuration Options

### Command-Line Flags

| Flag | Type | Description |
|------|------|-------------|
| `--timeout` | seconds | Request timeout (default: 30) |
| `--proxy` | URL | HTTP proxy URL |
| `--no-verify-ssl` | boolean | Disable SSL verification |
| `--follow-redirects` | count | Max redirects to follow (default: 5) |
| `--max-retries` | count | Max retry attempts (default: 0) |
| `--output-format` | string | Response format (json, raw, xml, csv, table) |
| `--headers-only` | boolean | Show headers only |
| `--verbose` | boolean | Verbose output |
| `--quiet` | boolean | Quiet output |
| `--config` | path | Configuration file |

### Environment Variables

| Variable | Type | Description |
|----------|------|-------------|
| `HURL_CONFIG` | path | Default config file |
| `HURL_TIMEOUT` | seconds | Default timeout |
| `HURL_PROXY` | URL | Default proxy |
| `HURL_NO_COLOR` | boolean | Disable colors |
| `HURL_VERBOSE` | boolean | Default verbose mode |
| `HTTP_PROXY` | URL | HTTP proxy (standard) |
| `HTTPS_PROXY` | URL | HTTPS proxy (standard) |

**Usage:**

```bash
export HURL_TIMEOUT=60
export HURL_PROXY=http://proxy.example.com:8080
hurl get https://api.example.com/data
```

### Configuration File (TOML)

Create `~/.hurl/config.toml`:

```toml
[defaults]
timeout = 30
verify_ssl = true
follow_redirects = 5
max_retries = 0

[output]
format = "json"
colors = true
pretty_print = true

[headers]
"User-Agent" = "HURL/0.1.0"
"Accept" = "application/json"
"Accept-Encoding" = "gzip, deflate, br"

[auth]
default_auth = "basic"

[proxy]
http = "http://proxy.example.com:8080"
https = "https://proxy.example.com:8080"
no_proxy = "localhost,127.0.0.1"
```

**Load configuration:**

```bash
hurl --config ~/.hurl/config.toml get https://api.example.com/users
```

## Request Examples by Method

### GET Request Chain

```bash
# Get list of users
hurl get https://api.example.com/users \
  -H "Authorization: Bearer token"

# Get specific user
hurl get https://api.example.com/users/1 \
  -H "Authorization: Bearer token"

# Get user posts
hurl get https://api.example.com/users/1/posts \
  -H "Authorization: Bearer token"
```

### POST with Different Content Types

```bash
# JSON
hurl post https://api.example.com/data \
  -H "Content-Type: application/json" \
  -d '{"key": "value"}'

# Form Data
hurl post https://api.example.com/form \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "key=value&foo=bar"

# XML
hurl post https://api.example.com/xml \
  -H "Content-Type: application/xml" \
  -d '<root><key>value</key></root>'

# Plain Text
hurl post https://api.example.com/log \
  -H "Content-Type: text/plain" \
  -d "Log message"
```

### Batch Request Patterns

```bash
# Sequential requests
for id in 1 2 3 4 5; do
  hurl get https://api.example.com/users/$id
done

# Parallel requests (background)
for id in 1 2 3 4 5; do
  hurl get https://api.example.com/users/$id &
done
wait

# Conditional requests
hurl get https://api.example.com/users/1 && \
hurl get https://api.example.com/users/2 && \
hurl get https://api.example.com/users/3
```

## Response Handling

### Save Response to File

```bash
hurl get https://api.example.com/data -o response.json
```

### Extract Specific Fields

Use with `jq`:
```bash
hurl get https://api.example.com/users | jq '.users[] | {id, name}'
```

### Compare Responses

```bash
hurl get https://api.example.com/users > before.json
# Make changes
hurl get https://api.example.com/users > after.json
diff before.json after.json
```

### Stream Large Responses

For large files:
```bash
hurl get https://api.example.com/large-file -o large_file.json
```

## Troubleshooting Commands

### Verbose Mode

See all request/response details:
```bash
hurl --verbose post https://api.example.com/users \
  -d '{"name": "test"}'
```

### Test Connectivity

```bash
hurl get https://api.example.com/health
```

### Check Headers

```bash
hurl head https://api.example.com/users
```

### View Configuration

```bash
hurl --verbose --config ~/.hurl/config.toml get https://api.example.com/users
```

## Performance Tips

### Reduce Timeout for Fast Failure

```bash
hurl --timeout 5 get https://api.example.com/users
```

### Use Quiet Mode to Reduce Overhead

```bash
hurl --quiet get https://api.example.com/users
```

### Batch Operations Efficiently

```bash
for i in {1..100}; do
  hurl get https://api.example.com/users/$i &
done
wait
```

### Reuse Connections

HURL automatically pools connections. For maximum performance:

```bash
# Good: Sequential requests reuse connection
for i in {1..1000}; do
  hurl get https://api.example.com/users/$i
done
```
