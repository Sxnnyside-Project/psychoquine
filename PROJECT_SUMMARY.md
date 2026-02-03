# PsychoQuine - Project Summary

## Overview

**PsychoQuine** is a universal, resource-agnostic Quine generator built as a CoreRed experimental tool. It transforms arbitrary textual input into self-replicating program structures.

**Version**: 0.1.0  
**License**: MIT  
**Status**: Alpha (Production-Ready)

---

## Repository Structure

```
psychoquine/
├── core/                    # Rust core engine (library + CLI)
│   ├── src/
│   │   ├── lib.rs          # Public API
│   │   ├── main.rs         # CLI binary
│   │   ├── generator.rs    # Core generation logic
│   │   ├── formatter.rs    # Output formatting
│   │   └── escape.rs       # Escape strategies
│   └── Cargo.toml
│
├── src-tauri/              # Tauri desktop application
│   ├── src/
│   │   ├── main.rs         # Tauri entry point
│   │   └── commands.rs     # IPC command handlers
│   ├── tauri.conf.json     # Tauri configuration
│   ├── build.rs            # Build script
│   └── Cargo.toml
│
├── ui/                     # Fresh + Deno web interface
│   ├── routes/             # File-based routing
│   │   ├── index.tsx       # Main page
│   │   ├── _app.tsx        # App wrapper
│   │   └── _404.tsx        # 404 page
│   ├── islands/            # Interactive components (hydrated)
│   │   └── QuineGenerator.tsx
│   ├── components/         # Static components
│   │   └── TerminalFrame.tsx
│   ├── static/
│   │   └── styles.css      # Dark terminal theme
│   ├── deno.json           # Deno configuration
│   ├── fresh.config.ts     # Fresh configuration
│   ├── fresh.gen.ts        # Generated manifest
│   ├── main.ts             # Entry point
│   └── dev.ts              # Dev server
│
├── docs/                   # Documentation
│   ├── DEVELOPER_GUIDE.md  # Architecture & development
│   └── USER_GUIDE.md       # Usage instructions
│
├── examples/               # Usage examples
│   ├── basic_usage.rs      # Rust API examples
│   └── README.md           # Examples documentation
│
├── .github/
│   └── workflows/
│       ├── ci.yml          # Continuous integration
│       └── release.yml     # Release automation
│
├── .vscode/                # VS Code configuration
│   ├── settings.json       # Editor settings
│   └── extensions.json     # Recommended extensions
│
├── README.md               # Main documentation
├── CONTRIBUTING.md         # Contribution guidelines
├── CODE_OF_CONDUCT.md      # Code of conduct
├── CHANGELOG.md            # Version history
├── SECURITY.md             # Security policy
├── QUICKSTART.md           # Quick start guide
├── LICENSE                 # MIT License
├── Cargo.toml              # Rust workspace config
├── Makefile                # Build automation
├── setup.sh                # Setup script
├── .gitignore              # Git ignore patterns
├── .editorconfig           # Editor configuration
└── rustfmt.toml            # Rust formatting rules
```

---

## Technology Stack

| Component | Technology | Purpose |
|-----------|------------|---------|
| Core Engine | Rust | Quine generation logic |
| CLI | Rust | Command-line interface |
| Desktop App | Tauri | Native desktop wrapper |
| UI Runtime | Deno | Secure TypeScript runtime |
| UI Framework | Fresh | Islands-based web framework |
| Language | TypeScript | Type-safe UI code |
| Styling | CSS | Dark terminal theme |

---

## Key Features

### Core Engine
- ✅ Universal text input acceptance
- ✅ Four escape strategies (Standard, Unicode, Hex, Raw)
- ✅ Dual output formats (one-line, multi-line)
- ✅ Deterministic generation
- ✅ Input validation and size limits
- ✅ Comprehensive error handling
- ✅ Statistics tracking

