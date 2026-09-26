/** Cross-language parity of the desktop load/reference-state readers.
 *
 * Runs the shared Rust/Python adversarial corpora over the committed producer
 * raws (fixtures/product_preview/load_reference*) in TypeScript:
 * - core/reporting/result_export/tests/fixtures/load_reference_mutations.json
 * - core/reporting/result_export/tests/fixtures/load_reference_source_mutations.json
 * Every case must reproduce the accept/refuse outcome that Rust and Python pin;
 * the direct reader calls must also reproduce Python's exact error code, except
 * for the declared JavaScript number-representation cases below. Set
 * LOAD_REFERENCE_PARITY_OUT to record `ts_outcomes.json` and
 * `ts_source_outcomes.json`; set LOAD_REFERENCE_PARITY_COMPARE to a folder with
 * the recorded Rust and Python logs to compare per-case outcomes with them too.
 * All inputs are invented; this proves consumer admission, never producer
 * authentication, numerical eligibility or engineering acceptance.
 */
import { describe, expect, it } from "vitest";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import type { MechanicsResult } from "../../types";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import { sourceContract, LOAD_REFERENCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID } from "./numericalResultQuality";
import { validateLoadReferenceEvidence, validateLoadReferenceTransportMetadata, verifyLoadReferenceTable, verifyLoadReferenceTransportSchema } from "./loadReferenceEvidence";
import { validateLoadReferenceSourceEvidence, validateLoadReferenceSourceTransportMetadata, verifyLoadReferenceSourceTable } from "./loadReferenceSourceEvidence";
import { validatePhysicsEvidence, validatePhysicsSourceEvidence } from "./physicsResultEvidence";
import { validatePhysicsSourceTransportMetadata } from "./physicsSourceRecovery";
import { validatePreviewPhysicsEvidence } from "./previewPhysicsEvidence";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const project = resolve(__dirname, "../../../../../");
const read = (path: string) => readFileSync(resolve(project, path), "utf8");
const LR = JSON.parse(read("core/reporting/result_export/tests/fixtures/load_reference_mutations.json"));
const LRS = JSON.parse(read("core/reporting/result_export/tests/fixtures/load_reference_source_mutations.json"));
const PHYSICS_PREFIX = "SOURCE_LOAD_REFERENCE_PHYSICS_EVIDENCE";
const JOIN_PREFIX = "SOURCE_LOAD_REFERENCE_JOIN_PHYSICS_SOURCE";
const domainHash = (domain: string, payload: unknown) => canonicalSha256HexCheckedV1({ domain, payload });

/** JavaScript holds every JSON number as binary64 and has no JSON text for a
 * non-finite value, so these direct-reader codes necessarily differ from
 * Python's. Their outcome (refusal) still agrees with both languages. */
const TS_CODES: Record<string, string> = {
  // JSON.parse maps 2^60 to a binary64 integer beyond 2^53-1, which the checked
  // canonical-JSON profile names as outside its number profile (the Python
  // adapter names the unsafe integer).
  "NUM-unsafe-integer-in-publication": "CHECKED-JSON-NUMBER-OUTSIDE-PROFILE",
};

const cache = new Map<string, string>();
function sourceText(sources: Record<string, string>, name: string): string {
  const path = sources[name];
  if (!path) throw new Error(`unknown source ${name}`);
  if (!cache.has(path)) cache.set(path, read(path));
  return cache.get(path)!;
}
const tokens = (path: string) => { expect(path.startsWith("/")).toBe(true); return path.slice(1).split("/").map(t => t.replaceAll("~1", "/").replaceAll("~0", "~")); };
function resolvePath(doc: Json, path: string): Json { let item = doc; for (const token of tokens(path)) item = Array.isArray(item) ? item[Number(token)] : item[token]; if (item === undefined) throw new Error(`missing ${path}`); return item; }
function parent(doc: Json, path: string): [Json, string] { const t = tokens(path); let item = doc; for (const token of t.slice(0, -1)) item = Array.isArray(item) ? item[Number(token)] : item[token]; return [item, t[t.length - 1]]; }
function setPath(doc: Json, path: string, value: unknown) { const [p, last] = parent(doc, path); if (Array.isArray(p)) p[Number(last)] = value; else p[last] = value; }
const matches = (item: Json, match: Json) => item && typeof item === "object" && Object.entries(match).every(([key, value]) => JSON.stringify(item[key]) === JSON.stringify(value));

