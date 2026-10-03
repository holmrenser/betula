#!/usr/bin/env python3
"""Set every version in the repo to X.Y.Z and turn "## Unreleased" into "## X.Y.Z".

Covers the schema $ids (and the $refs between them), all package manifests
and lockfiles, and the version mentions in the hand-written docs pages: the
same set scripts/check_versions.py verifies. Afterwards, run
scripts/generate_bindings.sh, since generated code embeds the schema $ids.

Usage: scripts/bump_version.py X.Y.Z
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def sub_once(path: Path, pattern: str, replacement: str) -> None:
    text = path.read_text()
    new, count = re.subn(pattern, replacement, text, count=1, flags=re.M)
    if count != 1:
        raise SystemExit(f"{path}: no match for {pattern!r}")
    path.write_text(new)


def set_json_version(path: Path, new: str, lockfile: bool = False) -> None:
    data = json.loads(path.read_text())
    data["version"] = new
    if lockfile:
        data["packages"][""]["version"] = new
    path.write_text(json.dumps(data, indent=2) + "\n")


def main() -> None:
    if len(sys.argv) != 2 or not re.fullmatch(r"\d+\.\d+\.\d+", sys.argv[1]):
        raise SystemExit("usage: scripts/bump_version.py X.Y.Z")
    new = sys.argv[1]
    old = json.loads((ROOT / "package.json").read_text())["version"]
    if new == old:
        raise SystemExit(f"already at {old}")

    for manifest in ("package.json", "bindings/typescript/package.json"):
        set_json_version(ROOT / manifest, new)
    for lockfile in ("package-lock.json", "bindings/typescript/package-lock.json"):
        set_json_version(ROOT / lockfile, new, lockfile=True)

    sub_once(ROOT / "bindings/python/pyproject.toml", rf'^version = "{re.escape(old)}"$', f'version = "{new}"')
    sub_once(ROOT / "bindings/python/src/betula_schema/__init__.py", rf'^__version__ = "{re.escape(old)}"$',
             f'__version__ = "{new}"')
    sub_once(ROOT / "bindings/rust/Cargo.toml", rf'^version = "{re.escape(old)}"$', f'version = "{new}"')
    sub_once(ROOT / "bindings/rust/Cargo.lock", rf'^(name = "betula-schema"\nversion = )"{re.escape(old)}"$',
             rf'\g<1>"{new}"')

    for schema in sorted((ROOT / "schema").rglob("*.schema.json")):
        schema.write_text(schema.read_text().replace(f"/{old}/schema.json", f"/{new}/schema.json"))
    for page in sorted((ROOT / "docs").glob("*.md")):
        page.write_text(re.sub(rf"(?<=[/v]){re.escape(old)}\b", new, page.read_text()))

    changelog = ROOT / "CHANGELOG.md"
    text = changelog.read_text()
    if "\n## Unreleased\n" in text:
        changelog.write_text(text.replace("\n## Unreleased\n", f"\n## {new}\n", 1))
    elif f"\n## {new}\n" not in text:
        print(f"note: CHANGELOG.md has neither '## Unreleased' nor '## {new}'; add release notes before tagging")

    print(f"Bumped {old} -> {new}. Next: scripts/generate_bindings.sh && python3 scripts/check_versions.py")


if __name__ == "__main__":
    main()
