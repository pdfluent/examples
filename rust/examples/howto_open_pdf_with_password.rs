//! Open a password-protected PDF in Rust
//!
//! The complete program from https://pdfluent.com/how-to/open-pdf-with-password-rust
//! Compiled against the published `pdfluent` crate by this repo's CI; the same code is
//! typed and run in the guide's video.

use pdfluent::{PdfDocument, OpenOptions};

fn main() -> pdfluent::Result<()> {
    let doc = PdfDocument::open_with(
        "locked.pdf",
        OpenOptions::new().with_password("s3cret"),
    )?;
    println!("{} pages", doc.page_count());
    Ok(())
}
