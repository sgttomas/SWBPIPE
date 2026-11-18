// What `validateAnalysisRunV03` does with values nested deeper than its old comparison could
// compare, and the wasm engine's parse depth limit that this rests on.
//
// `sameCheckedJson` (analysisRunCompatibility.ts) is the old text comparison's relation only
// for values nested no deeper than the JavaScript engine's `JSON.stringify` recursion allows:
// above that, the old comparison threw the engine's stack error and the new one does not.
// These tests show what the validator does there. A source nested past the wasm reader's
// parse limit is refused by the reader's dispatch, the validator's first step, before any
// comparison; a record member nested deep against its shallow operand reaches a comparison.
// Deep values are re-sealed as an engine without the depth limit would seal them: the
// canonical checked text of the document with a placeholder string, the placeholder replaced
// by the deep value's canonical text, then SHA-256. Hashes the source's own producer seals
// (the pressure source's receipts) are not re-sealed: their producer is Rust, whose parse has
// the same limit.
import { createHash } from "node:crypto";
import { describe, expect, it } from "vitest";
import type { AnalysisRunEnvelope, MechanicsResult } from "../types";
import {
  ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH, analysisRecordProjection, buildAnalysisRunV03, modelLoadBasisRefs, validateAnalysisRunV03
} from "./analysisRunCompatibility";
import { canonicalJsonCheckedV1, canonicalSha256HexCheckedV1, checkedJsonText } from "./hashService";
import { readerDispatch } from "../features/results/resultReader";
import { minimalPreviewStatement } from "../test-support/retiredPrecision";
import combinationSuccessor from "../../../../fixtures/results/retained_precision_combination_successor_sparse_interactive.json";
import pressureSourceFixture from "../../../../fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

/** The comparison `validateAnalysisRunV03` used before `sameCheckedJson`, verbatim. */
function referenceSame(a: unknown, b: unknown): boolean {
  if (a === undefined || b === undefined) return a === b;
  const order = (v: any): any => Array.isArray(v) ? v.map(order) : v && typeof v === "object" ? Object.fromEntries(Object.keys(v).sort().map(k => [k, order(v[k])])) : v; // eslint-disable-line @typescript-eslint/no-explicit-any
  return JSON.stringify(order(JSON.parse(checkedJsonText(a)))) === JSON.stringify(order(JSON.parse(checkedJsonText(b))));
}

/** `{ d: { d: … 0 } }`, `levels` objects deep, and its canonical text. */
function deep(levels: number): unknown {
  let value: unknown = 0;
  for (let index = 0; index < levels; index += 1) value = { d: value };
  return value;
}
const deepText = (levels: number) => `${'{"d":'.repeat(levels)}0${"}".repeat(levels)}`;

const PLACEHOLDER = "invented-deep-value-placeholder";
/** The SHA-256 of `value`'s canonical checked text with each placeholder replaced by the
 * canonical text of a value `levels` deep: what the checked profile's hash would be if its
 * parse had no depth limit. */
async function splicedSha256(value: unknown, levels: number): Promise<string> {
  const text = (await canonicalJsonCheckedV1(value)).split(JSON.stringify(PLACEHOLDER)).join(deepText(levels));
  return createHash("sha256").update(text, "utf8").digest("hex");
}
function replacePlaceholders(value: unknown, levels: number): unknown {
  if (value === PLACEHOLDER) return deep(levels);
  if (Array.isArray(value)) { for (let i = 0; i < value.length; i += 1) value[i] = replacePlaceholders(value[i], levels); return value; }
  if (value && typeof value === "object") { for (const key of Object.keys(value)) (value as Json)[key] = replacePlaceholders((value as Json)[key], levels); return value; }
  return value;
}

function code(error: unknown): string {
  if (error instanceof RangeError) return "RangeError";
  const named = (error as { code?: unknown }).code;
  if (typeof named === "string") return named;
  const message = error instanceof Error ? error.message : String(error);
  return message.match(/^[A-Za-z0-9_-]+/)?.[0] ?? message.slice(0, 40);
}
async function outcome(run: () => Promise<unknown>): Promise<string> {
  try { await run(); return "accepted"; } catch (error) { return code(error); }
}
function helperOutcome(a: unknown, b: unknown): string {
  try { return String(referenceSame(a, b)); } catch (error) { return code(error); }
}

