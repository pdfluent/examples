//! Generate page thumbnails from a PDF in Rust
//!
//! The complete program from https://pdfluent.com/how-to/generate-pdf-thumbnails-rust
//! Compiled against the published `pdfluent` crate by this repo's CI; the same code is
//! typed and run in the guide's video.

use pdfluent::{PdfDocument, ImageFormat};

fn main() -> pdfluent::Result<()> {
    let doc = PdfDocument::open("file.pdf")?;
    for i in 1..=doc.page_count() {
        let png = doc.render_page(i, 72, ImageFormat::Png)?;
        std::fs::write(format!("thumb-{}.png", i), png)?;
    }
    Ok(())
}
