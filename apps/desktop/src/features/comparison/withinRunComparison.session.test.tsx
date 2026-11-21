// "Within this run" in the actual workspace session (W-12 S3c; DEL-14-01 Design §4.3):
// the choice is held with the run it was made on. A new solve, a model edit (which
// retires the result) and a project replacement each read the default again; the
// choice is refused while the session is busy; and without a choice the session's
// packages carry the fixed pair's build: here, where the solve withheld C-OPER-ALT, its
// unavailable record, not an empty comparison. The run is the
// producer's range-envelope output replayed through the native solve-job transport
// (unit transport replay; NOT a native UI qualification witness). Invented data only.
// C1 (DEL-04-05 `c1-native-custody-plan.md` slice 4): the session holds a fresh result natively,
// and comparisons (a C2 consumer) read its full copy only once it is requested (the comparisons
// panel's "Load"); until then there is nothing to compare. Each solve's copy is requested here as
// the person does, through the session's `loadC2Full`.
import { act, cleanup, renderHook, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import type { PreviewModel } from "../../types";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge } from "../../services/previewService";
import { COMPARISON_BASIS_WITHOUT_ROWS } from "./withinRunComparison";
import { canonicalJsonString } from "../../services/hashService";
import { createSolveJobTransport } from "../../test/nativeMechanicsReplay";
import { createNativePersistenceFake, isNativePersistenceCommand } from "../../test/nativePersistenceFake";
import { createLocalProject } from "../../services/projectService";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { isNativeResult } from "../../services/nativeResult";
import { setNodeProximityToleranceIntent } from "../model-settings/modelSettingsIntents";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const MODEL = JSON.parse(readFileSync(resolve(root, "core/product_physics/tests/fixtures/preview_physics_range_envelope_model.json"), "utf8")) as PreviewModel;
const BYTES = readFileSync(resolve(root, "fixtures/results/preview_physics_range_envelope_sparse.json"), "utf8");
const RANGE = "combination:U4B-RANGE-MAX";

const liveHooks = new Set<{ unmount(): void }>();
afterEach(() => {
  cleanup();
  for (const hook of liveHooks) { try { hook.unmount(); } catch { /* already unmounted */ } }
  liveHooks.clear();
  invokeMock.mockReset(); window.localStorage.clear(); delete (window as Json).__TAURI_INTERNALS__;
  saves.length = 0;
});

/** The arguments of each `save_report_package` the session sends; the invented native side
 * performs no save (it refuses after recording what was sent). */
const saves: Json[] = [];

async function currentSession(answer: (bytes: string) => string | Promise<string> = (bytes) => bytes) {
  // `answer` maps the producer's bytes to what the replayed job returns.
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((sent) => {
    if (!isDeepStrictEqual(sent.model, MODEL)) throw new Error("REPLAY_REQUEST_MISMATCH");
    return answer(BYTES);
  });
  // C1 slice 5: Create Project of the native result goes from its job handle; the unit fake keeps the
  // forms today's submitted path writes in the browser adapter, for the reopen.
  const fake = createNativePersistenceFake({
    heldBytes: transport.heldBytes,
    heldIntake: transport.heldIntake,
    save: async ({ request }) => {
      const tauri = (window as Json).__TAURI_INTERNALS__;
      delete (window as Json).__TAURI_INTERNALS__;
      try {
        return await createLocalProject(request.model, request.editorIntents, request.proposal, request.selectedReviewTarget, request.mechanicsResult, request.analysisRun, request.modelHash, request.projectEnvelopeHash, request.confirmedRevision);
      } finally {
        (window as Json).__TAURI_INTERNALS__ = tauri;
      }
    }
  });
  invokeMock.mockImplementation(async (command: string, args: Json, options?: unknown) => {
    if (isNativePersistenceCommand(command)) return fake.invoke(command, args);
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(MODEL);
    if (command === "save_report_package") { saves.push(args); throw new Error("INVENTED_SAVE_NOT_PERFORMED"); }
    return transport.invoke(command, args, options);
  });
  (window as Json).__TAURI_INTERNALS__ = {};
  const hook = renderHook(() => useWorkspaceSession());
  liveHooks.add(hook);
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  await waitFor(() => expect(hook.result.current.model.modelHash).not.toBeNull());
  act(() => hook.result.current.results.setSolverMode("sparse_interactive"));
  await solve(hook);
  return hook;
}

