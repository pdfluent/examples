# Agent framework examples

Each folder reads a local PDF with PDFluent inside one agent framework. Every script runs against the bundled `../sample.pdf` and does as much as it can without a model: it starts the tool source, lists the tools and calls them directly. Where the framework allows a local stand-in model, the agent loop runs on that; otherwise the script says "requires your own model key" and stops before the model call. No script reads a key from anywhere except the environment variable it names.

| Folder | Framework | PDFluent route | Agent loop without a key |
|---|---|---|---|
| [`openai-agents-sdk`](openai-agents-sdk) | OpenAI Agents SDK | `pdfluent-mcp` stdio server | local stub model |
| [`vercel-ai-sdk`](vercel-ai-sdk) | Vercel AI SDK (TypeScript) | `pdfluent-mcp` through `@ai-sdk/mcp` | mock model |
| [`mastra`](mastra) | Mastra (TypeScript) | `pdfluent-mcp` through `@mastra/mcp` | stops before the model call |
| [`crewai`](crewai) | CrewAI | tool over the `pdfluent` package | stops before the model call |
| [`langgraph`](langgraph) | LangGraph | `langchain-pdfluent` loader | local stub chat model |
| [`semantic-kernel`](semantic-kernel) | Semantic Kernel (Python) | plugin over the `pdfluent` package | stops before the model call |

CI runs every script on each pull request and weekly (`.github/workflows/agents.yml`).
