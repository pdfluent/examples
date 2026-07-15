# PDFluent Examples

[![CI](https://github.com/pdfluent/examples/actions/workflows/ci.yml/badge.svg)](https://github.com/pdfluent/examples/actions/workflows/ci.yml)

Working code examples for the [PDFluent SDK](https://github.com/pdfluent/pdfluent-sdk), a pure-Rust PDF SDK with XFA, PDF/A, digital signatures, and redaction, with bindings for Rust, Python, Node.js, WASM, .NET, and Java.

This repository currently has Rust examples and a browser/WASM demo. For the other languages, see the install snippets and docs linked below.

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
| [`redact_text`](rust/examples/redact_text.rs) | Search-based redaction: find patterns and permanently remove them |
| [`wasm/index.html`](wasm/index.html) | Open a PDF and extract its text entirely in the browser via `@pdfluent/sdk-wasm` — no server, no upload |

All examples run against [`sample.pdf`](sample.pdf), a small public test document bundled in this repo.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
pdfluent = "1.0.0-beta.17"
```

Or with specific features:

```toml
[dependencies]
pdfluent = { version = "1.0.0-beta.17", features = ["signing", "pdfa", "redaction"] }
```

## Other languages

The same engine is available outside Rust:

| Language | Package | Docs |
|---|---|---|
| Python | [`pdfluent`](https://pypi.org/project/pdfluent) on PyPI | [pdfluent.com/docs/python](https://pdfluent.com/docs/python) |
| Node.js | [`@pdfluent/node`](https://www.npmjs.com/package/@pdfluent/node) on npm | [pdfluent.com/docs](https://pdfluent.com/docs) |
| Browser / WASM | [`@pdfluent/sdk-wasm`](https://www.npmjs.com/package/@pdfluent/sdk-wasm) on npm | [pdfluent.com/docs/wasm](https://pdfluent.com/docs/wasm) |
| .NET | [`pdfluent`](https://www.nuget.org/packages/pdfluent) on NuGet | [pdfluent.com/docs/dotnet](https://pdfluent.com/docs/dotnet) |
| Java | [`com.pdfluent:pdfluent`](https://central.sonatype.com/artifact/com.pdfluent/pdfluent) on Maven Central | [pdfluent.com/docs/java](https://pdfluent.com/docs/java) |

## Licensing

PDFluent is free for evaluation. A valid license is required for production use.

Output from unlicensed builds carries an "unlicensed evaluation" marker in the PDF metadata.

- **30-day evaluation key** (full features, no output watermark): <https://pdfluent.com/trial>
- **Pricing:** <https://pdfluent.com/sdk/pricing>
- **Full docs:** <https://pdfluent.com/docs>

## More resources

- [Documentation](https://pdfluent.com/docs)
- [Rust API reference](https://docs.rs/pdfluent)
- [Changelog](https://pdfluent.com/changelog)
- [Support](https://pdfluent.com/support)
