/** G10: the F2a milestone model shaped for the desktop app
 * (fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_model.json) goes
 * through the real model-index and input-manifest code paths the shell uses on open
 * and on solve. The request's bare model is the control: it lacks the display
 * fields and is refused on both paths. The PP integration test
 * core/product_physics/tests/g10_desktop_milestone_model.rs pins its solve bytes. */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { PreviewModel } from "../types";
import { buildModelIndex, modelIndexFor } from "../features/workspace/modelIndex";
import { buildCurrentSessionInputManifest, verifyCurrentSessionInputManifest } from "./inputManifestService";
import { evaluateModelDocumentLocal } from "./projectService";

const root = resolve(__dirname, "../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
const desktopModel = () => json("fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_model.json") as PreviewModel;
const bareModel = () => json("fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json").model as PreviewModel;

/** The solve path's manifest arguments (workspaceSession.ts), for one solver mode. */
function manifestArgs(model: PreviewModel, solverMode: string) {
  return {
    model,
    solver: {
      solver_name: "open_pipe_stress_product_physics",
      solver_version: "0.2.0",
      solver_build_ref: "open_pipe_stress_product_physics@0.2.0",
      solver_mode: solverMode,
      settings: {
        nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1",
        sparse_evidence_lane: solverMode === "sparse_interactive"
      }
    },
    active_rule_packs: [],
    external_assets: []
  };
}

describe("G10 desktop-shaped milestone model", () => {
  it("indexes without error, with every authored entity", () => {
    const model = desktopModel();
    const index = modelIndexFor(model, 1, 0);
    expect(index.summaries).toEqual({
      entityCount: 1 + model.materials!.length + model.nodes.length + model.pipe_segments.length + model.supports.length + model.load_cases.length,
      nodeCount: 2,
      pipeCount: 1,
      supportCount: model.supports.length,
      componentCount: 0,
      invalidGeometryCount: 0
    });
    expect(index.labels.get(index.projectKey)).toBe("RF-SKEW-T-CANT-OFF-122-r1e-04 milestone (G10 witness)");
  });

  it.each(["sparse_interactive", "dense_scrutiny"])("builds and verifies the %s input manifest", async (solverMode) => {
    const model = desktopModel();
    const evidence = await buildCurrentSessionInputManifest(manifestArgs(model, solverMode));
    await expect(verifyCurrentSessionInputManifest(evidence)).resolves.toBeUndefined();
    expect(evidence.manifest.model_basis.model_payload).toEqual(model);
    expect(evidence.manifest.load_basis.load_cases.map((item) => item.status)).toEqual(["preview_only"]);
  });

  it("opens as the current 0.8.0 document it is, unchanged", () => {
    const model = desktopModel();
    const before = JSON.stringify(model);
    expect(evaluateModelDocumentLocal(model)).toMatchObject({ status: "current", source_schema_version: "0.8.0", target_schema_version: "0.8.0" });
    expect(model.pressure_contract).toBeUndefined();
    expect(JSON.stringify(model)).toBe(before);
  });

  it("control: the request's bare model is refused on both paths", async () => {
    const bare = bareModel();
    expect(() => buildModelIndex(bare, 1, 0)).toThrow(TypeError);
    await expect(buildCurrentSessionInputManifest(manifestArgs(bare, "sparse_interactive"))).rejects.toThrow(
      "INPUT-MANIFEST-LOAD-BASIS-INCOMPLETE"
    );
  });
});