async function reseal(doc: Json, op: Json) {
  const evidence = doc.contract_evidence, body = doc.source_block_recovery.body;
  if (op.physical !== null) {
    for (let i = 0; i < Math.min(body.cases.length, evidence.exact_cases.length); i++) {
      const c = body.cases[i], exact = evidence.exact_cases[i];
      const pressure = evidence.pressure.filter((p: Json) => p.load_case_id === exact.load_case_id);
      if (op.physical === "joined") c.physical_evidence_sha256 = await domainHash("load_reference_source_case_evidence_v1", { exact_case: exact, pressure, load_reference_state: evidence.load_reference_states[i] });
      else if (op.physical === "joined_without_record") c.physical_evidence_sha256 = await domainHash("load_reference_source_case_evidence_v1", { exact_case: exact, pressure });
      else if (op.physical === "physics") c.physical_evidence_sha256 = await domainHash("physics_source_case_evidence_v1", { exact_case: exact, pressure });
      else throw new Error(`reseal ${op.physical}`);
    }
  }
  if (op.publication) body.publication_sha256 = await domainHash("source_blocks_publication_v1", Object.fromEntries(Object.entries(doc).filter(([key]) => key !== "source_block_recovery")));
  if (op.receipt) doc.source_block_recovery.receipt_sha256 = await domainHash("source_blocks_receipt_v1", body);
}
/** Mirror of the Rust and Python harnesses. `nonfinite` stores the JavaScript
 * non-finite number; `json_text` parses its literal with JSON.parse. */
async function apply(sources: Record<string, string>, name: string, ops: Json[]): Promise<Json> {
  const doc = JSON.parse(sourceText(sources, name));
  for (const op of ops) {
    const copy = (value: unknown): Json => structuredClone(value);
    switch (op.op) {
      case "set": setPath(doc, op.path, copy(op.value)); break;
      case "remove": { const [p, last] = parent(doc, op.path); if (Array.isArray(p)) p.splice(Number(last), 1); else { expect(Object.hasOwn(p, last)).toBe(true); delete p[last]; } break; }
      case "append": resolvePath(doc, op.path).push(copy(op.value)); break;
      case "copy_append": resolvePath(doc, op.path).push(copy(resolvePath(doc, op.from))); break;
      case "swap": { const a = copy(resolvePath(doc, op.path)), b = copy(resolvePath(doc, op.with)); setPath(doc, op.path, b); setPath(doc, op.with, a); break; }
      case "graft": setPath(doc, op.path, copy(resolvePath(JSON.parse(sourceText(sources, op.source)), op.from))); break;
      case "remove_where": { const array = resolvePath(doc, op.path); const kept = array.filter((item: Json) => !matches(item, op.match)); array.splice(0, array.length, ...kept); break; }
      case "set_where": for (const item of resolvePath(doc, op.path)) if (matches(item, op.match)) item[op.key] = copy(op.value); break;
      case "append_copy_where": { const array = resolvePath(doc, op.path); const found = copy(array.find((item: Json) => matches(item, op.match))); Object.assign(found, copy(op.set)); array.push(found); break; }
      case "nonfinite": setPath(doc, op.path, ({ Infinity: Infinity, "-Infinity": -Infinity, NaN: NaN } as Record<string, number>)[op.value]); break;
      case "json_text": setPath(doc, op.path, JSON.parse(op.value)); break;
      case "reseal": await reseal(doc, op); break;
      default: throw new Error(`unknown op ${op.op}`);
    }
  }
  return doc;
}
async function outcome(call: () => unknown): Promise<string> {
  try { await call(); return "accept"; } catch (error) { return error instanceof Error ? error.message : String(error); }
}
/** Desktop raw dispatch: explicit header dispatch, then the route's reader
 * (the TypeScript peer of Rust `for_source` / Python `_source_contract`). */
