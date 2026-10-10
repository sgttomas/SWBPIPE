import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import connectedSparse from "../../../../../fixtures/results/preview_physics_connected_sparse.json";
import connectedDense from "../../../../../fixtures/results/preview_physics_connected_dense.json";
import inventedSparse from "../../../../../fixtures/results/preview_physics_invented_sparse.json";
import inventedDense from "../../../../../fixtures/results/preview_physics_invented_dense.json";
import inventedModel from "../../../../../core/product_physics/tests/fixtures/preview_physics_invented_model.json";
import connectedModel from "../../../../../fixtures/model_operations/precision_connected_ui_model.json";
import precisionSparse from "../../../../../fixtures/results/precision_connected_ui_mechanics_sparse.json";
import table from "../../../../../fixtures/results/semantic_contract_v0_3_preview_physics_1.json";
import unicodeIdsSparse from "../../../../../fixtures/results/preview_physics_unicode_ids_sparse.json";
import unicodeIdsModel from "../../../../../core/product_physics/tests/fixtures/preview_physics_unicode_ids_model.json";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import { compareCodePoints, consistentNorm, lengthPrefixed, lengthPrefixedSegments, previewPhysicsSignature, validatePreviewPhysicsEvidence, validatePreviewPhysicsTransportMetadata } from "./previewPhysicsEvidence";
import { numericalResultStanding, sourceContract, sourceSemanticBinding, PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_SHA256 } from "./numericalResultQuality";
import { resultSemantics, semanticContractForSource } from "./resultSemantics";
import type { MechanicsResult, PreviewModel } from "../../types";

const clone = (value: unknown) => structuredClone(value) as MechanicsResult;
const fixtures = { connectedSparse, connectedDense, inventedSparse, inventedDense } as Record<string, unknown>;
const invalid = (source: MechanicsResult, detail?: string) =>
  expect(() => validatePreviewPhysicsEvidence(source)).toThrow(detail ? `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: ${detail}` : "SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID");
const rowsOf = (source: MechanicsResult, kind: string) => source.results.filter(r => r.kind === kind);
const evidence = (source: MechanicsResult) => source.contract_evidence as any;

describe("preview-physics-1 registry and dispatch", () => {
  it("pins the frozen table bytes and dispatches the producer output", async () => {
    // The TS import is the parsed table; the byte pin is checked by the generator's --check.
    expect(table.semantic_contract_id).toBe(PREVIEW_PHYSICS_CONTRACT_ID);
    expect(PREVIEW_PHYSICS_CONTRACT_SHA256).toBe("ae55503d44a4750714a35c423623e38cf4132099134097193024d1635bfbc88a");
    expect(table.source_signature_count).toBe(73);
    for (const fixture of Object.values(fixtures)) {
      const source = clone(fixture);
      expect(sourceContract(source)).toBe("preview_physics");
      expect(sourceSemanticBinding(source)).toEqual({ id: PREVIEW_PHYSICS_CONTRACT_ID, sha256: PREVIEW_PHYSICS_CONTRACT_SHA256 });
      expect(semanticContractForSource(source)).toBe(table);
    }
    expect(await canonicalSha256HexCheckedV1(table)).toMatch(/^[0-9a-f]{64}$/);
  });
  it("keeps precision-1 readable but never Current", () => {
    const source = clone(precisionSparse);
    expect(sourceContract(source)).toBe("precision");
    const standing = numericalResultStanding(source, connectedModel as unknown as PreviewModel);
    expect(standing.eligible).toBe(false);
    expect(standing.findings).toContain("PRECISION_1_HISTORICAL_SEMANTICS");
  });
  it("refuses a preview identity without contract evidence or with a receipt namespace", () => {
    const missing = clone(connectedSparse); delete missing.contract_evidence;
    expect(sourceContract(missing)).toBe("unsupported");
    const receipt = clone(connectedSparse) as any; receipt.source_block_recovery = {};
    expect(sourceContract(receipt)).toBe("unsupported");
  });
  it("selects arc variants by source_basis, first match in table order", () => {
    const row = { id: "r", kind: "element_local_axial_normal_stress", value: 1, unit: "MPa", entity_ref: "pipe:a", metadata: { component: "axial_normal_stress", coordinate_system: "element_local", location: "end_i", basis: "nominal_straight_beam_formula_on_arc_resultants", sign_convention: "x" } };
    const source = clone(connectedSparse);
    expect(resultSemantics(row, source)?.signature_id).toBe("preview-physics-arc-015");
    expect(previewPhysicsSignature(row)?.signature_id).toBe("preview-physics-arc-015");
    const straight = { ...row, metadata: { ...row.metadata, basis: "recovered_from_open_mechanics_stress_components" } };
    expect(resultSemantics(straight, source)?.signature_id).not.toBe("preview-physics-arc-015");
    expect((resultSemantics(straight, source) as { source_basis?: string }).source_basis).toBeUndefined();
  });
});