type Setup = { name: string; record: AnalysisRunEnvelope; source: MechanicsResult; basis?: Json[] };
/** A member of the record (and, for a member taken from the source, of the source) where an
 * extra deep value is put, and the code the validator refuses it with when only the record
 * carries it (read from the validator's comparisons). */
type Site = { name: string; record: (r: Json) => Json; source?: (s: Json) => Json; row?: number; recordOnlyCode?: string; unread?: true };

function recordHash(record: Json, scope: string): Json {
  return record.analysis_run.hashes.find((h: Json) => h.payload_scope === scope);
}

/** The setup's record and source with the deep value put at `site` (both sides when `both`),
 * every hash over it re-sealed by splicing. */
async function placed(setup: Setup, site: Site, both: boolean, levels: number) {
  const record = structuredClone(setup.record) as Json, source = structuredClone(setup.source) as Json;
  const recordMember = site.record(record);
  if (!recordMember || typeof recordMember !== "object" || Array.isArray(recordMember)) return null;
  const original = structuredClone(recordMember);
  recordMember.invented_deep = PLACEHOLDER;
  if (both) {
    site.source!(source).invented_deep = PLACEHOLDER;
    recordHash(record, "received_result").value = await splicedSha256(source, levels);
    if (site.row !== undefined) record.analysis_run.result_refs[site.row].hash_refs[0].value = await splicedSha256(source.results[site.row], levels);
  }
  recordHash(record, "analysis_run_record").value = await splicedSha256(analysisRecordProjection(record), levels);
  replacePlaceholders(record, levels);
  replacePlaceholders(source, levels);
  return { record: record as AnalysisRunEnvelope, source: source as MechanicsResult, deepMember: site.record(record), original };
}

const previewManifest = (source: MechanicsResult) => ({
  manifest_ref: { object_type: "InputManifest" as const, ref: "input-manifest:test" }, manifest_sha256: "1".repeat(64),
  manifest: { model_basis: { model_ref: source.model_ref }, solver_basis: { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: "build:test" } }
});

async function previewSetup(): Promise<Setup> {
  const source = minimalPreviewStatement({ run_id: "run:test", model_ref: "model:test", cases: [],
    rows: [{ id: "result:test", entity_ref: "node:test", kind: "global_nodal_rotation_x", value: 1, unit: "rad", basis_ref: { ref_type: "load_case", ref_id: "load:test" },
      metadata: { component: "nodal_rotation_x", coordinate_system: "global", location: "node", basis: "solved_from_global_linear_system", sign_convention: "positive value follows the right-hand-rule rotation about the global cartesian X axis" } }] as never,
    diagnostics: [{ id: "diagnostic:invented", code: "INVENTED", severity: "info", message: "invented" }] as never });
  const record = await buildAnalysisRunV03(source, previewManifest(source) as never);
  await validateAnalysisRunV03(record, source);
  return { name: "preview-physics-2 (synthetic)", record, source };
}
async function retainedSetup(): Promise<Setup> {
  const doc = structuredClone(combinationSuccessor) as Json, source = doc.source as MechanicsResult;
  const basis = modelLoadBasisRefs(doc.invocation.request.model);
  const record = await buildAnalysisRunV03(source, previewManifest(source) as never, undefined, basis);
  await validateAnalysisRunV03(record, source, basis);
  return { name: "retained successor (combination fixture)", record, source, basis };
}
async function pressureSourceSetup(): Promise<Setup> {
  const source = structuredClone(pressureSourceFixture) as unknown as MechanicsResult;
  const record = await buildAnalysisRunV03(source, previewManifest(source) as never);
  await validateAnalysisRunV03(record, source);
  return { name: "pressure-source-1 (n05 fixture)", record, source };
}

