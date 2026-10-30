import { describe, expect, it } from "vitest";
import type { MechanicsResult } from "../types";
import { ANALYSIS_RUN_SCHEMA_VERSION_RETIRED, buildAnalysisRunV03, verifyAnalysisRunRecord, validateAnalysisRunV03, analysisRecordProjection, modelLoadBasisRefs, sourceBasisReference, validateRetainedRecoverySource } from "./analysisRunCompatibility";
import combinationSuccessor from "../../../../fixtures/results/retained_precision_combination_successor_sparse_interactive.json";
import { checkedJsonText, canonicalSha256HexCheckedV1 } from "./hashService";
import { PREVIEW_PHYSICS_CONTRACT_ID } from "../features/results/numericalResultQuality";
import { PREVIEW_PHYSICS_CONTRACT_SHA256 } from "../test-support/contractBindings";
import { minimalPreviewStatement, syncPreviewCases } from "../test-support/retiredPrecision";
import { sourceContract } from "../features/results/resultReader";
import pressureSource from "../../../../fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json";
import { retiredMechanicsResult } from "../test-support/retiredMechanicsResult";

const result: MechanicsResult = {
  schema_version: "0.2.0", document_kind: "MechanicsResult", run_id: "run:test", model_ref: "model:test",
  status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE", professional_acceptance: "NOT_PROVIDED" }, diagnostics: [], summary: {},
  results: [{ id: "result:test", entity_ref: "node:test", kind: "displacement_magnitude", value: 1, unit: "mm" }],
};
const manifest = { manifest_ref: { object_type: "InputManifest" as const, ref: "input-manifest:test" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: "model:test" }, solver_basis: { solver_name: "solver", solver_version: "0.1.0", solver_build_ref: "build:test" } } };

