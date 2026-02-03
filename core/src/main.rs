//! PsychoQuine CLI
//!
//! Command-line interface for the PsychoQuine quine generator.

use std::io::{self, Read, Write};
use std::process::ExitCode;

use psychoquine_core::{generate, EscapeStrategy, FormatOptions, QuineGenerator};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BANNER: &str = r#"
╔═══════════════════════════════════════════════════════════════╗
║  ██████╗ ███████╗██╗   ██╗ ██████╗██╗  ██╗ ██████╗            ║
║  ██╔══██╗██╔════╝╚██╗ ██╔╝██╔════╝██║  ██║██╔═══██╗           ║
║  ██████╔╝███████╗ ╚████╔╝ ██║     ███████║██║   ██║           ║
║  ██╔═══╝ ╚════██║  ╚██╔╝  ██║     ██╔══██║██║   ██║           ║
║  ██║     ███████║   ██║   ╚██████╗██║  ██║╚██████╔╝           ║
║  ╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝ ╚═════╝            ║
║   ██████╗ ██╗   ██╗██╗███╗   ██╗███████╗                      ║
║  ██╔═══██╗██║   ██║██║████╗  ██║██╔════╝                      ║
║  ██║   ██║██║   ██║██║██╔██╗ ██║█████╗                        ║
║  ██║▄▄ ██║██║   ██║██║██║╚██╗██║██╔══╝                        ║
║  ╚██████╔╝╚██████╔╝██║██║ ╚████║███████╗                      ║
║   ╚══▀▀═╝  ╚═════╝ ╚═╝╚═╝  ╚═══╝╚══════╝                      ║
║                                                               ║
║  Universal Quine Generator                    CoreRed Project ║
╚═══════════════════════════════════════════════════════════════╝
"#;

fn print_help() {
    eprintln!("{}", BANNER);
    eprintln!("PsychoQuine v{}", VERSION);
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    psychoquine [OPTIONS] [INPUT]");
    eprintln!("    echo \"text\" | psychoquine [OPTIONS]");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("    -h, --help          Show this help message");
    eprintln!("    -v, --version       Show version information");
    eprintln!("    -o, --one-line      Output only one-line quine");
    eprintln!("    -m, --multi-line    Output only multi-line quine");
    eprintln!("    -b, --both          Output both formats (default)");
    eprintln!("    -e, --escape TYPE   Escape strategy: standard, unicode, hex, raw");
    eprintln!("    -s, --stats         Show generation statistics");
    eprintln!("    -q, --quiet         Suppress banner and decorations");
    eprintln!();
    eprintln!("EXAMPLES:");
    eprintln!("    psychoquine \"Hello, World!\"");
    eprintln!("    cat source.js | psychoquine -o");
    eprintln!("    psychoquine -e unicode -m \"test input\"");
}

fn print_version() {
    eprintln!("PsychoQuine {}", VERSION);
}

#[derive(Default)]
struct Args {
    help: bool,
    version: bool,
    one_line: bool,
    multi_line: bool,
    show_stats: bool,
    quiet: bool,
    escape: Option<String>,
    input: Option<String>,
}

fn parse_args() -> Args {
    let mut args = Args::default();
    let mut argv: Vec<String> = std::env::args().skip(1).collect();

    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "-h" | "--help" => args.help = true,
            "-v" | "--version" => args.version = true,
            "-o" | "--one-line" => args.one_line = true,
            "-m" | "--multi-line" => args.multi_line = true,
            "-b" | "--both" => {
                args.one_line = false;
                args.multi_line = false;
            }
            "-s" | "--stats" => args.show_stats = true,
            "-q" | "--quiet" => args.quiet = true,
            "-e" | "--escape" => {
                i += 1;
                if i < argv.len() {
                    args.escape = Some(argv[i].clone());
                }
            }
            arg if !arg.starts_with('-') => {
                args.input = Some(arg.to_string());
            }
            _ => {
                eprintln!("Unknown option: {}", argv[i]);
            }
        }
        i += 1;
    }

    args
}

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn main() -> ExitCode {
    let args = parse_args();

    if args.help {
        print_help();
        return ExitCode::SUCCESS;
    }

    if args.version {
        print_version();
        return ExitCode::SUCCESS;
    }

    // Get input from argument or stdin
    let input = match args.input {
        Some(text) => text,
        None => {
            // Check if stdin has data
            if atty::is(atty::Stream::Stdin) {
                print_help();
                return ExitCode::FAILURE;
            }
            match read_stdin() {
                Ok(text) => text,
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    return ExitCode::FAILURE;
                }
            }
        }
    };

    // Parse escape strategy
    let escape_strategy = match args.escape.as_deref() {
        Some("unicode") => EscapeStrategy::Unicode,
        Some("hex") => EscapeStrategy::Hexadecimal,
        Some("raw") => EscapeStrategy::Raw,
        Some("standard") | None => EscapeStrategy::Standard,
        Some(other) => {
            eprintln!("Unknown escape strategy: {}", other);
            return ExitCode::FAILURE;
        }
    };

    // Build generator
    let options = FormatOptions::default().with_escape_strategy(escape_strategy);
    let generator = QuineGenerator::with_options(options);

    // Generate quine
    let result = match generator.generate(&input) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Generation error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // Print output
    if !args.quiet {
        eprintln!("{}", BANNER);
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    if args.one_line && !args.multi_line {
        if !args.quiet {
            eprintln!("═══ ONE-LINE QUINE ═══");
        }
        writeln!(handle, "{}", result.one_line).ok();
    } else if args.multi_line && !args.one_line {
        if !args.quiet {
            eprintln!("═══ MULTI-LINE QUINE ═══");
        }
        writeln!(handle, "{}", result.multi_line).ok();
    } else {
        if !args.quiet {
            eprintln!("═══ ONE-LINE QUINE ═══");
        }
        writeln!(handle, "{}", result.one_line).ok();
        if !args.quiet {
            eprintln!();
            eprintln!("═══ MULTI-LINE QUINE ═══");
        }
        writeln!(handle, "{}", result.multi_line).ok();
    }

    // Show stats if requested
    if args.show_stats {
        eprintln!();
        eprintln!("═══ STATISTICS ═══");
        eprintln!("Input size:       {} bytes", result.stats.input_bytes);
        eprintln!("One-line size:    {} bytes", result.stats.one_line_bytes);
        eprintln!("Multi-line size:  {} bytes", result.stats.multi_line_bytes);
        eprintln!("Expansion ratio:  {:.2}x", result.stats.expansion_ratio);
    }

    ExitCode::SUCCESS
}
