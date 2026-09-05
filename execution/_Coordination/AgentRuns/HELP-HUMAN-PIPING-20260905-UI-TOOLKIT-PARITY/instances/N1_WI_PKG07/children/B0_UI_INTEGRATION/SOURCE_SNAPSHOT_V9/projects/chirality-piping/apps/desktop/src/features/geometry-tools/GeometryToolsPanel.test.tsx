import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { loadPreviewModel } from "../../services/previewService";
import * as hashing from "../../services/hashService";
import { applyOperationBatch, validateOperationBatch } from "../../services/operationBatchService";
import { GeometryToolsPanel } from "./GeometryToolsPanel";
import { buildGeometryBatch, emptyGeometryDraft, type GeometryDraft } from "./geometryDraft";
import type { PreviewModel } from "../../types";
afterEach(() => { cleanup(); vi.restoreAllMocks(); });
const enter = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
async function bareModel(): Promise<PreviewModel> {
  const model = await loadPreviewModel();
  const pipe = model.pipe_segments[0];
  model.pipe_segments = [pipe];
  model.nodes = model.nodes.filter(n => [pipe.from, pipe.to].includes(n.id));
  model.supports = []; model.components = []; model.load_cases = []; model.combinations = [];
  pipe.y_reference = { x: 0, y: 1, z: 0 };
  model.nodes[0].position = { x: 0, y: 0, z: 0 };
  model.nodes[1].position = { x: 4000, y: 0, z: 0 };
  model.project.units.length = "mm";
  return model;
}
function splitDraft(model: PreviewModel): GeometryDraft {
  return { ...emptyGeometryDraft(), action: "split", pipeRefs: [model.pipe_segments[0].id], fraction: "0.25",
    newNode: { id: "node:split-test", label: "Inserted test", provenance: "invented UI test" },
    newPipe: { id: "pipe:split-test", label: "Second half", provenance: "invented UI test" } };
}
function fillSplit(model: PreviewModel) {
  enter("Geometry tool", "split");
  fireEvent.click(screen.getByRole("checkbox"));
  enter("Split fraction", "0.25");
  for (const [kind, id] of [["node", "node:split-test"], ["pipe", "pipe:split-test"]]) {
    enter(`New ${kind} ID`, id); enter(`New ${kind} label`, "Invented test"); enter(`New ${kind} provenance`, "invented UI test");
  }
  expect(model.pipe_segments).toHaveLength(1);
}
describe("geometry tools exact engine batches", () => {
  it("makes pristine and edited clear observable without queueing", async () => {
    const model = await bareModel(); const onQueueBatch = vi.fn();
    render(<GeometryToolsPanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={onQueueBatch} />);
    fireEvent.click(screen.getByRole("button", { name: "Clear / cancel geometry draft" }));
    expect(screen.getByText("Geometry draft cleared. No model changes applied.")).toBeInTheDocument();
    fillSplit(model);
    fireEvent.click(screen.getByRole("button", { name: "Clear / cancel geometry draft" }));
    expect(screen.getByLabelText("Geometry tool")).toHaveValue("");
    expect(screen.getByText("Geometry draft cleared. No model changes applied.")).toBeInTheDocument();
    expect(onQueueBatch).not.toHaveBeenCalled();
  });
  it("queues a human split and the actual engine previews without publication then applies its exact geometry", async () => {
    const model = await bareModel(); const original = JSON.stringify(model); const onQueueBatch = vi.fn();
    render(<GeometryToolsPanel model={model} selection={{ type: "pipe", id: model.pipe_segments[0].id }} onQueueBatch={onQueueBatch} />);
    expect(screen.getByRole("button", { name: "Queue geometry batch" })).toBeDisabled();
    fillSplit(model); fireEvent.click(screen.getByRole("button", { name: "Queue geometry batch" }));
    await waitFor(() => expect(onQueueBatch).toHaveBeenCalledOnce());
    const batch = onQueueBatch.mock.calls[0][0];
    expect(batch.operations).toHaveLength(1);
    expect(batch.operations[0].change.before).toBe(await hashing.canonicalJsonString(model.pipe_segments[0]));
    const hash = await hashing.computeModelHash(model);
    const preview = await validateOperationBatch(model, batch, hash);
    expect(preview.validation.batch_validation_status, JSON.stringify(preview.diagnostics)).toBe("passed");
    expect(preview.applied_model).toBeNull();
    const outcome = await applyOperationBatch(model, batch, hash);
    expect(outcome.validation.application_status, JSON.stringify(outcome)).toBe("applied_to_session_model");
    expect(outcome.applied_model!.nodes.find(n => n.id === "node:split-test")!.position).toEqual({ x: 1000, y: 0, z: 0 });
    expect(outcome.applied_model!.pipe_segments).toHaveLength(2);
    expect(JSON.stringify(model)).toBe(original);
  });
  it.each(["translate", "rotate", "mirror"] as const)("applies exact %s copy maps through the engine without altering originals", async action => {
    const model = await bareModel(); const original = JSON.stringify(model);
    const draft: GeometryDraft = { ...emptyGeometryDraft(), action, pipeRefs: [model.pipe_segments[0].id], mode: "copy", coordinateSystem: "global",
      vector: { x: action === "translate" ? "1" : "0", y: "0", z: "0", unit: "m" },
      direction: action === "mirror" ? { x: "1", y: "0", z: "0" } : { x: "0", y: "0", z: "1" }, angle: { value: "90", unit: "deg" },
      copyNodes: model.nodes.map((n, index) => ({ source_ref: n.id, id: `node:copy-${index}`, label: `Test ${index}`, provenance: "invented UI test" })),
      copyPipes: [{ source_ref: model.pipe_segments[0].id, id: "pipe:copy", label: "Test copy", provenance: "invented UI test" }] };
    const batch = await buildGeometryBatch(model, draft, `batch:test:${action}`);
    const result = await applyOperationBatch(model, batch, await hashing.computeModelHash(model));
    expect(result.validation.application_status, JSON.stringify(result)).toBe("applied_to_session_model");
    const copied = result.applied_model!.nodes.find(n => n.id === "node:copy-1")!.position;
    expect(copied.x).toBeCloseTo(action === "translate" ? 5000 : action === "mirror" ? -4000 : 0);
    expect(copied.y).toBeCloseTo(action === "rotate" ? 4000 : 0);
    expect(result.applied_model!.pipe_segments[0]).toEqual(model.pipe_segments[0]);
    expect(JSON.stringify(model)).toBe(original);
  });
  it("requires every explicit copy identity in the human transform form", async () => {
    const model = await bareModel(); const onQueueBatch = vi.fn();
    render(<GeometryToolsPanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={onQueueBatch} />);
    enter("Geometry tool", "translate"); fireEvent.click(screen.getByRole("checkbox"));
    enter("Transform mode", "copy"); enter("Geometry coordinate system", "global");
    enter("Geometry X", "1"); enter("Geometry Y", "0"); enter("Geometry Z", "0"); enter("Geometry length unit", "m");
    expect(screen.getByRole("button", { name: "Queue geometry batch" })).toBeDisabled();
    for (const [index, node] of model.nodes.entries()) {
      enter(`Copied node ${node.id} ID`, `node:ui-copy-${index}`);
      enter(`Copied node ${node.id} label`, `Test node ${index}`);
      enter(`Copied node ${node.id} provenance`, "invented UI test");
    }
    const pipeId = model.pipe_segments[0].id;
    enter(`Copied pipe ${pipeId} ID`, "pipe:ui-copy");
    enter(`Copied pipe ${pipeId} label`, "Test copied run");
    enter(`Copied pipe ${pipeId} provenance`, "invented UI test");
    fireEvent.click(screen.getByRole("button", { name: "Queue geometry batch" }));
    await waitFor(() => expect(onQueueBatch).toHaveBeenCalledOnce());
    const batch = onQueueBatch.mock.calls[0][0];
    const after = JSON.parse(batch.operations[0].change.after);
    expect(after.copy_nodes.map((row: { id: string }) => row.id)).toEqual(["node:ui-copy-0", "node:ui-copy-1"]);
    const result = await applyOperationBatch(model, batch, await hashing.computeModelHash(model));
    expect(result.validation.application_status, JSON.stringify(result)).toBe("applied_to_session_model");
  });
  it("blocks endpoint components and invalid copy mappings without publishing any model", async () => {
    const model = await bareModel();
    model.components = [{ id: "component:test", label: "Test", kind: "valve", node: model.pipe_segments[0].from, provenance: "invented" }];
    const result = await applyOperationBatch(model, await buildGeometryBatch(model, splitDraft(model), "batch:blocked"), await hashing.computeModelHash(model));
    expect(result.validation.application_status).toBe("blocked"); expect(result.applied_model).toBeNull();
    const draft = { ...splitDraft(model), action: "translate", mode: "copy", coordinateSystem: "global", vector: { x: "0", y: "0", z: "1", unit: "m" } } as GeometryDraft;
    await expect(buildGeometryBatch(model, draft, "batch:missing-map")).rejects.toThrow(/mapping/);
  });
  it.each(["clear", "model", "selection", "epoch", "busy"])("withdraws pending canonical preparation on %s", async change => {
    const model = await bareModel(); const onQueueBatch = vi.fn();
    const originalCanonical = await hashing.canonicalJsonString(model.pipe_segments[0]);
    let release!: (text: string) => void;
    vi.spyOn(hashing, "canonicalJsonString").mockImplementation(() => new Promise(resolve => { release = resolve; }));
    const props = { model, selection: { type: "pipe" as const, id: model.pipe_segments[0].id }, onQueueBatch, requestEpoch: 0, busy: false };
    const view = render(<GeometryToolsPanel {...props} />); fillSplit(model);
    fireEvent.click(screen.getByRole("button", { name: "Queue geometry batch" }));
    if (change === "clear") fireEvent.click(screen.getByRole("button", { name: "Clear / cancel geometry draft" }));
    if (change === "model") view.rerender(<GeometryToolsPanel {...props} model={structuredClone(model)} />);
    if (change === "selection") view.rerender(<GeometryToolsPanel {...props} selection={{ type: "node", id: model.nodes[0].id }} />);
    if (change === "epoch") view.rerender(<GeometryToolsPanel {...props} requestEpoch={1} />);
    if (change === "busy") view.rerender(<GeometryToolsPanel {...props} busy />);
    release(originalCanonical);
    await waitFor(() => expect(screen.getByText(/Preparation cancelled/)).toBeInTheDocument());
    expect(onQueueBatch).not.toHaveBeenCalled();
  });
});
