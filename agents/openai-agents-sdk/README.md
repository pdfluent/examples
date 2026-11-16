# OpenAI Agents SDK with PDFluent

Runs `pdfluent-mcp` as an MCP stdio server inside the Agents SDK (`MCPServerStdio`). The script lists the server's tools, calls `get_metadata` and `extract_markdown` directly, then runs an `Agent` with the server attached.

## Run

```bash
git clone https://github.com/pdfluent/examples
cd examples/agents/openai-agents-sdk
python3 -m venv .venv && . .venv/bin/activate
pip install -r requirements.txt
python agent.py
```

The script reads `../../sample.pdf` unless you pass another path as the first argument. The PDFluent MCP server only reads and writes inside the folder of that file.

## Model

With `OPENAI_API_KEY` set the agent uses an OpenAI model. Without a key a small local stub model stands in for the LLM: it issues one `get_metadata` call and echoes the result, so the whole MCP round trip runs without a key. The stub has no language ability and is only there to exercise the wiring.

## Output

Recorded on 2026-11-16 (macOS, no model key set). The extracted markdown of the sample PDF is printed in between; it is left out here.

```text
tools: extract_markdown, extract_text, extract_tables, extract_blocks, get_metadata, search, list_form_fields, fill_form, vision_advice
pages: 2 | encrypted: False
--- extract_markdown ---
This is a small public sample PDF used by the PDFluent examples. It contains plain text so text-extraction demos have something to find. Generated for
<!-- page 1 -->
--- agent loop: local stub model (no key set) ---
--- agent answer ---
Tool result received: [{'type': 'input_text', 'text': '{\n  "path": "/path/to/examples/sample.pdf",\n  "bytes": 2172,\n  "pa
```

## Licence

This example is MIT. It uses `pdfluent` and `pdfluent-mcp`, which are released under AGPL-3.0 or a commercial licence; see the [PDFluent licence page](https://pdfluent.com/license).
