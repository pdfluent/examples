//! Extract text from a PDF — all pages combined, then per-page.
//!
//! Usage:
//!   cargo run --example text_extraction -- input.pdf
//!   cargo run --example text_extraction -- input.pdf --per-page

use pdfluent::prelude::*;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).map(|s| s.as_str()).unwrap_or("input.pdf");
    let per_page = args.iter().any(|a| a == "--per-page");

    let doc = PdfDocument::open(path)?;

    println!("File:  {path}");
    println!("Pages: {}", doc.page_count());
    println!("Version: {:?}", doc.version());
    println!();

    if per_page {
        for (i, page) in doc.pages().enumerate() {
            let page = page?;
            let text = page.text()?;
            println!("--- Page {} ({} chars) ---", i + 1, text.len());
            if text.trim().is_empty() {
                println!("(no text on this page)");
            } else {
                println!("{text}");
            }
            println!();
        }
    } else {
        let text = doc.extract_text()?;
        println!("Full text ({} chars):", text.len());
        println!("{text}");
    }

    Ok(())
}
