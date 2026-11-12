// The Solve request's model check on the native path (DEL-07-07 REQ-008): the
// check a Solve request starts is its own, and ending the request (a cancel, a
// model edit, the workspace's unmount) cancels it natively, never a newer check.
// A withheld request leaves a held result, its rule-check aggregate and report
// state as they were. The native host is mocked IPC: the check replies are
// invented, and the solve is a unit transport replay of committed producer bytes.
import { act, cleanup, renderHook, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { afterEach, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import * as previewService from "../../services/previewService";
import { getLocalStorageCapability } from "../../services/projectService";
import { canonicalJsonString } from "../../services/hashService";
import type { RuleCheckStatus } from "../../services/ruleCheckService";
import type { PreviewModel } from "../../types";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";
import { buildBulkChangeBatch } from "../bulk-change/bulkChange";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  previewService.resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const tauri = (on: boolean) => {
  if (on) (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  else delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
};
const calls = (command: string) => invokeMock.mock.calls.filter(([name]) => name === command).length;
const report = (diagnostics: unknown[] = []) => ({ checked: true, diagnostics, diagnostics_not_shown: 0, slot_findings: [], slot_findings_not_shown: 0 });

const PIPE = "pipe:P-100";
/** A browser-loaded session (the bundled invented model, pipe P-100's material
 * emptied) whose model checks, once the native runtime is switched on, wait in `held`. */
async function heldCheckSession() {
  const loaded = JSON.parse(JSON.stringify(await previewService.loadPreviewModel())) as PreviewModel;
  const material = loaded.pipe_segments.find((pipe) => pipe.id === PIPE)!.material;
  loaded.pipe_segments.find((pipe) => pipe.id === PIPE)!.material = "";
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(loaded);
  const start = vi.spyOn(previewService, "startPreviewMechanicsJob").mockRejectedValue(new Error("INVENTED-START-OBSERVED"));
  const held: Array<(value: unknown) => void> = [];
  invokeMock.mockImplementation((command: string) => {
    if (command === "check_preview_model") return new Promise((resolve) => held.push(resolve));
    return Promise.resolve(null);
  });
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return { ...hook, start, held, material };
}

async function pressSolveWithHeldCheck(result: { current: WorkspaceSession }, held: unknown[]) {
  tauri(true);
  let solving: Promise<void> = Promise.resolve();
  act(() => { solving = result.current.results.handleRun(); });
  await waitFor(() => expect(held).toHaveLength(1));
  expect(result.current.results.running).toBe(true);
  // Wrapped: an async function returning the pending solve would wait for it.
  return { solving };
}

it("a cancel during the Solve request's check cancels that check natively and releases the session while the check is still pending; its late result changes nothing", async () => {
  const { result, start, held } = await heldCheckSession();
  const r = result.current.results;
  const before = { result: r.result, analysisRun: r.analysisRun, inputManifest: r.inputManifest, solveJob: r.solveJob, solveProof: r.solveProof, ruleCheckAggregate: r.ruleCheckAggregate };
  const { solving } = await pressSolveWithHeldCheck(result, held);
  act(() => { result.current.results.handleCancelRun(); });
  // The check is still pending: its worker and grant are released by the native cancel.
  expect(calls("cancel_model_checks")).toBe(1);
  expect(start).not.toHaveBeenCalled();
  expect(result.current.results.running).toBe(false);
  expect(result.current.chrome.modelCheckBusy).toBe(false);
  const after = result.current.results;
  for (const [key, value] of Object.entries(before)) expect(after[key as keyof typeof before], key).toBe(value);
  // Solve is free again: a new request takes the token and starts its own check.
  let again: Promise<void> = Promise.resolve();
  act(() => { again = result.current.results.handleRun(); });
  await waitFor(() => expect(held).toHaveLength(2));
  // The cancelled check's late result changes nothing; the new request's check stands.
  await act(async () => { held[0](report()); await solving; });
  expect(start).not.toHaveBeenCalled();
  expect(result.current.chrome.modelCheckRun).toBeNull();
  expect(result.current.chrome.modelCheckBusy).toBe(true);
  expect(calls("cancel_model_checks")).toBe(1);
  await act(async () => { held[1](report()); await again; });
  expect(start).toHaveBeenCalledOnce();
});

it("a model edit during the Solve request's check cancels that check natively", async () => {
  const { result, start, held, material } = await heldCheckSession();
  const { solving } = await pressSolveWithHeldCheck(result, held);
  tauri(false); // The edit runs in the browser engine; the check it ends is native.
  const batch = await buildBulkChangeBatch(result.current.model.model!, [{ type: "pipe", id: PIPE }], { kind: "material", materialId: material }, canonicalJsonString, "solve-withheld-native-edit");
  await act(async () => { await result.current.operations.handleQueueOperationBatch(batch); });
  await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], false); });
  await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
  await waitFor(() => expect(calls("cancel_model_checks")).toBe(1));
  await act(async () => { held[0](report()); await solving; });
  expect(start).not.toHaveBeenCalled();
  expect(result.current.results.running).toBe(false);
});

