# Django build (UV-managed)

Use UV instead of pip:

```bash
uv sync
uv run python -c "from arxiv_ingest import ingest_stub; print(ingest_stub())"
```