describe("analysis record versions", () => {
  it.each(["0.1.0", "0.2.0"])("refuses a saved %s record by name: re-run the analysis", async version => {
    const record = { schema_version: version, analysis_run: { run_id: "run:test", hashes: [] } } as never;
    await expect(verifyAnalysisRunRecord(record)).rejects.toThrow(`${ANALYSIS_RUN_SCHEMA_VERSION_RETIRED}: analysis-run ${version} is no longer read; re-run the analysis`);
  });

  it("refuses an unknown record version", async () => {
    await expect(verifyAnalysisRunRecord({ schema_version: "0.9.0", analysis_run: { run_id: "run:test", hashes: [] } } as never)).rejects.toThrow("ANALYSIS-RUN-SCHEMA-VERSION-UNSUPPORTED: 0.9.0");
  });

  it("never records a historical result, in any version", async () => {
    const historical = retiredMechanicsResult();
    const before = JSON.stringify(historical);
    await expect(buildAnalysisRunV03(historical, { ...manifest, manifest: { ...manifest.manifest, model_basis: { model_ref: historical.model_ref } } })).rejects.toThrow("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
    expect(JSON.stringify(historical)).toBe(before);
  });

  it("rejects unsupported JavaScript shapes before transport", () => {
    expect(() => checkedJsonText([, 1])).toThrow("SPARSE-ARRAY");
    expect(() => checkedJsonText({ bad: "\ud800" })).toThrow("LONE-SURROGATE");
    expect(() => checkedJsonText({ [Symbol("bad")]: 1 })).toThrow("SYMBOL-KEY");
    const accessor = {}; Object.defineProperty(accessor, "x", { enumerable: true, get: () => 1 });
    expect(() => checkedJsonText(accessor)).toThrow("UNSUPPORTED-PROPERTY");
    const custom = {}; Object.defineProperty(custom, "toJSON", { value: () => ({}) });
    expect(() => checkedJsonText(custom)).toThrow("UNSUPPORTED-PROPERTY");
    const mutable = { x: 1 }; let reads = 0;
    const proxy = new Proxy(mutable, { getOwnPropertyDescriptor(target, key) { const descriptor = Reflect.getOwnPropertyDescriptor(target, key)!; if (key === "x") descriptor.value = ++reads; return descriptor; }, get() { throw new Error("validated source was traversed twice"); } });
    expect(checkedJsonText(proxy)).toBe('{"x":1}');
    expect(reads).toBe(1);
    const topLevelProto = JSON.parse('{"__proto__":{"polluted":true},"x":1}');
    const nestedProto = JSON.parse('{"outer":{"__proto__":{"polluted":true},"x":1}}');
    expect(JSON.parse(checkedJsonText(topLevelProto))).toEqual(topLevelProto);
    expect(JSON.parse(checkedJsonText(nestedProto))).toEqual(nestedProto);
    expect(({} as { polluted?: boolean }).polluted).toBeUndefined();
  });
});

// B1 retired precision-1, the header these record tests used over a synthetic row;
// they use the minimal preview-physics-2 transport statement (a signed rotation row, so no headline).
function syntheticPreview() {
  return minimalPreviewStatement({ run_id: "run:test", model_ref: "model:test", cases: [],
    rows: [{ id: "result:test", entity_ref: "node:test", kind: "global_nodal_rotation_x", value: 1, unit: "rad",
      metadata: { component: "nodal_rotation_x", coordinate_system: "global", location: "node", basis: "solved_from_global_linear_system", sign_convention: "positive value follows the right-hand-rule rotation about the global cartesian X axis" } }] });
}
it("binds the exact preview semantic contract and rejects unknown raw headers", async () => {
  const unknown = structuredClone(result); unknown.schema_version = "0.2.0";
  await expect(buildAnalysisRunV03(unknown, manifest)).rejects.toThrow("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
  const raw = syntheticPreview();
  raw.results[0].value = 1.0000000000000002;
  const previewManifest = structuredClone(manifest);
  previewManifest.manifest.solver_basis.solver_name = raw.producer!.component_name; previewManifest.manifest.solver_basis.solver_version = raw.producer!.component_version;
  const record = await buildAnalysisRunV03(raw, previewManifest);
  expect(record.schema_version).toBe("0.3.0");
  expect(record.analysis_run.reproducibility.semantic_contract).toEqual({ id: PREVIEW_PHYSICS_CONTRACT_ID, sha256: PREVIEW_PHYSICS_CONTRACT_SHA256 });
  expect(await verifyAnalysisRunRecord(record)).toBe("match");
  for (const modelState of [
    {object_type:"ModelState",ref:"state:other:preview"},
    {object_type:"Other",ref:`state:${raw.model_ref}:preview`},
  ]) {
    const bad = structuredClone(record); bad.analysis_run.model_state_ref = modelState;
    bad.analysis_run.hashes.find(h => h.payload_scope === "analysis_run_record")!.value = await canonicalSha256HexCheckedV1(analysisRecordProjection(bad));
    expect(await verifyAnalysisRunRecord(bad)).toBe("match");
    await expect(validateAnalysisRunV03(bad, raw)).rejects.toThrow("ANALYSIS_SOURCE_MODEL_STATE_MISMATCH");
  }
  for (const edit of [
    (r: typeof record) => { r.analysis_run.result_refs[0].semantic_contract!.id = "unknown"; },
    (r: typeof record) => { r.analysis_run.result_refs[0].source_annotation!.kind = "unknown"; },
    (r: typeof record) => { r.analysis_run.result_refs[0].interpretation!.status = "unavailable"; },
    (r: typeof record) => { r.analysis_run.reproducibility.semantic_contract!.sha256 = "0".repeat(64); },
  ]) {
    const bad = structuredClone(record); edit(bad);
    bad.analysis_run.hashes.find(h => h.payload_scope === "analysis_run_record")!.value = await canonicalSha256HexCheckedV1(analysisRecordProjection(bad));
    expect(await verifyAnalysisRunRecord(bad)).toBe("match");
    await expect(validateAnalysisRunV03(bad, raw)).rejects.toThrow("ANALYSIS_");
  }
});

function syntheticRecordSource() {
  const raw = syntheticPreview();
  raw.results[0].basis_ref = { ref_type: "load_case", ref_id: "load:test" };
  const input = structuredClone(manifest);
  input.manifest.solver_basis.solver_name = raw.producer!.component_name;
  input.manifest.solver_basis.solver_version = raw.producer!.component_version;
  return { raw, input };
}
it("rejects rehashed source diagnostics, mechanics and basis substitutions while preserving rule-only revisions", async () => {
  const { raw, input } = syntheticRecordSource(); const before = structuredClone(raw);
  const record = await buildAnalysisRunV03(raw, input);
  for (const mutate of [
    (r: typeof record) => { r.analysis_run.diagnostics = [{ source_annotation: { code: "fabricated" } }] as never; },
    (r: typeof record) => { r.analysis_run.analysis_status[1] = "MODEL_INCOMPLETE"; },
    (r: typeof record) => { r.analysis_run.analysis_status.push("USER_RULE_FAILED"); },
    (r: typeof record) => { r.analysis_run.load_basis_refs = [{ object_type: "LoadCase", ref: "other" }]; },
  ]) {
    const bad = structuredClone(record); mutate(bad);
    bad.analysis_run.hashes.find(h => h.payload_scope === "analysis_run_record")!.value = await canonicalSha256HexCheckedV1(analysisRecordProjection(bad));
    expect(await verifyAnalysisRunRecord(bad)).toBe("match");
    await expect(validateAnalysisRunV03(bad, raw)).rejects.toThrow("ANALYSIS_SOURCE_");
  }
  const revised = await buildAnalysisRunV03(raw, input, "USER_RULE_CHECKED");
  await expect(validateAnalysisRunV03(revised, raw)).resolves.toBeUndefined();
  expect(raw).toEqual(before);
  for (const invalid of ["", false, 0, "unknown"]) await expect(buildAnalysisRunV03(raw, input, invalid as never)).rejects.toThrow("ANALYSIS_RULE_STATUS_INVALID");
  delete (raw.status as any).rule_check;
  expect((await buildAnalysisRunV03(raw, input)).analysis_run.analysis_status).toContain("RULE_INPUTS_INCOMPLETE");
});
it("distinguishes independent explicit basis from source available scope, including empty and malformed bases", async () => {
  const { raw, input } = syntheticRecordSource();
  const refs = [{ object_type: "LoadCase", ref: "load:test" }, { object_type: "Combination", ref: "comb:requested" }];
  const broad = await buildAnalysisRunV03(raw, input, undefined, refs);
  await expect(validateAnalysisRunV03(broad, raw, refs)).resolves.toBeUndefined();
  await expect(validateAnalysisRunV03(broad, raw)).rejects.toThrow("LOAD_BASIS_MISMATCH");
  await expect(buildAnalysisRunV03(raw, input, undefined, [])).rejects.toThrow("SOURCE_SCOPE_MISMATCH");
  for (const basis of [{}, { ref_type: "", ref_id: "x" }, { ref_type: 3, ref_id: "x" }, { ref_type: "load_case", ref_id: "" }]) {
    // The preview reader refuses a malformed basis first; the record builder refuses the rest.
    const bad = structuredClone(raw); bad.results[0].basis_ref = basis as never;
    await expect(buildAnalysisRunV03(bad, input)).rejects.toThrow(/SOURCE_PREVIEW_PHYSICS_ROW_BASIS|SOURCE_REFERENCE_INVALID/);
  }
  raw.results[0].basis_ref = { ref_type: "generic", ref_id: "basis:generic" };
  expect((await buildAnalysisRunV03(raw, input)).analysis_run.load_basis_refs).toEqual([{ object_type: "ResultBasis", ref: "basis:generic" }]);
  for (const basis of [null, undefined]) {
    if (basis === undefined) delete raw.results[0].basis_ref;
    else raw.results[0].basis_ref = basis as never;
    expect((await buildAnalysisRunV03(raw, input, undefined, [])).analysis_run.load_basis_refs).toEqual([]);
  }
  raw.results[0].basis_ref = undefined;
  await expect(buildAnalysisRunV03(raw, input, undefined, [])).rejects.toThrow("CHECKED-JSON-UNSUPPORTED-TYPE: undefined");
});
it("accepts the shared generic numerical case basis shape", () => {
  const { raw } = syntheticRecordSource();
  raw.numerical_quality!.cases = [{ basis_ref: { ref_type: "generic", ref_id: "basis:generic" }, structural_status: "numerically_unresolved", solve_quality: "unresolved", model_matrix_fidelity: "not_assessed", accuracy_evidence: "unresolved", evidence_refs: [] }];
  syncPreviewCases(raw);
  expect(sourceContract(raw)).toBe("preview_physics");
});

it("validates source identities and requires explicit basis to cover emitted numerical cases", async () => {
  const { raw, input } = syntheticRecordSource();
  for (const key of ["run_id", "model_ref", "row_id"]) for (const value of ["", 0, null]) {
    const bad = structuredClone(raw);
    if (key === "row_id") bad.results[0].id = value as never; else (bad as any)[key] = value;
    // The preview reader refuses a non-string identity first; the record builder an empty one.
    await expect(buildAnalysisRunV03(bad, input)).rejects.toThrow(/SOURCE_PREVIEW_PHYSICS_STRING_INVALID|ANALYSIS_SOURCE_REFERENCE_INVALID/);
  }
  raw.numerical_quality!.cases = [{ basis_ref: { ref_type: "load_case", ref_id: "load:without-row" }, structural_status: "numerically_unresolved", solve_quality: "unresolved", model_matrix_fidelity: "not_assessed", accuracy_evidence: "unresolved", evidence_refs: [] }];
  syncPreviewCases(raw);
  const available = await buildAnalysisRunV03(raw, input);
  expect(available.analysis_run.load_basis_refs).toEqual([{ object_type: "LoadCase", ref: "load:test" }]);
  await expect(buildAnalysisRunV03(raw, input, undefined, available.analysis_run.load_basis_refs)).rejects.toThrow("ANALYSIS_LOAD_BASIS_SOURCE_SCOPE_MISMATCH");
  const refs = [...available.analysis_run.load_basis_refs, { object_type: "LoadCase", ref: "load:without-row" }];
  await expect(buildAnalysisRunV03(raw, input, undefined, refs)).resolves.toHaveProperty("schema_version", "0.3.0");
});

it("rejects invalid source rule statuses independently of a valid record override", async () => {
  const { raw, input } = syntheticRecordSource();
  const record = await buildAnalysisRunV03(raw, input, "USER_RULE_CHECKED");
  for (const rule of ["", false, 0, "unknown"]) {
    const invalid = structuredClone(raw); invalid.status.rule_check = rule as never;
    await expect(buildAnalysisRunV03(invalid, input, "USER_RULE_CHECKED")).rejects.toThrow("ANALYSIS_SOURCE_RULE_STATUS_INVALID");
    await expect(validateAnalysisRunV03(record, invalid)).rejects.toThrow("ANALYSIS_SOURCE_RULE_STATUS_INVALID");
  }
  for (const rule of [null, undefined]) {
    const absent = structuredClone(raw);
    if (rule === undefined) delete (absent.status as any).rule_check;
    else absent.status.rule_check = rule as never;
    const revised = await buildAnalysisRunV03(absent, input, "USER_RULE_CHECKED");
    await expect(validateAnalysisRunV03(revised, absent)).resolves.toBeUndefined();
    expect((await buildAnalysisRunV03(absent, input)).analysis_run.analysis_status).toContain("RULE_INPUTS_INCOMPLETE");
  }
});

/** T6S, tests only (B2-C §10.4, N-8): B2-P's committed W-CB3 combination successor. Its combination rows' basis is the
 * combination entry's (`Combination`), the record's load basis covers the model's combination beside its cases, and an
 * explicit basis without the combination is refused, as for an uncovered case. */
it("binds a combination successor's Combination basis references (T6S, B2-C §10.4)", async () => {
  const doc = structuredClone(combinationSuccessor) as any, source = doc.source as MechanicsResult, model = doc.invocation.request.model;
  const own = { ...manifest, manifest: { model_basis: { model_ref: source.model_ref }, solver_basis: { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: "build:test" } } };
  const rows = source.results.filter(r => r.basis_ref?.ref_type === "combination");
  // W-7: the combination's 12 global end rows (111 + 12).
  expect(rows.length).toBe(111 + 12);
  expect(new Set(rows.map(r => JSON.stringify(sourceBasisReference(r.basis_ref))))).toEqual(new Set([JSON.stringify({ object_type: "Combination", ref: "combination:ab" })]));
  for (const bad of [{ ref_type: "combination", ref_id: "" }, { ref_type: "combination" }, { ref_type: "combination", ref_id: "combination:ab", extra: 1 }]) expect(() => sourceBasisReference(bad)).toThrow("ANALYSIS_SOURCE_REFERENCE_INVALID");
  const refs = modelLoadBasisRefs(model);
  expect(refs).toEqual([{ object_type: "LoadCase", ref: "case:a" }, { object_type: "LoadCase", ref: "case:b" }, { object_type: "Combination", ref: "combination:ab" }]);
  const record = await buildAnalysisRunV03(source, own, undefined, refs);
  expect(record.analysis_run.load_basis_refs).toEqual(refs);
  await expect(validateAnalysisRunV03(record, source, refs)).resolves.toBeUndefined();
  expect((await buildAnalysisRunV03(source, own)).analysis_run.load_basis_refs).toContainEqual({ object_type: "Combination", ref: "combination:ab" });
  await expect(buildAnalysisRunV03(source, own, undefined, refs.filter(r => r.object_type !== "Combination"))).rejects.toThrow("ANALYSIS_LOAD_BASIS_SOURCE_SCOPE_MISMATCH");
});

describe.each([
  // B7b: the v3 twin (pressure-source-1); Rust's source reader keeps its PHYSICS_SOURCE_ code family.
  ["pressure-source", pressureSource, "PHYSICS_SOURCE_RECEIPT_HASH"],
] as const)("%s portable source consistency", (_route, fixture, receiptError) => {
  it("takes Rust's commitment verdict while preserving the surface's refusal codes", async () => {
    const source = structuredClone(fixture) as unknown as MechanicsResult;
    await expect(validateRetainedRecoverySource(source)).resolves.toBeUndefined();
    const badReceipt = structuredClone(source) as any;
    badReceipt.source_block_recovery.receipt_sha256 = "0".repeat(64);
    await expect(validateRetainedRecoverySource(badReceipt)).rejects.toThrow(receiptError);
    const badPublication = structuredClone(source);
    badPublication.run_id = "edited-publication";
    await expect(validateRetainedRecoverySource(badPublication)).rejects.toThrow("SOURCE_RECOVERY_PUBLICATION_HASH");
    const badShape = structuredClone(source) as any;
    badShape.source_block_recovery.extra = 1;
    await expect(validateRetainedRecoverySource(badShape)).rejects.toThrow("SOURCE_RECOVERY_METADATA_CONTRADICTION");
  });
});
