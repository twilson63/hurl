# HURL Architecture Guide

Comprehensive guide to HURL's internal architecture, design patterns, and implementation details.

## Table of Contents

- [System Overview](#system-overview)
- [Module Architecture](#module-architecture)
- [Data Flow](#data-flow)
- [Request/Response Flow](#requestresponse-flow)
- [Authentication Architecture](#authentication-architecture)
- [Error Handling Strategy](#error-handling-strategy)
- [Testing Architecture](#testing-architecture)
- [Storage Layer](#storage-layer)
- [Async/Await Patterns](#asyncawait-patterns)
- [Thread Safety](#thread-safety)
- [Extension Points](#extension-points)

## System Overview

HURL is built on two primary crates:

```
┌─────────────────────────────────────────────────────┐
│                   hurl-cli (Binary)                 │
│  CLI Interface, Argument Parsing, User I/O          │
└────────────────────┬────────────────────────────────┘
                     │
                     │ Uses
                     ▼
┌─────────────────────────────────────────────────────┐
│              hurl-lib (Library Core)                │
│  HTTP Client, Formatting, Testing, Storage          │
└─────────────────────────────────────────────────────┘
                     │
                     │ Uses
                     ▼
┌─────────────────────────────────────────────────────┐
│            External Dependencies                    │
│  reqwest, tokio, serde, colored, clap               │
└─────────────────────────────────────────────────────┘
```

### Design Principles

1. **Separation of Concerns**: CLI and library are distinct, library can be used independently
2. **Async-First**: Built on tokio for non-blocking I/O
3. **Type Safety**: Leverage Rust's type system for compile-time safety
4. **Error Handling**: Explicit error types with thiserror
5. **Composability**: Modular design allows feature composition

## Module Architecture

### hurl-lib Structure

```
hurl-lib/
├── lib.rs                 # Public API exports
├── error.rs              # Error type definitions
├── config.rs             # Configuration management
├── utils.rs              # Utility functions
├── http/                 # HTTP client implementation
│   ├── mod.rs           # Module exports
│   ├── client.rs        # HTTP client wrapper
│   ├── request.rs       # Request building
│   ├── response.rs      # Response handling
│   ├── auth.rs          # Authentication
│   ├── security.rs      # TLS/SSL handling
│   ├── cookies.rs       # Cookie management
│   ├── compression.rs   # Compression support
│   ├── chaining.rs      # Request chaining
│   └── tests.rs         # HTTP tests
├── test/                 # Testing utilities
│   ├── mod.rs           # Module exports
│   ├── assertions.rs    # Response assertions
│   ├── hml_parser.rs    # Test file parsing
│   └── runner.rs        # Test execution
├── storage/              # Data persistence
│   ├── mod.rs           # Module exports
│   ├── cache.rs         # Response caching
│   └── history.rs       # Request history
└── batch.rs              # Batch processing

hurl-cli/
├── main.rs               # Entry point
└── cli/
    ├── mod.rs           # Module exports
    ├── config.rs        # CLI configuration
    ├── parser.rs        # Argument parsing
    └── commands.rs      # Command implementations
```

### Module Responsibilities

#### http/client.rs
- Wraps reqwest HTTP client
- Manages connection pooling
- Handles timeouts and retries
- Returns typed responses

#### http/auth.rs
- Implements authentication schemes
- Basic, Bearer, Digest, OAuth2
- Header generation
- Credential management

#### http/request.rs
- Request building utilities
- Header manipulation
- Body encoding
- Query parameter handling

#### http/response.rs
- Response parsing
- Metadata extraction
- Body deserialization
- Status code handling

#### test/assertions.rs
- Assertion definition
- Status code checks
- Header validation
- Body content matching

#### storage/history.rs
- Request/response logging
- History retrieval
- Persistence (if enabled)

#### batch.rs
- Multiple request execution
- Sequential processing
- Error aggregation

## Data Flow

### High-Level Request Flow

```
User Command
    │
    ▼
CLI Parser (clap)
    │
    ├─ Parse arguments
    ├─ Load configuration
    └─ Extract options
    │
    ▼
CLI Command Handler
    │
    ├─ Validate input
    ├─ Prepare auth
    └─ Build headers
    │
    ▼
HTTP Client
    │
    ├─ Create request
    ├─ Apply auth
    ├─ Set headers
    └─ Add body
    │
    ▼
Network I/O (reqwest + tokio)
    │
    ├─ DNS lookup
    ├─ TLS handshake
    └─ HTTP transmission
    │
    ▼
Response Handling
    │
    ├─ Deserialize body
    ├─ Parse headers
    └─ Extract metadata
    │
    ▼
Format & Output
    │
    ├─ Apply format (JSON/XML/CSV/Table)
    ├─ Add colors
    └─ Write to stdout/file
    │
    ▼
User Output
```

### Async Processing Model

```
tokio Runtime
    │
    ├─ Main task (CLI handler)
    │   └─ Spawns HTTP task
    │
    ├─ HTTP task (reqwest)
    │   ├─ DNS resolver task
    │   ├─ TLS task
    │   └─ HTTP transmission task
    │
    └─ Timer tasks (timeouts)
```

## Request/Response Flow

### Detailed Request Processing

```rust
pub async fn send_request(config: RequestConfig) -> Result<Response> {
    // 1. Validation
    validate_url(&config.url)?;
    
    // 2. Client creation
    let client = create_client(&config)?;
    
    // 3. Request building
    let mut request = client.request(config.method, &config.url);
    
    // 4. Headers
    for (key, value) in &config.headers {
        request = request.header(key, value);
    }
    
    // 5. Authentication
    if let Some(auth) = &config.auth {
        request = apply_auth(request, auth)?;
    }
    
    // 6. Body
    if let Some(body) = &config.body {
        request = request.body(body.clone());
    }
    
    // 7. Timeout
    request = request.timeout(config.timeout);
    
    // 8. Send
    let response = request.send().await?;
    
    // 9. Parse response
    let parsed = parse_response(response).await?;
    
    // 10. Store history
    store_history(&config, &parsed)?;
    
    Ok(parsed)
}
```

### Response Processing Pipeline

```
Raw HTTP Response
    │
    ▼
Status Code Extraction
    │
    ▼
Header Parsing
    │
    ▼
Body Reading
    │
    ├─ Detect encoding (gzip, brotli, deflate)
    └─ Decompress if needed
    │
    ▼
Content-Type Detection
    │
    ├─ application/json → parse_json()
    ├─ application/xml → parse_xml()
    ├─ text/* → parse_text()
    └─ other → store_raw()
    │
    ▼
Response Object
    │
    ├─ status: u16
    ├─ headers: HashMap
    ├─ body: String/JSON/XML
    └─ metadata: ResponseMetadata
    │
    ▼
Formatting Layer
    │
    ├─ JSON Formatter
    ├─ XML Formatter
    ├─ CSV Formatter
    ├─ Table Formatter
    └─ Color Applier
    │
    ▼
Output
```

## Authentication Architecture

### Authentication Flow

```
Request with Auth
    │
    ▼
Auth Type Detection
    │
    ├─ Basic (user:password)
    │   └─ Base64 encode → "Basic dXNlcjpwYXNz"
    │
    ├─ Bearer (token)
    │   └─ Add header → "Authorization: Bearer token"
    │
    ├─ Digest
    │   ├─ Extract challenge
    │   ├─ Hash credentials
    │   └─ Generate response
    │
    ├─ OAuth2
    │   ├─ Get token from provider
    │   └─ Use as Bearer token
    │
    └─ Custom
        └─ Apply custom header
    │
    ▼
Apply to Request
    │
    ├─ Add header
    ├─ Modify request
    └─ Set credentials
    │
    ▼
Send Request
```

### Authentication Type Hierarchy

```
enum AuthMethod {
    Basic { username: String, password: String },
    Bearer { token: String },
    Digest { realm: String, response: String },
    OAuth2 { token: String, expires_at: DateTime },
    Custom { scheme: String, credentials: String },
}
```

## Error Handling Strategy

### Error Type Hierarchy

```rust
pub enum Error {
    // Network errors
    ConnectionError(String),
    DnsError(String),
    Timeout,
    TooManyRedirects,
    
    // Protocol errors
    InvalidUrl,
    InvalidHeader,
    InvalidBody,
    
    // Authentication errors
    AuthenticationFailed,
    InvalidCredentials,
    
    // HTTP errors
    HttpError(u16, String),  // Status code + reason
    
    // Parsing errors
    JsonParseError(String),
    XmlParseError(String),
    
    // File errors
    FileError(String),
    
    // Assertion errors
    AssertionFailed(String),
    
    // Configuration errors
    ConfigError(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Error::Timeout
        } else if err.is_connect() {
            Error::ConnectionError(err.to_string())
        } else {
            Error::HttpError(0, err.to_string())
        }
    }
}
```

### Error Propagation

```
Application Layer
    │
    ├─ Handles: User messages, exit codes
    │
    ▼
Command Layer
    │
    ├─ Handles: Argument validation
    │
    ▼
HTTP Layer
    │
    ├─ Handles: Network, parsing
    ├─ Propagates with ?
    │
    ▼
reqwest/tokio
    │
    └─ Network errors converted to Error enum
```

## Testing Architecture

### Test Execution Flow

```
Test File (.hml)
    │
    ▼
Parser
    │
    ├─ Parse HML syntax
    ├─ Extract requests
    └─ Extract assertions
    │
    ▼
Test Runner
    │
    ├─ For each test:
    │   ├─ Prepare request
    │   ├─ Send via HTTP
    │   ├─ Capture response
    │   ├─ Run assertions
    │   └─ Report result
    │
    ▼
Assertion Evaluator
    │
    ├─ Status code check
    ├─ Header validation
    ├─ Body content match
    ├─ Regex match
    └─ JSON path validation
    │
    ▼
Test Report
    │
    ├─ Passed: X
    ├─ Failed: Y
    └─ Execution time: Z
```

### Assertion Types

```rust
enum Assertion {
    StatusCode(u16),
    StatusRange(u16, u16),
    Header { name: String, value: String },
    BodyContains(String),
    BodyMatches(regex::Regex),
    JsonPath { path: String, value: serde_json::Value },
    ResponseTime { max_ms: u64 },
}
```

## Storage Layer

### History Storage

```
Request
    │
    ├─ URL
    ├─ Method
    ├─ Headers
    ├─ Body
    └─ Timestamp
    │
    ▼
Store in History
    │
    ├─ In-memory Vec<HistoryEntry>
    │   └─ Latest N entries
    │
    └─ Optional: Disk persistence
        └─ ~/.hurl/history.json
            {
              "timestamp": "2024-01-01T12:00:00Z",
              "method": "GET",
              "url": "https://api.example.com/users",
              "status": 200,
              "duration_ms": 150
            }

Response
    │
    ├─ Status
    ├─ Headers
    ├─ Body
    └─ Metadata
    │
    ▼
Correlate with Request
```

### Cache Strategy (if enabled)

```
Request
    │
    ▼
Generate Cache Key
    │
    └─ hash(method + url + headers)
    │
    ▼
Check Cache
    │
    ├─ Hit: Check TTL
    │   ├─ Valid: Return cached response
    │   └─ Expired: Clear, make request
    │
    └─ Miss: Make request
    │
    ▼
Store Response
    │
    ├─ Check Cache-Control header
    ├─ Respect Expires header
    └─ Store with TTL
```

## Async/Await Patterns

### Async Request Handling

```rust
// CLI command handler
pub async fn handle_get(args: GetArgs) -> Result<()> {
    // Run HTTP operation asynchronously
    let response = http::get(&args.url, &args.headers).await?;
    
    // Format asynchronously (large responses)
    let formatted = format_response(response).await?;
    
    // Write output
    println!("{}", formatted);
    
    Ok(())
}

// In main.rs
#[tokio::main]
async fn main() -> Result<()> {
    // Tokio runtime is automatically created
    run_cli().await
}
```

### Concurrent Request Patterns

```rust
// Process multiple requests concurrently
pub async fn batch_requests(urls: Vec<String>) -> Result<Vec<Response>> {
    let futures: Vec<_> = urls
        .into_iter()
        .map(|url| http::get(&url, &HashMap::new()))
        .collect();
    
    let responses = futures::future::try_join_all(futures).await?;
    Ok(responses)
}

// Process with controlled concurrency
pub async fn batch_requests_limited(urls: Vec<String>, limit: usize) -> Result<Vec<Response>> {
    let semaphore = Arc::new(tokio::sync::Semaphore::new(limit));
    let futures = urls.into_iter().map(|url| {
        let sem = Arc::clone(&semaphore);
        async move {
            let _permit = sem.acquire().await;
            http::get(&url, &HashMap::new()).await
        }
    });
    
    futures::future::try_join_all(futures).await
}
```

### Timeout Handling

```rust
pub async fn get_with_timeout(
    url: &str,
    timeout_secs: u64,
) -> Result<Response> {
    tokio::time::timeout(
        Duration::from_secs(timeout_secs),
        http::get(url, &HashMap::new()),
    )
    .await
    .map_err(|_| Error::Timeout)?
}
```

## Thread Safety

### Shared State Management

```rust
// CLI state (not shared, single-threaded)
pub struct CliState {
    config: Config,
    history: Vec<HistoryEntry>,
}

// HTTP client (thread-safe, shareable)
pub struct HttpClient {
    client: reqwest::Client,  // Internally thread-safe
}

// History (thread-safe if shared)
pub struct SharedHistory {
    entries: Arc<Mutex<Vec<HistoryEntry>>>,
}

impl SharedHistory {
    pub async fn add(&self, entry: HistoryEntry) -> Result<()> {
        let mut entries = self.entries.lock().await;
        entries.push(entry);
        Ok(())
    }
}
```

### Lock-Free Patterns

```rust
// Use Arc for shared ownership
let client = Arc::new(HttpClient::new());

// Spawn concurrent tasks
for _ in 0..10 {
    let client_clone = Arc::clone(&client);
    tokio::spawn(async move {
        let _ = client_clone.get("https://example.com").await;
    });
}
```

## Extension Points

### Custom Authentication Scheme

```rust
pub trait AuthProvider: Send + Sync {
    fn apply(&self, request: &mut Request) -> Result<()>;
    fn refresh(&mut self) -> Result<()>;
}

pub struct OAuthProvider {
    token: String,
    refresh_token: String,
}

impl AuthProvider for OAuthProvider {
    fn apply(&self, request: &mut Request) -> Result<()> {
        request.header("Authorization", format!("Bearer {}", self.token));
        Ok(())
    }
    
    fn refresh(&mut self) -> Result<()> {
        // Fetch new token
        Ok(())
    }
}
```

### Custom Output Format

```rust
pub trait OutputFormatter: Send + Sync {
    fn format(&self, response: &Response) -> Result<String>;
}

pub struct YamlFormatter;

impl OutputFormatter for YamlFormatter {
    fn format(&self, response: &Response) -> Result<String> {
        // Custom YAML formatting
        Ok(serde_yaml::to_string(&response)?)
    }
}
```

### Custom Assertion Types

```rust
pub trait AssertionEvaluator: Send + Sync {
    fn evaluate(&self, response: &Response) -> Result<bool>;
}

pub struct CustomAssertion {
    expression: String,
}

impl AssertionEvaluator for CustomAssertion {
    fn evaluate(&self, response: &Response) -> Result<bool> {
        // Evaluate custom expression
        evaluate_expression(&self.expression, response)
    }
}
```

## Performance Considerations

### Connection Pooling

```rust
let client = reqwest::Client::builder()
    .pool_max_idle_per_host(10)
    .http2_prior_knowledge()
    .build()?;
```

### Memory Efficiency

- Streaming response bodies for large files
- Iterator-based processing
- Lazy evaluation where possible

### CPU Efficiency

- Async I/O prevents thread blocking
- Connection reuse reduces overhead
- Parallel request processing with controlled concurrency

## Dependency Management

### Critical Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| reqwest | 0.11 | HTTP client |
| tokio | 1.0 | Async runtime |
| serde | 1.0 | Serialization |
| clap | 4.0 | CLI parsing |
| colored | 2.0 | Terminal colors |
| thiserror | 1.0 | Error handling |

### Dependency Tree

```
hurl-cli
├── hurl-lib
│   ├── reqwest (with json, cookies, stream)
│   ├── tokio (with full features)
│   ├── serde + serde_json
│   ├── colored
│   ├── regex
│   └── url
├── clap (with derive)
└── anyhow

hurl-lib
└── [same as above]
```

## Future Architecture Improvements

1. **Plugin System**: Dynamic loading of custom authenticators and formatters
2. **GraphQL Support**: Native GraphQL query builder
3. **WebSocket Support**: Upgrade from HTTP to WebSocket
4. **Service Mesh Integration**: Native Istio/Linkerd support
5. **OpenAPI Integration**: Automatic API client generation
6. **Request Tracing**: OpenTelemetry integration
7. **Response Caching**: TTL-based caching strategy
8. **Load Balancing**: Built-in load testing profiles