it("a newer check survives the end of an older Solve request: Cancel does not cancel the person's own Run model checks", async () => {
  const { result, start, held } = await heldCheckSession();
  const { solving } = await pressSolveWithHeldCheck(result, held);
  let checking: Promise<void> = Promise.resolve();
  act(() => { checking = result.current.chrome.handleRunModelChecks(); });
  await waitFor(() => expect(held).toHaveLength(2));
  act(() => { result.current.results.handleCancelRun(); });
  expect(calls("cancel_model_checks")).toBe(0);
  expect(result.current.chrome.modelCheckBusy).toBe(true);
  await act(async () => { held[1](report([{ id: "d:newer", code: "INVENTED_NEWER", severity: "warning", message: "invented newer check", affected_refs: [] }])); await checking; });
  await act(async () => { held[0](report()); await solving; });
  expect(result.current.chrome.modelCheckRun?.findings.map((finding) => finding.diagnostic.code)).toEqual(["INVENTED_NEWER"]);
  expect(result.current.chrome.modelCheckBusy).toBe(false);
  expect(start).not.toHaveBeenCalled();
});

it("an unmount during the Solve request's check cancels that check natively", async () => {
  const { result, start, held, unmount } = await heldCheckSession();
  const { solving } = await pressSolveWithHeldCheck(result, held);
  unmount();
  expect(calls("cancel_model_checks")).toBe(1);
  held[0](report());
  await solving;
  expect(start).not.toHaveBeenCalled();
});

let sequence = 0;
function inventedRun(aggregate: RuleCheckStatus): SessionRuleCheckRun {
  return {
    rulePackDocument: { metadata: { rule_pack_id: "invented-test-only", name: "Invented test pack", rule_pack_version: "1.0.0", source_notice: "Invented notice." } },
    result: {
      document_kind: "openpipestress.rule_check.run",
      rule_pack_id: "invented-test-only",
      grammar_version: "1.0.0",
      aggregate_status: aggregate,
      checks: [{ check_id: "invented_check", status: aggregate, acceptability_relation: "invented", bound_inputs: [], completeness_findings: [], evaluator_findings: [], diagnostic_codes: [] }],
      professional_boundary_notice: "notice"
    }
  };
}

it("a withheld request leaves a held result, its rule-check aggregate and run, and the report package state as they were", async () => {
  const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const storage = await getLocalStorageCapability(), knowledge = await previewService.loadDesignKnowledge();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return bytes;
  }, { jobIdPrefix: `unit-solve-withheld:${++sequence}` });
  // The check at the first Solve could not run (the native host refused it, as when every worker
  // slot is occupied), so the solve proceeded as before the gate; the person's later Check Model
  // runs and reports a blocking finding (an invented reply) for the same model value.
  let checkReply: unknown = new Error("invented native refusal of the check");
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(model);
    if (command === "check_preview_model") { if (checkReply instanceof Error) throw checkReply; return checkReply; }
    return transport.invoke(command, args, options);
  });
  tauri(true);
  const { result: hook } = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.current.model.model).not.toBeNull());
  await waitFor(() => expect(hook.current.model.modelHash).not.toBeNull());
  await act(async () => { await hook.current.results.handleRun(); });
  await waitFor(() => expect(hook.current.results.solveJob.state).toBe("completed"));
  expect(hook.current.results.currentSolvedResult).not.toBeNull();
  expect(hook.current.results.solveGate).toEqual({ kind: "unknown", reason: "checks_unavailable" });
  act(() => hook.current.results.setReportPackagePrivateIntent(true));
  const current = hook.current;
  await act(async () => {
    await current.results.handleRuleCheckAggregate("RULE_INPUTS_INCOMPLETE", {
      projectSessionGeneration: current.model.projectSessionGeneration,
      modelRevision: current.model.uiModelRevision,
      result: current.results.currentSolvedResult,
      inputManifest: current.results.inputManifest
    }, inventedRun("RULE_INPUTS_INCOMPLETE"));
  });
  await waitFor(() => expect(hook.current.results.analysisRun?.analysis_run.analysis_status).toContain("RULE_INPUTS_INCOMPLETE"));
  await act(async () => { await hook.current.results.handleSaveReportPackage(); });
  expect(hook.current.results.reportPackageRoute).not.toBeNull();

  const node = hook.current.model.model!.nodes[0].id;
  checkReply = report([{ id: "d:invented", code: "INVENTED_INPUT_MISSING", severity: "blocking", message: "invented missing value", affected_refs: [node] }]);
  await act(async () => { await hook.current.chrome.handleRunModelChecks(); });
  expect(hook.current.results.solveGate.kind).toBe("withheld");

  const r = hook.current.results;
  const before = {
    result: r.result, currentSolvedResult: r.currentSolvedResult, analysisRun: r.analysisRun, inputManifest: r.inputManifest,
    solveJob: r.solveJob, solveProof: r.solveProof, ruleCheckAggregate: r.ruleCheckAggregate, ruleCheckReportRun: r.ruleCheckReportRun,
    reportPackageRoute: r.reportPackageRoute, reportPackageRedaction: r.reportPackageRedaction, reportComposition: r.reportComposition,
    numericalHealth: r.numericalHealth, historicalRun: r.historicalRun, dormantOutputBasis: r.dormantOutputBasis
  };
  expect(before.result).not.toBeNull();
  expect(before.solveProof).not.toBeNull();
  expect(before.analysisRun).not.toBeNull();
  expect(before.ruleCheckAggregate).toBe("RULE_INPUTS_INCOMPLETE");
  expect(before.ruleCheckReportRun).not.toBeNull();
  const starts = calls("start_preview_mechanics_job");
  await act(async () => { await hook.current.results.handleRun(); });
  expect(calls("start_preview_mechanics_job")).toBe(starts);
  const after = hook.current.results;
  for (const [key, value] of Object.entries(before)) expect(after[key as keyof typeof before], key).toBe(value);
  expect(after.running).toBe(false);
});
