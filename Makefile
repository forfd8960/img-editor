.PHONY: help install dev build clean check test lint format

# Default target
help:
	@echo "Available targets:"
	@echo "  make install     - Install all dependencies (Rust + Frontend)"
	@echo "  make dev         - Start development server"
	@echo "  make build       - Build release version"
	@echo "  make check       - Check Rust code compilation"
	@echo "  make test        - Run all tests"
	@echo "  make lint        - Run linters"
	@echo "  make format      - Format code"
	@echo "  make clean       - Clean build artifacts"

# Install dependencies
install:
	@echo "Installing frontend dependencies..."
	cd frontend && npm install
	@echo "Dependencies installed!"

# Start development server
dev:
	@echo "Starting Tauri development server..."
	npm run tauri dev

# Build for production
build:
	@echo "Building Tauri application..."
	npm run tauri build

# Check Rust compilation
check:
	@echo "Checking Rust code..."
	cargo check
	@echo "Checking library..."
	cargo check --lib

# Run tests
test:
	@echo "Running Rust tests..."
	cargo test
	@echo "Running frontend tests..."
	cd frontend && npm test

# Run linters
lint:
	@echo "Running Rust linter..."
	cargo clippy -- -D warnings
	@echo "Running frontend linter..."
	cd frontend && npm run lint

# Format code
format:
	@echo "Formatting Rust code..."
	cargo fmt
	@echo "Formatting frontend code..."
	cd frontend && npm run format || echo "No format script defined"

# Clean build artifacts
clean:
	@echo "Cleaning Rust artifacts..."
	cargo clean
	@echo "Cleaning frontend artifacts..."
	cd frontend && rm -rf dist node_modules
	@echo "Clean complete!"
