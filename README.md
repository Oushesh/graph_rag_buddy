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

## How it works? 
   Build a Graph from Relationship from the PubArxiV Papers givrn
   
   How to scale those relationships and the papers found? 
   
   Github Actions: https://www.warpbuild.com/compare/github
   Using Warpbuild to double cut the costs run down to half and so
   reduce the costs of building to almost half.
   
   
## Key Things to understand
   How raw data is converted is extracted and saved in the DB.
   PubMed data --> 
   How many data are we ingesting and where and how? 

## Design the DataModel from the Paper of PubMed: 
   
## Key Outcomes: 
   Learn the difference and Implementation of the different 
   models of SeaORM and SQLITE and diesel data systems.