async function dispatch(doc: MechanicsResult): Promise<{ route: string; outcome: string }> {
  const route = sourceContract(doc);
  const result = await outcome(async () => {
    if (route === "unsupported") throw new Error("SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED");
    if (route === "load_reference") validateLoadReferenceEvidence(doc);
    else if (route === "load_reference_source") await validateLoadReferenceSourceEvidence(doc);
    else if (route === "physics") validatePhysicsEvidence(doc);
    else if (route === "physics_source") { await validatePhysicsSourceTransportMetadata(doc); validatePhysicsSourceEvidence(doc); }
    else if (route === "preview_physics") validatePreviewPhysicsEvidence(doc);
    else if (route !== "precision" && route !== "legacy") throw new Error(`route ${route} not exercised by this corpus`);
  });
  return { route, outcome: result };
}
const accepts = (value: string | null | undefined) => value === "accept";
function agreesCode(actual: string, expected: string): boolean {
  if (expected === PHYSICS_PREFIX || expected === JOIN_PREFIX) return actual.split(": ")[0] === expected;
  return actual === expected;
}
/** Outcome class pinned by the case file for one language. */
function pinned(c: Json, kind: string, language: "rust" | "python", fallback?: string): string {
  const specific = c[`${kind}_${language}`];
  if (typeof specific === "string") return specific;
  if (typeof c[kind] === "string") return c[kind];
  if (fallback) return pinned(c, fallback, language);
  throw new Error(`${c.id}: no ${kind}`);
}
function writeLog(name: string, log: unknown[]) {
  const dir = process.env.LOAD_REFERENCE_PARITY_OUT;
  if (!dir) return;
  mkdirSync(dir, { recursive: true });
  writeFileSync(resolve(dir, name), `${JSON.stringify(log, null, 1)}\n`);
}
/** LOAD_REFERENCE_PARITY_COMPARE holds `lr/` and `lrs/`, each with the
 * recorded rust_outcomes.json, rust_table_transport_outcomes.json and
 * python_outcomes.json of that identity's suite. */
function compareRecorded(file: string, log: Json[], keysByLanguage: string[]) {
  const dir = process.env.LOAD_REFERENCE_PARITY_COMPARE;
  if (!dir || !existsSync(resolve(dir, file))) return 0;
  const recorded: Json[] = JSON.parse(readFileSync(resolve(dir, file), "utf8"));
  const byId = new Map(recorded.map(entry => [entry.id, entry]));
  let compared = 0;
  for (const entry of log) {
    const other = byId.get(entry.id);
    expect(other, `${file}: ${entry.id} missing`).toBeDefined();
    for (const key of keysByLanguage) {
      if (!(key in entry) || !(key in other)) continue;
      expect(accepts(entry[key]), `${file} ${entry.id} ${key}: ts=${entry[key]} other=${other[key]}`).toBe(accepts(other[key]));
      compared += 1;
    }
  }
  expect(byId.size).toBe(log.length);
  return compared;
}

const rows = (list: Json[]): [string, Json][] => list.map(c => [c.id, c]);
const lrLog: Json[] = [];
const lrsLog: Json[] = [];
function metadataView(doc: Json): Json {
  const meta: Json = { schema_version: doc.schema_version };
  for (const key of ["producer", "numerical_quality", "formulation_basis", "contract_evidence", "source_block_recovery", "carrier_evidence"]) if (Object.hasOwn(doc, key)) meta[key] = doc[key];
  return meta;
}

