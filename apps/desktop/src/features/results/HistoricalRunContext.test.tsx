import { describe, expect, it } from "vitest";
import { buildHistoricalRunContext } from "./HistoricalRunContext";
import { loadPreviewModel, runPreviewMechanics, buildAnalysisRunPreview } from "../../services/previewService";
import { canonicalSha256Hex, computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import type { LocalProjectEnvelope } from "../../types";

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

describe("transient HistoricalRunContext integrity", () => {
  it("matches the existing bound analysis hash for native-shaped raw rows without changing saved evidence", async () => {
    const saved = await savedEnvelope(true);
    expect(saved.mechanics_result!.results.every((row) => !("dimension" in row))).toBe(true);
    const storedResultHash = saved.analysis_run!.analysis_run.hashes.find((hash) => hash.payload_scope === "result_envelope")!.value;
    expect(await canonicalSha256Hex(saved.mechanics_result)).not.toBe(storedResultHash.replace(/^sha256:/, ""));
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

  it("retains an incorrect explicit dimension and surfaces the existing binder admission failure", async () => {
    const saved = await savedEnvelope(true);
    const row = saved.mechanics_result!.results.find((item) => item.kind === "element_local_axial_force")!;
    row.dimension = "stress";
    const inputManifest = await buildCurrentSessionInputManifest({ model: saved.model, solver: { solver_name: "synthetic", solver_version: "1", solver_build_ref: "synthetic@1", solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] });
    await expect(buildAnalysisRunPreview(saved.mechanics_result!, { inputManifest })).rejects.toThrow("ANALYSIS-RUN-RESULT-DIMENSION-MISMATCH");
    const before = JSON.stringify(saved);
    const context = await buildHistoricalRunContext(saved);
    expect(context!.findings).toContain("HISTORICAL_HASH_RECOMPUTE_UNAVAILABLE");
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

});
