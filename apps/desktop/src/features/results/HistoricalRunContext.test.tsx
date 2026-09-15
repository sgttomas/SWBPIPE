import { describe, expect, it } from "vitest";
import { buildHistoricalRunContext } from "./HistoricalRunContext";
import { loadPreviewModel, runPreviewMechanics, buildAnalysisRunPreview, bindSourceResultDimensions } from "../../services/previewService";
import { canonicalSha256HexCheckedV1, computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import type { LocalProjectEnvelope } from "../../types";
import legacyPythonRecord from "../../../../../fixtures/analysis_runs/invented/legacy_python_v0_1.json";

async function savedEnvelope(nativeRowsWithoutDimensions = false) {
  const model = await loadPreviewModel();
  const mechanics_result = structuredClone(await runPreviewMechanics(model));
  if (nativeRowsWithoutDimensions) {
    for (const row of mechanics_result.results) delete row.dimension;
  }
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: "synthetic", solver_version: "1", solver_build_ref: "synthetic@1", solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] });
  const analysis_run = await buildAnalysisRunPreview(mechanics_result, { inputManifest });
  const model_hash = await computeModelHash(model);
  const payload = { model, mechanics_result, analysis_run, model_hash, editor_intents: [], proposal: null, selected_review_target: null };
  const project_envelope_hash = await computeProjectEnvelopeHash(payload);
  return { ...payload, project_envelope_hash } as unknown as LocalProjectEnvelope;
}

