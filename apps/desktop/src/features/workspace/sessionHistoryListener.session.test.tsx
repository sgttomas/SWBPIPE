/** Undo and Redo with a session event listener that fails while the step is
 * published (issue 575): the step is not split. The history stacks, the run
 * sheet's ledger, the model, the selection and the cleared computed state all
 * take the step together, the failure is said in the operation message and the
 * history failure line, and nothing escapes the handler. The session runs with
 * the run sheet's store listening on its event channel, the actual wasm engine
 * for the edits, and a unit transport replay of committed producer bytes for
 * the one solve whose held result the step must clear. NOT native UI
 * evidence. All data are invented. */
import { act, cleanup, renderHook, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
// The session's results state, captured so that a held result can be placed on
// an edited model: the replay solves only the committed fixture model, and Undo
// is possible only after an edit, which clears the result the solve left.
type ResultsState = ReturnType<typeof import("./resultsSessionState").useResultsSessionState>;
const resultsState = vi.hoisted(() => ({ current: null as ResultsState | null }));
vi.mock("./resultsSessionState", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./resultsSessionState")>();
  return {
    ...actual,
    useResultsSessionState: (...args: Parameters<typeof actual.useResultsSessionState>) => {
      const state = actual.useResultsSessionState(...args);
      resultsState.current = state;
      return state;
    }
  };
});
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { computeModelHash } from "../../services/hashService";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import type { PreviewModel } from "../../types";
import { useRunSheetStore } from "../model-tree/runSheet/runSheetStore";
import type { SessionEvent } from "./sessionEvents";
import { buildGridOperationIntent, type GridColumn } from "./table/modelTableAdapter";
import { useWorkspaceSession } from "./workspaceSession";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  resultsState.current = null;
  setNativeHost(false);
});

function setNativeHost(present: boolean) {
  if (present) (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  else delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
}

const ROOT = { type: "node", id: "node:fixture-root" } as const;
const TIP = { type: "node", id: "node:fixture-tip" } as const;
const LISTENER_FAILURE = "A part of the workspace did not record this step: Error: injected listener failure.";
const UNDESCRIBED_FAILURE = "A part of the workspace did not record this step: the failure could not be described.";
const UNDID = (operationId: string) =>
  `Undid ${operationId} in the local session; previous solve results were cleared. Save is still required to persist the current session model.`;
const REDID = (operationId: string) =>
  `Redid ${operationId} in the local session; previous solve results were cleared. Save is still required to persist the current session model.`;

let sequence = 0;
/** The session and the run sheet's store on the committed fixture model; with `solve`, one
 * solve is run and a rule-check aggregate recorded on it, and the held values are returned. */
async function harness({ solve }: { solve: boolean }) {
  const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return bytes;
  }, { jobIdPrefix: `unit-history-listener:${++sequence}` });
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(model);
    return transport.invoke(command, args, options);
  });
  setNativeHost(true);
  const hook = renderHook(() => {
    const session = useWorkspaceSession();
    const store = useRunSheetStore(session.operations.sessionEvents, session.model.activeModelIndex, session.model.model?.project.id ?? null);
    return { session, store };
  });
  const result = hook.result;
  await waitFor(() => expect(result.current.session.model.model).not.toBeNull());
  await waitFor(() => expect(result.current.session.model.modelHash).not.toBeNull());
  let held: HeldResult | null = null;
  if (solve) {
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
    const { result: heldResult, analysisRun, inputManifest, ruleCheckAggregate } = result.current.session.results;
    expect(heldResult).not.toBeNull();
    expect(analysisRun).not.toBeNull();
    expect(inputManifest).not.toBeNull();
    held = { result: heldResult, analysisRun, inputManifest, ruleCheckAggregate };
  }
  // The edits run on the wasm engine.
  setNativeHost(false);
  return { result, held };
}
type Harness = Awaited<ReturnType<typeof harness>>["result"];
type HeldResult = Pick<ResultsState, "result" | "analysisRun" | "inputManifest" | "ruleCheckAggregate">;

/** Applies a coordinate edit to the tip node's y and returns its operation id. */
async function applyTipY(result: Harness, value: string, sequenceNumber: number) {
  const model = result.current.session.model.model!;
  const target = model.nodes.find((node) => node.id === TIP.id)!;
  const column: GridColumn = { key: "y", label: "Y", fieldPath: "position.y", objectType: "Node", changeKind: "set_field", dimension: "length", sourceNote: "entered coordinate", unit: () => model.project.units.length, value: () => String(target.position.y) };
  const intent = buildGridOperationIntent({ column, model, row: { id: target.id, label: target.label, type: "node", raw: target, searchText: "" }, sequence: sequenceNumber, value, interaction: "cell" });
  let applied = false;
  await act(async () => { applied = await result.current.session.operations.handleApplyIntent(intent); });
  expect(applied).toBe(true);
  expect(result.current.session.model.model!.nodes.find((node) => node.id === TIP.id)!.position.y).toBe(Number(value));
  return intent.operation_id;
}

/** Places the solve's held result, its run, manifest and rule-check aggregate on the session. */
function placeHeldResult(result: Harness, held: HeldResult) {
  act(() => {
    const state = resultsState.current!;
    state.setResult(held.result);
    state.setAnalysisRun(held.analysisRun);
    state.setInputManifest(held.inputManifest);
    state.setRuleCheckAggregate(held.ruleCheckAggregate);
  });
  const results = result.current.session.results;
  expect(results.result).toBe(held.result);
  expect(results.analysisRun).toBe(held.analysisRun);
  expect(results.inputManifest).toBe(held.inputManifest);
  expect(results.ruleCheckAggregate).toBe("USER_RULE_CHECKED");
}

