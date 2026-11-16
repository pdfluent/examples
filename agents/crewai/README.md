# CrewAI with PDFluent

A CrewAI `BaseTool` (`pdf_to_markdown`) that wraps the `pdfluent` Python package. The script calls the tool directly, then builds an `Agent`, a `Task` and a `Crew` around it.

## Run

```bash
git clone https://github.com/pdfluent/examples
cd examples/agents/crewai
python3 -m venv .venv && . .venv/bin/activate
pip install -r requirements.txt
python crew.py
```

The script reads `../../sample.pdf` unless you pass another path as the first argument. The PDFluent MCP server only reads and writes inside the folder of that file.

## Model

`crew.kickoff()` requires your own model key: set `OPENAI_API_KEY`, or set `CREWAI_MODEL` to another provider's model name together with that provider's key. Without a key the script stops before the model call.

## Output

Recorded on 2026-11-16 (macOS, no model key set). The extracted markdown of the sample PDF is printed in between; it is left out here.

```text
--- tool called directly ---
This is a small public sample PDF used by the PDFluent examples. It contains plain text so text-extraction demos have something to find. Generated for
--- crew built: 1 agent, 1 task, ['pdf_to_markdown']
--- crew.kickoff() skipped: requires your own model key (OPENAI_API_KEY, or CREWAI_MODEL for another provider) ---
```

## Licence

This example is MIT. It uses `pdfluent` and `pdfluent-mcp`, which are released under AGPL-3.0 or a commercial licence; see the [PDFluent licence page](https://pdfluent.com/license).
