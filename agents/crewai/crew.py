"""CrewAI tool that wraps the pdfluent Python package.

The tool is plain CrewAI: a BaseTool whose _run calls pdfluent. The script calls
the tool directly (no model needed), builds the crew, and runs it only when a
model key is set.
"""

import os
import sys
from pathlib import Path

import pdfluent
from crewai import Agent, Crew, Task
from crewai.tools import BaseTool
from pydantic import BaseModel, Field

pdf = Path(sys.argv[1] if len(sys.argv) > 1 else Path(__file__).parents[2] / "sample.pdf").resolve()


class PdfInput(BaseModel):
    path: str = Field(description="Path of a local PDF file")


class PdfMarkdownTool(BaseTool):
    name: str = "pdf_to_markdown"
    description: str = "Read a local PDF and return it as markdown, in reading order."
    args_schema: type[BaseModel] = PdfInput

    def _run(self, path: str) -> str:
        doc = pdfluent.open(path)
        try:
            return doc.extract_markdown()
        finally:
            doc.close()


tool = PdfMarkdownTool()

print("--- tool called directly ---")
print(tool.run(path=str(pdf))[:600])

reader = Agent(
    role="PDF reader",
    goal="Answer questions about a PDF from its extracted markdown",
    backstory="You only use the pdf_to_markdown tool and never guess.",
    tools=[tool],
    llm=os.environ.get("CREWAI_MODEL", "gpt-4o-mini"),
    allow_delegation=False,
)
task = Task(
    description=f"List the headings of {pdf} and say how many pages it has.",
    expected_output="A short bullet list.",
    agent=reader,
)
crew = Crew(agents=[reader], tasks=[task])
print("--- crew built:", len(crew.agents), "agent,", len(crew.tasks), "task,", [t.name for t in reader.tools])

if os.environ.get("OPENAI_API_KEY"):
    print(crew.kickoff())
else:
    print("--- crew.kickoff() skipped: requires your own model key (OPENAI_API_KEY, or CREWAI_MODEL for another provider) ---")