describe("preview-physics-1 reader checks on actual producer output", () => {
  it.each(Object.keys(fixtures))("%s passes every S1 §9 check and the numerical standing", (name) => {
    const source = clone(fixtures[name]);
    expect(() => validatePreviewPhysicsEvidence(source)).not.toThrow();
    expect(() => validatePreviewPhysicsTransportMetadata(source)).not.toThrow();
    const model = (name.startsWith("connected") ? connectedModel : inventedModel) as unknown as PreviewModel;
    expect(numericalResultStanding(source, model).findings).toEqual([]);
  });
  it("accepts non-result references (hanger, DEC-046, user source references) as a positive control (F-1)", () => {
    const source = clone(inventedSparse);
    const refs = source.diagnostics.flatMap(d => d.affected_refs ?? []);
    expect(refs.some(ref => !ref.startsWith("result:") && !source.results.some(r => r.id === ref))).toBe(true);
    expect(source.diagnostics.some(d => d.affected_refs?.includes("hanger"))).toBe(true);
    expect(source.diagnostics.some(d => d.affected_refs?.includes("DEC-046"))).toBe(true);
    expect(() => validatePreviewPhysicsEvidence(source)).not.toThrow();
  });
  it("uses UTF-8 byte lengths and code-point order", () => {
    expect(lengthPrefixed("é", "a")).toBe("2:é1:a");
    expect(lengthPrefixedSegments("é", "a")).toBe("2:é:1:a");
    expect(compareCodePoints("\u{10000}", "￿")).toBe(1); // UTF-16 order would say -1
    expect(compareCodePoints("a", "ab")).toBe(-1);
  });
});

