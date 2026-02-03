# Contributing to PsychoQuine

First off, thank you for considering contributing to PsychoQuine. This is a CoreRed experimental project, and we appreciate any help in making it better.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Pull Request Process](#pull-request-process)
- [Code Style](#code-style)
- [Commit Guidelines](#commit-guidelines)

## Code of Conduct

This project adheres to a Code of Conduct. By participating, you are expected to uphold this code. Please read [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before contributing.

## Getting Started

1. Fork the repository
2. Clone your fork locally
3. Set up the development environment
4. Create a branch for your changes
5. Make your changes
6. Test your changes
7. Submit a pull request

## Development Setup

### Prerequisites

- **Rust** (stable toolchain) - [Install](https://rustup.rs/)
- **Deno** (v1.40+) - [Install](https://deno.land/)
- **Tauri CLI** - Install via `cargo install tauri-cli`

### Building

```bash
# Clone the repository
git clone https://github.com/Sxnnyside-Project/psychoquine.git
cd psychoquine

# Build the core library
cargo build

# Run tests
cargo test

# Build in release mode
cargo build --release
```

### Running the Desktop App

```bash
# Development mode
cd src-tauri
cargo tauri dev

# Production build
cargo tauri build
```

### Running the UI Standalone

```bash
cd ui
deno task dev
```

## How to Contribute

### Reporting Bugs

Before creating a bug report, please check existing issues to avoid duplicates.

When reporting a bug, include:

- **Clear title**: Descriptive summary of the issue
- **Steps to reproduce**: Detailed steps to reproduce the behavior
- **Expected behavior**: What you expected to happen
- **Actual behavior**: What actually happened
- **Environment**: OS, Rust version, Deno version
- **Screenshots**: If applicable

### Suggesting Features

Feature suggestions are welcome. Please provide:

- **Clear title**: Descriptive summary of the feature
- **Use case**: Why this feature would be useful
- **Proposed solution**: How you envision the feature working
- **Alternatives considered**: Other solutions you've thought about

### Code Contributions

We accept contributions for:

- Bug fixes
- Performance improvements
- Documentation improvements
- New features (please discuss first via an issue)

## Pull Request Process

1. **Create an issue first** for significant changes
2. **Fork and branch**: Create a feature branch from `main`
3. **Follow code style**: Ensure your code follows our style guidelines
4. **Write tests**: Add tests for new functionality
5. **Update documentation**: Update relevant documentation
6. **Run all tests**: Ensure all tests pass
7. **Commit properly**: Follow commit message guidelines
8. **Submit PR**: Create a pull request with a clear description

### PR Requirements

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Code follows style guidelines
- [ ] Documentation is updated
- [ ] Commit messages follow guidelines
- [ ] PR description explains the changes

## Code Style

### Rust

- Follow the [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Write documentation comments for public APIs
- Keep functions focused and small

```rust
// Good
/// Generates a quine from the input string.
///
/// # Arguments
/// * `input` - The source text to transform
///
/// # Returns
/// A `QuineOutput` containing both one-line and multi-line representations
pub fn generate(input: &str) -> Result<QuineOutput, QuineError> {
    // implementation
}
```

### TypeScript

- Use TypeScript, not JavaScript
- Follow Deno style conventions
- Use explicit types where helpful
- Keep components small and focused

```typescript
// Good
interface QuineOutput {
  oneLine: string;
  multiLine: string;
}

function generateQuine(input: string): QuineOutput {
  // implementation
}
```

### CSS

- Use CSS custom properties (variables)
- Follow the established color palette
- Keep selectors specific but not overly nested
- Comment sections for organization

## Commit Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/).

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples

```
feat(core): add Unicode escape strategy

Implements Unicode escape sequences (\u{XXXX}) for handling
special characters in quine output.

Closes #123
```

```
fix(ui): correct copy button state after clipboard write

The copy button now properly resets its state after the
clipboard operation completes.
```

```
docs: update installation instructions for Windows
```

### Rules

- Use imperative mood ("add" not "added")
- Don't capitalize the first letter
- No period at the end of the subject
- Limit subject line to 50 characters
- Wrap body at 72 characters

## Questions?

If you have questions, feel free to:

- Open an issue with the `question` label
- Start a discussion in the repository

---

Thank you for contributing to PsychoQuine!

*— The Sxnnyside Project Team*
