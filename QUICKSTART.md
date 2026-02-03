# Quick Start Guide

Get started with PsychoQuine in under 5 minutes.

## Installation

### Option 1: Install from Source (Recommended for Development)

```bash
git clone https://github.com/Sxnnyside-Project/psychoquine.git
cd psychoquine
./setup.sh
```

### Option 2: Install CLI Only

```bash
cargo install psychoquine-core
```

### Option 3: Download Desktop App

Download the latest release for your platform:
- [macOS (Apple Silicon)](https://github.com/Sxnnyside-Project/psychoquine/releases)
- [macOS (Intel)](https://github.com/Sxnnyside-Project/psychoquine/releases)
- [Windows](https://github.com/Sxnnyside-Project/psychoquine/releases)
- [Linux](https://github.com/Sxnnyside-Project/psychoquine/releases)

## First Steps

### Using the CLI

```bash
# Generate a quine from text
psychoquine "console.log('Hello');"

# From a file
cat mycode.js | psychoquine

# One-line output only
psychoquine -o "your code here"
```

### Using the Desktop App

1. Launch PsychoQuine
2. Paste your code in the INPUT panel
3. Click "GENERATE QUINE"
4. Toggle between ONE-LINE and MULTI-LINE views
5. Click "COPY" to copy the result

### Using as a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
psychoquine-core = "0.1"
```

Then in your code:

```rust
use psychoquine_core::generate;

fn main() {
    let output = generate("console.log('test');").unwrap();
    println!("{}", output.one_line);
}
```

## What is a Quine?

A quine is a program that outputs its own source code. Run this to see:

```bash
# Generate a quine
psychoquine "alert('I print myself');" > quine.js

# Execute it
node quine.js > output.js

# Verify they're the same
diff quine.js output.js
```

No differences! The quine reproduced itself perfectly.

## Common Use Cases

### Code Obfuscation

```bash
cat readable.js | psychoquine -o > obfuscated.js
```

### Meta-Programming Study

Generate and analyze self-referential code structures.

### Data Embedding

Embed data in executable format:

```bash
echo '{"key": "value"}' | psychoquine -o
```

## Next Steps

- Read the [User Guide](docs/USER_GUIDE.md) for detailed usage
- Check out [Examples](examples/README.md) for more use cases
- See the [Developer Guide](docs/DEVELOPER_GUIDE.md) to contribute

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/Sxnnyside-Project/psychoquine/issues)
- **Documentation**: See the `docs/` folder
- **Examples**: See the `examples/` folder

---

**Happy Quine Generation!**
