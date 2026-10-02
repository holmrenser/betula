"""Run every examples/ fixture through the public betula API.

Valid fixtures must parse (both from JSON text and from decoded dicts);
invalid ones must raise. Classes are looked up on the `betula` package by
schema title, so a schema that isn't exported fails here.
"""

import doctest
import json
import runpy
import unittest
from pathlib import Path

import betula

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
            model = getattr(betula, title)
            text = path.read_text()
            for label, run in (
                ("parse_json", lambda: betula.parse_json(model, text)),
                ("parse", lambda: betula.parse(model, json.loads(text))),
            ):
                with self.subTest(fixture=str(path.relative_to(REPO)), via=label):
                    if should_pass:
                        run()
                    else:
                        with self.assertRaises(betula.ValidationError):
                            run()
            count += 1
        self.assertGreater(count, 0)

    def test_discriminator_selects_variant(self):
        # "ACGN" is valid in both the DNA and RNA alphabets, so only `type` decides.
        rna = betula.parse_json(betula.Sequence, '{"type": "rna-sequence", "identifier": "s", "sequence": "ACGN"}')
        self.assertIsInstance(rna.root, betula.RnaSequence)
        untyped = betula.parse_json(betula.Sequence, '{"identifier": "s", "sequence": "ACGN"}')
        self.assertIsInstance(untyped.root, betula.UntypedSequence)

    def test_round_trip(self):
        for title, should_pass, path in fixtures():
            if not should_pass:
                continue
            with self.subTest(fixture=str(path.relative_to(REPO))):
                original = json.loads(path.read_text())
                parsed = betula.parse_json(getattr(betula, title), path.read_text())
                dumped = json.loads(parsed.model_dump_json(exclude_unset=True))
                self.assertEqual(dumped, original)


USAGE = Path(__file__).resolve().parents[1] / "examples" / "usage.py"


class Documentation(unittest.TestCase):
    def test_usage_example_runs(self):
        runpy.run_path(str(USAGE), run_name="__main__")

    def test_usage_example_covers_public_api(self):
        # The docs site's Bindings page embeds usage.py as the API tour, so
        # every exported name that isn't a generated model must appear in it.
        hand_written = [name for name in betula.__all__ if not hasattr(betula.models, name)]
        self.assertTrue(hand_written)
        text = USAGE.read_text()
        self.assertEqual([name for name in hand_written if name not in text], [])


def load_tests(loader, tests, ignore):
    tests.addTests(doctest.DocTestSuite(betula))
    return tests


if __name__ == "__main__":
    unittest.main()
