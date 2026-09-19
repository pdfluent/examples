//! Encrypt a PDF with a password in Rust
//!
//! The complete program from https://pdfluent.com/how-to/encrypt-pdf-rust
//! Compiled against the published `pdfluent` crate by this repo's CI; the same code is
//! typed and run in the guide's video.

use pdfluent::{PdfDocument, EncryptOptions};

fn main() -> pdfluent::Result<()> {
    let mut doc = PdfDocument::open("report.pdf")?;
    let opts = EncryptOptions::aes256()
        .with_user_password("open-sesame")
        .with_owner_password("owner-secret");
    doc.encrypt(opts)?;
    doc.save("report-encrypted.pdf")?;
    Ok(())
}
