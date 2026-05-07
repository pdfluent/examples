//! Validate a PDF for PDF/A-2b compliance and report findings.
//!
//! Usage:
//!   cargo run --example pdfa_validate -- input.pdf

use pdfluent::prelude::*;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).map(|s| s.as_str()).unwrap_or("input.pdf");

    let doc = PdfDocument::open(path)?;

    // Validate against PDF/A-2b (widely supported profile).
    let report = doc.validate_pdfa(PdfAProfile::A2b)?;

    println!("File:     {path}");
    println!("Profile:  PDF/A-2b");
    println!("Compliant: {}", report.is_compliant());
    println!("Findings:  {}", report.violations.len());

    if !report.violations.is_empty() {
        println!();
        for v in &report.violations {
            println!("  [{:?}] {} — {}", v.severity, v.rule, v.message);
        }
    }

    // Exit non-zero when not compliant, so this can be used in CI pipelines.
    if !report.is_compliant() {
        std::process::exit(1);
    }

    Ok(())
}
