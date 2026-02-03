#!/usr/bin/env bash
# ════════════════════════════════════════════════════════════════════════════
# PsychoQuine - Development Setup Script
# ════════════════════════════════════════════════════════════════════════════

set -e

echo ""
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║  PsychoQuine Development Setup                                ║"
echo "║  CoreRed Project                                              ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check for Rust
echo -n "Checking for Rust... "
if command -v cargo &> /dev/null; then
    RUST_VERSION=$(rustc --version | awk '{print $2}')
    echo -e "${GREEN}✓${NC} Found (v$RUST_VERSION)"
else
    echo -e "${RED}✗${NC} Not found"
    echo ""
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check for Deno
echo -n "Checking for Deno... "
if command -v deno &> /dev/null; then
    DENO_VERSION=$(deno --version | head -n 1 | awk '{print $2}')
    echo -e "${GREEN}✓${NC} Found (v$DENO_VERSION)"
else
    echo -e "${RED}✗${NC} Not found"
    echo ""
    echo "Please install Deno from https://deno.land/"
    exit 1
fi

# Check for Tauri CLI
echo -n "Checking for Tauri CLI... "
if command -v cargo-tauri &> /dev/null; then
    echo -e "${GREEN}✓${NC} Found"
else
    echo -e "${YELLOW}!${NC} Not found"
    echo ""
    echo "Installing Tauri CLI..."
    cargo install tauri-cli --version "^1.0"
fi

echo ""
echo "─────────────────────────────────────────────────────────────────"
echo "Building the project..."
echo "─────────────────────────────────────────────────────────────────"
echo ""

# Build core library
echo "Building core library..."
cargo build --package psychoquine-core
echo -e "${GREEN}✓${NC} Core library built"
echo ""

# Run tests
echo "Running tests..."
cargo test
echo -e "${GREEN}✓${NC} Tests passed"
echo ""

# Check formatting
echo "Checking code formatting..."
cargo fmt --check
echo -e "${GREEN}✓${NC} Code is formatted"
echo ""

# Run Clippy
echo "Running Clippy lints..."
cargo clippy -- -D warnings
echo -e "${GREEN}✓${NC} No lint warnings"
echo ""

echo "─────────────────────────────────────────────────────────────────"
echo "Setup complete!"
echo "─────────────────────────────────────────────────────────────────"
echo ""
echo "Next steps:"
echo ""
echo "  1. Run the CLI:"
echo "     cargo run --package psychoquine-core -- \"Hello, World!\""
echo ""
echo "  2. Start the desktop app in dev mode:"
echo "     cd src-tauri && cargo tauri dev"
echo ""
echo "  3. Run the UI standalone:"
echo "     cd ui && deno task dev"
echo ""
echo "For more information, see docs/DEVELOPER_GUIDE.md"
echo ""
