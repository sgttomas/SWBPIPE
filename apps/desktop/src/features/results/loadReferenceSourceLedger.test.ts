/** Check-for-check parity of the invocation-free source-blocks ledger port
 * (loadReferenceSourceEvidence.ts) with Python
 * source_blocks._validate_source_blocks(doc, None, context=physics_source).
 *
 * loadReferenceSourceLedger.cases.json holds concrete JSON-pointer ops on the
 * physics-source-1 projection of committed joined raws; the publication and
 * receipt hashes are resealed so the targeted check is reached. Each case's
 * `python` value is the code Python raised on the same bytes (authored by the
 * T1 WP2 return's make_ledger_cases.py). The desktop port must raise the same
 * code. The PREEMPTED cases target checks that the closed physics-source-1
 * receipt schema or an earlier check makes unreachable; there both languages
 * stop at that earlier check.
 * All values are invented. */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { MechanicsResult } from "../../types";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import { loadReferenceSourceLedgerTestSeam as seam } from "./loadReferenceSourceEvidence";
import cases from "./loadReferenceSourceLedger.cases.json";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const raw = (name: string) => JSON.parse(readFileSync(resolve(root, `fixtures/product_preview/load_reference_source/${name}.raw.json`), "utf8")) as MechanicsResult;
const hash = (domain: string, payload: unknown) => canonicalSha256HexCheckedV1({ domain, payload });
/** Cases aimed at a ported check that no input reaches: the closed receipt
 * schema or an earlier check of the same case stops both languages first. */
const PREEMPTED: Record<string, string> = {
  FAILED_OUTCOME: "PHYSICS_SOURCE_RECEIPT_SHAPE", FAILURE_BLOCK_ORDER: "PHYSICS_SOURCE_RECEIPT_SHAPE", FAILURE_CATEGORY: "PHYSICS_SOURCE_RECEIPT_SHAPE",
  PROJECTION_CRITERION: "PHYSICS_SOURCE_RECEIPT_SHAPE", DERIVED_RECIPE: "PHYSICS_SOURCE_RECEIPT_SHAPE", SPRING_ATTRIBUTION_DUPLICATE: "PHYSICS_SOURCE_RECEIPT_SHAPE",
  SUPPORT_NORM_INPUTS: "PHYSICS_SOURCE_RECEIPT_SHAPE", STRESS_RECIPE_INPUTS: "PHYSICS_SOURCE_RECEIPT_SHAPE", SUMMARY_RECIPE_INPUTS: "PHYSICS_SOURCE_RECEIPT_SHAPE",
  SECTION_RECIPE_KIND: "PHYSICS_SOURCE_RECEIPT_SHAPE", UNSELECTED_SUPPORT_CERTIFICATE: "SOURCE_BLOCKS_ORDINARY_SELECTION",
};

const base = new Map<string, Promise<Json>>();
const projected = (name: string) => {
  if (!base.has(name)) base.set(name, seam.project(raw(name)));
  return base.get(name)!.then(doc => structuredClone(doc));
};
const tokens = (path: string) => path.slice(1).split("/").map(t => t.replaceAll("~1", "/").replaceAll("~0", "~"));
function apply(doc: Json, ops: Json[]): Json {
  for (const op of ops) {
    const t = tokens(op.path);
    let parent = doc;
    for (const x of t.slice(0, -1)) parent = Array.isArray(parent) ? parent[Number(x)] : parent[x];
    const last = t[t.length - 1];
    const target = () => Array.isArray(parent) ? parent[Number(last)] : parent[last];
    if (op.op === "set") { if (Array.isArray(parent)) parent[Number(last)] = structuredClone(op.value); else parent[last] = structuredClone(op.value); }
    else if (op.op === "remove") { if (Array.isArray(parent)) parent.splice(Number(last), 1); else delete parent[last]; }
    else if (op.op === "append") target().push(structuredClone(op.value));
    else if (op.op === "remove_where") {
      const array = target();
      // Python dict equality: key order is not significant.
      const sorted = (v: Json): Json => Array.isArray(v) ? v.map(sorted) : v && typeof v === "object" ? Object.fromEntries(Object.keys(v).sort().map(k => [k, sorted(v[k])])) : v;
      const kept = array.filter((item: Json) => !Object.entries(op.match).every(([k, v]) => JSON.stringify(sorted(item[k])) === JSON.stringify(sorted(v))));
      array.splice(0, array.length, ...kept);
    } else throw new Error(`unknown op ${op.op}`);
  }
  return doc;
}
async function reseal(doc: Json, publication: boolean) {
  const body = doc.source_block_recovery.body;
  if (publication) body.publication_sha256 = await hash("source_blocks_publication_v1", Object.fromEntries(Object.entries(doc).filter(([k]) => k !== "source_block_recovery")));
  doc.source_block_recovery.receipt_sha256 = await hash("source_blocks_receipt_v1", body);
}

describe("invocation-free ledger port: every case raises Python's code", () => {
  it("covers the recorded case set", () => {
    expect((cases as Json[]).length).toBe(108);
    expect(new Set((cases as Json[]).map(c => c.id)).size).toBe((cases as Json[]).length);
    for (const [id, code] of Object.entries(PREEMPTED)) expect((cases as Json[]).find(c => c.id === id)!.python).toBe(code);
    for (const c of cases as Json[]) if (!Object.hasOwn(PREEMPTED, c.id) && !c.id.startsWith("ACCEPT")) expect(c.python.endsWith(c.id.split("#")[0])).toBe(true);
  });
  it.each((cases as Json[]).map(c => [c.id, c] as [string, Json]))("%s", async (_id, c: Json) => {
    const doc = apply(await projected(c.source), c.ops);
    if (c.reseal_publication !== null) await reseal(doc, c.reseal_publication);
    let outcome = "accept";
    try { await seam.validateLedger(doc); }
    catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      outcome = /^(SOURCE_BLOCKS_|PHYSICS_SOURCE_)/.test(message) ? message : "PHYSICS_SOURCE_MALFORMED_VALUE";
    }
    expect(outcome).toBe(c.python);
  });
});
