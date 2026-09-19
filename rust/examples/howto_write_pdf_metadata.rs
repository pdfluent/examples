//! Write PDF metadata in Rust
//!
//! The complete program from https://pdfluent.com/how-to/write-pdf-metadata-rust
//! Compiled against the published `pdfluent` crate by this repo's CI; the same code is
//! typed and run in the guide's video.

use pdfluent::PdfDocument;

fn main() -> pdfluent::Result<()> {
    let mut doc = PdfDocument::open("report.pdf")?;
    doc.metadata_mut()
        .set_title("Q4 Report")
        .set_author("Finance Team")
        .commit()?;
    doc.save("report-tagged.pdf")?;
    Ok(())
}
