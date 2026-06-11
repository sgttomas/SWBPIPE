import { describe, expect, it } from "vitest";
import type { PreviewModel } from "../types";
import { loadPreviewModel, runPreviewMechanics } from "./previewService";

function cloneModel(model: PreviewModel): PreviewModel {
  return JSON.parse(JSON.stringify(model)) as PreviewModel;
}

describe("previewService mechanics browser fallback", () => {
  it("returns the bundled solved fixture when no model payload is supplied", async () => {
    const result = await runPreviewMechanics();

    expect(result.status.mechanics).toBe("MECHANICS_SOLVED");
    expect(result.model_ref).toBe("project:invented-loop-01");
    expect(result.results.length).toBeGreaterThan(0);
  });

  it("keeps the bundled solved fixture for the unchanged bundled model", async () => {
    const model = await loadPreviewModel();
    const result = await runPreviewMechanics(model);

    expect(result.status.mechanics).toBe("MECHANICS_SOLVED");
    expect(result.model_ref).toBe("project:invented-loop-01");
    expect(result.results.length).toBeGreaterThan(0);
  });

  it("blocks edited-model fixture reuse instead of publishing stale result rows", async () => {
    const original = await loadPreviewModel();
    const edited = cloneModel(original);
    edited.project.id = "project:edited-browser-solve";
    edited.materials![0].elastic_modulus.value = 195_000_000_000;

    const result = await runPreviewMechanics(edited);

    expect(result.model_ref).toBe("project:edited-browser-solve");
    expect(result.status.mechanics).toBe("MODEL_INCOMPLETE");
    expect(result.status.rule_check).toBe("RULE_INPUTS_INCOMPLETE");
    expect(result.results).toEqual([]);
    expect(result.summary.node_count).toBe(edited.nodes.length);
    expect(result.summary.segment_count).toBe(edited.pipe_segments.length);
    expect(result.summary.support_count).toBe(edited.supports.length);
    expect(result.summary.load_case_count).toBe(edited.load_cases.length);
    expect(result.diagnostics).toHaveLength(1);
    expect(result.diagnostics[0]).toMatchObject({
      code: "BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL",
      severity: "blocking",
      affected_refs: ["project:edited-browser-solve"]
    });
    expect(original.project.id).toBe("project:invented-loop-01");
    expect(original.materials![0].elastic_modulus.value).toBe(200_000_000_000);
  });
});
