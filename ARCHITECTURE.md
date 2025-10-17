# HURL Architecture

## Overview

HURL is composed of two main crates:

### hurl-lib

The core library providing HTTP functionality, formatting, storage, and testing utilities.

#### Module Structure

- **http** - HTTP client implementation
  - `client.rs` - HTTP client wrapper around reqwest
  - `request.rs` - Request building utilities
  - `response.rs` - Response handling and metadata

- **cli** - CLI parsing and command handling
  - `parser.rs` - Command-line argument parsing
  - `commands.rs` - Command execution logic

- **format** - Response formatting
  - `json.rs` - JSON output formatting
  - `xml.rs` - XML output formatting
  - `table.rs` - Table format rendering
  - `csv.rs` - CSV output formatting
  - `colors.rs` - Color configuration and utilities

- **storage** - Data persistence
  - `history.rs` - Request/response history management

- **test** - Testing utilities
  - `assertions.rs` - Response assertion definitions and evaluation

- **error.rs** - Error types and handling
- **config.rs** - Configuration management
- **utils.rs** - Utility functions

### hurl-cli

The command-line application that provides the user interface.

#### Main Entry Point

- `main.rs` - CLI application with clap integration

#### Submodules

- `cli/config.rs` - CLI configuration
- `cli/commands.rs` - Command implementations
- `cli/parser.rs` - Request configuration parsing

## Dependency Flow

```
hurl-cli
  ├── hurl-lib
  ├── reqwest
  ├── tokio
  ├── clap
  └── ...

hurl-lib
  ├── reqwest
  ├── serde/serde_json
  ├── tokio
  └── ...
```

## Key Design Decisions

1. **Async/Await** - Uses tokio for non-blocking I/O
2. **Error Handling** - Uses thiserror for custom error types
3. **Serialization** - Serde for data serialization/deserialization
4. **CLI Framework** - Clap for command-line parsing
5. **HTTP Client** - Reqwest for HTTP operations

## Future Enhancements

- [ ] HTTP/2 multiplexing
- [ ] Request/response interceptors
- [ ] Environment variable interpolation
- [ ] Script execution
- [ ] Request templating
- [ ] WebSocket support
- [ ] GraphQL support
- [ ] OpenAPI integration
