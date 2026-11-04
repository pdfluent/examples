<h1 align="center">PDFluent Examples</h1>

<p align="center"><strong>Runnable examples for the PDFluent SDK, a pure-Rust PDF SDK.</strong></p>

<p align="center">
  <a href="https://github.com/pdfluent/examples/actions/workflows/ci.yml"><img src="https://github.com/pdfluent/examples/actions/workflows/ci.yml/badge.svg" alt="CI"/></a>
  <a href="https://crates.io/crates/pdfluent"><img src="https://img.shields.io/crates/v/pdfluent?label=crates.io&color=111111" alt="crates.io"/></a>
  <a href="https://www.npmjs.com/package/@pdfluent/sdk-wasm"><img src="https://img.shields.io/npm/v/%40pdfluent%2Fsdk-wasm?label=npm%20wasm&color=111111" alt="npm"/></a>
  <a href="https://pdfluent.com/docs/"><img src="https://img.shields.io/badge/docs-pdfluent.com-111111" alt="Documentation"/></a>
</p>

Working code examples for the [PDFluent SDK](https://github.com/pdfluent/pdfluent-sdk), a pure-Rust PDF SDK with XFA, PDF/A, digital signatures, and redaction, with bindings for Rust, Python, Node.js, WASM, .NET, and Java.

This repository currently has Rust examples and a browser/WASM demo. For the other languages, see the install snippets and docs linked below.

<p align="center">
  <a href="https://pdfluent.com/docs/"><b>Documentation</b></a> ·
  <a href="https://pdfluent.com/cookbook/"><b>Cookbook</b></a> ·
  <a href="https://docs.rs/pdfluent">Rust API reference</a> ·
  <a href="https://pdfluent.com/changelog">Changelog</a> ·
  <a href="https://pdfluent.com/support">Support</a>
</p>

---

## Quick start

```bash
git clone https://github.com/pdfluent/examples
cd examples/rust
cargo run --example text_extraction -- ../sample.pdf
```

## Examples

| Example | What it shows | Language | How to run |
|---|---|---|---|
| [`text_extraction`](rust/examples/text_extraction.rs) | Open a PDF, extract all text, iterate page by page | Rust | `cargo run --example text_extraction -- ../sample.pdf` |
| [`form_fill`](rust/examples/form_fill.rs) | Fill AcroForm text fields and checkboxes, then flatten to static PDF | Rust | `cargo run --example form_fill -- in.pdf out.pdf` |
| [`merge_pdfs`](rust/examples/merge_pdfs.rs) | Merge multiple PDFs into one with a shared bookmark structure | Rust | `cargo run --example merge_pdfs -- a.pdf b.pdf -o merged.pdf` |
| [`pdfa_validate`](rust/examples/pdfa_validate.rs) | Validate PDF/A-1b/2b/3b compliance, list violations | Rust | `cargo run --example pdfa_validate -- ../sample.pdf` |
| [`redact_text`](rust/examples/redact_text.rs) | Search-based redaction: find patterns and permanently remove them | Rust | `cargo run --example redact_text -- in.pdf out.pdf PATTERN` |
| [`wasm/index.html`](wasm/index.html) | Open a PDF and extract its text entirely in the browser via `@pdfluent/sdk-wasm`, no server, no upload | JavaScript (WASM) | Serve the repo root (`python3 -m http.server`), open `/wasm/` |

### How-to guide programs

The `howto_*` examples are the programs from the how-to guides on pdfluent.com, byte for byte; each guide page links here and embeds the video in which the same code is typed and run. Each one reads the input file named in its source, from the directory you run it in.

| Example | What it shows | Language | How to run |
|---|---|---|---|
| [`howto_compress_pdf`](rust/examples/howto_compress_pdf.rs) | Compress a PDF in Rust, the complete program from [the guide](https://pdfluent.com/how-to/compress-pdf-rust), also shown in its video | Rust | `cargo run --example howto_compress_pdf` |
| [`howto_encrypt_pdf`](rust/examples/howto_encrypt_pdf.rs) | Encrypt a PDF with a password in Rust, the complete program from [the guide](https://pdfluent.com/how-to/encrypt-pdf-rust), also shown in its video | Rust | `cargo run --example howto_encrypt_pdf` |
| [`howto_extract_text_by_page`](rust/examples/howto_extract_text_by_page.rs) | Extract text page by page from a PDF in Rust, the complete program from [the guide](https://pdfluent.com/how-to/extract-text-by-page-rust), also shown in its video | Rust | `cargo run --example howto_extract_text_by_page` |
| [`howto_extract_text_pdf`](rust/examples/howto_extract_text_pdf.rs) | Extract text from a PDF in Rust, the complete program from [the guide](https://pdfluent.com/how-to/extract-text-pdf-rust), also shown in its video | Rust | `cargo run --example howto_extract_text_pdf` |
| [`howto_read_pdf_metadata`](rust/examples/howto_read_pdf_metadata.rs) | Read PDF metadata in Rust, the complete program from [the guide](https://pdfluent.com/how-to/read-pdf-metadata-rust), also shown in its video | Rust | `cargo run --example howto_read_pdf_metadata` |
| [`howto_write_pdf_metadata`](rust/examples/howto_write_pdf_metadata.rs) | Write PDF metadata in Rust, the complete program from [the guide](https://pdfluent.com/how-to/write-pdf-metadata-rust), also shown in its video | Rust | `cargo run --example howto_write_pdf_metadata` |
| [`howto_generate_pdf_thumbnails`](rust/examples/howto_generate_pdf_thumbnails.rs) | Generate page thumbnails from a PDF in Rust, the complete program from [the guide](https://pdfluent.com/how-to/generate-pdf-thumbnails-rust), also shown in its video | Rust | `cargo run --example howto_generate_pdf_thumbnails` |
| [`howto_get_pdf_page_count`](rust/examples/howto_get_pdf_page_count.rs) | Get the page count of a PDF in Rust, the complete program from [the guide](https://pdfluent.com/how-to/get-pdf-page-count-rust), also shown in its video | Rust | `cargo run --example howto_get_pdf_page_count` |
| [`howto_merge_pdfs`](rust/examples/howto_merge_pdfs.rs) | Merge PDFs in Rust, the complete program from [the guide](https://pdfluent.com/how-to/merge-pdfs-rust), also shown in its video | Rust | `cargo run --example howto_merge_pdfs` |
| [`howto_render_pdf_to_jpeg`](rust/examples/howto_render_pdf_to_jpeg.rs) | Render PDF pages to JPEG images in Rust, the complete program from [the guide](https://pdfluent.com/how-to/render-pdf-to-jpeg-rust), also shown in its video | Rust | `cargo run --example howto_render_pdf_to_jpeg` |
| [`howto_render_pdf_to_png`](rust/examples/howto_render_pdf_to_png.rs) | Render a PDF page to PNG in Rust, the complete program from [the guide](https://pdfluent.com/how-to/render-pdf-to-png-rust), also shown in its video | Rust | `cargo run --example howto_render_pdf_to_png` |
| [`howto_open_pdf_with_password`](rust/examples/howto_open_pdf_with_password.rs) | Open a password-protected PDF in Rust, the complete program from [the guide](https://pdfluent.com/how-to/open-pdf-with-password-rust), also shown in its video | Rust | `cargo run --example howto_open_pdf_with_password` |
| [`howto_fill_pdf_form`](rust/examples/howto_fill_pdf_form.rs) | Fill PDF form fields in Rust, the complete program from [the guide](https://pdfluent.com/how-to/fill-pdf-form-rust), also shown in its video | Rust | `cargo run --example howto_fill_pdf_form` |

All examples run against [`sample.pdf`](sample.pdf), a small public test document bundled in this repo.

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
pdfluent = "1.0.0"
```

Or with specific features:

```toml
[dependencies]
pdfluent = { version = "1.0.0", features = ["signing", "pdfa", "redaction"] }
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

---

## Licensing

The examples in this repository are licensed under the MIT licence (see
[LICENSE](LICENSE)). The PDFluent SDK they call stays under AGPL-3.0 or a
commercial licence, at your option. The AGPL is the default and the complete
product: there is no licence key, no activation call and no tier, every feature
works in every build, and nothing marks the output. If you cannot accept the
copyleft obligation, the commercial licence is sold yearly and self-service in
four options: Commercial (per organisation), OEM Startup and OEM (per product),
and Priority support (an add-on). Security reports go to hello@pdfluent.com.

- **Licence terms and prices:** <https://pdfluent.com/sdk/pricing>
- **Full docs:** <https://pdfluent.com/docs>

## More resources

- [Documentation](https://pdfluent.com/docs)
- [Cookbook](https://pdfluent.com/cookbook/)
- [Rust API reference](https://docs.rs/pdfluent)
- [Changelog](https://pdfluent.com/changelog)
- [Support](https://pdfluent.com/support)

---

<p align="center">Built by Innovation Trigger B.V. · <a href="https://pdfluent.com/">pdfluent.com</a></p>
