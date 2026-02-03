# PsychoQuine User Guide

Welcome to PsychoQuine, the universal quine generator. This guide will help you understand what PsychoQuine does and how to use it effectively.

## Table of Contents

1. [What is PsychoQuine?](#what-is-psychoquine)
2. [What is a Quine?](#what-is-a-quine)
3. [Installation](#installation)
4. [Using the Desktop App](#using-the-desktop-app)
5. [Using the CLI](#using-the-cli)
6. [Understanding Output](#understanding-output)
7. [Escape Strategies](#escape-strategies)
8. [Use Cases](#use-cases)
9. [Limitations](#limitations)
10. [FAQ](#faq)

---

## What is PsychoQuine?

PsychoQuine is a universal, resource-agnostic quine generator. It takes any textual input—code, configuration files, documentation, or plain text—and transforms it into a self-replicating program structure.

**Key Features:**
- Accept any text as input
- Generate two output formats: one-line and multi-line
- Multiple escape strategies for different use cases
- Zero configuration required
- Cross-platform desktop app and CLI

**Philosophy:**

PsychoQuine is a CoreRed experimental tool built for developers who appreciate the art of meta-programming. It's minimal, precise, and intimidating—just like good tools should be.

---

## What is a Quine?

A **quine** is a program that produces a copy of its own source code as its only output.

### Classic Example

```javascript
(function(){var s='(function(){var s=%s;console.log(s,JSON.stringify(s))})()';console.log(s,JSON.stringify(s))})()
```

When executed, this program prints itself exactly.

### Why Quines Matter

Quines are:
- **Educational**: Teach fundamental concepts about code representation
- **Mind-bending**: Challenge how we think about programs and data
- **Practical**: Used in security, obfuscation, and code generation

PsychoQuine makes it trivial to create quines from any input.

---

## Installation

### Desktop App

**Download:**
1. Visit the [Releases page](https://github.com/Sxnnyside-Project/psychoquine/releases)
2. Download the installer for your platform:
   - **macOS**: `.dmg` file
   - **Windows**: `.msi` installer
   - **Linux**: `.deb` or `.AppImage`
3. Install and run

**Build from Source:**
```bash
git clone https://github.com/Sxnnyside-Project/psychoquine.git
cd psychoquine/src-tauri
cargo tauri build
```

### CLI

**Via Cargo:**
```bash
cargo install psychoquine-core
```

**From Source:**
```bash
git clone https://github.com/Sxnnyside-Project/psychoquine.git
cd psychoquine/core
cargo install --path .
```

---

## Using the Desktop App

### Interface Overview

The PsychoQuine desktop app has a clean, terminal-inspired interface:

```
┌─────────────────────────────────────────────────────────┐
│  PSYCHOQUINE                                    v0.1.0  │
│  Universal Quine Generator                    CoreRed   │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌───────────────────┐     ┌───────────────────┐      │
│  │   INPUT           │     │   OUTPUT          │      │
│  │                   │     │                   │      │
│  │  [text area]      │     │  [one-line]       │      │
│  │                   │     │  [multi-line]     │      │
│  │                   │     │                   │      │
│  └───────────────────┘     └───────────────────┘      │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Step-by-Step Usage

1. **Enter Input**
   - Type or paste your text in the INPUT panel
   - Any text format is supported

2. **Select Escape Strategy** (optional)
   - Standard (default)
   - Unicode
   - Hexadecimal
   - Raw

3. **Generate Quine**
   - Click "GENERATE QUINE" button
   - Both formats are generated instantly

4. **View Output**
   - Toggle between ONE-LINE and MULTI-LINE tabs
   - See statistics: input size, output size, expansion ratio

5. **Copy Output**
   - Click "COPY" to copy the current output to clipboard
   - Button shows "COPIED!" confirmation

### Example Session

**Input:**
```javascript
function hello() {
    console.log("Hello, World!");
}
```

**Output (One-Line):**
```javascript
(function(){var d="function hello() {\n    console.log(\"Hello, World!\");\n}";var q=String.fromCharCode(34);console.log("(function(){var d="+q+d+q+";"+d.slice(d.indexOf("var q")))})()
```

**Output (Multi-Line):**
```javascript
(function() {
    var d = "function hello() {\n    console.log(\"Hello, World!\");\n}";
    var q = String.fromCharCode(34);
    console.log("(function(){var d=" + q + d + q + ";" + d.slice(d.indexOf("var q")));
})()
```

---

## Using the CLI

### Basic Usage

```bash
# Direct input
psychoquine "Hello, World!"

# From stdin
echo "Hello, World!" | psychoquine

# From file
cat myfile.txt | psychoquine
```

### Options

```bash
-h, --help          Show help message
-v, --version       Show version information
-o, --one-line      Output only one-line quine
-m, --multi-line    Output only multi-line quine
-b, --both          Output both formats (default)
-e, --escape TYPE   Escape strategy: standard, unicode, hex, raw
-s, --stats         Show generation statistics
-q, --quiet         Suppress banner and decorations
```

### Examples

**One-line output only:**
```bash
psychoquine -o "console.log('test')"
```

**Multi-line with Unicode escaping:**
```bash
psychoquine -e unicode -m "Special chars: 你好"
```

**With statistics:**
```bash
echo "test input" | psychoquine -s
```

**Quiet mode (for scripting):**
```bash
cat source.js | psychoquine -q -o > output.js
```

### Piping and Redirection

```bash
# Save to file
psychoquine "code" > quine.js

# Chain with other tools
cat input.js | psychoquine -o | pbcopy  # macOS clipboard
cat input.js | psychoquine -o | xclip   # Linux clipboard
```

---

## Understanding Output

### One-Line Format

- **Purpose**: Compact representation
- **Use case**: Embedding in URLs, single-line contexts
- **Characteristics**: No line breaks, minimal whitespace

### Multi-Line Format

- **Purpose**: Readable representation
- **Use case**: Code review, debugging, human inspection
- **Characteristics**: Indented, line breaks preserved

### Statistics

PsychoQuine provides metadata about the generation:

- **Input Bytes**: Original input size
- **Output Bytes**: Generated quine size
- **Expansion Ratio**: Output size / Input size

Typical expansion ratios: 2x to 5x depending on input complexity.

---

## Escape Strategies

Choose the right escape strategy for your use case:

### Standard (Default)

Uses common escape sequences:
- `\n` for newline
- `\t` for tab
- `\"` for quote
- `\\` for backslash

**Best for**: Most use cases, JavaScript, JSON

### Unicode

Uses Unicode escapes: `\u{XXXX}`

**Best for**: International text, special characters

**Example:**
```
Input:  Hello 世界
Output: Hello \u{4e16}\u{754c}
```

### Hexadecimal

Uses hex escapes: `\xXX`

**Best for**: Binary data, low-level processing

**Example:**
```
Input:  ABC
Output: ABC (printable ASCII unchanged)
```

### Raw

Minimal escaping, preserves most characters as-is.

**Best for**: Already-escaped content, testing

---

## Use Cases

### 1. Code Obfuscation

Transform readable code into self-replicating form:

```bash
cat secret.js | psychoquine -o > obfuscated.js
```

### 2. Meta-Programming Education

Learn how programs represent themselves:

```javascript
// Original
console.log("Hello");

// Quine version demonstrates self-reference
```

### 3. Code Golf

Create compact self-replicating programs for competitions.

### 4. Security Research

Study code injection, escaping, and execution contexts.

### 5. Data Embedding

Embed data structures within self-extracting code.

### 6. Digital Art

Create artistic quines with ASCII art or poetry.

---

## Limitations

### What PsychoQuine Does NOT Do

1. **Execution**: PsychoQuine generates quines; it doesn't execute them
2. **Language-Specific**: Output is JavaScript-based (universal pattern)
3. **Semantic Analysis**: No understanding of code meaning
4. **Optimization**: No code minification or optimization

### Input Constraints

- **Maximum Size**: 10 MB by default (configurable)
- **Text Only**: Binary files must be converted first
- **UTF-8**: Input must be valid UTF-8

### Output Characteristics

- **JavaScript Syntax**: Output uses JavaScript quine pattern
- **Deterministic**: Same input always produces same output
- **No Side Effects**: Generated quines only print themselves

---

## FAQ

### Q: Will the generated quine work in my language?

The quine uses JavaScript syntax. For other languages, the pattern can be adapted, but PsychoQuine generates JavaScript by default.

### Q: Why is the output larger than the input?

Quines must contain both the data AND the code to print it, resulting in expansion. Typical ratio is 2-5x.

### Q: Can I customize the quine template?

Not in the current version. The template is optimized for universality. Consider forking for custom templates.

### Q: Is the output really a quine?

Yes. Execute it in a JavaScript environment (Node.js, browser console) and it will print itself.

### Q: What's the "CoreRed" mention?

CoreRed is an experimental project umbrella for hacker-focused developer tools.

### Q: Can I use this in production?

PsychoQuine is an experimental tool. Use at your own discretion. The license is MIT.

### Q: Does it support other languages besides JavaScript?

The output format is JavaScript-based. The quine pattern can be adapted to other languages manually.

### Q: How do I report bugs?

Open an issue on [GitHub](https://github.com/Sxnnyside-Project/psychoquine/issues).

---

## Tips and Tricks

### Tip 1: Preview Before Copying

Always review the multi-line output for readability before using the one-line version.

### Tip 2: Test Your Quines

```bash
# Generate
psychoquine -o "test" > quine.js

# Execute
node quine.js

# It should print itself
```

### Tip 3: Batch Processing

```bash
for file in *.js; do
    psychoquine < "$file" > "quine_$file"
done
```

### Tip 4: Use Quiet Mode for Scripts

```bash
#!/bin/bash
OUTPUT=$(psychoquine -q -o "input")
echo $OUTPUT
```

---

## Getting Help

- **Documentation**: [GitHub Repository](https://github.com/Sxnnyside-Project/psychoquine)
- **Issues**: [Issue Tracker](https://github.com/Sxnnyside-Project/psychoquine/issues)
- **Contributing**: See [CONTRIBUTING.md](../CONTRIBUTING.md)

---

**Happy Quine Generation!**

*— The PsychoQuine Team*
