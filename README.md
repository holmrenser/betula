# betula
Biological Entity Typed Universal Language Architecture - Schemas for common bioinformatics data formats

JSON Schema (Draft 2020-12) definitions for data shapes shared across
[picea](https://github.com/holmrenser/picea),
[react-bio-viz](https://github.com/holmrenser/react-bio-viz),
[acacia](https://github.com/wur-bioinformatics/acacia),
[blastserver](https://github.com/holmrenser/blastserver), and
[iqtreeserver](https://github.com/holmrenser/iqtreeserver),
so those projects can read and write the same JSON regardless of language.

## Layout

- `schema/` — canonical schemas, one file per object (`<name>.schema.json`).
- `examples/<name>/valid/` and `examples/<name>/invalid/` — fixtures that must
  pass or fail validation against `schema/<name>.schema.json`. These are the
  conformance tests: any implementation claiming to support a schema should
  validate against the same fixtures.
- `scripts/validate.py` and `scripts/validate.mjs` — reference validators
  (Python/`jsonschema` and Node/`ajv`) that check every example against its
  schema and assert valid examples pass and invalid examples fail.

## Current schemas

- **`sequence`** — a single sequence record: `{ identifier, sequence }`.
- **`tree`** — a phylogenetic (or other hierarchical) tree node:
  `{ id?, name, length, children }`. `name` carries the raw label as produced
  by the source (a leaf name, or an internal-node label such as a bootstrap
  value or a composite support string like IQ-TREE's `"95.3/88"`) so it can be
  round-tripped; `id` is optional since not every producer assigns one at
  parse time.
- **`annotation`** — a GFF3-derived feature/gene-model node:
  `{ ID, seqid, source, interval_type, start, end, score, strand, phase,
  attributes, children }`, nested (gene → mRNA → exon/CDS). Already shared
  verbatim between picea and react-bio-viz.
- **`alignment`** — a multiple sequence alignment: an array of `sequence`
  records (cross-references the `sequence` schema by `$id`).
- **`distance-matrix`** — a pairwise distance matrix:
  `{ labels, matrix, labelNames? }`.
- **`blast-result`** — a full BLAST search result matching blastserver's
  parsed-XML shape: hit-level, with nested HSPs (`$defs.Hsp`), cluster
  members (`$defs.HitMember`), and an optional taxonomy forest
  (`$defs.TaxonomyNode`). react-bio-viz's flattened single-row-per-hit
  visualization shape is a derived view of this, not a separate wire format.

Schemas may reference each other by `$id` (e.g. `alignment` → `sequence`), so
any tool consuming them needs to load the whole `schema/` directory into one
registry/instance before resolving `$ref`s — see `scripts/validate.*` for a
reference implementation in both languages.

Not yet modeled: IQ-TREE's `.iqtree` report summary (regex-extracted today,
no stable field set yet), cluster/`cluster_lca` metadata from blastserver's
Postgres schema, and ontology (OBO) terms from picea.

## Running the conformance tests

```bash
# Python
pip install -r requirements.txt
python3 scripts/validate.py

# Node
npm install
npm run validate
```

Both run in CI on every push/PR (`.github/workflows/validate.yml`).
