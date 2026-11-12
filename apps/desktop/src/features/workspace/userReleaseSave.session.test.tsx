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
// The real preparation, which a test may change after it runs (a returned refusal, arguments
// that cannot be encoded) or observe (the screened request it made).
type Preparation = Awaited<ReturnType<typeof import("../report/userReleasedReportPackageSave").prepareShareableReportPackageSave>>;
const preparation = vi.hoisted(() => ({ change: null as null | ((prepared: Preparation) => Preparation) }));
vi.mock("../report/userReleasedReportPackageSave", async (importOriginal) => {
  const original = await importOriginal<typeof import("../report/userReleasedReportPackageSave")>();
  return {
    ...original,
    prepareShareableReportPackageSave: async (input: Parameters<typeof original.prepareShareableReportPackageSave>[0]) => {
      const prepared = await original.prepareShareableReportPackageSave(input);
      return preparation.change ? preparation.change(prepared) : prepared;
    }
  };
});
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { canonicalSha256Hex } from "../../services/hashService";
import { useWorkspaceSession } from "./workspaceSession";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";
import { USER_RELEASE_STATEMENT } from "../report/reportPackageRequest";

afterEach(() => {
  preparation.change = null;
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

/** The native receipt's binding, distinct from every other digest of the session. */
const BINDING = "c".repeat(64);
const RECEIPT = { outcome: "saved", code: "REPORT-PACKAGE-SAVED", user_release_recorded: true, user_release_model_document_sha256: BINDING };
/** A native receipt as the session keeps it: without the record's binding (REQ-019). */
function withoutBinding<T extends { user_release_model_document_sha256?: unknown }>(receipt: T): Omit<T, "user_release_model_document_sha256"> {
  const { user_release_model_document_sha256: _binding, ...status } = receipt;
  return status;
}
const STATUS = withoutBinding(RECEIPT);

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
    expect(hook.current.results.reportPackageRoute).toEqual({ route: "tauri_report_package_save", receipt: STATUS });
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

// DEL-08-01 REQ-019, AC-010; DEL-02-05 AX-005: no outcome of a shareable save says a package was
// written or a user release made when none was, and an unknown outcome reads as unknown.
const NOT_WRITTEN = "Not saved: no shareable report package was written and no user release was made.";
const UNKNOWN = "It is not known whether the shareable report package was written.";
const TEMP_MAY_REMAIN = "The shareable report package was not saved to the selected file. A temporary file may remain beside it.";
const USER_RELEASED_AT = "2026-11-09T00:00:00Z";

/** Whether `value`, searched through the object graph it reaches (members, array elements, Map
 * and Set entries), holds anything of a user release record: its statement, its kind or the
 * time of this test's statement as a string, or its binding (a member named
 * `user_release_model_document_sha256` or `bound_to`, or the native receipt's binding value).
 * It does not examine functions, closures, private refs not reachable from `value`, WeakMaps
 * or DOM nodes. */
function holdsUserReleaseRecord(value: unknown, seen = new WeakSet<object>()): boolean {
  if (typeof value === "string") {
    return value.includes(USER_RELEASE_STATEMENT) || value.includes("user_release_of_own_project_data") || value.includes(USER_RELEASED_AT) || value.includes(BINDING);
  }
  if (!value || typeof value !== "object" || seen.has(value)) return false;
  seen.add(value);
  if (typeof Node !== "undefined" && value instanceof Node) return false;
  if (ArrayBuffer.isView(value) || value instanceof Promise) return false;
  if (!(value instanceof Map) && !(value instanceof Set) && ("user_release_model_document_sha256" in value || "bound_to" in value)) return true;
  const items: unknown[] = value instanceof Map ? [...value.keys(), ...value.values()]
    : value instanceof Set ? [...value]
    : Object.keys(value).map((key) => { try { return (value as Record<string, unknown>)[key]; } catch { return undefined; } });
  return items.some((item) => holdsUserReleaseRecord(item, seen));
}

describe("What a shareable save that writes nothing says (REQ-019, AC-010)", () => {
  /** The session with the native save answered by `answer`. */
  async function sessionAnswering(answer: () => Promise<unknown>) {
    const rendered = await solvedSession();
    const passThrough = invokeMock.getMockImplementation()!;
    invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) =>
      command === "save_report_package" ? answer() : passThrough(command, args, options));
    return rendered;
  }

  async function confirmOneSave(hook: { current: ReturnType<typeof useWorkspaceSession> }, sends = 1) {
    await act(async () => { await hook.current.results.requestSaveShareableReportPackage(); });
    await act(async () => { hook.current.results.userReleaseRequest!.save({ userReleasedAt: USER_RELEASED_AT }); });
    await waitFor(() => expect(hook.current.results.reportPackageRoute).not.toBeNull());
    await waitFor(() => expect(hook.current.results.reportPackageBusy).toBe(false));
    expect(saveCalls()).toHaveLength(sends);
    return hook.current.results.reportPackageRoute!;
  }

  function diagnostic(route: unknown): string {
    const read = route as { route: string; diagnostic: string };
    expect(read.route).toBe("redaction_blocked");
    return read.diagnostic;
  }

  it("a cancelled chooser's receipt records no user release", async () => {
    const cancelled = { ...RECEIPT, outcome: "cancelled", code: "REPORT-PACKAGE-SAVE-CANCELLED", byte_count: 0, durability: "not_applicable",
      selected_basename: "", user_release_recorded: false, user_release_model_document_sha256: "" };
    const { result: hook } = await sessionAnswering(async () => cancelled);
    expect(await confirmOneSave(hook)).toEqual({ route: "tauri_report_package_save", receipt: withoutBinding(cancelled) });
  });

  it.each([
    ["a coded refusal", { code: "REPORT-PACKAGE-SAVE-MODEL-UNBOUND", stage: "admit", message: "Invented refusal.", selected_basename: "", diagnostics: [] }],
    ["a writer error", { code: "REPORT-PACKAGE-RENAME-FAILED", stage: "rename", message: "Invented rename failure.", os_error_kind: "Other", selected_basename: "invented.opsproj", diagnostics: [] }]
  ])("%s says nothing was written, before its diagnostic", async (_name, error) => {
    const { result: hook } = await sessionAnswering(async () => { throw error; });
    expect(diagnostic(await confirmOneSave(hook))).toBe(`${NOT_WRITTEN} ${JSON.stringify(error)}`);
  });

  it("a writer error whose cleanup also failed says only that the selected file was not saved and a temporary file may remain", async () => {
    // The shape `report_package_save.rs`'s writer-error test serializes: the temporary file may
    // hold the whole package with its record, so nothing is said of a user release.
    const error = {
      code: "REPORT-PACKAGE-RENAME-FAILED", stage: "rename", message: "Invented rename failure.", os_error_kind: "Other", os_error_code: null,
      cleanup_code: "REPORT-PACKAGE-TEMP-CLEANUP-FAILED", cleanup_message: "Invented cleanup failure.", selected_basename: "invented.opsproj",
      path_containment: "best_effort_non_adversarial", limitation: "Invented limitation.", diagnostics: []
    };
    const { result: hook } = await sessionAnswering(async () => { throw error; });
    const text = diagnostic(await confirmOneSave(hook));
    expect(text).toBe(`${TEMP_MAY_REMAIN} ${JSON.stringify(error)}`);
    expect(text).not.toContain("no user release");
    expect(text).not.toContain("Not saved: no shareable");
  });

  it("arguments that cannot be encoded leave before nothing is sent: nothing was written", async () => {
    preparation.change = (prepared) => prepared.prepared.route !== "ready" ? prepared
      : { ...prepared, prepared: { ...prepared.prepared, args: { ...prepared.prepared.args, poison: BigInt(1) } as never } };
    const { result: hook } = await solvedSession();
    const text = diagnostic(await confirmOneSave(hook, 0));
    expect(text.startsWith(`${NOT_WRITTEN} `)).toBe(true);
    expect(text).not.toContain(UNKNOWN);
  });

  it("a refusal returned by the preparation says nothing was written, before its diagnostic", async () => {
    const refusal = "REPORT-PACKAGE-REDACTION-BLOCKED: 1 finding(s) prevent a shareable package.";
    preparation.change = (prepared) => ({ ...prepared, prepared: { route: "redaction_blocked", diagnostic: refusal } });
    const { result: hook } = await solvedSession();
    expect(diagnostic(await confirmOneSave(hook, 0))).toBe(`${NOT_WRITTEN} ${refusal}`);
  });

  it.each([
    ["a worker failure", { code: "REPORT-PACKAGE-SAVE-WORKER-FAILED", stage: "assemble", message: "REPORT-PACKAGE-SAVE-WORKER-FAILED: the save's worker did not complete; the destination may be unchanged.", diagnostics: [] }],
    ["an unreadable error", "invented unreadable failure"]
  ] as Array<[string, unknown]>)("%s says only that the outcome is not known", async (_name, error) => {
    const { result: hook } = await sessionAnswering(async () => { throw error; });
    const text = diagnostic(await confirmOneSave(hook));
    expect(text.startsWith(`${UNKNOWN} `)).toBe(true);
    expect(text).not.toContain("Not saved");
    expect(text).not.toContain("no user release");
    expect(text).not.toContain("user_release_recorded");
  });

  it("a new shareable attempt clears an earlier save's status, confirmed or cancelled", async () => {
    const { result: hook } = await solvedSession();
    expect(await confirmOneSave(hook)).toEqual({ route: "tauri_report_package_save", receipt: STATUS });
    await act(async () => { await hook.current.results.requestSaveShareableReportPackage(); });
    expect(hook.current.results.userReleaseRequest).not.toBeNull();
    expect(hook.current.results.reportPackageRoute).toBeNull();
    act(() => hook.current.results.userReleaseRequest!.cancel());
    expect(hook.current.results.reportPackageRoute).toBeNull();
    expect(saveCalls()).toHaveLength(1);
  });
});

