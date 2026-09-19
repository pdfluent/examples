//! Render a PDF page to PNG in Rust
//!
//! The complete program from https://pdfluent.com/how-to/render-pdf-to-png-rust
//! Compiled against the published `pdfluent` crate by this repo's CI; the same code is
//! typed and run in the guide's video.

use pdfluent::{PdfDocument, ImageFormat};

fn main() -> pdfluent::Result<()> {
    let doc = PdfDocument::open("file.pdf")?;
    let png = doc.render_page(1, 150, ImageFormat::Png)?;
    std::fs::write("page-1.png", png)?;
    Ok(())
}