describe("load-reference-1 shared corpus in TypeScript", () => {
  it("pins the transport schema bytes and the table bytes", async () => {
    await expect(verifyLoadReferenceTransportSchema(read("schemas/load_reference_state.schema.json"))).resolves.toBeTruthy();
    const table = await verifyLoadReferenceTable(read("fixtures/results/semantic_contract_v0_3_load_reference_1.json"));
    expect(table.rows).toEqual(JSON.parse(read("fixtures/results/semantic_contract_v0_3_physics_1.json")).rows);
  });

  it.each(rows(LR.cases))("case %s", async (_id, c: Json) => {
    const doc = await apply(LR.sources, c.source, c.ops) as MechanicsResult;
    const d = await dispatch(doc);
    const validator = await outcome(() => validateLoadReferenceEvidence(doc));
    lrLog.push({ id: c.id, dispatch: d.outcome, route: d.route, validator });
    for (const language of ["rust", "python"] as const) {
      expect(accepts(d.outcome), `${c.id} dispatch vs ${language}: ${d.outcome}`).toBe(accepts(pinned(c, "dispatch", language)));
      expect(accepts(validator), `${c.id} validator vs ${language}: ${validator}`).toBe(accepts(pinned(c, "validator", language, "dispatch")));
    }
    if (accepts(d.outcome)) expect(doc.producer?.semantic_contract_id).toBe(LOAD_REFERENCE_CONTRACT_ID);
    const python = TS_CODES[c.id] ?? pinned(c, "validator", "python", "dispatch");
    expect(agreesCode(validator, python), `${c.id} validator code: ${validator} != ${python}`).toBe(true);
  });

  it.each(rows(LR.table_cases))("table case %s", async (_id, c: Json) => {
    let text = read("fixtures/results/semantic_contract_v0_3_load_reference_1.json");
    if (c.find !== null) { expect(text.includes(c.find)).toBe(true); text = text.replace(c.find, c.replace); }
    const result = await outcome(() => verifyLoadReferenceTable(text));
    lrLog.push({ id: c.id, table: result });
    expect(result, c.id).toBe(c.expect);
  });

  it.each(rows(LR.transport_cases))("transport case %s", async (_id, c: Json) => {
    const doc = await apply(LR.sources, c.source, c.ops);
    const actual = await outcome(() => validateLoadReferenceTransportMetadata(metadataView(doc)));
    lrLog.push({ id: c.id, transport: actual });
    expect(agreesCode(actual, c.expect), `${c.id}: ${actual} != ${c.expect}`).toBe(true);
  });

  it("covers every shared case once and agrees with the recorded Rust and Python logs", () => {
    expect(lrLog.length).toBe(LR.cases.length + LR.table_cases.length + LR.transport_cases.length);
    expect(new Set(lrLog.map(e => e.id)).size).toBe(lrLog.length);
    writeLog("ts_outcomes.json", lrLog);
    const rust = compareRecorded("lr/rust_outcomes.json", lrLog.filter(e => "dispatch" in e), ["dispatch", "validator"]);
    const rustTables = compareRecorded("lr/rust_table_transport_outcomes.json", lrLog.filter(e => !("dispatch" in e)), ["table", "transport"]);
    const python = compareRecorded("lr/python_outcomes.json", lrLog, ["dispatch", "validator", "table", "transport"]);
    if (process.env.LOAD_REFERENCE_PARITY_COMPARE) expect(rust * rustTables * python).toBeGreaterThan(0);
  });
});

