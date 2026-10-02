# Changelog

All schemas in this repository version together (see "Versioning policy"
below); this changelog tracks the schema set as a whole.

## 0.4.0

- Added `schema/core/dna-alphabet`, `core/rna-alphabet`, `core/protein-alphabet`:
  IUPAC-code `pattern` constraints (ambiguity codes included — R/Y/S/W/K/M/
  B/D/H/V/N for nucleotides; B/Z/X/J/U/O plus the standard 20 for protein,
  which together cover all 26 letters). Each also allows `-`/`.` as
  alignment gap/missing-data characters, since `Sequence` doubles as an
  alignment row (see `alignment`'s `oneOf`).
- `sequence`'s `DnaSequence`/`RnaSequence`/`ProteinSequence` variants now
  `$ref` the matching alphabet instead of an unconstrained string, e.g. a
  `dna-sequence` containing `U` (RNA-only) or digits now fails validation.
  `UntypedSequence` is deliberately left unconstrained — its whole point is
  that the alphabet isn't known.
- This is the first schema-set change that's stricter in a way that could
  reject previously-valid data for the (currently nonexistent) adopters of
  the optional typed-sequence variants — flagged here per the "breaking
  change" definition below, even though nothing in picea/react-bio-viz/
  acacia emits typed sequences yet.
- Bumped every schema's `$id` version segment from `0.3.0` to `0.4.0` in
  lockstep (see policy below).

## 0.3.0

- Added the optional wire-format `type` discriminator to `annotation`
  (`"type": "annotation"`, distinct from `interval_type`'s GFF3 feature
  kind) and `alignment`.
- `alignment` is now a discriminated union (`oneOf` over `WrappedAlignment`
  `{ type: "alignment", sequences: [...] }` and `UnwrappedAlignment`, the
  bare array every current producer emits), mirroring how `sequence`
  already handles its own optional discriminator. The bare-array shape
  remains valid and unchanged.
- Bumped every schema's `$id` version segment from `0.2.0` to `0.3.0` in
  lockstep (see policy below).

## 0.2.0

- Added `schema/core/` — standalone, individually-referenceable primitives:
  `Identifier`, `Metadata`, `Location`, `SequenceReference`. `annotation` and
  `distance-matrix` now `$ref` `Identifier`/`Metadata` instead of redefining
  the same shape inline.
- `sequence` is now a discriminated union (`oneOf` over `DnaSequence` /
  `RnaSequence` / `ProteinSequence` / `UntypedSequence`), keyed by an
  optional `type` field. Every existing producer (picea, react-bio-viz,
  acacia) omits `type` today and still validates, via `UntypedSequence`.
- Added an optional wire-format `type` discriminator (e.g. `"type": "tree"`)
  to `tree`, `distance-matrix`, and `blast-result`, separate from `$schema`
  (which identifies the schema dialect/version, not the object kind).
  Deliberately *not* added to `alignment` (a bare array in every current
  producer; wrapping it would break that shape for no current consumer) or
  `annotation` (GFF3's `interval_type` already serves as the discriminator).
- Bumped every schema's `$id` version segment from `0.1.0` to `0.2.0` in
  lockstep (see policy below).

## 0.1.0

Initial schema set: `sequence`, `tree`, `annotation`, `alignment`,
`distance-matrix`, `blast-result`, derived from the actual current JSON
output of picea, react-bio-viz, acacia, blastserver, and iqtreeserver.
Conformance fixtures (`examples/`) and reference validators
(`scripts/validate.py`, `scripts/validate.mjs`) added alongside.

## Versioning policy

- Each schema's `$id` embeds its own semver, e.g.
  `https://schemas.wur.nl/betula/sequence/0.4.0/schema.json`. Changing a
  schema's `$id` version is how consumers notice the contract changed; the
  URI is the schema's identity, not its location in this repo.
- While the project is young, all schemas bump together, so a `$id` version
  always tells you which commit/tag produced it. Once the schema set
  stabilizes, schemas may version independently (e.g. `tree` reaching `1.0`
  while `blast-result` is still `0.x`) — nothing here prevents that split
  later; it's deferred because it isn't needed yet.
- A git tag (e.g. `v0.4.0`) marks the commit each schema-set version was
  released at. Tags are the source of truth for "what did a given version
  actually contain"; `main` can move ahead of the latest tag.
- Breaking change = an existing valid document stops validating. Additive
  change (new optional field, a previously-required field becoming looser)
  does not require a major bump pre-`1.0`.
