/** Undo and Redo with a session event listener that fails while the step is
 * published (issue 575): the step is not split. The history stacks, the run
 * sheet's ledger, the model, the selection and the cleared computed state all
 * take the step together, the failure is said in the operation message and the
 * history failure line, and nothing escapes the handler.
 *
 * The session runs with the run sheet's store listening on its event channel,
 * through mocked IPC: a unit transport replay of committed producer bytes
 * solves the committed fixture model, so a held result exists only on that
 * model, and the native Apply is answered by the same operation crate built as
 * wasm. To Undo from the solved model, the session loads the fixture with its
 * tip moved and an Apply moves it back; that answer's applied model is the
 * fixture itself. NOT native UI evidence. All data are invented. */
import { act, cleanup, renderHook, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { computeModelHash } from "../../services/hashService";
import { outcomeFromWasmEngineJson } from "../../services/operationService";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import type { PreviewModel } from "../../types";
import { useRunSheetStore } from "../model-tree/runSheet/runSheetStore";
import type { SessionEvent } from "./sessionEvents";
import { buildGridOperationIntent, type GridColumn } from "./table/modelTableAdapter";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const ROOT = { type: "node", id: "node:fixture-root" } as const;
const TIP = { type: "node", id: "node:fixture-tip" } as const;
const LISTENER_FAILURE = "A part of the workspace did not record this step: Error: injected listener failure.";
const UNDESCRIBED_FAILURE = "A part of the workspace did not record this step: the failure could not be described.";
const UNDID = (operationId: string) =>
  `Undid ${operationId} in the local session; previous solve results were cleared. Save is still required to persist the current session model.`;
const REDID = (operationId: string) =>
  `Redid ${operationId} in the local session; previous solve results were cleared. Save is still required to persist the current session model.`;

const tipOf = (model: PreviewModel) => model.nodes.find((node) => node.id === TIP.id)!;

let sequence = 0;
/** The session and the run sheet's store, loading the committed fixture model, or that
 * model with its tip at `loadTipY`. */
async function harness(loadTipY?: number) {
  const { model: fixture, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
  const loaded = structuredClone(fixture);
  if (loadTipY !== undefined) tipOf(loaded).position.y = loadTipY;
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model: fixture, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return bytes;
  }, { jobIdPrefix: `unit-history-listener:${++sequence}` });
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(loaded);
    if (command === "apply_model_operation") {
      const { model, intent, claimedModelHash } = args as { model: PreviewModel; intent: unknown; claimedModelHash: unknown };
      const engine = await loadWasmEngine();
      const outcome = outcomeFromWasmEngineJson(engine.applyOperationJson(JSON.stringify(model), JSON.stringify(intent), JSON.stringify(claimedModelHash ?? null)));
      // An edit that puts the tip back where the fixture has it is answered with the fixture itself.
      if (outcome.applied_model && isDeepStrictEqual(tipOf(outcome.applied_model).position, tipOf(fixture).position)) {
        outcome.applied_model = structuredClone(fixture);
        outcome.applied_model_backend_hash = (await computeModelHash(fixture))!.value;
      }
      return outcome;
    }
    return transport.invoke(command, args, options);
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const hook = renderHook(() => {
    const session = useWorkspaceSession();
    const store = useRunSheetStore(session.operations.sessionEvents, session.model.activeModelIndex, session.model.model?.project.id ?? null);
    return { session, store };
  });
  await waitFor(() => expect(hook.result.current.session.model.model).not.toBeNull());
  await waitFor(() => expect(hook.result.current.session.model.modelHash).not.toBeNull());
  return { result: hook.result, fixture };
}
type Harness = Awaited<ReturnType<typeof harness>>["result"];

/** Solves the current model and records a rule-check aggregate on the result. */
async function solveAndRecord(result: Harness) {
  await act(async () => { await result.current.session.results.handleRun(); });
  await waitFor(() => expect(result.current.session.results.solveJob.state).toBe("completed"));
  const solved = result.current.session;
  expect(solved.results.currentSolvedResult).not.toBeNull();
  await act(async () => {
    await solved.results.handleRuleCheckAggregate("USER_RULE_CHECKED", {
      projectSessionGeneration: solved.model.projectSessionGeneration,
      modelRevision: solved.model.uiModelRevision,
      result: solved.results.currentSolvedResult,
      inputManifest: solved.results.inputManifest
    }, null);
  });
  await waitFor(() => expect(result.current.session.results.ruleCheckAggregate).toBe("USER_RULE_CHECKED"));
  const results = result.current.session.results;
  expect(results.result).not.toBeNull();
  expect(results.analysisRun).not.toBeNull();
  expect(results.inputManifest).not.toBeNull();
}