function expectHeldResultCleared(result: Harness, action: "undo" | "redo") {
  const results = result.current.session.results;
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
const select = (result: Harness, entity: typeof ROOT | typeof TIP) => act(() => result.current.session.selection.handleSelectEntity(entity));

describe.each([
  ["an Error", new Error("injected listener failure"), LISTENER_FAILURE],
  ["a value that cannot be turned into text", Object.create(null) as unknown, UNDESCRIBED_FAILURE]
])("Undo with a listener that throws %s on its event", (_label, thrown, failureWords) => {
  it("restores the model and the selection, clears the held result, moves the stacks and the ledger once, and says the failure", async () => {
    const { result, held } = await harness({ solve: true });
    select(result, ROOT);
    const operationId = await applyTipY(result, "0.5", 1);
    placeHeldResult(result, held!);
    select(result, TIP);
    const checkpoint = result.current.session.operations.undoStack[0];
    const undoDepth = result.current.session.operations.undoStack.length;
    const redoDepth = result.current.session.operations.redoStack.length;
    const ledgerCursor = cursor(result);
    const listener = failingListener(result, "undone", thrown);

    expect(() => act(() => result.current.session.operations.handleUndoSessionModelEdit())).not.toThrow();
    listener.stop();

    const operations = result.current.session.operations;
    expect(listener.publishes()).toBe(1);
    expect(await modelHash(result)).toBe(await hashOf(checkpoint.model));
    expect(result.current.session.selection.selection).toEqual(ROOT);
    expect(operations.undoStack).toHaveLength(undoDepth - 1);
    expect(operations.redoStack).toHaveLength(redoDepth + 1);
    expect(operations.redoStack[0].operation_id).toBe(operationId);
    expectHeldResultCleared(result, "undo");
    expect(cursor(result)).toBe(ledgerCursor - 1);
    expect(operations.operationMessage).toBe(`${UNDID(operationId)} ${failureWords}`);
    expect(operations.historyFailure).toBe(`Undid ${operationId}. ${failureWords}`);
  });
});

describe.each([
  ["an Error", new Error("injected listener failure"), LISTENER_FAILURE],
  ["a value that cannot be turned into text", Object.create(null) as unknown, UNDESCRIBED_FAILURE]
])("Redo with a listener that throws %s on its event", (_label, thrown, failureWords) => {
  it("restores the model and the selection, clears the held result, moves the stacks and the ledger once, and says the failure", async () => {
    const { result, held } = await harness({ solve: true });
    select(result, ROOT);
    const operationId = await applyTipY(result, "0.5", 1);
    select(result, TIP);
    act(() => result.current.session.operations.handleUndoSessionModelEdit());
    expect(result.current.session.operations.operationMessage).toBe(UNDID(operationId));
    placeHeldResult(result, held!);
    const checkpoint = result.current.session.operations.redoStack[0];
    const undoDepth = result.current.session.operations.undoStack.length;
    const redoDepth = result.current.session.operations.redoStack.length;
    const ledgerCursor = cursor(result);
    const listener = failingListener(result, "redone", thrown);

    expect(() => act(() => result.current.session.operations.handleRedoSessionModelEdit())).not.toThrow();
    listener.stop();

    const operations = result.current.session.operations;
    expect(listener.publishes()).toBe(1);
    expect(await modelHash(result)).toBe(await hashOf(checkpoint.model));
    expect(result.current.session.selection.selection).toEqual(TIP);
    expect(operations.undoStack).toHaveLength(undoDepth + 1);
    expect(operations.redoStack).toHaveLength(redoDepth - 1);
    expect(operations.undoStack[0].operation_id).toBe(operationId);
    expectHeldResultCleared(result, "redo");
    expect(cursor(result)).toBe(ledgerCursor + 1);
    expect(operations.operationMessage).toBe(`${REDID(operationId)} ${failureWords}`);
    expect(operations.historyFailure).toBe(`Redid ${operationId}. ${failureWords}`);
  });
});

it("a second Undo after a listener failed steps from the right place, and Redo returns through each model", async () => {
  const { result } = await harness({ solve: false });
  const basis = await modelHash(result);
  const first = await applyTipY(result, "0.5", 1);
  const afterFirst = await modelHash(result);
  const second = await applyTipY(result, "0.75", 2);
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

it("an ordinary Undo and Redo, with every listener succeeding, say only the step and clear the held result", async () => {
  const { result, held } = await harness({ solve: true });
  const events: string[] = [];
  const stop = result.current.session.operations.sessionEvents.listen((event) => {
    if (event.type !== "undone" && event.type !== "redone") return;
    return () => { events.push(`${event.type}:${event.operationId}`); };
  });
  const basis = await modelHash(result);
  const operationId = await applyTipY(result, "0.5", 1);
  const applied = await modelHash(result);
  placeHeldResult(result, held!);

  expect(() => act(() => result.current.session.operations.handleUndoSessionModelEdit())).not.toThrow();
  expect(await modelHash(result)).toBe(basis);
  expectHeldResultCleared(result, "undo");
  expect(cursor(result)).toBe(0);
  expect(result.current.session.operations.operationMessage).toBe(UNDID(operationId));
  expect(result.current.session.operations.historyFailure).toBeNull();

  placeHeldResult(result, held!);
  expect(() => act(() => result.current.session.operations.handleRedoSessionModelEdit())).not.toThrow();
  stop();
  expect(await modelHash(result)).toBe(applied);
  expectHeldResultCleared(result, "redo");
  expect(cursor(result)).toBe(1);
  expect(result.current.session.operations.operationMessage).toBe(REDID(operationId));
  expect(result.current.session.operations.historyFailure).toBeNull();
  expect(events).toEqual([`undone:${operationId}`, `redone:${operationId}`]);
});
