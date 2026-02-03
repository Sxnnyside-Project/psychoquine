//! Tauri commands for PsychoQuine
//!
//! Exposes the core quine generation functionality to the frontend.

use psychoquine_core::{
    EscapeStrategy, FormatOptions, QuineError, QuineGenerator, QuineOutput,
};
use serde::{Deserialize, Serialize};

/// Options passed from the frontend
#[derive(Debug, Deserialize)]
pub struct GenerateOptions {
    pub escape_strategy: Option<String>,
    pub indent: Option<String>,
    pub max_line_length: Option<usize>,
}

/// Response structure for the frontend
#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub success: bool,
    pub data: Option<QuineOutput>,
    pub error: Option<String>,
}

impl From<Result<QuineOutput, QuineError>> for GenerateResponse {
    fn from(result: Result<QuineOutput, QuineError>) -> Self {
        match result {
            Ok(output) => GenerateResponse {
                success: true,
                data: Some(output),
                error: None,
            },
            Err(e) => GenerateResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
            },
        }
    }
}

/// Parse escape strategy from string
fn parse_escape_strategy(s: &str) -> EscapeStrategy {
    match s.to_lowercase().as_str() {
        "unicode" => EscapeStrategy::Unicode,
        "hex" | "hexadecimal" => EscapeStrategy::Hexadecimal,
        "raw" => EscapeStrategy::Raw,
        _ => EscapeStrategy::Standard,
    }
}

/// Generate a quine from the input
#[tauri::command]
pub fn generate_quine(input: String, options: Option<GenerateOptions>) -> GenerateResponse {
    let mut format_options = FormatOptions::default();

    if let Some(opts) = options {
        if let Some(strategy) = opts.escape_strategy {
            format_options.escape_strategy = parse_escape_strategy(&strategy);
        }
        if let Some(indent) = opts.indent {
            format_options.indent = indent;
        }
        if let Some(max_len) = opts.max_line_length {
            format_options.max_line_length = max_len;
        }
    }

    let generator = QuineGenerator::with_options(format_options);
    generator.generate(&input).into()
}

/// Get application version
#[tauri::command]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_quine_command() {
        let response = generate_quine("test".to_string(), None);
        assert!(response.success);
        assert!(response.data.is_some());
    }

    #[test]
    fn test_generate_with_options() {
        let options = GenerateOptions {
            escape_strategy: Some("unicode".to_string()),
            indent: Some("  ".to_string()),
            max_line_length: Some(120),
        };
        let response = generate_quine("test".to_string(), Some(options));
        assert!(response.success);
    }

    #[test]
    fn test_empty_input_error() {
        let response = generate_quine("".to_string(), None);
        assert!(!response.success);
        assert!(response.error.is_some());
    }
}
