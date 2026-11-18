/** The report package saves keep only the evidence they read of the package request's
 * redaction control. The control returns a decision and a finding for every value it screens;
 * the session keeps its summary and blocked flag (the package summary line) and nothing else,
 * and neither save holds the control result while its native save is awaited. Unit transport
 * replay of committed producer bytes through mocked IPC; NOT a native UI qualification
 * witness. */
import { act, cleanup, render, renderHook, screen, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { ControlledRouteExport } from "../redaction-controls/redactionExportControls";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
// Each package control result the saves make, and its controlled payload, observed through a
// WeakRef; a test that compares the shown line with the full result's asks for the result to be
// kept as well.
const observed = vi.hoisted(() => ({ keep: false, refs: [] as WeakRef<object>[], payloads: [] as WeakRef<object>[], kept: [] as unknown[] }));
function observe<T extends { payload: unknown }>(controlled: T): T {
  observed.refs.push(new WeakRef(controlled));
  if (typeof controlled.payload === "object" && controlled.payload !== null) observed.payloads.push(new WeakRef(controlled.payload));
  if (observed.keep) observed.kept.push(controlled);
  return controlled;
}
vi.mock("../report/reportRedactionProjector", async (importOriginal) => {
  const original = await importOriginal<typeof import("../report/reportRedactionProjector")>();
  return {
    ...original,
    controlReportPackageRequest: (...args: Parameters<typeof original.controlReportPackageRequest>) => observe(original.controlReportPackageRequest(...args))
  };
});
vi.mock("../redaction-controls/redactionExportControls", async (importOriginal) => {
  const original = await importOriginal<typeof import("../redaction-controls/redactionExportControls")>();
  return {
    ...original,
    controlUserReleasedReportPackageRequest: (...args: Parameters<typeof original.controlUserReleasedReportPackageRequest>) =>
      observe(original.controlUserReleasedReportPackageRequest(...args))
  };
});
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { collectGarbage } from "../../test/garbageCollection";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { RenderedReportPanel } from "../report/RenderedReportPanel";
import { useWorkspaceSession } from "./workspaceSession";

afterEach(() => {
  observed.keep = false;
  observed.refs = [];
  observed.payloads = [];
  observed.kept = [];
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const RECEIPT = {
  outcome: "saved", stage: "complete", code: "REPORT-PACKAGE-SAVED", container_file_name: "invented.opsproj", byte_count: 1,
  package_identity_sha256_hex: "a".repeat(64), container_sha256_hex: "b".repeat(64), members: [], replaced_existing: false, durability: "durable",
  redaction_route_id: "DREP-PACKAGE-SAVE-009", redaction_decision_count: 0, redaction_finding_count: 0, redaction_blocking_count: 0,
  local_first_route_id: "DREP-PACKAGE-SAVE-009", local_first_export_context: "local_private", local_first_storage_context: "local_private",
  local_first_action: "include_metadata_only", local_first_reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED", local_first_blocked: false,
  local_first_metadata_only: true, local_first_explicit_local_private_intent: true, selected_basename: "invented.opsproj",
  path_containment: "best_effort_non_adversarial", limitation: "Invented limitation.", warnings: [],
  user_release_recorded: true, user_release_model_document_sha256: "c".repeat(64)
};

let sequence = 0;
/** A solved Current preview-physics-2 session whose native package save answers only when the
 * test releases it. */
async function solvedSession() {
  const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return bytes;
  }, { jobIdPrefix: `unit-report-package-records:${++sequence}` });
  let releaseSave!: () => void;
  const saveHeld = new Promise<void>((resolve) => { releaseSave = resolve; });
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(model);
    if (command === "save_report_package") { await saveHeld; return RECEIPT; }
    return transport.invoke(command, args, options);
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const rendered = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(rendered.result.current.model.model).not.toBeNull());
  await waitFor(() => expect(rendered.result.current.model.modelHash).not.toBeNull());
  await act(async () => { await rendered.result.current.results.handleRun(); });
  await waitFor(() => expect(rendered.result.current.results.solveJob.state).toBe("completed"));
  expect(rendered.result.current.results.currentSolvedResult).not.toBeNull();
  return { ...rendered, releaseSave };
}

const saveCalls = () => invokeMock.mock.calls.filter(([command]) => command === "save_report_package");

