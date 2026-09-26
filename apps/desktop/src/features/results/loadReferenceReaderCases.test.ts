/** Targeted load-reference-1 reader refusals, each raising Python's code (T1 WP2).
 *
 * loadReferenceReaderCases.cases.json holds JSON-pointer ops on committed
 * load-reference-1 raws; each case's `python` value is the code Python
 * validate_load_reference_evidence raised on the same bytes (authored by the
 * T1 WP2 return's make_reader_cases.py). The desktop reader must raise the same
 * code. The remaining tests pin guards that no JSON value reaches: the Python
 * counterparts are the NON_JSON_VALUE and HOSTILE_MAPPING probes in that
 * script's log. All values are invented. */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { MechanicsResult } from "../../types";
import { at, guarded, LoadReferenceError, validateLoadReferenceEvidence, verifyLoadReferenceTransportSchema } from "./loadReferenceEvidence";
import { LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE, LOAD_REFERENCE_SOURCE_VALIDATION_REQUIRED, loadReferenceSourceStanding, validateLoadReferenceSourceEvidence } from "./loadReferenceSourceEvidence";
import cases from "./loadReferenceReaderCases.cases.json";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const json = (path: string) => JSON.parse(read(path));
const tokens = (path: string) => path.slice(1).split("/");
function apply(doc: Json, ops: Json[]): Json {
  for (const op of ops) {
    const t = tokens(op.path);
    let parent = doc;
    for (const x of t.slice(0, -1)) parent = Array.isArray(parent) ? parent[Number(x)] : parent[x];
    const key = Array.isArray(parent) ? Number(t[t.length - 1]) : t[t.length - 1];
    if (op.op === "set") parent[key] = structuredClone(op.value);
    else if (op.op === "append") parent[key].push(structuredClone(op.value));
    else throw new Error(`unknown op ${op.op}`);
  }
  return doc;
}
function outcome(source: MechanicsResult): string {
  try { validateLoadReferenceEvidence(source); return "accept"; }
  catch (error) { return error instanceof LoadReferenceError ? error.message : `foreign: ${String(error)}`; }
}
const LR = "fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json";
const LRS = "fixtures/product_preview/load_reference_source/mixed-sparse_interactive.raw.json";

describe("targeted load-reference-1 refusals raise Python's code", () => {
  it("covers the recorded case set", () => {
    expect((cases as Json[]).length).toBe(14);
    for (const c of cases as Json[]) expect(c.python).toMatch(/^SOURCE_LOAD_REFERENCE_[A-Z_]+$/);
  });
  it.each((cases as Json[]).map(c => [c.id, c] as [string, Json]))("%s", (_id, c: Json) => {
    expect(outcome(json(c.source))).toBe("accept");
    expect(outcome(apply(json(c.source), c.ops))).toBe(c.python);
  });
});

describe("guards reached only by non-JSON values", () => {
  it("a non-JSON value anywhere is refused as NUMBER_INVALID (Python: object())", () => {
    const source = json(LR);
    source.diagnostics[0].invented = () => 0;
    expect(outcome(source)).toBe("SOURCE_LOAD_REFERENCE_NUMBER_INVALID");
  });
  it("a foreign exception inside the reader is reported as MALFORMED (Python: hostile mapping)", async () => {
    const hostile = (path: string) => {
      const source = json(path);
      Object.defineProperty(source.contract_evidence, "pressure", { enumerable: true, get() { throw new TypeError("invented hostile mapping"); } });
      return source;
    };
    expect(outcome(hostile(LR))).toBe("SOURCE_LOAD_REFERENCE_MALFORMED");
    await expect(validateLoadReferenceSourceEvidence(hostile(LRS))).rejects.toThrow(/^SOURCE_LOAD_REFERENCE_MALFORMED$/);
  });
  it("a Python-style subscript of a missing key or a non-mapping is malformed", () => {
    expect(() => at({}, "invented")).toThrow();
    expect(() => at(null, "invented")).toThrow();
    expect(() => at([], "0")).toThrow();
    expect(at({ invented: null }, "invented")).toBeNull();
    expect(() => guarded(() => at({}, "invented"))).toThrow(/^SOURCE_LOAD_REFERENCE_MALFORMED$/);
  });
  it("the joined reader validates a snapshot taken at call time and registers exactly those bytes", async () => {
    const source = json(LRS);
    const member = source.contract_evidence.load_reference_states[0].members[0];
    const original = member.fit_strain;
    const pending = validateLoadReferenceSourceEvidence(source);
    member.fit_strain = 0.5; // edited while the asynchronous validation is in flight
    await expect(pending).resolves.toBe(false);
    expect(loadReferenceSourceStanding(source).findings).toEqual([LOAD_REFERENCE_SOURCE_VALIDATION_REQUIRED]);
    member.fit_strain = original;
    expect(loadReferenceSourceStanding(source).findings).toEqual([LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE]);
  });
  it("the transport schema is pinned by its bytes", async () => {
    const bytes = read("schemas/load_reference_state.schema.json");
    await expect(verifyLoadReferenceTransportSchema(bytes)).resolves.toMatchObject({ $defs: expect.any(Object) });
    await expect(verifyLoadReferenceTransportSchema(`${bytes} `)).rejects.toThrow(/^SOURCE_LOAD_REFERENCE_TRANSPORT_SCHEMA_HASH$/);
  });
});
