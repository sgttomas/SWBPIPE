/** W-18b desktop slice:
 * File > Save Shareable Report Package… through the workspace session. The command opens the
 * user-release dialog for the current report; its confirmation starts one route-010 save; a
 * basis change closes the dialog; nothing outlives the save. Unit transport replay of committed
 * producer bytes through mocked IPC; NOT a native UI qualification witness. */
import { act, cleanup, renderHook, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { canonicalSha256Hex } from "../../services/hashService";
import { useWorkspaceSession } from "./workspaceSession";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const RECEIPT = { outcome: "saved", code: "REPORT-PACKAGE-SAVED", user_release_recorded: true, user_release_model_document_sha256: "c".repeat(64) };

let sequence = 0;
async function solvedSession(profile: "preview" | "pressure" = "preview") {
  const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile });
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return bytes;
  }, { jobIdPrefix: `unit-user-release-save:${++sequence}` });
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(model);
    if (command === "save_report_package") return RECEIPT;
    return transport.invoke(command, args, options);
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const rendered = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(rendered.result.current.model.model).not.toBeNull());
  if (profile === "pressure") await waitFor(() => expect(rendered.result.current.model.modelHash).not.toBeNull());
  await act(async () => { await rendered.result.current.results.handleRun(); });
  await waitFor(() => expect(rendered.result.current.results.solveJob.state).toBe("completed"));
  return rendered;
}

const saveCalls = () => invokeMock.mock.calls.filter(([command]) => command === "save_report_package");

describe("Save Shareable Report Package… through the session (W-18b)", () => {
  it("opens the dialog from the menu, and its confirmation sends one route-010 save", async () => {
    const { result: hook } = await solvedSession();
    act(() => hook.current.chrome.runMenuCommand("file.save-shareable-report-package"));
    await waitFor(() => expect(hook.current.results.userReleaseRequest).not.toBeNull());
    const request = hook.current.results.userReleaseRequest!;
    const model = hook.current.model.model!;
    expect(request.refusal).toBeNull();
    expect(request.runId).toBe(hook.current.results.currentSolvedResult!.run_id);
    expect(request.modelDocumentSha256).toBe(await canonicalSha256Hex(model));
    expect(request.projectName).toBe(model.project.name);
    // The dialog holds the menu: another command does nothing while it is open.
    act(() => hook.current.chrome.runMenuCommand("file.save-report-package"));
    expect(saveCalls()).toHaveLength(0);
    await act(async () => { request.save({ userReleasedAt: "2026-11-09T00:00:00Z" }); });
    expect(hook.current.results.userReleaseRequest).toBeNull();
    await waitFor(() => expect(hook.current.results.reportPackageRoute).not.toBeNull());
    expect(saveCalls()).toHaveLength(1);
    const args = saveCalls()[0][1] as Record<string, any>; // eslint-disable-line @typescript-eslint/no-explicit-any
    expect(Object.keys(args).sort()).toEqual([
      "analysisRunCanonicalText", "inputManifestCanonicalText", "localFirstEvidence", "modelCanonicalText", "requestCanonicalText", "screening"
    ]);
    expect(args.screening.route_id).toBe("DREP-PACKAGE-USER-RELEASE-010");
    const sent = JSON.parse(args.requestCanonicalText);
    expect(sent.export_profile_id).toBe("desktop_user_released_report_package_1");
    expect(sent.report.report_context.user_release.user_released_at).toBe("2026-11-09T00:00:00Z");
    expect(hook.current.results.reportPackageRoute).toEqual({ route: "tauri_report_package_save", receipt: RECEIPT });
    expect(hook.current.results.reportPackageBusy).toBe(false);
  });

  it("closes the dialog without saving when the basis changes or the user cancels", async () => {
    const { result: hook } = await solvedSession();
    await act(async () => { await hook.current.results.requestSaveShareableReportPackage(); });
    expect(hook.current.results.userReleaseRequest).not.toBeNull();
    act(() => hook.current.results.setReportComposition({ composition_version: "1.0.0", sections: [] }));
    expect(hook.current.results.userReleaseRequest).toBeNull();
    await act(async () => { await hook.current.results.requestSaveShareableReportPackage(); });
    act(() => hook.current.results.userReleaseRequest!.cancel());
    expect(hook.current.results.userReleaseRequest).toBeNull();
    expect(saveCalls()).toHaveLength(0);
    expect(hook.current.results.reportPackageRoute).toBeNull();
  });

  it("a confirmed save whose basis moves on during its preparation sends nothing and releases the package controls", async () => {
    // The pressure replay profile holds the native reply of the rule-check record update.
    const { result: hook } = await solvedSession("pressure");
    await act(async () => { await hook.current.results.requestSaveShareableReportPackage(); });
    const request = hook.current.results.userReleaseRequest!;
    expect(request.refusal).toBeNull();
    // Hold the rule-check record's native read and every read of the save, so the record lands
    // while the save is preparing.
    const passThrough = invokeMock.getMockImplementation()!;
    let releaseRecord!: () => void, releaseSave!: () => void;
    const recordHeld = new Promise<void>((resolve) => { releaseRecord = resolve; });
    const saveHeld = new Promise<void>((resolve) => { releaseSave = resolve; });
    invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
      if (command === "read_preview_mechanics_job_analysis_run_preview") await recordHeld;
      else if (command !== "sync_native_shell_state") await saveHeld;
      return passThrough(command, args, options);
    });
    const run: SessionRuleCheckRun = {
      rulePackDocument: { metadata: { rule_pack_id: "invented-test-only", name: "Invented test pack" } },
      result: { document_kind: "openpipestress.rule_check.run", rule_pack_id: "invented-test-only", grammar_version: "1.0.0", aggregate_status: "USER_RULE_CHECKED", checks: [], professional_boundary_notice: "notice" }
    };
    const current = hook.current;
    let recorded!: Promise<void>;
    act(() => {
      recorded = current.results.handleRuleCheckAggregate("USER_RULE_CHECKED", {
        projectSessionGeneration: current.model.projectSessionGeneration,
        modelRevision: current.model.uiModelRevision,
        result: current.results.currentSolvedResult,
        inputManifest: current.results.inputManifest
      }, run);
    });
    await act(async () => { request.save({ userReleasedAt: "2026-11-09T00:00:00Z" }); });
    expect(hook.current.results.reportPackageBusy).toBe(true);
    const runBefore = hook.current.results.analysisRun;
    await act(async () => { releaseRecord(); await recorded; });
    expect(hook.current.results.analysisRun === runBefore, "the rule-check record replaced the run").toBe(false);
    await act(async () => { releaseSave(); });
    await waitFor(() => expect(hook.current.results.reportPackageBusy).toBe(false));
    expect(saveCalls()).toHaveLength(0);
  });
});
