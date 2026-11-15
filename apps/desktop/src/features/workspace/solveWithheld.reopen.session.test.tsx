/** The Solve request's model check with a saved run held (DEL-07-07 REQ-008). A solved result
 * never meets a pending gate check: the check run its own Solve request made stays current until
 * the model value changes, and a model change retires the result. A reopened saved run is the
 * output a session holds while no check run is current (a project session starts with none), so a
 * Solve request then runs the checks first. A cancel or a model change during that check, or a
 * check that fails, must leave the restored run, its handle and the historical results surface
 * as they were. The reopen is the actual Rust commands' recorded replies
 * (`test-support/native-reopen/native_reopen_replies.json`), as `nativeReopen.session.test.tsx`
 * replays them; the model-check replies are invented. All data are invented or committed fixtures. */
import { act, cleanup, render, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock, { nativeReopen: "actual" }) }));
import { nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { getLocalStorageCapability } from "../../services/projectService";
import { canonicalJsonString } from "../../services/hashService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { NATIVE_OPEN_PROJECT_COMMAND, NATIVE_READ_RUN_COMMAND } from "../../services/nativeReopen";
import { buildBulkChangeBatch } from "../bulk-change/bulkChange";
import { HistoricalRunPanel } from "../results/HistoricalRunContext";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const REPLIES = resolve(__dirname, "../../test-support/native-reopen/native_reopen_replies.json");
/** A disposed handle never resurrects, so each reopen holds its own. */
let reopenSequence = 0;
const READ_HANDLE = "restored-stored-result-replay-2";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  delete (window as Json).__TAURI_INTERNALS__;
});

function substitute(value: unknown, from: string, to: string): unknown {
  if (value === from) return to;
  if (Array.isArray(value)) return value.map((item) => substitute(item, from, to));
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, substitute(item, from, to)]));
  return value;
}
function delivered(reply: Json): unknown {
  if (!reply.ok) throw reply.error;
  if (reply.body === "json") return structuredClone(reply.value);
  const bytes = Buffer.from(reply.base64, "base64");
  return bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);
}

/** A session that has reopened the stored project with its saved run held natively; the model
 * checks then answer through `check`. */
async function reopenedSession(check: () => Promise<unknown>) {
  const HANDLE = `restored-stored-result-gate-${++reopenSequence}`;
  const document = JSON.parse(readFileSync(REPLIES, "utf8")) as Json;
  const { model } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const openReply = substitute(document.open.reply.value, "$HANDLE", HANDLE) as Json;
  const readReply = substitute(document.read.reply.value, "$HANDLE", READ_HANDLE) as Json;
  invokeMock.mockImplementation(async (command: string, args: Json) => {
    switch (command) {
      case "get_local_storage_capability": return storage;
      case "load_design_knowledge": return knowledge;
      case "sync_native_shell_state": return null;
      case "check_model_library_refs": return [];
      case "load_preview_model": return structuredClone(model);
      case "dispose_all_preview_mechanics_results": return { released_results: 0, released_bytes: 0, abandoned_jobs: 0 };
      case "list_model_states": return [];
      case "list_analysis_runs": return [readReply.run];
      case "dispose_preview_mechanics_result": return { job_id: args.jobId, released: true, released_bytes: 1, delivered: false };
      case "check_preview_model": return check();
      case "cancel_model_checks": return null;
      case "start_preview_mechanics_job": throw new Error("INVENTED-START-OBSERVED");
      case NATIVE_OPEN_PROJECT_COMMAND: return structuredClone(openReply);
      case NATIVE_READ_RUN_COMMAND: return structuredClone(readReply);
    }
    if (args?.jobId === HANDLE) {
      const request = substitute(args, HANDLE, "$HANDLE");
      const found = (document.requests as Json[]).find((entry) => entry.command === command && isDeepStrictEqual(entry.args, request));
      if (found) return found.reply.body === "json" ? substitute(delivered(found.reply), "$HANDLE", HANDLE) : delivered(found.reply);
    }
    return null;
  });
  (window as Json).__TAURI_INTERNALS__ = {};
  let current!: WorkspaceSession;
  function Harness() {
    current = useWorkspaceSession();
    const history = current.results.historicalRun;
    return history ? <HistoricalRunPanel key={history.runId} context={history} /> : null;
  }
  const view = render(<Harness />);
  await waitFor(() => expect(current.model.model).not.toBeNull());
  await act(async () => { await current.project.handleOpenProject(openReply.model.project.id as string); });
  await waitFor(() => expect(current.results.historicalRun?.designation).toBe("historical_saved_run"));
  await waitFor(() => expect(view.container.textContent).not.toBe(""));
  expect(current.results.solveGate).toEqual({ kind: "unknown", reason: "not_checked" });
  return { state: () => current, view, handle: HANDLE };
}

/** What the session holds of the saved run, and the results surface it shows. */
function held(state: WorkspaceSession, container: HTMLElement) {
  const r = state.results;
  return {
    historicalRun: r.historicalRun, result: r.result, currentSolvedResult: r.currentSolvedResult, analysisRun: r.analysisRun,
    inputManifest: r.inputManifest, solveProof: r.solveProof, solveJob: r.solveJob, ruleCheckAggregate: r.ruleCheckAggregate,
    surface: container.innerHTML
  };
}
const calls = (command: string) => invokeMock.mock.calls.filter(([name]) => name === command).length;

