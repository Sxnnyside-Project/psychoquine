//! PsychoQuine Core - Universal Quine Generator
//!
//! A resource-agnostic quine generation engine capable of transforming
//! arbitrary textual resources into self-replicating program structures.

mod escape;
mod formatter;
mod generator;

pub use escape::EscapeStrategy;
pub use formatter::{FormatOptions, OutputFormat};
pub use generator::{QuineGenerator, QuineOutput, QuineError};

/// Re-export of the main generation function for convenience
pub fn generate<S: AsRef<str>>(input: S) -> Result<QuineOutput, QuineError> {
    QuineGenerator::new().generate(input.as_ref())
}

/// Generate with custom options
pub fn generate_with_options<S: AsRef<str>>(
    input: S,
    options: FormatOptions,
) -> Result<QuineOutput, QuineError> {
    QuineGenerator::with_options(options).generate(input.as_ref())
}
