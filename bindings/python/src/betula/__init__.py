"""Pydantic bindings for the betula schemas.

`betula.models` is generated from the JSON Schemas; don't edit it by hand.

Parse with `parse_json` / `parse` rather than calling `model_validate*`
directly: they validate in Pydantic's strict mode, which is what makes these
models agree with the JSON Schema. Lax mode would coerce, e.g., the string
"1" into an int where the schema requires a JSON integer.
"""

from typing import Any, TypeVar

from pydantic import BaseModel, ValidationError

from betula.models import (
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
    """Parse JSON text into `model`, raising `ValidationError` if it doesn't conform."""
    return model.model_validate_json(data, strict=True)


def parse(model: type[M], data: Any) -> M:
    """Validate already-decoded JSON (dicts/lists/scalars) into `model`."""
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
