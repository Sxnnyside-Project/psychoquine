# Security Policy

## Supported Versions

PsychoQuine is currently in active development. Security updates will be provided for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability in PsychoQuine, please report it responsibly.

### How to Report

**DO NOT** open a public issue for security vulnerabilities.

Instead, please email security concerns to:

**security.sxnnyside@sxnnysideproject.com**

Or open a private security advisory on GitHub:
https://github.com/Sxnnyside-Project/psychoquine/security/advisories

### What to Include

When reporting a vulnerability, please include:

1. **Description**: Clear description of the vulnerability
2. **Impact**: Potential impact and affected versions
3. **Reproduction**: Step-by-step instructions to reproduce
4. **Proof of Concept**: Code or example demonstrating the issue
5. **Suggested Fix**: If you have ideas on how to fix it

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depending on severity, typically within 30 days

### Disclosure Policy

- Please allow us time to fix the vulnerability before public disclosure
- We will credit you in the security advisory (unless you prefer to remain anonymous)
- We may request your help in validating the fix

## Security Considerations

### Input Handling

PsychoQuine processes arbitrary text input. While the core engine is designed to handle this safely, be aware:

- Maximum input size is enforced (10 MB default)
- All input is validated as UTF-8
- No code execution occurs during generation
- Output is purely textual

### Desktop Application

The Tauri desktop application has restricted permissions:

- Clipboard write access (for copy functionality)
- Shell open (for external links)
- No filesystem access beyond application data
- No network access

### Dependencies

We regularly update dependencies to address known vulnerabilities:

- Rust dependencies: Updated via `cargo update`
- Deno dependencies: Pinned in `deno.json`
- Tauri: Following official security updates

## Known Security Limitations

### 1. Output Execution

Generated quines are executable JavaScript code. **Users must exercise caution** when executing generated output, especially from untrusted input sources.

### 2. Code Injection

PsychoQuine does not execute or validate input as code. It performs text transformation only. Users should not:

- Execute quines generated from untrusted sources without review
- Use PsychoQuine to process malicious input

### 3. Resource Exhaustion

While input size is limited, extremely complex inputs with many special characters may result in large outputs. This is expected behavior but could consume significant memory.

## Best Practices

### For Users

1. **Review Output**: Always review generated quines before execution
2. **Trusted Sources**: Only process input from trusted sources
3. **Update Regularly**: Keep PsychoQuine updated to the latest version
4. **Sandboxed Execution**: Execute generated quines in isolated environments

### For Developers

1. **Input Validation**: Always validate input before passing to PsychoQuine
2. **Size Limits**: Use appropriate max input size for your use case
3. **Error Handling**: Handle all errors returned by the API
4. **Update Dependencies**: Regularly update PsychoQuine and its dependencies

## Threat Model

### In Scope

- Input processing vulnerabilities
- Memory safety issues
- IPC security (Tauri)
- Dependency vulnerabilities
- Output integrity

### Out of Scope

- Execution of generated quines (user responsibility)
- Denial of service via legitimate large inputs
- Social engineering attacks
- Physical access to user's machine

## Security Updates

Security updates will be announced via:

1. GitHub Security Advisories
2. Release notes in CHANGELOG.md
3. GitHub Releases page

Critical security updates will be backported to supported versions when feasible.

---

Thank you for helping keep PsychoQuine secure!

*Last updated: 2026-02-03*
