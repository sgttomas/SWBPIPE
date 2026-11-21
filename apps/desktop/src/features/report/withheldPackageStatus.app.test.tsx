/** A report-package save says what happened even where the rendered report is withheld. With a
 * fitting result and a run record above the webview's limit, the rendered report's gate refuses
 * and its panel (which holds the package-save status) is not shown; a save started from the File
 * menu (the shareable save through its release dialog, or the local-private save) is refused for
 * the record, and its refusal is shown once in the gate's place in the Report section, and kept as
 * the session's outcome in the report output strip (issue 508). With the panel shown, the
 * receipt is the panel's, and the strip keeps the outcome in its own words. The whole App renders; the solve is the native job
 * transport replay of committed producer bytes (unit transport replay; NOT a native UI
 * qualification witness). All data are invented. */
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { App } from "../../App";
import { getLocalStorageCapability } from "../../services/projectService";
import * as nativeResult from "../../services/nativeResult";
import { RESULT_DETAIL_WEBVIEW_LIMIT_BYTES } from "../../services/nativeResult";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { createNativeMechanicsReplay, REPLAY_COMMANDS } from "../../test/nativeMechanicsReplay";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

/** The record-size refusal as the save's status shows it: its code and the record's sizes. */
const RECORD_REFUSAL = `RESULT-DETAIL-WEBVIEW-LIMIT (record_bytes=${RESULT_DETAIL_WEBVIEW_LIMIT_BYTES + 1}, limit_bytes=${RESULT_DETAIL_WEBVIEW_LIMIT_BYTES})`;

/** A saved shareable package's receipt, as the native save answers it. */
const SAVED_RECEIPT = {
  outcome: "saved", stage: "complete", code: "REPORT-PACKAGE-SAVED", container_file_name: "invented.opsproj", byte_count: 1,
  package_identity_sha256_hex: "a".repeat(64), container_sha256_hex: "b".repeat(64), members: [], replaced_existing: false, durability: "durable",
  redaction_route_id: "DREP-PACKAGE-USER-RELEASE-010", redaction_decision_count: 0, redaction_finding_count: 0, redaction_blocking_count: 0,
  local_first_route_id: "DREP-PACKAGE-USER-RELEASE-010", local_first_export_context: "user_released_report", local_first_storage_context: "private_report",
  local_first_action: "include_metadata_only", local_first_reason_code: "USER_RELEASE_METADATA_ALLOWED", local_first_blocked: false,
  local_first_metadata_only: true, local_first_explicit_local_private_intent: false, selected_basename: "invented-shared.opsproj",
  path_containment: "best_effort_non_adversarial", limitation: "Invented limitation.",
  user_release_recorded: true, user_release_model_document_sha256: "c".repeat(64)
};

async function solvedApp(options: { recordAboveLimit: boolean }) {
  const storage = await getLocalStorageCapability();
  const knowledge = await loadDesignKnowledge();
  const mechanics = createNativeMechanicsReplay({ profile: "preview" });
  invokeMock.mockImplementation((command: string, args?: unknown, invokeOptions?: unknown) => {
    if (command === "get_local_storage_capability") return Promise.resolve(storage);
    if (command === "load_design_knowledge") return Promise.resolve(knowledge);
    if (command === "sync_native_shell_state") return Promise.resolve(null);
    if (command === "save_report_package") return Promise.resolve(structuredClone(SAVED_RECEIPT));
    if (REPLAY_COMMANDS.includes(command)) return mechanics.invoke(command, args, invokeOptions);
    return Promise.reject(new Error(`WITHHELD_STATUS_COMMAND_UNSUPPORTED: ${command}`));
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  if (options.recordAboveLimit) {
    const preview = nativeResult.nativeRunPreview;
    vi.spyOn(nativeResult, "nativeRunPreview").mockImplementation((run) => ({ ...preview(run), record_bytes: RESULT_DETAIL_WEBVIEW_LIMIT_BYTES + 1 }));
  }
  render(<App />);
  await screen.findByTestId("workspace-toolbar");
  nativeMenuCommand("view.section.solve");
  const run = within(screen.getByTestId("workspace-section-solve")).getByTestId("run-mechanics-preview");
  await waitFor(() => expect(run).toBeEnabled());
  fireEvent.click(run);
  await waitFor(() => expect(screen.getByTestId("status-pill-mechanics")).toHaveAttribute("title", "MECHANICS_SOLVED"), { timeout: 30_000 });
  await settle();
  nativeMenuCommand("view.section.report");
  const report = screen.getByTestId("workspace-section-report");
  const copies = () => mechanics.commands().filter((command) => command === "read_preview_mechanics_job_rows_all").length;
  const sent = (command: string) => invokeMock.mock.calls.some(([name]) => name === command);
  return { report, copies, sent };
}

function nativeMenuCommand(command: string) {
  act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command })); });
}
async function settle() {
  await act(async () => { await new Promise((resolve) => setTimeout(resolve, 50)); });
}

