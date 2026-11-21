// in the app, a fresh solve's first paint
// copies nothing whole. The result stays held natively: no take, no C2 full copy and no full record
// built by the TypeScript builder, with the
// Solve section open (its rule checks wait for a request) and with the Results and Report sections
// opened earlier and now dormant. A C2 consumer copies the result when the person asks or opens its
// section, once per job; a second solve releases the first job and copies nothing until asked
// again. The solve is a unit transport replay of captured producer bytes for the invented preview
// model, answered by the actual Rust commands' recorded replies; NOT a native UI qualification witness.
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "./App";
import { getLocalStorageCapability } from "./services/projectService";
import * as previewService from "./services/previewService";
import * as nativeResult from "./services/nativeResult";
import { RESULT_DETAIL_WEBVIEW_LIMIT_BYTES } from "./services/nativeResult";
import { loadDesignKnowledge } from "./services/previewService";
import { createNativeMechanicsReplay, REPLAY_COMMANDS } from "./test/nativeMechanicsReplay";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

async function installPreviewReplay() {
  const storage = await getLocalStorageCapability();
  const knowledge = await loadDesignKnowledge();
  const mechanics = createNativeMechanicsReplay({ profile: "preview" });
  invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return Promise.resolve(storage);
    if (command === "load_design_knowledge") return Promise.resolve(knowledge);
    if (command === "sync_native_shell_state") return Promise.resolve(null);
    if (REPLAY_COMMANDS.includes(command)) return mechanics.invoke(command, args, options);
    return Promise.reject(new Error(`NATIVE_APP_REPLAY_COMMAND_UNSUPPORTED: ${command}`));
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  return mechanics;
}

function nativeMenuCommand(command: string) {
  act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command })); });
}
async function settle() {
  await act(async () => { await new Promise((resolve) => setTimeout(resolve, 50)); });
}
async function solve() {
  nativeMenuCommand("view.section.solve");
  const run = within(screen.getByTestId("workspace-section-solve")).getByTestId("run-mechanics-preview");
  await waitFor(() => expect(run).toBeEnabled());
  fireEvent.click(run);
  await waitFor(() => expect(screen.getByTestId("status-pill-mechanics")).toHaveAttribute("title", "MECHANICS_SOLVED"), { timeout: 30_000 });
  await settle();
}

