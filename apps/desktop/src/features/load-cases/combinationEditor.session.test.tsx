// RT-A4 on the desktop (DEL-05-02 Work `absolute-sum-srss-combinations.md` §2, §5.1; DEL-07-03):
// the three-axis static seismic template's four changes through the workspace session's one
// operation route (queued singly and applied in order, or moved to batch review and applied as
// one batch), Undo through the session checkpoint, and a saved and reopened model whose nested
// SRSS is edited again. The session runs the actual wasm engine on a model built in test code
// from the committed invented demo document.
import { act, cleanup, fireEvent, render, renderHook, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import * as previewService from "../../services/previewService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { applyModelOperation } from "../../services/operationService";
import { findLocalProject, openLocalProject, saveLocalProject } from "../../services/projectService";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { LoadCaseManagerPanel } from "./LoadCaseManagerPanel";
import { emptySeismicTemplateDraft, seismicTemplateIntents } from "./seismicTemplate";

const clone = <T,>(value: T): T => JSON.parse(JSON.stringify(value)) as T;

/** The demo document plus invented axis-state cases (copies of the base with one invented excitation each). */
function seismicModel(): PreviewModel {
  const model = clone(demoModel) as unknown as PreviewModel;
  model.components ??= [];
  model.diagnostics ??= [];
  const base = model.load_cases.find((loadCase) => loadCase.id === "load:L-100")!;
  for (const axis of ["X", "Y", "Z"] as const) {
    const state = clone(base);
    state.id = `load:EQ-${axis}`;
    state.label = `Invented base + ${axis} static excitation`;
    state.primitive_loads = [
      ...(state.primitive_loads ?? []).map((load) => ({ ...load, id: `${String(load.id)}-${axis}` })),
      { id: `load:EQ-${axis}-Q`, category: "occasional", target: { type: "element", pipe: "pipe:P-120" }, direction: `global_${axis.toLowerCase()}`,
        magnitude: { value: 40, unit: "N/m" }, dimension: "force_per_length", provenance: "invented_test_excitation" }
    ];
    model.load_cases.push(state);
  }
  return model;
}

function templateIntents(model: PreviewModel): EditorOperationIntent[] {
  const draft = emptySeismicTemplateDraft(model);
  return seismicTemplateIntents(model, { ...draft, baseId: "load:L-100", axisIds: { x: "load:EQ-X", y: "load:EQ-Y", z: "load:EQ-Z" }, basis: "srss" });
}

async function session(model: PreviewModel) {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(model);
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}
afterEach(() => { cleanup(); vi.restoreAllMocks(); });

const ids = (model: PreviewModel) => (model.combinations ?? []).map((combination) => combination.id);
const TEMPLATE_IDS = ["combination:SEISMIC-X-300", "combination:SEISMIC-Y-300", "combination:SEISMIC-Z-300", "combination:SEISMIC-300"];

describe("the seismic template through the session (queue, apply, Undo)", () => {
  it("applies the four changes singly in queue order; the SRSS applied first is refused by name; Undo restores each prior document exactly", async () => {
    const result = await session(seismicModel());
    const ops = () => result.current.operations;
    const hashes = [(await computeModelHash(result.current.model.model!))!.value];
    for (const intent of templateIntents(result.current.model.model!)) act(() => ops().handleQueueEditorIntent(intent));
    // The queue lists newest first; queue order is its reverse.
    const inOrder = () => [...ops().editorIntents].reverse();
    expect(inOrder().map((intent) => intent.target.ref)).toEqual(TEMPLATE_IDS);

    let applied = true;
    await act(async () => { applied = await ops().handleApplyIntent(inOrder()[3]); });
    expect(applied).toBe(false);
    expect(Object.values(ops().operationOutcomes).flatMap((outcome) => outcome.diagnostics.map((item) => item.code))).toContain("OP-COMBINATION-TERM-LOAD-NOT-FOUND");
    expect(ids(result.current.model.model!)).not.toContain("combination:SEISMIC-300");

    for (const ref of TEMPLATE_IDS) {
      const intent = inOrder().find((item) => item.target.ref === ref)!;
      await act(async () => { applied = await ops().handleApplyIntent(intent); });
      expect(applied).toBe(true);
      hashes.push((await computeModelHash(result.current.model.model!))!.value);
    }
    expect(ids(result.current.model.model!).slice(-4)).toEqual(TEMPLATE_IDS);
    expect(ops().undoStack).toHaveLength(4);
    for (const expected of [hashes[3], hashes[2], hashes[1], hashes[0]]) {
      act(() => ops().handleUndoSessionModelEdit());
      expect((await computeModelHash(result.current.model.model!))!.value).toBe(expected);
    }
    expect(ids(result.current.model.model!)).toEqual(["combination:C-OPER-ALT"]);
  });

  it("moved to batch review, the four apply as one batch in queue order, and one Undo restores the document", async () => {
    const result = await session(seismicModel());
    const ops = () => result.current.operations;
    const before = (await computeModelHash(result.current.model.model!))!.value;
    for (const intent of templateIntents(result.current.model.model!)) act(() => ops().handleQueueEditorIntent(intent));
    await act(async () => { await ops().handleReviewQueuedIntentsAsBatch(); });
    await waitFor(() => expect(ops().queuedBatches).toHaveLength(1));
    expect(ops().queuedBatches[0].batch.operations.map((intent) => intent.target.ref)).toEqual(TEMPLATE_IDS);
    await act(async () => { await ops().handleRunOperationBatch(ops().queuedBatches[0], true); });
    await waitFor(() => expect(ids(result.current.model.model!).slice(-4)).toEqual(TEMPLATE_IDS));
    const srss = result.current.model.model!.combinations!.find((item) => item.id === "combination:SEISMIC-300")!;
    expect(srss.terms).toEqual(TEMPLATE_IDS.slice(0, 3).map((id) => ({ combination: id, factor: 1 })));
    act(() => ops().handleUndoSessionModelEdit());
    expect((await computeModelHash(result.current.model.model!))!.value).toBe(before);
  });
});

describe("a saved and reopened model", () => {
  it("reopens with its nested SRSS shown as stored, applies a factor edit to a combination term, and saves and reopens it as applied", async () => {
    let edited = seismicModel();
    for (const intent of templateIntents(edited)) {
      const outcome = await applyModelOperation(edited, intent, await computeModelHash(edited));
      expect(outcome.validation.application_status).toBe("applied_to_session_model");
      edited = outcome.applied_model as PreviewModel;
    }
    edited.project.id = "project:rt-a4-combination-editor-reopen";
    const saved = await saveLocalProject(edited);
    expect(saved.model).toEqual(edited);
    const opened = (await openLocalProject("project:rt-a4-combination-editor-reopen"))!.model as PreviewModel;
    expect(await canonicalJsonString(opened)).toBe(await canonicalJsonString(edited));

    const list: EditorOperationIntent[] = [];
    render(<LoadCaseManagerPanel model={opened} onQueueIntent={(intent) => list.push(intent)} onSelect={() => undefined} selection={{ type: "combination", id: "combination:SEISMIC-300" }} />);
    expect(screen.getByTestId("load-manager-combination-combination:SEISMIC-300")).toHaveTextContent(
      "basis=srss; terms=combination combination:SEISMIC-X-300 x 1; combination combination:SEISMIC-Y-300 x 1; combination combination:SEISMIC-Z-300 x 1");
    expect(screen.getByTestId("load-manager-combination-resolved-combination:SEISMIC-Y-300")).toHaveTextContent("load:EQ-Y resolves to the load case; load:L-100 resolves to the load case");
    fireEvent.click(screen.getByTestId("load-manager-combination-term-combination:SEISMIC-300-1"));
    fireEvent.change(screen.getByTestId("load-manager-combination-factor-value"), { target: { value: "0.5" } });
    fireEvent.click(screen.getByTestId("queue-combination-factor-intent"));
    expect(list).toHaveLength(1);
    expect(list[0].change).toMatchObject({ field_path: "terms.1.factor", before: "1", after: "0.5" });
    const outcome = await applyModelOperation(opened, list[0], await computeModelHash(opened));
    expect(outcome.diagnostics.filter((item) => item.severity === "blocking")).toEqual([]);
    const current = outcome.applied_model as PreviewModel;

    const stored = await findLocalProject("project:rt-a4-combination-editor-reopen");
    await saveLocalProject(current, [], null, null, null, null, null, null, stored!.revision);
    const reopened = (await openLocalProject("project:rt-a4-combination-editor-reopen"))!.model as PreviewModel;
    expect(await canonicalJsonString(reopened)).toBe(await canonicalJsonString(current));
    expect(reopened.combinations!.find((item) => item.id === "combination:SEISMIC-300")!.terms).toEqual([
      { combination: "combination:SEISMIC-X-300", factor: 1 }, { combination: "combination:SEISMIC-Y-300", factor: 0.5 }, { combination: "combination:SEISMIC-Z-300", factor: 1 }
    ]);
    expect(reopened.combinations!.filter((item) => TEMPLATE_IDS.slice(0, 3).includes(item.id)).map((item) => [item.minuend_id, item.subtrahend_id])).toEqual([
      ["load:EQ-X", "load:L-100"], ["load:EQ-Y", "load:L-100"], ["load:EQ-Z", "load:L-100"]
    ]);
  });
});