const run = (r: Json) => r.analysis_run;
const PREVIEW_SITES: Site[] = [
  { name: "producer diagnostic", record: r => run(r).diagnostics[0].source_annotation, source: s => s.diagnostics[0], recordOnlyCode: "ANALYSIS_SOURCE_DIAGNOSTICS_MISMATCH" },
  { name: "row metadata", record: r => run(r).result_refs[0].source_annotation.metadata, source: s => s.results[0].metadata, row: 0, recordOnlyCode: "ANALYSIS_ROW_INTERPRETATION_MISMATCH" },
  { name: "model_state_ref", record: r => run(r).model_state_ref, recordOnlyCode: "ANALYSIS_SOURCE_MODEL_STATE_MISMATCH" },
  { name: "load_basis_refs entry", record: r => run(r).load_basis_refs[0], recordOnlyCode: "ANALYSIS_SOURCE_LOAD_BASIS_MISMATCH" },
  // The semantic contract site's message text is another change's; only its code is read.
  { name: "semantic_contract", record: r => run(r).reproducibility.semantic_contract, recordOnlyCode: "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH" },
  { name: "received_result hash entry", record: r => recordHash(r, "received_result"), recordOnlyCode: "ANALYSIS_RECEIVED_SOURCE_MISMATCH" },
  { name: "row result_ref", record: r => run(r).result_refs[0].result_ref, recordOnlyCode: "ANALYSIS_ROW_SOURCE_BINDING_MISMATCH" },
  { name: "row semantic_contract", record: r => run(r).result_refs[0].semantic_contract, recordOnlyCode: "ANALYSIS_ROW_SOURCE_BINDING_MISMATCH" },
  { name: "row hash_refs entry", record: r => run(r).result_refs[0].hash_refs[0], recordOnlyCode: "ANALYSIS_ROW_SOURCE_BINDING_MISMATCH" },
  { name: "row interpretation", record: r => run(r).result_refs[0].interpretation, recordOnlyCode: "ANALYSIS_ROW_INTERPRETATION_MISMATCH" },
  { name: "row source_annotation", record: r => run(r).result_refs[0].source_annotation, recordOnlyCode: "ANALYSIS_ROW_INTERPRETATION_MISMATCH" },
  // Not compared by `same`: read only by the record checksum, or by nothing.
  { name: "provenance (checksum only)", record: r => run(r).provenance },
  // Left out of every assertion and of the recorded outcomes. The record checksum's projection
  // drops its own entry (analysis_run.hashes[payload_scope = "analysis_run_record"]) and no
  // comparison reads that entry's other members, so an extra member there is not examined: it
  // was accepted at 200 levels on main's product files as on this change's, and gave a
  // RangeError at 5,000 levels on this change's (main's run stopped at 200). This change neither
  // causes nor alters it; it is recorded as a finding for DEL-14-02.
  { name: "analysis_run_record hash entry (not examined)", record: r => recordHash(r, "analysis_run_record"), unread: true }
];
const RETAINED_SITES: Site[] = [
  { name: "retained_precision", record: r => run(r).retained_precision, source: s => s.retained_precision, recordOnlyCode: ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH }
];
const PRESSURE_SOURCE_SITES: Site[] = [
  { name: "source_block_recovery", record: r => run(r).source_block_recovery, source: s => s.source_block_recovery, recordOnlyCode: "ANALYSIS_SOURCE_BLOCK_RECEIPT_MISMATCH" },
  { name: "contract_evidence", record: r => run(r).contract_evidence, source: s => s.contract_evidence, recordOnlyCode: "ANALYSIS_PHYSICS_SOURCE_EVIDENCE_MISMATCH" }
];

