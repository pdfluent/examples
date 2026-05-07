//! Fill an AcroForm PDF and save a flattened (non-editable) copy.
//!
//! Usage:
//!   cargo run --example form_fill -- input.pdf output.pdf
//!
//! The example fills whichever fields it finds by name. Adapt the
//! field names and values to match your actual form.

use pdfluent::prelude::*;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = args.get(1).map(|s| s.as_str()).unwrap_or("form.pdf");
    let output = args.get(2).map(|s| s.as_str()).unwrap_or("form-filled.pdf");

    let mut doc = PdfDocument::open(input)?;

    // List all form fields before filling.
    let fields = doc.form_fields()?;
    println!("Found {} form field(s) in {input}:", fields.len());
    for field in &fields {
        println!("  {:?}  name={:?}  value={:?}", field.field_type, field.name, field.value);
    }
    println!();

    // Fill fields by name. Unknown names produce an error — catch them if
    // you're not sure which fields your PDF has.
    {
        let mut form = doc.form_mut();
        form.set_text("name", "Jasper de Winter")?
            .set_text("email", "jasper@example.com")?
            .set_text("company", "Innovation Trigger BV")?
            .set_checkbox("agree_terms", true)?;
    }

    // Flatten: converts form fields to static content so they can't be edited.
    doc.flatten_forms()?;

    doc.save(output)?;
    println!("Saved flattened PDF to: {output}");

    Ok(())
}
