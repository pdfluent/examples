"""Semantic Kernel (Python) plugin over the pdfluent package.

The plugin is a plain class with @kernel_function methods. The script registers
it, prints the function metadata a model would see, and invokes the functions
through the kernel. That path needs no model. Handing the kernel to a chat
completion service with function calling turned on (see the README) requires
your own model key.
"""

import asyncio
import sys
from pathlib import Path
from typing import Annotated

import pdfluent
from semantic_kernel import Kernel
from semantic_kernel.functions import kernel_function

pdf = Path(sys.argv[1] if len(sys.argv) > 1 else Path(__file__).parents[2] / "sample.pdf").resolve()


class PdfPlugin:
    @kernel_function(name="pdf_markdown", description="Read a local PDF and return it as markdown.")
    def pdf_markdown(self, path: Annotated[str, "Path of a local PDF file"]) -> str:
        doc = pdfluent.open(path)
        try:
            return doc.extract_markdown()
        finally:
            doc.close()

    @kernel_function(name="pdf_search", description="Return the 0-based pages of a PDF that contain a phrase.")
    def pdf_search(
        self,
        path: Annotated[str, "Path of a local PDF file"],
        query: Annotated[str, "Phrase to look for"],
    ) -> str:
        doc = pdfluent.open(path)
        try:
            hits = [i for i in range(doc.page_count) if query.lower() in doc.get_page_text(i).lower()]
        finally:
            doc.close()
        return f"pages containing {query!r}: {hits}"


async def main() -> None:
    kernel = Kernel()
    kernel.add_plugin(PdfPlugin(), plugin_name="pdf")

    for f in kernel.get_full_list_of_function_metadata():
        print("function:", f"{f.plugin_name}-{f.name}", [p.name for p in f.parameters])

    md = await kernel.invoke(plugin_name="pdf", function_name="pdf_markdown", path=str(pdf))
    print("--- pdf-pdf_markdown ---")
    print(str(md)[:600])
    hit = await kernel.invoke(plugin_name="pdf", function_name="pdf_search", path=str(pdf), query="multi-page")
    print("--- pdf-pdf_search ---")
    print(hit)
    print("--- chat completion with function calling skipped: requires your own model key ---")


asyncio.run(main())
