// ══════════════════════════════════════════════════════════════════════════════
// PsychoQuine - Basic Usage Example
// ══════════════════════════════════════════════════════════════════════════════

use psychoquine_core::{generate, EscapeStrategy, FormatOptions, QuineGenerator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════════════");
    println!("  PsychoQuine - Basic Usage Example");
    println!("═══════════════════════════════════════════════════════════════\n");

    // ──────────────────────────────────────────────────────────────────────────
    // Example 1: Simple generation
    // ──────────────────────────────────────────────────────────────────────────

    println!("Example 1: Simple generation\n");

    let input = "console.log('Hello, World!');";
    let output = generate(input)?;

    println!("Input: {}", input);
    println!("\nOne-line output:");
    println!("{}", output.one_line);
    println!("\nMulti-line output:");
    println!("{}", output.multi_line);
    println!("\nStatistics:");
    println!("  Input bytes:      {}", output.stats.input_bytes);
    println!("  One-line bytes:   {}", output.stats.one_line_bytes);
    println!("  Multi-line bytes: {}", output.stats.multi_line_bytes);
    println!("  Expansion ratio:  {:.2}x", output.stats.expansion_ratio);

    println!("\n───────────────────────────────────────────────────────────────\n");

    // ──────────────────────────────────────────────────────────────────────────
    // Example 2: Using different escape strategies
    // ──────────────────────────────────────────────────────────────────────────

    println!("Example 2: Escape strategies\n");

    let input_with_special = "Hello\nWorld!\t\"Test\"";

    // Standard escape
    let standard_options = FormatOptions::default()
        .with_escape_strategy(EscapeStrategy::Standard);
    let standard_gen = QuineGenerator::with_options(standard_options);
    let standard_output = standard_gen.generate(input_with_special)?;

    println!("Standard escape:");
    println!("{}\n", standard_output.one_line);

    // Unicode escape
    let unicode_options = FormatOptions::default()
        .with_escape_strategy(EscapeStrategy::Unicode);
    let unicode_gen = QuineGenerator::with_options(unicode_options);
    let unicode_output = unicode_gen.generate(input_with_special)?;

    println!("Unicode escape:");
    println!("{}\n", unicode_output.one_line);

    println!("───────────────────────────────────────────────────────────────\n");

    // ──────────────────────────────────────────────────────────────────────────
    // Example 3: Custom formatting options
    // ──────────────────────────────────────────────────────────────────────────

    println!("Example 3: Custom formatting\n");

    let custom_options = FormatOptions::default()
        .with_indent("  ")
        .with_max_line_length(120);

    let custom_gen = QuineGenerator::with_options(custom_options);
    let custom_output = custom_gen.generate("function test() { return 42; }")?;

    println!("Custom formatted output:");
    println!("{}\n", custom_output.multi_line);

    println!("───────────────────────────────────────────────────────────────\n");

    // ──────────────────────────────────────────────────────────────────────────
    // Example 4: Builder pattern
    // ──────────────────────────────────────────────────────────────────────────

    println!("Example 4: Builder pattern\n");

    use psychoquine_core::QuineGeneratorBuilder;

    let generator = QuineGeneratorBuilder::new()
        .escape_strategy(EscapeStrategy::Hexadecimal)
        .indent("    ")
        .max_line_length(100)
        .max_input_size(1024 * 1024)
        .build();

    let result = generator.generate("const x = 10;")?;

    println!("Built with custom configuration:");
    println!("{}\n", result.multi_line);

    println!("═══════════════════════════════════════════════════════════════");

    Ok(())
}
