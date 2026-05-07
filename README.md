# PDFluent Examples

Working code examples for the [PDFluent SDK](https://pdfluent.com) — a pure-Rust PDF SDK with XFA, PDF/A, digital signatures, redaction, and WASM support.

## Quick start

```bash
git clone https://github.com/pdfluent/examples
cd examples
cargo run --example text_extraction -- /path/to/your.pdf
```

## Examples

| Example | What it demonstrates |
|---|---|
| [`text_extraction`](rust/examples/text_extraction.rs) | Open a PDF, extract all text, iterate page by page |
| [`form_fill`](rust/examples/form_fill.rs) | Fill AcroForm text fields and checkboxes, then flatten to static PDF |
| [`merge_pdfs`](rust/examples/merge_pdfs.rs) | Merge multiple PDFs into one with a shared bookmark structure |
| [`pdfa_validate`](rust/examples/pdfa_validate.rs) | Validate PDF/A-1b/2b/3b compliance, list violations |
| [`redact_text`](rust/examples/redact_text.rs) | Search-based redaction — find patterns and permanently remove them |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
pdfluent = "1.0.0-beta.5"
```

Or with specific features:

```toml
[dependencies]
pdfluent = { version = "1.0.0-beta.5", features = ["signing", "pdfa", "redaction"] }
```

## Licensing

PDFluent is free for evaluation. A valid license is required for production use.

Output from unlicensed builds carries an "unlicensed evaluation" marker in the PDF metadata.

- **Trial key (30 days):** <https://pdfluent.com/trial>
- **Pricing:** <https://pdfluent.com/pricing>
- **Full docs:** <https://pdfluent.com/docs>

## More resources

- [Documentation](https://pdfluent.com/docs)
- [Rust API reference](https://docs.rs/pdfluent)
- [Changelog](https://pdfluent.com/changelog)
- [Support](https://pdfluent.com/support)
