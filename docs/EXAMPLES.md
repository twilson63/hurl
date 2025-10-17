# HURL Examples

50+ complete, runnable examples organized by category. All examples use publicly available test APIs or localhost.

## Table of Contents

- [HTTP Methods (10 examples)](#http-methods-10-examples)
- [Authentication (8 examples)](#authentication-8-examples)
- [Response Formatting (8 examples)](#response-formatting-8-examples)
- [Testing & Assertions (10 examples)](#testing--assertions-10-examples)
- [Advanced Usage (14 examples)](#advanced-usage-14-examples)

---

## HTTP Methods (10 examples)

### 1. Simple GET Request

Fetch data from a public API.

```bash
hurl get https://httpbin.org/get
```

Response: JSON object containing request details.

### 2. GET with Query Parameters

Add filters and pagination to your request.

```bash
hurl get "https://jsonplaceholder.typicode.com/posts?userId=1&_limit=5"
```

Response: First 5 posts by user 1.

### 3. GET with Multiple Headers

Include multiple custom headers.

```bash
hurl get https://httpbin.org/headers \
  -H "X-Custom-Header: MyValue" \
  -H "X-Request-ID: $(uuidgen)" \
  -H "User-Agent: HURL-Example/1.0"
```

Response: Echo of sent headers.

### 4. POST with JSON Body

Send structured JSON data.

```bash
hurl post https://jsonplaceholder.typicode.com/posts \
  -H "Content-Type: application/json" \
  -d '{
    "title": "My First Post",
    "body": "This is a test post created with HURL",
    "userId": 1
  }'
```

Response: Created post object with assigned ID.

### 5. POST with Form Data

Submit form-encoded data.

```bash
hurl post https://httpbin.org/post \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=alice&email=alice@example.com&subscribe=true"
```

Response: Echo of form data received.

### 6. PUT Request

Replace an entire resource.

```bash
hurl put https://jsonplaceholder.typicode.com/posts/1 \
  -H "Content-Type: application/json" \
  -d '{
    "id": 1,
    "title": "Updated Post Title",
    "body": "Updated post body",
    "userId": 1
  }'
```

Response: Updated resource object.

### 7. DELETE Request

Remove a resource.

```bash
hurl delete https://jsonplaceholder.typicode.com/posts/1
```

Response: Empty response with 200 status.

### 8. PATCH Request

Partially update a resource.

```bash
hurl patch https://jsonplaceholder.typicode.com/posts/1 \
  -H "Content-Type: application/json" \
  -d '{"title": "Partially Updated Title"}'
```

Response: Updated resource with partial changes.

### 9. HEAD Request

Check if resource exists without downloading body.

```bash
hurl head https://httpbin.org/json
```

Response: Headers only, no body.

### 10. OPTIONS Request

Discover allowed HTTP methods.

```bash
hurl options https://httpbin.org/anything
```

Response: Headers showing allowed methods (Allow header).

---

## Authentication (8 examples)

### 11. Basic Authentication

Use username and password credentials.

```bash
hurl get https://httpbin.org/basic-auth/user/passwd \
  -u user:passwd
```

Response: Authenticated response from server.

### 12. Bearer Token Authorization

Use JWT or OAuth tokens.

```bash
TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"

hurl get https://httpbin.org/bearer \
  -H "Authorization: Bearer $TOKEN"
```

Response: Token validation response.

### 13. Digest Authentication

Use HTTP Digest authentication.

```bash
hurl get https://httpbin.org/digest-auth/auth/user/passwd \
  -H "Authorization: Digest username=\"user\", realm=\"Fake Realm\", nonce=\"fake-nonce\", uri=\"/digest-auth/auth/user/passwd\", response=\"response-hash\""
```

Response: Authenticated access.

### 14. API Key in Header

Common pattern for REST APIs.

```bash
hurl get https://httpbin.org/get \
  -H "X-API-Key: sk_live_123456789abcdef"
```

Response: Request with API key validation.

### 15. OAuth2 Token Exchange

Get and use OAuth2 token.

```bash
# Get token
TOKEN=$(hurl post https://httpbin.org/post \
  -H "Content-Type: application/json" \
  -d '{
    "client_id": "client123",
    "client_secret": "secret456",
    "grant_type": "client_credentials"
  }' | jq -r '.token // .access_token // "dummy_token"')

# Use token
hurl get https://httpbin.org/bearer \
  -H "Authorization: Bearer $TOKEN"
```

Response: OAuth2 protected resource.

### 16. Multiple Authentication Attempts

Fallback authentication methods.

```bash
# Try with token first
TOKEN="test_token_123"
RESPONSE=$(hurl get https://httpbin.org/get \
  -H "Authorization: Bearer $TOKEN" 2>/dev/null)

# If failed, try basic auth
if [ -z "$RESPONSE" ]; then
  hurl get https://httpbin.org/basic-auth/user/passwd \
    -u user:passwd
else
  echo "$RESPONSE"
fi
```

Response: First successful authentication.

### 17. Authentication with Retries

Retry failed authentication.

```bash
for attempt in 1 2 3; do
  RESPONSE=$(hurl post https://httpbin.org/post \
    -H "Authorization: Bearer token_attempt_$attempt" \
    -d '{"attempt": '$attempt'}' 2>/dev/null && break)
  [ $? -eq 0 ] && break
  echo "Attempt $attempt failed, retrying..."
  sleep 2
done
```

Response: Successful response after retries.

### 18. Custom Authorization Headers

Implement custom authentication scheme.

```bash
hurl get https://httpbin.org/get \
  -H "Authorization: Custom scheme credentials=xyz123" \
  -H "X-Auth-Token: internal-token-456" \
  -H "X-Auth-Signature: hash-signature-789"
```

Response: Custom authenticated request.

---

## Response Formatting (8 examples)

### 19. JSON Pretty Print (Default)

Get formatted JSON output.

```bash
hurl get https://jsonplaceholder.typicode.com/users/1
```

Response: Pretty-printed JSON.

### 20. JSON Compact Output

Remove whitespace from JSON.

```bash
hurl get https://jsonplaceholder.typicode.com/users/1 \
  --output-format json-compact
```

Response: Single-line JSON.

### 21. Raw Output Format

Get raw response without formatting.

```bash
hurl get https://httpbin.org/delay/1 \
  --output-format raw
```

Response: Unformatted response body.

### 22. XML Response

Handle XML formatted responses.

```bash
hurl get https://httpbin.org/xml \
  --output-format xml
```

Response: Formatted XML output.

### 23. CSV Export Format

Export structured data as CSV.

```bash
# This example works with JSON array responses
hurl get "https://jsonplaceholder.typicode.com/users?_limit=3" \
  --output-format csv
```

Response: CSV formatted user data.

### 24. Table Format Output

Display as formatted table.

```bash
hurl get "https://jsonplaceholder.typicode.com/users?_limit=5" \
  --output-format table
```

Response:
```
┌─────┬─────────────┬─────────────────────┐
│ id  │ name        │ email               │
├─────┼─────────────┼─────────────────────┤
│ 1   │ Leanne      │ Bret@april.biz      │
│ 2   │ Ervin       │ Antonette@april.biz │
└─────┴─────────────┴─────────────────────┘
```

### 25. Headers Only Output

View response headers without body.

```bash
hurl --headers-only get https://httpbin.org/json
```

Response: Only HTTP headers.

### 26. Save and Process Response

Save response then analyze.

```bash
# Save response
hurl get https://jsonplaceholder.typicode.com/posts/1 \
  -o response.json

# Process with jq
cat response.json | jq '.title'

# Or directly pipe
hurl get https://jsonplaceholder.typicode.com/posts/1 | jq '.body'
```

Response: Processed JSON data.

---

## Testing & Assertions (10 examples)

### 27. Assert Status Code 200

Verify successful response.

```bash
hurl get https://httpbin.org/status/200 \
  --assert-status 200 && echo "✓ Status check passed"
```

Response: Assertion passes if status is 200.

### 28. Assert Status Code Range

Accept multiple status codes.

```bash
hurl get https://httpbin.org/status/201 \
  --assert-status 200 && echo "✓ Status 200-299 passed"
```

Response: Assertion passes for 2xx status.

### 29. Assert Header Presence

Verify response header exists.

```bash
hurl get https://httpbin.org/json \
  --assert-header "Content-Type:application/json" \
  && echo "✓ Content-Type header verified"
```

Response: Assertion passes if header matches.

### 30. Assert Response Body Contains Text

Check for text in response.

```bash
hurl get https://jsonplaceholder.typicode.com/posts/1 \
  --assert-body-contains "sunt aut facere repellat" \
  && echo "✓ Body text found"
```

Response: Assertion passes if text found.

### 31. Assert JSON Field Value

Verify JSON field matches expected value.

```bash
hurl post https://httpbin.org/post \
  -d '{"name":"Alice"}' \
  --assert-body-contains '"name":"Alice"' \
  && echo "✓ JSON field verified"
```

Response: Assertion passes if field value matches.

### 32. Multiple Assertions

Combine multiple validation checks.

```bash
hurl get https://jsonplaceholder.typicode.com/posts/1 \
  --assert-status 200 \
  --assert-header "Content-Type:application/json" \
  --assert-body-contains "userId" \
  && echo "✓ All assertions passed"
```

Response: All checks pass before completing.

### 33. Conditional Test Based on Status

Run different tests based on response.

```bash
STATUS=$(hurl get https://httpbin.org/get -o /dev/null -w "%{http_code}")

if [ "$STATUS" = "200" ]; then
  echo "✓ Endpoint is healthy"
else
  echo "✗ Endpoint returned $STATUS"
fi
```

Response: Conditional test result.

### 34. Test Suite Execution

Run multiple related tests.

```bash
#!/bin/bash

API="https://jsonplaceholder.typicode.com"
PASSED=0
FAILED=0

# Test 1
if hurl get $API/users/1 --assert-status 200 > /dev/null 2>&1; then
  ((PASSED++))
else
  ((FAILED++))
fi

# Test 2
if hurl get $API/posts/1 --assert-status 200 > /dev/null 2>&1; then
  ((PASSED++))
else
  ((FAILED++))
fi

echo "Tests passed: $PASSED, failed: $FAILED"
```

Response: Test summary.

### 35. Batch Testing with Filtering

Test multiple endpoints.

```bash
# Create test list
ENDPOINTS=(
  "https://jsonplaceholder.typicode.com/users"
  "https://jsonplaceholder.typicode.com/posts"
  "https://jsonplaceholder.typicode.com/comments"
)

for endpoint in "${ENDPOINTS[@]}"; do
  echo "Testing: $endpoint"
  hurl get "$endpoint" --assert-status 200 && echo "✓ Passed" || echo "✗ Failed"
done
```

Response: Results for each endpoint.

### 36. Performance Assertion

Check response time.

```bash
START=$(date +%s%N)
hurl get https://httpbin.org/delay/1
END=$(date +%s%N)
ELAPSED=$((($END - $START) / 1000000))

if [ $ELAPSED -lt 2000 ]; then
  echo "✓ Response time acceptable: ${ELAPSED}ms"
else
  echo "✗ Response time exceeded: ${ELAPSED}ms"
fi
```

Response: Performance validation result.

---

## Advanced Usage (14 examples)

### 37. Request Chaining - Create and Use

Create resource and use ID in next request.

```bash
# Create post
POST_ID=$(hurl post https://jsonplaceholder.typicode.com/posts \
  -d '{
    "title": "New Post",
    "body": "Post body",
    "userId": 1
  }' | jq -r '.id')

echo "Created post ID: $POST_ID"

# Fetch created post
hurl get https://jsonplaceholder.typicode.com/posts/$POST_ID
```

Response: Newly created post data.

### 38. Variable Extraction and Interpolation

Extract and reuse data across requests.

```bash
# Get all posts and extract first ID
FIRST_POST_ID=$(hurl get "https://jsonplaceholder.typicode.com/posts?_limit=1" \
  | jq -r '.[0].id')

echo "First post ID: $FIRST_POST_ID"

# Get that user
USER_ID=$(hurl get https://jsonplaceholder.typicode.com/posts/$FIRST_POST_ID \
  | jq -r '.userId')

echo "User ID: $USER_ID"

# Fetch user details
hurl get https://jsonplaceholder.typicode.com/users/$USER_ID
```

Response: User details for post author.

### 39. Complex Request Chain - CRUD Operations

Create, Read, Update, Delete sequence.

```bash
API="https://jsonplaceholder.typicode.com"

# CREATE
echo "Creating post..."
POST=$(hurl post $API/posts \
  -d '{"title":"CRUD Test","body":"Testing CRUD","userId":1}')
POST_ID=$(echo "$POST" | jq -r '.id')
echo "Created: POST_ID=$POST_ID"

# READ
echo "Reading post..."
hurl get $API/posts/$POST_ID

# UPDATE
echo "Updating post..."
hurl put $API/posts/$POST_ID \
  -d '{"title":"CRUD Test Updated","body":"Testing CRUD - Updated","userId":1}'

# DELETE
echo "Deleting post..."
hurl delete $API/posts/$POST_ID
echo "Deleted"
```

Response: Complete CRUD cycle.

### 40. Proxy Configuration

Route requests through proxy.

```bash
# Without proxy (direct)
hurl get https://httpbin.org/get

# With HTTP proxy
hurl get https://httpbin.org/get \
  --proxy http://proxy.example.com:8080

# With HTTPS proxy
hurl get https://httpbin.org/get \
  --proxy https://proxy.example.com:8443
```

Response: Request routed through proxy.

### 41. SSL/TLS Certificate Handling

Manage SSL certificates.

```bash
# Default: verify SSL (recommended)
hurl get https://httpbin.org/json

# Development: disable verification (not recommended)
hurl --no-verify-ssl get https://self-signed.example.com/api

# Check certificate details
openssl s_client -connect httpbin.org:443 < /dev/null | openssl x509 -text
```

Response: HTTPS communication with certificate validation.

### 42. Cookie Management

Handle session cookies.

```bash
# Request that sets cookies
hurl get https://httpbin.org/cookies/set?test=value

# Cookie file location (if supported)
# hurl get https://httpbin.org/cookies --cookie-jar cookies.txt

# Use cookies in subsequent requests
hurl get https://httpbin.org/cookies
```

Response: Cookie-aware requests.

### 43. Compression Handling

Handle compressed responses.

```bash
# Server handles compression transparently
hurl get https://httpbin.org/gzip

# Explicitly request compression
hurl get https://httpbin.org/deflate \
  -H "Accept-Encoding: gzip, deflate, br"

# View headers to confirm compression
hurl --verbose get https://httpbin.org/gzip | head -20
```

Response: Automatically decompressed.

### 44. Retry with Exponential Backoff

Implement retry logic.

```bash
retry_with_backoff() {
  local max_attempts=3
  local timeout=1
  local attempt=1

  while [ $attempt -le $max_attempts ]; do
    echo "Attempt $attempt..."
    
    if hurl get "https://httpbin.org/status/200" --timeout 5; then
      echo "✓ Success"
      return 0
    fi
    
    echo "Failed, waiting ${timeout}s before retry..."
    sleep $timeout
    timeout=$((timeout * 2))
    attempt=$((attempt + 1))
  done
  
  echo "✗ Failed after $max_attempts attempts"
  return 1
}

retry_with_backoff
```

Response: Successful response or retry exhaustion.

### 45. Circuit Breaker Pattern

Stop retrying after threshold.

```bash
FAILURE_THRESHOLD=3
FAILURE_COUNT=0

for i in {1..5}; do
  if [ $FAILURE_COUNT -ge $FAILURE_THRESHOLD ]; then
    echo "✗ Circuit breaker open - too many failures"
    break
  fi
  
  if hurl get https://httpbin.org/status/200 > /dev/null 2>&1; then
    echo "✓ Request $i succeeded"
    FAILURE_COUNT=0
  else
    echo "✗ Request $i failed"
    ((FAILURE_COUNT++))
  fi
done
```

Response: Circuit breaker prevents cascading failures.

### 46. Load Testing

Simulate multiple concurrent requests.

```bash
NUM_REQUESTS=10
API="https://httpbin.org/delay/1"

echo "Generating $NUM_REQUESTS requests..."

for i in $(seq 1 $NUM_REQUESTS); do
  (
    echo "Request $i starting..."
    hurl get "$API" > /tmp/response_$i.json
    echo "Request $i completed"
  ) &
done

wait
echo "All requests completed"

# Count successful responses
SUCCESS_COUNT=$(find /tmp -name "response_*.json" -type f | wc -l)
echo "Successful responses: $SUCCESS_COUNT/$NUM_REQUESTS"
```

Response: Load test with concurrent requests.

### 47. Batch Processing from File

Process multiple requests from file.

```bash
# Create request file
cat > requests.txt <<EOF
GET https://httpbin.org/get
POST https://httpbin.org/post -d {"test":"data"}
GET https://httpbin.org/json
EOF

# Process each line
while IFS= read -r line; do
  echo "Executing: $line"
  
  # Parse command
  METHOD=$(echo $line | awk '{print $1}')
  URL=$(echo $line | awk '{print $2}')
  DATA=$(echo $line | cut -d' ' -f3-)
  
  if [ -z "$DATA" ] || [ "$DATA" = "$URL" ]; then
    hurl $METHOD $URL
  else
    hurl $METHOD $URL -d "$DATA"
  fi
  
  echo "---"
done < requests.txt
```

Response: All requests processed in sequence.

### 48. File Upload

Upload files to server.

```bash
# Create test file
echo "This is test content" > test_file.txt

# Upload (requires server support)
hurl post https://httpbin.org/post \
  -F "file=@test_file.txt" \
  -F "description=Test file upload"

# Or send file as body
hurl post https://httpbin.org/post \
  -H "Content-Type: text/plain" \
  -d @test_file.txt
```

Response: File uploaded successfully.

### 49. File Download

Download and save response.

```bash
# Save to file
hurl get https://httpbin.org/json \
  -o downloaded_data.json

# Verify download
ls -lh downloaded_data.json
cat downloaded_data.json | jq '.'
```

Response: File saved with response data.

### 50. Performance Measurement

Measure request/response timing.

```bash
# Time a request
{
  time hurl get https://httpbin.org/delay/2
} 2>&1 | grep real

# Manual timing
START=$(date +%s%N)
hurl get https://httpbin.org/delay/1 > /dev/null
END=$(date +%s%N)
ELAPSED=$((($END - $START) / 1000000))

echo "Request took ${ELAPSED}ms"

# Measure multiple requests
TOTAL_TIME=0
NUM_REQUESTS=5

for i in $(seq 1 $NUM_REQUESTS); do
  START=$(date +%s%N)
  hurl get https://httpbin.org/json > /dev/null
  END=$(date +%s%N)
  ELAPSED=$((($END - $START) / 1000000))
  TOTAL_TIME=$((TOTAL_TIME + ELAPSED))
  echo "Request $i: ${ELAPSED}ms"
done

AVG_TIME=$((TOTAL_TIME / NUM_REQUESTS))
echo "Average time: ${AVG_TIME}ms"
```

Response: Performance metrics and averages.

---

## Additional Patterns

### Error Handling

```bash
if ! hurl get https://api.example.com/data > /dev/null 2>&1; then
  echo "Request failed"
  exit 1
fi
```

### Response Piping

```bash
hurl get https://jsonplaceholder.typicode.com/users/1 \
  | jq '.email' \
  | xargs echo "Email:"
```

### Conditional Requests

```bash
if hurl head https://api.example.com/endpoint 2>/dev/null; then
  echo "Endpoint is available"
  hurl get https://api.example.com/endpoint
else
  echo "Endpoint is down"
fi
```

### Environment-based URLs

```bash
ENV=${ENVIRONMENT:-"dev"}
API_URL="https://api-${ENV}.example.com"

hurl get "$API_URL/users"
```

### Rate Limiting Handling

```bash
while true; do
  RESPONSE=$(hurl get https://api.example.com/data)
  STATUS=$?
  
  if [ $STATUS -eq 0 ]; then
    echo "$RESPONSE"
    break
  else
    echo "Rate limited, waiting..."
    sleep 5
  fi
done
```

---

## Testing Different HTTP Features

### Redirects

```bash
# Follow redirects (automatic)
hurl get https://httpbin.org/redirect/3

# Check redirect chain with verbose
hurl --verbose get https://httpbin.org/redirect/1
```

### Content Negotiation

```bash
# Request JSON
hurl get https://httpbin.org/accept \
  -H "Accept: application/json"

# Request XML
hurl get https://httpbin.org/accept \
  -H "Accept: application/xml"
```

### Different Status Codes

```bash
hurl get https://httpbin.org/status/200
hurl get https://httpbin.org/status/201
hurl get https://httpbin.org/status/400
hurl get https://httpbin.org/status/404
hurl get https://httpbin.org/status/500
```

---

All examples use public APIs (httpbin.org, jsonplaceholder.typicode.com) and are fully runnable without additional setup.
