/** Issue 508, DEL-08-01 REQ-022: a report package save's outcome (Save Report Package… and Save
 * Shareable Report Package…) is kept in the session's report output outcomes, shown by
 * `ReportOutputStatus` apart from the panel, until it is dismissed: settled from the native reply
 * whatever has happened to the workspace since the request was sent (a model edit, a solve start,
 * a reference inspection, opening another project), unaffected by the same changes once shown,
 * and stated as saved, saved without confirmed durability, cancelled, refused, failed or unknown.
 * A "not requested" status set while a solve runs does not stand beside that solve's result.
 * Each test reads the words a person sees and the role announced. The session is the actual
 * workspace session; the solve is a unit transport replay of committed producer bytes through
 * mocked IPC, and every native save reply is invented (NOT a native UI qualification witness). */
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { createLocalProject, getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import * as requestModule from "../report/reportPackageRequest";
import { ReportOutputStatus } from "../report/ReportOutputStatus";
import { ReportPackageSaveStatus } from "../report/RenderedReportPanel";
import type { PreviewModel } from "../../types";
import { buildGridOperationIntent, type GridColumn } from "./table/modelTableAdapter";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

type Kind = "report_package" | "shareable_report_package";
const KINDS: readonly [Kind, string][] = [["report_package", "Report package"], ["shareable_report_package", "Shareable report package"]];

function deferred<T>() {
  let resolve!: (value: T) => void, reject!: (error: unknown) => void;
  const promise = new Promise<T>((res, rej) => { resolve = res; reject = rej; });
  return { promise, resolve, reject };
}

/** An invented native receipt of a saved package, for either route. */
function receipt(kind: Kind, members: Record<string, unknown> = {}) {
  const route = kind === "report_package" ? "DREP-PACKAGE-SAVE-009" : "DREP-PACKAGE-USER-RELEASE-010";
  return {
    outcome: "saved", stage: "complete", code: "REPORT-PACKAGE-SAVED", container_file_name: "invented-suggested.opsproj", byte_count: 4096,
    package_identity_sha256_hex: "a".repeat(64), container_sha256_hex: "b".repeat(64), members: [], replaced_existing: false, durability: "durable",
    redaction_route_id: route, redaction_decision_count: 0, redaction_finding_count: 0, redaction_blocking_count: 0,
    local_first_route_id: route, local_first_export_context: kind === "report_package" ? "local_private" : "user_released_report",
    local_first_storage_context: kind === "report_package" ? "local_private" : "private_report", local_first_action: "include_metadata_only",
    local_first_reason_code: kind === "report_package" ? "PRIVATE_LOCAL_METADATA_ALLOWED" : "USER_RELEASE_METADATA_ALLOWED", local_first_blocked: false,
    local_first_metadata_only: true, local_first_explicit_local_private_intent: kind === "report_package", selected_basename: "invented.opsproj",
    path_containment: "best_effort_non_adversarial", limitation: "Invented limitation.", warnings: [],
    ...(kind === "shareable_report_package" ? { user_release_recorded: true, user_release_model_document_sha256: "c".repeat(64) } : {}),
    ...members
  };
}

let jobSequence = 0;
/** A solved Current session, rendered with the outcome strip and the package status as App
 * shows them. `save` answers `save_report_package`; `solve` answers each solve after the first. */
async function solvedSession(save: (args: Record<string, unknown>) => Promise<unknown>, solve?: () => Promise<void>) {
  const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  let solves = 0;
  const transport = createSolveJobTransport(async (request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    if (++solves > 1 && solve) await solve();
    return bytes;
  }, { jobIdPrefix: `unit-report-package-outcomes:${++jobSequence}` });
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(model);
    if (command === "save_report_package") return save(args as Record<string, unknown>);
    if (transport.handles(command)) return transport.invoke(command, args, options);
    throw new Error(`REPORT_PACKAGE_OUTCOMES_COMMAND_UNSUPPORTED: ${command}`);
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  let current!: WorkspaceSession;
  function Harness() {
    current = useWorkspaceSession();
    const results = current.results;
    return (
      <>
        <ReportOutputStatus outcomes={results.reportOutputs} entries={results.reportOutputEntries} currentSequence={results.reportSourceSequence} />
        <section aria-label="Report section">{results.reportPackageRoute ? <ReportPackageSaveStatus route={results.reportPackageRoute} announce /> : null}</section>
      </>
    );
  }
  render(<Harness />);
  const state = () => current;
  await waitFor(() => expect(state().model.model).not.toBeNull());
  await waitFor(() => expect(state().model.modelHash).not.toBeNull());
  await act(async () => { await state().results.handleRun(); });
  await waitFor(() => expect(state().results.solveJob.state).toBe("completed"));
  expect(state().results.currentSolvedResult).not.toBeNull();
  act(() => state().results.setReportPackagePrivateIntent(true));
  // What the report was prepared from, as the session holds it (never a hardcoded handle).
  const basis = { model: state().model.model!.project.name, run: state().results.analysisRun!.analysis_run.run_id };
  return { state, basis };
}

const saveCalls = () => invokeMock.mock.calls.filter(([command]) => command === "save_report_package");

/** Starts one package save of `kind` as a person does: the local-private save directly, the
 * shareable save through its dialog's confirmation. */
async function startSave(kind: Kind, state: () => WorkspaceSession) {
  if (kind === "report_package") {
    act(() => { void state().results.handleSaveReportPackage(); });
    return;
  }
  await act(async () => { await state().results.requestSaveShareableReportPackage(); });
  const request = state().results.userReleaseRequest;
  expect(request?.refusal ?? null).toBeNull();
  act(() => request!.save({ userReleasedAt: "2026-11-21T00:00:00Z" }));
}

/** The outcome strip's announced entries, in order. */
function outcomeStatuses(): HTMLElement[] {
  const strip = screen.queryByRole("region", { name: "Report output outcomes" });
  return strip ? within(strip).queryAllByRole("status") : [];
}
/** The line of each announced entry (its first paragraph), in order. */
const outcomeLines = () => outcomeStatuses().map((status) => status.querySelector("p")?.textContent ?? "");

function coordinateEdit(model: PreviewModel) {
  const target = model.nodes[0];
  const column: GridColumn = { key: "y", label: "Y", fieldPath: "position.y", objectType: "Node", changeKind: "set_field", dimension: "length", sourceNote: "entered coordinate", unit: () => model.project.units.length, value: () => String(target.position.y) };
  return buildGridOperationIntent({ column, model, row: { id: target.id, label: target.label, type: "node", raw: target, searchText: "" }, sequence: 1, value: String(target.position.y + 1.5), interaction: "cell" });
}

const OTHER_PROJECT_ID = "invented-other-project";
/** Stores another project in the browser preview's store, to be opened later. */
async function storeOtherProject(state: () => WorkspaceSession) {
  const model = structuredClone(state().model.model!);
  model.project = { ...model.project, id: OTHER_PROJECT_ID, name: "Invented other project" };
  const internals = (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
  try { await createLocalProject(model); } finally { (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = internals; }
}

type Change = "a model edit" | "a solve start" | "a reference inspection" | "opening another project";
const CHANGES: readonly Change[] = ["a model edit", "a solve start", "a reference inspection", "opening another project"];

/** Makes `change` in the workspace and waits until the workspace has moved on. */
async function makeChange(change: Change, state: () => WorkspaceSession) {
  const before = state().results.reportSourceSequence;
  switch (change) {
    case "a model edit": {
      // The edit runs in the browser engine (the wasm operation applier), as a person's edit is
      // applied; the native save's reply is still outstanding meanwhile.
      const internals = (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
      delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
      let applied: unknown;
      try {
        await act(async () => { applied = await state().operations.handleApplyIntent(coordinateEdit(state().model.model!)); });
      } finally { (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = internals; }
      expect(applied, JSON.stringify(state().operations.operationOutcomes)).toBe(true);
      break;
    }
    case "a solve start":
      await act(async () => { await state().results.handleRun(); });
      break;
    case "a reference inspection":
      // The inspection retires the current result at once; the bundled reference's own load is
      // not answered here, which is not what is observed.
      await act(async () => { await state().results.handleInspectBundledReference().catch(() => undefined); });
      break;
    case "opening another project":
      // The browser preview's open path (no native runtime), from its store.
      delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
      await act(async () => { await state().project.handleOpenProject(OTHER_PROJECT_ID); });
      expect(state().model.model?.project.id).toBe(OTHER_PROJECT_ID);
      break;
  }
  await waitFor(() => expect(state().results.reportSourceSequence).not.toBe(before));
}

const earlier = (basis: { model: string; run: string }) =>
  ` It is of an earlier run: ${basis.model}, run ${basis.run}. The model or its results have changed since.`;

describe.each(KINDS)("%s: the outcome is kept whatever happens to the workspace", (kind, name) => {
  it.each(CHANGES)("published natively, then %s before the reply: the entry is there and says what was saved", async (change) => {
    const reply = deferred<unknown>();
    const { state, basis } = await solvedSession(() => reply.promise);
    if (change === "opening another project") await storeOtherProject(state);
    await startSave(kind, state);
    await waitFor(() => expect(saveCalls()).toHaveLength(1));
    // Sent: its pending line is announced, and it offers no Dismiss.
    await waitFor(() => expect(outcomeLines()).toEqual([`Saving ${name.toLowerCase()}: choose where to save it.`]));
    expect(within(outcomeStatuses()[0]).queryByRole("button", { name: "Dismiss" })).toBeNull();
    await makeChange(change, state);
    await act(async () => { reply.resolve(receipt(kind)); await Promise.resolve(); });
    await waitFor(() => expect(outcomeLines()).toEqual([`${name} saved: invented.opsproj.${earlier(basis)}`]));
    expect(within(outcomeStatuses()[0]).getByRole("button", { name: "Dismiss" })).toBeInTheDocument();
  }, 90_000);

  it.each(CHANGES)("shown, then %s: the entry stays, now saying the workspace has changed", async (change) => {
    const { state, basis } = await solvedSession(async () => receipt(kind));
    if (change === "opening another project") await storeOtherProject(state);
    await startSave(kind, state);
    await waitFor(() => expect(outcomeLines()).toEqual([`${name} saved: invented.opsproj`]));
    await makeChange(change, state);
    expect(outcomeLines()).toEqual([`${name} saved: invented.opsproj.${earlier(basis)}`]);
  }, 90_000);
});

/** A rejection as the native command gives it (`ReportPackageSaveCommandError`). */
const commandError = (members: Record<string, unknown>) => ({
  os_error_kind: null, os_error_code: null, cleanup_code: null, cleanup_message: null, selected_basename: "",
  path_containment: "best_effort_non_adversarial", limitation: "Invented limitation.", diagnostics: [], ...members
});

describe.each(KINDS)("%s: each outcome is stated as such", (kind, name) => {
  const cases: readonly [string, () => Promise<unknown>, string][] = [
    ["durability not confirmed", async () => receipt(kind, { durability: "saved_durability_uncertain", code: "REPORT-PACKAGE-SAVED-DURABILITY-UNCERTAIN" }),
      `${name} saved: invented.opsproj; the system did not confirm the file is safely on disk.`],
    ["cancelled at the chooser", async () => receipt(kind, { outcome: "cancelled", stage: "picker", code: "REPORT-PACKAGE-SAVE-CANCELLED", selected_basename: "" }),
      `${name} not saved: cancelled.`],
    ["refused, with its code", () => Promise.reject(commandError({ code: "REPORT-PACKAGE-PICKER-PATH-INVALID", stage: "assemble", message: "REPORT-PACKAGE-PICKER-PATH-INVALID: the chosen path is not an invented file path" })),
      `${name} not saved: the chosen path is not an invented file path (REPORT-PACKAGE-PICKER-PATH-INVALID).`],
    ["failed, its temporary file not removed", () => Promise.reject(commandError({ code: "REPORT-PACKAGE-WRITE-FAILED", stage: "write", message: "invented: no space left on device", cleanup_code: "REPORT-PACKAGE-TEMP-CLEANUP-FAILED", selected_basename: "invented.opsproj" })),
      `${name} not saved: invented: no space left on device (REPORT-PACKAGE-WRITE-FAILED). A temporary file may remain beside invented.opsproj.`],
    ["no native answer after it was sent", () => Promise.reject(new Error("invented: the native channel closed")),
      `${name}: outcome unknown. Check the file you selected.`],
    ["the native save's worker did not complete", () => Promise.reject(commandError({ code: "REPORT-PACKAGE-SAVE-WORKER-FAILED", stage: "assemble", message: "REPORT-PACKAGE-SAVE-WORKER-FAILED: the save's worker did not complete; the destination may be unchanged.", selected_basename: "invented.opsproj" })),
      `${name}: outcome unknown. Check invented.opsproj.`]
  ];
  it.each(cases)("%s", async (_case, answer, line) => {
    const { state } = await solvedSession(answer);
    await startSave(kind, state);
    await waitFor(() => expect(outcomeLines()).toEqual([line]));
    if (line.includes("outcome unknown")) {
      // An unknown outcome never reads as nothing written.
      expect(outcomeStatuses()[0].textContent).not.toMatch(/not saved|nothing was (written|sent)|was not written/i);
    }
  }, 90_000);
});

describe("Save Report Package…: what the entry shows and how it goes", () => {
  it("shows the receipt's warnings with the entry and its figures behind a closed Details", async () => {
    const warnings = [{ code: "MODEL-LIBRARY-REF-UNRESOLVED", path: "/model/materials/0", message: "invented: the library reference could not be resolved." }];
    const { state, basis } = await solvedSession(async () => receipt("report_package", { warnings, replaced_existing: true }));
    await startSave("report_package", state);
    await waitFor(() => expect(outcomeLines()).toEqual(["Report package saved: invented.opsproj"]));
    const status = outcomeStatuses()[0];
    expect(within(within(status).getByRole("list", { name: "Warnings" })).getAllByRole("listitem").map((item) => item.textContent)).toEqual([
      "Warning: MODEL-LIBRARY-REF-UNRESOLVED at /model/materials/0: invented: the library reference could not be resolved."
    ]);
    const details = status.querySelector("details")!;
    expect(details.open).toBe(false);
    expect(within(details).getByText("Details").tagName).toBe("SUMMARY");
    expect([...details.querySelectorAll("li")].map((item) => item.textContent)).toEqual([
      `Report basis: model ${basis.model}, run ${basis.run}.`,
      `4096 bytes; container SHA-256 ${"b".repeat(64)}; package identity SHA-256 ${"a".repeat(64)}.`,
      "It replaced the file that was there.",
      "The system confirmed the file is durable on disk.",
      "Receipt code: REPORT-PACKAGE-SAVED.",
      "Limitation: Invented limitation."
    ]);
  }, 90_000);

  it("Dismiss removes that entry and no other", async () => {
    let saves = 0;
    const { state } = await solvedSession(async () => receipt("report_package", { selected_basename: `invented-${++saves}.opsproj` }));
    await startSave("report_package", state);
    await waitFor(() => expect(outcomeLines()).toEqual(["Report package saved: invented-1.opsproj"]));
    await startSave("report_package", state);
    await waitFor(() => expect(outcomeLines()).toEqual(["Report package saved: invented-1.opsproj", "Report package saved: invented-2.opsproj"]));
    fireEvent.click(within(outcomeStatuses()[0]).getByRole("button", { name: "Dismiss" }));
    expect(outcomeLines()).toEqual(["Report package saved: invented-2.opsproj"]);
  }, 90_000);

  it("a save whose basis changes before its request is sent is recorded as not sent", async () => {
    const { state, basis } = await solvedSession(async () => receipt("report_package"));
    const gate = deferred<void>();
    const real = requestModule.buildReportPackageRequest;
    vi.spyOn(requestModule, "buildReportPackageRequest").mockImplementation(async (input) => {
      const request = await real(input);
      await gate.promise;
      return request;
    });
    await startSave("report_package", state);
    await waitFor(() => expect(state().results.reportPackageBusy).toBe(true));
    act(() => state().results.setReportComposition({ composition_version: "1.0.0", sections: [] }));
    await act(async () => { gate.resolve(); });
    await waitFor(() => expect(outcomeLines()).toEqual([
      `Report package not saved: the workspace changed before the request was sent (REPORT-REQUEST-RETIRED). Nothing was sent.${earlier(basis)}`
    ]));
    expect(saveCalls()).toHaveLength(0);
  }, 90_000);
});

describe("a \"not requested\" status (issue 508, comment)", () => {
  it("set while a solve runs, does not remain beside that solve's result", async () => {
    const held = deferred<void>();
    const { state } = await solvedSession(async () => receipt("report_package"), () => held.promise);
    let solving!: Promise<void>;
    act(() => { solving = state().results.handleRun(); });
    await waitFor(() => expect(state().results.running).toBe(true));
    await act(async () => { await state().results.handleSaveReportPackage(); });
    const refusal = "Not saved: a solve was running. Save the report package when it has finished.";
    const section = screen.getByRole("region", { name: "Report section" });
    expect(within(section).getByRole("status")).toHaveTextContent(refusal);
    await act(async () => { held.resolve(); await solving; });
    await waitFor(() => expect(state().results.solveJob.state).toBe("completed"));
    expect(state().results.currentSolvedResult).not.toBeNull();
    expect(within(section).queryByText(refusal)).toBeNull();
    expect(saveCalls()).toHaveLength(0);
  }, 90_000);
});
