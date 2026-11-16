# Vercel AI SDK (TypeScript) with PDFluent

Connects to `pdfluent-mcp` with the AI SDK MCP client (`@ai-sdk/mcp`, stdio transport), lists the tools, calls `get_metadata` directly and passes the tools to `generateText`.

## Run

```bash
git clone https://github.com/pdfluent/examples
cd examples/agents/vercel-ai-sdk
python3 -m pip install pdfluent-mcp   # puts `pdfluent-mcp` on PATH
npm install
npx tsx agent.ts
```

The script reads `../../sample.pdf` unless you pass another path as the first argument. The PDFluent MCP server only reads and writes inside the folder of that file.

## Model

With `OPENAI_API_KEY` set the model is OpenAI. Without a key a mock language model from `ai/test` issues one `extract_markdown` call, so the tool call runs through the real MCP server. The client option `protocolVersionDiscovery: false` is set because the server needs about a second to start and the default protocol probe can time out and race the handshake.

## Output

Recorded on 2026-11-16 (macOS, no model key set). The extracted markdown of the sample PDF is printed in between; it is left out here.

```text
tools: extract_markdown, extract_text, extract_tables, extract_blocks, get_metadata, search, list_form_fields, fill_form, vision_advice
--- get_metadata ---
{"content":[{"type":"text","text":"{\n  \"path\": \"/path/to/examples/sample.pdf\",\n  \"bytes\": 2172,\n  \"pages\": 2,\n  \"title\": \"untitled\",\n
--- model: local mock (no OPENAI_API_KEY set; a real run requires your own model key) ---
answer: Mock model: the tool result is above.
```

## Licence

This example is MIT. It uses `pdfluent` and `pdfluent-mcp`, which are released under AGPL-3.0 or a commercial licence; see the [PDFluent licence page](https://pdfluent.com/license).
