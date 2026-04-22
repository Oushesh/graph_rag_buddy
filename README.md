# graph_rag_buddy

Initial scaffold for comparing Rust ORMs and a Django (UV-managed) setup,
built around a PubMed / ArXiv paper ingestion pipeline.

## Folder layout

```
graph_rag_buddy/
├── rust_build/
│   ├── Cargo.toml              # workspace manifest
│   ├── data/
│   │   ├── pubmed_dataset.json.gz   # compressed source data
│   │   └── pubmed_dataset.json      # generated after running deflate (gitignored)
│   └── dataorm/
│       ├── seaorm/             # SeaORM ingestion crate  ← main entry point
│       ├── diesel/             # Diesel ingestion stub
│       └── rusqlite/           # Rusqlite ingestion stub
└── django_build/               # Django / UV Python setup
```

## Data Pipeline

### Step 1 – Deflate the compressed dataset

The raw PubMed dataset ships as `rust_build/data/pubmed_dataset.json.gz`.
The `seaorm_ingest` binary decompresses it and writes the plain JSON file next
to it inside the same `data/` folder.

**How it works (`dataorm/seaorm/src/`):**

| File | Role |
|------|------|
| `utils.rs` | `pub fn deflate(path) -> Result<()>` – opens the `.gz`, wraps it in a `MultiGzDecoder` + `BufReader`, and streams the bytes into `<same-name>.json` via `std::io::copy`. |
| `main.rs`  | Resolves the data path, calls `utils::deflate`, and prints the result. |

```rust
// main.rs
let data_path = PathBuf::from("data/pubmed_dataset.json.gz");
match utils::deflate(data_path) {
    Ok(()) => println!("Processing Complete"),
    Err(e) => panic!("Error processing file: {}", e),
}
```

**Run the deflate step:**

```bash
# from the rust_build/ directory (so the relative path "data/…" resolves correctly)
cd rust_build
cargo run -p seaorm_ingest
# output: rust_build/data/pubmed_dataset.json
```

### Step 2 – Ingestion into the database (next)

```
Deflated JSON  →  parse records  →  ORM insert  →  Graph DB
```

The ingestion layer (SeaORM / Diesel / Rusqlite) will read the deflated
`pubmed_dataset.json` and upsert paper records into the target database.

## Quick checks

```bash
cd rust_build
cargo test          # runs all workspace unit tests
cargo build         # compile all crates
```

## How it works

Build a knowledge graph from the relationships found in PubMed / ArXiv papers.

**Scaling strategy:**
- GitHub Actions CI via [WarpBuild](https://www.warpbuild.com/compare/github) –
  cuts build times roughly in half compared to standard GitHub-hosted runners.

## Key things to understand

- How raw compressed data is extracted and persisted.
- How many records are ingested and into which store.
- Design the data model from PubMed paper metadata.

## Key outcomes

Compare real-world differences between SeaORM, SQLite (rusqlite), and Diesel
as ORM / query layers for the same ingestion workload.