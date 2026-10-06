/** I75 scratch probe (never committed): the preview-physics-1 Current exports that the new
 * result-export test builds, run identically on base and candidate copies for the dump comparison. */
import { expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import { buildAnalysisRunPreview, runPreviewMechanics } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { createNativeMechanicsReplay, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { buildCurrentResultExport } from "./resultExportAdapter";
it("preview Current exports", async () => {
  for (const mode of ["sparse_interactive", "dense_scrutiny"] as const) {
    const { model } = nativeMechanicsReplayPair(mode, { profile: "preview" });
    (window as any).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(createNativeMechanicsReplay({ profile: "preview" }).invoke);
    const result = await runPreviewMechanics(model, mode);
    const inputManifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: "test:t6s-replay", solver_mode: mode, settings: {} }, active_rule_packs: [], external_assets: [] });
    const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
    expect((await buildCurrentResultExport({ model, result, analysisRun, inputManifest })).schema_version).toBe("0.3.0");
  }
});