it("a cancel while the Solve request's check runs leaves the reopened saved run, its handle and the results surface as they were", async () => {
  let release: (value: unknown) => void = () => {};
  const { state, view, handle } = await reopenedSession(() => new Promise((resolve) => { release = resolve; }));
  const before = held(state(), view.container);
  expect(before.historicalRun?.restored?.handle).toBe(handle);
  const disposals = calls("dispose_preview_mechanics_result");
  let solving: Promise<unknown> = Promise.resolve();
  act(() => { solving = state().results.handleRun(); });
  await waitFor(() => expect(calls("check_preview_model")).toBe(1));
  expect(state().results.running).toBe(true);
  act(() => { state().results.handleCancelRun(); });
  // While the check is still pending: its native cancel, no solve, nothing of the session changed.
  expect(calls("cancel_model_checks")).toBe(1);
  expect(calls("start_preview_mechanics_job")).toBe(0);
  expect(state().results.running).toBe(false);
  for (const [key, value] of Object.entries(before)) expect(held(state(), view.container)[key as keyof typeof before], key).toBe(value);
  // The late reply changes nothing.
  await act(async () => { release({ checked: true, diagnostics: [], diagnostics_not_shown: 0, slot_findings: [], slot_findings_not_shown: 0 }); await solving; });
  expect(calls("start_preview_mechanics_job")).toBe(0);
  for (const [key, value] of Object.entries(before)) expect(held(state(), view.container)[key as keyof typeof before], key).toBe(value);
  expect(calls("dispose_preview_mechanics_result")).toBe(disposals);
});

it("a model change while the Solve request's check runs: the edit's own effect is all that changes; the gate starts nothing and changes nothing after it", async () => {
  // The gate cannot keep a saved run across an edit: the edit itself retires it (the
  // output no longer describes the model). This shows only that the gate's ended check
  // adds nothing to the edit's effect: no solve, no further change, its late reply inert.
  let release: (value: unknown) => void = () => {};
  const { state, view } = await reopenedSession(() => new Promise((resolve) => { release = resolve; }));
  const beforeEdit = held(state(), view.container);
  let solving: Promise<unknown> = Promise.resolve();
  act(() => { solving = state().results.handleRun(); });
  await waitFor(() => expect(calls("check_preview_model")).toBe(1));
  // The edit runs in the browser engine; the check it ends is native.
  delete (window as Json).__TAURI_INTERNALS__;
  const model = state().model.model!;
  const batch = await buildBulkChangeBatch(model, [{ type: "pipe", id: model.pipe_segments[0].id }], { kind: "tag", tag: "invented-edit" }, canonicalJsonString, "solve-withheld-reopen-edit");
  await act(async () => { await state().operations.handleQueueOperationBatch(batch); });
  await act(async () => { await state().operations.handleRunOperationBatch(state().operations.queuedBatches[0], false); });
  await act(async () => { await state().operations.handleRunOperationBatch(state().operations.queuedBatches[0], true); });
  const afterEdit = held(state(), view.container);
  expect(afterEdit.historicalRun).not.toBe(beforeEdit.historicalRun);
  expect(calls("cancel_model_checks")).toBe(1);
  expect(calls("start_preview_mechanics_job")).toBe(0);
  expect(state().results.running).toBe(false);
  await act(async () => { release({ checked: true, diagnostics: [], diagnostics_not_shown: 0, slot_findings: [], slot_findings_not_shown: 0 }); await solving; });
  expect(calls("start_preview_mechanics_job")).toBe(0);
  for (const [key, value] of Object.entries(afterEdit)) expect(held(state(), view.container)[key as keyof typeof afterEdit], key).toBe(value);
});

it("a check that fails during the Solve request leaves the saved run and the results surface as they were while it runs; the solve then proceeds as before the gate", async () => {
  // Once the solve proceeds, its own start retires the saved run as any solve does; the
  // gate cannot keep it past a solve it lets start. What is shown is the gate's part:
  // nothing changed up to and during the check, and the solve then started.
  const { state, view } = await reopenedSession(() => Promise.reject(new Error("invented native failure of the check")));
  const before = held(state(), view.container);
  const disposals = calls("dispose_preview_mechanics_result");
  const atCheck: Array<ReturnType<typeof held>> = [];
  const inner = invokeMock.getMockImplementation()!;
  invokeMock.mockImplementation(async (command: string, args: Json) => {
    if (command === "check_preview_model") atCheck.push(held(state(), view.container));
    return inner(command, args);
  });
  await act(async () => { await state().results.handleRun(); });
  expect(atCheck).toHaveLength(1);
  for (const [key, value] of Object.entries(before)) expect(atCheck[0][key as keyof typeof before], key).toBe(value);
  expect(calls("start_preview_mechanics_job")).toBe(1);
  expect(calls("dispose_preview_mechanics_result")).toBe(disposals);
  expect(state().results.solveGate).toEqual({ kind: "unknown", reason: "checks_unavailable" });
  expect(state().results.running).toBe(false);
});
