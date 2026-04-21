from dataclasses import dataclass


@dataclass(frozen=True)
class ArxivPaper:
    paper_id: str
    title: str


def ingest_stub() -> list[ArxivPaper]:
    return [
        ArxivPaper(
            paper_id="arXiv:2305.10403",
            title="GraphRAG: Unlocking LLM discovery on private data",
        )
    ]
