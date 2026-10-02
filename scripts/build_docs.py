#!/usr/bin/env python3
"""Render schema/**/*.schema.json + examples/ into a static docs site in docs/.

No templating dependency on purpose: this project otherwise only needs
jsonschema/ajv, and pulling in a whole docs-generator package for six
schemas isn't worth the extra surface. Keep this file boring.
"""
import html
import json
import shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SCHEMA_DIR = ROOT / "schema"
EXAMPLES_DIR = ROOT / "examples"
OUT_DIR = ROOT / "docs"


def load_schemas() -> dict[str, dict]:
    return {
        path.relative_to(SCHEMA_DIR).as_posix().removesuffix(".schema.json"): json.loads(path.read_text())
        for path in sorted(SCHEMA_DIR.rglob("*.schema.json"))
    }


def page_filename(name: str) -> str:
    return name.replace("/", "-") + ".html"


def esc(s: str) -> str:
    return html.escape(str(s), quote=True)


class Linker:
    def __init__(self, schemas: dict[str, dict]):
        self.schemas = schemas
        self.id_to_name = {schema["$id"]: name for name, schema in schemas.items()}

    def resolve(self, ref: str, current_name: str) -> tuple[str, str]:
        """Return (href, label) for a $ref, relative to current_name's page."""
        if ref == "#":
            return f"{page_filename(current_name)}", self.schemas[current_name].get("title", current_name)
        if ref.startswith("#/$defs/"):
            def_name = ref.removeprefix("#/$defs/")
            def_schema = self.schemas[current_name].get("$defs", {}).get(def_name, {})
            return f"{page_filename(current_name)}#def-{def_name}", def_schema.get("title", def_name)
        base, _, fragment = ref.partition("#")
        target_name = self.id_to_name.get(base)
        if target_name is None:
            return ref, ref  # external/unknown ref, link as-is
        title = self.schemas[target_name].get("title", target_name)
        if fragment.startswith("/$defs/"):
            def_name = fragment.removeprefix("/$defs/")
            def_schema = self.schemas[target_name].get("$defs", {}).get(def_name, {})
            return f"{page_filename(target_name)}#def-{def_name}", def_schema.get("title", def_name)
        return page_filename(target_name), title


def describe_type(node: dict, current_name: str, linker: Linker) -> str:
    if "$ref" in node:
        href, label = linker.resolve(node["$ref"], current_name)
        return f'<a href="{esc(href)}">{esc(label)}</a>'
    if "const" in node:
        return f'<code>{esc(json.dumps(node["const"]))}</code>'
    if "enum" in node:
        return "one of " + ", ".join(f"<code>{esc(json.dumps(v))}</code>" for v in node["enum"])
    for combinator in ("oneOf", "anyOf"):
        if combinator in node:
            parts = [describe_type(sub, current_name, linker) for sub in node[combinator]]
            return " | ".join(parts)
    t = node.get("type")
    if isinstance(t, list):
        return " or ".join(f"<code>{esc(x)}</code>" for x in t)
    if t == "array":
        items = node.get("items", {})
        return f"array of {describe_type(items, current_name, linker)}" if items else "array"
    if t == "object" and "properties" not in node and isinstance(node.get("additionalProperties"), dict):
        return f"object of {describe_type(node['additionalProperties'], current_name, linker)}"
    if t:
        constraints = []
        if "minimum" in node:
            constraints.append(f"&ge;{node['minimum']}")
        if "maximum" in node:
            constraints.append(f"&le;{node['maximum']}")
        if "minLength" in node:
            constraints.append(f"minLength {node['minLength']}")
        if "minItems" in node:
            constraints.append(f"minItems {node['minItems']}")
        if "pattern" in node:
            constraints.append(f"pattern <code>{esc(node['pattern'])}</code>")
        suffix = f" ({', '.join(constraints)})" if constraints else ""
        return f"<code>{esc(t)}</code>{suffix}"
    return "any"


def render_properties_table(obj: dict, current_name: str, linker: Linker) -> str:
    properties = obj.get("properties", {})
    if not properties:
        return ""
    required = set(obj.get("required", []))
    rows = []
    for prop_name, prop_schema in properties.items():
        type_desc = describe_type(prop_schema, current_name, linker)
        description = prop_schema.get("description", "")
        req = "required" if prop_name in required else "optional"
        rows.append(
            f"<tr><td><code>{esc(prop_name)}</code></td><td>{type_desc}</td>"
            f'<td class="req-{req}">{req}</td><td>{esc(description)}</td></tr>'
        )
    if obj.get("additionalProperties") is False:
        note = "<p class=\"note\">No properties beyond those listed are allowed.</p>"
    else:
        note = ""
    return (
        '<table class="props"><thead><tr><th>Field</th><th>Type</th><th>Required</th>'
        f"<th>Description</th></tr></thead><tbody>{''.join(rows)}</tbody></table>{note}"
    )


