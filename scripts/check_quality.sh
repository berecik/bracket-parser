#!/bin/bash
set -e

# Script to control code quality for bracket-parser
# This script runs various checks to ensure code quality standards are met

echo "🔍 Running code quality checks for bracket-parser..."

# Check if all required tools are installed
echo -e "\n📋 Checking required tools..."
if ! command -v cargo &> /dev/null; then
    echo "❌ cargo is not installed. Please install Rust and Cargo first."
    exit 1
fi

# Check code formatting
echo -e "\n📝 Checking code formatting..."
cargo fmt -- --check || {
    echo "❌ Code formatting issues found. Run 'cargo fmt' to fix them."
    exit 1
}
echo "✅ Code formatting is correct."

# Run clippy for linting
echo -e "\n🔎 Running clippy for linting..."
cargo clippy -- -D warnings || {
    echo "❌ Clippy found issues. Please fix them before proceeding."
    exit 1
}
echo "✅ No linting issues found."

# Run tests
echo -e "\n🧪 Running tests..."
cargo test || {
    echo "❌ Some tests failed. Please fix them before proceeding."
    exit 1
}
echo "✅ All tests passed."

# Check documentation coverage
echo -e "\n📚 Checking documentation..."
cargo doc --no-deps || {
    echo "❌ Documentation build failed."
    exit 1
}
echo "✅ Documentation built successfully."

# Check for outdated dependencies
echo -e "\n📦 Checking for outdated dependencies..."
cargo outdated || {
    echo "⚠️ Some dependencies might be outdated. Consider updating them."
}

# Check for security vulnerabilities
echo -e "\n🔒 Checking for security vulnerabilities..."
if command -v cargo-audit &> /dev/null; then
    cargo audit || {
        echo "⚠️ Security vulnerabilities found. Please review them."
    }
else
    echo "⚠️ cargo-audit is not installed. Install it with 'cargo install cargo-audit' to check for security vulnerabilities."
fi

echo -e "\n✨ All quality checks completed!"
echo "If all checks passed, your code meets the quality standards."