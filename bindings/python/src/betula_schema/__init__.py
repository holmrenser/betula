"""Pydantic bindings for the betula schemas.

`betula_schema.models` is generated from the JSON Schemas; don't edit it by hand.

Parse with `parse_json` / `parse` rather than calling `model_validate*`
directly: they validate in Pydantic's strict mode, which is what makes these
models agree with the JSON Schema. Lax mode would coerce, e.g., the string
"1" into an int where the schema requires a JSON integer.
"""

from typing import Any, TypeVar

from pydantic import BaseModel, ValidationError

from betula_schema.models import (
    Alignment,
    Annotation,
    BlastResult,
    DistanceMatrix,
    DnaAlphabet,
    DnaSequence,
    Hit,
    HitMember,
    Hsp,
    Identifier,
    Location,
    Metadata,
    ProteinAlphabet,
    ProteinSequence,
    RnaAlphabet,
    RnaSequence,
    Sequence,
    SequenceReference,
    TaxonomyNode,
    Tree,
    UntypedSequence,
    UnwrappedAlignment,
    WrappedAlignment,
)

__version__ = "0.5.0"

M = TypeVar("M", bound=BaseModel)


def parse_json(model: type[M], data: str | bytes) -> M:
    """Parse JSON text into `model`, raising `ValidationError` if it doesn't conform.

    >>> tree = parse_json(Tree, '{"name": "A", "length": 0.1, "children": []}')
    >>> tree.name
    'A'
    >>> parse_json(Tree, '{"name": "A", "length": -1, "children": []}')  # doctest: +IGNORE_EXCEPTION_DETAIL
    Traceback (most recent call last):
    ValidationError: 1 validation error for Tree
    """
    return model.model_validate_json(data, strict=True)


def parse(model: type[M], data: Any) -> M:
    """Validate already-decoded JSON (dicts/lists/scalars) into `model`.

    Strict, like the schema: a numeric string is not coerced to a number.

    >>> parse(Tree, {"name": "A", "length": 0.1, "children": []}).length
    0.1
    >>> parse(Tree, {"name": "A", "length": "0.1", "children": []})  # doctest: +IGNORE_EXCEPTION_DETAIL
    Traceback (most recent call last):
    ValidationError: 1 validation error for Tree
    """
    return model.model_validate(data, strict=True)


__all__ = [
    "Alignment",
    "Annotation",
    "BlastResult",
    "DistanceMatrix",
    "DnaAlphabet",
    "DnaSequence",
    "Hit",
    "HitMember",
    "Hsp",
    "Identifier",
    "Location",
    "Metadata",
    "ProteinAlphabet",
    "ProteinSequence",
    "RnaAlphabet",
    "RnaSequence",
    "Sequence",
    "SequenceReference",
    "TaxonomyNode",
    "Tree",
    "UntypedSequence",
    "UnwrappedAlignment",
    "ValidationError",
    "WrappedAlignment",
    "parse",
    "parse_json",
]
