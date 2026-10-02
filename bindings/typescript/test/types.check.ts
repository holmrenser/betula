// Compile-time checks only (tsc --noEmit); never executed.
import { parse, type DnaSequence, type Sequence, type Tree } from "../src/index.js";

declare const data: unknown;

const tree: Tree = parse("Tree", data);
const childNames: string[] = tree.children.map((child) => child.name);

// @ts-expect-error parse("Sequence") must not be assignable to Tree
const wrong: Tree = parse("Sequence", data);

const seq: Sequence = parse("Sequence", data);
if ("type" in seq && seq.type === "dna-sequence") {
  const dna: DnaSequence = seq;
  void dna;
}

// @ts-expect-error unknown kinds are rejected at compile time
parse("NotABetulaKind", data);

void childNames;
void wrong;
