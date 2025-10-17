# Getting Started with HURL

Welcome to HURL! This guide will get you up and running with your first HTTP requests in 10 minutes.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Verification](#verification)
- [Your First Request](#your-first-request)
- [Common Patterns](#common-patterns)
- [Debugging Tips](#debugging-tips)
- [Performance Tuning Basics](#performance-tuning-basics)
- [Next Steps](#next-steps)
- [Frequently Asked Questions](#frequently-asked-questions)

## Prerequisites

Before installing HURL, ensure you have:

1. **Rust 1.70+** (only if building from source)
   ```bash
   rustc --version
   ```

2. **A working internet connection** (for package downloads)

3. **macOS, Linux, or Windows** (all platforms supported)

That's it! No additional dependencies needed.

## Installation

### Option 1: Binary Installation (Recommended)

Download pre-built binaries from the releases page:

```bash
# macOS with Homebrew
brew install hurl

# Linux with apt (Ubuntu/Debian)
sudo apt-get install hurl

# Or using cargo (works on all platforms)
cargo install hurl-cli
```

### Option 2: Build from Source

```bash
git clone https://github.com/hurl/hurl
cd hurl
cargo install --path crates/hurl-cli
```

This installs HURL to `~/.cargo/bin/hurl`. Ensure `~/.cargo/bin` is in your `$PATH`.

### Option 3: Docker

```bash
docker pull ghcr.io/hurl/hurl:latest
docker run ghcr.io/hurl/hurl:latest get https://example.com
```

Or create an alias:
```bash
alias hurl='docker run --rm ghcr.io/hurl/hurl:latest'
```

## Verification

Verify your installation:

```bash
hurl --version
```

You should see:
```
hurl 0.1.0
```

Check available commands:
```bash
hurl --help
```

## Your First Request

### Request #1: Simple GET

```bash
hurl get https://httpbin.org/get
```

You should see the JSON response with your request details.

### Request #2: With a Custom Header

```bash
hurl get https://httpbin.org/headers \
  -H "X-Custom-Header: Hello-HURL"
```

### Request #3: POST with Data

```bash
hurl post https://httpbin.org/post \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello from HURL"}'
```

### Request #4: Authentication

```bash
hurl get https://httpbin.org/basic-auth/user/passwd \
  -u user:passwd
```

### Request #5: Save Response to File

```bash
hurl get https://httpbin.org/json \
  -o response.json

cat response.json
```

Congratulations! You've successfully made your first requests with HURL.

## Common Patterns

### Pattern 1: Exploring an API

When exploring a new API:

```bash
# Get base information
hurl get https://api.example.com

# List resources
hurl get https://api.example.com/users

# Get specific resource
hurl get https://api.example.com/users/123

# Check headers
hurl head https://api.example.com/users
```

### Pattern 2: Testing Multiple Endpoints

Create a test script:

```bash
#!/bin/bash

BASE_URL="https://api.example.com"

echo "Testing health endpoint..."
hurl get $BASE_URL/health

echo "Testing users endpoint..."
hurl get $BASE_URL/users

echo "Testing posts endpoint..."
hurl get $BASE_URL/posts
```

Save as `test_api.sh`, then:
```bash
chmod +x test_api.sh
./test_api.sh
```

### Pattern 3: Using API Keys

```bash
API_KEY="your-api-key-here"

hurl get https://api.example.com/data \
  -H "X-API-Key: $API_KEY"
```

Or set as environment variable:

```bash
export API_KEY="your-api-key-here"

hurl get https://api.example.com/data \
  -H "X-API-Key: $API_KEY"
```

### Pattern 4: Bearer Token Authentication

```bash
TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."

hurl get https://api.example.com/users \
  -H "Authorization: Bearer $TOKEN"
```

### Pattern 5: Extracting Data with jq

Combine HURL with `jq` for data extraction:

```bash
# Get user list
hurl get https://api.example.com/users | jq '.users[] | .name'

# Get specific field
hurl get https://api.example.com/users/1 | jq '.email'

# Extract and use in another request
USER_ID=$(hurl get https://api.example.com/users | jq -r '.[0].id')
hurl get https://api.example.com/users/$USER_ID
```

### Pattern 6: Bulk Requests

Process multiple URLs:

```bash
URLs="https://api.example.com/users
https://api.example.com/posts
https://api.example.com/comments"

while IFS= read -r url; do
  echo "Requesting: $url"
  hurl get "$url"
  echo "---"
done <<< "$URLs"
```

## Debugging Tips

### Enable Verbose Output

See detailed request/response information:

```bash
hurl --verbose get https://api.example.com/data
```

Output includes:
- Request headers
- Response headers
- Timing information
- Redirect chains

### Check Response Headers Only

```bash
hurl head https://api.example.com/data
```

### Save Full Response

For detailed inspection:

```bash
hurl get https://api.example.com/data \
  -o full_response.json

cat full_response.json | jq '.'
```

### Test with Local Server

Start a local test server:

```bash
# Simple Python HTTP server
python3 -m http.server 8000

# In another terminal
hurl get http://localhost:8000/
```

### Checking Request Method

Verify the HTTP method being used:

```bash
# Explicit method confirmation
hurl get https://httpbin.org/get
hurl post https://httpbin.org/post
```

### Network Debugging

Check DNS and connectivity:

```bash
# Test connectivity
ping api.example.com

# Check DNS
nslookup api.example.com

# Trace route
traceroute api.example.com
```

## Performance Tuning Basics

### Basic Optimization

1. **Use connection pooling** (automatic in HURL)

2. **Reuse authentication tokens**:
   ```bash
   TOKEN=$(hurl post https://api.example.com/auth -d '{"user":"x","pass":"y"}' | jq -r '.token')
   for i in {1..100}; do
     hurl get https://api.example.com/users -H "Authorization: Bearer $TOKEN"
   done
   ```

3. **Batch similar requests**:
   ```bash
   # Good - reuse connection
   for id in {1..10}; do
     hurl get https://api.example.com/users/$id
   done
   
   # Better - parallel requests
   for id in {1..10}; do
     hurl get https://api.example.com/users/$id &
   done
   wait
   ```

4. **Reduce output verbosity**:
   ```bash
   hurl --quiet get https://api.example.com/data
   ```

5. **Set appropriate timeouts**:
   ```bash
   # Fast responses
   hurl get https://api.example.com/fast --timeout 5
   
   # Slow operations
   hurl post https://api.example.com/process --timeout 120
   ```

### Memory Considerations

For large responses:

```bash
# Direct to file instead of memory
hurl get https://api.example.com/large-file \
  -o large_data.json

# Process streaming (if available)
hurl --stream get https://api.example.com/stream
```

## Next Steps

After mastering the basics:

1. **Read API Reference**: Learn all available flags in [API_REFERENCE.md](API_REFERENCE.md)

2. **Explore Examples**: See 50+ real-world examples in [EXAMPLES.md](EXAMPLES.md)

3. **Advanced Features**:
   - Request chaining
   - Batch processing
   - Custom assertions
   - Configuration files

4. **Integration**:
   - Use in CI/CD pipelines
   - Integrate with shell scripts
   - Combine with other tools

5. **Performance Tuning**: Check [PERFORMANCE.md](PERFORMANCE.md) for optimization strategies

## Frequently Asked Questions

### Q: How do I save a response to a file?

A: Use the `-o` flag:
```bash
hurl get https://api.example.com/data -o response.json
```

### Q: How do I include multiple headers?

A: Use multiple `-H` flags:
```bash
hurl get https://api.example.com/data \
  -H "Authorization: Bearer token" \
  -H "X-Custom-Header: value" \
  -H "Accept: application/json"
```

### Q: How do I send JSON data?

A: Use the `-d` flag with JSON string:
```bash
hurl post https://api.example.com/users \
  -d '{"name": "John", "email": "john@example.com"}'
```

### Q: How do I use basic authentication?

A: Use the `-u` flag:
```bash
hurl get https://api.example.com/secure -u username:password
```

### Q: Can I chain requests together?

A: Yes, extract data and use in subsequent requests:
```bash
TOKEN=$(hurl post https://api.example.com/auth \
  -d '{"user":"x","pass":"y"}' | jq -r '.token')

hurl get https://api.example.com/users \
  -H "Authorization: Bearer $TOKEN"
```

### Q: How do I handle redirects?

A: HURL follows redirects by default. To see the chain:
```bash
hurl --verbose get https://example.com/redirect
```

### Q: What if I get SSL certificate errors?

A: For development/testing only, disable verification:
```bash
hurl --no-verify-ssl get https://api.example.com/data
```

### Q: How do I increase the timeout?

A: Use the `--timeout` flag (in seconds):
```bash
hurl get https://api.example.com/slow --timeout 60
```

### Q: Can I use environment variables?

A: Yes, use shell variable expansion:
```bash
export API_KEY="my-secret-key"
hurl get https://api.example.com/data \
  -H "X-API-Key: $API_KEY"
```

### Q: How do I extract specific fields from JSON responses?

A: Use `jq` tool:
```bash
hurl get https://api.example.com/users | jq '.users[0].name'
```

### Q: Can I make multiple requests in parallel?

A: Yes, use shell background processes:
```bash
hurl get https://api1.example.com &
hurl get https://api2.example.com &
wait
```

### Q: How do I compare responses?

A: Save responses and use diff:
```bash
hurl get https://api.example.com/users > response1.json
hurl get https://api.example.com/users > response2.json
diff response1.json response2.json
```

### Q: Is HURL suitable for production use?

A: Yes! HURL is production-ready and used in many environments. For critical applications, combine with proper error handling and monitoring.

### Q: How do I report issues?

A: Report bugs and feature requests at https://github.com/hurl/hurl/issues

## Getting Help

- 📚 Full documentation: See [README.md](../README.md)
- 🔍 API Reference: See [API_REFERENCE.md](API_REFERENCE.md)
- 📖 Examples: See [EXAMPLES.md](EXAMPLES.md)
- 🐛 Issues: https://github.com/hurl/hurl/issues
- 💬 Discussions: https://github.com/hurl/hurl/discussions
