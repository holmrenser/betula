# betula-schema (Python)

Pydantic v2 models and strict parsers for the [betula](https://holmrenser.github.io/betula/)
JSON Schemas: sequences, alignments, phylogenetic trees, gene annotations, distance matrices,
and BLAST results.

```python
from betula_schema import Tree, parse_json

tree = parse_json(Tree, '{"name": "A", "length": 0.1, "children": []}')
```

Every schema title is a model in `betula_schema`. Use `parse` / `parse_json` rather than
`Model.model_validate*`: they validate in Pydantic's strict mode, which is what makes them reject
exactly what the schema rejects.

See [Getting started](https://holmrenser.github.io/betula/getting-started/) for the full API tour,
and the [schema pages](https://holmrenser.github.io/betula/schemas/) for each type. The models are
generated from the schemas; the version matches the schema `$id` versions.
