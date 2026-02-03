# Changelog

All notable changes to PsychoQuine will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- WASM build for browser usage
- Additional language-specific quine templates
- Plugin system for custom transformations
- Performance optimizations for large inputs

---

## [0.1.0] - 2026-02-03

### Added
- Initial release of PsychoQuine
- Core quine generation engine in Rust
- Four escape strategies: Standard, Unicode, Hexadecimal, Raw
- One-line and multi-line output formats
- CLI interface with multiple options
- Tauri-based desktop application
- Fresh + Deno web UI with Islands architecture
- Dark, terminal-inspired UI design
- Real-time quine generation
- Copy-to-clipboard functionality
- Generation statistics (input size, output size, expansion ratio)
- Comprehensive documentation (User Guide, Developer Guide)
- MIT License
- Full test coverage for core engine

### Architecture
- Rust core library (`psychoquine-core`)
- Tauri desktop wrapper
- Fresh web framework with Deno runtime
- TypeScript-only UI implementation
- IPC-based communication between UI and backend

### Documentation
- README with installation and usage instructions
- Developer Guide with architecture overview
- User Guide with detailed usage examples
- Contributing guidelines
- Code of Conduct

---

## Release Notes

### 0.1.0 - "First Contact"

This is the first public release of PsychoQuine, a CoreRed experimental tool for universal quine generation.

**What's Working:**
- ✅ Core generation engine
- ✅ CLI tool with all features
- ✅ Desktop app with full UI
- ✅ Multiple escape strategies
- ✅ Dual output formats
- ✅ Cross-platform support

**Known Limitations:**
- Output format is JavaScript-based
- Maximum input size is 10 MB
- No custom template support yet

**Platform Support:**
- macOS (Intel + Apple Silicon)
- Windows (x64)
- Linux (x64)

---

[Unreleased]: https://github.com/Sxnnyside-Project/psychoquine/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Sxnnyside-Project/psychoquine/releases/tag/v0.1.0
