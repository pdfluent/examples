"""OpenAI Agents SDK with the pdfluent-mcp stdio server as a tool source.

It starts the server, lists the tools and calls two of them directly, then runs
an agent loop. With OPENAI_API_KEY set the agent uses a real OpenAI model;
without one it uses a local stub model that issues one tool call, so the MCP
round trip is still exercised end to end.
"""

import asyncio
import json
import os
import sys
from pathlib import Path

from agents import Agent, ModelResponse, Runner, Usage
from agents.mcp import MCPServerStdio
from agents.models.interface import Model
from openai.types.responses import ResponseFunctionToolCall, ResponseOutputMessage, ResponseOutputText

pdf = Path(sys.argv[1] if len(sys.argv) > 1 else Path(__file__).parents[2] / "sample.pdf").resolve()


class StubModel(Model):
    """Local stand-in for the LLM: calls get_metadata once, then answers.

    It lets the whole agent loop run (tool call over MCP, tool result, final
    answer) without a model key. It has no language ability.
    """

    def __init__(self, path: str) -> None:
        self.path = path

    async def get_response(self, system_instructions, input, model_settings, tools, output_schema,
                           handoffs, tracing, **kwargs):
        history = input if isinstance(input, list) else []
        done = any(isinstance(i, dict) and i.get("type") == "function_call_output" for i in history)
        if not done:
            call = ResponseFunctionToolCall(
                type="function_call", call_id="call_1", name="get_metadata",
                arguments=json.dumps({"path": self.path}),
            )
            return ModelResponse(output=[call], usage=Usage(), response_id="stub-1")
        last = [i for i in history if isinstance(i, dict) and i.get("type") == "function_call_output"][-1]
        msg = ResponseOutputMessage(
            id="m1", type="message", role="assistant", status="completed",
            content=[ResponseOutputText(type="output_text", annotations=[],
                                        text="Tool result received: " + str(last["output"])[:200])],
        )
        return ModelResponse(output=[msg], usage=Usage(), response_id="stub-2")

    def stream_response(self, *args, **kwargs):
        raise NotImplementedError("the stub model does not stream")


def text_of(result) -> str:
    return "".join(c.text for c in result.content if getattr(c, "type", "") == "text")


async def main() -> None:
    # The server reads and writes only inside --root.
    params = {"command": "pdfluent-mcp", "args": ["--root", str(pdf.parent)]}
    async with MCPServerStdio(params=params, name="pdfluent") as server:
        tools = await server.list_tools()
        print("tools:", ", ".join(t.name for t in tools))

        meta = json.loads(text_of(await server.call_tool("get_metadata", {"path": str(pdf)})))
        print("pages:", meta["pages"], "| encrypted:", meta["encrypted"])

        md = text_of(await server.call_tool("extract_markdown", {"path": str(pdf)}))
        print("--- extract_markdown ---")
        print(md[:600])

        # A real model needs your own OPENAI_API_KEY; without one the stub runs the loop.
        live = bool(os.environ.get("OPENAI_API_KEY"))
        print("--- agent loop:", "OpenAI model" if live else "local stub model (no key set)", "---")
        agent = Agent(
            name="PDF reader",
            instructions="Answer from the PDF with the pdfluent tools. Page numbers are 0-based.",
            mcp_servers=[server],
            **({} if live else {"model": StubModel(str(pdf))}),
        )
        out = await Runner.run(agent, f"What is {pdf} about? Name its headings.")
        print("--- agent answer ---")
        print(out.final_output)


asyncio.run(main())
