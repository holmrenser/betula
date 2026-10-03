"""Parsing betula JSON with the Python bindings. Run in CI; the asserts are checked."""

import json

from betula_schema import (
    Alignment,
    RnaSequence,
    Sequence,
    Tree,
    UnwrappedAlignment,
    ValidationError,
    parse,
    parse_json,
)

# Parse JSON text into a model. Every schema title is a class in betula_schema.
tree = parse_json(
    Tree,
    '{"name": "root", "length": 0, "children": [{"name": "A", "length": 0.1, "children": []}]}',
)
assert [child.name for child in tree.children] == ["A"]

# Or validate data you've already decoded. Parsing is strict: "0.1" is a
# string, not a number, so this is rejected rather than coerced.
try:
    parse(Tree, {"name": "A", "length": "0.1", "children": []})
except ValidationError as err:
    assert err.error_count() == 1
else:
    raise AssertionError("expected a ValidationError")

# Union kinds (Sequence, Alignment) are RootModels; the variant that matched
# is `.root`. Constrained strings (Identifier, the alphabets) are RootModels
# too, so their plain value is also `.root`.
seq = parse(Sequence, {"type": "rna-sequence", "identifier": "s1", "sequence": "ACGU"})
match seq.root:
    case RnaSequence(identifier=identifier, sequence=letters):
        assert (identifier.root, letters.root) == ("s1", "ACGU")
    case other:
        raise AssertionError(f"unexpected variant {other!r}")

# A bare-array alignment matches UnwrappedAlignment, whose list is again `.root`.
alignment = parse(
    Alignment,
    [{"identifier": "s1", "sequence": "AC-GT"}, {"identifier": "s2", "sequence": "ACTGT"}],
)
assert isinstance(alignment.root, UnwrappedAlignment)
assert [s.root.identifier.root for s in alignment.root.root] == ["s1", "s2"]

# ValidationError lists every problem, with its location.
try:
    parse_json(Sequence, '{"type": "dna-sequence", "identifier": "s2", "sequence": "ACGU"}')
except ValidationError as err:
    assert any("pattern" in error["msg"] for error in err.errors())
else:
    raise AssertionError("expected a ValidationError")

# Serialize with exclude_unset so optional fields you never set stay out.
assert json.loads(tree.model_dump_json(exclude_unset=True)) == {
    "name": "root",
    "length": 0.0,
    "children": [{"name": "A", "length": 0.1, "children": []}],
}
print("python usage example ok")
