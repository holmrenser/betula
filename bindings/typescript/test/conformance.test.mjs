// Run every examples/ fixture through the public betula API. Valid fixtures
// must parse; invalid ones must throw BetulaValidationError.
import { test } from "node:test";
import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import { BetulaValidationError, is, parse, parseJson } from "../dist/index.js";

const repo = join(fileURLToPath(import.meta.url), "..", "..", "..", "..");
const schemaDir = join(repo, "schema");
const examplesDir = join(repo, "examples");

function schemaFiles(dir) {
  return readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) return schemaFiles(full);
    return entry.name.endsWith(".schema.json") ? [full] : [];
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

// The docs site's Bindings page embeds examples/usage.ts as the API tour, so
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
