#!/usr/bin/env python3
"""Validate examples/<name>/{valid,invalid}/*.json against schema/<name>.schema.json.

Schemas may $ref each other by $id (e.g. alignment -> sequence), so every
schema in schema/ is loaded into one registry before any of them is used.
"""
import json
import sys
from pathlib import Path

import jsonschema
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT202012

ROOT = Path(__file__).resolve().parent.parent
SCHEMA_DIR = ROOT / "schema"
EXAMPLES_DIR = ROOT / "examples"


def load_schemas() -> dict[str, dict]:
    return {
        path.relative_to(SCHEMA_DIR).as_posix().removesuffix(".schema.json"): json.loads(path.read_text())
        for path in sorted(SCHEMA_DIR.rglob("*.schema.json"))
    }


def build_registry(schemas: dict[str, dict]) -> Registry:
    resources = [
        (schema["$id"], Resource.from_contents(schema, default_specification=DRAFT202012))
        for schema in schemas.values()
    ]
    return Registry().with_resources(resources)


def main() -> int:
    schemas = load_schemas()
    registry = build_registry(schemas)
    failures = []

    for name, schema in schemas.items():
        validator = jsonschema.Draft202012Validator(schema, registry=registry)

        example_dir = EXAMPLES_DIR / name
        if not example_dir.is_dir():
            continue

        for valid_path in sorted((example_dir / "valid").glob("*.json")):
            instance = json.loads(valid_path.read_text())
            errors = list(validator.iter_errors(instance))
            if errors:
                failures.append(f"{valid_path}: expected VALID, got errors: {errors[0].message}")

        for invalid_path in sorted((example_dir / "invalid").glob("*.json")):
            instance = json.loads(invalid_path.read_text())
            errors = list(validator.iter_errors(instance))
            if not errors:
                failures.append(f"{invalid_path}: expected INVALID, but it validated")

    if failures:
        print(f"FAILED ({len(failures)}):")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("All examples validated as expected.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
