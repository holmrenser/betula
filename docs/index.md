# betula

**B**iological **E**ntity **T**yped **U**niversal **L**anguage **A**rchitecture: JSON Schemas
(Draft 2020-12) for the data shapes shared across
[picea](https://github.com/holmrenser/picea),
[react-bio-viz](https://github.com/holmrenser/react-bio-viz),
[acacia](https://github.com/wur-bioinformatics/acacia),
[blastserver](https://github.com/holmrenser/blastserver), and
[iqtreeserver](https://github.com/holmrenser/iqtreeserver),
so those projects can read and write the same JSON regardless of language.

- [Schemas](schemas.md): one page per schema, with its properties and the valid and invalid
  example documents that define its conformance tests.
- [Language bindings](bindings.md): generated types and validating parsers for Python,
  TypeScript, and Rust.

The raw schema files are also served from this site under `schema/`, e.g.
`schema/sequence.schema.json`, and the fixtures under `examples/`.

Schemas are versioned in their `$id` (e.g. `.../sequence/0.5.0/schema.json`) and released
as git tags; see the [changelog](https://github.com/holmrenser/betula/blob/main/CHANGELOG.md)
for what changed and the versioning policy.
