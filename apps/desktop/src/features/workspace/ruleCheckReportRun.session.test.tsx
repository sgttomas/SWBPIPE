/** W-3 (PRD §19.2 user-rule checks in the rendered report): the session holds the
 * rule-check run behind the lifted aggregate and exposes it to the report only while
 * that aggregate is current. A re-solve retires it; a failed aggregate recording rolls
 * it back with the aggregate. Unit transport replay of committed producer bytes through
 * mocked IPC; the rule-check run is an invented unit fixture, NOT rule-evaluation
 * evidence. All data are invented. C1 (DEL-04-05 `c1-native-custody-plan.md` slice 4): the
 * result is held natively, so a revised record is the native analysis-run preview (the actual
 * Rust command's recorded reply); a failed recording is that command failing. */
import { act, cleanup, renderHook, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { ANALYSIS_RUN_PREVIEW_COMMAND } from "../../services/analysisRunPreview";
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

let sequence = 0;
/** When set, the native analysis-run preview (the revised record's builder) fails. */
let previewOutage = false;
async function solvedSession() {
  previewOutage = false;
  const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return bytes;
  }, { jobIdPrefix: `unit-rule-check-report-run:${++sequence}` });
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(model);
    if (command === ANALYSIS_RUN_PREVIEW_COMMAND && previewOutage) throw new Error("invented hashing outage");
    return transport.invoke(command, args, options);
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const rendered = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(rendered.result.current.model.model).not.toBeNull());
  await waitFor(() => expect(rendered.result.current.model.modelHash).not.toBeNull());
  await act(async () => { await rendered.result.current.results.handleRun(); });
  await waitFor(() => expect(rendered.result.current.results.solveJob.state).toBe("completed"));
  expect(rendered.result.current.results.currentSolvedResult).not.toBeNull();
  return rendered;
}

function invented(aggregate: RuleCheckStatus): SessionRuleCheckRun {
  return {
    rulePackDocument: { metadata: { rule_pack_id: "invented-test-only", name: "Invented test pack" } },
    result: {
      document_kind: "openpipestress.rule_check.run",
      rule_pack_id: "invented-test-only",
      grammar_version: "1.0.0",
      aggregate_status: aggregate,
      checks: [],
      professional_boundary_notice: "notice"
    }
  };
}

async function record(state: () => WorkspaceSession, aggregate: RuleCheckStatus | null, run: SessionRuleCheckRun | null) {
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

describe("the session rule-check run held for the rendered report (W-3)", () => {
  it("is exposed with its current aggregate and withheld after a re-solve resets the aggregate", async () => {
    const { result: hook } = await solvedSession();
    const run = invented("USER_RULE_CHECKED");
    await record(() => hook.current, "USER_RULE_CHECKED", run);
    await waitFor(() => expect(hook.current.results.analysisRun?.analysis_run.analysis_status).toContain("USER_RULE_CHECKED"));
    expect(hook.current.results.ruleCheckAggregate).toBe("USER_RULE_CHECKED");
    expect(hook.current.results.ruleCheckReportRun).toBe(run);

    await act(async () => { await hook.current.results.handleRun(); });
    await waitFor(() => expect(hook.current.results.solveJob.state).toBe("completed"));
    expect(hook.current.results.ruleCheckAggregate).toBeNull();
    expect(hook.current.results.ruleCheckReportRun).toBeNull();
  });

  it("rolls the held run back with the aggregate when recording the aggregate fails", async () => {
    const { result: hook } = await solvedSession();
    const first = invented("USER_RULE_CHECKED");
    await record(() => hook.current, "USER_RULE_CHECKED", first);
    await waitFor(() => expect(hook.current.results.analysisRun?.analysis_run.analysis_status).toContain("USER_RULE_CHECKED"));
    expect(hook.current.results.ruleCheckReportRun).toBe(first);

    previewOutage = true;
    const before = invokeMock.mock.calls.filter(([command]) => command === ANALYSIS_RUN_PREVIEW_COMMAND).length;
    await record(() => hook.current, "USER_RULE_FAILED", invented("USER_RULE_FAILED"));
    expect(invokeMock.mock.calls.filter(([command]) => command === ANALYSIS_RUN_PREVIEW_COMMAND).length).toBeGreaterThan(before);
    await waitFor(() => expect(hook.current.results.ruleCheckAggregate).toBe("USER_RULE_CHECKED"));
    expect(hook.current.results.ruleCheckReportRun).toBe(first);
  });

  it("is never exposed for a run whose aggregate differs from the recorded one", async () => {
    const { result: hook } = await solvedSession();
    await record(() => hook.current, "USER_RULE_CHECKED", invented("USER_RULE_FAILED"));
    expect(hook.current.results.ruleCheckAggregate).toBe("USER_RULE_CHECKED");
    expect(hook.current.results.ruleCheckReportRun).toBeNull();
  });
});