// DEL-08-01 REQ-019 "Selection", AC-010: nothing of a user release stays in the session outside
// the report and its package, whatever the save's ending.
describe("No user release record stays in the session after a shareable save (REQ-019)", () => {
  /** A shareable save answered by `answer` (and its preparation changed by `change`), whose
   * screened request is checked to hold the record, so the search below is shown to find it. */
  async function endedSave(answer: () => Promise<unknown>, change: ((prepared: Preparation) => Preparation) | null = null) {
    let screened: unknown = null;
    preparation.change = (prepared) => { screened = prepared.controlled.payload; return change ? change(prepared) : prepared; };
    const rendered = await solvedSession();
    const passThrough = invokeMock.getMockImplementation()!;
    invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) =>
      command === "save_report_package" ? answer() : passThrough(command, args, options));
    const hook = rendered.result;
    await act(async () => { await hook.current.results.requestSaveShareableReportPackage(); });
    await act(async () => { hook.current.results.userReleaseRequest!.save({ userReleasedAt: USER_RELEASED_AT }); });
    await waitFor(() => expect(hook.current.results.reportPackageRoute).not.toBeNull());
    await waitFor(() => expect(hook.current.results.reportPackageBusy).toBe(false));
    expect(holdsUserReleaseRecord(screened), "the screened request holds the record").toBe(true);
    return hook;
  }

  function expectNoRecord(hook: { current: ReturnType<typeof useWorkspaceSession> }) {
    expect(hook.current.results.reportPackageRedaction).toBeNull();
    expect(hook.current.results.userReleaseRequest).toBeNull();
    expect(holdsUserReleaseRecord(hook.current), "a user release record in the session's state").toBe(false);
  }

  it("after a cancelled chooser", async () => {
    const cancelled = { ...RECEIPT, outcome: "cancelled", code: "REPORT-PACKAGE-SAVE-CANCELLED", user_release_recorded: false, user_release_model_document_sha256: "" };
    expectNoRecord(await endedSave(async () => cancelled));
  });

  it("after a refusal returned by the preparation", async () => {
    const hook = await endedSave(async () => RECEIPT, (prepared) => ({ ...prepared, prepared: { route: "redaction_blocked", diagnostic: "REPORT-PACKAGE-REDACTION-BLOCKED: invented." } }));
    expect(saveCalls()).toHaveLength(0);
    expectNoRecord(hook);
  });

  it("after a coded refusal of the native save", async () => {
    expectNoRecord(await endedSave(async () => { throw { code: "REPORT-PACKAGE-SAVE-MODEL-UNBOUND", stage: "admit", message: "Invented refusal.", diagnostics: [] }; }));
  });

  it("after a thrown failure", async () => {
    expectNoRecord(await endedSave(async () => { throw "invented unreadable failure"; }));
  });

  it.each([
    ["a durable save", RECEIPT],
    ["a save with durability not confirmed", { ...RECEIPT, code: "REPORT-PACKAGE-SAVED-DURABILITY-UNCERTAIN", durability: "saved_durability_uncertain" }]
  ])("after %s, the status keeps that a user release is recorded and not its binding", async (_name, receipt) => {
    const hook = await endedSave(async () => receipt);
    const route = hook.current.results.reportPackageRoute;
    expect(route).toEqual({ route: "tauri_report_package_save", receipt: withoutBinding(receipt) });
    expect(route?.route === "tauri_report_package_save" && "user_release_model_document_sha256" in route.receipt).toBe(false);
    expectNoRecord(hook);
  });

  it("opening the dialog clears an earlier local-private save's screened request and status", async () => {
    const { result: hook } = await solvedSession();
    act(() => hook.current.results.setReportPackagePrivateIntent(true));
    await act(async () => { await hook.current.results.handleSaveReportPackage(); });
    await waitFor(() => expect(hook.current.results.reportPackageRoute).not.toBeNull());
    expect(hook.current.results.reportPackageRedaction).not.toBeNull();
    await act(async () => { await hook.current.results.requestSaveShareableReportPackage(); });
    expect(hook.current.results.reportPackageRedaction).toBeNull();
    expect(hook.current.results.reportPackageRoute).toBeNull();
    act(() => hook.current.results.userReleaseRequest!.cancel());
    expect(hook.current.results.reportPackageRedaction).toBeNull();
  });
});
