import { describe, expect, it } from "vitest";
import { loadPreviewModel } from "../../services/previewService";
import type { PreviewModel } from "../../types";
import { runModelChecks } from "./modelChecks";

// DEL-04-06 Design `on-demand-model-checks.md`: the on-demand checks run the
// solve's own pre-formation checks through the wasm engine, beside the
// document validator's slot checks; invented changes to the bundled demo.
function clone(model: PreviewModel): PreviewModel {
  return JSON.parse(JSON.stringify(model)) as PreviewModel;
}

/** The demo with a second line no element joins to it, anchored at its first node. */
function withSecondLine(model: PreviewModel): PreviewModel {
  const next = clone(model);
  next.nodes.push(
    { ...next.nodes[0], id: "node:checks-c", position: { x: 40, y: 40, z: 0 } },
    { ...next.nodes[0], id: "node:checks-d", position: { x: 41, y: 40, z: 0 } }
  );
  next.pipe_segments.push({ ...next.pipe_segments[0], id: "pipe:checks-c-d", from: "node:checks-c", to: "node:checks-d" });
  next.supports.push({ ...next.supports[0], id: "support:checks-c", node: "node:checks-c" });
  return next;
}

describe("on-demand model checks", () => {
  it("lists the solve's topology warning with its code and text, linked to the entity, and leaves the model unchanged", async () => {
    const model = withSecondLine(await loadPreviewModel());
    const before = JSON.stringify(model);
    const run = runModelChecks(model);
    expect(JSON.stringify(model)).toBe(before);
    expect(run.model).toBe(model);
    expect(run.solve).toEqual({ kind: "checked" });
    const disconnected = run.findings.filter((item) => item.diagnostic.code === "MODEL_TOPOLOGY_DISCONNECTED");
    expect(disconnected).toHaveLength(1);
    expect(disconnected[0].group).toBe("solve");
    expect(disconnected[0].diagnostic).toMatchObject({ id: "diagnostic:topology:disconnected", severity: "warning", source: "core/product_physics" });
    expect(disconnected[0].diagnostic.message).toContain("form 2 disconnected parts");
    expect(disconnected[0].diagnostic.message).toContain("(from node:checks-c; 2 nodes; supported)");
    expect(disconnected[0].entity).toEqual({ type: "node", id: model.nodes[0].id });
  });

  it("reports the solve's restraint pre-check as blocking, with no entity when it names none", async () => {
    const model = clone(await loadPreviewModel());
    model.supports = [];
    const run = runModelChecks(model);
    const missing = run.findings.find((item) => item.diagnostic.code === "SUPPORT_INPUT_MISSING");
    expect(missing?.diagnostic).toMatchObject({ severity: "blocking", affected_refs: ["supports"] });
    expect(missing?.entity).toBeNull();
  });

  it("names a pipe end that points nowhere, linked to the pipe", async () => {
    const model = clone(await loadPreviewModel());
    model.pipe_segments[0] = { ...model.pipe_segments[0], to: "node:checks-missing" };
    const run = runModelChecks(model);
    const unknown = run.findings.find((item) => item.group === "solve" && item.diagnostic.code === "PIPE_ENDPOINT_UNKNOWN");
    expect(unknown?.diagnostic.severity).toBe("blocking");
    expect(unknown?.entity).toEqual({ type: "pipe", id: model.pipe_segments[0].id });
  });

  it("states that the solve's checks did not run when the solve cannot read the model, with the solve's refusal", async () => {
    const model = clone(await loadPreviewModel());
    (model.nodes[0] as unknown as Record<string, unknown>).undeclared_member = true;
    const run = runModelChecks(model);
    expect(run.solve.kind).toBe("refused");
    expect(run.solve.kind === "refused" && run.solve.refusal).toContain("undeclared_member");
    expect(run.findings.some((item) => item.group === "solve")).toBe(false);
  });
});
