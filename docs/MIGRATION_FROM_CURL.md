# Migration Guide: From cURL to HURL

A comprehensive guide to help cURL users transition to HURL (Modern HTTP CLI). This guide covers command mapping, common patterns, best practices, and tips for maximizing productivity.

**Target Audience**: Users experienced with cURL who want to leverage HURL's modern features  
**Last Updated**: October 16, 2025  
**HURL Version**: 0.1.0+

---

## Table of Contents

1. [Introduction](#introduction)
2. [Basic Syntax Comparison](#basic-syntax-comparison)
3. [HTTP Method Mapping](#http-method-mapping)
4. [Request Headers](#request-headers)
5. [Request Body](#request-body)
6. [Authentication](#authentication)
7. [Output Handling](#output-handling)
8. [Advanced Features](#advanced-features)
9. [Performance Optimization](#performance-optimization)
10. [Scripting & Automation](#scripting--automation)
11. [Common Patterns](#common-patterns)
12. [Troubleshooting](#troubleshooting)
13. [Feature Comparison](#feature-comparison)
14. [Tips & Tricks](#tips--tricks)

---

## Introduction

### Why Switch from cURL to HURL?

**HURL Advantages**:
- ✅ **Modern Rust Implementation** - Blazing fast performance
- ✅ **Friendly CLI** - More intuitive command structure
- ✅ **Request Chaining** - Execute sequences with variable extraction
- ✅ **Built-in Caching** - Smart response caching with TTL
- ✅ **History Tracking** - Search and replay past requests
- ✅ **Better Output** - Colorized, formatted responses
- ✅ **Assertions** - Built-in response validation
- ✅ **Batch Processing** - Execute multiple requests concurrently

**When to Keep cURL**:
- Extremely wide compatibility requirements
- Legacy system integration
- Minimal dependencies needed
- Specific cURL-only features

### Installation

```bash
# macOS (Homebrew)
brew install hurl

# Linux (Cargo)
cargo install hurl-cli

# From Source
git clone https://github.com/hurl/hurl
cd hurl
cargo install --path crates/hurl-cli
```

---

## Basic Syntax Comparison

### Simple GET Request

**cURL**:
```bash
curl https://api.example.com/users
```

**HURL**:
```bash
hurl get https://api.example.com/users
```

**Key Difference**: HURL explicitly specifies the HTTP method.

### POST Request with Data

**cURL**:
```bash
curl -X POST https://api.example.com/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John"}'
```

**HURL**:
```bash
hurl post https://api.example.com/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John"}'
```

**Difference**: No need for explicit `-X POST` (inferred from command).

### Saving Output

**cURL**:
```bash
curl https://api.example.com/users -o response.json
```

**HURL**:
```bash
hurl get https://api.example.com/users -o response.json
```

---

## HTTP Method Mapping

### Available Methods in HURL

| cURL | HURL | Purpose |
|------|------|---------|
| `curl` (GET) | `hurl get` | Retrieve data |
| `curl -X POST` | `hurl post` | Create resource |
| `curl -X PUT` | `hurl put` | Replace resource |
| `curl -X DELETE` | `hurl delete` | Remove resource |
| `curl -X PATCH` | `hurl patch` | Partial update |
| `curl -X HEAD` | `hurl head` | Headers only |
| `curl -X OPTIONS` | `hurl options` | Query options |

### Method-Specific Examples

**GET Request**:
```bash
# cURL
curl https://api.example.com/users/123

# HURL
hurl get https://api.example.com/users/123
```

**POST Request**:
```bash
# cURL
curl -X POST https://api.example.com/users -d '{"name":"John"}'

# HURL
hurl post https://api.example.com/users -d '{"name":"John"}'
```

**PUT Request**:
```bash
# cURL
curl -X PUT https://api.example.com/users/123 -d '{"name":"Jane"}'

# HURL
hurl put https://api.example.com/users/123 -d '{"name":"Jane"}'
```

**DELETE Request**:
```bash
# cURL
curl -X DELETE https://api.example.com/users/123

# HURL
hurl delete https://api.example.com/users/123
```

---

## Request Headers

### Single Header

**cURL**:
```bash
curl https://api.example.com/users \
  -H "X-API-Key: secret123"
```

**HURL**:
```bash
hurl get https://api.example.com/users \
  -H "X-API-Key: secret123"
```

### Multiple Headers

**cURL**:
```bash
curl https://api.example.com/users \
  -H "X-API-Key: secret123" \
  -H "Accept: application/json" \
  -H "User-Agent: MyApp/1.0"
```

**HURL**:
```bash
hurl get https://api.example.com/users \
  -H "X-API-Key: secret123" \
  -H "Accept: application/json" \
  -H "User-Agent: MyApp/1.0"
```

### Headers from File

**cURL**:
```bash
# Headers in file: headers.txt
# X-API-Key: secret123
# Accept: application/json

curl https://api.example.com/users -H @headers.txt
```

**HURL**:
```bash
# Use multiple -H flags for each header
hurl get https://api.example.com/users \
  -H "X-API-Key: secret123" \
  -H "Accept: application/json"
```

### Common Headers

**Authorization Header**:
```bash
# cURL
curl https://api.example.com/users \
  -H "Authorization: Bearer token123"

# HURL
hurl get https://api.example.com/users \
  -H "Authorization: Bearer token123"
```

**Content-Type**:
```bash
# cURL
curl https://api.example.com/users \
  -H "Content-Type: application/json" \
  -d '{"name":"John"}'

# HURL
hurl post https://api.example.com/users \
  -H "Content-Type: application/json" \
  -d '{"name":"John"}'
```

---

## Request Body

### JSON Data

**cURL**:
```bash
curl -X POST https://api.example.com/users \
  -H "Content-Type: application/json" \
  -d '{"name":"John","email":"john@example.com"}'
```

**HURL**:
```bash
hurl post https://api.example.com/users \
  -H "Content-Type: application/json" \
  -d '{"name":"John","email":"john@example.com"}'
```

### Form Data (URL-encoded)

**cURL**:
```bash
curl -X POST https://api.example.com/login \
  -d "username=john&password=secret"
```

**HURL**:
```bash
hurl post https://api.example.com/login \
  -d "username=john&password=secret"
```

### Multipart Form Data

**cURL**:
```bash
curl -X POST https://api.example.com/upload \
  -F "file=@document.pdf" \
  -F "description=My Document"
```

**HURL**:
```bash
hurl post https://api.example.com/upload \
  -F "file=@document.pdf" \
  -F "description=My Document"
```

### Body from File

**cURL**:
```bash
curl -X POST https://api.example.com/users \
  -d @request_body.json
```

**HURL**:
```bash
hurl post https://api.example.com/users \
  -d @request_body.json
```

### Raw Binary Data

**cURL**:
```bash
curl -X POST https://api.example.com/upload \
  --data-binary @image.png
```

**HURL**:
```bash
hurl post https://api.example.com/upload \
  --binary @image.png
```

---

## Authentication

### Basic Authentication

**cURL**:
```bash
curl https://api.example.com/protected \
  -u username:password
```

**HURL**:
```bash
hurl get https://api.example.com/protected \
  -u username:password
```

### Bearer Token

**cURL**:
```bash
curl https://api.example.com/protected \
  -H "Authorization: Bearer token123"
```

**HURL**:
```bash
hurl get https://api.example.com/protected \
  -H "Authorization: Bearer token123"
```

### API Key (Header)

**cURL**:
```bash
curl https://api.example.com/data \
  -H "X-API-Key: abc123def456"
```

**HURL**:
```bash
hurl get https://api.example.com/data \
  -H "X-API-Key: abc123def456"
```

### OAuth Token

**cURL**:
```bash
curl https://api.example.com/profile \
  -H "Authorization: Bearer $(get_oauth_token)"
```

**HURL**:
```bash
hurl get https://api.example.com/profile \
  -H "Authorization: Bearer $(get_oauth_token)"
```

### Digest Authentication

**cURL**:
```bash
curl https://api.example.com/secure \
  --digest -u username:password
```

**HURL** (v0.1.0):
```bash
# Not yet supported in v0.1.0
# Workaround: Use custom headers with pre-computed digest
hurl get https://api.example.com/secure \
  -H "Authorization: Digest ..."
```

---

## Output Handling

### Display Response Body (Default)

**cURL**:
```bash
curl https://api.example.com/users
# Output: response body to stdout
```

**HURL**:
```bash
hurl get https://api.example.com/users
# Output: response body to stdout
```

### Headers Only

**cURL**:
```bash
curl -i https://api.example.com/users
# or
curl -I https://api.example.com/users
```

**HURL**:
```bash
hurl get https://api.example.com/users --headers
# or
hurl head https://api.example.com/users
```

### Save to File

**cURL**:
```bash
curl https://api.example.com/users -o response.json
# or
curl https://api.example.com/users > response.json
```

**HURL**:
```bash
hurl get https://api.example.com/users -o response.json
```

### Append to File

**cURL**:
```bash
curl https://api.example.com/users >> responses.log
```

**HURL**:
```bash
hurl get https://api.example.com/users >> responses.log
```

### Silent Mode (No Output)

**cURL**:
```bash
curl -s https://api.example.com/users
# or
curl --silent https://api.example.com/users
```

**HURL**:
```bash
hurl get https://api.example.com/users -s
# or
hurl get https://api.example.com/users --silent
```

### Verbose Output

**cURL**:
```bash
curl -v https://api.example.com/users
# or
curl --verbose https://api.example.com/users
```

**HURL**:
```bash
hurl get https://api.example.com/users -v
# or
hurl get https://api.example.com/users --verbose
```

### Pretty Print JSON

**cURL**:
```bash
curl https://api.example.com/users | jq .
```

**HURL**:
```bash
hurl get https://api.example.com/users --pretty-json
# or (HURL automatically detects and formats JSON)
hurl get https://api.example.com/users
```

### Colorized Output

**cURL**:
```bash
curl https://api.example.com/users | pygmentize -g
```

**HURL**:
```bash
hurl get https://api.example.com/users
# Automatic colorization if terminal supports it
```

---

## Advanced Features

### Query Parameters

**cURL**:
```bash
curl "https://api.example.com/users?page=1&limit=10"
```

**HURL**:
```bash
hurl get "https://api.example.com/users?page=1&limit=10"
# or using query flags
hurl get https://api.example.com/users -q page=1 -q limit=10
```

### Timeouts

**cURL**:
```bash
curl --max-time 30 https://api.example.com/users
```

**HURL**:
```bash
hurl get https://api.example.com/users --timeout 30
```

### Redirects

**cURL**:
```bash
# Default: follow up to 10 redirects
curl https://api.example.com/redirect

# Disable redirects
curl -L https://api.example.com/redirect
curl --max-redirs 0 https://api.example.com/redirect
```

**HURL**:
```bash
# Default: follow redirects
hurl get https://api.example.com/redirect

# Limit redirects
hurl get https://api.example.com/redirect --max-redirects 5
```

### Proxy

**cURL**:
```bash
curl https://api.example.com/users \
  -x http://proxy.example.com:8080 \
  -U proxyuser:proxypass
```

**HURL**:
```bash
hurl get https://api.example.com/users \
  --proxy http://proxy.example.com:8080 \
  -u proxyuser:proxypass
```

### Cookies

**cURL**:
```bash
# Save cookies
curl https://api.example.com/login -c cookies.txt

# Use cookies
curl https://api.example.com/profile -b cookies.txt
```

**HURL**:
```bash
# HURL v0.1.0 handles cookies automatically
# No explicit cookie file needed in most cases
hurl post https://api.example.com/login
hurl get https://api.example.com/profile
```

### Retries

**cURL**:
```bash
# cURL doesn't have built-in retry, use shell loop
for i in {1..3}; do
  curl https://api.example.com/users && break
  sleep 1
done
```

**HURL** (v0.2.0 planned):
```bash
# Planned for v0.2.0
# For now, use shell loop as with cURL
```

### Request Chaining (HURL Exclusive)

**cURL**:
```bash
# 1. Get token
TOKEN=$(curl -s https://api.example.com/auth \
  -d '{"user":"john"}' | jq -r '.token')

# 2. Use token
curl -s https://api.example.com/profile \
  -H "Authorization: Bearer $TOKEN"
```

**HURL** (New!):
```bash
hurl request-chain --output json \
  --request "POST https://api.example.com/auth" \
  --body '{"user":"john"}' \
  --extract-var "token" "data.token" \
  --request "GET https://api.example.com/profile" \
  --header "Authorization: Bearer ${token}"
```

### Response Caching (HURL Exclusive)

**HURL** provides built-in caching:

```bash
# First request (cached)
hurl get https://api.example.com/users --cache

# Subsequent requests (from cache within TTL)
hurl get https://api.example.com/users --cache

# Clear cache
hurl cache clear
```

### Response Assertions (HURL Exclusive)

**cURL** (manual with jq):
```bash
RESPONSE=$(curl -s https://api.example.com/users)
STATUS=$(echo $RESPONSE | jq '.status')
if [ "$STATUS" = "200" ]; then
  echo "Success"
else
  echo "Failed"
fi
```

**HURL** (built-in):
```bash
hurl get https://api.example.com/users \
  --assert "status==200" \
  --assert "response.data[]|exists"
```

### History Tracking (HURL Exclusive)

**HURL** automatically tracks request history:

```bash
# View recent requests
hurl history list

# Search history
hurl history search --url "api.example.com"
hurl history search --method "POST" --status "200"

# Replay request
hurl history replay <id>
```

---

## Performance Optimization

### Parallel Requests

**cURL** (background jobs):
```bash
# Execute in parallel
for url in url1 url2 url3; do
  curl "$url" &
done
wait
```

**HURL** (batch processing):
```bash
hurl batch \
  https://api.example.com/users/1 \
  https://api.example.com/users/2 \
  https://api.example.com/users/3 \
  --concurrent 10
```

### Connection Reuse

**cURL**:
```bash
# Use -K/--config for keepalive settings
# Manual implementation needed
```

**HURL**:
```bash
# Automatic connection pooling
hurl get https://api.example.com/users
hurl get https://api.example.com/posts
# Reuses connection automatically
```

### Compression

**cURL**:
```bash
curl --compressed https://api.example.com/users
```

**HURL**:
```bash
# Automatic compression negotiation
hurl get https://api.example.com/users
# Automatically handles gzip/brotli
```

---

## Scripting & Automation

### Shell Integration

**cURL with jq**:
```bash
#!/bin/bash
response=$(curl -s https://api.example.com/users)
count=$(echo $response | jq '.data | length')
echo "User count: $count"
```

**HURL with jq**:
```bash
#!/bin/bash
response=$(hurl get https://api.example.com/users -s)
count=$(echo $response | jq '.data | length')
echo "User count: $count"
```

### Error Handling

**cURL**:
```bash
#!/bin/bash
curl -f https://api.example.com/users || {
  echo "Request failed"
  exit 1
}
```

**HURL**:
```bash
#!/bin/bash
hurl get https://api.example.com/users || {
  echo "Request failed"
  exit 1
}
```

### Batch File Processing

**cURL**:
```bash
#!/bin/bash
while IFS= read -r line; do
  curl -X POST https://api.example.com/users \
    -H "Content-Type: application/json" \
    -d "$line"
done < users.jsonl
```

**HURL**:
```bash
#!/bin/bash
hurl batch @users.jsonl \
  --endpoint https://api.example.com/users \
  --method POST \
  --concurrent 10
```

---

## Common Patterns

### Pattern 1: Login and Access Protected Endpoint

**cURL**:
```bash
# Get auth token
TOKEN=$(curl -s https://api.example.com/auth \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"secret"}' \
  | jq -r '.token')

# Use token
curl https://api.example.com/profile \
  -H "Authorization: Bearer $TOKEN"
```

**HURL**:
```bash
# Single chain operation
hurl request-chain \
  --request "POST https://api.example.com/auth" \
  --body '{"email":"user@example.com","password":"secret"}' \
  --extract-var token "data.token" \
  --request "GET https://api.example.com/profile" \
  --header "Authorization: Bearer \${token}"
```

### Pattern 2: Pagination Loop

**cURL**:
```bash
#!/bin/bash
page=1
while true; do
  response=$(curl -s "https://api.example.com/users?page=$page")
  count=$(echo $response | jq '.data | length')
  if [ $count -eq 0 ]; then break; fi
  echo $response | jq '.data[]'
  ((page++))
done
```

**HURL**:
```bash
#!/bin/bash
page=1
while true; do
  response=$(hurl get "https://api.example.com/users?page=$page" -s)
  count=$(echo $response | jq '.data | length')
  if [ $count -eq 0 ]; then break; fi
  echo $response | jq '.data[]'
  ((page++))
done
```

### Pattern 3: Conditional Requests

**cURL**:
```bash
#!/bin/bash
response=$(curl -s https://api.example.com/users)
status=$(echo $response | jq -r '.status')

if [ "$status" = "success" ]; then
  echo "Processing successful"
else
  echo "Processing failed: $status"
  exit 1
fi
```

**HURL**:
```bash
#!/bin/bash
hurl get https://api.example.com/users \
  --assert "status=='success'" || exit 1
echo "Processing successful"
```

### Pattern 4: Data Transformation

**cURL**:
```bash
#!/bin/bash
curl -s https://api.example.com/users \
  | jq '[.data[] | {name: .full_name, email: .email_address}]'
```

**HURL**:
```bash
#!/bin/bash
hurl get https://api.example.com/users -s \
  | jq '[.data[] | {name: .full_name, email: .email_address}]'
```

---

## Troubleshooting

### Issue: "Command not found: hurl"

**Solution**:
```bash
# Ensure HURL is in PATH
which hurl

# If not found, reinstall
cargo install hurl-cli

# Or add to PATH if installed from source
export PATH="$PATH:/path/to/hurl/target/release"
```

### Issue: SSL Certificate Errors

**cURL**:
```bash
curl -k https://untrusted.example.com/api
```

**HURL**:
```bash
hurl get https://untrusted.example.com/api --insecure
```

### Issue: Timeout Errors

**cURL**:
```bash
curl --max-time 60 https://slow-api.example.com/data
```

**HURL**:
```bash
hurl get https://slow-api.example.com/data --timeout 60
```

### Issue: Connection Refused

**Both cURL and HURL**:
```bash
# Verify endpoint is running
# Check firewall rules
# Verify correct hostname/IP
```

### Issue: JSON Response Not Pretty-Printed

**HURL**:
```bash
# HURL should auto-detect and colorize JSON
# If not, use jq
hurl get https://api.example.com/users | jq .

# Or force pretty-print
hurl get https://api.example.com/users --pretty-json
```

---

## Feature Comparison

### Feature Matrix

| Feature | cURL | HURL | Notes |
|---------|------|------|-------|
| Basic HTTP Requests | ✅ | ✅ | Both excellent |
| Headers | ✅ | ✅ | Identical |
| Authentication | ✅ | ✅ | HURL simpler |
| Output Control | ✅ | ✅ | HURL more intuitive |
| Compression | ✅ | ✅ | Automatic in HURL |
| Cookies | ✅ | ✅ | HURL automatic |
| Proxies | ✅ | ✅ | Both supported |
| Timeouts | ✅ | ✅ | Both excellent |
| SSL/TLS | ✅ | ✅ | Both robust |
| Request Chaining | ❌ | ✅ | HURL exclusive |
| Response Caching | ❌ | ✅ | HURL exclusive |
| History Tracking | ❌ | ✅ | HURL exclusive |
| Built-in Assertions | ❌ | ✅ | HURL exclusive |
| Batch Processing | ❌ | ✅ | HURL exclusive |
| Cross-platform | ✅ | ✅ | Both excellent |
| Performance | Good | Excellent | HURL faster |
| Ease of Use | Moderate | High | HURL more intuitive |

---

## Tips & Tricks

### Tip 1: Use Variables for Repeated Values

**cURL**:
```bash
API="https://api.example.com"
KEY="your-api-key"
curl "$API/users" -H "X-API-Key: $KEY"
curl "$API/posts" -H "X-API-Key: $KEY"
```

**HURL**:
```bash
API="https://api.example.com"
KEY="your-api-key"
hurl get "$API/users" -H "X-API-Key: $KEY"
hurl get "$API/posts" -H "X-API-Key: $KEY"
```

### Tip 2: Create Response Templates

**Create a helper script** (`api.sh`):
```bash
#!/bin/bash
API_BASE="https://api.example.com"
API_KEY="your-api-key"

api_get() {
  hurl get "$API_BASE$1" -H "X-API-Key: $API_KEY"
}

api_post() {
  hurl post "$API_BASE$1" \
    -H "X-API-Key: $API_KEY" \
    -H "Content-Type: application/json" \
    -d "$2"
}

# Usage
api_get "/users"
api_post "/users" '{"name":"John"}'
```

### Tip 3: Combine with jq for Data Processing

**Extract specific fields**:
```bash
hurl get https://api.example.com/users -s | jq '.data[] | select(.status=="active") | .email'
```

### Tip 4: Use Response History for Debugging

```bash
# View recent requests
hurl history list

# Find problematic request
hurl history search --status "500"

# Examine details
hurl history show <id>
```

### Tip 5: Cache Frequently Accessed Data

```bash
# First request (makes HTTP call)
hurl get https://api.example.com/config --cache

# Second request (uses cache)
hurl get https://api.example.com/config --cache

# Verify cache hit
hurl cache stats
```

### Tip 6: Use Batch Mode for Performance

```bash
# Instead of:
for user_id in {1..100}; do
  hurl get "https://api.example.com/users/$user_id"
done

# Use batch mode:
hurl batch \
  $(for i in {1..100}; do echo "https://api.example.com/users/$i"; done) \
  --concurrent 20
```

### Tip 7: Enable Request Chaining for Complex Workflows

```bash
# Multi-step process in single command
hurl request-chain \
  --step1 "POST https://api.example.com/auth" --body '{"credentials":"..."}' \
  --extract token "response.data.token" \
  --step2 "GET https://api.example.com/profile" --header "Authorization: Bearer ${token}" \
  --extract user_id "response.data.id" \
  --step3 "GET https://api.example.com/settings/${user_id}"
```

---

## Migration Checklist

Use this checklist when migrating your scripts from cURL to HURL:

- [ ] Install HURL
- [ ] Update all `curl` commands to `hurl <method>`
- [ ] Verify all headers work correctly
- [ ] Test authentication flows
- [ ] Verify output handling (file saves, etc.)
- [ ] Update timeout settings if needed
- [ ] Test redirect handling
- [ ] Create helper scripts for repeated patterns
- [ ] Consider using history tracking for debugging
- [ ] Evaluate caching opportunities
- [ ] Test batch operations where applicable
- [ ] Migrate assertions from shell logic to HURL assertions

---

## Resources

### Official Documentation
- [HURL README](../README.md)
- [HURL Architecture](../ARCHITECTURE.md)
- [HURL Development Guide](../DEVELOPMENT.md)

### Related Tools
- [cURL Manual](https://curl.se/docs/)
- [jq Manual](https://stedolan.github.io/jq/)
- [Postman Collection Import](docs/postman-import.md) (planned v0.2.0)

### Community
- GitHub Issues: https://github.com/hurl/hurl/issues
- GitHub Discussions: https://github.com/hurl/hurl/discussions

---

## Frequently Asked Questions

**Q: Will HURL run all my cURL scripts?**  
A: Most scripts will work with minor modifications. See command mapping above.

**Q: Is HURL faster than cURL?**  
A: For single requests, similar. For batch operations, HURL is significantly faster due to parallel processing.

**Q: Can I use HURL in CI/CD pipelines?**  
A: Yes, HURL is designed for automation and scripting.

**Q: Does HURL support HTTP/2?**  
A: Yes, automatically detected and used when available.

**Q: Can I disable SSL verification?**  
A: Yes, use `--insecure` flag (not recommended for production).

**Q: How do I report bugs?**  
A: Open an issue on GitHub: https://github.com/hurl/hurl/issues

---

## Conclusion

HURL provides a modern, user-friendly alternative to cURL with additional features for request chaining, caching, history, and batch processing. While cURL remains universally compatible, HURL excels for HTTP client scripting and automation tasks.

**Key Takeaways**:
1. HURL syntax is more intuitive (explicit method names)
2. HURL offers unique features (chaining, caching, history)
3. Migration is straightforward for most use cases
4. Combine HURL with jq for powerful data processing
5. Use batch mode for significant performance gains

For questions, feedback, or issues, please visit the [HURL GitHub repository](https://github.com/hurl/hurl).

---

**Document Version**: 1.0  
**Last Updated**: October 16, 2025  
**HURL Version**: 0.1.0+  
**Status**: Complete and Ready for Use
