// Run sheet S2 (DEL-07-02 Design `run-sheet-entry.md` §3.6, §7 S2) against the
// workspace session itself, with the sheet's store listening on the session's
// event channel and the actual wasm engine: injected failures in Prepare and
// Notify for apply, Undo and Redo; the apply order and the retirement of stale
// registrations; withdrawal; abandonment; Open State's history reset; and the
// ledger's transitions. Invented S1 fixture model.
import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import fixtureJson from "../../../../../../fixtures/model_operations/run_sheet_authoring.json";
import { computeModelHash } from "../../../services/hashService";
import * as operationBatchService from "../../../services/operationBatchService";
import * as previewService from "../../../services/previewService";
import type { PreviewModel } from "../../../types";
import { buildReconnectBatch, emptyGeometryDraft } from "../../geometry-tools/geometryDraft";
import { buildRouteSubmission, type FrozenDraftReview, type RouteDraft } from "../../viewport/routeDraft";
import { buildGridOperationIntent, type GridColumn } from "../../workspace/table/modelTableAdapter";
import * as sessionModel from "../../workspace/sessionModel";
import type { SessionEvent } from "../../workspace/sessionEvents";
import { RETIRED_BATCH_NOTICE, useWorkspaceSession } from "../../workspace/workspaceSession";
import * as ledger from "./runSheetLedger";
import { initialRunSheetState, type RunSheetState } from "./runSheetLedger";
import { useRunSheetStore } from "./runSheetStore";
import { assignIds, buildDraftBatch, markSubmitted, setCell, sheetView, type SheetColumnKey } from "./runSheetView";

const fixture = fixtureJson as unknown as { basis_model: PreviewModel };
const SOURCE = "invented run-sheet source";
afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

async function harness() {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(structuredClone(fixture.basis_model));
  const events: string[] = [];
  const hook = renderHook(() => {
    const session = useWorkspaceSession();
    const store = useRunSheetStore(session.operations.sessionEvents, session.model.activeModelIndex, session.model.model?.project.id ?? null);
    return { session, store };
  });
  await waitFor(() => expect(hook.result.current.session.model.model).not.toBeNull());
  hook.result.current.session.operations.sessionEvents.listen((event: SessionEvent) => {
    events.push(`${event.type}:${"batchId" in event ? event.batchId : "operationId" in event ? event.operationId : "projectId" in event ? event.projectId : ""}`);
  });
  return { result: hook.result, events };
}
type Harness = Awaited<ReturnType<typeof harness>>["result"];

/** Type rows into the store as the sheet's cells do, then Queue them as the sheet does. */
async function queueRows(result: Harness, rows: Array<Partial<Record<SheetColumnKey, string>> & { key?: string }>): Promise<string> {
  const { session, store } = result.current;
  const model = session.model.model!;
  act(() => store.update((current) => {
    let next: RunSheetState = { ...current, header: { ...current.header, source: SOURCE } };
    rows.forEach((row, index) => {
      const key = row.key ?? `r${index + 1}`;
      if (!next.rows.some((candidate) => candidate.key === key)) next = { ...next, rows: [...next.rows, ledger.blankRow(key)], nextRowKey: next.nextRowKey + 1 };
      for (const [column, text] of Object.entries(row)) if (column !== "key") next = setCell(next, key, column as SheetColumnKey, text as string);
      next = assignIds(next, key, model, []);
    });
    return next;
  }));
  const view = sheetView({ state: result.current.store.current(), statuses: result.current.store.statuses, model, conversions: new Map(), reservedIds: [] });
  expect(view.queueBlocked, JSON.stringify(view.rows.map((row) => row.refusals))).toBeNull();
  const build = await buildDraftBatch(view, model);
  act(() => result.current.store.update((current) => markSubmitted(current, view, build)));
  let key: string | null = null;
  await act(async () => { key = await result.current.session.operations.registerOperationBatch(build.batch, { reveal: false }); });
  expect(key).not.toBeNull();
  return build.batch.batch_id;
}
const CHAIN = [
  { kind: "F", node: "node:E" },
  { dx: "3", od: "0.1143", wall: "0.00602", material: "material:steel", orientation: "0 0 1" },
  { dy: "2", support: "A", restraints: "*" },
];
const entryOf = (result: Harness, batchId: string) => result.current.session.operations.queuedBatches.find((entry) => entry.batch.batch_id === batchId)!;
async function apply(result: Harness, batchId: string) {
  await act(async () => { await result.current.session.operations.handleRunOperationBatch(entryOf(result, batchId), true); });
}
const statusList = (result: Harness) => result.current.store.state.rows.map((row) => result.current.store.statuses.get(row.key)?.status);
/** Everything a failed Prepare must leave as it was. */
async function snapshot(result: Harness) {
  const { session, store } = result.current;
  return {
    hash: (await computeModelHash(session.model.model!))!.value,
    undo: session.operations.undoStack.map((item) => item.operation_id),
    redo: session.operations.redoStack.map((item) => item.operation_id),
    queue: session.operations.queuedBatches.map((entry) => [entry.key, entry.batch.batch_id, Boolean(entry.retired)]),
    cursor: store.state.ledger.cursor,
    records: store.state.ledger.records.length,
    buffers: store.state.rows.map((row) => [row.key, row.edits, row.restored, row.pending?.batchId ?? null]),
    statuses: statusList(result),
  };
}
const identityHash = sessionModel.uiModelIdentityHash;
const realTransition = ledger.applySessionEvent;

