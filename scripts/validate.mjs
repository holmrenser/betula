#!/usr/bin/env node
// Validate examples/<name>/{valid,invalid}/*.json against schema/<name>.schema.json.
//
// Schemas may $ref each other by $id (e.g. alignment -> sequence), so every
// schema in schema/ is added to one Ajv instance before any of them is used.
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";

const root = join(fileURLToPath(import.meta.url), "..", "..");
const schemaDir = join(root, "schema");
const examplesDir = join(root, "examples");

function listJson(dir) {
  try {
    return readdirSync(dir)
      .filter((f) => f.endsWith(".json"))
      .map((f) => join(dir, f));
  } catch {
    return [];
  }
}

function listSchemaFilesRecursive(dir) {
  const out = [];
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) out.push(...listSchemaFilesRecursive(full));
    else if (entry.name.endsWith(".schema.json")) out.push(full);
  }
  return out;
}

const ajv = new Ajv2020({ strict: true, allowUnionTypes: true });

const schemas = listSchemaFilesRecursive(schemaDir)
  .sort()
  .map((f) => ({
    name: relative(schemaDir, f).replaceAll("\\", "/").replace(/\.schema\.json$/, ""),
    schema: JSON.parse(readFileSync(f, "utf8")),
  }));

for (const { schema } of schemas) ajv.addSchema(schema, schema.$id);

const failures = [];

for (const { name, schema } of schemas) {
  const validate = ajv.getSchema(schema.$id);

  const exampleDir = join(examplesDir, name);
  if (!statSync(exampleDir, { throwIfNoEntry: false })?.isDirectory()) continue;

  for (const validPath of listJson(join(exampleDir, "valid"))) {
    const instance = JSON.parse(readFileSync(validPath, "utf8"));
    if (!validate(instance)) {
      failures.push(`${validPath}: expected VALID, got errors: ${ajv.errorsText(validate.errors)}`);
    }
  }

  for (const invalidPath of listJson(join(exampleDir, "invalid"))) {
    const instance = JSON.parse(readFileSync(invalidPath, "utf8"));
    if (validate(instance)) {
      failures.push(`${invalidPath}: expected INVALID, but it validated`);
    }
  }
}

if (failures.length > 0) {
  console.log(`FAILED (${failures.length}):`);
  for (const failure of failures) console.log(`  - ${failure}`);
  process.exit(1);
}

console.log("All examples validated as expected.");
