// New F/S rows do not consume entity numbers. Existing assignments and model
// records remain fixed; only never-queued draft pipe reservations can release.
import { act, cleanup, renderHook, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import fixtureJson from "../../../../../../fixtures/model_operations/run_sheet_authoring.json";
import referenceJson from "../../../../../../fixtures/reference_system/reference_system.json";
import { computeModelHash } from "../../../services/hashService";
import { applyOperationBatch } from "../../../services/operationBatchService";
import { createLocalProject, openLocalProject } from "../../../services/projectService";
import * as previewService from "../../../services/previewService";
import type { PreviewModel } from "../../../types";
import { useWorkspaceSession } from "../../workspace/workspaceSession";
import { initialRunSheetState, type RunSheetState } from "./runSheetLedger";
import { useRunSheetStore } from "./runSheetStore";
import { assignIds, buildDraftBatch, deleteDraftRow, insertRow, markSubmitted, recordShownPipeIds, releaseUnregistered, setCell, sheetView, type SheetColumnKey } from "./runSheetView";

const fixture = fixtureJson as unknown as { basis_model: PreviewModel; cases: Array<{ expected_applied_model: PreviewModel | null }> };
const basis = () => structuredClone(fixture.basis_model);
const source = "invented numbering regression";
afterEach(() => { cleanup(); vi.restoreAllMocks(); });
function start(model: PreviewModel, rowCount = 5): RunSheetState {
  const state = initialRunSheetState(model.project.id, "numbering", rowCount);
  return { ...state, header: { ...state.header, source, lengthUnit: "m", numbering: { ...state.header.numbering, pipeNumber: "start" } } };
}
const view = (state: RunSheetState, model: PreviewModel) => sheetView({ state, model, statuses: new Map(), conversions: new Map(), reservedIds: [] });
/** Follow the actual UI order: commit, assign once, then retain displayed ids. */
function write(state: RunSheetState, model: PreviewModel, key: string, column: SheetColumnKey, text: string): RunSheetState {
  let next = setCell(state, key, column, text);
  if (text.trim()) next = assignIds(next, key, model, []);
  return recordShownPipeIds(next, view(next, model));
}
function point(state: RunSheetState, model: PreviewModel): RunSheetState {
  for (const [column, text] of [["kind", "P"], ["dx", "30"], ["dy", "30"], ["od", "0.1143"], ["wall", "0.00602"], ["material", "material:steel"], ["orientation", "0 0 1"]] as const) state = write(state, model, "r1", column, text);
  return state;
}
function branch(state: RunSheetState, model: PreviewModel): RunSheetState {
  state = write(state, model, "r3", "kind", "F");
  state = write(state, model, "r3", "node", "node:N10");
  return write(state, model, "r4", "dy", "1");
}

describe("numbering only new entity rows", () => {
  it("matches reference node/pipe identities in natural row order, without typing pipe ids first", () => {
    // ID-only entry-order witness. Component/support/bend forms and solving
    // have separate evidence; no numerical result is claimed by this test.
    const model = basis(); model.nodes = []; model.pipe_segments = []; model.components = []; model.supports = [];
    let state = start(model, 26);
    state = { ...state, header: { ...state.header, numbering: { ...state.header.numbering, nodePrefix: "node:RS-", pipePrefix: "pipe:RS-P", pad: 3 } } };
    const steps: Array<Partial<Record<SheetColumnKey, string>>> = [
      { dz: "1", kind: "P" },
      { dx: "0.4", od: "0.1143", wall: "0.00602", material: "material:steel", orientation: "0 0 1" },
      { dx: "0.15", kind: "L" }, { dx: "0.6", kind: "V" }, { dx: "0.15", kind: "L" },
      { dx: "2.2" }, { kind: "S", support: "V", restraints: "UZ" },
      { dx: "2.5", kind: "B", nodeId: "node:RS-B1-CORNER" },
      { dz: "3", pipeId: "pipe:RS-P070" },
      { dz: "3", kind: "B", nodeId: "node:RS-B2-CORNER" },
      { dy: "3", pipeId: "pipe:RS-P090" }, { dy: "4" }, { dy: "4" }, { dy: "4" }, { dy: "3" }, { dy: "2" },
      { dy: "0.4", dz: "-0.025" }, { dy: "2.6" }, { dy: "2" }, { dy: "0.6" }, { dy: "2.4" }, { dy: "2" },
      { kind: "F", node: "node:RS-130" },
      { dz: "-2.5", nodeId: "node:RS-215", pipeId: "pipe:RS-P210" },
      { dz: "-2.5", kind: "B", nodeId: "node:RS-B3-CORNER", pipeId: "pipe:RS-P220" },
      { dx: "2", nodeId: "node:RS-230", pipeId: "pipe:RS-P230" },
    ];
    let typedIds = 0;
    steps.forEach((step, index) => {
      for (const [column, text] of Object.entries(step)) {
        // Object order is the script's ordinary entry order: geometry before
        // overrides, and the S row stays in place instead of being added last.
        state = write(state, model, `r${index + 1}`, column as SheetColumnKey, text);
        if (column === "nodeId" || column === "pipeId") typedIds += 1;
      }
    });
    const rows = view(state, model).rows.map((row) => row.resolved);
    const reference = referenceJson as unknown as PreviewModel;
    const arcs = new Set(["pipe:RS-P065", "pipe:RS-P085", "pipe:RS-P225"]);
    expect(rows.filter((row) => row.ids.pipe).map((row) => row.ids.pipe).sort()).toEqual(reference.pipe_segments.filter((pipe) => !arcs.has(pipe.id)).map((pipe) => pipe.id).sort());
    for (const row of rows) if (row.end && row.kind !== "bend") expect(reference.nodes.some((node) => node.id === row.end!.id)).toBe(true);
    expect(state.idState.assigned.r7).toBeUndefined();
    expect(state.idState.assigned.r23).toBeUndefined();
    expect(typedIds).toBe(10);
    expect(rows.flatMap((row) => row.refusals).filter((item) => item.rule === "ids")).toEqual([]);
  });

  it("leaves new F/S rows unnumbered without advancing the counter", () => {
    const model = basis(); let state = point(start(model), model);
    for (const [key, column, text] of [["r2", "kind", "S"], ["r2", "support", "V"], ["r2", "restraints", "UZ"], ["r3", "dx", "1"], ["r4", "kind", "F"], ["r4", "node", "node:N20"], ["r5", "dy", "1"]] as const) state = write(state, model, key, column, text);
    expect(state.idState.assigned).toEqual({ r1: 10, r3: 20, r5: 30 });
    expect(state.idState.next).toBe(40);
    expect(view(state, model).rows[2].resolved.ids.pipe).toBe("pipe:P10");
    expect(view(state, model).rows[4].resolved.ids.pipe).toBe("pipe:P20");
    expect(view(state, model).queueBlocked).toBeNull();
  });

  it("preserves existing numbered rows through inserted F/S rows and explicit kind changes", () => {
    const model = basis(); let state = point(start(model, 3), model);
    state = write(state, model, "r2", "dx", "1");
    state = write(state, model, "r3", "dy", "1");
    const before = view(state, model).rows.map((row) => [row.sheetRow.key, row.resolved.ids]);
    const assigned = { ...state.idState.assigned }; const counter = state.idState.next;
    const inserted = insertRow(state, 2); state = inserted.state;
    state = write(state, model, inserted.key, "kind", "F");
    state = write(state, model, inserted.key, "node", "node:N20");
    expect(state.idState.assigned).toEqual(assigned); expect(state.idState.next).toBe(counter);
    for (const [key, ids] of before) expect(view(state, model).rows.find((row) => row.sheetRow.key === key)!.resolved.ids).toEqual(ids);
    // A row that already displayed numbered identities keeps its assignment.
    state = write(state, model, "r2", "kind", "S");
    expect(state.idState.assigned.r2).toBe(20);
    expect(state.idState.next).toBe(counter);
  });

  it("reopens legacy records unchanged, then creates new rows without renumbering any old record", async () => {
    const legacy = structuredClone(fixture.cases.find((item) => item.expected_applied_model)!.expected_applied_model!);
    await createLocalProject(legacy);
    const opened = (await openLocalProject(legacy.project.id))!.model;
    expect(opened).toEqual(legacy);
    let state = start(opened, 2);
    const end = opened.nodes.at(-1)!;
    state = write(state, opened, "r1", "kind", "F");
    state = write(state, opened, "r1", "node", end.id);
    for (const [column, text] of [["dy", "1"], ["od", "0.1143"], ["wall", "0.00602"], ["material", "material:steel"], ["orientation", "0 0 1"]] as const) state = write(state, opened, "r2", column, text);
    const shown = view(state, opened); expect(shown.queueBlocked).toBeNull();
    const build = await buildDraftBatch(shown, opened);
    const applied = await applyOperationBatch(opened, build.batch, await computeModelHash(opened));
    expect(applied.validation.batch_validation_status, JSON.stringify(applied.diagnostics)).toBe("passed");
    for (const record of legacy.nodes) expect(applied.applied_model!.nodes.find((node) => node.id === record.id)).toEqual(record);
    for (const record of legacy.pipe_segments) expect(applied.applied_model!.pipe_segments.find((pipe) => pipe.id === record.id)).toEqual(record);
    expect(opened).toEqual(legacy);
  });
});

describe("releasing only draft pipe reservations", () => {
  it("releases an overwritten unqueued id so a later branch can use it in natural entry order", () => {
    const model = basis(); let state = point(start(model, 4), model);
    state = write(state, model, "r2", "dx", "1");
    expect(state.idState.pipes!.r2.id).toBe("pipe:P10");
    state = write(state, model, "r2", "pipeId", "pipe:explicit");
    expect(state.idState.pipes!.r2).toBeUndefined();
    state = branch(state, model);
    expect(view(state, model).rows[3].resolved.ids.pipe).toBe("pipe:P10");
    expect(view(state, model).queueBlocked).toBeNull();
  });

  it("releases a deleted unqueued id but never reuses the row's consumed number", () => {
    const model = basis(); let state = point(start(model, 4), model);
    state = write(state, model, "r2", "dx", "1");
    state = deleteDraftRow(state, "r2");
    expect(state.idState.pipes!.r2).toBeUndefined();
    state = branch(state, model);
    expect(state.idState.assigned.r4).toBe(30);
    expect(view(state, model).rows.find((row) => row.sheetRow.key === "r4")!.resolved.ids.pipe).toBe("pipe:P10");
    expect(view(state, model).queueBlocked).toBeNull();
  });

  it("a prepared batch that was never registered does not make its id permanent", async () => {
    const model = basis(); let state = point(start(model, 4), model);
    state = write(state, model, "r2", "dx", "1");
    const shown = view(state, model); const build = await buildDraftBatch(shown, model);
    state = releaseUnregistered(markSubmitted(state, shown, build), build.batch.batch_id);
    expect(state.idState.retainedPipes ?? {}).toEqual({});
    state = write(state, model, "r2", "pipeId", "pipe:explicit");
    state = branch(state, model);
    expect(view(state, model).rows[3].resolved.ids.pipe).toBe("pipe:P10");
  });

  it.each(["withdraw", "apply-undo"] as const)("keeps a successfully registered id reserved after %s, overwrite and deletion", async (mode) => {
    vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(basis());
    const hook = renderHook(() => {
      const session = useWorkspaceSession();
      const store = useRunSheetStore(session.operations.sessionEvents, session.model.activeModelIndex, session.model.model?.project.id ?? null);
      return { session, store };
    });
    await waitFor(() => expect(hook.result.current.session.model.model).not.toBeNull());
    const model = hook.result.current.session.model.model!;
    let state = point(start(model, 4), model); state = write(state, model, "r2", "dx", "1");
    const shown = view(state, model); const build = await buildDraftBatch(shown, model);
    act(() => hook.result.current.store.update(() => markSubmitted(state, shown, build)));
    await act(async () => { await hook.result.current.session.operations.registerOperationBatch(build.batch, { reveal: false }); });
    expect(hook.result.current.store.current().idState.retainedPipes).toMatchObject({ "pipe:P10": "r2" });
    if (mode === "withdraw") act(() => hook.result.current.session.operations.handleClearReviewQueue());
    else {
      await act(async () => { await hook.result.current.session.operations.handleRunOperationBatch(hook.result.current.session.operations.queuedBatches[0], true); });
      expect(hook.result.current.session.operations.undoStack).toHaveLength(1);
      act(() => hook.result.current.session.operations.handleUndoSessionModelEdit());
    }
    expect(hook.result.current.session.model.model).toEqual(model);
    state = write(hook.result.current.store.current(), model, "r2", "pipeId", "pipe:explicit");
    state = deleteDraftRow(state, "r2"); state = branch(state, model);
    expect(state.idState.retainedPipes).toMatchObject({ "pipe:P10": "r2" });
    expect(view(state, model).rows.find((row) => row.sheetRow.key === "r4")!.resolved.ids.pipe).not.toBe("pipe:P10");
    state = write(state, model, "r4", "pipeId", "pipe:P10");
    expect(view(state, model).rows.find((row) => row.sheetRow.key === "r4")!.refusals.some((item) => item.message.includes("pipe:P10"))).toBe(true);
  });
});
