# betula-schema (TypeScript)

TypeScript types and runtime parsers for the [betula](https://holmrenser.github.io/betula/)
JSON Schemas: sequences, alignments, phylogenetic trees, gene annotations, distance matrices,
and BLAST results.

```ts
import { parseJson, type Tree } from "betula-schema";

const tree: Tree = parseJson("Tree", '{"name": "A", "length": 0.1, "children": []}');
```

Every schema title is an exported type, and `parse(kind, data)` / `parseJson(kind, text)` return
that type. They validate with ajv against the same JSON Schemas, so they reject exactly what the
schema rejects and throw `BetulaValidationError` when it does. `is(kind, data)` is the
non-throwing type guard.

See [Getting started](https://holmrenser.github.io/betula/getting-started/) for the full API tour,
and the [schema pages](https://holmrenser.github.io/betula/schemas/) for each type. The types are
generated from the schemas; the version matches the schema `$id` versions.
