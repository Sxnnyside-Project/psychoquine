//! Core quine generation logic
//!
//! Transforms arbitrary input text into self-replicating program structures.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::escape::EscapeStrategy;
use crate::formatter::{FormatOptions, Formatter, OutputFormat};

/// Errors that can occur during quine generation
#[derive(Error, Debug)]
pub enum QuineError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Input exceeds maximum size of {max} bytes (got {actual})")]
    InputTooLarge { max: usize, actual: usize },
    #[error("Input contains invalid UTF-8 sequences")]
    InvalidUtf8,
    #[error("Formatting error: {0}")]
    FormattingError(String),
}

/// The output of quine generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuineOutput {
    /// The original input
    pub original: String,
    /// One-line quine representation
    pub one_line: String,
    /// Multi-line formatted quine representation
    pub multi_line: String,
    /// The escape strategy used
    pub escape_strategy: EscapeStrategy,
    /// Statistics about the generation
    pub stats: QuineStats,
}

/// Statistics about the quine generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuineStats {
    /// Original input size in bytes
    pub input_bytes: usize,
    /// One-line output size in bytes
    pub one_line_bytes: usize,
    /// Multi-line output size in bytes
    pub multi_line_bytes: usize,
    /// Expansion ratio (one-line output / input)
    pub expansion_ratio: f64,
}

/// The main quine generator
pub struct QuineGenerator {
    options: FormatOptions,
    max_input_size: usize,
}

impl Default for QuineGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl QuineGenerator {
    /// Maximum input size (10 MB)
    const DEFAULT_MAX_INPUT_SIZE: usize = 10 * 1024 * 1024;

    /// Create a new generator with default options
    pub fn new() -> Self {
        Self {
            options: FormatOptions::default(),
            max_input_size: Self::DEFAULT_MAX_INPUT_SIZE,
        }
    }

    /// Create a generator with custom options
    pub fn with_options(options: FormatOptions) -> Self {
        Self {
            options,
            max_input_size: Self::DEFAULT_MAX_INPUT_SIZE,
        }
    }

    /// Set the maximum input size
    pub fn with_max_input_size(mut self, size: usize) -> Self {
        self.max_input_size = size;
        self
    }

    /// Generate a quine from the input
    pub fn generate(&self, input: &str) -> Result<QuineOutput, QuineError> {
        // Validation
        if input.is_empty() {
            return Err(QuineError::EmptyInput);
        }

        if input.len() > self.max_input_size {
            return Err(QuineError::InputTooLarge {
                max: self.max_input_size,
                actual: input.len(),
            });
        }

        // Escape the input
        let escaped = self.options.escape_strategy.escape(input);

        // Create formatter
        let formatter = Formatter::new(self.options.clone());

        // Generate both formats
        let one_line = formatter.format_one_line(input, &escaped);
        let multi_line = formatter.format_multi_line(input, &escaped);

        // Calculate stats
        let stats = QuineStats {
            input_bytes: input.len(),
            one_line_bytes: one_line.len(),
            multi_line_bytes: multi_line.len(),
            expansion_ratio: one_line.len() as f64 / input.len() as f64,
        };

        Ok(QuineOutput {
            original: input.to_string(),
            one_line,
            multi_line,
            escape_strategy: self.options.escape_strategy,
            stats,
        })
    }

    /// Generate only one-line output
    pub fn generate_one_line(&self, input: &str) -> Result<String, QuineError> {
        let output = self.generate(input)?;
        Ok(output.one_line)
    }

    /// Generate only multi-line output
    pub fn generate_multi_line(&self, input: &str) -> Result<String, QuineError> {
        let output = self.generate(input)?;
        Ok(output.multi_line)
    }
}

/// Builder pattern for QuineGenerator
pub struct QuineGeneratorBuilder {
    options: FormatOptions,
    max_input_size: usize,
}

impl QuineGeneratorBuilder {
    pub fn new() -> Self {
        Self {
            options: FormatOptions::default(),
            max_input_size: QuineGenerator::DEFAULT_MAX_INPUT_SIZE,
        }
    }

    pub fn escape_strategy(mut self, strategy: EscapeStrategy) -> Self {
        self.options.escape_strategy = strategy;
        self
    }

    pub fn indent<S: Into<String>>(mut self, indent: S) -> Self {
        self.options.indent = indent.into();
        self
    }

    pub fn max_line_length(mut self, length: usize) -> Self {
        self.options.max_line_length = length;
        self
    }

    pub fn max_input_size(mut self, size: usize) -> Self {
        self.max_input_size = size;
        self
    }

    pub fn output_format(mut self, format: OutputFormat) -> Self {
        self.options.output_format = format;
        self
    }

    pub fn build(self) -> QuineGenerator {
        QuineGenerator {
            options: self.options,
            max_input_size: self.max_input_size,
        }
    }
}

impl Default for QuineGeneratorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_generation() {
        let generator = QuineGenerator::new();
        let result = generator.generate("hello world");
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.one_line.is_empty());
        assert!(!output.multi_line.is_empty());
        assert_eq!(output.original, "hello world");
    }

    #[test]
    fn test_empty_input_error() {
        let generator = QuineGenerator::new();
        let result = generator.generate("");
        assert!(matches!(result, Err(QuineError::EmptyInput)));
    }

    #[test]
    fn test_input_too_large() {
        let generator = QuineGenerator::new().with_max_input_size(10);
        let result = generator.generate("this is a very long input string");
        assert!(matches!(result, Err(QuineError::InputTooLarge { .. })));
    }

    #[test]
    fn test_special_characters() {
        let generator = QuineGenerator::new();
        let result = generator.generate("hello\nworld\t\"test\"");
        assert!(result.is_ok());

        let output = result.unwrap();
        // Escaped characters should be present
        assert!(output.one_line.contains("\\n") || output.multi_line.contains("\\n"));
    }

    #[test]
    fn test_builder_pattern() {
        let generator = QuineGeneratorBuilder::new()
            .escape_strategy(EscapeStrategy::Unicode)
            .indent("  ")
            .max_line_length(120)
            .build();

        let result = generator.generate("test");
        assert!(result.is_ok());
    }
}