async function legacyDigest(value: unknown) {
  const sort = (item: any): any => Array.isArray(item) ? item.map(sort) : item && typeof item === "object" ? Object.fromEntries(Object.entries(item).sort(([a], [b]) => a.localeCompare(b)).map(([key, child]) => [key, sort(child)])) : item;
  const digest = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(JSON.stringify(sort(value))));
  return [...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

async function legacySavedEnvelope() {
  const saved = await savedEnvelope(true);
  const received = saved.mechanics_result!;
  const enriched = bindSourceResultDimensions(received);
  const inputManifest = await buildCurrentSessionInputManifest({ model: saved.model, solver: { solver_name: "synthetic", solver_version: "1", solver_build_ref: "synthetic@1", solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] });
  const loadBasisRefs = Array.from(new Map(enriched.results.flatMap((row) => row.basis_ref ? [[`${row.basis_ref.ref_type}:${row.basis_ref.ref_id}`, { object_type: row.basis_ref.ref_type === "combination" ? "Combination" : "LoadCase", ref: row.basis_ref.ref_id }]] : [])).values());
  const resultRefs = await Promise.all(enriched.results.slice().sort((a, b) => a.id.localeCompare(b.id)).map(async (row) => ({ result_ref: { object_type: "Result", ref: row.id }, result_family: "legacy", source_dimension: row.dimension, hash_refs: [{ algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_ref: { object_type: "Result", ref: row.id }, payload_scope: "result_value", value: await legacyDigest(row) }], privacy_classification: "invented_public_example" })));
  const recordPayload = { run_id: received.run_id, model_ref: received.model_ref, status: received.status, load_basis_refs: loadBasisRefs, result_ids: enriched.results.map((row) => row.id).sort(), diagnostic_ids: enriched.diagnostics.map((item) => item.id ?? "diagnostic:unknown").sort(), input_manifest_ref: inputManifest.manifest_ref, input_manifest_sha256: inputManifest.manifest_sha256, result_dimensions: enriched.results.map((row) => ({ result_id: row.id, dimension: row.dimension })).sort((a, b) => a.result_id.localeCompare(b.result_id)) };
  saved.analysis_run = { schema_version: "0.1.0", deliverable_id: "DEL-14-02", package_id: "PKG-14", scope_item: "SOW-072", objectives: ["OBJ-016"], run_contract_status: {record_contract:"schema_first_analysis_run_records",model_state_binding:"schemas/model_state.schema.json",result_binding:"schemas/results.schema.yaml",physical_project_container:"TBD",external_validation_boundary:"reference_only_not_determined_by_software"}, analysis_run: { run_id: received.run_id, run_name: `${received.run_id} preview mechanics run`, run_kind: "mechanics_solve", model_state_ref: { object_type: "ModelState", ref: `state:${received.model_ref}:preview` }, load_basis_refs: loadBasisRefs, result_refs: resultRefs, hashes: [{ algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_ref: { object_type: "AnalysisRun", ref: received.run_id }, payload_scope: "analysis_run_record", value: await legacyDigest(recordPayload) }, { algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_ref: { object_type: "ResultEnvelope", ref: `result-envelope:${received.run_id}` }, payload_scope: "result_envelope", value: await legacyDigest(enriched) }], analysis_status: ["HUMAN_REVIEW_REQUIRED", received.status.mechanics, received.status.rule_check].sort(), reproducibility: { input_manifest_refs: [inputManifest.manifest_ref], input_manifest_hashes: [{ algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_ref: inputManifest.manifest_ref, payload_scope: "input_manifest", value: inputManifest.manifest_sha256 }],determinism_notes:["analysis run binds the exact current-session input-manifest ref and SHA-256","source result dimensions are explicit declarations, not inferred from unit text"],unresolved_tbd:["physical project container","release-grade solver build provenance"] }, immutability_policy: {run_record_is_read_only:true,mutation_policy:"changes_create_new_analysis_run",new_run_required_for_change:true,hash_invalidates_external_acceptance:true}, professional_boundary: { human_review_required: true, software_makes_compliance_claim: false, software_makes_certification_claim: false, software_makes_sealing_claim: false, software_makes_approval_claim: false, software_makes_authentication_claim: false } } } as any;
  saved.project_envelope_hash = await computeProjectEnvelopeHash({ model: saved.model, editor_intents: saved.editor_intents ?? [], proposal: saved.proposal ?? null, selected_review_target: saved.selected_review_target ?? null, mechanics_result: saved.mechanics_result, analysis_run: saved.analysis_run, model_hash: saved.model_hash });
  return saved;
}

describe("transient HistoricalRunContext integrity", () => {
  it("verifies exact known desktop 0.1 preimages without rewriting received evidence", async () => {
    const saved = await legacySavedEnvelope();
    const before = JSON.stringify(saved);
    const context = await buildHistoricalRunContext(saved);
    expect(context!.findings).not.toEqual(expect.arrayContaining(["HISTORICAL_RESULT_HASH_MISMATCH", "HISTORICAL_ANALYSIS_HASH_MISMATCH", "HISTORICAL_RESULT_HASH_UNVERIFIABLE_LEGACY_PREIMAGE", "HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE_LEGACY_PREIMAGE"]));
    expect(JSON.stringify(saved)).toBe(before);
  });

  it("reports a changed desktop 0.1 claim as mismatch and ambiguous preimage as unverifiable", async () => {
    const tampered = await legacySavedEnvelope();
    tampered.analysis_run!.analysis_run.hashes.find((hash) => hash.payload_scope === "result_envelope")!.value = "0".repeat(64);
    const tamperedBefore = JSON.stringify(tampered);
    expect((await buildHistoricalRunContext(tampered))!.findings).toContain("HISTORICAL_RESULT_HASH_UNVERIFIABLE_LEGACY_PREIMAGE");
    expect((await buildHistoricalRunContext(tampered))!.findings).not.toContain("HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE_LEGACY_PREIMAGE");
    expect(JSON.stringify(tampered)).toBe(tamperedBefore);
    const ambiguous = await legacySavedEnvelope();
    ambiguous.mechanics_result!.results[0].kind = "unknown_legacy_kind";
    const ambiguousBefore = JSON.stringify(ambiguous);
    const findings = (await buildHistoricalRunContext(ambiguous))!.findings;
    expect(findings).toEqual(expect.arrayContaining(["HISTORICAL_RESULT_HASH_UNVERIFIABLE_LEGACY_PREIMAGE", "HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE_LEGACY_PREIMAGE"]));
    expect(JSON.stringify(ambiguous)).toBe(ambiguousBefore);
  });
  it("keeps legacy desktop record and result checksum failures scoped", async () => {
    const recordTampered = await legacySavedEnvelope();
    recordTampered.analysis_run!.analysis_run.hashes.find((hash) => hash.payload_scope === "analysis_run_record")!.value = "0".repeat(64);
    const recordFindings = (await buildHistoricalRunContext(recordTampered))!.findings;
    expect(recordFindings).toContain("HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE_LEGACY_PREIMAGE");
    expect(recordFindings).not.toContain("HISTORICAL_RESULT_HASH_UNVERIFIABLE_LEGACY_PREIMAGE");
    const rowTampered = await legacySavedEnvelope();
    rowTampered.analysis_run!.analysis_run.result_refs[0].hash_refs[0].value = "0".repeat(64);
    const rowFindings = (await buildHistoricalRunContext(rowTampered))!.findings;
    expect(rowFindings).toContain("HISTORICAL_RESULT_HASH_UNVERIFIABLE_LEGACY_PREIMAGE");
    expect(rowFindings).not.toContain("HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE_LEGACY_PREIMAGE");
  });
  it("does not misclassify a full Python 0.1 record as the abbreviated desktop profile", async () => {
    const saved = await savedEnvelope();
    saved.analysis_run = structuredClone(legacyPythonRecord) as any;
    saved.mechanics_result = { schema_version: "0.2.0", document_kind: "MechanicsResult", run_id: "run:legacy-python", model_ref: "model:legacy-python", status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE" }, summary: {}, results: [{ id: "result:legacy-python", entity_ref: "node:legacy", kind: "displacement_magnitude", value: 1, unit: "mm", dimension: "length" }], diagnostics: [] } as any;
    const before = JSON.stringify(saved);
    const findings = (await buildHistoricalRunContext(saved))!.findings;
    expect(findings).toContain("HISTORICAL_RESULT_HASH_UNVERIFIABLE_LEGACY_PREIMAGE");
    expect(findings).not.toContain("HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE_LEGACY_PREIMAGE");
    expect(findings).not.toContain("HISTORICAL_ANALYSIS_HASH_MISMATCH");
    expect(findings).not.toContain("HISTORICAL_RESULT_HASH_MISMATCH");
    expect(JSON.stringify(saved)).toBe(before);
  });
  it("requires complete nested Python 0.1 profile evidence before record verification", async () => {
    const saved = await savedEnvelope();
    saved.analysis_run = structuredClone(legacyPythonRecord) as any;
    delete (saved.analysis_run!.analysis_run.solver_version as any).build_ref;
    saved.mechanics_result = { schema_version: "0.2.0", document_kind: "MechanicsResult", run_id: "run:legacy-python", model_ref: "model:legacy-python", status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE" }, summary: {}, results: [{ id: "result:legacy-python", entity_ref: "node:legacy", kind: "displacement_magnitude", value: 1, unit: "mm", dimension: "length" }], diagnostics: [] } as any;
    const findings = (await buildHistoricalRunContext(saved))!.findings;
    expect(findings).toContain("HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE_LEGACY_PREIMAGE");
    expect(findings).not.toContain("HISTORICAL_ANALYSIS_HASH_MISMATCH");
  });
  it("keeps locale-sensitive desktop 0.1 replay explicitly unverifiable", async () => {
    expect(await legacyDigest({ z: 1, "ä": 2 })).toBe("aebaf850cdb107bb9d7d528ee0c02528804edaac037352a39fb8ddef450fd003");
    const saved = await legacySavedEnvelope();
    saved.mechanics_result!.results[0].metadata = { z: 1, "ä": 2 } as any;
    saved.analysis_run!.analysis_run.result_refs[0].hash_refs[0].value = "7832a5d6150a56da1a4f0c8fa00c26a7350389b0fc8696707cd2abbbd32be0c1";
    const findings = (await buildHistoricalRunContext(saved))!.findings;
    expect(findings).toContain("HISTORICAL_RESULT_HASH_UNVERIFIABLE_LEGACY_PREIMAGE");
    expect(findings).not.toContain("HISTORICAL_RESULT_HASH_MISMATCH");
  });
  it("rejects malformed and hybrid desktop profile discriminators before verification", async () => {
    for (const mutate of [
      (saved: LocalProjectEnvelope) => { delete (saved.analysis_run!.analysis_run as any).professional_boundary; },
      (saved: LocalProjectEnvelope) => { (saved.analysis_run!.analysis_run as any).created_at = null; },
    ]) {
      const saved = await legacySavedEnvelope(); mutate(saved);
      const findings = (await buildHistoricalRunContext(saved))!.findings;
      expect(findings).toEqual(expect.arrayContaining(["HISTORICAL_RESULT_HASH_UNVERIFIABLE_LEGACY_PREIMAGE", "HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE_LEGACY_PREIMAGE"]));
      expect(findings).not.toEqual(expect.arrayContaining(["HISTORICAL_RESULT_HASH_MISMATCH", "HISTORICAL_ANALYSIS_HASH_MISMATCH"]));
    }
  });
  it("matches the existing bound analysis hash for native-shaped raw rows without changing saved evidence", async () => {
    const saved = await savedEnvelope(true);
    expect(saved.mechanics_result!.results.every((row) => !("dimension" in row))).toBe(true);
    const storedResultHash = saved.analysis_run!.analysis_run.hashes.find((hash) => hash.payload_scope === "received_result")!.value;
    expect(await canonicalSha256HexCheckedV1(saved.mechanics_result)).toBe(storedResultHash);
    const before = JSON.stringify(saved);
    const context = await buildHistoricalRunContext(saved);
    expect(context!.findings).toContain("HISTORICAL_INPUT_MANIFEST_MISSING");
    expect(context!.findings).not.toContain("HISTORICAL_RESULT_HASH_MISMATCH");
    expect(context!.findings).not.toContain("HISTORICAL_ENVELOPE_HASH_MISMATCH");
    expect(context!.findings).not.toContain("HISTORICAL_MODEL_HASH_MISMATCH");
    expect(context!.mechanicsResult).toBe(saved.mechanics_result);
    expect(context!.analysisRun).toBe(saved.analysis_run);
    expect(context!.modelHash).toBe(saved.model_hash);
    expect(context!.envelopeHash).toBe(saved.project_envelope_hash);
    expect(context).not.toHaveProperty("inputManifest");
    expect(JSON.stringify(saved)).toBe(before);
    expect(context!.mechanicsResult!.results.every((row) => !("dimension" in row))).toBe(true);
  });

  it("still reports a real changed result value against the bound native-shaped analysis hash", async () => {
    const saved = await savedEnvelope(true);
    saved.mechanics_result!.results.find((row) => row.kind === "element_local_axial_force")!.value += 1;
    const before = JSON.stringify(saved);
    const context = await buildHistoricalRunContext(saved);
    expect(context!.findings).toContain("HISTORICAL_RESULT_HASH_MISMATCH");
    expect(context!.findings).not.toContain("HISTORICAL_HASH_RECOMPUTE_UNAVAILABLE");
    expect(JSON.stringify(saved)).toBe(before);
  });

  it("retains an incorrect explicit dimension and reports the changed received carrier", async () => {
    const saved = await savedEnvelope(true);
    const row = saved.mechanics_result!.results.find((item) => item.kind === "element_local_axial_force")!;
    row.dimension = "stress";
    const inputManifest = await buildCurrentSessionInputManifest({ model: saved.model, solver: { solver_name: "synthetic", solver_version: "1", solver_build_ref: "synthetic@1", solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] });
    await expect(buildAnalysisRunPreview(saved.mechanics_result!, { inputManifest })).rejects.toThrow("ANALYSIS-RUN-RESULT-DIMENSION-MISMATCH");
    const before = JSON.stringify(saved);
    const context = await buildHistoricalRunContext(saved);
    expect(context!.findings).toContain("HISTORICAL_RESULT_HASH_MISMATCH");
    expect(context!.findings).not.toContain("HISTORICAL_HASH_RECOMPUTE_UNAVAILABLE");
    expect(context!.mechanicsResult).toBe(saved.mechanics_result);
    expect(row.dimension).toBe("stress");
    expect(JSON.stringify(saved)).toBe(before);
  });

  it("preserves saved bytes and reports missing historical manifest without rebinding inputs", async () => {
    const saved = await savedEnvelope();
    const before = JSON.stringify(saved);
    const context = await buildHistoricalRunContext(saved);
    expect(context!.designation).toBe("historical_saved_run");
    expect(context!.findings).toContain("HISTORICAL_INPUT_MANIFEST_MISSING");
    expect(context!.findings).not.toContain("HISTORICAL_RESULT_HASH_MISMATCH");
    expect(context!.mechanicsResult).toEqual(saved.mechanics_result);
    expect(context!.analysisRun).toEqual(saved.analysis_run);
    expect(context).not.toHaveProperty("inputManifest");
    expect(JSON.stringify(saved)).toBe(before);
  });

  it("surfaces inconsistent identities and all saved hash findings while retaining history", async () => {
    const saved = await savedEnvelope();
    saved.mechanics_result!.model_ref = "project:inconsistent";
    saved.mechanics_result!.run_id = "run:inconsistent";
    saved.model_hash!.value = "sha256:inconsistent";
    saved.project_envelope_hash!.value = "sha256:inconsistent";
    const context = await buildHistoricalRunContext(saved);
    expect(context!.findings).toEqual(expect.arrayContaining(["HISTORICAL_MODEL_REF_MISMATCH", "HISTORICAL_RUN_REF_MISMATCH", "HISTORICAL_MODEL_HASH_MISMATCH", "HISTORICAL_ENVELOPE_HASH_MISMATCH", "HISTORICAL_RESULT_HASH_MISMATCH"]));
    expect(context!.mechanicsResult).toEqual(saved.mechanics_result);
  });

  it("retains missing original hashes without inventing a manifest", async () => {
    const saved = await savedEnvelope();
    saved.model_hash = null;
    saved.project_envelope_hash = null;
    saved.analysis_run!.analysis_run.hashes = [];
    const context = await buildHistoricalRunContext(saved);
    expect(context!.modelHash).toBeNull();
    expect(context!.findings).toEqual(expect.arrayContaining(["HISTORICAL_MODEL_HASH_MISSING", "HISTORICAL_ENVELOPE_HASH_MISSING", "HISTORICAL_RESULT_HASH_MISSING"]));
  });
  it("keeps valid saved mechanics readable when analysis evidence is malformed", async () => {
    const saved = await savedEnvelope();
    saved.analysis_run = { analysis_run: { run_id: "saved-malformed", hashes: "corrupt" } } as unknown as LocalProjectEnvelope["analysis_run"];
    const before = JSON.stringify(saved.analysis_run);
    const context = await buildHistoricalRunContext(saved);
    expect(context!.findings).toContain("HISTORICAL_ANALYSIS_EVIDENCE_INVALID");
    expect(context!.mechanicsResult).toEqual(saved.mechanics_result);
    expect(JSON.stringify(context!.analysisRun)).toBe(before);
  });

  it.each([
    ["false", false],
    ["zero", 0],
    ["empty string", ""],
    ["array", []],
    ["incomplete object", { run_id: "received-incomplete" }],
    ["invalid row", {
      schema_version: "0.2.0", document_kind: "MechanicsResult", run_id: "received-invalid-row",
      model_ref: "model:invalid", status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE", professional_acceptance: "NOT_PROVIDED" },
      summary: {}, results: [{ id: "unsafe", kind: "displacement_magnitude", value: "not-a-number", unit: "mm", entity_ref: "node:unsafe" }], diagnostics: []
    }]
  ])("preserves a malformed %s carrier while withholding it from rendering", async (_label, carrier) => {
    const saved = await savedEnvelope();
    (saved as unknown as { mechanics_result: unknown }).mechanics_result = carrier;
    const before = structuredClone(carrier);
    const context = await buildHistoricalRunContext(saved);
    expect(context).not.toBeNull();
    expect(context!.designation).toBe("historical_saved_run");
    expect(context!.findings).toContain("HISTORICAL_MECHANICS_EVIDENCE_MALFORMED");
    expect(context!.mechanicsResult).toBeNull();
    expect(context!.rawMechanicsResult).toEqual(before);
    const expectedEnvelope = await computeProjectEnvelopeHash({
      model: saved.model, editor_intents: saved.editor_intents ?? [], proposal: saved.proposal ?? null,
      selected_review_target: saved.selected_review_target ?? null,
      mechanics_result: carrier, analysis_run: saved.analysis_run, model_hash: saved.model_hash
    } as never);
    expect(context!.envelopePayloadHash).toBe(expectedEnvelope!.value);
    expect((saved as unknown as { mechanics_result: unknown }).mechanics_result).toEqual(before);
  });

});
