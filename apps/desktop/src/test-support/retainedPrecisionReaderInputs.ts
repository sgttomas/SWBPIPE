import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { canonicalSha256HexCheckedV1 } from "../services/hashService";
import { readRetainedPrecisionCorpus } from "./retainedPrecisionCorpus";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
export type ReaderInput = { key: string; build: () => Promise<{ source: Json; invocation: Json | undefined }> };

const PROJECT_ROOT = resolve(__dirname, "../../../../");
const readJson = (path: string): Json => JSON.parse(readFileSync(resolve(PROJECT_ROOT, path), "utf8"));
const DEF_O = "64ec69b842d6278323aa2ab2e140e999db52eb28e4a9965a567a90b04e228b41";
/** DEF-E's H, as result_export's `retained_precision::EXACT_DEFINITION_HASH` (re-based on
 * pressure-1 by a43583c625, B7b D4 (a); T4-U6's DEF-E states zero mill tolerance and lists the
 * reduced-basis rows); the rehash mirrors that constant, it states no result. */
const DEF_E = "bd3c7de76f4ba2d1f7a72894c173e9c449e27f318359de648f912a192f7cb031";
const EXACT_ID = "openpipestress.result_semantics/0.3.0/physics-retained-1";
/** The shared corpus's successor fixtures (`retained_precision_*_successor_*.json`). */
export const SUCCESSOR_FIXTURES = [
  "combination_successor_dense_scrutiny", "combination_successor_sparse_interactive",
  "exact_successor_dense_scrutiny", "exact_successor_derivative_dense_scrutiny", "exact_successor_derivative_sparse_interactive",
  "exact_successor_sparse_interactive", "l0_successor_dense_scrutiny", "l0_successor_sparse_interactive",
  "milestone_successor_dense_scrutiny", "milestone_successor_sparse_interactive", "w_c2_successor_dense_scrutiny", "w_c2_successor_sparse_interactive",
] as const;

const domainHash = (domain: string, payload: Json) => canonicalSha256HexCheckedV1({ domain, payload });
/** A strict index (07e format rule), else undefined. */
function index(v: unknown): number | undefined {
  return typeof v === "number" && Number.isInteger(v) && v >= 0 && !Object.is(v, -0) ? v : undefined;
}
const at = (items: Json[] | undefined, ref: unknown): Json => { const i = index(ref); return i === undefined || !items ? undefined : items[i]; };
/** As result_export's `retained_precision_contract.rs` `edit`. */
function edit(root: Json, e: Json): void {
  const path: unknown[] = e.path;
  let parent = root;
  for (const key of path.slice(0, -1)) parent = parent[(index(key) ?? key) as string | number];
  const last = path[path.length - 1], i = index(last);
  if (e.op === "remove") { if (i !== undefined) parent.splice(i, 1); else delete parent[last as string]; }
  else parent[(i ?? last) as string | number] = structuredClone(e.value);
}
/** As result_export's `rehash` (the 07e format rule with 07o's steps). */
async function rehash(source: Json): Promise<void> {
  const b = source?.retained_precision?.body;
  if (!b || typeof b !== "object" || Array.isArray(b)) return;
  const h = source.producer?.semantic_contract_id === EXACT_ID ? DEF_E : DEF_O;
  const members = (x: Json) => x.preparation.members.map((m: Json) => ({ member: m.member, old_source: m.old_source, old_facts: m.old_facts, section: m.result.section }));
  const attempts = structuredClone(b.product_attempts);
  for (const s of b.sources) {
    const a = at(attempts, s.preparation?.attempt_ref);
    if (!a || typeof a !== "object" || !a.preparation.members.every((m: Json) => m.result.kind === "prepared")) continue;
    s.preparation.sha256 = await domainHash("retained_precision_preparation_v1", { definition_id: a.definition_id, definition_sha256: h, owner_ref: a.owner_ref,
      ordinary_attempt_ref: a.ordinary_attempt_ref, material_basis_ref: a.material_basis_ref, members: members(a) });
  }
  const records = structuredClone(b.operand_preparations);
  for (const s of b.sources) {
    const p = at(records, s.preparation?.operand_preparation_ref);
    if (!p || typeof p !== "object" || p.result?.kind !== "prepared" || !p.preparation.members.every((m: Json) => m.result.kind === "prepared")) continue;
    s.preparation.sha256 = await domainHash("retained_precision_operand_preparation_v1", { definition_id: p.definition_id, definition_sha256: DEF_O, owner_ref: p.owner_ref,
      ordinary_attempt_ref: p.ordinary_attempt_ref, material_basis_ref: p.material_basis_ref, purpose: p.purpose, members: members(p) });
  }
  const identity = (s: Json) => { const { index: _, ...rest } = s; return domainHash("retained_precision_source_mp_v2", rest); };
  let sources = structuredClone(b.sources);
  for (const c of b.cases) {
    if (c.status !== "selected") continue;
    const s = at(sources, c.source_ref);
    if (s && typeof s === "object" && !Array.isArray(s)) c.source_identity_sha256 = await identity(s);
  }
  for (const s of b.sources) {
    if (s.owner?.kind !== "combination") continue;
    // Rust's `s["operands"].as_array_mut()` inserts a missing member as null; so does this rule.
    if (!Object.hasOwn(s, "operands")) s.operands = null;
    for (const o of Array.isArray(s.operands) ? s.operands : []) {
      const named = at(sources, o.source_ref);
      if (named && typeof named === "object" && !Array.isArray(named)) o.source_identity_sha256 = await identity(named);
    }
  }
  sources = structuredClone(b.sources);
  if (!Object.hasOwn(b, "combinations")) b.combinations = null; // as `b["combinations"].as_array_mut()`
  for (const c of Array.isArray(b.combinations) ? b.combinations : []) {
    if (c.disposition !== "retained_selected") continue;
    const s = at(sources, c.source_ref);
    if (s && typeof s === "object" && !Array.isArray(s)) c.source_identity_sha256 = await identity(s);
  }
  const { retained_precision: _, ...publication } = source;
  b.publication_sha256 = await domainHash("retained_precision_publication_mp_v2", publication);
  source.retained_precision.receipt_sha256 = await domainHash("retained_precision_receipt_mp_v2", b);
}
/** As result_export's `apply_on`: edits, invocation edits (rebinding the receipt's invocation digest), rehash, after-rehash edits. */
async function applyOn(base: Json, entry: Json): Promise<{ source: Json; invocation: Json }> {
  const source = structuredClone(base.source), invocation = structuredClone(base.invocation);
  for (const e of entry.edits ?? []) edit(source, e);
  const invocationEdits: Json[] = entry.invocation_edits ?? [];
  for (const e of invocationEdits) edit(invocation, e);
  if (invocationEdits.length) source.retained_precision.body.invocation.value = await domainHash("source_blocks_invocation_v1", invocation);
  await rehash(source);
  for (const e of entry.after_rehash ?? []) edit(source, e);
  return { source, invocation };
}