/** The package summary line the report panel shows for `packageRedaction`. */
function packageSummaryLine(model: Parameters<typeof RenderedReportPanel>[0]["model"], packageRedaction: Parameters<typeof RenderedReportPanel>[0]["packageRedaction"]): string | null {
  const view = render(<RenderedReportPanel model={model} result={null} analysisRun={null} projectSummary={null} packageRedaction={packageRedaction} />);
  const line = screen.getByTestId("report-package-redaction-summary").textContent;
  view.unmount();
  return line;
}

describe("the report package saves keep only what they read of the redaction control", () => {
  it("keeps the local-private control's summary and blocked flag, with the same summary line, and none of its records", async () => {
    observed.keep = true;
    const { result: hook, releaseSave } = await solvedSession();
    act(() => hook.current.results.setReportPackagePrivateIntent(true));
    let saving!: Promise<void>;
    act(() => { saving = hook.current.results.handleSaveReportPackage(); });
    await waitFor(() => expect(saveCalls()).toHaveLength(1));
    expect(observed.kept).toHaveLength(1);
    const controlled = observed.kept[0] as ControlledRouteExport;
    expect(controlled.decisions.length).toBeGreaterThan(0);
    expect(controlled.findings.length).toBeGreaterThan(0);
    const stored = hook.current.results.reportPackageRedaction;
    expect(stored).not.toBeNull();
    expect(stored).not.toHaveProperty("decisions");
    expect(stored).not.toHaveProperty("findings");
    expect(stored).not.toHaveProperty("payload");
    expect(stored!.summary).toEqual(controlled.summary);
    expect(stored!.blocked).toBe(controlled.blocked);
    const model = hook.current.model.model!;
    const line = packageSummaryLine(model, stored);
    expect(line).toBe(packageSummaryLine(model, controlled));
    expect(line).toContain(`decisions=${controlled.decisions.length}; findings=${controlled.findings.length}`);
    await act(async () => { releaseSave(); await saving; });
    expect(hook.current.results.reportPackageRoute).toMatchObject({ route: "tauri_report_package_save" });
    expect(packageSummaryLine(model, hook.current.results.reportPackageRedaction)).toBe(line);
  });

  it("holds no local-private control result while the native save is awaited", async () => {
    const { result: hook, releaseSave } = await solvedSession();
    act(() => hook.current.results.setReportPackagePrivateIntent(true));
    let saving!: Promise<void>;
    act(() => { saving = hook.current.results.handleSaveReportPackage(); });
    await waitFor(() => expect(saveCalls()).toHaveLength(1));
    expect(observed.refs).toHaveLength(1);
    expect(observed.payloads).toHaveLength(1);
    await collectGarbage();
    expect(observed.refs[0].deref()).toBeUndefined();
    // Nor its controlled payload: what is sent is the prepared canonical text.
    expect(observed.payloads[0].deref()).toBeUndefined();
    expect(hook.current.results.reportPackageRedaction?.blocked).toBe(false);
    await act(async () => { releaseSave(); await saving; });
    expect(hook.current.results.reportPackageRoute).toMatchObject({ route: "tauri_report_package_save" });
  });

  it("holds no user-release control result while the native save is awaited", async () => {
    const { result: hook, releaseSave } = await solvedSession();
    await act(async () => { await hook.current.results.requestSaveShareableReportPackage(); });
    const request = hook.current.results.userReleaseRequest!;
    expect(request.refusal).toBeNull();
    act(() => { request.save({ userReleasedAt: "2026-11-09T00:00:00Z" }); });
    await waitFor(() => expect(saveCalls()).toHaveLength(1));
    expect(observed.refs).toHaveLength(1);
    expect(observed.payloads).toHaveLength(1);
    await collectGarbage();
    expect(observed.refs[0].deref()).toBeUndefined();
    // Nor its controlled payload: what is sent is the prepared canonical text.
    expect(observed.payloads[0].deref()).toBeUndefined();
    expect(hook.current.results.reportPackageRedaction).toBeNull();
    await act(async () => { releaseSave(); });
    await waitFor(() => expect(hook.current.results.reportPackageBusy).toBe(false));
    expect(hook.current.results.reportPackageRoute).toMatchObject({ route: "tauri_report_package_save" });
  });
});
