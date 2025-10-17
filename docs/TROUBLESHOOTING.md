# HURL Troubleshooting Guide

Comprehensive guide to solving common HURL issues and debugging problems.

## Table of Contents

- [Connection Issues](#connection-issues)
- [Timeout Problems](#timeout-problems)
- [SSL/TLS Issues](#ssltls-issues)
- [Authentication Failures](#authentication-failures)
- [Proxy Issues](#proxy-issues)
- [Memory Issues](#memory-issues)
- [Performance Problems](#performance-problems)
- [Assertion Failures](#assertion-failures)
- [Parsing Errors](#parsing-errors)
- [File Operations](#file-operations)
- [Network Issues](#network-issues)
- [Debug Logging](#debug-logging)
- [FAQ](#faq)

## Connection Issues

### Issue: Connection Refused

**Error Message:**
```
error: Connection refused (os error 111)
```

**Causes:**
- Server not running
- Wrong port
- Firewall blocking connection
- Service not bound to interface

**Solutions:**

1. **Verify server is running:**
   ```bash
   # Check if port is listening
   netstat -tlnp | grep :8080
   lsof -i :8080
   ```

2. **Test with correct URL:**
   ```bash
   # Wrong
   hurl get http://localhost:8080/api
   
   # Right (verify port)
   hurl get http://localhost:3000/api
   ```

3. **Check firewall:**
   ```bash
   # macOS
   sudo pfctl -f /etc/pf.conf
   
   # Linux
   sudo ufw allow 8080
   ```

4. **Verify service binding:**
   ```bash
   # Server should listen on 0.0.0.0 or specific interface
   ps aux | grep your-service
   ```

5. **Test with curl first:**
   ```bash
   curl -v http://localhost:8080/api
   ```

### Issue: No Route to Host

**Error Message:**
```
error: No route to host
```

**Causes:**
- Network unreachable
- Wrong IP address
- Routing issues
- DNS misconfiguration

**Solutions:**

1. **Check network connectivity:**
   ```bash
   ping 8.8.8.8
   traceroute api.example.com
   ```

2. **Verify IP address:**
   ```bash
   # Get correct IP
   nslookup api.example.com
   dig api.example.com
   ```

3. **Check routing:**
   ```bash
   route -n
   ip route show
   ```

4. **Test with specific IP:**
   ```bash
   hurl get http://192.168.1.100:8080/api
   ```

### Issue: Connection Reset by Peer

**Error Message:**
```
error: Connection reset by peer (os error 104)
```

**Causes:**
- Server killed connection
- Network instability
- Firewall closing connection
- Server resource exhaustion

**Solutions:**

1. **Implement retry logic:**
   ```bash
   for attempt in 1 2 3; do
     hurl get https://api.example.com/data && break
     sleep 2
   done
   ```

2. **Check server logs:**
   ```bash
   # Check if server is running
   systemctl status your-service
   journalctl -u your-service -n 50
   ```

3. **Reduce request frequency:**
   ```bash
   hurl get https://api.example.com/data --timeout 10
   ```

4. **Check network stability:**
   ```bash
   ping -c 100 api.example.com
   ```

## Timeout Problems

### Issue: Request Timeout

**Error Message:**
```
error: Timeout waiting for response
```

**Causes:**
- Server too slow
- Network latency
- Default timeout too low (30s)
- Server hung

**Solutions:**

1. **Increase timeout:**
   ```bash
   # Default is 30 seconds
   hurl get https://api.example.com/slow --timeout 60
   
   # For very slow operations
   hurl post https://api.example.com/process --timeout 300
   ```

2. **Test server response time:**
   ```bash
   time hurl get https://api.example.com/data
   ```

3. **Check server performance:**
   ```bash
   # Monitor server resources
   htop
   vmstat 1
   ```

4. **Identify slow endpoint:**
   ```bash
   hurl --verbose get https://api.example.com/slow
   ```

5. **Check network latency:**
   ```bash
   ping -c 5 api.example.com
   mtr api.example.com
   ```

### Issue: Connection Timeout

**Error Message:**
```
error: Timeout connecting to server
```

**Causes:**
- Server not responding
- Network unreachable
- Firewall blocking
- DNS slow

**Solutions:**

1. **Check server is online:**
   ```bash
   hurl head https://api.example.com/
   ```

2. **Verify DNS resolution:**
   ```bash
   nslookup api.example.com
   host api.example.com
   ```

3. **Test with IP instead of hostname:**
   ```bash
   IP=$(dig +short api.example.com | head -1)
   hurl get "http://$IP/api"
   ```

4. **Increase timeout for slow networks:**
   ```bash
   hurl get https://api.example.com/data --timeout 60
   ```

## SSL/TLS Issues

### Issue: SSL Certificate Verification Failed

**Error Message:**
```
error: SSL certificate problem: certificate verify failed
```

**Causes:**
- Self-signed certificate
- Expired certificate
- Wrong hostname in certificate
- CA certificate not installed
- Intermediate certificates missing

**Solutions:**

1. **For development/testing (not production):**
   ```bash
   hurl --no-verify-ssl get https://self-signed.example.com/api
   ```

2. **Verify certificate details:**
   ```bash
   openssl s_client -connect api.example.com:443 -showcerts
   ```

3. **Check certificate expiration:**
   ```bash
   echo | openssl s_client -servername api.example.com -connect api.example.com:443 2>/dev/null | \
   openssl x509 -noout -dates
   ```

4. **Verify hostname in certificate:**
   ```bash
   echo | openssl s_client -servername api.example.com -connect api.example.com:443 2>/dev/null | \
   openssl x509 -noout -text | grep "Subject:"
   ```

5. **Install CA certificate (Linux):**
   ```bash
   # Add CA certificate
   sudo cp ca.pem /usr/local/share/ca-certificates/
   sudo update-ca-certificates
   
   # Retry request
   hurl get https://self-signed.example.com/api
   ```

6. **Use system CA bundle:**
   ```bash
   export SSL_CERT_FILE=/etc/ssl/certs/ca-bundle.crt
   hurl get https://api.example.com/data
   ```

### Issue: Certificate Chain Incomplete

**Error Message:**
```
error: SSL certificate chain incomplete
```

**Solutions:**

1. **Check certificate chain:**
   ```bash
   openssl s_client -connect api.example.com:443 -showcerts
   ```

2. **Request through intermediate:**
   ```bash
   curl --cacert ca-chain.pem https://api.example.com/api
   ```

## Authentication Failures

### Issue: 401 Unauthorized

**Error Message:**
```
HTTP/1.1 401 Unauthorized
```

**Causes:**
- Missing credentials
- Wrong credentials
- Expired token
- Invalid header format
- Auth server down

**Solutions:**

1. **Verify credentials:**
   ```bash
   hurl get https://api.example.com/secure -u username:password
   ```

2. **Check token validity:**
   ```bash
   TOKEN="your-token-here"
   hurl get https://api.example.com/data \
     -H "Authorization: Bearer $TOKEN"
   ```

3. **Decode JWT token (if applicable):**
   ```bash
   TOKEN="eyJhbGciOi..."
   echo $TOKEN | cut -d. -f2 | base64 -d | jq .
   ```

4. **Test auth endpoint separately:**
   ```bash
   hurl post https://api.example.com/auth/login \
     -d '{"username":"user","password":"pass"}'
   ```

5. **Check authentication header format:**
   ```bash
   # Correct formats
   hurl get https://api.example.com/data \
     -H "Authorization: Bearer token123"
   
   hurl get https://api.example.com/data \
     -H "Authorization: Basic dXNlcm5hbWU6cGFzc3dvcmQ="
   
   hurl get https://api.example.com/data \
     -H "X-API-Key: key123"
   ```

### Issue: 403 Forbidden

**Error Message:**
```
HTTP/1.1 403 Forbidden
```

**Causes:**
- Insufficient permissions
- IP blocked
- Rate limit exceeded
- Resource access denied

**Solutions:**

1. **Check permissions:**
   ```bash
   # Try with different user/role
   hurl get https://api.example.com/admin -u admin:password
   ```

2. **Check IP restrictions:**
   ```bash
   # Get your IP
   curl ifconfig.me
   
   # Verify against whitelist
   grep $(curl ifconfig.me) /etc/hosts.deny
   ```

3. **Check rate limits:**
   ```bash
   # Check rate limit headers
   hurl --verbose get https://api.example.com/data | grep -i "rate\|limit"
   ```

4. **Wait and retry:**
   ```bash
   hurl get https://api.example.com/data
   sleep 60
   hurl get https://api.example.com/data
   ```

## Proxy Issues

### Issue: Cannot Connect Through Proxy

**Error Message:**
```
error: Proxy connection failed
```

**Causes:**
- Wrong proxy URL
- Proxy requires authentication
- Proxy connection refused
- Proxy timeout

**Solutions:**

1. **Verify proxy is running:**
   ```bash
   # Test proxy connectivity
   curl -I -x http://proxy.example.com:8080 http://example.com
   ```

2. **Check proxy format:**
   ```bash
   # Correct format
   hurl get https://api.example.com/data \
     --proxy http://proxy.example.com:8080
   
   # With authentication
   hurl get https://api.example.com/data \
     --proxy http://user:pass@proxy.example.com:8080
   ```

3. **Use environment variables:**
   ```bash
   export HTTP_PROXY=http://proxy.example.com:8080
   export HTTPS_PROXY=https://proxy.example.com:8443
   hurl get https://api.example.com/data
   ```

4. **Bypass proxy for local addresses:**
   ```bash
   export NO_PROXY=localhost,127.0.0.1,*.local
   hurl get http://localhost:8000/api
   ```

## Memory Issues

### Issue: Out of Memory

**Error Message:**
```
error: Cannot allocate memory
```

**Causes:**
- Large response body
- Multiple large concurrent requests
- Memory leak
- System resource exhaustion

**Solutions:**

1. **Stream large responses:**
   ```bash
   # Save to file instead of memory
   hurl get https://api.example.com/large-file \
     -o large_file.json
   ```

2. **Reduce concurrent requests:**
   ```bash
   # Sequential instead of parallel
   for id in {1..1000}; do
     hurl get https://api.example.com/users/$id
   done
   ```

3. **Process in batches:**
   ```bash
   for page in {1..100}; do
     hurl get "https://api.example.com/users?page=$page&limit=100" \
       -o page_$page.json
   done
   ```

4. **Monitor memory usage:**
   ```bash
   # Check system memory
   free -h
   
   # Monitor process memory
   ps aux | grep hurl
   ```

## Performance Problems

### Issue: Slow Request Execution

**Symptoms:**
- Requests take too long
- Consistent delay
- Works with curl but slow with hurl

**Solutions:**

1. **Profile the request:**
   ```bash
   time hurl get https://api.example.com/data
   ```

2. **Check network latency:**
   ```bash
   ping -c 10 api.example.com
   ```

3. **Check DNS resolution time:**
   ```bash
   time nslookup api.example.com
   ```

4. **Reduce request overhead:**
   ```bash
   hurl --quiet get https://api.example.com/data
   ```

5. **Use connection pooling:**
   ```bash
   # Multiple requests reuse connection
   for i in {1..10}; do
     hurl get https://api.example.com/users/$i
   done
   ```

### Issue: High CPU Usage

**Symptoms:**
- HURL consumes excessive CPU
- System becomes unresponsive
- Fan loud/hot

**Solutions:**

1. **Check for infinite loops:**
   ```bash
   # Add timeout to prevent hanging
   hurl get https://api.example.com/data --timeout 10
   ```

2. **Limit concurrent requests:**
   ```bash
   MAX_CONCURRENT=5
   for i in {1..100}; do
     ( hurl get https://api.example.com/users/$i ) &
     
     # Wait if too many running
     while [ $(jobs -r -p | wc -l) -ge $MAX_CONCURRENT ]; do
       sleep 0.1
     done
   done
   wait
   ```

3. **Disable verbose output:**
   ```bash
   hurl --quiet get https://api.example.com/data
   ```

## Assertion Failures

### Issue: Assertion Failed

**Error Message:**
```
error: Assertion failed: status code 200 != 404
```

**Causes:**
- Wrong expected value
- API behavior changed
- Network issue
- Wrong endpoint

**Solutions:**

1. **Check actual response:**
   ```bash
   hurl get https://api.example.com/users
   ```

2. **Verify expected assertion:**
   ```bash
   # Get status code
   hurl --headers-only get https://api.example.com/users
   ```

3. **Debug with verbose output:**
   ```bash
   hurl --verbose get https://api.example.com/users
   ```

4. **Test with simpler assertion:**
   ```bash
   hurl get https://api.example.com/users \
     --assert-body-contains "error"
   ```

### Issue: Header Assertion Fails

**Error Message:**
```
error: Assertion failed: header not found
```

**Solutions:**

1. **Check header names (case-sensitive):**
   ```bash
   hurl --headers-only get https://api.example.com/users
   ```

2. **Extract specific header:**
   ```bash
   hurl get https://api.example.com/users | grep -i "content-type"
   ```

## Parsing Errors

### Issue: JSON Parse Error

**Error Message:**
```
error: Failed to parse JSON
```

**Causes:**
- Invalid JSON response
- Not JSON content (HTML error page)
- Character encoding issue
- Malformed JSON

**Solutions:**

1. **Check response format:**
   ```bash
   hurl --headers-only get https://api.example.com/data \
     | grep "Content-Type"
   ```

2. **View raw response:**
   ```bash
   hurl get https://api.example.com/data \
     --output-format raw
   ```

3. **Validate JSON:**
   ```bash
   hurl get https://api.example.com/data | jq .
   ```

4. **Check character encoding:**
   ```bash
   file <(hurl get https://api.example.com/data)
   ```

### Issue: XML Parse Error

**Error Message:**
```
error: Failed to parse XML
```

**Solutions:**

1. **Validate XML:**
   ```bash
   hurl get https://api.example.com/data \
     --output-format xml | xmllint -
   ```

2. **Check for HTML error page:**
   ```bash
   hurl get https://api.example.com/data \
     --output-format raw | head -20
   ```

## File Operations

### Issue: Cannot Write to Output File

**Error Message:**
```
error: Permission denied
```

**Causes:**
- Directory read-only
- File already open
- No write permission
- Disk full

**Solutions:**

1. **Check file permissions:**
   ```bash
   ls -la output.json
   chmod 644 output.json
   ```

2. **Check directory permissions:**
   ```bash
   ls -ld .
   chmod 755 .
   ```

3. **Try different location:**
   ```bash
   hurl get https://api.example.com/data -o ~/output.json
   ```

4. **Check disk space:**
   ```bash
   df -h
   du -sh .
   ```

### Issue: Output File Already Exists

**Error Message:**
```
error: File already exists
```

**Solutions:**

1. **Overwrite file:**
   ```bash
   hurl get https://api.example.com/data -o output.json --force
   ```

2. **Append to file:**
   ```bash
   hurl get https://api.example.com/data >> output.json
   ```

3. **Create new file:**
   ```bash
   hurl get https://api.example.com/data -o output_$(date +%s).json
   ```

## Network Issues

### Issue: DNS Resolution Fails

**Error Message:**
```
error: Failed to resolve domain
```

**Causes:**
- DNS server down
- Wrong domain name
- Network connectivity issue
- DNS cache stale

**Solutions:**

1. **Test DNS resolution:**
   ```bash
   nslookup api.example.com
   dig api.example.com
   host api.example.com
   ```

2. **Try different DNS server:**
   ```bash
   # Google DNS
   nslookup api.example.com 8.8.8.8
   ```

3. **Clear DNS cache:**
   ```bash
   # macOS
   sudo dscacheutil -flushcache
   
   # Linux
   sudo systemd-resolve --flush-caches
   ```

4. **Use IP address directly:**
   ```bash
   IP=$(dig +short api.example.com | head -1)
   hurl get "http://$IP/api"
   ```

### Issue: Network Unreachable

**Error Message:**
```
error: Network is unreachable
```

**Causes:**
- Internet disconnected
- Wrong network interface
- Routing issue
- VPN disconnected

**Solutions:**

1. **Check network connectivity:**
   ```bash
   ping 8.8.8.8
   ping 1.1.1.1
   ```

2. **Check active interface:**
   ```bash
   ip route show
   netstat -rn
   ```

3. **Test with traceroute:**
   ```bash
   traceroute api.example.com
   ```

4. **Check VPN status (if applicable):**
   ```bash
   # Reconnect if needed
   sudo openvpn --config config.ovpn
   ```

## Debug Logging

### Enable Verbose Output

```bash
hurl --verbose get https://api.example.com/data
```

Shows:
- Request method, URL, headers
- Response status, headers
- Connection timing
- Redirect chains

### Enable Debug Logging

```bash
RUST_LOG=debug hurl get https://api.example.com/data
```

### Log to File

```bash
hurl get https://api.example.com/data 2>&1 | tee request.log
```

### Detailed Network Debugging

```bash
# Use tcpdump to capture traffic
sudo tcpdump -i any -A "tcp port 443" | grep --line-buffered -oP '(Host|Authorization):.*'

# Use strace to trace system calls
strace -e openat,connect,write hurl get https://api.example.com/data
```

## FAQ

### Q: How do I see what HURL is sending?

A: Use `--verbose` flag:
```bash
hurl --verbose post https://api.example.com/users \
  -d '{"name":"test"}'
```

### Q: How do I save both request and response?

A: Combine with output redirection:
```bash
hurl --verbose get https://api.example.com/data 2>&1 | tee complete_log.txt
```

### Q: Can I retry failed requests automatically?

A: Wrap in shell script:
```bash
for attempt in 1 2 3; do
  hurl get https://api.example.com/data && break
  sleep $((attempt * 2))
done
```

### Q: How do I handle rate limiting?

A: Add delays between requests:
```bash
for id in {1..100}; do
  hurl get https://api.example.com/users/$id
  sleep 1  # 1 second between requests
done
```

### Q: Can I use HURL with authentication that requires exchange?

A: Yes, extract token and use:
```bash
TOKEN=$(hurl post https://api.example.com/auth \
  -d '{"user":"x","pass":"y"}' | jq -r '.token')

hurl get https://api.example.com/data \
  -H "Authorization: Bearer $TOKEN"
```

### Q: How do I handle requests with custom headers and body?

A: Use multiple `-H` and `-d`:
```bash
hurl post https://api.example.com/data \
  -H "Authorization: Bearer token" \
  -H "X-Custom: value" \
  -d '{"key":"value"}'
```

### Q: Can I use HURL in Docker?

A: Yes, build or use official image:
```bash
docker run -it ghcr.io/hurl/hurl:latest \
  get https://example.com
```

### Q: How do I export large responses efficiently?

A: Use `-o` to save to file:
```bash
hurl get https://api.example.com/large-dataset \
  -o dataset.json
```

### Q: What exit codes does HURL use?

A: 
- 0: Success
- 1: General error
- 2: Connection error
- 3: Timeout error
- 4: Invalid arguments
- 5: Authentication failed
- 6: Assertion failed

### Q: Can I chain requests together?

A: Yes, extract data and pass to next request:
```bash
ID=$(hurl post https://api.example.com/users \
  -d '{"name":"test"}' | jq -r '.id')

hurl get https://api.example.com/users/$ID
```