describe("run sheet S2 against the session: injected failures (§3.6 Prepare, Publish, Notify)", () => {
  it.each([
    ["the sheet's staged transition", () => vi.spyOn(ledger, "applySessionEvent").mockImplementation((state, event) => {
      if (event.type === "checkpoint") throw new Error("injected staged-transition failure");
      return realTransition(state, event);
    })],
    ["the identity hash", () => vi.spyOn(sessionModel, "uiModelIdentityHash").mockImplementationOnce(() => { throw new Error("injected identity-hash failure"); })],
    ["the checkpoint clone", () => vi.spyOn(sessionModel, "clonePreviewModel").mockImplementationOnce(() => { throw new Error("injected clone failure"); })],
  ])("a failure in Prepare via %s changes nothing, and a second Apply succeeds", async (_label, inject) => {
    const { result, events } = await harness();
    const batchId = await queueRows(result, CHAIN);
    const before = await snapshot(result);
    expect(before.statuses).toEqual(["queued", "queued", "queued"]);
    const spy = inject();
    await apply(result, batchId);
    expect(result.current.session.operations.batchMessage).toMatch(/^Batch request failed: Error: injected .* failure$/);
    expect(await snapshot(result)).toEqual(before);
    expect(events.filter((event) => event.startsWith("checkpoint") || event.startsWith("invalidated") || event.startsWith("abandoned"))).toEqual([]);
    spy.mockRestore();
    await apply(result, batchId);
    expect(statusList(result)).toEqual(["applied", "applied", "applied"]);
    expect(result.current.session.operations.undoStack.map((item) => item.operation_id)).toEqual([batchId]);
    expect(result.current.store.state.ledger.cursor).toBe(1);
  });

  it("a failure in Notify (the viewport's retire-box-gesture) is reported once; everything is published once and nothing is rolled back", async () => {
    const { result, events } = await harness();
    const batchId = await queueRows(result, CHAIN);
    let failures = 0;
    result.current.session.chrome.viewportViewCommandRef.current = (command) => {
      if (command.type === "retire-box-gesture") { failures += 1; throw new Error("injected viewport failure"); }
    };
    await apply(result, batchId);
    expect(failures).toBe(1);
    expect(result.current.session.operations.batchMessage).toBe("Batch applied once. Previous solve results were cleared; one undo checkpoint is available. The viewport did not retire its box gesture: Error: injected viewport failure");
    expect(result.current.session.operations.undoStack).toHaveLength(1);
    expect(result.current.session.operations.queuedBatches).toHaveLength(0);
    expect(events.filter((event) => event === `checkpoint:${batchId}`)).toHaveLength(1);
    expect(result.current.store.state.ledger.cursor).toBe(1);
    expect(statusList(result)).toEqual(["applied", "applied", "applied"]);
    expect(result.current.session.model.model!.nodes.some((node) => node.id === "node:N30")).toBe(true);
  });

  it("the same for Undo and Redo: a Prepare failure changes nothing and a retry steps; a Notify failure is reported and the step stands", async () => {
    const { result } = await harness();
    const batchId = await queueRows(result, CHAIN);
    await apply(result, batchId);
    for (const step of ["undo", "redo"] as const) {
      const before = await snapshot(result);
      const spy = vi.spyOn(ledger, "applySessionEvent").mockImplementation((state, event) => {
        if (event.type === (step === "undo" ? "undone" : "redone")) throw new Error("injected staged-transition failure");
        return realTransition(state, event);
      });
      act(() => step === "undo" ? result.current.session.operations.handleUndoSessionModelEdit() : result.current.session.operations.handleRedoSessionModelEdit());
      expect(result.current.session.operations.operationMessage).toBe(`${step === "undo" ? "Undo" : "Redo"} failed to run: Error: injected staged-transition failure`);
      expect(await snapshot(result)).toEqual(before);
      spy.mockRestore();
      const hash = vi.spyOn(sessionModel, "uiModelIdentityHash").mockImplementationOnce(() => { throw new Error("injected identity-hash failure"); });
      act(() => step === "undo" ? result.current.session.operations.handleUndoSessionModelEdit() : result.current.session.operations.handleRedoSessionModelEdit());
      expect(await snapshot(result)).toEqual(before);
      hash.mockRestore();
      let failures = 0;
      result.current.session.chrome.viewportViewCommandRef.current = (command) => {
        if (command.type === "retire-box-gesture") { failures += 1; throw new Error("injected viewport failure"); }
      };
      act(() => step === "undo" ? result.current.session.operations.handleUndoSessionModelEdit() : result.current.session.operations.handleRedoSessionModelEdit());
      expect(failures).toBe(1);
      expect(result.current.session.operations.historyFailure).toContain("The viewport did not retire its box gesture: Error: injected viewport failure");
      expect(result.current.store.state.ledger.cursor).toBe(step === "undo" ? 0 : 1);
      expect(statusList(result)).toEqual(step === "undo" ? ["draft", "draft", "draft"] : ["applied", "applied", "applied"]);
      result.current.session.chrome.viewportViewCommandRef.current = null;
    }
    expect(identityHash).toBe(sessionModel.uiModelIdentityHash);
  });
});

