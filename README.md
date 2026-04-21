# graph_rag_buddy

Initial scaffold for comparing Rust ORMs and a Django (UV-managed) setup.

## Folder layout

- `rust_build/` – Rust workspace
  - `dataorm/seaorm/` – SeaORM ingestion simulation stub
  - `dataorm/diesel/` – Diesel ingestion simulation stub
  - `dataorm/rusqlite/` – Rusqlite ingestion simulation stub
- `django_build/` – Django-oriented Python setup managed via UV (`pyproject.toml`)

## Quick checks

```bash
cd rust_build
cargo test
```