describe("load-reference-source-1 shared corpus in TypeScript", () => {
  it("pins the joined table bytes", async () => {
    const table = await verifyLoadReferenceSourceTable(read("fixtures/results/semantic_contract_v0_3_load_reference_source_1.json"));
    expect(table.rows).toEqual(JSON.parse(read("fixtures/results/semantic_contract_v0_3_physics_source_1.json")).rows);
  });

  it.each(rows(LRS.cases))("case %s", async (_id, c: Json) => {
    const doc = await apply(LRS.sources, c.source, c.ops) as MechanicsResult;
    const d = await dispatch(doc);
    const joined = await outcome(() => validateLoadReferenceSourceEvidence(doc));
    const entry: Json = { id: c.id, dispatch: d.outcome, route: d.route, joined };
    if ("lr" in c) entry.lr = await outcome(() => validateLoadReferenceEvidence(doc));
    if ("ps" in c) entry.ps = await outcome(async () => { await validatePhysicsSourceTransportMetadata(doc); validatePhysicsSourceEvidence(doc); });
    lrsLog.push(entry);
    for (const language of ["rust", "python"] as const) {
      expect(accepts(d.outcome), `${c.id} dispatch vs ${language}: ${d.outcome}`).toBe(accepts(pinned(c, "dispatch", language)));
      expect(accepts(joined), `${c.id} joined vs ${language}: ${joined}`).toBe(accepts(pinned(c, "joined", language, "dispatch")));
      for (const kind of ["lr", "ps"]) if (kind in c) expect(accepts(entry[kind]), `${c.id} ${kind}: ${entry[kind]}`).toBe(accepts(c[kind]));
    }
    const contract = c.accept_contract ?? LOAD_REFERENCE_SOURCE_CONTRACT_ID;
    if (accepts(d.outcome)) expect(doc.producer?.semantic_contract_id).toBe(contract);
    const python = TS_CODES[c.id] ?? pinned(c, "joined", "python", "dispatch");
    expect(agreesCode(joined, python), `${c.id} joined code: ${joined} != ${python}`).toBe(true);
    if ("lr" in c) expect(agreesCode(entry.lr, c.lr), `${c.id} lr code: ${entry.lr} != ${c.lr}`).toBe(true);
  });

  it.each(rows(LRS.table_cases))("table case %s", async (_id, c: Json) => {
    let text = read(c.file ?? "fixtures/results/semantic_contract_v0_3_load_reference_source_1.json");
    if (c.find !== null) { expect(text.includes(c.find)).toBe(true); text = text.replace(c.find, c.replace); }
    const result = await outcome(() => verifyLoadReferenceSourceTable(text));
    lrsLog.push({ id: c.id, table: result });
    expect(result, c.id).toBe(c.expect);
  });

  it.each(rows(LRS.transport_cases))("transport case %s", async (_id, c: Json) => {
    const doc = await apply(LRS.sources, c.source, c.ops);
    const result = await outcome(() => validateLoadReferenceSourceTransportMetadata(metadataView(doc)));
    lrsLog.push({ id: c.id, transport: result });
    expect(agreesCode(result, c.expect), `${c.id}: ${result} != ${c.expect}`).toBe(true);
  });

  it("covers every shared case once and agrees with the recorded Rust and Python logs", () => {
    expect(lrsLog.length).toBe(LRS.cases.length + LRS.table_cases.length + LRS.transport_cases.length);
    expect(new Set(lrsLog.map(e => e.id)).size).toBe(lrsLog.length);
    writeLog("ts_source_outcomes.json", lrsLog);
    const rust = compareRecorded("lrs/rust_outcomes.json", lrsLog.filter(e => "dispatch" in e), ["dispatch", "joined", "lr", "ps"]);
    const rustTables = compareRecorded("lrs/rust_table_transport_outcomes.json", lrsLog.filter(e => !("dispatch" in e)), ["table", "transport"]);
    const python = compareRecorded("lrs/python_outcomes.json", lrsLog, ["dispatch", "joined", "lr", "ps", "table", "transport"]);
    if (process.env.LOAD_REFERENCE_PARITY_COMPARE) expect(rust * rustTables * python).toBeGreaterThan(0);
  });
});
