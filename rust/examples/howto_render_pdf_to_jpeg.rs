//! Render PDF pages to JPEG images in Rust
//!
//! The complete program from https://pdfluent.com/how-to/render-pdf-to-jpeg-rust
//! Compiled against the published `pdfluent` crate by this repo's CI; the same code is
//! typed and run in the guide's video.

use pdfluent::{PdfDocument, ImageFormat};

fn main() -> pdfluent::Result<()> {
    let doc = PdfDocument::open("file.pdf")?;
    let jpg = doc.render_page(1, 150, ImageFormat::Jpeg)?;
    std::fs::write("page-1.jpg", jpg)?;
    Ok(())
}
