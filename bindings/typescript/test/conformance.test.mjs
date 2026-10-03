// Run every examples/ fixture through the public betula API. Valid fixtures
// must parse; invalid ones must throw BetulaValidationError.
import { test } from "node:test";
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdtempSync, readFileSync, readdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import { BetulaValidationError, is, parse, parseJson } from "../dist/index.js";

const repo = join(fileURLToPath(import.meta.url), "..", "..", "..", "..");
const schemaDir = join(repo, "schema");
const examplesDir = join(repo, "examples");

function schemaFiles(dir, suffix = ".schema.json") {
  return readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) return schemaFiles(full, suffix);
    return entry.name.endsWith(suffix) ? [full] : [];
  });
}

function listJson(dir) {
  try {
    return readdirSync(dir).filter((f) => f.endsWith(".json")).sort().map((f) => join(dir, f));
  } catch {
    return [];
  }
}

const fixtures = schemaFiles(schemaDir)
  .sort()
  .flatMap((file) => {
    const name = relative(schemaDir, file).replaceAll("\\", "/").replace(/\.schema\.json$/, "");
    const { title } = JSON.parse(readFileSync(file, "utf8"));
    return ["valid", "invalid"].flatMap((kind) =>
      listJson(join(examplesDir, name, kind)).map((path) => ({ title, shouldPass: kind === "valid", path })),
    );
  });

test("fixtures exist", () => assert.ok(fixtures.length > 0));

// Each schema page shows its examples/schemas/<name>.ts snippet; run the
// compiled snippet against the schema's first valid fixture, under the
// filename it reads. Requires `tsc -p tsconfig.examples.json` first.
test("every schema snippet runs", () => {
  const pkg = join(repo, "bindings", "typescript");
  const names = schemaFiles(schemaDir)
    .map((f) => relative(schemaDir, f).replaceAll("\\", "/").replace(/\.schema\.json$/, ""))
    .sort();
  const snippets = schemaFiles(join(pkg, "examples", "schemas"), ".ts")
    .map((f) => relative(join(pkg, "examples", "schemas"), f).replaceAll("\\", "/").replace(/\.ts$/, ""))
    .sort();
  assert.deepEqual(snippets, names);

  const tmp = mkdtempSync(join(tmpdir(), "betula-snippets-"));
  for (const name of names) {
    const source = readFileSync(join(pkg, "examples", "schemas", `${name}.ts`), "utf8");
    const filename = source.match(/readFileSync\("([^"]+)"/)[1];
    const fixture = listJson(join(examplesDir, name, "valid"))[0];
    copyFileSync(fixture, join(tmp, filename));
    const run = spawnSync(process.execPath, [join(pkg, "build", "examples", "schemas", `${name}.js`)], {
      cwd: tmp,
      encoding: "utf8",
    });
    assert.equal(run.status, 0, `${name} snippet failed:\n${run.stderr}`);
    assert.ok(run.stdout.trim(), `${name} snippet printed nothing`);
  }
});

// The docs' Getting started section embeds examples/usage.ts as the API tour, so
// every runtime export must appear in it.
test("usage example covers the public API", async () => {
  const api = Object.keys(await import("../dist/index.js"));
  assert.ok(api.length > 0);
  const usage = readFileSync(join(repo, "bindings", "typescript", "examples", "usage.ts"), "utf8");
  assert.deepEqual(api.filter((name) => !usage.includes(name)), []);
});

for (const { title, shouldPass, path } of fixtures) {
  test(relative(repo, path), () => {
    const text = readFileSync(path, "utf8");
    assert.equal(is(title, JSON.parse(text)), shouldPass);
    if (shouldPass) {
      assert.deepEqual(parseJson(title, text), JSON.parse(text));
      parse(title, JSON.parse(text));
    } else {
      assert.throws(() => parseJson(title, text), BetulaValidationError);
    }
  });
}