/** Applies a coordinate edit to the tip node's y and returns its operation id. */
async function applyTipY(result: Harness, value: number, sequenceNumber: number) {
  const model = result.current.session.model.model!;
  const target = tipOf(model);
  const column: GridColumn = { key: "y", label: "Y", fieldPath: "position.y", objectType: "Node", changeKind: "set_field", dimension: "length", sourceNote: "entered coordinate", unit: () => model.project.units.length, value: () => String(target.position.y) };
  const intent = buildGridOperationIntent({ column, model, row: { id: target.id, label: target.label, type: "node", raw: target, searchText: "" }, sequence: sequenceNumber, value: String(value), interaction: "cell" });
  let applied = false;
  await act(async () => { applied = await result.current.session.operations.handleApplyIntent(intent); });
  expect(applied).toBe(true);
  expect(tipOf(result.current.session.model.model!).position.y).toBe(value);
  return intent.operation_id;
}

const select = (result: Harness, entity: typeof ROOT | typeof TIP) => act(() => result.current.session.selection.handleSelectEntity(entity));

/** Undo from a solved model: loaded with the tip moved, edited back to the fixture (ROOT
 * selected), solved, then TIP selected. The Undo restores the loaded model and ROOT. */
async function solvedForUndo() {
  const { result, fixture } = await harness(0.5);
  select(result, ROOT);
  const operationId = await applyTipY(result, tipOf(fixture).position.y, 1);
  expect(result.current.session.model.model).toEqual(fixture);
  await solveAndRecord(result);
  select(result, TIP);
  return { result, operationId, checkpoint: result.current.session.operations.undoStack[0], selection: ROOT };
}

/** Redo onto a solved model: the fixture edited (ROOT selected), TIP selected, undone back
 * to the fixture and solved. The Redo restores the edited model and TIP. */
async function solvedForRedo() {
  const { result, fixture } = await harness();
  select(result, ROOT);
  const operationId = await applyTipY(result, 0.5, 1);
  select(result, TIP);
  act(() => result.current.session.operations.handleUndoSessionModelEdit());
  expect(result.current.session.model.model).toEqual(fixture);
  expect(result.current.session.operations.operationMessage).toBe(UNDID(operationId));
  await solveAndRecord(result);
  return { result, operationId, checkpoint: result.current.session.operations.redoStack[0], selection: TIP };
}

function expectHeldResultCleared(session: WorkspaceSession, action: "undo" | "redo") {
  const results = session.results;
  expect(results.result).toBeNull();
  expect(results.currentSolvedResult).toBeNull();
  expect(results.analysisRun).toBeNull();
  expect(results.inputManifest).toBeNull();
  expect(results.ruleCheckAggregate).toBeNull();
  expect(results.historicalRun).toBeNull();
  expect(results.solveJob.job_id).toBe(`job:preview-linear-static:session-${action}`);
}

/** A listener after the run sheet's store whose staged publish throws `thrown` on `type`. */
function failingListener(result: Harness, type: SessionEvent["type"], thrown: unknown) {
  let publishes = 0;
  const stop = result.current.session.operations.sessionEvents.listen((event) => {
    if (event.type !== type) return;
    return () => { publishes += 1; throw thrown; };
  });
  return { stop, publishes: () => publishes };
}

const hashOf = async (model: PreviewModel) => (await computeModelHash(model))!.value;
const modelHash = (result: Harness) => hashOf(result.current.session.model.model!);
const cursor = (result: Harness) => result.current.store.state.ledger.cursor;

const STEPS = {
  undo: { setup: solvedForUndo, event: "undone", step: (s: WorkspaceSession) => s.operations.handleUndoSessionModelEdit(), words: UNDID, verb: "Undid", toward: -1 },
  redo: { setup: solvedForRedo, event: "redone", step: (s: WorkspaceSession) => s.operations.handleRedoSessionModelEdit(), words: REDID, verb: "Redid", toward: 1 }
} as const;

const FAILING_LISTENERS: Array<["undo" | "redo", string, unknown, string]> = [
  ["undo", "an Error", new Error("injected listener failure"), LISTENER_FAILURE],
  ["undo", "a value that cannot be turned into text", Object.create(null), UNDESCRIBED_FAILURE],
  ["redo", "an Error", new Error("injected listener failure"), LISTENER_FAILURE],
  ["redo", "a value that cannot be turned into text", Object.create(null), UNDESCRIBED_FAILURE]
];