def render_type_body(node: dict, current_name: str, linker: Linker) -> str:
    t = node.get("type")
    if t == "object":
        return render_properties_table(node, current_name, linker)
    if t == "array":
        items = node.get("items", {})
        extra = f" (minItems {node['minItems']})" if "minItems" in node else ""
        return f"<p>Array of {describe_type(items, current_name, linker)}{extra}.</p>"
    if "oneOf" in node:
        parts = "".join(f"<li>{describe_type(sub, current_name, linker)}</li>" for sub in node["oneOf"])
        return f"<p>One of:</p><ul>{parts}</ul>"
    return f"<p>{describe_type(node, current_name, linker)}</p>"


def render_defs(schema: dict, current_name: str, linker: Linker) -> str:
    defs = schema.get("$defs", {})
    if not defs:
        return ""
    sections = []
    for def_name, def_schema in defs.items():
        title = def_schema.get("title", def_name)
        description = def_schema.get("description", "")
        sections.append(
            f'<section class="def" id="def-{esc(def_name)}"><h3>{esc(title)}</h3>'
            f"<p>{esc(description)}</p>{render_type_body(def_schema, current_name, linker)}</section>"
        )
    return f'<section class="defs"><h2>Definitions</h2>{"".join(sections)}</section>'


def humanize(stem: str) -> str:
    return stem.replace("-", " ").replace("_", " ").capitalize()


def render_examples(name: str) -> str:
    example_dir = EXAMPLES_DIR / name
    if not example_dir.is_dir():
        return ""

    def block(kind: str, label: str) -> str:
        files = sorted((example_dir / kind).glob("*.json"))
        if not files:
            return ""
        items = []
        for f in files:
            pretty = json.dumps(json.loads(f.read_text()), indent=2)
            items.append(
                f"<figure><figcaption>{esc(humanize(f.stem))}</figcaption>"
                f"<pre><code>{esc(pretty)}</code></pre></figure>"
            )
        return f'<div class="examples-{kind}"><h3>{esc(label)}</h3>{"".join(items)}</div>'

    valid_html = block("valid", "Valid")
    invalid_html = block("invalid", "Invalid")
    if not valid_html and not invalid_html:
        return ""
    return f'<section class="examples"><h2>Examples</h2>{valid_html}{invalid_html}</section>'


PAGE_TEMPLATE = """<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title} · betula</title>
<link rel="stylesheet" href="style.css">
</head>
<body>
<header><a class="back" href="index.html">&larr; betula</a><a class="nav" href="bindings.html">Language bindings</a></header>
<main>
<h1>{title}</h1>
<p class="schema-id"><a href="{raw_href}">{schema_id}</a></p>
<p class="description">{description}</p>
{body}
{defs}
{examples}
</main>
</body>
</html>
"""


def render_schema_page(name: str, schema: dict, linker: Linker) -> str:
    title = schema.get("title", name)
    description = esc(schema.get("description", ""))
    body = render_type_body(schema, name, linker)
    defs = render_defs(schema, name, linker)
    examples = render_examples(name)
    raw_href = f"schema/{name}.schema.json"
    return PAGE_TEMPLATE.format(
        title=esc(title),
        description=description,
        body=body,
        defs=defs,
        examples=examples,
        raw_href=esc(raw_href),
        schema_id=esc(schema["$id"]),
    )


INDEX_TEMPLATE = """<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>betula</title>
<link rel="stylesheet" href="style.css">
</head>
<body>
<header><span class="back">betula</span><a class="nav" href="bindings.html">Language bindings</a></header>
<main>
<h1>betula</h1>
<p class="description">Schemas for common bioinformatics data formats, shared across
<a href="https://github.com/holmrenser/picea">picea</a>,
<a href="https://github.com/holmrenser/react-bio-viz">react-bio-viz</a>,
<a href="https://github.com/wur-bioinformatics/acacia">acacia</a>,
<a href="https://github.com/holmrenser/blastserver">blastserver</a>, and
<a href="https://github.com/holmrenser/iqtreeserver">iqtreeserver</a>.</p>
<h2>Schemas</h2>
<ul class="card-list">
{schema_cards}
</ul>
<h2>Core primitives</h2>
<ul class="card-list">
{core_cards}
</ul>
<h2>Language bindings</h2>
<p>Generated types with validating parsers for <strong>Python</strong>, <strong>TypeScript</strong>,
and <strong>Rust</strong>, each tested against every example on these pages.
See <a href="bindings.html">Language bindings</a> for installation and usage.</p>
</main>
</body>
</html>
"""


