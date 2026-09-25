import { describe, expect, it } from "vitest";
import type { MechanicsResult } from "../types";
import { buildAnalysisRunV02, buildAnalysisRunV03, verifyAnalysisRunRecord, validateAnalysisRunV03, analysisRecordProjection } from "./analysisRunCompatibility";
import { checkedJsonText, canonicalSha256HexCheckedV1 } from "./hashService";
import { sourceContract, PRECISION_CONTRACT_ID, PRECISION_CONTRACT_SHA256 } from "../features/results/numericalResultQuality";
import expectedRecord from "../../../../fixtures/analysis_runs/invented/analysis_run_v0_2.json";

const result: MechanicsResult = {
  schema_version: "0.2.0", document_kind: "MechanicsResult", run_id: "run:test", model_ref: "model:test",
  status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE", professional_acceptance: "NOT_PROVIDED" }, diagnostics: [], summary: {},
  results: [{ id: "result:test", entity_ref: "node:test", kind: "displacement_magnitude", value: 1, unit: "mm" }],
};
const manifest = { manifest_ref: { object_type: "InputManifest" as const, ref: "input-manifest:test" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: "model:test" }, solver_basis: { solver_name: "solver", solver_version: "0.1.0", solver_build_ref: "build:test" } } };

describe("analysis record 0.2", () => {
  it("preserves raw rows and binds immutable rule revisions", async () => {
    const before = structuredClone(result);
    const first = await buildAnalysisRunV02(result, manifest);
    const revised = await buildAnalysisRunV02(result, manifest, "USER_RULE_CHECKED");
    expect(result).toEqual(before);
    expect(first.schema_version).toBe("0.2.0");
    expect(first.analysis_run.result_refs).toHaveLength(1);
    expect(first.analysis_run.analysis_status).not.toContain("NOT_PROVIDED");
    expect(first.analysis_run.solver_version).toEqual({ solver_name: "solver", solver_version: "0.1.0", build_ref: { object_type: "ExternalReference", ref: "build:test" } });
    expect(first.analysis_run.hashes.find((row) => row.payload_scope === "received_result")?.value).toBe(revised.analysis_run.hashes.find((row) => row.payload_scope === "received_result")?.value);
    expect(first.analysis_run.hashes.find((row) => row.payload_scope === "analysis_run_record")?.value).not.toBe(revised.analysis_run.hashes.find((row) => row.payload_scope === "analysis_run_record")?.value);
    expect(await verifyAnalysisRunRecord(first)).toBe("match");
  });

  it("matches the language-neutral Python fixture byte for byte", async () => {
    const withBasis = structuredClone(result);
    withBasis.results[0].basis_ref = { ref_type: "load_case", ref_id: "load:test" };
    expect(await buildAnalysisRunV02(withBasis, manifest)).toEqual(expectedRecord);
    expect(await verifyAnalysisRunRecord(expectedRecord as never)).toBe("match");
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

it("binds the exact precision semantic contract and rejects unknown raw headers", async () => {
  const raw = structuredClone(result); raw.schema_version = "0.2.0";
  await expect(buildAnalysisRunV03(raw, manifest)).rejects.toThrow("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
  raw.producer = { component_name: "open_pipe_stress_product_physics", component_version: "0.2.0", semantic_contract_id: PRECISION_CONTRACT_ID };
  raw.numerical_quality = { value_representation: "finite_binary64", publication_quantization: "none", integrity_policy: "M03-INTEGRITY-v1", status: "not_assessed", cases: [] };
  raw.formulation_basis = { profile_id: "product_preview_mechanics_v1", limitations: ["bounded preview"] };
  raw.results[0].value = 1.0000000000000002;
  const precisionManifest = structuredClone(manifest);
  precisionManifest.manifest.solver_basis.solver_name = raw.producer.component_name; precisionManifest.manifest.solver_basis.solver_version = raw.producer.component_version;
  const record = await buildAnalysisRunV03(raw, precisionManifest);
  expect(record.schema_version).toBe("0.3.0");
  await expect(buildAnalysisRunV02(raw, manifest)).rejects.toThrow("HISTORICAL_ANALYSIS_SOURCE_UNSUPPORTED");
  expect(record.analysis_run.reproducibility.semantic_contract).toEqual({ id: PRECISION_CONTRACT_ID, sha256: PRECISION_CONTRACT_SHA256 });
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

it("historical construction rejects every present falsy precision metadata field", async () => {
  for (const version of ["0.1.0", "0.2.0"]) {
    for (const key of ["producer", "numerical_quality", "formulation_basis"]) {
      for (const value of [null, false, 0, ""]) {
        const contradictory = Object.assign(structuredClone(result), { schema_version: version, [key]: value });
        const before = JSON.stringify(contradictory);
        await expect(buildAnalysisRunV02(contradictory, manifest)).rejects.toThrow("HISTORICAL_ANALYSIS_SOURCE_UNSUPPORTED");
        expect(JSON.stringify(contradictory)).toBe(before);
      }
    }
  }
});

function syntheticPrecision() {
  const raw = structuredClone(result);
  raw.producer = { component_name: "open_pipe_stress_product_physics", component_version: "0.2.0", semantic_contract_id: PRECISION_CONTRACT_ID };
  raw.numerical_quality = { value_representation: "finite_binary64", publication_quantization: "none", integrity_policy: "M03-INTEGRITY-v1", status: "not_assessed", cases: [] };
  raw.formulation_basis = { profile_id: "product_preview_mechanics_v1", limitations: ["Synthetic consumer fixture only"] };
  raw.results[0].basis_ref = { ref_type: "load_case", ref_id: "load:test" };
  const input = structuredClone(manifest);
  input.manifest.solver_basis.solver_name = raw.producer.component_name;
  input.manifest.solver_basis.solver_version = raw.producer.component_version;
  return { raw, input };
}
it("rejects rehashed source diagnostics, mechanics and basis substitutions while preserving rule-only revisions", async () => {
  const { raw, input } = syntheticPrecision(); const before = structuredClone(raw);
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
  const { raw, input } = syntheticPrecision();
  const refs = [{ object_type: "LoadCase", ref: "load:test" }, { object_type: "Combination", ref: "comb:requested" }];
  const broad = await buildAnalysisRunV03(raw, input, undefined, refs);
  await expect(validateAnalysisRunV03(broad, raw, refs)).resolves.toBeUndefined();
  await expect(validateAnalysisRunV03(broad, raw)).rejects.toThrow("LOAD_BASIS_MISMATCH");
  await expect(buildAnalysisRunV03(raw, input, undefined, [])).rejects.toThrow("SOURCE_SCOPE_MISMATCH");
  for (const basis of [{}, { ref_type: "", ref_id: "x" }, { ref_type: 3, ref_id: "x" }, { ref_type: "load_case", ref_id: "" }]) {
    const bad = structuredClone(raw); bad.results[0].basis_ref = basis as never;
    await expect(buildAnalysisRunV03(bad, input)).rejects.toThrow("SOURCE_REFERENCE_INVALID");
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
  const { raw } = syntheticPrecision();
  raw.numerical_quality!.cases = [{ basis_ref: { ref_type: "generic", ref_id: "basis:generic" }, structural_status: "numerically_unresolved", solve_quality: "unresolved", model_matrix_fidelity: "not_assessed", accuracy_evidence: "unresolved", evidence_refs: [] }];
  expect(sourceContract(raw)).toBe("precision");
});

it("validates source identities and requires explicit basis to cover emitted numerical cases", async () => {
  const { raw, input } = syntheticPrecision();
  for (const key of ["run_id", "model_ref", "row_id"]) for (const value of ["", 0, null]) {
    const bad = structuredClone(raw);
    if (key === "row_id") bad.results[0].id = value as never; else (bad as any)[key] = value;
    await expect(buildAnalysisRunV03(bad, input)).rejects.toThrow("ANALYSIS_SOURCE_REFERENCE_INVALID");
  }
  raw.numerical_quality!.cases = [{ basis_ref: { ref_type: "load_case", ref_id: "load:without-row" }, structural_status: "numerically_unresolved", solve_quality: "unresolved", model_matrix_fidelity: "not_assessed", accuracy_evidence: "unresolved", evidence_refs: [] }];
  const available = await buildAnalysisRunV03(raw, input);
  expect(available.analysis_run.load_basis_refs).toEqual([{ object_type: "LoadCase", ref: "load:test" }]);
  await expect(buildAnalysisRunV03(raw, input, undefined, available.analysis_run.load_basis_refs)).rejects.toThrow("ANALYSIS_LOAD_BASIS_SOURCE_SCOPE_MISMATCH");
  const refs = [...available.analysis_run.load_basis_refs, { object_type: "LoadCase", ref: "load:without-row" }];
  await expect(buildAnalysisRunV03(raw, input, undefined, refs)).resolves.toHaveProperty("schema_version", "0.3.0");
});

it("rejects invalid source rule statuses independently of a valid record override", async () => {
  const { raw, input } = syntheticPrecision();
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