/** The save's refusal on screen: once in the Report section, announced by its status there, and
 * once more as the session's kept outcome in the report output strip (DEL-08-01 REQ-022, issue
 * 508), announced by that entry's status. `strip` is the strip's line for it. */
async function expectWithheldRefusal(report: HTMLElement, strip: string) {
  const shown = await within(report).findByText((text) => text.includes(RECORD_REFUSAL));
  expect(within(report).getAllByText((text) => text.includes(RECORD_REFUSAL))).toEqual([shown]);
  expect(shown).toHaveAttribute("data-testid", "report-package-save-status");
  const outcomes = screen.getByRole("region", { name: "Report output outcomes" });
  const kept = within(outcomes).getByRole("status");
  expect(kept.querySelector("p")?.textContent).toBe(strip);
  expect(screen.getAllByText((text) => text.includes(RECORD_REFUSAL))).toHaveLength(2);
  expect(screen.getAllByRole("status").filter((status) => status.textContent?.includes(RECORD_REFUSAL))).toEqual([kept, shown]);
  expect(within(report).getByTestId("c2-result-withheld")).toContainElement(shown);
  // It stands below the rendered report's own refusal, in the panel's place.
  expect(within(report).getAllByTestId("c2-result-refused").length).toBeGreaterThan(0);
  expect(within(report).queryByTestId("rendered-report-panel")).toBeNull();
  return shown;
}

describe("a report-package save started while the rendered report is withheld for its record", () => {
  it("File > Save Shareable Report Package…: confirmed, its refusal is shown once in the Report section", async () => {
    const { report, copies, sent } = await solvedApp({ recordAboveLimit: true });
    expect(await within(report).findAllByTestId("c2-result-refused")).not.toHaveLength(0);
    expect(within(report).queryByTestId("report-package-save-status")).toBeNull();
    nativeMenuCommand("file.save-shareable-report-package");
    await screen.findByTestId("user-release-dialog");
    fireEvent.click(screen.getByTestId("user-release-statement"));
    fireEvent.click(screen.getByTestId("user-release-confirm"));
    const shown = await expectWithheldRefusal(report,
      `Shareable report package not saved: ${RECORD_REFUSAL}: the run record is above the size the webview can hold whole. Nothing was sent.`);
    expect(shown.textContent).toBe(`Not saved: no shareable report package was written and no user release was made. ${RECORD_REFUSAL}: the run record is above the size the webview can hold whole.`);
    expect(copies()).toBe(0);
    expect(sent("save_report_package")).toBe(false);
  }, 90_000);

  it("File > Save Report Package…: its refusal is shown once in the Report section", async () => {
    const { report, copies, sent } = await solvedApp({ recordAboveLimit: true });
    expect(await within(report).findAllByTestId("c2-result-refused")).not.toHaveLength(0);
    nativeMenuCommand("file.save-report-package");
    const shown = await expectWithheldRefusal(report,
      `Report package not saved: ${RECORD_REFUSAL}: the run record is above the size the webview can hold whole. Nothing was sent.`);
    expect(shown.textContent).toBe(`${RECORD_REFUSAL}: the run record is above the size the webview can hold whole.`);
    expect(copies()).toBe(0);
    expect(sent("save_report_package")).toBe(false);
  }, 90_000);
});

describe("with the rendered report shown", () => {
  it("a saved package's receipt is the panel's, once, in the panel's own words, and the gate adds none", async () => {
    const { report, sent } = await solvedApp({ recordAboveLimit: false });
    const panel = await within(report).findByTestId("rendered-report-panel");
    nativeMenuCommand("file.save-shareable-report-package");
    await screen.findByTestId("user-release-dialog");
    fireEvent.click(screen.getByTestId("user-release-statement"));
    fireEvent.click(screen.getByTestId("user-release-confirm"));
    const status = await within(panel).findByTestId("report-package-save-status");
    expect(sent("save_report_package")).toBe(true);
    expect(screen.getAllByTestId("report-package-save-status")).toEqual([status]);
    expect(screen.queryByTestId("c2-result-withheld")).toBeNull();
    // The words a person sees, line by line; the panel's receipt keeps the element it had (no role).
    expect([...status.children].map((line) => line.textContent)).toEqual([
      "outcome=saved; code=REPORT-PACKAGE-SAVED; file=invented-shared.opsproj",
      `bytes=1; container_sha256=${"b".repeat(64)}; package_identity=${"a".repeat(64)}`,
      "replacement=false; durability=durable; redaction_route=DREP-PACKAGE-USER-RELEASE-010",
      "path_containment=best_effort_non_adversarial; limitation=Invented limitation.",
      "user_release_recorded=true"
    ]);
    expect(status).not.toHaveAttribute("role");
    // The session keeps the outcome apart from the panel, in the strip's own words (issue 508).
    const kept = within(screen.getByRole("region", { name: "Report output outcomes" })).getByRole("status");
    expect(kept.querySelector("p")?.textContent).toBe("Shareable report package saved: invented-shared.opsproj");
  }, 90_000);
});
