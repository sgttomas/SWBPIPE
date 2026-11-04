/** RT-U4 (DEL-04-05 `c1-slice7-run-proposal.md`, "A-only 500-node attempt: result, diagnosis and
 * repair"): a fresh native-custody solve reaches the solve timeline's `result_committed` and
 * `first_paint` marks through the actual App, session, native-result and results-panel path.
 *
 * The A-only attempt's timeline ended at `result_set` and never marked a first paint: the timeline
 * recognised only a result read from taken bytes (`noteSolveResult` in `receiveResultBytes`), so
 * the native handle the session commits was never "the solve's result". These tests pin:
 * - the complete mark sequence, with `result_committed` and `first_paint` after `result_set`;
 * - that the window `reader_verdict_end` → `analysis_run_built` awaits exactly one native call, the
 *   analysis-run preview, and that holding it pending reproduces the attempt's state;
 * - that nothing between `result_set` and `first_paint` reads the whole result (no take, no C2 full
 *   copy) and every row read is one bounded page.
 * Unit transport replay of committed producer bytes through mocked IPC; NOT a native UI
 * qualification witness, a timing or a measurement. All data are invented. */
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "../../App";
import { createNativeMechanicsReplay, REPLAY_COMMANDS, type NativeMechanicsReplayProfile } from "../../test/nativeMechanicsReplay";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { ANALYSIS_RUN_PREVIEW_COMMAND } from "../../services/analysisRunPreview";
import { NATIVE_FULL_COPY_COMMAND } from "../../services/nativeResult";
import { RESULT_DETAIL_MAX_PAGE_ROWS } from "../../services/resultDetail";
import { readSolveTimeline, resetSolveTimelineForTests, type SolveStage } from "../../services/solveTimeline";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  resetSolveTimelineForTests();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

type Call = { command: string; stage: SolveStage | null; args: unknown };

/** The App on a native replay of `profile`, every native call recorded with the timeline's last
 * stage when it was issued. `hold` keeps one command's replies pending until released. */
async function install(profile: NativeMechanicsReplayProfile, hold: string | null = null) {
  const storage = await getLocalStorageCapability();
  const knowledge = await loadDesignKnowledge();
  const mechanics = createNativeMechanicsReplay({ profile });
  const calls: Call[] = [];
  let release: () => void = () => undefined;
  const held = new Promise<void>((resolve) => { release = resolve; });
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (!REPLAY_COMMANDS.includes(command)) throw new Error(`NATIVE_APP_REPLAY_COMMAND_UNSUPPORTED: ${command}`);
    calls.push({ command, stage: readSolveTimeline()?.marks.at(-1)?.stage ?? null, args });
    if (command === hold) await held;
    return mechanics.invoke(command, args, options);
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  return { calls, release };
}

function openSection(sectionId: "solve" | "results") {
  const section = screen.queryByTestId(`workspace-section-${sectionId}`);
  if (!section || section.classList.contains("inactive")) {
    act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: `view.section.${sectionId}` })); });
  }
  return screen.getByTestId(`workspace-section-${sectionId}`);
}

async function run() {
  await screen.findByTestId("workspace-toolbar");
  fireEvent.click(within(openSection("solve")).getByTestId("run-mechanics-preview"));
}

const stages = () => readSolveTimeline()?.marks.map((mark) => mark.stage) ?? [];
const ORDER: readonly SolveStage[] = ["solve_requested", "job_started", "job_done", "reader_verdict_start", "reader_verdict_end", "analysis_run_built", "result_set", "result_committed", "first_paint"];

describe.each(["pressure", "preview"] as const)("a fresh native-custody solve (%s replay) reaches first paint", (profile) => {
  it("marks result_committed and first_paint after result_set, reading no whole result before it", async () => {
    const { calls } = await install(profile);
    await run();
    await waitFor(() => expect(stages()).toContain("first_paint"), { timeout: 10000 });
    expect(stages()).toEqual(ORDER);
    expect(readSolveTimeline()!.outcome).toBe("completed");
    // reader_verdict_end → analysis_run_built awaits exactly one native call: the record's preview.
    expect(calls.filter((call) => call.stage === "reader_verdict_end").map((call) => call.command)).toEqual([ANALYSIS_RUN_PREVIEW_COMMAND]);
    // Never a take or a C2 full copy on the way to the first paint.
    expect(calls.map((call) => call.command)).not.toContain("take_preview_mechanics_job_result");
    expect(calls.map((call) => call.command)).not.toContain(NATIVE_FULL_COPY_COMMAND);
    // The results view, opened after the paint, reads rows only by bounded native pages.
    const rowsBefore = calls.length;
    openSection("results");
    await waitFor(() => expect(screen.getByTestId("workspace-section-results").querySelector("[data-testid^='result-row-']")).not.toBeNull(), { timeout: 10000 });
    const later = calls.slice(rowsBefore);
    expect(later.map((call) => call.command)).not.toContain(NATIVE_FULL_COPY_COMMAND);
    expect(later.map((call) => call.command)).not.toContain("take_preview_mechanics_job_result");
    for (const call of later) {
      const limit = (call.args as { limit?: unknown } | undefined)?.limit;
      if (typeof limit === "number") expect(limit).toBeLessThanOrEqual(RESULT_DETAIL_MAX_PAGE_ROWS);
    }
  });
});

describe("the reader_verdict_end → analysis_run_built window", () => {
  it("is the awaited native analysis-run preview: holding it reproduces the attempt's state", async () => {
    const { calls, release } = await install("pressure", ANALYSIS_RUN_PREVIEW_COMMAND);
    await run();
    await waitFor(() => expect(calls.map((call) => call.command)).toContain(ANALYSIS_RUN_PREVIEW_COMMAND), { timeout: 10000 });
    // Some turns later the session still waits on the preview alone: no further mark, no other call.
    const issued = calls.length;
    await act(async () => { for (let turn = 0; turn < 20; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0)); });
    expect(stages().at(-1)).toBe("reader_verdict_end");
    expect(stages()).not.toContain("analysis_run_built");
    expect(calls.length).toBe(issued);
    release();
    await waitFor(() => expect(stages()).toContain("first_paint"), { timeout: 10000 });
    expect(stages()).toEqual(ORDER);
  });
});
