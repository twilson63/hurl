# Development Guidelines

## Setup

### Prerequisites

- Rust 1.56 or later
- Cargo (installed with Rust)

### Installation

```bash
git clone https://github.com/hurl/hurl
cd hurl
```

## Development Workflow

### Building the Project

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

### Running Tests

```bash
# Run all tests
make test

# Run specific test
cargo test --lib test_name

# Run with output
cargo test -- --nocapture
```

### Code Quality

#### Formatting

```bash
# Format code
make fmt

# Check formatting
make fmt-check
```

#### Linting

```bash
# Run clippy
make lint

# Run with all lints
cargo clippy --all --all-targets -- -D warnings
```

#### Type Checking

```bash
# Check types without compiling
cargo check --all
```

## Code Style

### Naming Conventions

- **Variables/Functions**: `snake_case`
- **Types/Structs**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`

### Error Handling

Use `Result<T>` with custom error types from `error.rs`:

```rust
pub fn operation() -> crate::Result<()> {
    // Implementation
    Ok(())
}
```

### Documentation

All public items should have doc comments:

```rust
/// Brief description
///
/// Longer explanation if needed
pub fn my_function() -> Result<()> {
    // Implementation
}
```

## Git Workflow

### Branch Naming

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation updates
- `refactor/description` - Code refactoring

### Commit Messages

```
type(scope): short description

Longer explanation if needed. Keep to 72 characters per line.
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

## Testing

### Unit Tests

Write tests in the same file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Test code
    }
}
```

### Integration Tests

Create tests in `tests/` directory at crate root.

### Running Coverage

```bash
cargo tarpaulin --verbose --all --out Html
```

## Debugging

### Debug Output

Use `eprintln!` for debug output:

```rust
eprintln!("[DEBUG] Some value: {:?}", value);
```

### Logging

The project uses `tracing` for structured logging:

```rust
use tracing::{debug, info, warn, error};

debug!("Debug message");
info!("Info message");
```

## Performance

### Profiling

```bash
cargo flamegraph --bin hurl -- <args>
```

### Benchmarking

Add benchmarks in `benches/` directory:

```rust
#[bench]
fn bench_operation(b: &mut Bencher) {
    b.iter(|| {
        // Operation to benchmark
    });
}
```

## Documentation

### Building Docs

```bash
make doc
```

### Doc Tests

```bash
cargo test --doc
```

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
4. Push tag: `git push origin vX.Y.Z`

## Common Issues

### Dependency Conflicts

```bash
cargo update
cargo clean
cargo build
```

### Tests Failing

```bash
# Run tests with backtrace
RUST_BACKTRACE=1 cargo test

# Run single test
cargo test test_name -- --exact
```

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Reqwest Documentation](https://docs.rs/reqwest/)
- [Tokio Documentation](https://tokio.rs/)