async function solve(hook: { result: { current: ReturnType<typeof useWorkspaceSession> } }) {
  const before = hook.result.current.results.currentSolvedResult;
  await act(async () => { await hook.result.current.results.handleRun(); });
  await waitFor(() => expect(hook.result.current.results.currentSolvedResult).not.toBeNull());
  // Identity only: a deep comparison would read the native handles' guarded members.
  expect(hook.result.current.results.currentSolvedResult !== before).toBe(true);
  const solved = hook.result.current.results.currentSolvedResult!;
  expect(isNativeResult(solved)).toBe(true);
  // Nothing is compared until the comparison's full copy is requested.
  expect(hook.result.current.results.withinRun).toEqual({ status: "no_result" });
  await act(async () => { await hook.result.current.results.loadC2Full(solved); });
  expect(hook.result.current.results.fullResult).not.toBeNull();
}

const pairOf = (results: ReturnType<typeof useWorkspaceSession>["results"]) =>
  results.withinRun.status === "ready" ? { ...results.withinRun.pair, source: results.withinRun.source } : results.withinRun.status;

it("carries the choice across solves that offer its bases, and packages follow it", async () => {
  const hook = await currentSession();
  const results = () => hook.result.current.results;
  // The default: the fixed pair is not offered (C-OPER-ALT has no rows), so the first offered.
  expect(pairOf(results())).toEqual({ left: "load:L-100", right: RANGE, source: "default" });
  // Without a choice, the packages carry the fixed pair's build: C-OPER-ALT has no rows (the
  // solve withheld it), so its unavailable record and no comparison.
  expect(results().comparison).toBeNull();
  expect(results().comparisonUnavailable).toMatchObject({ status: "unavailable", left_basis_ref: "load:L-100", right_basis_ref: "combination:C-OPER-ALT", reason_code: COMPARISON_BASIS_WITHOUT_ROWS });

  act(() => results().handleSelectWithinRunBases({ left: "load:L-200", right: RANGE }));
  expect(pairOf(results())).toEqual({ left: "load:L-200", right: RANGE, source: "selected" });
  // The packages now carry the choice: the producer's range-envelope rows are refused by the
  // unchanged builder (undeclared kinds, the known gap routed to T6), so a structured record.
  expect(results().comparison).toBeNull();
  expect(results().comparisonUnavailable).toMatchObject({ status: "unavailable", left_basis_ref: "load:L-200", right_basis_ref: RANGE, reason_code: "ANALYSIS-RUN-RESULT-DIMENSION-UNDECLARED" });

  // A new solve that offers both bases carries the choice (the producer's run_id repeats).
  await solve(hook);
  expect(pairOf(results())).toEqual({ left: "load:L-200", right: RANGE, source: "selected" });
  expect(results().droppedWithinRunBases).toBeNull();

  // (A run that no longer offers a chosen basis drops it with a notice:
  // withinRun.resultsSessionState.test.ts, over two producer outputs.)

  // A model edit retires the run: nothing is compared or packaged until the next solve,
  // which revalidates the choice.
  const edit = await setNodeProximityToleranceIntent(canonicalJsonString, hook.result.current.model.model!, "2", "mm");
  // The edit applies through the browser operation engine (the replay serves solves only).
  delete (window as Json).__TAURI_INTERNALS__;
  await act(async () => { expect(await hook.result.current.operations.handleApplyIntent(edit)).toBe(true); });
  expect(results().currentSolvedResult).toBeNull();
  expect(results().withinRun).toEqual({ status: "no_result" });
  expect(results().comparison).toBeNull();
  expect(results().droppedWithinRunBases).toBeNull();
});

it("a project replacement retires the run; the choice is revalidated against the next Current run", async () => {
  const hook = await currentSession();
  const results = () => hook.result.current.results;
  act(() => results().handleSelectWithinRunBases({ left: "load:L-200", right: RANGE }));
  expect(pairOf(results())).toEqual({ left: "load:L-200", right: RANGE, source: "selected" });
  await act(async () => { await hook.result.current.project.handleCreateProject(); });
  delete (window as Json).__TAURI_INTERNALS__;
  await act(async () => { await hook.result.current.project.handleOpenProject(); });
  expect(results().currentSolvedResult).toBeNull();
  expect(results().withinRun).toEqual({ status: "no_result" });
});

