//! Get the page count of a PDF in Rust
//!
//! The complete program from https://pdfluent.com/how-to/get-pdf-page-count-rust
//! Compiled against the published `pdfluent` crate by this repo's CI; the same code is
//! typed and run in the guide's video.

use pdfluent::PdfDocument;

fn main() -> pdfluent::Result<()> {
    let doc = PdfDocument::open("file.pdf")?;
    println!("{} pages", doc.page_count());
    Ok(())
}