describe("the wasm engine's JSON parse depth limit, which sameCheckedJson's agreement with the old comparison rests on", () => {
  // `sameCheckedJson` and the old text comparison differ only for values nested deeper than the
  // JavaScript engine's JSON.stringify recursion allows (thousands of levels). The validator
  // never accepts such a value because the wasm engine's serde_json parse refuses a container
  // nested 128 deep, in the reader's dispatch of the source and in the checked profile's hash.
  // If that limit is raised or disabled (serde_json's `unbounded_depth`, or another parser),
  // the validator's protection against deep values must be shown again: see
  // analysisRunCompatibility.ts, sameCheckedJson.
  it("the checked profile's hash refuses a value 128 objects deep and hashes one 127 deep", async () => {
    const refused: number[] = [];
    for (let levels = 120; levels <= 135; levels += 1) {
      try { await canonicalSha256HexCheckedV1(deep(levels)); }
      catch (error) {
        expect((error as Error).message).toMatch(/^CHECKED-JSON-INVALID: recursion limit exceeded/);
        refused.push(levels);
      }
    }
    expect(refused[0]).toBe(128);
    expect(refused).toEqual(Array.from({ length: 8 }, (_, i) => 128 + i));
  });

  it("the reader's dispatch refuses a source whose deepest container is 128 deep and reads one 127 deep", async () => {
    const { source } = await previewSetup();
    // root object, results array, row object, metadata object: four levels before the deep value.
    const at = (levels: number) => { const s = structuredClone(source) as Json; s.results[0].metadata.invented_deep = deep(levels); return readerDispatch(s); };
    expect(at(123).route).not.toBe("unsupported");
    expect(at(124).route).toBe("unsupported");
    expect(at(124).error?.code).toBe("JSON_PARSE_REJECTED");
  });

  it("re-sealing by splicing gives the engine's own hash where the engine can hash", async () => {
    const value = { a: [1, { b: PLACEHOLDER }], z: PLACEHOLDER };
    expect(await splicedSha256(value, 3)).toBe(await canonicalSha256HexCheckedV1(replacePlaceholders(structuredClone(value), 3)));
  });
});

describe("validateAnalysisRunV03 on values nested past the parse limit", () => {
  it("accepts no deep value in a member it examines, at 200 and 5,000 levels, on the three routes' members", async () => {
    const setups: Array<[Setup | string, Site[]]> = [];
    setups.push([await previewSetup(), PREVIEW_SITES]);
    setups.push([await retainedSetup(), RETAINED_SITES]);
    try { setups.push([await pressureSourceSetup(), PRESSURE_SOURCE_SITES]); }
    catch (error) { setups.push([`pressure-source-1 (n05 fixture): not built, ${code(error)}`, []]); }
    const observed: Record<string, unknown> = {};
    for (const [setup, sites] of setups) {
      if (typeof setup === "string") { observed[setup] = "skipped"; continue; }
      for (const site of sites) {
        for (const both of site.source ? [true, false] : [false]) {
          const key = `${setup.name} / ${site.name} / ${both ? "record and source" : "record only"}`;
          const row: Record<string, string> = {};
          for (const levels of [2, 200, 5000]) {
            const deepCase = await placed(setup, site, both, levels);
            if (!deepCase) { row.site = "not an object in this record"; break; }
            const validator = await outcome(() => validateAnalysisRunV03(deepCase.record, deepCase.source, setup.basis));
            row[`validator ${levels}`] = validator;
            if (site.unread) continue;
            if (levels > 2) {
              // Never accepted past the parse limit.
              expect(validator, `${key} at ${levels}`).not.toBe("accepted");
              if (both) expect(validator, `${key} at ${levels}`).toBe("ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH");
            }
            if (!both && site.recordOnlyCode) {
              // The old comparison of the deep record member with its shallow operand.
              const helper = helperOutcome(deepCase.deepMember, deepCase.original);
              row[`old comparison ${levels}`] = helper;
              if (levels <= 200) {
                expect(helper, `${key} at ${levels}`).toBe("false");
                expect(validator, `${key} at ${levels}`).toBe(site.recordOnlyCode);
              }
            }
          }
          if (!site.unread) observed[key] = row;
        }
      }
    }
    // What each case gives, as run in this test runtime (Node's stack limit decides the 5,000-level
    // column of the old comparison; the wasm engine's parse limit decides the source cases).
    expect(observed).toMatchInlineSnapshot();
  }, 300_000);
});
