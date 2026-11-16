# Mastra with PDFluent

Starts `pdfluent-mcp` through Mastra's `MCPClient`, lists the tools (prefixed with the server name, for example `pdfluent_extract_markdown`), calls two of them directly and builds an `Agent` with them.

## Run

```bash
git clone https://github.com/pdfluent/examples
cd examples/agents/mastra
python3 -m pip install pdfluent-mcp   # puts `pdfluent-mcp` on PATH
npm install
npx tsx agent.ts
```

The script reads `../../sample.pdf` unless you pass another path as the first argument. The PDFluent MCP server only reads and writes inside the folder of that file.

## Model

The `agent.generate()` call needs a model, so it requires your own model key (`OPENAI_API_KEY`). Without one the script builds the agent and stops before the model call; everything up to that point runs against the real server.

## Output

Recorded on 2026-11-16 (macOS, no model key set). The extracted markdown of the sample PDF is printed in between; it is left out here.

```text
tools: pdfluent_extract_markdown, pdfluent_extract_text, pdfluent_extract_tables, pdfluent_extract_blocks, pdfluent_get_metadata, pdfluent_search, pdf
--- pdfluent_get_metadata ---
{"result":{"path":"/path/to/examples/sample.pdf","bytes":2172,"pages":2,"title":"untitled","author":"anonymous","subject":"unspecified","created":"D:2
--- pdfluent_extract_markdown ---
--- agent built with 9 tools ---
--- agent.generate() skipped: requires your own model key (OPENAI_API_KEY) ---
```

## Licence

This example is MIT. It uses `pdfluent` and `pdfluent-mcp`, which are released under AGPL-3.0 or a commercial licence; see the [PDFluent licence page](https://pdfluent.com/license).
