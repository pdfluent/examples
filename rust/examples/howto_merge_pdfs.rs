//! Merge PDFs in Rust
//!
//! The complete program from https://pdfluent.com/how-to/merge-pdfs-rust
//! Compiled against the published `pdfluent` crate by this repo's CI; the same code is
//! typed and run in the guide's video.

use pdfluent::prelude::*;

fn main() -> Result<()> {
    let merged = PdfMerger::new()
        .add(PdfDocument::open("part1.pdf")?)
        .add(PdfDocument::open("part2.pdf")?)
        .add(PdfDocument::open("part3.pdf")?)
        .build()?;

    merged.save("combined.pdf")?;

    println!("Merged {} pages into combined.pdf", merged.page_count());
    Ok(())
}
