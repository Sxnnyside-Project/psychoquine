#!/usr/bin/env bash
# ════════════════════════════════════════════════════════════════════════════
# PsychoQuine - Repository Verification Script
# ════════════════════════════════════════════════════════════════════════════

set -e

echo ""
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║  PsychoQuine Repository Verification                         ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

ERRORS=0

# Function to check file exists
check_file() {
    if [ -f "$1" ]; then
        echo -e "${GREEN}✓${NC} $1"
    else
        echo -e "${RED}✗${NC} $1 (missing)"
        ERRORS=$((ERRORS + 1))
    fi
}

# Function to check directory exists
check_dir() {
    if [ -d "$1" ]; then
        echo -e "${GREEN}✓${NC} $1/"
    else
        echo -e "${RED}✗${NC} $1/ (missing)"
        ERRORS=$((ERRORS + 1))
    fi
}

echo "Checking root files..."
check_file "README.md"
check_file "LICENSE"
check_file "CONTRIBUTING.md"
check_file "CODE_OF_CONDUCT.md"
check_file "CHANGELOG.md"
check_file "SECURITY.md"
check_file "QUICKSTART.md"
check_file "PROJECT_SUMMARY.md"
check_file "FILE_MANIFEST.md"
check_file "Cargo.toml"
check_file "Makefile"
check_file "setup.sh"
check_file ".gitignore"
check_file ".editorconfig"
check_file "rustfmt.toml"

echo ""
echo "Checking core files..."
check_dir "core"
check_dir "core/src"
check_file "core/Cargo.toml"
check_file "core/src/lib.rs"
check_file "core/src/main.rs"
check_file "core/src/generator.rs"
check_file "core/src/formatter.rs"
check_file "core/src/escape.rs"

echo ""
echo "Checking Tauri files..."
check_dir "src-tauri"
check_dir "src-tauri/src"
check_file "src-tauri/Cargo.toml"
check_file "src-tauri/build.rs"
check_file "src-tauri/tauri.conf.json"
check_file "src-tauri/src/main.rs"
check_file "src-tauri/src/commands.rs"

echo ""
echo "Checking UI files..."
check_dir "ui"
check_dir "ui/routes"
check_dir "ui/islands"
check_dir "ui/components"
check_dir "ui/static"
check_file "ui/deno.json"
check_file "ui/main.ts"
check_file "ui/dev.ts"
check_file "ui/fresh.config.ts"
check_file "ui/fresh.gen.ts"
check_file "ui/routes/index.tsx"
check_file "ui/routes/_app.tsx"
check_file "ui/routes/_404.tsx"
check_file "ui/islands/QuineGenerator.tsx"
check_file "ui/components/TerminalFrame.tsx"
check_file "ui/static/styles.css"

echo ""
echo "Checking documentation..."
check_dir "docs"
check_file "docs/DEVELOPER_GUIDE.md"
check_file "docs/USER_GUIDE.md"

echo ""
echo "Checking examples..."
check_dir "examples"
check_file "examples/README.md"
check_file "examples/basic_usage.rs"

echo ""
echo "Checking GitHub workflows..."
check_dir ".github"
check_dir ".github/workflows"
check_file ".github/workflows/ci.yml"
check_file ".github/workflows/release.yml"

echo ""
echo "Checking VS Code configuration..."
check_dir ".vscode"
check_file ".vscode/settings.json"
check_file ".vscode/extensions.json"

echo ""
echo "═══════════════════════════════════════════════════════════════"

if [ $ERRORS -eq 0 ]; then
    echo -e "${GREEN}✓ All files present!${NC}"
    echo ""
    echo "Repository structure is complete."
    echo ""
    echo "Next steps:"
    echo "  1. Run: ./setup.sh"
    echo "  2. Run: make check"
    echo "  3. Run: make dev"
    echo ""
    exit 0
else
    echo -e "${RED}✗ Found $ERRORS missing file(s)${NC}"
    echo ""
    echo "Please ensure all required files are present."
    echo ""
    exit 1
fi