### CLI Tool
- ✅ Stdin/argument input
- ✅ Multiple output modes
- ✅ Escape strategy selection
- ✅ Statistics display
- ✅ Quiet mode for scripting
- ✅ Cross-platform support

### Desktop Application
- ✅ Native performance via Tauri
- ✅ Dark terminal-inspired UI
- ✅ Real-time generation
- ✅ Copy-to-clipboard
- ✅ Tab-based output switching
- ✅ Generation statistics
- ✅ Minimal bundle size

### UI/UX
- ✅ Hacker aesthetic
- ✅ High contrast design
- ✅ Monospace typography
- ✅ Fast and responsive
- ✅ No unnecessary animations
- ✅ Keyboard-friendly

---

## Architecture Highlights

### Separation of Concerns
- **Core**: Pure Rust library, no dependencies on UI
- **UI**: TypeScript-only, uses core via Tauri IPC
- **CLI**: Standalone binary, no GUI dependencies

### Security
- Tauri allowlist restricts capabilities
- No arbitrary code execution
- Input size limits enforced
- UTF-8 validation

### Performance
- Written in Rust for speed
- Minimal JavaScript (Islands architecture)
- Lazy hydration of interactive components
- Efficient string escaping algorithms

---

## Quality Metrics

### Testing
- ✅ Unit tests for core engine
- ✅ Error case coverage
- ✅ Escape strategy tests
- ✅ Builder pattern tests

### Code Quality
- ✅ Formatted with rustfmt
- ✅ Linted with Clippy
- ✅ Zero warnings in release mode
- ✅ Type-safe TypeScript

### Documentation
- ✅ Inline API documentation
- ✅ Comprehensive README
- ✅ Developer guide
- ✅ User guide
- ✅ Examples with explanations

### CI/CD
- ✅ Automated testing on push/PR
- ✅ Multi-platform builds (macOS, Linux, Windows)
- ✅ Automated releases
- ✅ Format and lint checks

---

## Build & Run

### Quick Start
```bash
# Setup everything
./setup.sh

# Run CLI
cargo run --package psychoquine-core -- "input"

# Run desktop app
cd src-tauri && cargo tauri dev

# Run UI standalone
cd ui && deno task dev
```

### With Make
```bash
make dev          # Development mode
make build        # Build everything
make test         # Run tests
make check        # Format, lint, test
make install      # Install CLI
```

---

## Design Philosophy

### CoreRed Values
- **Minimal**: No bloat, only essential features
- **Precise**: Deterministic, predictable behavior
- **Intimidating**: Hacker aesthetic, no hand-holding
- **Experimental**: Pushing boundaries of meta-programming

### Color Palette
- Black (#000000) - Background
- Deep Red (#b00000) - Accent
- Gray shades - Text and borders

### Typography
- Monospace only (JetBrains Mono, Fira Code)
- High contrast
- No rounded fonts

---

## Roadmap

### Current (v0.1.0)
- ✅ Core quine generation
- ✅ CLI interface
- ✅ Desktop application
- ✅ Multiple escape strategies
- ✅ Dual output formats

### Planned (v0.2.0)
- ⏳ WASM build for browser
- ⏳ Additional language templates
- ⏳ Plugin system
- ⏳ Performance optimizations

### Future (v1.0.0)
- 🔮 Custom template engine
- 🔮 Language-specific quines
- 🔮 Advanced obfuscation options
- 🔮 Web-based playground

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development setup
- Code style guidelines
- Commit message format
- Pull request process

---

## License

MIT License - See [LICENSE](LICENSE) for details.

Free to use, modify, and distribute.

---

## Links

- **Repository**: https://github.com/Sxnnyside-Project/psychoquine
- **Issues**: https://github.com/Sxnnyside-Project/psychoquine/issues
- **Releases**: https://github.com/Sxnnyside-Project/psychoquine/releases

---

## Credits

**PsychoQuine** is a CoreRed experimental project.

© 2026 Sxnnyside Project

---

*Built with Rust, Tauri, Deno, and Fresh.*

*"Code that writes itself."*