describe("preview-physics-1 tamper tests (DESIGN §9.2)", () => {
  it("refuses a retired kind", () => {
    const source = clone(inventedSparse);
    source.results.push({ ...source.results[0], id: "result:reaction:support-S-100", kind: "reaction_resultant", unit: "N" });
    invalid(source, "retired kind reaction_resultant");
  });
  it("refuses a retired diagnostic code", () => {
    const source = clone(inventedSparse);
    source.diagnostics.push({ ...source.diagnostics[0], id: "diagnostic:tamper", code: "COMPONENT_STRESS_MULTIPLIER_APPLIED" });
    invalid(source, "retired diagnostic code COMPONENT_STRESS_MULTIPLIER_APPLIED");
  });
  it("refuses a dangling result: reference", () => {
    const source = clone(inventedSparse);
    source.diagnostics[0].affected_refs = [...(source.diagnostics[0].affected_refs ?? []), "result:stress:pipe-P-100"];
    invalid(source, "dangling result reference result:stress:pipe-P-100");
  });
  it("refuses a headline that does not govern, and one while coverage is incomplete", () => {
    const wrong = clone(inventedSparse);
    const lower = rowsOf(wrong, "pipe_elastic_normal_stress_maximum_v2").find(r => r.id !== wrong.summary.max_open_formula_stress!.result_ref)!;
    wrong.summary.max_open_formula_stress = { value: lower.value, unit: "Pa", location_ref: lower.entity_ref, result_ref: lower.id };
    invalid(wrong, "stress headline does not govern");
    const incomplete = clone(inventedSparse);
    const c = evidence(incomplete).preview_cases[0];
    const x = c.pipe_stress_extrema.pop();
    incomplete.results = incomplete.results.filter(r => r.id !== x.result_id);
    c.stress_maximum_coverage = { complete: false, unavailable_pipe_ids: [x.pipe_id], outside_domain_pipe_ids: [] };
    invalid(incomplete, "stress headline while coverage is incomplete or no maximum exists");
  });
  it("refuses a first-case-only displacement headline when a later case governs by value or tie", () => {
    const source = clone(inventedSparse);
    const rows = rowsOf(source, "displacement_magnitude");
    const other = rows.find(r => r.id !== source.summary.max_displacement!.result_ref)!;
    source.summary.max_displacement = { value: other.value, unit: "mm", location_ref: other.entity_ref, result_ref: other.id };
    invalid(source, "displacement headline does not govern");
  });
  it("refuses a maximum outside or off its enclosure", () => {
    const source = clone(inventedSparse);
    const x = evidence(source).preview_cases[0].pipe_stress_extrema[0];
    x.value_upper_pa = x.value_lower_pa;
    const row = source.results.find(r => r.id === x.result_id)!;
    if (row.value === x.value_lower_pa) row.value = x.value_lower_pa * 1.5 + 1;
    invalid(source);
  });
  it("refuses inconsistent support magnitudes", () => {
    const source = clone(inventedSparse);
    const row = rowsOf(source, "support_reaction_force_magnitude_v2").find(r => r.value > 0)!;
    row.value *= 1 + 1e-9;
    invalid(source, "support force magnitude inconsistent with components");
  });
  it("refuses a missing withheld record and a zero-filled withheld support", () => {
    const source = clone(inventedSparse);
    const c = evidence(source).preview_cases.find((c: any) => c.support_attribution.withheld.length);
    expect(c).toBeTruthy();
    const withheld = c.support_attribution.withheld.shift();
    invalid(source);
    const zero = clone(inventedSparse);
    const zc = evidence(zero).preview_cases.find((c: any) => c.support_attribution.withheld.length);
    const support = zc.support_attribution.withheld[0].support_id;
    const template = rowsOf(zero, "support_reaction_component_v2").find(r => r.basis_ref?.ref_id === zc.load_case_id)!;
    zero.results.push({ ...structuredClone(template), id: `result:support-action:${lengthPrefixedSegments(zc.load_case_id, support)}:Fx`, entity_ref: support, value: 0 });
    invalid(zero, "support action rows do not match attribution");
    expect(withheld.support_id).toBeTruthy();
  });
  it("refuses a combination row for a gated combination and an intensified row in a combination", () => {
    const source = clone(inventedSparse);
    const gate = evidence(source).combination_gates[0];
    expect(gate.withheld).toBe(true);
    const template = rowsOf(source, "global_nodal_displacement_x")[0];
    source.results.push({ ...structuredClone(template), id: "result:combination:tamper", basis_ref: { ref_type: "combination", ref_id: gate.combination_id }, source_result_refs: [template.id] });
    invalid(source, "combination row without an admitting gate");
    const intensified = clone(inventedSparse);
    evidence(intensified).combination_gates[0] = { combination_id: gate.combination_id, withheld: false, reason: null };
    const irow = rowsOf(intensified, "component_equal_factor_intensified_bending_stress_v1")[0];
    intensified.results.push({ ...structuredClone(irow), id: "result:combination:tamper-i", basis_ref: { ref_type: "combination", ref_id: gate.combination_id } });
    invalid(intensified, "maximum or intensified row for a combination");
  });
  it("refuses an intensified value that includes k or an axial term", () => {
    const source = clone(inventedSparse);
    const row = rowsOf(source, "component_equal_factor_intensified_bending_stress_v1").find(r => r.value > 0)!;
    row.value *= 1.2;
    invalid(source, "intensified value inconsistent with its inputs");
  });
  it("refuses an arc maximum without an enclosure and non-table limitations", () => {
    const source = clone(inventedSparse);
    const template = rowsOf(source, "pipe_elastic_normal_stress_maximum_v2")[0];
    source.results.push({ ...structuredClone(template), id: "result:elastic-maximum:tamper" });
    invalid(source);
    const limits = clone(inventedSparse);
    limits.formulation_basis!.limitations = [...limits.formulation_basis!.limitations.slice(1)];
    invalid(limits, "formulation profile or limitations");
  });
  it("refuses open or extra contract-evidence keys", () => {
    const source = clone(connectedSparse);
    (source.contract_evidence as any).extra = [];
    invalid(source, "contract evidence namespace");
  });
  it("A1 a: accepts a diagnostic without affected_refs", () => {
    const source = clone(inventedSparse);
    delete source.diagnostics.find(d => d.severity === "info")!.affected_refs;
    expect(() => validatePreviewPhysicsEvidence(source)).not.toThrow();
  });
  it("A1 b: refuses a withheld record without its diagnostic, and a diagnostic without its record", () => {
    const source = clone(inventedSparse);
    const index = source.diagnostics.findIndex(d => d.code === "SUPPORT_ACTION_ATTRIBUTION_WITHHELD" || d.code === "CONSTANT_EFFORT_NOT_CONSUMED");
    expect(index).toBeGreaterThanOrEqual(0);
    source.diagnostics.splice(index, 1);
    invalid(source, "withheld records do not match withheld diagnostics");
    const extra = clone(inventedSparse);
    const support = evidence(extra).preview_cases[0].support_attribution.attributed_support_ids[0];
    extra.diagnostics.push({ id: `diagnostic:preview-physics:constant-effort-not-consumed:${lengthPrefixed(support)}`, code: "CONSTANT_EFFORT_NOT_CONSUMED", severity: "info", message: "tamper", affected_refs: [support] });
    invalid(extra, "withheld records do not match withheld diagnostics");
    const unlisted = clone(inventedSparse);
    const nonlinear = rowsOf(unlisted, "nonlinear_support_final_displacement")[0];
    expect(nonlinear).toBeTruthy();
    nonlinear.entity_ref = "support:not-in-the-model";
    invalid(unlisted, "per-support nonlinear row names an unlisted support");
  });
  it("A1 e: accepts a sanitized blocked envelope and refuses rows or headlines on it", () => {
    const blocked = clone(connectedSparse);
    blocked.status.mechanics = "MODEL_INCOMPLETE";
    blocked.results = [];
    blocked.contract_evidence = { preview_cases: [], combination_gates: [] };
    blocked.summary.max_open_formula_stress = null;
    blocked.summary.max_displacement = null;
    blocked.diagnostics = blocked.diagnostics.filter(d => !(d.affected_refs ?? []).some(ref => ref.startsWith("result:")));
    expect(() => validatePreviewPhysicsEvidence(blocked)).not.toThrow();
    const withRow = clone(blocked); withRow.results = [structuredClone(connectedSparse.results[0]) as MechanicsResult["results"][number]];
    invalid(withRow);
    const withHeadline = clone(blocked); withHeadline.summary.max_displacement = { value: 1, unit: "mm", location_ref: "node:x", result_ref: "node:x" };
    invalid(withHeadline); // refused (dangling summary ref, then the blocked-envelope rule)
    const nonResultHeadline = clone(blocked); nonResultHeadline.summary.max_displacement = { value: 1, unit: "mm", location_ref: "node:x", result_ref: null } as any;
    invalid(nonResultHeadline, "blocked envelope carries evidence, rows or headlines");
    const withRef = clone(blocked); withRef.diagnostics.push({ id: "diagnostic:tamper", code: "X", severity: "blocking", message: "x", affected_refs: ["result:disp:node-fixture-tip"] });
    invalid(withRef, "dangling result reference result:disp:node-fixture-tip");
  });
  it("A1 d: refuses a negative lower bound and a value off the enclosure midpoint", () => {
    const source = clone(connectedSparse);
    const x = evidence(source).preview_cases[0].pipe_stress_extrema[0];
    const row = source.results.find(r => r.id === x.result_id)!;
    row.value = x.value_upper_pa;
    if (x.value_upper_pa !== x.value_lower_pa) invalid(source, "maximum outside or off its enclosure");
    const negative = clone(connectedSparse);
    evidence(negative).preview_cases[0].pipe_stress_extrema[0].value_lower_pa = -1;
    invalid(negative, "extrema bounds");
  });
});

