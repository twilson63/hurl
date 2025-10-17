# Contributing to HURL

Thank you for your interest in contributing to HURL! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Code Style Guidelines](#code-style-guidelines)
- [Testing Requirements](#testing-requirements)
- [Submission Process](#submission-process)
- [Acknowledgments](#acknowledgments)

---

## Code of Conduct

### Our Commitment

We are committed to providing a welcoming and inspiring community for all. Please read and adhere to our Code of Conduct:

- **Be Respectful**: Treat all community members with respect and dignity
- **Be Inclusive**: Welcome people of all backgrounds and experiences
- **Be Professional**: Keep discussions focused and professional
- **Be Collaborative**: Work together constructively
- **Report Issues**: If you witness or experience violations, please report them to maintainers

### Unacceptable Behavior

- Harassment or discrimination of any kind
- Personal attacks or inflammatory comments
- Trolling or deliberate provocation
- Sharing others' private information
- Any behavior that creates a hostile environment

Violations may result in removal from the project.

---

## Getting Started

### Prerequisites

- Rust 1.70+ (Install from [rustup.rs](https://rustup.rs/))
- Cargo (included with Rust)
- Git
- A GitHub account
- Familiarity with Git workflow

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/hurl.git
   cd hurl
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/hurl/hurl.git
   ```

---

## Development Setup

### Initial Setup

1. **Verify Rust installation**:
   ```bash
   rustc --version  # Should be 1.70+
   cargo --version
   ```

2. **Install development tools**:
   ```bash
   # macOS
   brew install llvm clang-format

   # Linux (Ubuntu/Debian)
   sudo apt-get install build-essential llvm clang-format-14

   # Windows
   # Install Visual Studio Build Tools or equivalent
   ```

3. **Setup pre-commit hooks** (recommended):
   ```bash
   curl -sSL https://pre-commit.com/install | bash
   # or
   pip install pre-commit
   pre-commit install
   ```

### Build Commands

```bash
# Full build
make build              # or: cargo build

# Debug build
cargo build

# Release build with optimizations
cargo build --release

# Build library only
cargo build --lib

# Build CLI only
cargo build -p hurl-cli

# Documentation
cargo doc --no-deps --open
```

### Running Tests

```bash
# Run all tests
make test               # or: cargo test

# Run specific test
cargo test storage::tests

# Run tests with output
cargo test -- --nocapture

# Run with multiple threads
cargo test -- --test-threads=4

# Integration tests
cargo test --test '*'

# With code coverage (requires tarpaulin)
cargo tarpaulin --out Html
```

### Code Formatting and Linting

```bash
# Format code (required before submission)
make fmt               # or: cargo fmt

# Check formatting without modifying
cargo fmt -- --check

# Lint with Clippy
make lint              # or: cargo clippy

# Clippy with all warnings
cargo clippy --all-targets --all-features -- -W clippy::all

# Security audit
cargo audit

# Check documentation
cargo doc --no-deps
```

### Local Development Workflow

```bash
# 1. Create feature branch
git checkout -b feature/my-feature

# 2. Make changes
# ... edit files ...

# 3. Format and lint
make fmt
make lint

# 4. Run tests
make test

# 5. Commit changes
git commit -m "feat: add new feature"

# 6. Push to your fork
git push origin feature/my-feature

# 7. Create Pull Request on GitHub
```

---

## How to Contribute

### Types of Contributions

1. **Bug Reports**
   - Check if issue already exists
   - Include reproduction steps
   - Include environment information
   - Provide error messages/logs

2. **Feature Requests**
   - Describe the use case
   - Explain the expected behavior
   - Provide examples or mockups
   - Consider backwards compatibility

3. **Code Contributions**
   - Bug fixes
   - Feature implementations
   - Performance improvements
   - Documentation improvements
   - Test coverage expansion

4. **Documentation**
   - README improvements
   - API documentation
   - Usage examples
   - Troubleshooting guides
   - Blog posts or tutorials

### Creating an Issue

Before coding, open an issue describing:

- **Clear title**: Summarize the issue in 5-10 words
- **Description**: What is the problem or feature request?
- **Reproducibility**: Steps to reproduce (for bugs)
- **Environment**: OS, Rust version, any relevant context
- **Expected vs Actual**: What should happen vs what happens

### Starting Development

1. Create a feature branch (never commit to main):
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Keep changes focused and atomic
3. Write tests for new functionality
4. Update documentation as needed
5. Keep commits clean and descriptive

---

## Code Style Guidelines

### Rust Conventions

1. **Naming**
   - Functions: `snake_case`
   - Types: `PascalCase`
   - Constants: `SCREAMING_SNAKE_CASE`
   - Private items: prefix with `_` if unused

2. **Comments and Documentation**
   - Use `///` for public API documentation
   - Use `//` for implementation comments
   - Write clear, concise comments
   - Include examples in doc comments

   ```rust
   /// Sends an HTTP request and returns the response.
   ///
   /// # Arguments
   ///
   /// * `method` - HTTP method (GET, POST, etc.)
   /// * `url` - Request URL
   ///
   /// # Returns
   ///
   /// Returns `Ok(response)` on success, `Err(error)` on failure
   ///
   /// # Example
   ///
   /// ```
   /// let response = client.send_request("GET", "https://example.com")?;
   /// ```
   pub fn send_request(method: &str, url: &str) -> Result<Response> {
       // Implementation
   }
   ```

3. **Error Handling**
   - Always use `Result<T>` for fallible operations
   - Don't unwrap in library code
   - Provide context with `.context()` or `?`
   - Use custom error types when needed

   ```rust
   // Good
   fn parse_url(url: &str) -> Result<Url> {
       Url::parse(url).context("Failed to parse URL")
   }

   // Avoid
   fn parse_url(url: &str) -> Url {
       Url::parse(url).unwrap()  // Don't do this!
   }
   ```

4. **Type Safety**
   - Leverage the type system
   - Avoid generic `String` for specific data
   - Use enums for variants
   - Prefer immutability

5. **Formatting**
   - Use `cargo fmt` (automatic)
   - Max line length: 100 characters (guide, not strict)
   - Group related imports
   - One blank line between functions

   ```rust
   // Good
   use crate::http::{Client, Request, Response};
   use std::collections::HashMap;

   use serde::{Deserialize, Serialize};

   fn send_request(client: &Client, request: &Request) -> Result<Response> {
       // Implementation
   }
   ```

6. **Testing**
   - Place tests in the same file with `#[cfg(test)]`
   - Use descriptive test names: `test_<function>_<scenario>`
   - Include comments explaining complex test setups

   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_parse_url_with_valid_url() {
           let url = "https://example.com";
           assert!(parse_url(url).is_ok());
       }

       #[test]
       fn test_parse_url_with_invalid_url() {
           let url = "not a url";
           assert!(parse_url(url).is_err());
       }
   }
   ```

### Project Structure

```
hurl/
├── crates/
│   ├── hurl-lib/              # HTTP client library
│   │   ├── src/
│   │   │   ├── http/          # HTTP protocol handling
│   │   │   ├── storage/       # History and caching
│   │   │   ├── test/          # Test execution
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   └── hurl-cli/              # CLI application
│       ├── src/
│       │   ├── cli/           # CLI logic
│       │   └── main.rs
│       └── Cargo.toml
├── tests/                     # Integration tests
├── docs/                      # Documentation
├── scripts/                   # Build and utility scripts
├── Cargo.toml                 # Workspace manifest
└── Makefile                   # Development tasks
```

---

## Testing Requirements

### Mandatory for All Contributions

1. **Unit Tests**
   - Write tests for all new public functions
   - Aim for >80% code coverage
   - Include happy path and error cases

2. **Integration Tests**
   - Test feature interactions
   - Test end-to-end workflows
   - Include realistic scenarios

3. **Documentation Tests**
   - Update doc comments with examples
   - Ensure examples compile and run

### Running Tests Locally

```bash
# Run all tests
cargo test --all

# Run specific crate tests
cargo test -p hurl-lib
cargo test -p hurl-cli

# Run with verbose output
cargo test -- --nocapture --test-threads=1

# Check test coverage
cargo tarpaulin --out Html --output-dir coverage/

# Run clippy tests
cargo clippy --all-targets

# Security audit
cargo audit
```

### Test Requirements Checklist

- [ ] All tests pass: `cargo test --all`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Formatting correct: `cargo fmt -- --check`
- [ ] Doc tests pass: `cargo test --doc`
- [ ] New functions have doc comments
- [ ] New code has test coverage
- [ ] No unsafe code (unless absolutely necessary)

---

## Submission Process

### Before Submitting

1. **Sync with upstream**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Final checks**:
   ```bash
   cargo fmt
   cargo clippy
   cargo test --all
   cargo doc --no-deps
   ```

3. **Clean commit history**:
   - Rebase/squash commits if needed
   - Write clear commit messages
   - Reference issues with `Fixes #123`

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test additions/changes
- `ci`: CI/CD configuration
- `chore`: Build/dependency changes

**Examples**:

```bash
git commit -m "feat(storage): add SQLite backend for history

Implement persistent storage using SQLite database for history tracking.
Includes migration from in-memory storage and backwards compatibility.

Fixes #42"

git commit -m "fix(http): correct content-length header calculation"

git commit -m "docs: add migration guide from cURL"

git commit -m "test: add coverage for error scenarios in batch operations"
```

### Creating a Pull Request

1. Push your branch to your fork:
   ```bash
   git push origin feature/your-feature
   ```

2. Create PR on GitHub with:
   - **Title**: Short, descriptive
   - **Description**: Reference issue, explain changes
   - **Testing**: Describe tests added/modified
   - **Documentation**: List documentation updates
   - **Screenshots**: Include if UI-related

3. PR Template:

```markdown
## Description

Fixes #<issue_number>

Brief description of changes

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing

Describe the testing done:
- [ ] Unit tests added
- [ ] Integration tests added
- [ ] Manual testing performed

## Checklist

- [ ] Code follows style guidelines (`cargo fmt`)
- [ ] Self-review completed
- [ ] Commented complex logic
- [ ] Updated documentation
- [ ] Tests pass locally
- [ ] No new warnings from clippy
- [ ] Changes are backwards compatible
```

### Review Process

1. **Maintainer Review**
   - Code quality check
   - Testing verification
   - Documentation review
   - Compatibility assessment

2. **Feedback**
   - Respond to feedback promptly
   - Push additional commits (don't force push after review starts)
   - Mark conversations as resolved

3. **Merging**
   - Automatic merge after approval
   - Delete branch after merge
   - Celebrate contribution!

### After Merge

- Your code will be included in the next release
- You'll be credited in CHANGELOG.md and ACKNOWLEDGMENTS
- Join the community of contributors!

---

## Acknowledgments

### Contributors

We recognize and appreciate all contributors to HURL, including:

- **Bug reporters** - Help us find and fix issues
- **Feature suggesters** - Drive the product roadmap
- **Code contributors** - Implement new features and fixes
- **Documentation writers** - Keep us well-documented
- **Community managers** - Foster a welcoming environment
- **Translators** - Make HURL accessible globally

### Special Thanks

- **reqwest** - Excellent HTTP client library
- **tokio** - Outstanding async runtime
- **serde** - Serialization framework
- **clap** - CLI argument parsing
- All Rust community for incredible tools and support

### Getting Credit

By contributing to HURL, you agree to:
1. License your contribution under MIT OR Apache-2.0
2. Be listed in CHANGELOG.md and ACKNOWLEDGMENTS
3. Optionally include your name/GitHub in commits

---

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)
- [Git Guide](https://git-scm.com/book/en/v2)
- [GitHub Help](https://docs.github.com/)

---

## Questions?

- Open a GitHub Discussion
- Check existing issues
- Review documentation
- Ask in community chat

Thank you for contributing to HURL! 🎉
