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
import { computeModelHash, canonicalJsonString, canonicalSha256Hex } from "../../services/hashService";
import type { PreviewModel } from "../../types";
import { chooseVirtualMultiTarget } from "../../test-support/workspaceTestControls";
import { reconnectModel } from "../../test-support/reconnectModel";
import { clearReconnectExampleCache, operationExamples, reconnectOperationExample, RECONNECT_EXAMPLE_CANDIDATE_CAP } from "../geometry-tools/reconnectExample";
import { validateOperationBatch } from "../../services/operationBatchService";
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
  enter("Self-weight provenance", "Invented test only"); chooseVirtualMultiTarget("self-weight-pipes", model.pipe_segments[0].id);
}
describe("bounded workflow actual engine tests", () => {
  it("replaces old support stiffness, records the library reference and provenance record only, and rejects stale before state", async () => {
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
    // W-17 §2.6: the short reference only; no library metadata or record snapshot.
    expect(after.provenance).toBe(`library:hanger:${doc.hanger_library.library_id}#${doc.hanger_records[0].hanger_id}`);
    expect(after.provenance).toBe(batch.operations[0].change.source_note);
    expect(batch.operations[1].change.field_path).toBe("provenance_record");
    const result = await applyOperationBatch(model, batch, await computeModelHash(model));
    expect(result.validation.application_status, JSON.stringify(result.diagnostics)).toBe("applied_to_session_model");
    expect(result.applied_model!.supports[0].stiffness).toBeUndefined();
    expect(result.applied_model!.supports[0].hanger!.stiffness).toEqual({ dof: "UY", value: { value: 123, unit: "N/m" } });
    const record = result.applied_model!.supports[0].provenance_record!;
    expect(record.library_ref).toEqual({ library_kind: "hanger", library_id: doc.hanger_library.library_id,
      library_version: `sha256:${await canonicalSha256Hex(doc.hanger_records[0])}`, record_id: doc.hanger_records[0].hanger_id });
    expect(record).toMatchObject({ source_name: doc.hanger_records[0].provenance.source_name, redistribution_status: "private_only", review_status: "pending" });
    const saved = JSON.stringify(result.applied_model);
    expect(saved).not.toContain("record_snapshot"); expect(saved).not.toContain("library_metadata"); expect(saved).not.toContain(doc.hanger_library.name);
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
  it("publishes an executable reconnect example that passes actual offline intake and validates", async () => {
    clearReconnectExampleCache();
    const model = reconnectModel(); const queue = vi.fn();
    const loader = vi.fn(() => operationExamples(model));
    render(<OfflineProposalIntakePanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={queue}
      capabilityReference={{ source: "manager catalog" }} loadOperationExamples={loader} />);
    expect(loader).not.toHaveBeenCalled(); // on demand, never on render
    const details = screen.getByText("Offline authoring reference").closest("details")!;
    details.open = true; fireEvent(details, new Event("toggle"));
    await waitFor(() => expect(screen.getByTestId("capability-reference-json")).toHaveTextContent("reconnect_pipe_run"));
    const reference = JSON.parse(screen.getByTestId("capability-reference-json").textContent!);
    const example = reference.operation_examples.reconnect_pipe_run;
    expect(example.wire).toMatchObject({ change_kind: "reconnect_pipe_run", field_path: "pipe_segments", unit: "none", dimension: "dimensionless" });
    expect(JSON.parse(example.batch.operations[0].change.after)).toEqual({ pipe_ref: "pipe:P", end: "to", target_node_ref: "node:C" });
    enter("Proposal batch JSON", JSON.stringify(example.batch)); fireEvent.click(screen.getByRole("button", { name: "Queue imported batch for review" }));
    expect(queue).toHaveBeenCalledWith(example.batch);
    const outcome = await validateOperationBatch(model, queue.mock.calls[0][0], await computeModelHash(model));
    expect(outcome.validation.batch_validation_status).toBe("passed");
    // Cached against the model hash: the same promise result, no second search.
    expect(await reconnectOperationExample(model)).toEqual(example);
  });
  it("skips a candidate with an unsupported element-load shape (R13) and omits the example with its reason when none passes", async () => {
    clearReconnectExampleCache();
    const model = reconnectModel();
    model.load_cases[0].primitive_loads = [{ id: "load:extent", category: "distributed_force", target: { type: "element", pipe: "pipe:P" },
      direction: "global_z", magnitude: { value: -1, unit: "N/m" }, dimension: "force_per_length", provenance: "invented", extent: { start_fraction: 0.2 } }];
    const example = await reconnectOperationExample(model);
    expect("batch" in example && example.batch.operations[0].target.ref).toBe("pipe:Q");
    const outcome = await validateOperationBatch(model, (example as { batch: Parameters<typeof validateOperationBatch>[1] }).batch, await computeModelHash(model));
    expect(outcome.validation.batch_validation_status).toBe("passed");
    // The bundled demo: every pipe is named by or ends at a component, so no
    // candidate passes; with three loose nodes the search reaches its cap.
    const demo = await loadPreviewModel();
    const none = await reconnectOperationExample(demo);
    expect(none).toMatchObject({ omitted_reason: "no valid reconnect found among the 24 candidates of this model", candidates_checked: 24 });
    for (const id of ["node:X-1", "node:X-2", "node:X-3"]) demo.nodes.push({ ...demo.nodes[0], id, label: id });
    const capped = await reconnectOperationExample(demo);
    expect(capped).toMatchObject({ omitted_reason: `no valid reconnect found in the first ${RECONNECT_EXAMPLE_CANDIDATE_CAP} candidates`, candidates_checked: 32 });
  });
});
