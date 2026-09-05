import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import fixture from "../../../../../fixtures/hanger/invented_hanger_library_valid.json";
import { buildHangerSelectionBatch, type HangerDocument } from "../hanger-selection/hangerSelection";
import { HangerSelectionPanel } from "../hanger-selection/HangerSelectionPanel";
import { SelfWeightPlanPanel, selfWeightPlanBatch } from "../self-weight-authoring/SelfWeightPlanPanel";
import { OfflineProposalIntakePanel, parseOfflineBatch } from "./OfflineProposalIntakePanel";
import * as selfWeightService from "../../services/selfWeightPlanService";
import { generateSelfWeightPlan } from "../../services/selfWeightPlanService";
import { loadPreviewModel } from "../../services/previewService";
import { applyOperationBatch } from "../../services/operationBatchService";
import { computeModelHash, canonicalJsonString } from "../../services/hashService";
import type { PreviewModel } from "../../types";
afterEach(() => { cleanup(); vi.restoreAllMocks(); });
const enter = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
async function massModel() {
  const model = await loadPreviewModel();
  model.pipe_segments[0].section.material_density = { value: 7800, unit: "kg/m^3" };
  return model;
}
function fill(model: PreviewModel) {
  enter("Self-weight case ID", "load:self-weight-invented"); enter("Self-weight case label", "Invented self-weight");
  enter("Gravity value", "-9.8"); enter("Gravity unit", "m/s^2"); enter("Gravity direction", "global_y");
  enter("Self-weight provenance", "Invented test only"); fireEvent.click(screen.getByLabelText(model.pipe_segments[0].id));
}
describe("bounded workflow actual engine tests", () => {
  it("replaces old support stiffness, retains complete selected provenance and rejects stale before state", async () => {
    const model = await loadPreviewModel(); const support = model.supports[0];
    support.family = "variable_spring_hanger";
    support.stiffness = { dof: "UY", value: { value: 111, unit: "N/m" } };
    support.hanger = { hanger_type: "variable_spring_hanger", stiffness: { dof: "UY", value: { value: 222, unit: "N/m" } } };
    const original = await canonicalJsonString(model); const doc = structuredClone(fixture) as HangerDocument;
    // Explicit invented test values, never production defaults or substitutions.
    doc.hanger_records[0].hanger.cold_load = { magnitude: 15, unit: "N", dimension: "force", provenance: { ...doc.hanger_records[0].provenance } };
    doc.hanger_records[0].hanger.hot_load = { magnitude: 18, unit: "N", dimension: "force", provenance: { ...doc.hanger_records[0].provenance } };
    const batch = await buildHangerSelectionBatch(model, doc, doc.hanger_records[0].hanger_id, { supportId: support.id, restraints: ["UY"] });
    const after = JSON.parse(batch.operations[0].change.after);
    expect(after.stiffness).toBeUndefined(); expect(after.nonlinear).toBeUndefined();
    expect(JSON.parse(after.provenance).record_snapshot).toEqual(doc.hanger_records[0]);
    expect(after.provenance).toBe(batch.operations[0].change.source_note);
    const result = await applyOperationBatch(model, batch, await computeModelHash(model));
    expect(result.validation.application_status, JSON.stringify(result.diagnostics)).toBe("applied_to_session_model");
    expect(result.applied_model!.supports[0].stiffness).toBeUndefined();
    expect(result.applied_model!.supports[0].hanger!.stiffness).toEqual({ dof: "UY", value: { value: 123, unit: "N/m" } });
    expect(JSON.parse(result.applied_model!.supports[0].provenance).library_metadata).toEqual(doc.hanger_library);
    expect(await canonicalJsonString(model)).toBe(original);
    const stale = structuredClone(model); stale.supports[0].provenance = "Changed after preparation";
    const rejected = await applyOperationBatch(stale, batch, await computeModelHash(stale));
    expect(rejected.validation.application_status).toBe("blocked"); expect(rejected.applied_model).toBeNull();
  });
  it("preserves missing imported values and returns backend missing-field diagnostics", async () => {
    const model = await loadPreviewModel(); const doc = fixture as HangerDocument;
    const batch = await buildHangerSelectionBatch(model, doc, doc.hanger_records[0].hanger_id, { supportId: model.supports[0].id, restraints: ["UY"] });
    expect(JSON.parse(batch.operations[0].change.after).hanger.cold_load).toBeUndefined();
    const result = await applyOperationBatch(model, batch, await computeModelHash(model));
    expect(result.validation.application_status).toBe("blocked"); expect(result.applied_model).toBeNull();
    expect(result.diagnostics.some(item => item.message.includes("cold_load"))).toBe(true);
  });
  it("does not carry variable stiffness into constant-effort selection and requires explicit direction", async () => {
    const model = await loadPreviewModel(); const doc = fixture as HangerDocument;
    const selected = doc.hanger_records.find(record => record.hanger.hanger_type === "constant_effort_support")!;
    const batch = await buildHangerSelectionBatch(model, doc, selected.hanger_id, { supportId: model.supports[0].id, restraints: ["UY"] });
    expect(JSON.parse(batch.operations[0].change.after).hanger.stiffness).toBeUndefined();
    const result = await applyOperationBatch(model, batch, await computeModelHash(model));
    expect(result.validation.application_status, JSON.stringify(result.diagnostics)).toBe("applied_to_session_model");
    await expect(buildHangerSelectionBatch(model, doc, "missing", { supportId: model.supports[0].id, restraints: ["UY"] })).rejects.toThrow("missing");
    await expect(buildHangerSelectionBatch(model, doc, selected.hanger_id, { supportId: model.supports[0].id, restraints: [] })).rejects.toThrow("Explicitly");
  });
  it("reports native-only library access honestly in browser mode", async () => {
    const model = await loadPreviewModel(); const queue = vi.fn();
    render(<HangerSelectionPanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={queue} />);
    fireEvent.click(screen.getByRole("button", { name: "Refresh hanger libraries" }));
    await waitFor(() => expect(screen.getByRole("alert")).toHaveTextContent("DESKTOP-ONLY"));
    expect(queue).not.toHaveBeenCalled();
  });
  it("generates and applies an entire self-weight plan atomically against the original model", async () => {
    const model = await massModel(); const before = await canonicalJsonString(model); const hash = (await computeModelHash(model))!;
    const result = await generateSelfWeightPlan(model, { case_id: "load:invented-mass", label: "Invented mass", pipe_refs: [model.pipe_segments[0].id],
      gravity: { value: -9.8, unit: "m/s^2", axis: "global_y" }, provenance: "Invented test", source_model_hash: hash.value });
    expect(result.status, JSON.stringify(result.diagnostics)).toBe("ready");
    const batch = selfWeightPlanBatch(result.plan!, hash.value);
    expect(batch.operations).toHaveLength(2); expect(batch.operations[1].change.source_note).toBe(result.plan!.changes[1].source_note);
    const applied = await applyOperationBatch(model, batch, hash);
    expect(applied.validation.application_status, JSON.stringify(applied.diagnostics)).toBe("applied_to_session_model");
    expect(applied.applied_model!.load_cases.some(load => load.id === "load:invented-mass")).toBe(true);
    expect(await canonicalJsonString(model)).toBe(before);
    const badBatch = structuredClone(batch); badBatch.operations[1].change.after = "{}";
    const rejected = await applyOperationBatch(model, badBatch, hash);
    expect(rejected.validation.application_status).toBe("blocked"); expect(rejected.applied_model).toBeNull();
    expect(() => selfWeightPlanBatch(result.plan!, "sha256:stale")).toThrow("current model");
  });
  it("reviews a self-weight plan and queues one batch without changing the model", async () => {
    const model = await massModel(); const before = JSON.stringify(model); const queue = vi.fn();
    render(<SelfWeightPlanPanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={queue} />);
    fill(model); fireEvent.click(screen.getByRole("button", { name: "Generate self-weight plan" }));
    await waitFor(() => expect(screen.getByRole("button", { name: "Queue complete self-weight plan" })).toBeEnabled());
    expect(queue).not.toHaveBeenCalled(); fireEvent.click(screen.getByRole("button", { name: "Queue complete self-weight plan" }));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    expect(queue.mock.calls[0][0].operations).toHaveLength(2); expect(JSON.stringify(model)).toBe(before);
  });
  it("withdraws plans after clear, model, selection or busy changes", async () => {
    const model = await massModel(); const queue = vi.fn(); const selection = { type: "project" as const, id: model.project.id };
    const view = render(<SelfWeightPlanPanel model={model} selection={selection} onQueueBatch={queue} requestEpoch={0} />);
    fill(model); fireEvent.click(screen.getByRole("button", { name: "Generate self-weight plan" }));
    await waitFor(() => expect(screen.getByRole("button", { name: "Queue complete self-weight plan" })).toBeEnabled());
    view.rerender(<SelfWeightPlanPanel model={model} selection={selection} onQueueBatch={queue} requestEpoch={1} busy />);
    view.rerender(<SelfWeightPlanPanel model={model} selection={selection} onQueueBatch={queue} requestEpoch={1} />);
    expect(screen.queryByRole("button", { name: "Queue complete self-weight plan" })).toBeNull(); expect(queue).not.toHaveBeenCalled();
  });
  it("discards an asynchronous generation response after the user clears the draft", async () => {
    const model = await massModel(); const queue = vi.fn(); const selection = { type: "project" as const, id: model.project.id };
    let release!: (value: Awaited<ReturnType<typeof generateSelfWeightPlan>>) => void;
    const actual = selfWeightService.generateSelfWeightPlan;
    let generated!: Awaited<ReturnType<typeof generateSelfWeightPlan>>;
    const spy = vi.spyOn(selfWeightService, "generateSelfWeightPlan").mockImplementation(async (source, request) => {
      generated = await actual(source, request);
      return new Promise(resolve => { release = resolve; });
    });
    const view = render(<SelfWeightPlanPanel model={model} selection={selection} onQueueBatch={queue} requestEpoch={0} />);
    fill(model); fireEvent.click(screen.getByRole("button", { name: "Generate self-weight plan" }));
    await waitFor(() => expect(release).toBeTypeOf("function"));
    expect(screen.getByRole("group", { name: "Load case and gravity" })).toHaveAccessibleDescription("Preparing the self-weight plan or its review batch. Wait for it to finish.");
    view.rerender(<SelfWeightPlanPanel model={model} selection={selection} onQueueBatch={queue} requestEpoch={1} />);
    release(generated);
    await waitFor(() => expect(screen.getByRole("button", { name: "Generate self-weight plan" })).toBeEnabled());
    expect(screen.queryByRole("button", { name: "Queue complete self-weight plan" })).toBeNull();
    expect(queue).not.toHaveBeenCalled(); spy.mockRestore();
  });
  it("requires explicit gravity/selection and surfaces missing density", async () => {
    const model = await loadPreviewModel(); delete model.pipe_segments[0].section.material_density; const queue = vi.fn();
    render(<SelfWeightPlanPanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={queue} />);
    fireEvent.click(screen.getByRole("button", { name: "Generate self-weight plan" }));
    await waitFor(() => expect(screen.getByRole("alert")).toHaveTextContent("Enter the case ID"));
    fill(model); fireEvent.click(screen.getByRole("button", { name: "Generate self-weight plan" }));
    await waitFor(() => expect(screen.getByRole("alert")).toHaveTextContent("density")); expect(queue).not.toHaveBeenCalled();
  });
  it("retains untrusted offline metadata and flags exactly, including fields backend must reject", async () => {
    const model = await loadPreviewModel(); const queue = vi.fn();
    const payload = { batch_id: "batch:offline", source: { external: "unverified" }, operations: [
      { author_type: "agent", source: { source_role: "offline agent" }, rationale: "Exact rationale", audit_boundary: { requires_user_acceptance: false }, custom: "unchanged" }
    ] };
    expect(parseOfflineBatch(JSON.stringify(payload))).toEqual(payload);
    render(<OfflineProposalIntakePanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={queue} capabilityReference={{ source: "manager catalog" }} />);
    enter("Proposal batch JSON", JSON.stringify(payload)); fireEvent.click(screen.getByRole("button", { name: "Queue imported batch for review" }));
    expect(queue).toHaveBeenCalledWith(payload);
    expect(screen.getByRole("link", { name: "Download capability and schema reference" })).toHaveAttribute("download", "piping-operation-capabilities.json");
    const result = await applyOperationBatch(model, parseOfflineBatch(JSON.stringify(payload)), await computeModelHash(model));
    expect(result.validation.application_status).toBe("blocked"); expect(result.applied_model).toBeNull();
  });
  it("explains why an empty offline proposal cannot be queued", async () => {
    const model = await loadPreviewModel();
    render(<OfflineProposalIntakePanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={vi.fn()} />);
    const button = screen.getByRole("button", { name: "Queue imported batch for review" });
    expect(button).toBeDisabled();
    expect(button).toHaveAccessibleDescription("Paste a proposal batch or choose a proposal file before queueing it for review.");
    expect(button).toHaveAttribute("title", "Paste a proposal batch or choose a proposal file before queueing it for review.");
  });
  it("explains externally busy controls in each workflow", async () => {
    const model = await loadPreviewModel();
    const props = { model, selection: { type: "project" as const, id: model.project.id }, onQueueBatch: vi.fn(), busy: true };
    render(<><HangerSelectionPanel {...props} /><SelfWeightPlanPanel {...props} /><OfflineProposalIntakePanel {...props} /></>);
    for (const name of ["Local hanger library", "Support assignment", "Load case and gravity"]) {
      const group = screen.getByRole("group", { name });
      expect(group).toBeDisabled();
      expect(group).toHaveAccessibleDescription("Wait for the current operation to finish before changing this draft.");
    }
    expect(screen.getByRole("button", { name: "Queue imported batch for review" })).toHaveAccessibleDescription("Wait for the current operation to finish before editing or queueing a proposal.");
  });
  it("shows malformed and empty input errors without queueing", async () => {
    const model = await loadPreviewModel(); const queue = vi.fn();
    render(<OfflineProposalIntakePanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={queue} />);
    for (const text of ["{", "[]", '{"batch_id":"b","operations":[]}']) {
      enter("Proposal batch JSON", text); fireEvent.click(screen.getByRole("button", { name: "Queue imported batch for review" }));
      expect(screen.getByRole("alert")).toBeInTheDocument();
    }
    expect(queue).not.toHaveBeenCalled();
  });
});
