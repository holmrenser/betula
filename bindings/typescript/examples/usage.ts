// Parsing betula JSON with the TypeScript bindings. Compiled and run in CI;
// the asserts are checked.
import assert from "node:assert/strict";
import { BetulaValidationError, is, parse, parseJson, type Kind, type Sequence, type Tree } from "betula";

// Parse JSON text. The kind name picks both the schema and the return type.
const tree: Tree = parseJson(
  "Tree",
  '{"name": "root", "length": 0, "children": [{"name": "A", "length": 0.1, "children": []}]}',
);
assert.deepEqual(tree.children.map((child) => child.name), ["A"]);

// Or validate data you've already decoded. parse() returns its input,
// typed; it doesn't copy or transform it.
const decoded: unknown = JSON.parse('{"identifier": "s1", "sequence": "ACGT"}');
const untyped = parse("Sequence", decoded);
assert.equal(untyped, decoded);

// Union kinds narrow on their `type` discriminator. UntypedSequence has no
// `type` field, so check for it first.
function describe(seq: Sequence): string {
  if (!("type" in seq)) return `${seq.identifier}: unknown alphabet`;
  switch (seq.type) {
    case "dna-sequence":
      return `${seq.identifier}: DNA`;
    case "rna-sequence":
      return `${seq.identifier}: RNA`;
    case "protein-sequence":
      return `${seq.identifier}: protein`;
  }
}
assert.equal(describe(parse("Sequence", { type: "rna-sequence", identifier: "s2", sequence: "ACGU" })), "s2: RNA");
assert.equal(describe(untyped), "s1: unknown alphabet");

// is() is a type guard for branching instead of throwing.
const maybeTree: unknown = { name: "A", length: -1, children: [] };
assert.equal(is("Tree", maybeTree), false);

// Invalid data throws BetulaValidationError, carrying the kind and ajv's errors.
assert.throws(
  () => parseJson("Sequence", '{"type": "dna-sequence", "identifier": "s3", "sequence": "ACGU"}'),
  (err: unknown) => err instanceof BetulaValidationError && err.kind === "Sequence" && err.errors.length > 0,
);

// Kind is the union of every schema title, so generic helpers stay typed.
function count<K extends Kind>(kind: K, items: unknown[]): number {
  return items.filter((item) => is(kind, item)).length;
}
assert.equal(count("Tree", [tree, maybeTree]), 1);

console.log("typescript usage example ok");
