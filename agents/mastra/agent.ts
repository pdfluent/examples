// Mastra with the pdfluent-mcp stdio server as a tool source.
//
// Starts the server through MCPClient, lists its tools, calls one directly, then
// builds a Mastra Agent with those tools. The agent call needs a model:
// with OPENAI_API_KEY set it uses OpenAI, without one the script stops before
// the model call ("requires your own model key").
import { Agent } from "@mastra/core/agent";
import { MCPClient } from "@mastra/mcp";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const pdf = resolve(process.argv[2] ?? resolve(here, "../../sample.pdf"));

const mcp = new MCPClient({
  servers: {
    pdfluent: {
      command: "pdfluent-mcp",
      args: ["--root", dirname(pdf)], // the server reads and writes only inside --root
    },
  },
});

try {
  const tools = await mcp.listTools();
  console.log("tools:", Object.keys(tools).join(", "));

  const meta = await tools["pdfluent_get_metadata"].execute!({ path: pdf });
  console.log("--- pdfluent_get_metadata ---");
  console.log(JSON.stringify(meta).slice(0, 300));

  const md = await tools["pdfluent_extract_markdown"].execute!({ path: pdf });
  console.log("--- pdfluent_extract_markdown ---");
  console.log(JSON.stringify(md).slice(0, 400));

  const agent = new Agent({
    id: "pdf-reader",
    name: "pdf-reader",
    instructions: "Answer from the PDF with the pdfluent tools. Page numbers are 0-based.",
    model: "openai/gpt-4o-mini",
    tools,
  });
  console.log("--- agent built with", Object.keys(await agent.listTools()).length, "tools ---");

  if (process.env.OPENAI_API_KEY) {
    const out = await agent.generate(`Read ${pdf} and name its headings.`);
    console.log(out.text);
  } else {
    console.log("--- agent.generate() skipped: requires your own model key (OPENAI_API_KEY) ---");
  }
} finally {
  await mcp.disconnect();
}
