# ══════════════════════════════════════════════════════════════════════════════
# PsychoQuine Makefile
# ══════════════════════════════════════════════════════════════════════════════

.PHONY: help build test clean install dev fmt lint check all

# Default target
help:
	@echo "═══════════════════════════════════════════════════════════════"
	@echo "  PsychoQuine - Makefile Commands"
	@echo "═══════════════════════════════════════════════════════════════"
	@echo ""
	@echo "Development:"
	@echo "  make dev          - Run Tauri in development mode"
	@echo "  make dev-ui       - Run UI standalone (Deno)"
	@echo "  make dev-cli      - Run CLI with example input"
	@echo ""
	@echo "Building:"
	@echo "  make build        - Build all components"
	@echo "  make build-core   - Build core library and CLI"
	@echo "  make build-tauri  - Build Tauri desktop app"
	@echo "  make build-release - Build everything in release mode"
	@echo ""
	@echo "Testing:"
	@echo "  make test         - Run all tests"
	@echo "  make test-core    - Run core library tests"
	@echo ""
	@echo "Code Quality:"
	@echo "  make fmt          - Format all code"
	@echo "  make lint         - Run all linters"
	@echo "  make check        - Run all checks (fmt, lint, test)"
	@echo ""
	@echo "Maintenance:"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make install      - Install CLI locally"
	@echo ""

# ──────────────────────────────────────────────────────────────────────────────
# Development
# ──────────────────────────────────────────────────────────────────────────────

dev:
	@echo "Starting Tauri development mode..."
	cd src-tauri && cargo tauri dev

dev-ui:
	@echo "Starting UI development server..."
	cd ui && deno task dev

dev-cli:
	@echo "Running CLI with example..."
	cargo run --package psychoquine-core -- "console.log('Hello, World!');"

# ──────────────────────────────────────────────────────────────────────────────
# Building
# ──────────────────────────────────────────────────────────────────────────────

build: build-core

build-core:
	@echo "Building core library..."
	cargo build --package psychoquine-core

build-tauri:
	@echo "Building Tauri desktop app..."
	cd src-tauri && cargo tauri build

build-release:
	@echo "Building release versions..."
	cargo build --release --workspace
	@which cargo-tauri > /dev/null 2>&1 || (echo "Installing tauri-cli..." && cargo install tauri-cli)
	cd src-tauri && cargo tauri build

# ──────────────────────────────────────────────────────────────────────────────
# Testing
# ──────────────────────────────────────────────────────────────────────────────

test:
	@echo "Running all tests..."
	cargo test --workspace

test-core:
	@echo "Running core tests..."
	cargo test --package psychoquine-core

test-verbose:
	@echo "Running tests with output..."
	cargo test --workspace -- --nocapture

# ──────────────────────────────────────────────────────────────────────────────
# Code Quality
# ──────────────────────────────────────────────────────────────────────────────

fmt:
	@echo "Formatting Rust code..."
	cargo fmt --all
	@echo "Formatting Deno code..."
	cd ui && deno fmt

fmt-check:
	@echo "Checking Rust formatting..."
	cargo fmt --all -- --check
	@echo "Checking Deno formatting..."
	cd ui && deno fmt --check

lint:
	@echo "Running Clippy..."
	cargo clippy --all-targets --all-features -- -D warnings
	@echo "Running Deno lint..."
	cd ui && deno lint

check: fmt-check lint test
	@echo ""
	@echo "═══════════════════════════════════════════════════════════════"
	@echo "  All checks passed! ✓"
	@echo "═══════════════════════════════════════════════════════════════"

# ──────────────────────────────────────────────────────────────────────────────
# Maintenance
# ──────────────────────────────────────────────────────────────────────────────

clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf ui/_fresh
	rm -rf target

install:
	@echo "Installing CLI..."
	cargo install --path core

uninstall:
	@echo "Uninstalling CLI..."
	cargo uninstall psychoquine-core

# ──────────────────────────────────────────────────────────────────────────────
# Documentation
# ──────────────────────────────────────────────────────────────────────────────

docs:
	@echo "Building Rust documentation..."
	cargo doc --no-deps --open

docs-all:
	@echo "Building all documentation..."
	cargo doc --workspace --no-deps --open

# ──────────────────────────────────────────────────────────────────────────────
# Utility
# ──────────────────────────────────────────────────────────────────────────────

update:
	@echo "Updating dependencies..."
	cargo update
	cd ui && deno cache --reload main.ts

version:
	@echo "PsychoQuine versions:"
	@echo "  Core:  $$(cargo pkgid -p psychoquine-core | cut -d'#' -f2)"
	@echo "  Tauri: $$(cargo pkgid -p psychoquine-tauri | cut -d'#' -f2)"
	@echo ""
	@rustc --version
	@deno --version | head -n 1

all: fmt lint test build
	@echo ""
	@echo "═══════════════════════════════════════════════════════════════"
	@echo "  Build complete! ✓"
	@echo "═══════════════════════════════════════════════════════════════"