describe("run sheet S2 against the session: injected failures on the single-intent and draft-review apply paths", () => {
  function nodeIntent(model: PreviewModel) {
    const target = model.nodes.find((node) => node.id === "node:A")!;
    const column: GridColumn = { key: "y", label: "Y", fieldPath: "position.y", objectType: "Node", changeKind: "set_field", dimension: "length", sourceNote: "entered coordinate", unit: () => "m", value: () => String(target.position.y) };
    return buildGridOperationIntent({ column, model, row: { id: target.id, label: target.label, type: "node", raw: target, searchText: "" }, sequence: 1, value: "0.5", interaction: "cell" });
  }
  /** A route from node:D: to a new node:Z (a batch), or to the stored node:E (a single intent). */
  function route(kind: "batch" | "single"): RouteDraft {
    return {
      startNodeId: "node:D", endMode: kind === "single" ? "existing" : "new", existingEndNodeId: "node:E",
      newEnd: { id: "node:Z", label: "Z", x: "12", y: "4", z: "0", coordinateUnit: "m", provenance: "invented session test input" },
      pipe: { id: "pipe:Z", label: "Z", materialId: "material:steel", outsideDiameter: "0.1", wallThickness: "0.005", lengthUnit: "m", yReferenceX: "0", yReferenceY: "0", yReferenceZ: "1", provenance: "invented session test input" },
    };
  }
  async function review(result: Harness, kind: "batch" | "single"): Promise<FrozenDraftReview> {
    const built = buildRouteSubmission(result.current.session.model.model!, [], route(kind), 1, "length=passed");
    if (!built.ok || built.submission.kind !== kind) throw new Error(`expected a ${kind} route submission`);
    let frozen: FrozenDraftReview | null = null;
    await act(async () => { frozen = await result.current.session.operations.handleAddDraftReview(built.submission, 1); });
    expect(frozen).not.toBeNull();
    return frozen!;
  }
  const paths = {
    "single-intent apply": async (result: Harness) => {
      let applied = false;
      await act(async () => { applied = await result.current.session.operations.handleApplyIntent(nodeIntent(result.current.session.model.model!)); });
      return applied;
    },
    "draft-review apply (batch)": async (result: Harness) => {
      const frozen = await review(result, "batch");
      let applied = false;
      await act(async () => { applied = await result.current.session.operations.handleApplyDraftReview(frozen); });
      return applied;
    },
    "draft-review apply (single)": async (result: Harness) => {
      const frozen = await review(result, "single");
      let applied = false;
      await act(async () => { applied = await result.current.session.operations.handleApplyDraftReview(frozen); });
      return applied;
    },
  } as const;
  /** A sheet batch applied first, so the ledger has a record and the rows a status to keep. */
  async function withAppliedSheet() {
    const { result, events } = await harness();
    await apply(result, await queueRows(result, CHAIN));
    return { result, events };
  }
  it.each(Object.keys(paths) as Array<keyof typeof paths>)("%s: a failure in Prepare via the sheet's staged transition changes nothing; a retry applies", async (path) => {
    const { result, events } = await withAppliedSheet();
    const before = await snapshot(result);
    const spy = vi.spyOn(ledger, "applySessionEvent").mockImplementation((state, event) => {
      if (event.type === "checkpoint") throw new Error("injected staged-transition failure");
      return realTransition(state, event);
    });
    expect(await paths[path](result)).toBe(false);
    expect(result.current.session.operations.operationMessage).toMatch(/failed to run: Error: injected staged-transition failure$/);
    expect(await snapshot(result)).toEqual(before);
    const checkpoints = events.filter((event) => event.startsWith("checkpoint")).length;
    spy.mockRestore();
    expect(await paths[path](result)).toBe(true);
    expect(events.filter((event) => event.startsWith("checkpoint")).length).toBe(checkpoints + 1);
    expect(result.current.session.operations.undoStack).toHaveLength(2);
    expect([result.current.store.state.ledger.cursor, result.current.store.state.ledger.records.at(-1)?.foreign]).toEqual([2, true]);
    expect(statusList(result)).toEqual(["applied", "applied", "applied"]);
  });
  it.each(Object.keys(paths) as Array<keyof typeof paths>)("%s: a failure in Notify is reported once; the publication and the sheet's record stand", async (path) => {
    const { result } = await withAppliedSheet();
    let failures = 0;
    result.current.session.chrome.viewportViewCommandRef.current = (command) => {
      if (command.type === "retire-box-gesture") { failures += 1; throw new Error("injected viewport failure"); }
    };
    expect(await paths[path](result)).toBe(true);
    expect(failures).toBe(1);
    expect(result.current.session.operations.operationMessage).toContain("The viewport did not retire its box gesture: Error: injected viewport failure");
    expect(result.current.session.operations.undoStack).toHaveLength(2);
    expect([result.current.store.state.ledger.cursor, result.current.store.state.ledger.records.at(-1)?.foreign]).toEqual([2, true]);
    expect(statusList(result)).toEqual(["applied", "applied", "applied"]);
    result.current.session.chrome.viewportViewCommandRef.current = null;
  });
});

