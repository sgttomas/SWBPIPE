/** Run sheet S2 prerequisite (DEL-07-02 Design `run-sheet-entry.md` §3.5, §7 S2,
 * O6): a batch or intent containing `insert_bend_at_corner` is applied only after
 * a clean preview of that exact content on the current model, whatever its
 * producer: a run-sheet batch with a bend, the bend form's intent (routed from
 * the single-intent apply path to batch review), and a requeued single-intent
 * batch as the stored review context builds it (`App.tsx` "Requeue for fresh
 * validation"). The session runs the actual wasm engine on the invented S1
 * fixture model. */
import { act, fireEvent, render, renderHook, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import fixtureJson from "../../../../../fixtures/model_operations/run_sheet_authoring.json";
import * as previewService from "../../services/previewService";
import { computeModelHash } from "../../services/hashService";
import type { OperationBatch } from "../../services/operationBatchService";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { InsertBendAtCornerForm } from "../bend-authoring/BendAuthoringForms";
import { useWorkspaceSession } from "./workspaceSession";

type Fixture = { basis_model: PreviewModel; cases: Array<{ id: string; expected_batch: OperationBatch }> };
const fixture = fixtureJson as unknown as Fixture;
/** The fixture basis with pipe:CD given pipe:BC's section and material, so corner
 * node:C (a 90 degree turn) admits a bend; the sheet batch below never reads either pipe. */
function basis(): PreviewModel {
  const model = structuredClone(fixture.basis_model);
  const bc = model.pipe_segments.find((pipe) => pipe.id === "pipe:BC")!;
  const cd = model.pipe_segments.find((pipe) => pipe.id === "pipe:CD")!;
  cd.section = structuredClone(bc.section);
  cd.material = bc.material;
  return model;
}
const sheetBatch = (): OperationBatch => structuredClone(fixture.cases.find((item) => item.id === "two-bend-chain")!.expected_batch);

async function session() {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(basis());
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}
afterEach(() => { vi.restoreAllMocks(); });

/** The bend form's own intent at corner node:C (pipes BC and CD), as the form queues it. */
async function bendFormIntent(model: PreviewModel, radius = "0.5"): Promise<EditorOperationIntent> {
  const queue = vi.fn();
  const view = render(<InsertBendAtCornerForm model={model} node="node:C" selection={{ type: "node", id: "node:C" }} onQueueIntent={queue} />);
  const change = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
  change("Bend radius R value", radius);
  change("Corner bend flexibility factor k", "1.5");
  change("Corner bend source of k", "invented k (user entry)");
  change("Geometry source reference", "invented geometry source");
  change("Corner bend provenance", "invented bend provenance");
  fireEvent.click(screen.getByRole("button", { name: "Queue insert bend at corner" }));
  await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  view.unmount();
  return queue.mock.calls[0][0] as EditorOperationIntent;
}

type Session = Awaited<ReturnType<typeof session>>;
/** Apply with no preview: refused with the reason naming the bend, model and history unchanged. */
async function expectRefusedWithoutPreview(result: Session, index: number) {
  const before = structuredClone(result.current.model.model!);
  const beforeHash = (await computeModelHash(before))!.value;
  const entry = result.current.operations.queuedBatches[index];
  expect(entry.requiresPreview).toBe(true);
  await act(async () => { await result.current.operations.handleRunOperationBatch(entry, true); });
  expect(result.current.operations.batchMessage).toContain(`Batch ${entry.batch.batch_id} was not applied.`);
  expect(result.current.operations.batchMessage).toContain("insert_bend_at_corner");
  expect(result.current.model.model).toEqual(before);
  expect((await computeModelHash(result.current.model.model!))!.value).toBe(beforeHash);
  expect(result.current.operations.undoStack).toHaveLength(0);
  expect(result.current.operations.queuedBatches[index].key).toBe(entry.key);
}
/** A clean preview of that exact content, then Apply: one checkpoint, the corner replaced by the bend. */
async function expectAppliesAfterPreview(result: Session, index: number, corner: string) {
  const entry = result.current.operations.queuedBatches[index];
  await act(async () => { await result.current.operations.handleRunOperationBatch(entry, false); });
  expect(result.current.operations.batchOutcomes[entry.key].validation.batch_validation_status).toBe("passed");
  await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[index], true); });
  expect(result.current.operations.undoStack).toHaveLength(1);
  expect(result.current.operations.undoStack[0].operation_id).toBe(entry.batch.batch_id);
  expect(result.current.model.model!.nodes.some((node) => node.id === corner)).toBe(false);
  expect(result.current.model.model!.components.some((component) => component.kind === "bend")).toBe(true);
}

