# Semantic Kernel (Python) with PDFluent

A Semantic Kernel plugin class with two `@kernel_function` methods over the `pdfluent` package (`pdf_markdown`, `pdf_search`). The script registers it, prints the function metadata a model would see and invokes both functions through the kernel.

## Run

```bash
git clone https://github.com/pdfluent/examples
cd examples/agents/semantic-kernel
python3 -m venv .venv && . .venv/bin/activate
pip install -r requirements.txt
python plugin.py
```

The script reads `../../sample.pdf` unless you pass another path as the first argument. The PDFluent MCP server only reads and writes inside the folder of that file.

## Model

Letting a model call the plugin means adding a chat completion service with function calling enabled, which requires your own model key. The script does not do that step; it tests everything up to it.

## Output

Recorded on 2026-11-16 (macOS, no model key set). The extracted markdown of the sample PDF is printed in between; it is left out here.

```text
function: pdf-pdf_markdown ['path']
function: pdf-pdf_search ['path', 'query']
--- pdf-pdf_markdown ---
This is a small public sample PDF used by the PDFluent examples. It contains plain text so text-extraction demos have something to find. Generated for
--- pdf-pdf_search ---
pages containing 'multi-page': [1]
--- chat completion with function calling skipped: requires your own model key ---
```

## Licence

This example is MIT. It uses `pdfluent` and `pdfluent-mcp`, which are released under AGPL-3.0 or a commercial licence; see the [PDFluent licence page](https://pdfluent.com/license).