describe("run sheet S2 against the session: apply order, retirement, withdrawal and abandonment", () => {
  it("the sheet's own apply emits checkpoint and no invalidated for its batch; a geometry-tools batch queued before it on the same basis is retired with invalidated", async () => {
    const { result, events } = await harness();
    const model = structuredClone(result.current.session.model.model!);
    const geometry = await buildReconnectBatch(model, { ...emptyGeometryDraft(), action: "reconnect", pipeRefs: ["pipe:CD"], reconnectEnd: "to", targetNode: "node:H", yReference: { x: "0", y: "0", z: "1" } }, "batch:geometry");
    await act(async () => { await result.current.session.operations.handleQueueOperationBatch(geometry); });
    const sheetBatch = await queueRows(result, CHAIN);
    await apply(result, sheetBatch);
    const relevant = events.filter((event) => !event.startsWith("validated"));
    expect(relevant).toEqual(["queued:batch:geometry", `queued:${sheetBatch}`, `checkpoint:${sheetBatch}`, "invalidated:batch:geometry"]);
    expect(result.current.session.operations.queuedBatches.map((entry) => [entry.batch.batch_id, entry.retired])).toEqual([["batch:geometry", RETIRED_BATCH_NOTICE]]);
    // The retired registration cannot run.
    await act(async () => { await result.current.session.operations.handleRunOperationBatch(result.current.session.operations.queuedBatches[0], false); });
    expect(result.current.session.operations.batchMessage).toContain("is not registered in the review queue");
    act(() => result.current.session.operations.handleDismissRetiredBatch(result.current.session.operations.queuedBatches[0].key));
    expect(result.current.session.operations.queuedBatches).toEqual([]);
  });

  it("Clear pending changes emits withdrawn; the sheet's rows return to draft with their buffers", async () => {
    const { result, events } = await harness();
    const batchId = await queueRows(result, CHAIN);
    const buffers = result.current.store.state.rows.map((row) => row.edits);
    act(() => result.current.session.operations.handleClearReviewQueue());
    expect(events.at(-1)).toBe(`withdrawn:${batchId}`);
    expect(statusList(result)).toEqual(["draft", "draft", "draft"]);
    expect(result.current.store.state.rows.map((row) => row.edits)).toEqual(buffers);
    expect(result.current.store.state.rows.some((row) => row.requeueNeeded)).toBe(false);
  });

  it("a model edit on another surface retires the queued sheet batch with invalidated; Requeue builds a new batch on the current model", async () => {
    const { result, events } = await harness();
    const first = await queueRows(result, CHAIN);
    const otherModel = structuredClone(result.current.session.model.model!);
    const reconnect = await buildReconnectBatch(otherModel, { ...emptyGeometryDraft(), action: "reconnect", pipeRefs: ["pipe:CD"], reconnectEnd: "to", targetNode: "node:H", yReference: { x: "0", y: "0", z: "1" } }, "batch:other");
    await act(async () => { await result.current.session.operations.handleQueueOperationBatch(reconnect); });
    const other = entryOf(result, "batch:other");
    await act(async () => { await result.current.session.operations.handleRunOperationBatch(other, false); });
    await act(async () => { await result.current.session.operations.handleRunOperationBatch(entryOf(result, "batch:other"), true); });
    expect(events).toContain(`invalidated:${first}`);
    expect(statusList(result)).toEqual(["draft", "draft", "draft"]);
    expect(result.current.store.state.rows.every((row) => row.requeueNeeded)).toBe(true);
    const requeued = await queueRows(result, []);
    expect(requeued).not.toBe(first);
    expect(entryOf(result, requeued).basisRevision).toBe(result.current.session.model.modelRevision.current);
    await apply(result, requeued);
    expect(statusList(result)).toEqual(["applied", "applied", "applied"]);
  });

  it("a running batch whose run ends with no outcome while still registered emits abandoned and releases its rows, marked Requeue needed", async () => {
    const { result, events } = await harness();
    const batchId = await queueRows(result, CHAIN);
    const validate = operationBatchService.validateOperationBatch;
    vi.spyOn(operationBatchService, "validateOperationBatch").mockImplementationOnce(async (...args) => {
      const outcome = await validate(...args);
      // The revision moves while the run waits, before any retirement sees it.
      result.current.session.model.modelRevision.current += 1;
      return outcome;
    });
    await act(async () => { await result.current.session.operations.handleRunOperationBatch(entryOf(result, batchId), false); });
    expect(events.at(-1)).toBe(`abandoned:${batchId}`);
    expect(statusList(result)).toEqual(["draft", "draft", "draft"]);
    expect(result.current.store.state.rows.every((row) => row.requeueNeeded)).toBe(true);
  });
});

