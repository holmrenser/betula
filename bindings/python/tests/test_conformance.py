"""Run every examples/ fixture through the public betula API.

Valid fixtures must parse (both from JSON text and from decoded dicts);
invalid ones must raise. Classes are looked up on the `betula_schema` package by
schema title, so a schema that isn't exported fails here.
"""

import contextlib
import doctest
import io
import json
import re
import runpy
import shutil
import tempfile
import unittest
from pathlib import Path

import betula_schema

REPO = Path(__file__).resolve().parents[3]
SCHEMA_DIR = REPO / "schema"
EXAMPLES_DIR = REPO / "examples"


def fixtures():
    for schema_path in sorted(SCHEMA_DIR.rglob("*.schema.json")):
        name = schema_path.relative_to(SCHEMA_DIR).as_posix().removesuffix(".schema.json")
        title = json.loads(schema_path.read_text())["title"]
        for kind in ("valid", "invalid"):
            for path in sorted((EXAMPLES_DIR / name / kind).glob("*.json")):
                yield title, kind == "valid", path


class Conformance(unittest.TestCase):
    def test_fixtures(self):
        count = 0
        for title, should_pass, path in fixtures():
            model = getattr(betula_schema, title)
            text = path.read_text()
            for label, run in (
                ("parse_json", lambda: betula_schema.parse_json(model, text)),
                ("parse", lambda: betula_schema.parse(model, json.loads(text))),
            ):
                with self.subTest(fixture=str(path.relative_to(REPO)), via=label):
                    if should_pass:
                        run()
                    else:
                        with self.assertRaises(betula_schema.ValidationError):
                            run()
            count += 1
        self.assertGreater(count, 0)

    def test_discriminator_selects_variant(self):
        # "ACGN" is valid in both the DNA and RNA alphabets, so only `type` decides.
        rna = betula_schema.parse_json(betula_schema.Sequence, '{"type": "rna-sequence", "identifier": "s", "sequence": "ACGN"}')
        self.assertIsInstance(rna.root, betula_schema.RnaSequence)
        untyped = betula_schema.parse_json(betula_schema.Sequence, '{"identifier": "s", "sequence": "ACGN"}')
        self.assertIsInstance(untyped.root, betula_schema.UntypedSequence)

    def test_round_trip(self):
        for title, should_pass, path in fixtures():
            if not should_pass:
                continue
            with self.subTest(fixture=str(path.relative_to(REPO))):
                original = json.loads(path.read_text())
                parsed = betula_schema.parse_json(getattr(betula_schema, title), path.read_text())
                dumped = json.loads(parsed.model_dump_json(exclude_unset=True))
                self.assertEqual(dumped, original)


USAGE = Path(__file__).resolve().parents[1] / "examples" / "usage.py"
SNIPPETS = Path(__file__).resolve().parents[1] / "examples" / "schemas"


class Snippets(unittest.TestCase):
    def test_every_schema_snippet_runs(self):
        # Each schema page shows its examples/schemas/<name>.py snippet; run it
        # against the schema's first valid fixture, under the filename it opens.
        schema_names = [
            path.relative_to(SCHEMA_DIR).as_posix().removesuffix(".schema.json")
            for path in sorted(SCHEMA_DIR.rglob("*.schema.json"))
        ]
        snippet_names = sorted(p.relative_to(SNIPPETS).with_suffix("").as_posix() for p in SNIPPETS.rglob("*.py"))
        self.assertEqual(snippet_names, sorted(schema_names))
        with tempfile.TemporaryDirectory() as tmp, contextlib.chdir(tmp):
            for name in schema_names:
                with self.subTest(snippet=name):
                    snippet = SNIPPETS / f"{name}.py"
                    filename = re.search(r'open\("([^"]+)"\)', snippet.read_text()).group(1)
                    fixture = sorted((EXAMPLES_DIR / name / "valid").glob("*.json"))[0]
                    shutil.copy(fixture, Path(tmp) / filename)
                    out = io.StringIO()
                    with contextlib.redirect_stdout(out):
                        runpy.run_path(str(snippet), run_name="__main__")
                    self.assertTrue(out.getvalue().strip())


class Documentation(unittest.TestCase):
    def test_usage_example_runs(self):
        runpy.run_path(str(USAGE), run_name="__main__")

    def test_usage_example_covers_public_api(self):
        # The docs' Getting started section embeds usage.py as the API tour, so
        # every exported name that isn't a generated model must appear in it.
        hand_written = [name for name in betula_schema.__all__ if not hasattr(betula_schema.models, name)]
        self.assertTrue(hand_written)
        text = USAGE.read_text()
        self.assertEqual([name for name in hand_written if name not in text], [])


def load_tests(loader, tests, ignore):
    tests.addTests(doctest.DocTestSuite(betula_schema))
    return tests


if __name__ == "__main__":
    unittest.main()
