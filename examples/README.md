# PsychoQuine Examples

This directory contains examples demonstrating various use cases of PsychoQuine.

## Running Examples

### Rust Examples

```bash
# Run the basic usage example
cargo run --example basic_usage

# Run with specific features
cargo run --example basic_usage --features "serde"
```

## Available Examples

### 1. Basic Usage (`basic_usage.rs`)

Demonstrates:
- Simple quine generation
- Different escape strategies
- Custom formatting options
- Builder pattern usage

Run:
```bash
cargo run --example basic_usage
```

### 2. CLI Examples

#### Generate from text
```bash
echo "console.log('test');" | psychoquine
```

#### One-line output
```bash
psychoquine -o "function hello() { return 'world'; }"
```

#### Multi-line with Unicode escaping
```bash
psychoquine -e unicode -m "Hello 世界"
```

#### With statistics
```bash
cat source.js | psychoquine -s
```

### 3. Library Integration

```rust
use psychoquine_core::generate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = generate("your code here")?;
    println!("{}", output.one_line);
    Ok(())
}
```

## Use Cases

### Code Obfuscation

Transform readable code into quine format:

```bash
cat readable.js | psychoquine -o > obfuscated.js
node obfuscated.js  # Prints itself
```

### Meta-Programming

Create self-replicating programs:

```rust
let code = "console.log('I am a quine');";
let quine = generate(code)?;
// quine.one_line is now executable and self-replicating
```

### Educational Purposes

Demonstrate self-reference and code representation:

```bash
# Generate a quine
psychoquine "alert('hello');" > quine.js

# Execute it
node quine.js > output.js

# Verify they're identical
diff quine.js output.js
```

### Data Embedding

Embed data in self-extracting code:

```rust
let data = serde_json::to_string(&my_data)?;
let quine = generate(&data)?;
// Now quine contains the data in executable form
```

## Advanced Usage

### Custom Escape Strategy

```rust
use psychoquine_core::{EscapeStrategy, FormatOptions, QuineGenerator};

let options = FormatOptions::default()
    .with_escape_strategy(EscapeStrategy::Unicode);

let generator = QuineGenerator::with_options(options);
let output = generator.generate("input")?;
```

### Error Handling

```rust
use psychoquine_core::{generate, QuineError};

match generate("") {
    Ok(output) => println!("{}", output.one_line),
    Err(QuineError::EmptyInput) => eprintln!("Input cannot be empty"),
    Err(QuineError::InputTooLarge { max, actual }) => {
        eprintln!("Input too large: {} bytes (max: {})", actual, max)
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Size Limits

```rust
let generator = QuineGenerator::new()
    .with_max_input_size(1024 * 1024); // 1 MB

let result = generator.generate(large_input)?;
```

## Tips

1. **Test Your Quines**: Always test that generated quines are executable
   ```bash
   psychoquine "code" > quine.js && node quine.js
   ```

2. **Check Output Size**: Use `-s` flag to see expansion ratios
   ```bash
   psychoquine -s "input"
   ```

3. **Compare Formats**: Generate both formats to choose the best one
   ```bash
   psychoquine -b "input"
   ```

4. **Pipe to Tools**: Chain with other tools
   ```bash
   cat input.js | psychoquine -q -o | pbcopy
   ```

## Contributing Examples

Have a cool use case? Contribute an example:

1. Create a new `.rs` file in this directory
2. Add documentation at the top
3. Submit a pull request

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.