describe("run sheet S2 against the session: Open State folds the ledger and the rows reconcile", () => {
  it("opening a saved state of the same project resets history; the ledger folds; rows whose records the state holds stay applied and the others detach", async () => {
    const { result, events } = await harness();
    const first = await queueRows(result, CHAIN);
    await apply(result, first);
    // A second batch on the first chain's end.
    const second = await queueRows(result, [{ key: "r4", dx: "1.5" }]);
    await apply(result, second);
    expect(result.current.store.state.ledger.cursor).toBe(2);
    // The saved state: the model after the first batch only.
    const saved = structuredClone(result.current.session.operations.undoStack[0].model);
    const hash = (await computeModelHash(saved))!.value;
    const project = saved.project.id;
    const summary = {
      project_id: project, state_id: `state:${"1".padStart(32, "0")}`, state_kind: "design_snapshot", created_at: "2026-10-28T10:00:01.000Z",
      parent_state_id: null, model_hash: hash, record_hash: "r", labels_updated_at: "2026-10-28T10:00:01.000Z", child_state_ids: [], recorded_run_count: 0,
      analysis_status: ["HUMAN_REVIEW_REQUIRED"], state_name: "Invented state", tags: [], notes: [], assumptions: [],
    };
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(async (command: string) => {
      switch (command) {
        case "list_model_states": return [summary];
        case "list_analysis_runs": return [];
        case "read_model_state": return { state: summary, record: {}, model: structuredClone(saved), model_document_version: { status: "current" } };
        default: throw new Error(`unexpected ${command}`);
      }
    });
    await act(async () => { await result.current.session.project.history.refreshStates(project); });
    let opened: Promise<boolean> | null = null;
    act(() => { opened = result.current.session.project.history.openState(summary.state_id); });
    await waitFor(() => expect(result.current.session.project.unsavedChangesPrompt).not.toBeNull());
    act(() => result.current.session.project.unsavedChangesPrompt!.choose("discard"));
    await act(async () => { await opened; });
    delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
    expect(events).toContain(`history-reset:${project}`);
    expect(result.current.session.operations.undoStack).toEqual([]);
    const state = result.current.store.state;
    expect([state.ledger.cursor, state.ledger.records.length, Object.keys(state.ledger.base).sort()]).toEqual([0, 0, ["r1", "r2", "r3", "r4"]]);
    // r1–r3's records are in the opened state; r4's pipe and node are not.
    expect(statusList(result)).toEqual(["applied", "applied", "applied", "detached"]);
    expect(result.current.store.statuses.get("r4")!.changed).toEqual(["node:N40", "pipe:P40"]);
  });
});

