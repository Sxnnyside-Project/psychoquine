//! Output formatting for quine generation
//!
//! Handles the transformation of raw quine data into one-line
//! and multi-line formatted representations.

use serde::{Deserialize, Serialize};
use crate::escape::EscapeStrategy;

/// Output format selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum OutputFormat {
    /// Single line output, no line breaks
    OneLine,
    /// Formatted with line breaks and indentation
    #[default]
    MultiLine,
    /// Both formats
    Both,
}

/// Configuration options for quine formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatOptions {
    /// The escape strategy to use
    pub escape_strategy: EscapeStrategy,
    /// Indentation string for multi-line output
    pub indent: String,
    /// Maximum line length for multi-line output (0 = no limit)
    pub max_line_length: usize,
    /// Whether to include a trailing newline
    pub trailing_newline: bool,
    /// Output format selection
    pub output_format: OutputFormat,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            escape_strategy: EscapeStrategy::Standard,
            indent: String::from("    "),
            max_line_length: 80,
            trailing_newline: true,
            output_format: OutputFormat::Both,
        }
    }
}

impl FormatOptions {
    /// Create options for one-line output
    pub fn one_line() -> Self {
        Self {
            output_format: OutputFormat::OneLine,
            ..Default::default()
        }
    }

    /// Create options for multi-line output
    pub fn multi_line() -> Self {
        Self {
            output_format: OutputFormat::MultiLine,
            ..Default::default()
        }
    }

    /// Set the escape strategy
    pub fn with_escape_strategy(mut self, strategy: EscapeStrategy) -> Self {
        self.escape_strategy = strategy;
        self
    }

    /// Set the indentation string
    pub fn with_indent<S: Into<String>>(mut self, indent: S) -> Self {
        self.indent = indent.into();
        self
    }

    /// Set the maximum line length
    pub fn with_max_line_length(mut self, length: usize) -> Self {
        self.max_line_length = length;
        self
    }
}

/// Formatter for quine output
pub struct Formatter {
    options: FormatOptions,
}

impl Formatter {
    /// Create a new formatter with the given options
    pub fn new(options: FormatOptions) -> Self {
        Self { options }
    }

    /// Format the quine data as a single line
    pub fn format_one_line(&self, _data: &str, escaped: &str) -> String {
        let template = self.generate_quine_template(escaped);
        template.replace('\n', "").replace("  ", " ")
    }

    /// Format the quine data with line breaks and indentation
    pub fn format_multi_line(&self, _data: &str, escaped: &str) -> String {
        let template = self.generate_quine_template(escaped);
        self.apply_formatting(&template)
    }

    /// Generate the core quine template
    fn generate_quine_template(&self, escaped_data: &str) -> String {
        // Universal quine structure that works across languages
        // The pattern: define data, then use data to print both code and data
        format!(
            r#"(function(){{var d="{}";var q=String.fromCharCode(34);console.log("(function(){{var d="+q+d+q+";"+d.slice(d.indexOf("var q")))}})()"#,
            escaped_data
        )
    }

    fn apply_formatting(&self, code: &str) -> String {
        let mut result = String::new();
        let mut depth = 0;
        let mut in_string = false;
        let mut escape_next = false;

        for ch in code.chars() {
            if escape_next {
                result.push(ch);
                escape_next = false;
                continue;
            }

            match ch {
                '\\' if in_string => {
                    result.push(ch);
                    escape_next = true;
                }
                '"' => {
                    result.push(ch);
                    in_string = !in_string;
                }
                '{' if !in_string => {
                    depth += 1;
                    result.push(ch);
                    result.push('\n');
                    result.push_str(&self.options.indent.repeat(depth));
                }
                '}' if !in_string => {
                    depth = depth.saturating_sub(1);
                    result.push('\n');
                    result.push_str(&self.options.indent.repeat(depth));
                    result.push(ch);
                }
                ';' if !in_string => {
                    result.push(ch);
                    result.push('\n');
                    result.push_str(&self.options.indent.repeat(depth));
                }
                _ => result.push(ch),
            }
        }

        if self.options.trailing_newline && !result.ends_with('\n') {
            result.push('\n');
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let options = FormatOptions::default();
        assert_eq!(options.indent, "    ");
        assert_eq!(options.max_line_length, 80);
        assert!(options.trailing_newline);
    }

    #[test]
    fn test_formatter_one_line() {
        let formatter = Formatter::new(FormatOptions::one_line());
        let result = formatter.format_one_line("test", "test");
        assert!(!result.contains('\n') || result.ends_with('\n') && result.matches('\n').count() == 1);
    }
}