// Shared id vector across the three readers (Rust:
// shared_unicode_id_vector_is_admitted_and_byte_lengths_are_required; Python: same name).
describe("shared unicode id vector (actual producer output)", () => {
  it("admits the fixture and requires UTF-8 byte lengths, not character counts", () => {
    const source = clone(unicodeIdsSparse);
    expect(() => validatePreviewPhysicsEvidence(source)).not.toThrow();
    expect(numericalResultStanding(source, unicodeIdsModel as unknown as PreviewModel).findings).toEqual([]);
    const rowIds = new Set(source.results.map(r => r.id));
    for (const id of [
      "result:support-action:7:load:é:11:support:锚:Fx",
      "result:elastic-maximum:7:load:é:10:pipe:α-β",
      "result:elastic-maximum:9:load:𝔫:10:pipe:β-γ",
      "result:intensified-bending:component-ç:pipe-α-β:end-j",
      "result:loadcase:load-𝔫:intensified-bending:component-ç:pipe-β-γ:end-i",
    ]) expect(rowIds.has(id)).toBe(true);
    expect(source.diagnostics.some(d => d.id === "diagnostic:preview-physics:constant-effort-not-consumed:13:support:ü-ce")).toBe(true);
    const text = JSON.stringify(unicodeIdsSparse);
    for (const [from, to, detail] of [
      ["result:elastic-maximum:7:load:é:", "result:elastic-maximum:6:load:é:", "maximum row identity"],
      ["constant-effort-not-consumed:13:support:ü-ce", "constant-effort-not-consumed:12:support:ü-ce", "withheld diagnostic identity"],
    ]) {
      expect(text.includes(from)).toBe(true);
      invalid(JSON.parse(text.replaceAll(from, to)) as MechanicsResult, detail);
    }
  });
});

