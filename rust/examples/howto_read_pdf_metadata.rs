//! Read PDF metadata in Rust
//!
//! The complete program from https://pdfluent.com/how-to/read-pdf-metadata-rust
//! Compiled against the published `pdfluent` crate by this repo's CI; the same code is
//! typed and run in the guide's video.

use pdfluent::PdfDocument;

fn main() -> pdfluent::Result<()> {
    let doc = PdfDocument::open("report.pdf")?;
    let meta = doc.metadata();
    println!("title: {:?}", meta.title);
    println!("author: {:?}", meta.author);
    Ok(())
}
