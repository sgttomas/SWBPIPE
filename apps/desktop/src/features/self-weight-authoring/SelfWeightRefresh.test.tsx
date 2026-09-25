import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { SelfWeightPlanPanel, selfWeightPlanBatch } from "./SelfWeightPlanPanel";
import { draftIntent } from "../offline-proposal-intake/workflowSupport";
import { generateSelfWeightPlan } from "../../services/selfWeightPlanService";
import { applyOperationBatch, type OperationBatch } from "../../services/operationBatchService";
import { computeModelHash, canonicalJsonString } from "../../services/hashService";
import { loadPreviewModel } from "../../services/previewService";
import type { PreviewModel, EditorOperationIntent } from "../../types";

// Actual candidate WASM services throughout. vi.fn observes only the queue callback.
afterEach(cleanup);
const CASE = "load:invented-refresh";
const GRAVITY = -7;
const densityWeight = (density: number) => Math.PI / 4 * (0.1 ** 2 - 0.08 ** 2) * density * GRAVITY;
const enter = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
const caseOf = (model: PreviewModel) => model.load_cases.find(item => item.id === CASE)!;
const primitives = (model: PreviewModel) => caseOf(model).primitive_loads!;
const intensity = (model: PreviewModel) => (primitives(model)[0].magnitude as { value: number }).value;
function withoutProvenance(record: Record<string, unknown>) {
  const { provenance: _provenance, ...physical } = record;
  return physical;
}
async function apply(model: PreviewModel, batch: OperationBatch) {
  const before = await canonicalJsonString(model);
  const outcome = await applyOperationBatch(model, batch, await computeModelHash(model));
  expect(outcome.validation.application_status, JSON.stringify(outcome.diagnostics)).toBe("applied_to_session_model");
  expect(await canonicalJsonString(model)).toBe(before);
  return outcome.applied_model!;
}
async function edit(model: PreviewModel, target: EditorOperationIntent["target"], field: string, before: number, after: number, unit: string, dimension: string) {
  return apply(model, { batch_id: `batch:edit:${field}`, operations: [draftIntent({
    operation_kind: "modify", target,
    change: { change_id: `change:${field}`, change_kind: target.object_type === "Load" ? "update_load" : "set_field",
      field_label: field, field_path: field, before: String(before), after: String(after), unit, dimension,
      source_note: "Explicit invented refresh regression edit" }, rationale: "Invented refresh regression"
  })] });
}
async function fixture() {
  const model = structuredClone(await loadPreviewModel());
  // Only local invented fixture edits. Existing unrelated cases are retained.
  const pipe = model.pipe_segments[0];
  pipe.section = { outside_diameter: { value: 0.1, unit: "m" }, wall_thickness: { value: 0.01, unit: "m" }, material_density: { value: 1000, unit: "kg/m^3" } };
  delete pipe.section_ref;
  const hash = (await computeModelHash(model))!;
  const plan = await generateSelfWeightPlan(model, { case_id: CASE, label: "Invented refresh case", pipe_refs: [pipe.id],
    gravity: { value: GRAVITY, unit: "m/s^2", axis: "global_y" }, provenance: "Invented explicit inputs only", source_model_hash: hash.value });
  expect(plan.status, JSON.stringify(plan.diagnostics)).toBe("ready");
  const generated = await apply(model, selfWeightPlanBatch(plan.plan!, hash.value));
  const manual = { id: "load:manual-refresh-tip", category: "concentrated_force", target: { type: "node", node: pipe.to },
    direction: "global_y", magnitude: { value: -11, unit: "N" }, dimension: "force", provenance: "Ordinary invented manual load" };
  return apply(generated, { batch_id: "batch:manual-extra", operations: [draftIntent({ operation_kind: "create", target: { object_type: "Load", ref: CASE },
    change: { change_id: "change:manual-extra", change_kind: "create_primitive_load", field_label: "Manual extra", field_path: "primitive_loads",
      before: "not_present", after: JSON.stringify(manual), unit: "N", dimension: "force", source_note: "Ordinary manual load" }, rationale: "Invented manual extra" })] });
}
async function staleFixture() {
  const model = await fixture();
  return edit(model, { object_type: "Element", ref: model.pipe_segments[0].id }, "section.material_density.value", 1000, 2000, "kg/m^3", "density");
}
function chooseRefresh() {
  enter("Self-weight action", "refresh");
  enter("Existing self-weight case", CASE);
}
async function reviewRefresh() {
  fireEvent.click(screen.getByRole("button", { name: "Generate self-weight refresh plan" }));
  await waitFor(() => expect(screen.getByRole("button", { name: "Queue complete self-weight plan" })).toBeEnabled());
  expect(screen.getByLabelText("Self-weight source evidence")).toBeVisible();
}
async function queueReview(queue: ReturnType<typeof vi.fn>): Promise<OperationBatch> {
  expect(queue).not.toHaveBeenCalled();
  fireEvent.click(screen.getByRole("button", { name: "Queue complete self-weight plan" }));
  await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  return queue.mock.calls[0][0] as OperationBatch;
}
function assertPreserved(before: PreviewModel, after: PreviewModel) {
  expect(after.load_cases.map(item => item.id)).toEqual(before.load_cases.map(item => item.id));
  expect(after.load_cases.filter(item => item.id !== CASE)).toEqual(before.load_cases.filter(item => item.id !== CASE));
  const { primitive_loads: _beforeLoads, ...beforeCase } = caseOf(before);
  const { primitive_loads: _afterLoads, ...afterCase } = caseOf(after);
  expect(afterCase).toEqual(beforeCase);
  expect(primitives(after).map(item => item.id)).toEqual(primitives(before).map(item => item.id));
  expect(primitives(after)).toHaveLength(2);
  expect(primitives(after)[1]).toEqual(primitives(before)[1]);
  expect({ ...after, load_cases: before.load_cases }).toEqual(before);
}