// A2 shared tamper vector: the same variants and expected outcomes as the Rust
// and Python suites, as RFC 6901 ops over actual producer bases.
describe("A2 shared tamper vector", () => {
  const root = resolve(__dirname, "../../../../../");
  const vector = JSON.parse(readFileSync(resolve(root, "fixtures/results/preview_physics_tamper_vector.json"), "utf8")) as {
    bases: Record<string, string>;
    variants: { id: string; base: string; ops: { op: "add" | "replace" | "remove" | "reverse"; path: string; value?: unknown }[]; expect: "accepted" | "refused" }[];
  };
  // TS detail per variant: each is refused by the A2 check it targets, not incidentally.
  const TS_REFUSAL_DETAIL: Record<string, string> = {
    "V1-station-fraction-above-1": "extrema fractions",
    "V2-subdivisions-above-cap": "extrema integers",
    "V3-negative-span-index": "extrema integers",
    "V4-preview-cases-reordered": "preview case coverage",
    "V5-combination-support-row-not-global": "support action frame", // A4 N6 (all support rows global) now fires before A2 4
    "V6-null-affected-refs": "diagnostic reference list",
    "V7-blocked-modifier-count-nonzero": "blocked envelope modifier count",
    "V8-empty-sif-source-reference": "intensified measure identity",
    "V9-missing-entity-ref": "source row fields",
    "V10-negative-intensified-value": "negative intensified value",
    "V11-support-both-attributed-and-withheld": "support both attributed and withheld",
    "A4-N2-basis-ref-extra-key": "row basis reference",
    "A4-N3-diagnostic-without-id": "evidence identities",
    "A4-N4-combination-source-result-refs-null": "combination source reference",
    "A4-N5-duplicate-combination-force-magnitude": "duplicate combined support component",
    "A4-N6-support-row-without-basis-ref-element-local": "support action frame",
    "A4-N8-blocked-envelope-without-summary": "summary",
  };
  const decode = (token: string) => token.replaceAll("~1", "/").replaceAll("~0", "~");
  function locate(document: any, pointer: string, forAdd = false): [any, string] {
    const tokens = pointer.split("/").slice(1).map(decode);
    const key = tokens.pop()!;
    let parent = document;
    for (const token of tokens) {
      if (parent === null || typeof parent !== "object" || !Object.hasOwn(parent, token)) throw new Error(`TAMPER_POINTER_UNRESOLVED: ${pointer}`);
      parent = parent[token];
    }
    if (parent === null || typeof parent !== "object") throw new Error(`TAMPER_POINTER_UNRESOLVED: ${pointer}`);
    // RFC 6901/6902 add: the target need not exist; "-" appends to an array.
    if (!forAdd && !Object.hasOwn(parent, key)) throw new Error(`TAMPER_POINTER_UNRESOLVED: ${pointer}`);
    return [parent, key];
  }
  function apply(document: any, op: { op: string; path: string; value?: unknown }) {
    const [parent, key] = locate(document, op.path, op.op === "add");
    if (op.op === "add") {
      if (Array.isArray(parent)) {
        const index = key === "-" ? parent.length : Number(key);
        if (!Number.isInteger(index) || index < 0 || index > parent.length) throw new Error(`TAMPER_ADD_INDEX_INVALID: ${op.path}`);
        parent.splice(index, 0, structuredClone(op.value));
      } else parent[key] = structuredClone(op.value);
    }
    else if (op.op === "replace") parent[key] = structuredClone(op.value);
    else if (op.op === "remove") { if (Array.isArray(parent)) parent.splice(Number(key), 1); else delete parent[key]; }
    else if (op.op === "reverse") { if (!Array.isArray(parent[key])) throw new Error("TAMPER_REVERSE_NOT_ARRAY"); parent[key].reverse(); }
    else throw new Error(`TAMPER_OP_UNSUPPORTED: ${op.op}`);
  }
  it("has all 22 entries: 2 unchanged controls, 3 A4 acceptances and 17 refusals", () => {
    expect(vector.variants).toHaveLength(22);
    expect(vector.variants.filter(v => v.ops.length === 0 && v.expect === "accepted")).toHaveLength(2);
    expect(vector.variants.filter(v => v.ops.length > 0 && v.expect === "accepted").map(v => v.id).sort()).toEqual(["A4-N1-intensified-metadata-extra-key", "A4-N10-integer-span-index-written-as-float", "A4-N7-integer-count-written-as-float"]);
    expect(vector.variants.filter(v => v.expect === "refused")).toHaveLength(17);
    expect(vector.variants.filter(v => v.expect === "refused").every(v => Object.hasOwn(TS_REFUSAL_DETAIL, v.id))).toBe(true);
  });
  it.each(vector.variants.map(v => [v.id, v] as const))("%s", (_id, variant) => {
    const source = JSON.parse(readFileSync(resolve(root, vector.bases[variant.base]), "utf8"));
    for (const op of variant.ops) apply(source, op);
    if (variant.expect === "accepted") expect(() => validatePreviewPhysicsEvidence(source)).not.toThrow();
    else invalid(source, TS_REFUSAL_DETAIL[variant.id]);
  });
});

