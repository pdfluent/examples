# LangGraph with PDFluent

A LangChain tool that loads a PDF with `langchain-pdfluent` (`PDFluentLoader`, one `Document` per heading, paragraph, list item or table, with `page` and `bbox` metadata), wired into a LangGraph agent built with `langchain.agents.create_agent`.

## Run

```bash
git clone https://github.com/pdfluent/examples
cd examples/agents/langgraph
python3 -m venv .venv && . .venv/bin/activate
pip install -r requirements.txt
python graph.py
```

The script reads `../../sample.pdf` unless you pass another path as the first argument. The PDFluent MCP server only reads and writes inside the folder of that file.

## Model

With `OPENAI_API_KEY` set (and `langchain-openai` installed) the agent uses `gpt-4o-mini`. Without a key a local stub chat model issues one `read_pdf` call and echoes the result, so the graph, the tool node and the loader all run for real. The stub has no language ability.

## Output

Recorded on 2026-11-16 (macOS, no model key set). The extracted markdown of the sample PDF is printed in between; it is left out here.

```text
--- model: local stub (no OPENAI_API_KEY set; a real run requires your own model key) ---
human: 'What is in /path/to/examples/sample.pdf?'
ai: '' tool_calls=['read_pdf']
```

## Licence

This example is MIT. It uses `pdfluent` and `pdfluent-mcp`, which are released under AGPL-3.0 or a commercial licence; see the [PDFluent licence page](https://pdfluent.com/license).
