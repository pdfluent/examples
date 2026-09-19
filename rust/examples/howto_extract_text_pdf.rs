//! Extract text from a PDF in Rust
//!
//! The complete program from https://pdfluent.com/how-to/extract-text-pdf-rust
//! Compiled against the published `pdfluent` crate by this repo's CI; the same code is
//! typed and run in the guide's video.

use pdfluent::PdfDocument;

fn main() -> pdfluent::Result<()> {
    let doc = PdfDocument::open("file.pdf")?;
    println!("{}", doc.extract_text()?);
    Ok(())
}
