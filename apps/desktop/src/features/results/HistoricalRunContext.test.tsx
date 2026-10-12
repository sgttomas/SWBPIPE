import { buildAnalysisRunV03, ANALYSIS_RUN_SCHEMA_VERSION_RETIRED } from "../../services/analysisRunCompatibility";
import { render, screen, cleanup } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { buildHistoricalRunContext, buildBundledReferenceContext, HistoricalRunPanel } from "./HistoricalRunContext";
import { loadBundledMechanicsReference, hasNativeMechanicsInvocation } from "../../services/previewService";
import historicalModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import historicalResult from "../../../../../fixtures/product_preview/invented_demo_result_legacy_0_1.json";
import { canonicalSha256HexCheckedV1, computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { PRECISION_CONTRACT_ID } from "./numericalResultQuality";
import type { LocalProjectEnvelope, PreviewModel, MechanicsResult } from "../../types";

/** A saved run with a current (0.3.0) analysis record. Its received result is a
 * synthetic precision-1 statement over the demo carrier's rows: a header-only
 * route whose record needs no producer evidence, so these tests exercise the
 * saved-run integrity checks only. */
async function savedEnvelope() {
  const model = structuredClone(historicalModel) as unknown as PreviewModel;
  const mechanics_result = structuredClone(historicalResult) as unknown as MechanicsResult;
  Object.assign(mechanics_result, {
    schema_version: "0.2.0",
    producer: { component_name: "open_pipe_stress_product_physics", component_version: "0.2.0", semantic_contract_id: PRECISION_CONTRACT_ID },
    numerical_quality: { value_representation: "finite_binary64", publication_quantization: "none", integrity_policy: "M03-INTEGRITY-v1", status: "not_assessed", cases: [] },
    formulation_basis: { profile_id: "product_preview_mechanics_v1", limitations: ["Synthetic historical-run fixture only"] },
  });
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: mechanics_result.producer!.component_name, solver_version: mechanics_result.producer!.component_version, solver_build_ref: "synthetic@1", solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] });
  const analysis_run = await buildAnalysisRunV03(mechanics_result, inputManifest);
  const model_hash = await computeModelHash(model);
  const payload = { model, mechanics_result, analysis_run, model_hash, editor_intents: [], proposal: null, selected_review_target: null };
  const project_envelope_hash = await computeProjectEnvelopeHash(payload);
  return { ...payload, project_envelope_hash } as unknown as LocalProjectEnvelope;
}

async function rehash(saved: LocalProjectEnvelope) {
  saved.project_envelope_hash = await computeProjectEnvelopeHash({ model: saved.model, editor_intents: saved.editor_intents ?? [], proposal: saved.proposal ?? null, selected_review_target: saved.selected_review_target ?? null, mechanics_result: saved.mechanics_result, analysis_run: saved.analysis_run, model_hash: saved.model_hash });
  return saved;
}

