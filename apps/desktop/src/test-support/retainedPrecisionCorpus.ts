import { readFileSync } from "node:fs";
import { resolve } from "node:path";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

/**
 * The current shared retained-precision corpus is one lossless DAG/v1 document.
 * Each occurrence expands into independent containers. Fixture reads establish
 * no new native execution; readers consume the current reviewed data directly.
 */
export const RETAINED_PRECISION_CORPUS_FILES = [
  "fixtures/results/retained_precision_cases.json",
] as const;
const LIST_MEMBERS = new Set(["cases", "mutations", "must_pass"]);
const PROJECT_ROOT = resolve(__dirname, "../../../../");

/** Strict current test storage; every occurrence gets independent containers. */
export function expandRetainedPrecisionDefinition(document: Json): Json {
  if (!document || typeof document !== "object" || Array.isArray(document)
    || Object.keys(document).sort().join(",") !== "format,layout,nodes,root"
    || document.format !== "retained-precision-corpus-dag/v1"
    || !["compact", "indent-2"].includes(document.layout) || !Array.isArray(document.nodes)) {
    throw new Error("Unsupported retained-precision corpus definition");
  }
  const nodes = document.nodes as Json[];
  const reference = (value: Json, upper: number) => Number.isInteger(value) && !Object.is(value, -0) && value >= 0 && value < upper;
  if (!reference(document.root, nodes.length)) throw new Error("Invalid corpus root");
  for (let index = 0; index < nodes.length; index += 1) {
    const node = nodes[index];
    if (!Array.isArray(node)) {
      if (node !== null && (!["string", "number", "boolean"].includes(typeof node) || (typeof node === "number" && !Number.isFinite(node)))) {
        throw new Error("Invalid corpus scalar");
      }
      continue;
    }
    if (!Number.isInteger(node[0]) || Object.is(node[0], -0) || ![0, 1].includes(node[0]) || !node.slice(1).every(ref => reference(ref, index))) {
      throw new Error("Corpus references must name earlier nodes");
    }
    if (node[0] === 1) {
      const keys = node.filter((_, i) => i % 2 === 1).map(ref => nodes[ref]);
      if (node.length % 2 !== 1 || !keys.every(key => typeof key === "string") || new Set(keys).size !== keys.length) {
        throw new Error("Corpus object keys must be distinct strings");
      }
    }
  }
  const expand = (index: number): Json => {
    const node = nodes[index];
    if (!Array.isArray(node)) return node;
    if (node[0] === 0) return node.slice(1).map(expand);
    const entries = [];
    for (let i = 1; i < node.length; i += 2) entries.push([nodes[node[i]], expand(node[i + 1])]);
    return Object.fromEntries(entries);
  };
  return expand(document.root);
}

let sharedCorpus: Json;
function sharedRetainedPrecisionCorpus(): Json {
  if (sharedCorpus) return sharedCorpus;
  const merged: Record<string, Json> = {};
  for (const name of RETAINED_PRECISION_CORPUS_FILES) {
    const part: Record<string, Json> = expandRetainedPrecisionDefinition(JSON.parse(readFileSync(resolve(PROJECT_ROOT, name), "utf8")));
    for (const [key, value] of Object.entries(part)) {
      if (LIST_MEMBERS.has(key)) {
        if (!Array.isArray(value)) throw new Error(`${name}: ${key} is not an array`);
        merged[key] = [...(merged[key] ?? []), ...value];
      } else if (key in merged) {
        if (JSON.stringify(merged[key]) !== JSON.stringify(value)) throw new Error(`${name}: corpus member ${key} differs from an earlier file`);
      } else {
        merged[key] = value;
      }
    }
  }
  sharedCorpus = merged;
  return sharedCorpus;
}

export function readRetainedPrecisionCorpus(): Json {
  // Callers historically received fresh mutable trees; preserve their isolation
  // while parsing and expanding the shared immutable basis only once.
  return structuredClone(sharedRetainedPrecisionCorpus());
}