/** Every reader input of the shared retained corpus, keyed, in the order of result_export's
 * `retained_precision_contract.rs` `r1_reader_inputs` (whose outcomes are committed in
 * `fixtures/results/retained_precision_reader_outcomes.json`). Each input is built on demand. */
export function retainedReaderInputs(): ReaderInput[] {
  const shared = readRetainedPrecisionCorpus();
  const caseOf = (id: string) => shared.cases.find((c: Json) => c.id === id);
  const out: ReaderInput[] = [];
  for (const c of shared.cases) out.push({ key: `corpus:case:${c.id}`, build: async () => ({ source: structuredClone(c.source), invocation: structuredClone(c.invocation) }) });
  for (const [kind, list] of [["mutation", "mutations"], ["must_pass", "must_pass"]] as const) {
    for (const e of shared[list]) {
      if (e.rehash !== "all") throw new Error(`${e.id}: rehash ${e.rehash}`);
      out.push({ key: `corpus:${kind}:${e.id}`, build: () => applyOn(caseOf(e.base), e) });
    }
  }
  for (const s of readJson("fixtures/results/retained_precision_b2_parity_probes.json").shapes) {
    if (s.rehash !== "all") throw new Error(`${s.name}: rehash ${s.rehash}`);
    out.push({ key: `b2_parity:${s.name}`, build: () => applyOn(caseOf(s.base), s) });
  }
  for (const s of readJson("fixtures/results/retained_precision_rv120_b3_inputs.json").shapes) {
    const [kind, id] = s.base;
    const base = kind === "corpus" ? caseOf(id) : kind === "m3x_producer" ? readJson(`fixtures/results/retained_precision_exact_successor_${id}.json`) : null;
    if (!base) throw new Error(`rv120 base ${kind}`);
    out.push({ key: `rv120:${s.name}`, build: () => applyOn(base, s) });
  }
  for (const name of SUCCESSOR_FIXTURES) {
    const doc = readJson(`fixtures/results/retained_precision_${name}.json`);
    out.push({ key: `successor:${name}`, build: async () => Object.hasOwn(doc, "result_envelope")
      ? { source: structuredClone(doc.result_envelope), invocation: undefined }
      : { source: structuredClone(doc.source), invocation: structuredClone(doc.invocation) } });
  }
  const carriers = readJson("fixtures/results/retained_precision_carrier_cases.json");
  const fixture = (id: string): { source: Json; invocation: Json } => {
    const f = carriers.fixtures[id], doc = readJson(f.path);
    return f.shape === "milestone" ? { source: doc.source, invocation: doc.invocation } : { source: doc, invocation: null };
  };
  // As the carriers apply them (`retained_precision_carriers.rs` `apply_shared`).
  const apply = (c: Json, f: { source: Json; invocation: Json }) => {
    const source = structuredClone(f.source), invocation = structuredClone(f.invocation), targets: Json = { source, invocation };
    for (const e of c.edits) {
      if (e.op !== "set") throw new Error(`carrier edit ${e.op}`);
      let parent = targets, key: string | number = e.target === "source" ? "source" : "invocation";
      for (const k of e.path) { parent = parent[key]; key = k; }
      parent[key] = structuredClone(e.value);
    }
    const bound = c.invocation !== null && typeof c.invocation === "object" ? structuredClone(c.invocation) : c.invocation === null ? undefined : targets.invocation;
    return { source: targets.source, invocation: bound };
  };
  for (const c of carriers.cases) out.push({ key: `carrier:case:${c.id}`, build: async () => apply(c, fixture(c.fixture)) });
  for (const d of carriers.declared_differences) for (const f of d.forms) for (const id of f.fixtures) {
    out.push({ key: `carrier:declared:${d.id}:${f.label}:${id}`, build: async () => apply(f, fixture(id)) });
  }
  return out;
}