describe("bend preview gate (O6): every producer of insert_bend_at_corner", () => {
  it("a run-sheet batch with a bend is refused without preview and applies after a clean preview", async () => {
    const result = await session();
    await act(async () => { await result.current.operations.handleQueueOperationBatch(sheetBatch()); });
    await expectRefusedWithoutPreview(result, 0);
    await expectAppliesAfterPreview(result, 0, "node:N20");
  });

  it("the bend form's intent is routed from the single-intent apply path to batch review, refused there without preview, and applies after one", async () => {
    const result = await session();
    const model = structuredClone(result.current.model.model!);
    const intent = await bendFormIntent(model);
    expect(intent.change.change_kind).toBe("insert_bend_at_corner");
    let applied = true;
    await act(async () => { applied = await result.current.operations.handleApplyIntent(intent); });
    expect(applied).toBe(false);
    expect(result.current.model.model).toEqual(model);
    expect(result.current.operations.undoStack).toHaveLength(0);
    expect(result.current.operations.operationMessage).toContain("insert_bend_at_corner");
    expect(result.current.operations.queuedBatches).toHaveLength(1);
    expect(result.current.operations.queuedBatches[0].batch.operations).toEqual([intent]);
    await expectRefusedWithoutPreview(result, 0);
    await expectAppliesAfterPreview(result, 0, "node:C");
  });

  it("a requeued single-intent bend batch is gated as the stored review context builds it", async () => {
    const result = await session();
    const intent = await bendFormIntent(structuredClone(result.current.model.model!));
    await act(async () => { await result.current.operations.handleQueueOperationBatch({ batch_id: "requeue-0", operations: [structuredClone(intent)] }); });
    await expectRefusedWithoutPreview(result, 0);
    await expectAppliesAfterPreview(result, 0, "node:C");
  });

  it("an edited batch needs a new preview: a clean preview of one content does not admit another", async () => {
    const result = await session();
    const model = structuredClone(result.current.model.model!);
    const first = await bendFormIntent(model, "0.5");
    const edited = await bendFormIntent(model, "0.4");
    await act(async () => { await result.current.operations.handleQueueOperationBatch({ batch_id: "batch:bend", operations: [first] }); });
    const previewed = result.current.operations.queuedBatches[0];
    await act(async () => { await result.current.operations.handleRunOperationBatch(previewed, false); });
    expect(result.current.operations.batchOutcomes[previewed.key].validation.batch_validation_status).toBe("passed");
    // The edit is queued again under the same batch id: a new registration with no preview of its own.
    await act(async () => { await result.current.operations.handleQueueOperationBatch({ batch_id: "batch:bend", operations: [edited] }); });
    await expectRefusedWithoutPreview(result, 1);
    // The previewed registration cannot carry the edited content either: the run reads the registered batch.
    const forged = { ...structuredClone(previewed), batch: { batch_id: "batch:bend", operations: [structuredClone(edited)] }, requiresPreview: false };
    await act(async () => { await result.current.operations.handleRunOperationBatch(forged as typeof previewed, true); });
    const arc = result.current.model.model!.pipe_segments.find((pipe) => pipe.id === "pipe:C-bend");
    expect(arc).toBeDefined();
    const component = result.current.model.model!.components.find((item) => item.id === "component:C-bend")!;
    expect(JSON.stringify(component)).toContain("0.5");
    expect(result.current.operations.undoStack).toHaveLength(1);
  });

});