describe("run sheet ledger (§3.6 Transitions), pure", () => {
  const appliedModel = (ids: string[]): PreviewModel => ({ ...structuredClone(fixture.basis_model), nodes: ids.map((id) => ({ id, label: id, position: { x: 0, y: 0, z: 0 } })) } as PreviewModel);
  function submitted(state: RunSheetState, batchId: string, rowKey: string, node: string): RunSheetState {
    return {
      ...state,
      submitted: { ...state.submitted, [batchId]: { batchId, batch: { batch_id: batchId, operations: [] }, rows: [{ key: rowKey, operationIds: [], ownedIds: [node] }], ownedRefs: { [rowKey]: [{ type: "node", id: node }] }, values: { [rowKey]: { dx: node } } } },
      rows: state.rows.map((row) => row.key === rowKey ? { ...row, pending: { batchId, registrationKey: "k", action: "create" } } : row),
    };
  }
  it("create, a foreign checkpoint, Undo, Undo, Redo, then a divergent checkpoint disposes the undone records", () => {
    let state: RunSheetState = { ...initialRunSheetState("project:x"), rows: [ledger.blankRow("a"), ledger.blankRow("b")] };
    state = ledger.applySessionEvent(submitted(state, "batch:A", "a", "node:A1"), { type: "checkpoint", operationId: "batch:A", appliedModel: appliedModel(["node:A1"]), operationOutcomes: [] });
    state = ledger.applySessionEvent(state, { type: "checkpoint", operationId: "op:foreign" });
    expect([state.ledger.cursor, state.ledger.records.map((record) => [record.operationId, record.foreign])]).toEqual([2, [["batch:A", false], ["op:foreign", true]]]);
    state = ledger.applySessionEvent(state, { type: "undone", operationId: "op:foreign" });
    expect(ledger.currentEntry(state.ledger, "a")?.batchId).toBe("batch:A");
    state = ledger.applySessionEvent(state, { type: "undone", operationId: "batch:A" });
    expect(ledger.currentEntry(state.ledger, "a")).toBeNull();
    expect(state.rows[0].restored).toEqual({ dx: "node:A1" });
    state = ledger.applySessionEvent(state, { type: "redone", operationId: "batch:A" });
    expect([state.ledger.cursor, state.rows[0].restored]).toEqual([1, {}]);
    state = ledger.applySessionEvent(submitted(state, "batch:B", "b", "node:B1"), { type: "checkpoint", operationId: "batch:B", appliedModel: appliedModel(["node:A1", "node:B1"]), operationOutcomes: [] });
    expect(state.ledger.records.map((record) => record.operationId)).toEqual(["batch:A", "batch:B"]);
    // An Undo or Redo the ledger does not mirror folds it.
    state = ledger.applySessionEvent(state, { type: "undone", operationId: "op:unknown" });
    expect([state.ledger.cursor, state.ledger.records.length, Object.keys(state.ledger.base).sort()]).toEqual([0, 0, ["a", "b"]]);
  });
  it("a history reset of another project starts an empty sheet; of the same project it folds", () => {
    let state: RunSheetState = { ...initialRunSheetState("project:x"), rows: [ledger.blankRow("a")] };
    state = ledger.applySessionEvent(submitted(state, "batch:A", "a", "node:A1"), { type: "checkpoint", operationId: "batch:A", appliedModel: appliedModel(["node:A1"]), operationOutcomes: [] });
    const folded = ledger.applySessionEvent(state, { type: "history-reset", projectId: "project:x" });
    expect([folded.ledger.cursor, Object.keys(folded.ledger.base)]).toEqual([0, ["a"]]);
    const other = ledger.applySessionEvent(state, { type: "history-reset", projectId: "project:y" });
    expect([other.projectId, other.rows.map((row) => row.key), other.ledger.records.length]).toEqual(["project:y", ["r1"], 0]);
  });
  it("reconciliation is exact JSON-value equality, independent of key order, with +0 and -0 alike, and checks tombstones", () => {
    let state: RunSheetState = { ...initialRunSheetState("project:x"), rows: [ledger.blankRow("a")] };
    state = ledger.applySessionEvent(submitted(state, "batch:A", "a", "node:A1"), { type: "checkpoint", operationId: "batch:A", appliedModel: appliedModel(["node:A1"]), operationOutcomes: [] });
    const record = { position: { z: -0, y: 0, x: 0 }, label: "node:A1", id: "node:A1" };
    expect(ledger.reconcileRunSheet(state, () => record).get("a")!.status).toBe("applied");
    expect(ledger.reconcileRunSheet(state, () => ({ ...record, label: "moved" })).get("a")).toMatchObject({ status: "detached", changed: ["node:A1"] });
    expect(ledger.reconcileRunSheet(state, () => undefined).get("a")).toMatchObject({ status: "detached", changed: ["node:A1"] });
    const tomb: RunSheetState = { ...state, ledger: { ...state.ledger, records: [{ ...state.ledger.records[0], rowEntries: { a: { ...state.ledger.records[0].rowEntries.a, removedIds: [{ type: "support", id: "support:S" }], expected: [...state.ledger.records[0].rowEntries.a.expected, { ref: { type: "support", id: "support:S" }, record: null }] } } }] } };
    expect(ledger.reconcileRunSheet(tomb, (ref) => ref.type === "support" ? { id: "support:S" } : record).get("a")).toMatchObject({ status: "detached", restoredOutside: ["support:S"] });
  });
});