describe.each(FAILING_LISTENERS)("%s with a listener that throws %s on its event", (action, _label, thrown, failureWords) => {
  it("restores the model and the selection, clears the held result, moves the stacks and the ledger once, and says the failure", async () => {
    const step = STEPS[action];
    const { result, operationId, checkpoint, selection } = await step.setup();
    const undoDepth = result.current.session.operations.undoStack.length;
    const redoDepth = result.current.session.operations.redoStack.length;
    const ledgerCursor = cursor(result);
    const listener = failingListener(result, step.event, thrown);

    expect(() => act(() => step.step(result.current.session))).not.toThrow();
    listener.stop();

    const operations = result.current.session.operations;
    expect(listener.publishes()).toBe(1);
    expect(await modelHash(result)).toBe(await hashOf(checkpoint.model));
    expect(result.current.session.selection.selection).toEqual(selection);
    // Undo: the undo stack one shorter and the redo stack one longer; Redo the reverse.
    expect(operations.undoStack).toHaveLength(undoDepth + step.toward);
    expect(operations.redoStack).toHaveLength(redoDepth - step.toward);
    expect((action === "undo" ? operations.redoStack : operations.undoStack)[0].operation_id).toBe(operationId);
    expectHeldResultCleared(result.current.session, action);
    expect(cursor(result)).toBe(ledgerCursor + step.toward);
    expect(operations.operationMessage).toBe(`${step.words(operationId)} ${failureWords}`);
    expect(operations.historyFailure).toBe(`${step.verb} ${operationId}. ${failureWords}`);
  });
});

describe.each(["undo", "redo"] as const)("an ordinary %s, with every listener succeeding", (action) => {
  it("says only the step, restores the model and the selection, clears the held result and moves the ledger once", async () => {
    const step = STEPS[action];
    const { result, operationId, checkpoint, selection } = await step.setup();
    const ledgerCursor = cursor(result);
    const events: string[] = [];
    const stop = result.current.session.operations.sessionEvents.listen((event) => {
      if (event.type !== "undone" && event.type !== "redone") return;
      return () => { events.push(`${event.type}:${event.operationId}`); };
    });

    expect(() => act(() => step.step(result.current.session))).not.toThrow();
    stop();

    const operations = result.current.session.operations;
    expect(events).toEqual([`${step.event}:${operationId}`]);
    expect(await modelHash(result)).toBe(await hashOf(checkpoint.model));
    expect(result.current.session.selection.selection).toEqual(selection);
    expectHeldResultCleared(result.current.session, action);
    expect(cursor(result)).toBe(ledgerCursor + step.toward);
    expect(operations.operationMessage).toBe(step.words(operationId));
    expect(operations.historyFailure).toBeNull();
  });
});

it("a second Undo after a listener failed steps from the right place, and Redo returns through each model", async () => {
  const { result } = await harness();
  const basis = await modelHash(result);
  const first = await applyTipY(result, 0.5, 1);
  const afterFirst = await modelHash(result);
  const second = await applyTipY(result, 0.75, 2);
  const afterSecond = await modelHash(result);
  expect(cursor(result)).toBe(2);
  const listener = failingListener(result, "undone", new Error("injected listener failure"));

  expect(() => act(() => result.current.session.operations.handleUndoSessionModelEdit())).not.toThrow();
  listener.stop();
  expect(await modelHash(result)).toBe(afterFirst);
  expect(cursor(result)).toBe(1);
  expect(result.current.session.operations.operationMessage).toBe(`${UNDID(second)} ${LISTENER_FAILURE}`);

  act(() => result.current.session.operations.handleUndoSessionModelEdit());
  const operations = result.current.session.operations;
  expect(await modelHash(result)).toBe(basis);
  expect(cursor(result)).toBe(0);
  expect(operations.undoStack).toHaveLength(0);
  expect(operations.redoStack.map((checkpoint) => checkpoint.operation_id)).toEqual([first, second]);
  expect(await hashOf(operations.redoStack[0].model)).toBe(afterFirst);
  expect(await hashOf(operations.redoStack[1].model)).toBe(afterSecond);
  expect(operations.operationMessage).toBe(UNDID(first));
  expect(operations.historyFailure).toBeNull();

  act(() => result.current.session.operations.handleRedoSessionModelEdit());
  expect(await modelHash(result)).toBe(afterFirst);
  act(() => result.current.session.operations.handleRedoSessionModelEdit());
  expect(await modelHash(result)).toBe(afterSecond);
  expect(cursor(result)).toBe(2);
  expect(result.current.session.operations.operationMessage).toBe(REDID(second));
});
