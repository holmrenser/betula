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

Each binding exposes one type per schema title and a parse function that
rejects anything the JSON Schema rejects. All three run the full `examples/`
fixture set in CI.

**Python** (Pydantic v2) — `pip install ./bindings/python`

```python
import betula

tree = betula.parse_json(betula.Tree, '{"name": "A", "length": 0.1, "children": []}')
seq = betula.parse(betula.Sequence, {"type": "dna-sequence", "identifier": "s1", "sequence": "ACGT"})
isinstance(seq.root, betula.DnaSequence)  # True
```

Use `parse` / `parse_json` rather than `Model.model_validate*`: they run in
Pydantic's strict mode. Lax mode would coerce e.g. `"1"` into an int where
the schema requires a JSON integer.

**TypeScript** (types from `json-schema-to-typescript`, validation by ajv)

```ts
import { parse, parseJson, is, type Tree } from "betula";

const tree: Tree = parseJson("Tree", text);    // throws BetulaValidationError
if (is("Sequence", data) && "type" in data && data.type === "dna-sequence") {
  data.sequence; // narrowed to DnaSequence
}
```

**Rust** (types from typify, validation by the `jsonschema` crate)

```rust
let tree: betula::Tree = betula::parse_str(r#"{"name": "A", "length": 0.1, "children": []}"#)?;
```

Use `betula::parse` / `parse_str` rather than `serde_json::from_*`: typify
doesn't enforce every keyword (e.g. `minItems`), so these validate against
the canonical schema first, then deserialize. Serializing and re-parsing
yields an equal value, but empty optional arrays are omitted on output.

R isn't covered: there's no JSON-Schema-to-R code generator comparable to
the above. From R, read with `jsonlite` and validate with the
`jsonvalidate` package against the self-contained bundle from
`python3 scripts/bundle_schema.py` (the individual `schema/` files `$ref`
each other by URIs that don't resolve over the network).

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