// B1's reader follow-up toward I4' (I101; RR "I4 made at `30f3d1b24a`; …", rulings 2 and 3): PY's extrema-number
// demand. readCases runs it for the raw reader and for the transport check, so both refuse a non-number
// global_upper_bound_pa or certified_gap_pa (null included) with "extrema numbers", at PY's place.
describe("rulings 2 and 3: each extremum's global_upper_bound_pa and certified_gap_pa is a finite number", () => {
  const message = (run: () => void): string | null => { try { run(); return null; } catch (error) { return (error as Error).message; } };
  const both = (source: MechanicsResult) => [message(() => validatePreviewPhysicsEvidence(source)), message(() => validatePreviewPhysicsTransportMetadata(source))];
  const refused = (detail: string) => [`SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: ${detail}`, `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: ${detail}`];
  const edited = (fixture: unknown, edit: (x: any) => void, at = [0, 0]) => {
    const source = clone(fixture);
    edit(evidence(source).preview_cases[at[0]].pipe_stress_extrema[at[1]]);
    return source;
  };
  const shapes: [string, string, unknown][] = [
    ["global_upper_bound_pa a string", "global_upper_bound_pa", "x"],
    ["global_upper_bound_pa null", "global_upper_bound_pa", null],
    ["global_upper_bound_pa a boolean", "global_upper_bound_pa", true],
    ["certified_gap_pa null", "certified_gap_pa", null],
    ["certified_gap_pa a string", "certified_gap_pa", "0"],
    ["certified_gap_pa a list", "certified_gap_pa", [0]],
  ];
  it.each(shapes)("%s: refused by the raw reader and the transport check", (_name, member, value) => {
    for (const fixture of [connectedSparse, inventedSparse]) expect(both(edited(fixture, x => { x[member] = value; }))).toEqual(refused("extrema numbers"));
    // A later case's later extremum is read the same way.
    expect(both(edited(inventedSparse, x => { x[member] = value; }, [1, 3]))).toEqual(refused("extrema numbers"));
  });
  it("admits integers; the demand precedes the fraction, integer and bound checks", () => {
    expect(both(edited(connectedSparse, x => { x.global_upper_bound_pa = 41354909; x.certified_gap_pa = 0; }))).toEqual([null, null]);
    expect(both(edited(connectedSparse, x => { x.global_upper_bound_pa = "x"; x.station_fraction = 2; }))).toEqual(refused("extrema numbers"));
    expect(both(edited(connectedSparse, x => { x.certified_gap_pa = null; x.span_index = -1; }))).toEqual(refused("extrema numbers"));
    expect(both(edited(connectedSparse, x => { x.station_fraction = 2; }))).toEqual(refused("extrema fractions"));
  });
});

/** B2-C REVISION_02 §4.5 (NC-3), with the binding Design text's correction: G7's guard admits p, a combination's
 * displacement magnitude formed as RN64 (ties to even) of the exact 3-norm of its components, and refuses p * (1 + 2^-40)
 * for p >= MIN_POSITIVE, or p + 256 * 2^-1074 below it (+0 included). The vectors are I97's `exact_norm_vectors.json`
 * (frozen NUM `e85d383b64`, `I97/b2_c_01/_run_records/r2/`): its curated cases with a finite result and its RV118 triples
 * whose result is subnormal, as [label, x, y, z, p] binary64 words. Run on the JavaScript engine that CI uses (Node's V8). */
