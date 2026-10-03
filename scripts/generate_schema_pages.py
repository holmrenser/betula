#!/usr/bin/env python3
"""Write a MyST Markdown page per schema into docs/schemas/, plus docs/schemas.md.

mystmd (see docs/myst.yml) turns these and the hand-written pages into the
site. Example fixtures and raw schemas are pulled in with literalinclude
rather than copied, so a renamed or deleted file fails scripts/build_docs.sh.
"""
import json
import os
import shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SCHEMA_DIR = ROOT / "schema"
EXAMPLES_DIR = ROOT / "examples"
DOCS_DIR = ROOT / "docs"
PAGES_DIR = DOCS_DIR / "schemas"

MARKDOWN_SPECIAL = set("\\`*_[]<>$|~#@")


def load_schemas() -> dict[str, dict]:
    return {
        path.relative_to(SCHEMA_DIR).as_posix().removesuffix(".schema.json"): json.loads(path.read_text())
        for path in sorted(SCHEMA_DIR.rglob("*.schema.json"))
    }


def md(text: str) -> str:
    """Escape prose so schema descriptions render literally."""
    return "".join(f"\\{c}" if c in MARKDOWN_SPECIAL else c for c in text)


def schema_label(name: str) -> str:
    return "schema-" + name.replace("/", "-")


class Linker:
    def __init__(self, schemas: dict[str, dict]):
        self.schemas = schemas
        self.id_to_name = {schema["$id"]: name for name, schema in schemas.items()}

    def link(self, ref: str, current: str) -> str:
        if ref == "#":
            return f"[{self.schemas[current]['title']}](#{schema_label(current)})"
        if ref.startswith("#/$defs/"):
            def_name = ref.removeprefix("#/$defs/")
            return f"[{def_name}](#def-{def_name})"
        base, _, fragment = ref.partition("#")
        target = self.id_to_name[base]
        if fragment.startswith("/$defs/"):
            def_name = fragment.removeprefix("/$defs/")
            return f"[{def_name}](#def-{def_name})"
        return f"[{self.schemas[target]['title']}](#{schema_label(target)})"


def describe(node: dict, current: str, linker: Linker) -> str:
    if "$ref" in node:
        return linker.link(node["$ref"], current)
    if "const" in node:
        return f"`{json.dumps(node['const'])}`"
    if "enum" in node:
        return "one of " + ", ".join(f"`{json.dumps(v)}`" for v in node["enum"])
    for combinator in ("oneOf", "anyOf"):
        if combinator in node:
            return " | ".join(describe(sub, current, linker) for sub in node[combinator])
    t = node.get("type")
    if isinstance(t, list):
        return " or ".join(f"`{x}`" for x in t)
    if t == "array":
        items = node.get("items")
        return f"array of {describe(items, current, linker)}" if items else "array"
    if t == "object" and "properties" not in node and isinstance(node.get("additionalProperties"), dict):
        return f"object of {describe(node['additionalProperties'], current, linker)}"
    if not t:
        return "any"
    constraints = []
    if "minimum" in node:
        constraints.append(f"≥ {node['minimum']}")
    if "maximum" in node:
        constraints.append(f"≤ {node['maximum']}")
    for keyword in ("minLength", "minItems"):
        if keyword in node:
            constraints.append(f"{keyword} {node[keyword]}")
    if "pattern" in node:
        constraints.append(f"pattern `{node['pattern']}`")
    return f"`{t}`" + (f" ({', '.join(constraints)})" if constraints else "")


def properties_table(obj: dict, current: str, linker: Linker) -> list[str]:
    properties = obj.get("properties", {})
    if not properties:
        return []
    required = set(obj.get("required", []))
    lines = [":::{list-table}", ":header-rows: 1", "* - Field", "  - Type", "  - Required", "  - Description"]
    for name, prop in properties.items():
        lines += [
            f"* - `{name}`",
            f"  - {describe(prop, current, linker)}",
            f"  - {'required' if name in required else 'optional'}",
            f"  - {md(prop.get('description', ''))}",
        ]
    lines.append(":::")
    if obj.get("additionalProperties") is False:
        lines += ["", "No properties beyond those listed are allowed."]
    return lines


def type_body(node: dict, current: str, linker: Linker) -> list[str]:
    t = node.get("type")
    if t == "object":
        return properties_table(node, current, linker)
    if t == "array":
        extra = f" (minItems {node['minItems']})" if "minItems" in node else ""
        return [f"Array of {describe(node.get('items', {}), current, linker)}{extra}."]
    if "oneOf" in node:
        return ["One of:", ""] + [f"- {describe(sub, current, linker)}" for sub in node["oneOf"]]
    return [describe(node, current, linker)]


def include(path: Path, page: Path) -> list[str]:
    rel = os.path.relpath(path, page.parent).replace(os.sep, "/")
    return [f"```{{literalinclude}} {rel}", ":language: json", "```"]


def schema_page(name: str, schema: dict, linker: Linker) -> tuple[Path, str]:
    page = PAGES_DIR / f"{name}.md"
    lines = [f"({schema_label(name)})=", f"# {schema['title']}", "", f"`{schema['$id']}`", ""]
    lines += [md(schema.get("description", "")), ""]
    lines += type_body(schema, name, linker)

    defs = schema.get("$defs", {})
    if defs:
        lines += ["", "## Definitions"]
        for def_name, def_schema in defs.items():
            lines += ["", f"(def-{def_name})=", f"### {def_name}", ""]
            if def_schema.get("description"):
                lines += [md(def_schema["description"]), ""]
            lines += type_body(def_schema, name, linker)

    for kind, heading in (("valid", "Valid examples"), ("invalid", "Invalid examples")):
        files = sorted((EXAMPLES_DIR / name / kind).glob("*.json"))
        if files:
            lines += ["", f"## {heading}"]
            for f in files:
                lines += [""] + include(f, page)

    source = SCHEMA_DIR / f"{name}.schema.json"
    lines += ["", ":::{dropdown} Schema source"] + include(source, page) + [":::", ""]
    return page, "\n".join(lines)


def overview(schemas: dict[str, dict]) -> str:
    def table(names: list[str]) -> list[str]:
        lines = [":::{list-table}", ":header-rows: 1", "* - Schema", "  - Description"]
        for name in names:
            first = schemas[name].get("description", "").split(". ")[0].rstrip(".") + "."
            lines += [f"* - [{schemas[name]['title']}](#{schema_label(name)})", f"  - {md(first)}"]
        return lines + [":::"]

    core = [n for n in schemas if n.startswith("core/")]
    main = [n for n in schemas if not n.startswith("core/")]
    return "\n".join(
        ["# Schemas", "", "One page per schema, each with its properties, definitions, and the",
         "conformance fixtures every binding is tested against.", ""]
        + table(main)
        + ["", "## Core primitives", "", "Reusable shapes the schemas above `$ref`.", ""]
        + table(core)
        + [""]
    )


def main() -> None:
    schemas = load_schemas()
    linker = Linker(schemas)
    if PAGES_DIR.exists():
        shutil.rmtree(PAGES_DIR)
    for name, schema in schemas.items():
        page, text = schema_page(name, schema, linker)
        page.parent.mkdir(parents=True, exist_ok=True)
        page.write_text(text)
    (DOCS_DIR / "schemas.md").write_text(overview(schemas))
    print(f"Wrote {len(schemas)} schema pages and schemas.md")


if __name__ == "__main__":
    main()
