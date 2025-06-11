# Makefile for bracket-parser

.PHONY: all build test doc clean quality fmt lint check-outdated audit

# Default target
all: build

# Build the project
build:
	cargo build

# Build with release optimization
release:
	cargo build --release

# Run the project
run:
	cargo run

# Run tests
test:
	cargo test

# Generate documentation
doc:
	cargo doc --no-deps

# Clean build artifacts
clean:
	cargo clean

# Format code
fmt:
	cargo fmt

# Run linter
lint:
	cargo clippy -- -D warnings

# Check for outdated dependencies
check-outdated:
	cargo outdated

# Run security audit
audit:
	cargo audit

# Run all quality checks
quality:
	bash scripts/check_quality.sh

# Install development dependencies
dev-deps:
	cargo install cargo-audit cargo-outdated

# Help target
help:
	@echo "Available targets:"
	@echo "  all            - Build the project (default)"
	@echo "  build          - Build the project"
	@echo "  release        - Build with release optimization"
	@echo "  run            - Run the project"
	@echo "  test           - Run tests"
	@echo "  doc            - Generate documentation"
	@echo "  clean          - Clean build artifacts"
	@echo "  fmt            - Format code"
	@echo "  lint           - Run linter"
	@echo "  check-outdated - Check for outdated dependencies"
	@echo "  audit          - Run security audit"
	@echo "  quality        - Run all quality checks"
	@echo "  dev-deps       - Install development dependencies"
	@echo "  help           - Show this help message"