describe("a native result's first paint in the app", () => {
  it("takes nothing and copies nothing whole until a C2 consumer is asked for or opened, then copies once per job", async () => {
    const mechanics = await installPreviewReplay();
    const sent = (command: string) => mechanics.commands().filter((name) => name === command).length;
    const fullRecordBuilds = vi.spyOn(previewService, "buildAnalysisRunPreview");
    render(<App />);
    await screen.findByTestId("workspace-toolbar");
    // Sections opened before any result stay mounted, dormant, when the person moves on.
    nativeMenuCommand("view.section.results");
    nativeMenuCommand("view.section.report");

    await solve();
    expect(sent("read_preview_mechanics_job_summary")).toBeGreaterThan(0);
    expect(sent("take_preview_mechanics_job_result")).toBe(0);
    expect(sent("read_preview_mechanics_job_rows_all")).toBe(0);
    expect(fullRecordBuilds).not.toHaveBeenCalled();
    // The Solve section is open: its rule checks offer to read the full rows, and wait.
    const solveSection = screen.getByTestId("workspace-section-solve");
    expect(within(solveSection).queryByTestId("rule-check-load-demo")).toBeNull();
    fireEvent.click(within(solveSection).getByTestId("c2-result-load"));
    expect(await within(solveSection).findByTestId("rule-check-load-demo")).toBeInTheDocument();
    expect(sent("read_preview_mechanics_job_rows_all")).toBe(1);
    // Rule checks are given no run record, so none is built for them.
    expect(fullRecordBuilds).not.toHaveBeenCalled();

    // Opening the Report section reads the same copy: none is made again for the job.
    nativeMenuCommand("view.section.report");
    expect(await within(screen.getByTestId("workspace-section-report")).findByTestId("report-panel")).toBeInTheDocument();
    await settle();
    expect(sent("read_preview_mechanics_job_rows_all")).toBe(1);
    // ... and the report, given the record, gets its full record, built once on that copy and shared.
    expect(fullRecordBuilds).toHaveBeenCalledTimes(1);

    // A second solve releases the first job; with Report dormant again, nothing is copied.
    await solve();
    await waitFor(() => expect(mechanics.disposed()).toHaveLength(1));
    expect(mechanics.nativeHeld()).toHaveLength(1);
    expect(sent("read_preview_mechanics_job_rows_all")).toBe(1);
    expect(fullRecordBuilds).toHaveBeenCalledTimes(1);
    expect(within(screen.getByTestId("workspace-section-solve")).getByTestId("c2-result-load")).toBeInTheDocument();
    expect(sent("take_preview_mechanics_job_result")).toBe(0);
  }, 90_000);

  it("refuses for a run record above the limit only the outputs given it: comparisons and rule checks load, the report shows the record's sizes", async () => {
    const mechanics = await installPreviewReplay();
    const sent = (command: string) => mechanics.commands().filter((name) => name === command).length;
    const fullRecordBuilds = vi.spyOn(previewService, "buildAnalysisRunPreview");
    const preview = nativeResult.nativeRunPreview;
    vi.spyOn(nativeResult, "nativeRunPreview").mockImplementation((run) => ({ ...preview(run), record_bytes: RESULT_DETAIL_WEBVIEW_LIMIT_BYTES + 1 }));
    render(<App />);
    await screen.findByTestId("workspace-toolbar");
    await solve();

    const solveSection = screen.getByTestId("workspace-section-solve");
    fireEvent.click(within(solveSection).getByTestId("c2-result-load"));
    expect(await within(solveSection).findByTestId("rule-check-load-demo")).toBeInTheDocument();
    expect(within(solveSection).queryByTestId("c2-result-refused")).toBeNull();

    nativeMenuCommand("view.section.results");
    const resultsSection = screen.getByTestId("workspace-section-results");
    fireEvent.click(within(resultsSection).getByTestId("c2-result-load"));
    // The comparison panel is shown on the full copy, with what this run gives within it: its
    // packaged basis, and no combination with result rows to compare.
    const comparison = await within(resultsSection).findByTestId("comparison-panel");
    expect(await within(comparison).findByTestId("comparison-within-run")).toBeInTheDocument();
    expect(within(comparison).getByTestId("comparison-package-basis-text")).toHaveTextContent("The packages carry no comparison of load case L-100 with combination C-OPER-ALT: load case L-100 and combination C-OPER-ALT have no results in this run.");
    expect(within(comparison).getByTestId("comparison-unavailable")).toHaveTextContent("No combination in this run has result rows, so there is nothing to compare within this run.");
    expect(within(resultsSection).queryByTestId("c2-result-refused")).toBeNull();
    expect(within(resultsSection).queryByTestId("c2-result-failed")).toBeNull();

    nativeMenuCommand("view.section.report");
    const reportSection = screen.getByTestId("workspace-section-report");
    const refusals = await within(reportSection).findAllByTestId("c2-result-refused");
    for (const refusal of refusals) {
      expect(refusal).toHaveTextContent(`RESULT-DETAIL-WEBVIEW-LIMIT: This run record is ${RESULT_DETAIL_WEBVIEW_LIMIT_BYTES + 1} bytes`);
      expect(refusal).toHaveTextContent(`above the ${RESULT_DETAIL_WEBVIEW_LIMIT_BYTES} bytes (64 MiB) the webview can hold whole`);
    }
    expect(within(reportSection).queryByTestId("report-panel")).toBeNull();
    expect(fullRecordBuilds).not.toHaveBeenCalled();
    expect(sent("read_preview_mechanics_job_rows_all")).toBe(1);
  }, 90_000);

  it("refuses a result above its own limit for every output, with or without the record", async () => {
    const mechanics = await installPreviewReplay();
    const forward = invokeMock.getMockImplementation()!;
    invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => command === "read_preview_mechanics_job_rows_all"
      ? Promise.reject({ code: "INVALID_REQUEST", message: `RESULT-DETAIL-WEBVIEW-LIMIT (result_bytes=${RESULT_DETAIL_WEBVIEW_LIMIT_BYTES + 1}, limit_bytes=${RESULT_DETAIL_WEBVIEW_LIMIT_BYTES}): the result is above the bytes the webview can hold whole; nothing was copied, and the result stays held natively.` })
      : forward(command, args, options));
    render(<App />);
    await screen.findByTestId("workspace-toolbar");
    await solve();
    const solveSection = screen.getByTestId("workspace-section-solve");
    fireEvent.click(within(solveSection).getByTestId("c2-result-load"));
    expect(await within(solveSection).findByTestId("c2-result-refused")).toHaveTextContent(`RESULT-DETAIL-WEBVIEW-LIMIT: This result is ${RESULT_DETAIL_WEBVIEW_LIMIT_BYTES + 1} bytes`);
    nativeMenuCommand("view.section.report");
    const refusals = await within(screen.getByTestId("workspace-section-report")).findAllByTestId("c2-result-refused");
    for (const refusal of refusals) expect(refusal).toHaveTextContent(`RESULT-DETAIL-WEBVIEW-LIMIT: This result is ${RESULT_DETAIL_WEBVIEW_LIMIT_BYTES + 1} bytes`);
    expect(mechanics.nativeHeld()).toHaveLength(1);
  }, 90_000);
});
