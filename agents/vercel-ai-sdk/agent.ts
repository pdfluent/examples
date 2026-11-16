// Vercel AI SDK with the pdfluent-mcp stdio server as a tool source.
//
// Starts the server, lists its tools, calls one directly, then runs generateText
// with the MCP tools. With OPENAI_API_KEY set the model is OpenAI (install
// @ai-sdk/openai first); without a key a local mock model issues one tool call,
// so the MCP round trip still runs end to end.
import { generateText, stepCountIs } from "ai";
import { MockLanguageModelV4 } from "ai/test";
import { createMCPClient } from "@ai-sdk/mcp";
import { StdioMCPTransport } from "@ai-sdk/mcp/mcp-stdio";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const pdf = resolve(process.argv[2] ?? resolve(here, "../../sample.pdf"));

const client = await createMCPClient({
  // pdfluent-mcp needs about a second to start. The client's default probe for the newer
  // protocol times out first and then races the normal handshake, so connect the plain way.
  protocolVersionDiscovery: false,
  transport: new StdioMCPTransport({
    command: "pdfluent-mcp",
    args: ["--root", dirname(pdf)], // the server reads and writes only inside --root
  }),
});

try {
  const tools = await client.tools();
  console.log("tools:", Object.keys(tools).join(", "));

  const meta = await tools.get_metadata.execute!({ path: pdf }, { toolCallId: "direct", messages: [], context: {} });
  console.log("--- get_metadata ---");
  console.log(JSON.stringify(meta).slice(0, 300));

  let model;
  if (process.env.OPENAI_API_KEY) {
    const { openai } = await import("@ai-sdk/openai");
    model = openai("gpt-4o-mini");
    console.log("--- model: OpenAI ---");
  } else {
    // Local stand-in for the LLM: one tool call, then a fixed answer. No language ability.
    let step = 0;
    model = new MockLanguageModelV4({
      doGenerate: async () => {
        step += 1;
        const usage = {
          inputTokens: { total: 0, noCache: 0, cacheRead: 0, cacheWrite: 0 },
          outputTokens: { total: 0, text: 0, reasoning: 0 },
        };
        return step === 1
          ? {
              content: [{ type: "tool-call" as const, toolCallId: "call_1", toolName: "extract_markdown", input: JSON.stringify({ path: pdf }) }],
              finishReason: { unified: "tool-calls" as const, raw: undefined },
              usage,
              warnings: [],
            }
          : { content: [{ type: "text" as const, text: "Mock model: the tool result is above." }], finishReason: { unified: "stop" as const, raw: undefined }, usage, warnings: [] };
      },
    });
    console.log("--- model: local mock (no OPENAI_API_KEY set; a real run requires your own model key) ---");
  }

  const result = await generateText({
    model,
    tools,
    stopWhen: stepCountIs(3),
    prompt: `Read ${pdf} and name its headings.`,
  });
  for (const call of result.steps.flatMap((s) => s.toolResults)) {
    console.log(`tool ${call.toolName} returned:`, JSON.stringify(call.output).slice(0, 300));
  }
  console.log("answer:", result.text);
} finally {
  await client.close();
}
