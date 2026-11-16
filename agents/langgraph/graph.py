"""LangGraph agent whose tool loads a PDF with langchain-pdfluent.

The loader turns the PDF into one LangChain Document per block (heading,
paragraph, list item or table) with page and bounding box in the metadata.
Without a model key the graph runs on a local stub model that issues one tool
call, so the graph, the tool node and the loader all run for real.
"""

import os
import sys
from pathlib import Path

from langchain_core.language_models.chat_models import BaseChatModel
from langchain_core.messages import AIMessage
from langchain_core.outputs import ChatGeneration, ChatResult
from langchain_core.tools import tool
from langchain_pdfluent import PDFluentLoader
from langchain.agents import create_agent  # builds a LangGraph graph

pdf = Path(sys.argv[1] if len(sys.argv) > 1 else Path(__file__).parents[2] / "sample.pdf").resolve()


@tool
def read_pdf(path: str) -> str:
    """Read a local PDF and return its blocks, one per line, with kind and page (0-based)."""
    docs = PDFluentLoader(path).load()
    return "\n".join(f"[{d.metadata['kind']} p{d.metadata['page']}] {d.page_content}" for d in docs)


class StubChatModel(BaseChatModel):
    """Calls read_pdf once, then answers with what the tool returned. No language ability."""

    path: str

    @property
    def _llm_type(self) -> str:
        return "stub"

    def bind_tools(self, tools, **kwargs):
        return self

    def _generate(self, messages, stop=None, run_manager=None, **kwargs):
        if messages[-1].type == "tool":
            msg = AIMessage(content="Tool result: " + messages[-1].content)
        else:
            msg = AIMessage(content="", tool_calls=[{"name": "read_pdf", "args": {"path": self.path}, "id": "call_1"}])
        return ChatResult(generations=[ChatGeneration(message=msg)])


if os.environ.get("OPENAI_API_KEY"):
    from langchain_openai import ChatOpenAI  # pip install langchain-openai

    model = ChatOpenAI(model="gpt-4o-mini")
    print("--- model: OpenAI ---")
else:
    model = StubChatModel(path=str(pdf))
    print("--- model: local stub (no OPENAI_API_KEY set; a real run requires your own model key) ---")

agent = create_agent(model, [read_pdf])
result = agent.invoke({"messages": [("user", f"What is in {pdf}?")]})
for m in result["messages"]:
    print(f"{m.type}: {str(m.content)[:400]!r}" + (f" tool_calls={[c['name'] for c in m.tool_calls]}" if getattr(m, "tool_calls", None) else ""))