describe("NC-3: consistentNorm on the adversarial and subnormal exact-norm vectors", () => {
  const VECTORS: [string, string, string, string, string][] = [
    ["A-5 midpoint, ties to even", "3de0000002000000", "3f80000004000000", "0000000000000000", "3f80000004000000"],
    ["A-5 midpoint plus 2^-1074: rounds up", "3de0000002000000", "3f80000004000000", "0000000000000001", "3f80000004000001"],
    ["A-5 midpoint plus 2^-600: rounds up", "3de0000002000000", "3f80000004000000", "1a70000000000000", "3f80000004000001"],
    ["A-5 midpoint, first component one ulp lower: rounds down", "3de0000001ffffff", "3f80000004000000", "0000000000000000", "3f80000004000000"],
    ["A-5 midpoint, signs flipped", "bde0000002000000", "bf80000004000000", "8000000000000000", "3f80000004000000"],
    ["3,4,12 -> 13 exactly", "c008000000000000", "4010000000000000", "4028000000000000", "402a000000000000"],
    ["1, 2^-1074, 0 -> 1", "3ff0000000000000", "0000000000000001", "0000000000000000", "3ff0000000000000"],
    ["1e300, 1e-300, 2^-1074", "7e37e43c8800759c", "01a56e1fc2f8f359", "0000000000000001", "7e37e43c8800759c"],
    ["smallest subnormal", "0000000000000001", "0000000000000000", "0000000000000000", "0000000000000001"],
    ["three smallest subnormals", "0000000000000001", "8000000000000001", "0000000000000001", "0000000000000002"],
    ["two smallest subnormals", "0000000000000001", "0000000000000001", "0000000000000000", "0000000000000001"],
    ["largest subnormal, three times", "000fffffffffffff", "000fffffffffffff", "000fffffffffffff", "001bb67ae8584ca9"],
    ["MIN_POSITIVE and a subnormal", "0010000000000000", "0000000000000001", "8000000000000001", "0010000000000000"],
    ["MAX, 0, 0 -> MAX", "7fefffffffffffff", "0000000000000000", "8000000000000000", "7fefffffffffffff"],
    ["2^1023, 2^1023, 0 -> finite", "7fe0000000000000", "7fe0000000000000", "0000000000000000", "7fe6a09e667f3bcd"],
    ["1e308 three times -> finite", "7fe1ccf385ebc8a0", "7fe1ccf385ebc8a0", "7fe1ccf385ebc8a0", "7feed4df0150215a"],
    ["1e200 (an unscaled hypot would overflow)", "6974e718d7d7625a", "e974e718d7d7625a", "6974e718d7d7625a", "69821a2f9c3d2c96"],
    ["1e-200 (an unscaled hypot would underflow)", "16687e92154ef7ac", "16687e92154ef7ac", "96687e92154ef7ac", "167536793539fd32"],
    ["zeros +0,+0,+0", "0000000000000000", "0000000000000000", "0000000000000000", "0000000000000000"],
    ["zeros +0,+0,-0", "0000000000000000", "0000000000000000", "8000000000000000", "0000000000000000"],
    ["zeros +0,-0,+0", "0000000000000000", "8000000000000000", "0000000000000000", "0000000000000000"],
    ["zeros +0,-0,-0", "0000000000000000", "8000000000000000", "8000000000000000", "0000000000000000"],
    ["zeros -0,+0,+0", "8000000000000000", "0000000000000000", "0000000000000000", "0000000000000000"],
    ["zeros -0,+0,-0", "8000000000000000", "0000000000000000", "8000000000000000", "0000000000000000"],
    ["zeros -0,-0,+0", "8000000000000000", "8000000000000000", "0000000000000000", "0000000000000000"],
    ["zeros -0,-0,-0", "8000000000000000", "8000000000000000", "8000000000000000", "0000000000000000"],
    ["RV118 triple 28", "8000000000056ef8", "8000000000056f17", "0000000000056ef8", "000000000009694c"],
    ["RV118 triple 35", "80000001240332be", "800000012422ffc9", "00000001240332be", "00000001f9da2a11"],
    ["RV118 triple 37", "0000000000000003", "0000000000000000", "0000000000000000", "0000000000000003"],
    ["RV118 triple 38", "00000171f4a4f7ea", "000001722c0f0e6e", "80000171f4a4f7ea", "00000280e82f0cf2"],
    ["RV118 triple 74", "800000000022403a", "0000000000000000", "0000000000000000", "000000000022403a"],
    ["RV118 triple 79", "8000000001fd3f07", "8000000001fd5372", "0000000001fd3f07", "000000000372161b"],
    ["RV118 triple 81", "8000000000b4cd3c", "8000000000000000", "8000000000000000", "0000000000b4cd3c"],
    ["RV118 triple 103", "800102f7655fac9b", "800000000000817c", "800000000000817c", "000102f7655fac9b"],
    ["RV118 triple 120", "800000001dc02eb6", "800000001dc6a936", "000000001dc02eb6", "00000000338b5b3e"],
    ["RV118 triple 131", "80000000b49c4932", "0000000000000000", "80000148ffdcf8ea", "00000149000e8c0b"],
    ["RV118 triple 132", "80000000002b88a6", "80000000002b9344", "00000000002b88a6", "00000000004b6d3b"],
    ["RV118 triple 146", "80000000000000ca", "8000000000000000", "8000000000000000", "00000000000000ca"],
    ["RV118 triple 174", "80000183e84d7803", "8000000000000000", "8000000000000000", "00000183e84d7803"],
    ["RV118 triple 200", "000000000015d1ee", "000000000015d6dd", "800000000015d1ee", "000000000025cdf7"],
    ["RV118 triple 209", "0000016a138d50f7", "0000016a68f160b3", "8000016a138d50f7", "0000027353cb0884"],
    ["RV118 triple 221", "0000008e5765893b", "000000000000008e", "0000000000000000", "0000008e5765893b"],
    ["RV118 triple 225", "0000000000003cef", "0000000000000000", "0000000000000000", "0000000000003cef"],
    ["RV118 triple 230", "000000000022fdaf", "0000000000000000", "0000000000000000", "000000000022fdaf"],
    ["RV118 triple 234", "00000000023d100d", "00000000023d3cfb", "80000000023d100d", "0000000003e0acd0"],
    ["RV118 triple 235", "0000000000007287", "000000000000728a", "8000000000007287", "000000000000c660"],
    ["RV118 triple 238", "80000021134953f0", "80000021158f9762", "00000021134953f0", "000000394b156654"],
    ["RV118 triple 239", "8003c41c92b94122", "8003c484fef22c2a", "0003c41c92b94122", "0006862036b01474"],
    ["RV118 triple 246", "00000000000002c8", "00000000000002c8", "80000000000002c8", "00000000000004d1"],
  ];
  const word = (bits: string) => { const view = new DataView(new ArrayBuffer(8)); view.setBigUint64(0, BigInt(`0x${bits}`)); return view.getFloat64(0); };
  const MIN_POSITIVE = 2 ** -1022, TINY_STEP = 256 * 2 ** -1074;
  it("accepts each vector's RN64 magnitude and refuses the perturbed one", () => {
    expect(VECTORS).toHaveLength(49);
    const labels = VECTORS.map(v => v[0]);
    for (const required of ["A-5 midpoint, ties to even", "A-5 midpoint plus 2^-1074: rounds up", "smallest subnormal", "largest subnormal, three times", "1e200 (an unscaled hypot would overflow)", "1e-200 (an unscaled hypot would underflow)", "1e308 three times -> finite", "zeros -0,-0,-0"]) expect(labels).toContain(required);
    const misses: string[] = [], unrepresentable: string[] = [];
    for (const [label, x, y, z, p] of VECTORS) {
      const components = [x, y, z].map(word), magnitude = word(p);
      const perturbed = magnitude >= MIN_POSITIVE ? magnitude * (1 + 2 ** -40) : magnitude + TINY_STEP;
      if (!consistentNorm(magnitude, components)) misses.push(`${label}: p refused`);
      // MAX * (1 + 2^-40) is not a finite binary64, so no published row can carry it.
      if (!Number.isFinite(perturbed)) unrepresentable.push(label);
      else if (consistentNorm(perturbed, components)) misses.push(`${label}: the perturbed magnitude admitted`);
    }
    expect(misses).toEqual([]);
    expect(unrepresentable).toEqual(["MAX, 0, 0 -> MAX"]);
  });
});
