/** W-18b desktop slice (DEL-08-01 Design `report-user-release.md` §2.2, §2.3 "Session"):
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
async function solvedSession() {
  const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
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
});