describe("transient HistoricalRunContext integrity", () => {
  it.each(["0.1.0", "0.2.0"])("refuses a saved %s analysis record by name and verifies none of its claims", async version => {
    const saved = await savedEnvelope();
    saved.analysis_run!.schema_version = version;
    await rehash(saved);
    const before = JSON.stringify(saved);
    const context = await buildHistoricalRunContext(saved);
    expect(context!.findings).toContain(ANALYSIS_RUN_SCHEMA_VERSION_RETIRED);
    expect(context!.findings).not.toEqual(expect.arrayContaining(["HISTORICAL_RESULT_HASH_MISMATCH"]));
    expect(context!.findings).not.toContain("HISTORICAL_ANALYSIS_HASH_MISMATCH");
    expect(context!.findings).not.toContain("HISTORICAL_HASH_RECOMPUTE_UNAVAILABLE");
    // The saved evidence stays readable and unchanged; only a re-run records a current run.
    expect(context!.analysisRun).toBe(saved.analysis_run);
    expect(context!.mechanicsResult).toBe(saved.mechanics_result);
    expect(JSON.stringify(saved)).toBe(before);
  });

  it("names an unknown saved record version as unsupported", async () => {
    const saved = await savedEnvelope();
    saved.analysis_run!.schema_version = "0.9.0";
    const findings = (await buildHistoricalRunContext(await rehash(saved)))!.findings;
    expect(findings).toContain("HISTORICAL_ANALYSIS_SCHEMA_VERSION_UNSUPPORTED");
    expect(findings).not.toContain(ANALYSIS_RUN_SCHEMA_VERSION_RETIRED);
  });

  it("verifies a current record and reports a changed record claim", async () => {
    const saved = await savedEnvelope();
    expect(saved.analysis_run!.schema_version).toBe("0.3.0");
    const findings = (await buildHistoricalRunContext(saved))!.findings;
    expect(findings).not.toEqual(expect.arrayContaining(["HISTORICAL_RESULT_HASH_MISMATCH", "HISTORICAL_ANALYSIS_HASH_MISMATCH", "HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE", ANALYSIS_RUN_SCHEMA_VERSION_RETIRED]));
    const tampered = await savedEnvelope();
    tampered.analysis_run!.analysis_run.hashes.find((hash) => hash.payload_scope === "analysis_run_record")!.value = "0".repeat(64);
    const tamperedFindings = (await buildHistoricalRunContext(await rehash(tampered)))!.findings;
    expect(tamperedFindings).toContain("HISTORICAL_ANALYSIS_HASH_MISMATCH");
    expect(tamperedFindings).not.toContain("HISTORICAL_RESULT_HASH_MISMATCH");
  });

  it("matches the bound received-result hash without changing saved evidence", async () => {
    const saved = await savedEnvelope();
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
  });

  it("still reports a real changed result value against the bound received-result hash", async () => {
    const saved = await savedEnvelope();
    saved.mechanics_result!.results.find((row) => row.kind === "element_local_axial_force")!.value += 1;
    const before = JSON.stringify(saved);
    const context = await buildHistoricalRunContext(saved);
    expect(context!.findings).toContain("HISTORICAL_RESULT_HASH_MISMATCH");
    expect(context!.findings).not.toContain("HISTORICAL_HASH_RECOMPUTE_UNAVAILABLE");
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

it("keeps unsupported raw0.2 readable history and unqualified without rewriting evidence", async () => {
 const saved = await savedEnvelope();
 for (const header of ["producer", "numerical_quality", "formulation_basis"]) delete (saved.mechanics_result as unknown as Record<string, unknown>)[header];
 const before = JSON.stringify(saved.mechanics_result);
 const context = await buildHistoricalRunContext(saved);
 expect(context?.mechanicsResult).not.toBeNull();
 expect(context?.findings).toContain("SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED");
 expect(JSON.stringify(saved.mechanics_result)).toBe(before);
});

it("retains contradictory null-header historical bytes and hash claims while withholding interpretation", async () => {
  const saved = await savedEnvelope();
  Object.assign(saved.mechanics_result!, { producer: null });
  const before = JSON.stringify(saved);
  const context = await buildHistoricalRunContext(saved);
  expect(context?.mechanicsResult).not.toBeNull();
  expect(context?.rawMechanicsResult).toEqual(saved.mechanics_result);
  expect(context?.findings).toContain("SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED");
  expect(JSON.stringify(saved)).toBe(before);
});

afterEach(cleanup);
describe("bundled reference inspection", () => {
  it("retains source and original model without Current evidence or saved-history designation", async () => {
    const reference = await loadBundledMechanicsReference();
    const before = JSON.stringify(reference);
    const context = buildBundledReferenceContext(reference);
    expect(context.designation).toBe("bundled_reference");
    expect(context.rawMechanicsResult).toBe(reference.source);
    expect(context.referenceModel).toBe(reference.model);
    expect(context.rawAnalysisRun).toBeNull();
    expect(context.modelHash).toBeNull();
    expect(context.envelopeHash).toBeNull();
    expect(hasNativeMechanicsInvocation(reference.source, reference.model)).toBe(false);
    render(<HistoricalRunPanel context={context} />);
    expect(screen.getByRole("region", { name: "Bundled reference — not a solve for the current model" })).toBeInTheDocument();
    expect(screen.queryByText("Historical saved run")).not.toBeInTheDocument();
    expect(JSON.stringify(reference)).toBe(before);
  });
});