REPO_URL = "https://github.com/holmrenser/betula"

BINDINGS = [
    {
        "id": "python",
        "name": "Python",
        "summary": "Pydantic v2 models generated by datamodel-code-generator. <code>parse</code> and "
        "<code>parse_json</code> validate in Pydantic's strict mode, which is what makes them agree with "
        "the schema; calling <code>Model.model_validate</code> directly would coerce, e.g., <code>\"1\"</code> "
        "into an int.",
        "install": 'pip install "git+{repo}@v{version}#subdirectory=bindings/python"',
        "example": "bindings/python/examples/usage.py",
        "reference": "Docstrings on <code>parse</code> and <code>parse_json</code> (<code>help(betula.parse)</code>) "
        "carry examples that run as doctests.",
    },
    {
        "id": "typescript",
        "name": "TypeScript",
        "summary": "Types generated by json-schema-to-typescript, validated at runtime by ajv against the same "
        "schemas. <code>parse(kind, data)</code> is typed by the kind name, so <code>parse(\"Tree\", data)</code> "
        "returns a <code>Tree</code>.",
        "install": "# not on npm yet: build from a checkout, then install it into your project\n"
        "git clone {repo} && cd betula/bindings/typescript && npm ci && npm run build\n"
        "cd /your/project && npm install /path/to/betula/bindings/typescript",
        "example": "bindings/typescript/examples/usage.ts",
        "reference": "Signatures and TSDoc comments ship in the package's <code>.d.ts</code> files, so they "
        "show up in your editor.",
    },
    {
        "id": "rust",
        "name": "Rust",
        "summary": "Serde types generated by typify. typify doesn't enforce every JSON Schema keyword, so "
        "<code>parse</code> and <code>parse_str</code> validate against the schema with the "
        "<code>jsonschema</code> crate before deserializing. Serializing and re-parsing gives an equal "
        "value, though empty optional arrays are omitted on output.",
        "install": '# Cargo.toml\n[dependencies]\nbetula = {{ git = "{repo}", tag = "v{version}" }}',
        "example": "bindings/rust/examples/usage.rs",
        "reference": "Run <code>cargo doc --open -p betula</code>; the doc comments' examples run as doctests.",
    },
]

BINDINGS_TEMPLATE = """<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Language bindings · betula</title>
<link rel="stylesheet" href="style.css">
</head>
<body>
<header><a class="back" href="index.html">&larr; betula</a></header>
<main>
<h1>Language bindings</h1>
<p class="description">Each binding has one type per schema, named after the schema's title (see the
<a href="index.html">schema pages</a>), and a parse function that rejects exactly what the schema rejects.
All three run every example on the schema pages in CI.</p>
<p>The usage code below is each binding's <code>examples/</code> file, included verbatim. CI runs these files and
checks their assertions, and fails if a public function or type isn't used in them, so this page covers the
whole hand-written API and can't drift from it.</p>
<p class="toc">{toc}</p>
{sections}
<h2 id="r">R</h2>
<p>There's no JSON-Schema-to-R code generator comparable to the ones above. From R, read with
<code>jsonlite</code> and validate with the <code>jsonvalidate</code> package against the self-contained bundle
from <code>python3 scripts/bundle_schema.py</code> (the individual schema files <code>$ref</code> each other by
URIs that don't resolve over the network).</p>
</main>
</body>
</html>
"""


def render_bindings(version: str) -> str:
    sections = []
    for binding in BINDINGS:
        install = binding["install"].format(repo=REPO_URL, version=version)
        example_path = binding["example"]
        code = (ROOT / example_path).read_text()
        sections.append(
            f'<h2 id="{binding["id"]}">{binding["name"]}</h2>'
            f'<p>{binding["summary"]}</p>'
            f"<h3>Install</h3><pre><code>{esc(install)}</code></pre>"
            f'<h3>Usage</h3><figure><figcaption><a href="{REPO_URL}/blob/main/{example_path}">'
            f"{esc(example_path)}</a></figcaption><pre><code>{esc(code)}</code></pre></figure>"
            f'<p class="note">API reference: {binding["reference"]}</p>'
        )
    toc = " · ".join(f'<a href="#{b["id"]}">{b["name"]}</a>' for b in BINDINGS) + ' · <a href="#r">R</a>'
    return BINDINGS_TEMPLATE.format(toc=toc, sections="\n".join(sections))


