# betula
Biological Entity Typed Universal Language Architecture - Schemas for common bioinformatics data formats

JSON Schema (Draft 2020-12) definitions for data shapes shared across
[picea](https://github.com/holmrenser/picea),
[react-bio-viz](https://github.com/holmrenser/react-bio-viz),
[acacia](https://github.com/wur-bioinformatics/acacia),
[blastserver](https://github.com/holmrenser/blastserver), and
[iqtreeserver](https://github.com/holmrenser/iqtreeserver),
so those projects can read and write the same JSON regardless of language.

**Docs:** https://holmrenser.github.io/betula/ (rebuilt and redeployed on every push to `main`).

## Layout

- `schema/` — canonical schemas, one file per object (`<name>.schema.json`).
  `schema/core/` holds standalone, individually-`$ref`-able primitives
  (`Identifier`, `Metadata`, `Location`, `SequenceReference`, and the
  DNA/RNA/protein alphabets) rather than
  letting the same shapes get redefined inline in multiple schemas.
- `examples/<name>/valid/` and `examples/<name>/invalid/` — fixtures that must
  pass or fail validation against `schema/<name>.schema.json`. These are the
  conformance tests: any implementation claiming to support a schema should
  validate against the same fixtures.
- `scripts/validate.py` and `scripts/validate.mjs` — reference validators
  (Python/`jsonschema` and Node/`ajv`) that check every example against its
  schema and assert valid examples pass and invalid examples fail. Schemas
  reference each other by `$id` (e.g. `alignment` → `sequence`, `annotation`
  → `core/identifier`), so both scripts load the whole `schema/` tree into one
  registry/instance before resolving any `$ref`.
- `scripts/build_docs.py` — renders `schema/` + `examples/` into the static
  site published at the docs link above (`docs/` itself is a build artifact,
  not committed — see `.gitignore`).
- `bindings/{python,typescript,rust}/` — generated types plus validating
  parsers for each language (see below).

## Language bindings

Generated types plus validating parsers for Python (Pydantic), TypeScript
(ajv), and Rust (typify + the `jsonschema` crate) live in `bindings/`. Each
exposes one type per schema title and a parse function that rejects exactly
what the schema rejects, and runs every `examples/` fixture in CI.

Installation, usage, and API notes are on the docs site's
[Language bindings](https://holmrenser.github.io/betula/bindings.html) page.
Its code is each binding's `examples/usage.*` file, which CI runs, and
fails if a public function or type isn't used in it. Python and Rust also
carry doctests on the parse functions.

### Regenerating

`scripts/generate_bindings.sh` regenerates all three from `schema/` (via a
self-contained bundle, since the schemas' `https://schemas.wur.nl/...`
`$id`s don't resolve over the network). CI reruns it and fails if the
committed output differs, so a schema change must ship with regenerated
bindings. It needs `pip install -r bindings/python/requirements-dev.txt`,
`npm ci` in `bindings/typescript`, and
`cargo install cargo-typify --version 0.8.0 --locked`; generator versions
are pinned so output is reproducible.

## Current schemas

- **`sequence`** — a single sequence record: `{ identifier, sequence }`, or a
  discriminated variant (`DnaSequence` / `RnaSequence` / `ProteinSequence`)
  when the alphabet is known, in which case `sequence` is also validated
  against the matching IUPAC alphabet (`core/dna-alphabet`,
  `core/rna-alphabet`, `core/protein-alphabet` — ambiguity codes included,
  plus `-`/`.` as alignment gap characters). Every current producer (picea,
  react-bio-viz, acacia) omits the discriminator and validates as the plain
  unconstrained shape.
- **`tree`** — a phylogenetic (or other hierarchical) tree node:
  `{ type?, id?, name, length, children }`. `name` carries the raw label as
  produced by the source (a leaf name, or an internal-node label such as a
  bootstrap value or a composite support string like IQ-TREE's `"95.3/88"`)
  so it can be round-tripped; `id` is optional since not every producer
  assigns one at parse time.
- **`annotation`** — a GFF3-derived feature/gene-model node:
  `{ type?, ID, seqid, source, interval_type, start, end, score, strand,
  phase, attributes, children }`, nested (gene → mRNA → exon/CDS). Already
  shared verbatim between picea and react-bio-viz (modulo the new optional
  `type`).
- **`alignment`** — a multiple sequence alignment: either the bare array of
  `sequence` records every current producer (react-bio-viz, acacia) emits,
  or `{ type: "alignment", sequences: [...] }` for producers that want the
  discriminator.
- **`distance-matrix`** — a pairwise distance matrix:
  `{ type?, labels, matrix, labelNames? }`.
- **`blast-result`** — a full BLAST search result matching blastserver's
  parsed-XML shape: hit-level, with nested HSPs (`$defs.Hsp`), cluster
  members (`$defs.HitMember`), and an optional taxonomy forest
  (`$defs.TaxonomyNode`). react-bio-viz's flattened single-row-per-hit
  visualization shape is a derived view of this, not a separate wire format.

`type` above is an optional wire-format discriminator distinct from
`$schema` (which identifies the schema dialect/version, not the object
kind) — see `CHANGELOG.md` for the full rationale per schema.

Not yet modeled: IQ-TREE's `.iqtree` report summary (regex-extracted today,
no stable field set yet), cluster/`cluster_lca` metadata from blastserver's
Postgres schema, and ontology (OBO) terms from picea.

## Versioning

See `CHANGELOG.md`. Short version: every schema's `$id` embeds its own
semver (e.g. `.../sequence/0.5.0/schema.json`), all schemas currently bump
together, and a git tag marks the commit each version was released at.

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

## Building the docs site locally

```bash
pip install -r requirements.txt
python3 scripts/build_docs.py
# then open docs/index.html
```
