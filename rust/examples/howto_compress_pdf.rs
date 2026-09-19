//! Compress a PDF in Rust
//!
//! The complete program from https://pdfluent.com/how-to/compress-pdf-rust
//! Compiled against the published `pdfluent` crate by this repo's CI; the same code is
//! typed and run in the guide's video.

use pdfluent::{PdfDocument, CompressOptions};

fn main() -> pdfluent::Result<()> {
    let mut doc = PdfDocument::open("large.pdf")?;
    let report = doc.compress(CompressOptions::archival())?;
    println!("{} streams compressed", report.streams_compressed);
    doc.save("compressed.pdf")?;
    Ok(())
}
