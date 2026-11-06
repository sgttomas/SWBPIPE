// C1 slice 4 (DEL-04-05 `c1-native-custody-plan.md`): in the app, a fresh solve's first paint
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
    expect(fullRecordBuilds).toHaveBeenCalledTimes(1);

    // Opening the Report section reads the same copy: none is made again for the job.
    nativeMenuCommand("view.section.report");
    expect(await within(screen.getByTestId("workspace-section-report")).findByTestId("report-panel")).toBeInTheDocument();
    await settle();
    expect(sent("read_preview_mechanics_job_rows_all")).toBe(1);
    // ... and its full record, built once on that copy, is shared.
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
});
