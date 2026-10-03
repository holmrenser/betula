# betula

**B**iological **E**ntity **T**yped **U**niversal **L**anguage **A**rchitecture: JSON Schemas for
common bioinformatics data (sequences, alignments, phylogenetic trees, gene annotations, distance
matrices, and BLAST results), so tools written in different languages can read and write the same
JSON.

## Why

The projects below each defined these shapes on their own, and the definitions drifted. A sequence
record is `{header, sequence}` in picea and react-bio-viz but `{identifier, sequence}` in acacia;
tree support values are packed into node names in different encodings; BLAST hits nest multiple
HSPs in blastserver but are flattened to one row in react-bio-viz. Nothing checked any of it.

betula fixes one definition per shape as a JSON Schema, the language-neutral source of truth, and
derives everything else from it:

- **[Schemas](schemas.md)**: one per shape, built from shared primitives such as identifiers and
  IUPAC sequence alphabets.
- **Conformance fixtures**: valid and invalid example documents for each schema. Any
  implementation must accept and reject exactly the same ones.
- **Language bindings**: generated types with validating parsers, tested against every fixture.
  See [Getting started](getting-started.md).

Schemas are versioned in their `$id` (e.g. `.../sequence/0.5.0/schema.json`) and released as git
tags; see the [changelog](https://github.com/holmrenser/betula/blob/main/CHANGELOG.md). The raw
schemas and fixtures are also served from this site, under `schema/` and `examples/`.

## Implementations

All three are released as `betula-schema`, versioned in lockstep with the schemas (the first
release is still to come; see [Getting started](getting-started.md) for installing in the meantime).

| Language   | Package                         | Source                                                                                     | Built on                                  |
| ---------- | ------------------------------- | ------------------------------------------------------------------------------------------ | ----------------------------------------- |
| Python     | `betula-schema` (PyPI)          | [`bindings/python`](https://github.com/holmrenser/betula/tree/main/bindings/python)         | Pydantic v2, via datamodel-code-generator |
| TypeScript | `betula-schema` (npm)           | [`bindings/typescript`](https://github.com/holmrenser/betula/tree/main/bindings/typescript) | json-schema-to-typescript types, ajv      |
| Rust       | `betula-schema` (crates.io)     | [`bindings/rust`](https://github.com/holmrenser/betula/tree/main/bindings/rust)             | typify types, the `jsonschema` crate      |

## Users

The schemas are derived from the JSON these projects produce and consume today. None of them
depends on betula yet.

| Project                                                       | Language   | Shapes                                                       |
| ------------------------------------------------------------- | ---------- | ------------------------------------------------------------ |
| [picea](https://github.com/holmrenser/picea)                  | Python     | trees, sequences, alignments, gene annotations               |
| [react-bio-viz](https://github.com/holmrenser/react-bio-viz)  | TypeScript | trees, alignments, gene models, distance matrices, BLAST hits |
| [acacia](https://github.com/wur-bioinformatics/acacia)        | TypeScript | alignments, trees, distance matrices                         |
| [blastserver](https://github.com/holmrenser/blastserver)      | TypeScript | BLAST results                                                |
| [iqtreeserver](https://github.com/holmrenser/iqtreeserver)    | TypeScript | trees                                                        |