describe("existing self-weight refresh through real WASM and structured apply", () => {
  it("reviews and queues a hash-bound refresh, then doubles density weight without duplicate or manual/case changes", async () => {
    const model = await staleFixture(); const original = await canonicalJsonString(model); const queue = vi.fn();
    render(<SelfWeightPlanPanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={queue} />);
    expect(screen.getByLabelText("Self-weight action")).toHaveValue("create");
    chooseRefresh();
    expect(screen.queryByRole("button", { name: "Queue complete self-weight plan" })).toBeNull();
    await reviewRefresh();
    expect(await canonicalJsonString(model)).toBe(original);
    const batch = await queueReview(queue);
    expect(batch.source_model_hash).toBe((await computeModelHash(model))!.value);
    expect(batch.operations).toHaveLength(1);
    expect(batch.operations[0].change.field_path).toBe("generated_self_weight");
    expect(await canonicalJsonString(model)).toBe(original);
    const refreshed = await apply(model, batch);
    expect(Math.abs(intensity(refreshed) - densityWeight(2000))).toBeLessThan(1e-10);
    assertPreserved(model, refreshed);
    // A queue must not rebind itself to a freshly computed claim after editing.
    const later = await edit(model, { object_type: "Element", ref: model.pipe_segments[0].id }, "section.material_density.value", 2000, 3000, "kg/m^3", "density");
    const rejected = await applyOperationBatch(later, batch, await computeModelHash(later));
    expect(rejected.validation.application_status).toBe("blocked");
    expect(rejected.applied_model).toBeNull();
  });

  it("blocks a modified generated magnitude until explicit keep, then preserves physical bytes as a manual override", async () => {
    const initial = await staleFixture();
    const model = await edit(initial, { object_type: "Load", ref: CASE }, "primitive_loads.0.magnitude.value", intensity(initial), -123.456, "N/m", "force_per_length");
    const original = await canonicalJsonString(model); const physical = JSON.stringify(withoutProvenance(primitives(model)[0])); const queue = vi.fn();
    render(<SelfWeightPlanPanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={queue} />);
    chooseRefresh();
    expect(screen.getByLabelText("Keep modified generated loads as manual overrides")).not.toBeChecked();
    fireEvent.click(screen.getByRole("button", { name: "Generate self-weight refresh plan" }));
    await waitFor(() => expect(screen.getByRole("alert")).toHaveTextContent("SELF_WEIGHT_GENERATED_LOAD_MODIFIED"));
    expect(screen.queryByRole("button", { name: "Queue complete self-weight plan" })).toBeNull();
    expect(queue).not.toHaveBeenCalled();
    expect(await canonicalJsonString(model)).toBe(original);
    fireEvent.click(screen.getByLabelText("Keep modified generated loads as manual overrides"));
    await reviewRefresh();
    const batch = await queueReview(queue);
    expect(await canonicalJsonString(model)).toBe(original);
    const preserved = await apply(model, batch);
    expect(JSON.stringify(withoutProvenance(primitives(preserved)[0]))).toBe(physical);
    expect(intensity(preserved)).toBe(-123.456);
    const provenance = JSON.parse(primitives(preserved)[0].provenance as string);
    expect(provenance.method).toBe("manual_override_of_generated_self_weight/v1");
    expect(provenance.decision).toBe("preserve_modified_generated_load");
    expect(provenance.source_model_hash).toBe(batch.source_model_hash);
    expect(provenance.original_generation_provenance).toEqual(JSON.parse(primitives(model)[0].provenance as string));
    assertPreserved(model, preserved);
  });

  it.each(["model", "requestEpoch", "selection", "busy", "inputs"] as const)("withdraws a prepared refresh after %s changes until generated again", async change => {
    const model = await staleFixture(); const queue = vi.fn();
    const props = { model, selection: { type: "project" as const, id: model.project.id }, onQueueBatch: queue, requestEpoch: 0, busy: false };
    const view = render(<SelfWeightPlanPanel {...props} />);
    chooseRefresh(); await reviewRefresh();
    if (change === "inputs") fireEvent.click(screen.getByLabelText("Keep modified generated loads as manual overrides"));
    else {
      const next = change === "model" ? { ...props, model: await edit(model, { object_type: "Element", ref: model.pipe_segments[0].id }, "section.material_density.value", 2000, 3000, "kg/m^3", "density") }
        : change === "requestEpoch" ? { ...props, requestEpoch: 1 }
        : change === "selection" ? { ...props, selection: { type: "project" as const, id: "project:changed-selection" } }
        : { ...props, busy: true };
      view.rerender(<SelfWeightPlanPanel {...next} />);
      if (change === "busy") view.rerender(<SelfWeightPlanPanel {...props} />);
    }
    expect(screen.queryByRole("button", { name: "Queue complete self-weight plan" })).toBeNull();
    expect(queue).not.toHaveBeenCalled();
    await reviewRefresh();
    const batch = await queueReview(queue);
    expect(batch.operations).toHaveLength(1);
  });
});
