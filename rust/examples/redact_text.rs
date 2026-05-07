//! Permanently redact text from a PDF.
//!
//! Usage:
//!   cargo run --example redact_text -- input.pdf output.pdf "search term"
//!   cargo run --example redact_text -- input.pdf output.pdf "PATTERN" --regex

use pdfluent::prelude::*;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: redact_text input.pdf output.pdf TEXT [--regex]");
        std::process::exit(1);
    }

    let input = &args[1];
    let output = &args[2];
    let pattern = &args[3];
    let use_regex = args.iter().any(|a| a == "--regex");

    let mut doc = PdfDocument::open(input)?;

    println!("Redacting {:?} from {input} (regex: {use_regex})", pattern);

    let opts = RedactOptions::new().regex(use_regex);
    doc.redact(pattern, opts)?;

    doc.save(output)?;
    println!("Saved redacted PDF to: {output}");

    Ok(())
}
