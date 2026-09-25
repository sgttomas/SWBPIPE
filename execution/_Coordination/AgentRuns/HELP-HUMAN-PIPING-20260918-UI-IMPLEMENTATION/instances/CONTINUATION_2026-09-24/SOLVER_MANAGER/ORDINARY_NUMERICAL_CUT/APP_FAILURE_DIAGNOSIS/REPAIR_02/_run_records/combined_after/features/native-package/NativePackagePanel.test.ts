import { describe, expect, it } from "vitest";
import { buildNativePackageReview } from "./NativePackagePanel";
import { loadPreviewModel, runPreviewMechanics, buildAnalysisRunPreview } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";

async function currentPrecisionSession() {
  const model = await loadPreviewModel();
  const result = await runPreviewMechanics(model);
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: { solver_name: "open_pipe_stress_product_physics", solver_version: "0.2.0", solver_build_ref: "open_pipe_stress_product_physics@0.2.0", solver_mode: "sparse_interactive", settings: {} },
    active_rule_packs: [], external_assets: []
  });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  const receivedHash = analysisRun.analysis_run.hashes.find(hash => hash.payload_scope === "received_result")!.value;
  expect(analysisRun.schema_version).toBe("0.3.0");
  return { model, result, analysisRun, receivedHash };
}

describe("native review package received-result hash binding", () => {
  it("uses the actual current received hash and discloses unsupported future versions", async () => {
    const session = await currentPrecisionSession();
    const input = { ...session, editorIntents: [], modelHash: null, projectSummary: null, proposal: null, selectedReviewTarget: null, storageCapability: null };
    const before = JSON.stringify(input);
    const packet = buildNativePackageReview(input);
    const member = packet.manifest.package_members.find(item => item.path === "results/result_envelope_ref.json");
    expect(member?.hash_status).toBe(`sha256:${session.receivedHash}`);
    expect(JSON.stringify(input)).toBe(before);
    const future = structuredClone(session.analysisRun);
    future.schema_version = "0.4.0";
    const unsupported = buildNativePackageReview({ ...input, analysisRun: future });
    expect(unsupported.manifest.package_members.find(item => item.path === "results/result_envelope_ref.json")?.hash_status).toBe("TBD_analysis_record_version_unsupported");
  });
});
