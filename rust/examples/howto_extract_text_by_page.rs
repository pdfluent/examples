//! Extract text page by page from a PDF in Rust
//!
//! The complete program from https://pdfluent.com/how-to/extract-text-by-page-rust
//! Compiled against the published `pdfluent` crate by this repo's CI; the same code is
//! typed and run in the guide's video.

use pdfluent::PdfDocument;

fn main() -> pdfluent::Result<()> {
    let doc = PdfDocument::open("file.pdf")?;
    for page in doc.pages() {
        println!("{}", page.text()?);
    }
    Ok(())
}
