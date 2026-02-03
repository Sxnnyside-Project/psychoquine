# PsychoQuine Developer Guide

Welcome to the PsychoQuine developer documentation. This guide provides a comprehensive overview of the architecture, implementation details, and development workflow.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Core Engine](#core-engine)
3. [Desktop Application](#desktop-application)
4. [UI Layer](#ui-layer)
5. [Build System](#build-system)
6. [Testing](#testing)
7. [Deployment](#deployment)

---

## Architecture Overview

PsychoQuine follows a layered architecture with clear separation of concerns:

```
┌─────────────────────────────────────────┐
│           UI Layer (Fresh)              │
│         TypeScript + Preact             │
└─────────────────┬───────────────────────┘
                  │
┌─────────────────▼───────────────────────┐
│      Tauri Bridge (IPC)                 │
│    JSON-based Command Interface         │
└─────────────────┬───────────────────────┘
                  │
┌─────────────────▼───────────────────────┐
│     Core Engine (Rust Library)          │
│   QuineGenerator + Formatters           │
└─────────────────────────────────────────┘
```

### Design Principles

1. **Resource Agnostic**: Accept any textual input
2. **Deterministic**: Same input always produces same output
3. **Zero Config**: Works out of the box with sensible defaults
4. **Extensible**: Easy to add new escape strategies and formatters
5. **Fast**: Written in Rust for maximum performance

---

## Core Engine

Location: `/core/src/`

The core engine is a Rust library that handles all quine generation logic.

### Module Structure

```
core/src/
├── lib.rs           # Public API and re-exports
├── generator.rs     # Main generation logic
├── escape.rs        # Escape strategy implementations
├── formatter.rs     # Output formatting
└── main.rs          # CLI binary
```

### Key Components

#### QuineGenerator

The main entry point for quine generation.

```rust
pub struct QuineGenerator {
    options: FormatOptions,
    max_input_size: usize,
}
```

**Responsibilities:**
- Input validation
- Coordinating escape and formatting
- Generating both one-line and multi-line outputs
- Computing statistics

**Usage:**

```rust
use psychoquine_core::{QuineGenerator, FormatOptions};

let generator = QuineGenerator::new();
let output = generator.generate("Hello, World!")?;

println!("One-line: {}", output.one_line);
println!("Multi-line: {}", output.multi_line);
```

#### EscapeStrategy

Defines how special characters are escaped.

```rust
pub enum EscapeStrategy {
    Standard,      // \n, \t, \"
    Unicode,       // \u{XXXX}
    Hexadecimal,   // \xXX
    Raw,           // Minimal escaping
}
```

**Implementation:**

Each strategy implements character-by-character transformation:

```rust
fn escape_standard(input: &str) -> String {
    let mut output = String::with_capacity(input.len() * 2);
    for ch in input.chars() {
        match ch {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            // ... more cases
        }
    }
    output
}
```

#### Formatter

Transforms escaped data into final quine structure.

```rust
pub struct Formatter {
    options: FormatOptions,
}
```

**Quine Template:**

The core quine structure uses a self-referential pattern:

```javascript
(function(){
    var d="<ESCAPED_DATA>";
    var q=String.fromCharCode(34);
    console.log("(function(){var d="+q+d+q+";"+d.slice(d.indexOf("var q")))
})()
```

This pattern:
1. Stores data in variable `d`
2. Creates quote character dynamically
3. Reconstructs itself by concatenating code + data

### Error Handling

All errors use `thiserror` for ergonomic error definitions:

```rust
#[derive(Error, Debug)]
pub enum QuineError {
    #[error("Input is empty")]
    EmptyInput,
    
    #[error("Input exceeds maximum size of {max} bytes (got {actual})")]
    InputTooLarge { max: usize, actual: usize },
    
    #[error("Input contains invalid UTF-8 sequences")]
    InvalidUtf8,
}
```

### Testing Strategy

The core engine has comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_generation() {
        let generator = QuineGenerator::new();
        let result = generator.generate("hello world");
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_input_error() {
        let generator = QuineGenerator::new();
        let result = generator.generate("");
        assert!(matches!(result, Err(QuineError::EmptyInput)));
    }
}
```

Run tests with:

```bash
cargo test
cargo test --release  # With optimizations
```

---

## Desktop Application

Location: `/src-tauri/`

The desktop app uses Tauri to provide a native wrapper around the web UI.

### Architecture

```
src-tauri/
├── src/
│   ├── main.rs       # Tauri initialization
│   └── commands.rs   # Command handlers
├── tauri.conf.json   # Tauri configuration
└── build.rs          # Build script
```

### Tauri Commands

Commands are exposed to the frontend via the `#[tauri::command]` macro:

```rust
#[tauri::command]
pub fn generate_quine(
    input: String, 
    options: Option<GenerateOptions>
) -> GenerateResponse {
    let generator = QuineGenerator::with_options(format_options);
    generator.generate(&input).into()
}
```

### IPC Communication

Frontend invokes commands using Tauri's `invoke` API:

```typescript
import { invoke } from "@tauri-apps/api";

const response = await invoke<GenerateResponse>("generate_quine", {
  input: "Hello, World!",
  options: {
    escape_strategy: "standard"
  }
});
```

### Security

Tauri's allowlist ensures only specific APIs are exposed:

```json
{
  "allowlist": {
    "all": false,
    "shell": {
      "open": true
    },
    "clipboard": {
      "writeText": true
    }
  }
}
```

---

## UI Layer

Location: `/ui/`

The UI is built with Fresh (Deno's web framework) and uses Islands architecture.

### Project Structure

```
ui/
├── routes/          # File-based routing
│   ├── index.tsx    # Home page
│   ├── _app.tsx     # App wrapper
│   └── _404.tsx     # 404 page
├── islands/         # Interactive components
│   └── QuineGenerator.tsx
├── components/      # Static components
│   └── TerminalFrame.tsx
├── static/          # Static assets
│   └── styles.css
├── deno.json        # Deno configuration
└── main.ts          # Entry point
```

### Islands Architecture

Fresh uses Islands for hydration:

- **Routes**: Server-rendered pages
- **Islands**: Client-side interactive components
- **Components**: Static, non-interactive components

Only Islands are shipped to the client with JavaScript.

### QuineGenerator Island

The main interactive component:

```typescript
export default function QuineGenerator() {
  const input = useSignal("");
  const output = useSignal<QuineOutput | null>(null);
  
  const handleGenerate = useCallback(async () => {
    if (isTauri) {
      const response = await invoke("generate_quine", { input: input.value });
      output.value = response.data;
    } else {
      // Web fallback
      output.value = generateQuineFallback(input.value);
    }
  }, []);
  
  // ... render logic
}
```

### Styling Philosophy

PsychoQuine follows a dark, terminal-inspired aesthetic:

**Color Palette:**
- Black (`#000000`) - Primary background
- Deep Red (`#b00000`) - Accent color
- Gray shades - Text and borders

**Typography:**
- Monospace fonts only
- JetBrains Mono, Fira Code, SF Mono

**Layout:**
- High contrast
- Sharp edges, no rounded corners
- Minimal animations
- Terminal-like interface

### Deno Configuration

```json
{
  "tasks": {
    "dev": "deno run -A --watch=static/,routes/ dev.ts",
    "build": "deno run -A dev.ts build",
    "start": "deno run -A main.ts"
  },
  "imports": {
    "$fresh/": "https://deno.land/x/fresh@1.6.1/",
    "preact": "https://esm.sh/preact@10.19.3",
    "preact/": "https://esm.sh/preact@10.19.3/"
  }
}
```

---

## Build System

### Workspace Configuration

The project uses Cargo workspaces:

```toml
[workspace]
resolver = "2"
members = ["core", "src-tauri"]

[workspace.package]
version = "0.1.0"
edition = "2021"
```

### Building the CLI

```bash
cd core
cargo build --release

# Binary location
./target/release/psychoquine
```

### Building the Desktop App

```bash
cd src-tauri
cargo tauri build

# Outputs platform-specific installers:
# - macOS: .dmg, .app
# - Windows: .msi, .exe
# - Linux: .deb, .AppImage
```

### Development Workflow

```bash
# Terminal 1: Run Tauri dev mode
cd src-tauri
cargo tauri dev

# This automatically:
# 1. Starts the Deno dev server (ui/)
# 2. Compiles the Rust backend
# 3. Opens the desktop app with hot reload
```

---

## Testing

### Rust Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_basic_generation

# Run tests in release mode
cargo test --release
```

### Linting

```bash
# Clippy for linting
cargo clippy

# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Deno Tests

```bash
cd ui
deno test
```

---

## Deployment

### Publishing the CLI

```bash
# To crates.io
cargo publish --package psychoquine-core

# Install from crates.io
cargo install psychoquine-core
```

### Distributing the Desktop App

After building with `cargo tauri build`, distribute the installers:

- **macOS**: `.dmg` file
- **Windows**: `.msi` installer
- **Linux**: `.deb` or `.AppImage`

### GitHub Releases

Recommended workflow:

1. Tag the release: `git tag v0.1.0`
2. Push tag: `git push origin v0.1.0`
3. Build for all platforms
4. Upload installers to GitHub Releases
5. Include release notes

---

## Development Tips

### Debugging

**Rust:**
```bash
RUST_LOG=debug cargo run
```

**Tauri:**
```bash
# Open DevTools in Tauri app
Right-click > Inspect Element
```

### Performance Profiling

```bash
# Flamegraph
cargo install flamegraph
cargo flamegraph --bin psychoquine -- "input text"
```

### Hot Reload

Both the UI and Rust backend support hot reload in dev mode:

- UI: Automatic via Deno Fresh
- Rust: Recompiles on save via Tauri

---

## Contribution Workflow

1. **Fork** the repository
2. **Create branch**: `git checkout -b feature/your-feature`
3. **Make changes** and test thoroughly
4. **Run checks**:
   ```bash
   cargo fmt
   cargo clippy
   cargo test
   ```
5. **Commit**: Follow conventional commits
6. **Push** and create Pull Request

---

## Architecture Decisions

### Why Rust?

- Memory safety without garbage collection
- Excellent performance for text processing
- Strong type system prevents runtime errors
- Easy FFI for library usage

### Why Deno + Fresh?

- Modern TypeScript-first runtime
- Built-in tooling (formatter, linter, test runner)
- Secure by default
- Islands architecture for minimal JavaScript

### Why Tauri?

- Smaller bundle size than Electron
- Native performance
- Security-focused architecture
- Rust backend integration

---

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tauri Documentation](https://tauri.app/)
- [Deno Manual](https://deno.land/manual)
- [Fresh Documentation](https://fresh.deno.dev/)

---

**Questions?** Open an issue on GitHub.

*— The PsychoQuine Team*