it("refuses a choice while a solve runs, and states the reason", async () => {
  let gate: Promise<void> | null = null;
  const hook = await currentSession((answer) => (gate ? gate.then(() => answer) : answer));
  const results = () => hook.result.current.results;
  expect(results().withinRunDisabledReason).toBeNull();
  let release!: () => void;
  gate = new Promise<void>((resolve) => { release = resolve; });
  const select = results().handleSelectWithinRunBases;
  let running!: Promise<void>;
  act(() => { running = results().handleRun(); });
  await waitFor(() => expect(results().running).toBe(true));
  expect(results().withinRunDisabledReason).toBe("A solve is in progress; its run will replace this one.");
  // Refused from the busy render, and from the render before it: no choice is held.
  act(() => results().handleSelectWithinRunBases({ left: "load:L-200", right: RANGE }));
  act(() => select({ left: "load:L-200", right: RANGE }));
  release();
  await act(async () => { await running; });
  await waitFor(() => expect(results().currentSolvedResult).not.toBeNull());
  expect(results().withinRunDisabledReason).toBeNull();
  // The new run's comparison copy, requested: no refused choice was held for it.
  await act(async () => { await results().loadC2Full(results().currentSolvedResult!); });
  expect(pairOf(results())).toEqual({ left: "load:L-100", right: RANGE, source: "default" });
});

/** A new solve whose comparison copy is not requested: the session compares nothing yet. */
async function freshSolve(hook: { result: { current: ReturnType<typeof useWorkspaceSession> } }) {
  const before = hook.result.current.results.currentSolvedResult;
  await act(async () => { await hook.result.current.results.handleRun(); });
  await waitFor(() => expect(hook.result.current.results.currentSolvedResult).not.toBeNull());
  expect(hook.result.current.results.currentSolvedResult !== before).toBe(true);
  expect(hook.result.current.results.withinRun).toEqual({ status: "no_result" });
  expect(hook.result.current.results.comparison).toBeNull();
  expect(hook.result.current.results.comparisonUnavailable).toBeNull();
}

/** The comparison members of the last request the session sent. */
function sentComparison() {
  expect(saves.length).toBeGreaterThan(0);
  const request = JSON.parse(saves[saves.length - 1].requestCanonicalText);
  const record = request.state_comparison_handoff_records[0];
  return { unavailable: record.comparison_unavailable ?? null, sections: record.sections.comparison_sections };
}

it("File > Save Report Package on a fresh native solve, before its comparison copy is loaded, carries the comparison the session resolves", async () => {
  const hook = await currentSession();
  const results = () => hook.result.current.results;
  await freshSolve(hook);
  act(() => results().setReportPackagePrivateIntent(true));
  await act(async () => { await results().handleSaveReportPackage(); });
  expect(saves).toHaveLength(1);
  // Without a choice, the fixed pair: C-OPER-ALT is withheld here, so its unavailable record
  // (not two absent members, and not an empty comparison).
  const sent = sentComparison();
  expect(sent.sections).toEqual([]);
  expect(sent.unavailable).toMatchObject({ left_basis_ref: "load:L-100", right_basis_ref: "combination:C-OPER-ALT", reason_code: COMPARISON_BASIS_WITHOUT_ROWS });
  // The save read the report run's full copy; the session's own comparison is now the same.
  expect(results().comparisonUnavailable).toEqual(sent.unavailable);
});

it("the same save carries a choice held from an earlier run, and a choice attempted during the save neither applies nor changes the package", async () => {
  const hook = await currentSession();
  const results = () => hook.result.current.results;
  act(() => results().handleSelectWithinRunBases({ left: "load:L-200", right: RANGE }));
  expect(results().heldWithinRunSelection).toEqual({ left: "load:L-200", right: RANGE });
  await freshSolve(hook);
  act(() => results().setReportPackagePrivateIntent(true));
  let saving!: Promise<void>;
  act(() => { saving = results().handleSaveReportPackage(); });
  // Refused while the package is being prepared: the held choice stays the save's basis.
  act(() => results().handleSelectWithinRunBases({ left: "load:L-100", right: RANGE }));
  await act(async () => { await saving; });
  expect(results().heldWithinRunSelection).toEqual({ left: "load:L-200", right: RANGE });
  expect(saves).toHaveLength(1);
  // The held choice's outcome: the unchanged builder refuses the producer's range-envelope rows.
  expect(sentComparison().unavailable).toMatchObject({ left_basis_ref: "load:L-200", right_basis_ref: RANGE, reason_code: "ANALYSIS-RUN-RESULT-DIMENSION-UNDECLARED" });
});
