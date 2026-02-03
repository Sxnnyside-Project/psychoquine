# PsychoQuine

<p align="center">
  <strong>Universal Quine Generator</strong><br>
  <em>A CoreRed Experimental Tool</em>
</p>

<p align="center">
  <a href="#about">About</a> •
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#architecture">Architecture</a> •
  <a href="#contributing">Contributing</a>
</p>

---

## About

**PsychoQuine** is a universal, resource-agnostic Quine generator capable of transforming arbitrary textual resources—programs, SDKs, configurations, or any text—into self-replicating program structures.

A [quine](https://en.wikipedia.org/wiki/Quine_(computing)) is a program that produces a copy of its own source code as its only output. PsychoQuine extends this concept by allowing you to embed any input into a quine structure.

### Philosophy

> *"Code that writes itself."*

PsychoQuine embodies the hacker ethos: minimal, precise, and uncompromising. No bloat, no unnecessary features—just raw functionality delivered with surgical precision.

This is a **CoreRed** project, part of an experimental toolkit for developers who appreciate the art of meta-programming.

## Features

- **Universal Input**: Accept any textual resource as input
- **Dual Output Formats**:
  - One-line quine: Compact, single-line representation
  - Multi-line quine: Formatted, readable representation
- **Multiple Escape Strategies**: Standard, Unicode, Hexadecimal, Raw
- **Zero Configuration**: Works out of the box with sensible defaults
- **Cross-Platform**: Desktop app for macOS, Windows, and Linux
- **CLI Tool**: Scriptable command-line interface
- **Library**: Embeddable Rust crate for custom integrations

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [Deno](https://deno.land/) (v1.40+)
- [Tauri CLI](https://tauri.app/) (for desktop builds)

### From Source

```bash
# Clone the repository
git clone https://github.com/Sxnnyside-Project/psychoquine.git
cd psychoquine

# Build the core library and CLI
cargo build --release

# The CLI binary will be at target/release/psychoquine
```

### CLI Only

```bash
# Install directly via Cargo
cargo install --path core
```

### Desktop App

```bash
# Install Tauri CLI if not already installed
cargo install tauri-cli

# Run in development mode
cd src-tauri
cargo tauri dev

# Build for production
cargo tauri build
```

## Usage

### CLI

```bash
# Basic usage - pipe input
echo "Hello, World!" | psychoquine

# Direct input
psychoquine "Your text here"

# One-line output only
psychoquine -o "input text"

# Multi-line output only
psychoquine -m "input text"

# With Unicode escaping
psychoquine -e unicode "input with émojis 🎉"

# Show statistics
psychoquine -s "input text"

# Quiet mode (no banner)
psychoquine -q "input text"
```

### Desktop App

1. Launch PsychoQuine
2. Enter your text in the **INPUT** panel
3. Select your preferred escape strategy
4. Click **GENERATE QUINE**
5. Toggle between **ONE-LINE** and **MULTI-LINE** views
6. Click **COPY** to copy the output to clipboard

### As a Library

```rust
use psychoquine_core::{generate, FormatOptions, QuineGenerator};

// Simple usage
let output = generate("Hello, World!")?;
println!("One-line: {}", output.one_line);
println!("Multi-line: {}", output.multi_line);

// With custom options
let generator = QuineGenerator::with_options(
    FormatOptions::default()
        .with_escape_strategy(EscapeStrategy::Unicode)
        .with_indent("  ")
);
let output = generator.generate("Your input")?;
```

## Architecture

```
psychoquine/
├── core/               # Rust quine engine (library + CLI)
│   └── src/
│       ├── lib.rs      # Public API
│       ├── main.rs     # CLI binary
│       ├── generator.rs # Core generation logic
│       ├── formatter.rs # Output formatting
│       └── escape.rs   # Escape strategies
│
├── ui/                 # Fresh + Deno UI
│   ├── routes/         # Page routes
│   ├── islands/        # Interactive components
│   ├── components/     # Static components
│   └── static/         # CSS and assets
│
├── src-tauri/          # Tauri desktop wrapper
│   └── src/
│       ├── main.rs     # Tauri entry point
│       └── commands.rs # IPC commands
│
└── docs/               # Documentation
```

### Stack

| Component | Technology |
|-----------|------------|
| Core Engine | Rust |
| Desktop App | Tauri |
| UI Runtime | Deno |
| UI Framework | Fresh |
| Language | TypeScript |

## Project Status

**Current Version**: 0.1.0 (Alpha)

This project is under active development. Core functionality is complete, but the API may change before 1.0.

### Roadmap

- [x] Core quine generation engine
- [x] CLI interface
- [x] Desktop UI
- [ ] Additional language-specific quine templates
- [ ] Plugin system for custom transformations
- [ ] WASM build for browser usage

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Before contributing, please read our [Code of Conduct](CODE_OF_CONDUCT.md).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<p align="center">
  <strong>PsychoQuine</strong> — A CoreRed Project<br>
  <em>© 2026 Sxnnyside Project</em>
</p>
