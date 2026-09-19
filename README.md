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
| [`howto_compress_pdf`](rust/examples/howto_compress_pdf.rs) | Compress a PDF in Rust — the complete program from [the guide](https://pdfluent.com/how-to/compress-pdf-rust), also shown in its video |
| [`howto_encrypt_pdf`](rust/examples/howto_encrypt_pdf.rs) | Encrypt a PDF with a password in Rust — the complete program from [the guide](https://pdfluent.com/how-to/encrypt-pdf-rust), also shown in its video |
| [`howto_extract_text_by_page`](rust/examples/howto_extract_text_by_page.rs) | Extract text page by page from a PDF in Rust — the complete program from [the guide](https://pdfluent.com/how-to/extract-text-by-page-rust), also shown in its video |
| [`howto_extract_text_pdf`](rust/examples/howto_extract_text_pdf.rs) | Extract text from a PDF in Rust — the complete program from [the guide](https://pdfluent.com/how-to/extract-text-pdf-rust), also shown in its video |
| [`howto_read_pdf_metadata`](rust/examples/howto_read_pdf_metadata.rs) | Read PDF metadata in Rust — the complete program from [the guide](https://pdfluent.com/how-to/read-pdf-metadata-rust), also shown in its video |
| [`howto_write_pdf_metadata`](rust/examples/howto_write_pdf_metadata.rs) | Write PDF metadata in Rust — the complete program from [the guide](https://pdfluent.com/how-to/write-pdf-metadata-rust), also shown in its video |
| [`howto_generate_pdf_thumbnails`](rust/examples/howto_generate_pdf_thumbnails.rs) | Generate page thumbnails from a PDF in Rust — the complete program from [the guide](https://pdfluent.com/how-to/generate-pdf-thumbnails-rust), also shown in its video |
| [`howto_get_pdf_page_count`](rust/examples/howto_get_pdf_page_count.rs) | Get the page count of a PDF in Rust — the complete program from [the guide](https://pdfluent.com/how-to/get-pdf-page-count-rust), also shown in its video |
| [`howto_merge_pdfs`](rust/examples/howto_merge_pdfs.rs) | Merge PDFs in Rust — the complete program from [the guide](https://pdfluent.com/how-to/merge-pdfs-rust), also shown in its video |
| [`howto_render_pdf_to_jpeg`](rust/examples/howto_render_pdf_to_jpeg.rs) | Render PDF pages to JPEG images in Rust — the complete program from [the guide](https://pdfluent.com/how-to/render-pdf-to-jpeg-rust), also shown in its video |
| [`howto_render_pdf_to_png`](rust/examples/howto_render_pdf_to_png.rs) | Render a PDF page to PNG in Rust — the complete program from [the guide](https://pdfluent.com/how-to/render-pdf-to-png-rust), also shown in its video |
| [`wasm/index.html`](wasm/index.html) | Open a PDF and extract its text entirely in the browser via `@pdfluent/sdk-wasm` — no server, no upload |

The `howto_*` examples are the programs from the how-to guides on pdfluent.com, byte for byte; each guide page links here and embeds the video in which the same code is typed and run.

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

AGPL-3.0 or a commercial licence, at your option. The AGPL is the default and
the complete product: there is no licence key, no activation call and no tier,
every feature works in every build, and nothing marks the output. If you cannot
accept the copyleft obligation, the commercial licence is sold yearly and
self-service in four options: Commercial (per organisation), OEM Startup and
OEM (per product), and Priority support (an add-on).

- **Licence terms and prices:** <https://pdfluent.com/sdk/pricing>
- **Full docs:** <https://pdfluent.com/docs>

## More resources

- [Documentation](https://pdfluent.com/docs)
- [Rust API reference](https://docs.rs/pdfluent)
- [Changelog](https://pdfluent.com/changelog)
- [Support](https://pdfluent.com/support)
