//! Fill PDF form fields in Rust
//!
//! The complete program from https://pdfluent.com/how-to/fill-pdf-form-rust
//! Compiled against the published `pdfluent` crate by this repo's CI; the same code is
//! typed and run in the guide's video.

use pdfluent::prelude::*;

fn main() -> pdfluent::Result<()> {
    let mut doc = PdfDocument::open("application.pdf")?;

    if doc.form_fields()?.is_empty() {
        eprintln!("This PDF has no form fields.");
        return Ok(());
    }

    let mut form = doc.form_mut();
    form.set_text("first_name", "Jane")?
        .set_text("last_name", "Smith")?
        .set_text("email", "jane@example.com")?;
    form.set_checkbox("agree_terms", true)?
        .set_checkbox("newsletter", false)?
        .set_radio("payment_method", "credit_card")?;

    doc.save("filled-application.pdf")?;
    Ok(())
}
