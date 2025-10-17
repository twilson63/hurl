.PHONY: build test fmt lint clean check help dev

help:
	@echo "HURL - Modern HTTP CLI"
	@echo ""
	@echo "Available commands:"
	@echo "  make build        - Build the project"
	@echo "  make test         - Run all tests"
	@echo "  make fmt          - Format code with rustfmt"
	@echo "  make fmt-check    - Check code formatting"
	@echo "  make lint         - Run clippy linter"
	@echo "  make check        - Run cargo check"
	@echo "  make clean        - Remove build artifacts"
	@echo "  make dev          - Run in development mode"
	@echo "  make doc          - Generate documentation"

build:
	cargo build --release

test:
	cargo test --all

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --all --all-targets -- -D warnings

check:
	cargo check --all

clean:
	cargo clean

dev:
	cargo run --bin hurl -- --help

doc:
	cargo doc --no-deps --open
