//! Escape handling for quine generation
//!
//! Provides strategies for escaping special characters in various contexts.

use serde::{Deserialize, Serialize};

/// Strategy for escaping special characters in quine output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EscapeStrategy {
    /// Standard escape sequences (\\n, \\t, \\", etc.)
    #[default]
    Standard,
    /// Unicode escape sequences (\\u{XXXX})
    Unicode,
    /// Hexadecimal escape sequences (\\xXX)
    Hexadecimal,
    /// Raw string literals where possible
    Raw,
}

impl EscapeStrategy {
    /// Escape a string according to the selected strategy
    pub fn escape(&self, input: &str) -> String {
        match self {
            EscapeStrategy::Standard => Self::escape_standard(input),
            EscapeStrategy::Unicode => Self::escape_unicode(input),
            EscapeStrategy::Hexadecimal => Self::escape_hex(input),
            EscapeStrategy::Raw => Self::escape_raw(input),
        }
    }

    fn escape_standard(input: &str) -> String {
        let mut output = String::with_capacity(input.len() * 2);
        for ch in input.chars() {
            match ch {
                '\\' => output.push_str("\\\\"),
                '"' => output.push_str("\\\""),
                '\'' => output.push_str("\\'"),
                '\n' => output.push_str("\\n"),
                '\r' => output.push_str("\\r"),
                '\t' => output.push_str("\\t"),
                '\0' => output.push_str("\\0"),
                c if c.is_control() => {
                    output.push_str(&format!("\\x{:02x}", c as u32));
                }
                c => output.push(c),
            }
        }
        output
    }

    fn escape_unicode(input: &str) -> String {
        let mut output = String::with_capacity(input.len() * 6);
        for ch in input.chars() {
            match ch {
                '\\' => output.push_str("\\\\"),
                '"' => output.push_str("\\\""),
                c if c.is_ascii_graphic() || c == ' ' => output.push(c),
                c => output.push_str(&format!("\\u{{{:04x}}}", c as u32)),
            }
        }
        output
    }

    fn escape_hex(input: &str) -> String {
        let mut output = String::with_capacity(input.len() * 4);
        for ch in input.chars() {
            match ch {
                '\\' => output.push_str("\\\\"),
                '"' => output.push_str("\\\""),
                c if c.is_ascii_graphic() || c == ' ' => output.push(c),
                c if c as u32 <= 0xFF => {
                    output.push_str(&format!("\\x{:02x}", c as u32));
                }
                c => output.push_str(&format!("\\u{{{:04x}}}", c as u32)),
            }
        }
        output
    }

    fn escape_raw(input: &str) -> String {
        // For raw strings, we only need to handle the delimiter
        // In this simplified version, we use standard escaping as fallback
        Self::escape_standard(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_escape() {
        let strategy = EscapeStrategy::Standard;
        assert_eq!(strategy.escape("hello"), "hello");
        assert_eq!(strategy.escape("hello\nworld"), "hello\\nworld");
        assert_eq!(strategy.escape("say \"hi\""), "say \\\"hi\\\"");
        assert_eq!(strategy.escape("back\\slash"), "back\\\\slash");
    }

    #[test]
    fn test_unicode_escape() {
        let strategy = EscapeStrategy::Unicode;
        assert_eq!(strategy.escape("hello"), "hello");
        assert_eq!(strategy.escape("hello\nworld"), "hello\\u{000a}world");
    }

    #[test]
    fn test_hex_escape() {
        let strategy = EscapeStrategy::Hexadecimal;
        assert_eq!(strategy.escape("hello"), "hello");
        assert_eq!(strategy.escape("hello\nworld"), "hello\\x0aworld");
    }
}
