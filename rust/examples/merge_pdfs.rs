//! Merge multiple PDFs into one.
//!
//! Usage:
//!   cargo run --example merge_pdfs -- a.pdf b.pdf c.pdf -o merged.pdf
//!
//! All input PDFs are merged in the order given.

use pdfluent::prelude::*;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // Parse: collect input files and optional -o <output> flag.
    let mut inputs: Vec<String> = Vec::new();
    let mut output = String::from("merged.pdf");
    let mut i = 1;
    while i < args.len() {
        if args[i] == "-o" {
            i += 1;
            if let Some(o) = args.get(i) {
                output = o.clone();
            }
        } else {
            inputs.push(args[i].clone());
        }
        i += 1;
    }

    if inputs.is_empty() {
        eprintln!("Usage: merge_pdfs a.pdf b.pdf [-o merged.pdf]");
        std::process::exit(1);
    }

    let mut merger = PdfMerger::new();

    for path in &inputs {
        let doc = PdfDocument::open(path)?;
        println!("  + {} ({} pages)", path, doc.page_count());
        merger = merger.add(doc);
    }

    let merged = merger.build()?;
    println!("\nMerged: {} pages total", merged.page_count());

    merged.save(&output)?;
    println!("Saved to: {output}");

    Ok(())
}
