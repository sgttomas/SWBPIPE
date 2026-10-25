/** W-18 (b2): File > Save Report Package… through the workspace session (DEL-08-01 Design
 * `report-package-current-results.md` §2.5). The save basis holds the rule-check
 * aggregate and run, and its guard reads them through refs; a basis change during the
 * awaited build, during the preparation's canonicalization and hash awaits, or before a
 * headroom re-send publishes nothing and sends nothing; a new run under an unchanged
 * aggregate clears the computed package state. Unit transport replay of committed
 * producer bytes through mocked IPC; NOT a native UI qualification witness. The
 * rule-check runs are invented unit fixtures, NOT rule-evaluation evidence. */
import { act, cleanup, renderHook, waitFor } from "@testing-library/react";
import { createHash } from "node:crypto";
import { isDeepStrictEqual } from "node:util";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { canonicalJsonString } from "../../services/hashService";
import * as saveService from "../../services/reportPackageSaveService";
import * as requestModule from "../report/reportPackageRequest";
import type { RuleCheckStatus } from "../../services/ruleCheckService";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

function deferred<T>() {
  let resolve!: (value: T) => void, reject!: (error: unknown) => void;
  const promise = new Promise<T>((res, rej) => { resolve = res; reject = rej; });
  return { promise, resolve, reject };
}

const RECEIPT = {
  outcome: "saved", stage: "complete", code: "REPORT-PACKAGE-SAVED", container_file_name: "invented.opsproj", byte_count: 1,
  package_identity_sha256_hex: "a".repeat(64), container_sha256_hex: "b".repeat(64), members: [], replaced_existing: false, durability: "durable",
  redaction_route_id: "DREP-PACKAGE-SAVE-009", redaction_decision_count: 0, redaction_finding_count: 0, redaction_blocking_count: 0,
  local_first_route_id: "DREP-PACKAGE-SAVE-009", local_first_export_context: "local_private", local_first_storage_context: "local_private",
  local_first_action: "include_metadata_only", local_first_reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED", local_first_blocked: false,
  local_first_metadata_only: true, local_first_explicit_local_private_intent: true, selected_basename: "invented.opsproj",
  path_containment: "best_effort_non_adversarial", limitation: "Invented limitation.", warnings: []
};

let sequence = 0;
/** A solved Current preview-physics-2 session; `save` answers `save_report_package`. */
async function solvedSession(save: (args: Record<string, unknown>) => Promise<unknown> = async () => RECEIPT) {
  const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return bytes;
  }, { jobIdPrefix: `unit-report-package-save:${++sequence}` });
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(model);
    if (command === "save_report_package") return save(args as Record<string, unknown>);
    return transport.invoke(command, args, options);
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const rendered = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(rendered.result.current.model.model).not.toBeNull());
  await waitFor(() => expect(rendered.result.current.model.modelHash).not.toBeNull());
  await act(async () => { await rendered.result.current.results.handleRun(); });
  await waitFor(() => expect(rendered.result.current.results.solveJob.state).toBe("completed"));
  expect(rendered.result.current.results.currentSolvedResult).not.toBeNull();
  act(() => rendered.result.current.results.setReportPackagePrivateIntent(true));
  return { ...rendered, model };
}

function inventedRun(aggregate: RuleCheckStatus, checkId = "invented_check"): SessionRuleCheckRun {
  return {
    rulePackDocument: { metadata: { rule_pack_id: "invented-test-only", name: "Invented test pack", rule_pack_version: "1.0.0", source_notice: "Invented notice." } },
    result: {
      document_kind: "openpipestress.rule_check.run",
      rule_pack_id: "invented-test-only",
      grammar_version: "1.0.0",
      aggregate_status: aggregate,
      checks: [{ check_id: checkId, status: aggregate, acceptability_relation: "invented", bound_inputs: [], completeness_findings: [], evaluator_findings: [], diagnostic_codes: [] }],
      professional_boundary_notice: "notice"
    }
  };
}

async function recordRuleCheck(state: () => WorkspaceSession, aggregate: RuleCheckStatus | null, run: SessionRuleCheckRun | null) {
  const current = state();
  await act(async () => {
    await current.results.handleRuleCheckAggregate(aggregate, {
      projectSessionGeneration: current.model.projectSessionGeneration,
      modelRevision: current.model.uiModelRevision,
      result: current.results.currentSolvedResult,
      inputManifest: current.results.inputManifest
    }, run);
  });
}

const saveCalls = () => invokeMock.mock.calls.filter(([command]) => command === "save_report_package");

