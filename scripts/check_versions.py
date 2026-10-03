#!/usr/bin/env python3
"""Fail unless every schema $id and every package manifest carry the same version."""
import json
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def versions() -> dict[str, str]:
    found = {"package.json": json.loads((ROOT / "package.json").read_text())["version"]}
    for path in sorted((ROOT / "schema").rglob("*.schema.json")):
        schema_id = json.loads(path.read_text())["$id"]
        match = re.search(r"/(\d+\.\d+\.\d+)/schema\.json$", schema_id)
        found[str(path.relative_to(ROOT))] = match.group(1) if match else f"<no version in {schema_id}>"
    found["bindings/python/pyproject.toml"] = tomllib.loads(
        (ROOT / "bindings/python/pyproject.toml").read_text()
    )["project"]["version"]
    init = (ROOT / "bindings/python/src/betula_schema/__init__.py").read_text()
    found["bindings/python/src/betula_schema/__init__.py"] = re.search(r'__version__ = "([^"]+)"', init).group(1)
    found["bindings/typescript/package.json"] = json.loads(
        (ROOT / "bindings/typescript/package.json").read_text()
    )["version"]
    found["bindings/rust/Cargo.toml"] = tomllib.loads((ROOT / "bindings/rust/Cargo.toml").read_text())["package"][
        "version"
    ]
    # Hand-written docs pages pin install instructions and examples to a version.
    for page in sorted((ROOT / "docs").glob("*.md")):
        for i, version in enumerate(re.findall(r"(?:@v|tag = \"v|/)(\d+\.\d+\.\d+)\b", page.read_text())):
            found[f"{page.relative_to(ROOT)} (mention {i + 1})"] = version
    return found


def main() -> int:
    found = versions()
    expected = found["package.json"]
    mismatched = {path: v for path, v in found.items() if v != expected}
    if mismatched:
        print(f"Version mismatch (package.json says {expected}):")
        for path, version in mismatched.items():
            print(f"  {path}: {version}")
        return 1
    print(f"All {len(found)} versions are {expected}.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
