import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { SelfWeightPlanPanel, selfWeightPlanBatch } from "./SelfWeightPlanPanel";
import { generateSelfWeightPlan, SELF_WEIGHT_SCOPES } from "../../services/selfWeightPlanService";
import { applyOperationBatch } from "../../services/operationBatchService";
import { computeModelHash, canonicalJsonString } from "../../services/hashService";
import { loadPreviewModel } from "../../services/previewService";
import { chooseVirtualMultiTarget } from "../../test-support/workspaceTestControls";
import type { PreviewModel } from "../../types";

// W-15 slice b through the actual candidate Wasm engines: the bundled demo's
// valve C-130 (420 N, centre of gravity (0.12, 0.02, 0) m in global X/Y/Z from
// its node, mechanics_geometry_only) maps pipe:P-130.
afterEach(() => { cleanup(); vi.restoreAllMocks(); });
const enter = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
const CASE = "load:invented-component-weight";
async function plan(model: PreviewModel) {
  const hash = (await computeModelHash(model))!;
  const outcome = await generateSelfWeightPlan(model, { case_id: CASE, label: "Invented component weight", pipe_refs: ["pipe:P-130"],
    gravity: { value: -9.8, unit: "m/s^2", axis: "global_y" }, provenance: "Invented test", source_model_hash: hash.value });
  expect(outcome.status, JSON.stringify(outcome.diagnostics)).toBe("ready");
  return { hash, plan: outcome.plan! };
}
describe("component weight in the self-weight plan (W-15 slice b)", () => {
  it("replaces the mapped member's self-weight with the valve's nodal force and moment, and applies atomically", async () => {
    const model = await loadPreviewModel(); const before = await canonicalJsonString(model);
    const { hash, plan: generated } = await plan(model);
    expect(generated.scope_label).toBe(SELF_WEIGHT_SCOPES[1]);
    expect(generated.warnings ?? []).toEqual([]);
    const loads = generated.changes.slice(1).map((change) => JSON.parse(change.after));
    // F = (0, -420, 0) N at node:N-130; M = r x F = (0, 0, 0.12 * -420) N*m.
    expect(loads.map((load) => [load.category, load.target, load.direction, load.magnitude])).toEqual([
      ["weight", { type: "node", node: "node:N-130" }, "global_y", { value: -420, unit: "N" }],
      ["weight", { type: "node", node: "node:N-130" }, "rotation_z", { value: 0.12 * -420, unit: "N*m" }]
    ]);
    for (const evidence of generated.source_evidence as Record<string, unknown>[]) {
      expect([evidence.component_id, evidence.omitted_pipe_id]).toEqual(["component:C-130", "pipe:P-130"]);
    }
    const batch = selfWeightPlanBatch(generated, hash.value);
    expect(batch.operations).toHaveLength(3);
    const applied = await applyOperationBatch(model, batch, hash);
    expect(applied.validation.application_status, JSON.stringify(applied.diagnostics)).toBe("applied_to_session_model");
    expect(applied.applied_model!.load_cases.find((load) => load.id === CASE)!.primitive_loads).toHaveLength(2);
    expect(await canonicalJsonString(model)).toBe(before);
  });
  it("shows the warning when the valve fails a weight condition and keeps the pipe weight", async () => {
    const model = await loadPreviewModel();
    const valve = model.components.find((component) => component.id === "component:C-130")!;
    delete valve.mechanics_interface;
    const pipe = model.pipe_segments.find((segment) => segment.id === "pipe:P-130")!;
    pipe.section.material_density = { value: 7800, unit: "kg/m^3" };
    const queue = vi.fn();
    render(<SelfWeightPlanPanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={queue} />);
    expect(screen.getByText(/carries that component's weight instead, at the component's node with the moment r × F/)).toBeVisible();
    enter("Self-weight case ID", CASE); enter("Self-weight case label", "Invented component weight");
    enter("Gravity value", "-9.8"); enter("Gravity unit", "m/s^2"); enter("Gravity direction", "global_y");
    enter("Self-weight provenance", "Invented test only"); chooseVirtualMultiTarget("self-weight-pipes", "pipe:P-130");
    fireEvent.click(screen.getByRole("button", { name: "Generate self-weight plan" }));
    await waitFor(() => expect(screen.getByRole("button", { name: "Queue complete self-weight plan" })).toBeEnabled());
    const warnings = screen.getByTestId("self-weight-plan-warnings");
    expect(warnings).toHaveTextContent("SELF_WEIGHT_COMPONENT_WEIGHT_NOT_APPLIED");
    expect(warnings).toHaveTextContent("component:C-130");
    expect(warnings).toHaveTextContent("generated in its place");
    expect(screen.getByText("2 proposed changes for 1 selected pipes.")).toBeVisible();
    fireEvent.click(screen.getByRole("button", { name: "Queue complete self-weight plan" }));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    expect(JSON.parse(queue.mock.calls[0][0].operations[1].change.after).category).toBe("distributed_force");
  });
});