describe("Save Report Package… through the session (W-18 (b2))", () => {
  it("sends the request and model canonical texts with screening bound to the request text", async () => {
    const { result: hook, model } = await solvedSession();
    await act(async () => { await hook.current.results.handleSaveReportPackage(); });
    expect(saveCalls()).toHaveLength(1);
    const args = saveCalls()[0][1] as Record<string, any>; // eslint-disable-line @typescript-eslint/no-explicit-any
    expect(Object.keys(args).sort()).toEqual(["localFirstEvidence", "modelCanonicalText", "requestCanonicalText", "screening"]);
    const request = JSON.parse(args.requestCanonicalText);
    expect(await canonicalJsonString(request)).toBe(args.requestCanonicalText);
    expect(request.export_profile_id).toBe("desktop_local_private_report_package_1");
    expect(request.rule_check_aggregate).toBeNull();
    expect(args.modelCanonicalText).toBe(await canonicalJsonString(model));
    expect(args.screening.payload_sha256).toBe(`sha256:${createHash("sha256").update(args.requestCanonicalText, "utf8").digest("hex")}`);
    expect(args.screening).toMatchObject({ route_id: "DREP-PACKAGE-SAVE-009", export_context: "local_private", explicit_local_private_intent: true, exact_payload_match: true, lossless_required: true, blocked: false });
    expect(hook.current.results.reportPackageRoute).toEqual({ route: "tauri_report_package_save", receipt: RECEIPT });
    expect(hook.current.results.reportPackageBusy).toBe(false);
  });

  it("publishes nothing and sends nothing when the rule-check basis changes during the awaited build", async () => {
    const { result: hook } = await solvedSession();
    const gate = deferred<void>();
    const real = requestModule.buildReportPackageRequest;
    vi.spyOn(requestModule, "buildReportPackageRequest").mockImplementation(async (input) => {
      const request = await real(input);
      await gate.promise;
      return request;
    });
    let saving!: Promise<void>;
    act(() => { saving = hook.current.results.handleSaveReportPackage(); });
    await waitFor(() => expect(hook.current.results.reportPackageBusy).toBe(true));
    await recordRuleCheck(() => hook.current, "RULE_INPUTS_INCOMPLETE", inventedRun("RULE_INPUTS_INCOMPLETE"));
    await act(async () => { gate.resolve(); await saving; });
    expect(saveCalls()).toHaveLength(0);
    expect(hook.current.results.reportPackageRoute).toBeNull();
    expect(hook.current.results.reportPackageRedaction).toBeNull();
  });

  it("publishes nothing and sends nothing when the basis changes during the preparation's canonicalization and hash awaits", async () => {
    const { result: hook } = await solvedSession();
    const gate = deferred<void>();
    const real = saveService.prepareReportPackageSave;
    const prepared = vi.spyOn(saveService, "prepareReportPackageSave").mockImplementation(async (...args) => {
      const value = await real(...args);
      await gate.promise;
      return value;
    });
    let saving!: Promise<void>;
    act(() => { saving = hook.current.results.handleSaveReportPackage(); });
    await waitFor(() => expect(prepared).toHaveBeenCalled());
    await recordRuleCheck(() => hook.current, "RULE_INPUTS_INCOMPLETE", inventedRun("RULE_INPUTS_INCOMPLETE"));
    await act(async () => { gate.resolve(); await saving; });
    expect(saveCalls()).toHaveLength(0);
    expect(hook.current.results.reportPackageRoute).toBeNull();
  });

  it("runs the last guard inside the invocation and never re-sends a retired basis after a headroom refusal", async () => {
    const first = deferred<unknown>();
    const { result: hook } = await solvedSession(() => first.promise);
    const invoked = vi.spyOn(saveService, "invokeReportPackageSave");
    let saving!: Promise<void>;
    act(() => { saving = hook.current.results.handleSaveReportPackage(); });
    await waitFor(() => expect(saveCalls()).toHaveLength(1));
    const guard = invoked.mock.calls[0][1];
    expect(guard()).toBe(true);
    await recordRuleCheck(() => hook.current, "RULE_INPUTS_INCOMPLETE", inventedRun("RULE_INPUTS_INCOMPLETE"));
    expect(guard()).toBe(false);
    await act(async () => {
      first.reject({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "headroom_insufficient", required_bytes: 2, available_bytes: 1 }, pool: null });
      await saving;
    });
    expect(saveCalls()).toHaveLength(1);
    expect(hook.current.results.reportPackageRoute).toBeNull();
  });

  it("clears the computed package state when a new run arrives under the same aggregate", async () => {
    const { result: hook } = await solvedSession();
    await recordRuleCheck(() => hook.current, "RULE_INPUTS_INCOMPLETE", inventedRun("RULE_INPUTS_INCOMPLETE", "first_check"));
    await waitFor(() => expect(hook.current.results.analysisRun?.analysis_run.analysis_status).toContain("RULE_INPUTS_INCOMPLETE"));
    expect(hook.current.results.ruleCheckReportRun).not.toBeNull();
    await act(async () => { await hook.current.results.handleSaveReportPackage(); });
    // The model references no pack, so the rule check cannot stand in for the solve's packs.
    expect(hook.current.results.reportPackageRoute).toMatchObject({ route: "redaction_blocked", diagnostic: expect.stringContaining("REPORT-PACKAGE-RULE-PACKS-DISAGREE") });
    expect(saveCalls()).toHaveLength(0);

    const second = inventedRun("RULE_INPUTS_INCOMPLETE", "second_check");
    await recordRuleCheck(() => hook.current, "RULE_INPUTS_INCOMPLETE", second);
    expect(hook.current.results.ruleCheckAggregate).toBe("RULE_INPUTS_INCOMPLETE");
    expect(hook.current.results.ruleCheckReportRun).toBe(second);
    expect(hook.current.results.reportPackageRoute).toBeNull();
    expect(hook.current.results.reportPackageRedaction).toBeNull();
  });
});