def render_index(schemas: dict[str, dict]) -> str:
    def card(name: str, schema: dict) -> str:
        title = esc(schema.get("title", name))
        desc = schema.get("description", "")
        first_sentence = desc.split(". ")[0].strip()
        if first_sentence and not first_sentence.endswith("."):
            first_sentence += "."
        return f'<li><a href="{esc(page_filename(name))}"><strong>{title}</strong></a><p>{esc(first_sentence)}</p></li>'

    schema_cards = "\n".join(card(n, s) for n, s in schemas.items() if not n.startswith("core/"))
    core_cards = "\n".join(card(n, s) for n, s in schemas.items() if n.startswith("core/"))
    return INDEX_TEMPLATE.format(schema_cards=schema_cards, core_cards=core_cards)


CSS = """
:root {
  --bg: #ffffff; --fg: #1a1a1a; --muted: #5f6368; --accent: #2f6f4f;
  --border: #e1e4e8; --code-bg: #f6f8fa; --req-bg: #fde8e8; --opt-bg: #eef1f3;
}
@media (prefers-color-scheme: dark) {
  :root { --bg: #14171a; --fg: #e6e8eb; --muted: #9aa4af; --accent: #7fd1a8;
    --border: #2a2f34; --code-bg: #1d2125; --req-bg: #3a2222; --opt-bg: #23282d; }
}
* { box-sizing: border-box; }
body { background: var(--bg); color: var(--fg); font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  margin: 0; line-height: 1.5; }
header { padding: 1rem 1.25rem; border-bottom: 1px solid var(--border); }
header .back { color: var(--accent); text-decoration: none; font-weight: 600; }
header .nav { color: var(--muted); text-decoration: none; margin-left: 1.25rem; }
.toc { color: var(--muted); }
main { max-width: 860px; margin: 0 auto; padding: 1.5rem 1.25rem 4rem; }
h1 { margin-top: 0.5rem; }
h2 { margin-top: 2.5rem; border-bottom: 1px solid var(--border); padding-bottom: 0.3rem; }
h3 { margin-top: 1.75rem; }
.description { color: var(--muted); }
.schema-id a { color: var(--muted); font-size: 0.85rem; font-family: ui-monospace, monospace; }
code { font-family: ui-monospace, SFMono-Regular, Menlo, monospace; background: var(--code-bg);
  padding: 0.1em 0.35em; border-radius: 4px; font-size: 0.9em; }
table.props { border-collapse: collapse; width: 100%; margin: 1rem 0; }
table.props th, table.props td { text-align: left; padding: 0.5rem 0.6rem; border-bottom: 1px solid var(--border);
  vertical-align: top; font-size: 0.92rem; }
table.props th { color: var(--muted); font-weight: 600; }
.req-required { background: var(--req-bg); border-radius: 4px; }
.req-optional { background: var(--opt-bg); border-radius: 4px; }
.note { color: var(--muted); font-size: 0.85rem; }
section.def { border-top: 1px dashed var(--border); padding-top: 0.5rem; }
figure { margin: 0.75rem 0 1.25rem; }
figcaption { color: var(--muted); font-size: 0.85rem; margin-bottom: 0.3rem; }
pre { background: var(--code-bg); border: 1px solid var(--border); border-radius: 6px; padding: 0.85rem;
  overflow-x: auto; }
pre code { background: none; padding: 0; }
.card-list { list-style: none; padding: 0; display: grid; gap: 0.75rem; }
.card-list li { border: 1px solid var(--border); border-radius: 8px; padding: 0.85rem 1rem; }
.card-list a { color: var(--fg); text-decoration: none; }
.card-list strong { color: var(--accent); }
.card-list p { margin: 0.35rem 0 0; color: var(--muted); font-size: 0.9rem; }
"""


def main() -> None:
    schemas = load_schemas()
    linker = Linker(schemas)

    if OUT_DIR.exists():
        shutil.rmtree(OUT_DIR)
    OUT_DIR.mkdir(parents=True)
    (OUT_DIR / "style.css").write_text(CSS)
    (OUT_DIR / "index.html").write_text(render_index(schemas))
    version = json.loads((ROOT / "package.json").read_text())["version"]
    (OUT_DIR / "bindings.html").write_text(render_bindings(version))

    for name, schema in schemas.items():
        (OUT_DIR / page_filename(name)).write_text(render_schema_page(name, schema, linker))

    shutil.copytree(SCHEMA_DIR, OUT_DIR / "schema")
    shutil.copytree(EXAMPLES_DIR, OUT_DIR / "examples")

    print(f"Wrote {len(schemas) + 2} pages to {OUT_DIR}")


if __name__ == "__main__":
    main()
