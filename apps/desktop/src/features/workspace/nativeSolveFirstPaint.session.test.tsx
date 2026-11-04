/** RT-U4 (DEL-04-05 `c1-slice7-run-proposal.md`, "A-only 500-node attempt: result, diagnosis and
 * repair"): a fresh native-custody solve's `result_committed` and `first_paint` marks through the
 * actual App, session, native-result and results-panel path.
 *
 * `first_paint` of a native handle is the frame that shows the Results table's first page: the
 * page's rows and their row semantics read, committed while the table can be seen, then two
 * animation frames. These tests pin:
 * - the complete mark sequence when Results is opened during the solve, as the perf driver does,
 *   with the first page and its semantics read after `result_committed` and before `first_paint`;
 * - no qualifying paint while the first page or its semantics are pending, then exactly two frames
 *   after they are released and rendered; none for a refused first page; none while Results is
 *   dormant, then two frames after it is opened and its page renders;
 * - none for a qualifying page hidden, or replaced by a filter or page change, before the browser
 *   paints it (either frame); the page shown next arms its own two frames (review F2);
 * - that `reader_verdict_end` → `analysis_run_built` awaits exactly one native call, the
 *   analysis-run preview, journalled as the `run_preview` phase; holding it reproduces the attempt;
 * - every row read in the whole call sequence is bounded (no take, no C2 full copy).
 * Unit transport replay of committed producer bytes through mocked IPC; NOT a native UI
 * qualification witness, a timing or a measurement. All data are invented. */
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "../../App";
import { createNativeMechanicsReplay, REPLAY_COMMANDS, type NativeMechanicsReplayProfile } from "../../test/nativeMechanicsReplay";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { ANALYSIS_RUN_PREVIEW_COMMAND } from "../../services/analysisRunPreview";
import { RESULT_DETAIL_MAX_KEYS, RESULT_DETAIL_MAX_PAGE_ROWS } from "../../services/resultDetail";
import { DEFAULT_RESULT_PAGE_SIZE } from "../results/ResultsPanel";
import { readSolveTimeline, resetSolveTimelineForTests, type NativeReaderBoundary, type SolveStage } from "../../services/solveTimeline";
import { useModelledVisibility } from "../../test-support/modelledVisibility";

const PAGE_QUERY = "query_preview_mechanics_job_rows";
const SEMANTICS = "read_preview_mechanics_job_row_semantics";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  resetSolveTimelineForTests();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});
// jsdom does no layout: the shell's `.inactive` sections are modelled as hidden.
useModelledVisibility();

type Call = { command: string; stage: SolveStage | null; args: unknown };
type Options = {
  /** Keeps every reply of this command pending until released. */
  hold?: string;
  /** Holds or refuses only calls issued once the solve's result is committed (the panel's reads). */
  afterCommit?: boolean;
  /** Refuses this command (after the commit) as the native detail reader does. */
  refuse?: string;
};
const COMMITTED: ReadonlySet<SolveStage | null> = new Set(["result_committed", "first_paint"]);

/** The App on a native replay of `profile`, every native call recorded with the timeline's last
 * stage when it was issued. */
async function install(profile: NativeMechanicsReplayProfile, { hold, afterCommit = false, refuse }: Options = {}) {
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
    const stage = readSolveTimeline()?.marks.at(-1)?.stage ?? null;
    calls.push({ command, stage, args });
    const inScope = !afterCommit || COMMITTED.has(stage);
    if (command === refuse && inScope) throw new Error("RESULT-DETAIL-REFUSED: the page was refused (unit replay)");
    if (command === hold && inScope) await held;
    return mechanics.invoke(command, args, options);
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  await screen.findByTestId("workspace-toolbar");
  return { calls, release };
}

const menu = (detail: string) => act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail })); });
const results = () => screen.getByTestId("workspace-section-results");
const stages = () => readSolveTimeline()?.marks.map((mark) => mark.stage) ?? [];
const rowShown = () => results().querySelector("[data-testid^='result-row-']") !== null;
const ORDER: readonly SolveStage[] = ["solve_requested", "job_started", "job_done", "reader_verdict_start", "reader_verdict_end", "analysis_run_built", "result_set", "result_committed", "first_paint"];

/** Analyze → Run (which shows Solve), then Results once the solve has begun: the driver's order. */
async function runThenOpenResults() {
  menu("analyze.run");
  await waitFor(() => expect(stages()).toContain("solve_requested"));
  menu("view.section.results");
  expect(results().classList.contains("inactive")).toBe(false);
}

/** Animation frames under the test's control: `frame()` runs the callbacks queued so far. */
function manualFrames() {
  let queued: FrameRequestCallback[] = [];
  vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => { queued.push(callback); return queued.length; });
  vi.stubGlobal("cancelAnimationFrame", () => undefined);
  return {
    frame: () => act(() => { const now = queued; queued = []; for (const callback of now) callback(performance.now()); }),
    pending: () => queued.length
  };
}
async function turns(count = 20) {
  await act(async () => { for (let turn = 0; turn < count; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0)); });
}

/** Every row read of the whole sequence is bounded: page reads carry a numeric limit of at most
 * `RESULT_DETAIL_MAX_PAGE_ROWS`, key reads at most `RESULT_DETAIL_MAX_KEYS` keys; no row command
 * outside these (a take or the C2 full copy) is issued at all. The expected first page occurred. */
function expectBoundedRowReads(calls: readonly Call[]) {
  const PAGED = new Set([PAGE_QUERY, "query_preview_mechanics_job_source_recovery"]);
  const KEYED = new Set(["lookup_preview_mechanics_job_rows", SEMANTICS]);
  const rowCommands = calls.filter((call) => /rows|semantics|recovery|take_|full_copy/.test(call.command));
  expect(rowCommands.length).toBeGreaterThan(0);
  for (const call of rowCommands) {
    expect([...PAGED, ...KEYED]).toContain(call.command);
    const args = call.args as Record<string, unknown>;
    if (PAGED.has(call.command)) {
      expect(Number.isInteger(args.limit)).toBe(true);
      expect(args.limit as number).toBeGreaterThanOrEqual(1);
      expect(args.limit as number).toBeLessThanOrEqual(RESULT_DETAIL_MAX_PAGE_ROWS);
    } else {
      const keys = ["ordinals", "ids", "kinds"].map((key) => args[key]).filter((value) => value !== undefined);
      expect(keys.length).toBeGreaterThan(0);
      for (const value of keys) {
        expect(Array.isArray(value)).toBe(true);
        expect((value as unknown[]).length).toBeLessThanOrEqual(RESULT_DETAIL_MAX_KEYS);
      }
    }
  }
  const firstPage = calls.find((call) => call.command === PAGE_QUERY && (call.args as { offset?: unknown }).offset === 0 && (call.args as { limit?: unknown }).limit === DEFAULT_RESULT_PAGE_SIZE);
  expect(firstPage).toBeDefined();
  return firstPage!;
}

describe.each(["pressure", "preview"] as const)("a fresh native-custody solve (%s replay) with Results opened during the solve", (profile) => {
  it("marks first_paint after its first page and row semantics, reading only bounded rows", async () => {
    const { calls } = await install(profile);
    await runThenOpenResults();
    await waitFor(() => expect(stages()).toContain("first_paint"), { timeout: 10000 });
    expect(stages()).toEqual(ORDER);
    expect(readSolveTimeline()!.outcome).toBe("completed");
    expect(rowShown()).toBe(true);
    // reader_verdict_end → analysis_run_built awaits exactly one native call: the record's preview.
    expect(calls.filter((call) => call.stage === "reader_verdict_end").map((call) => call.command)).toEqual([ANALYSIS_RUN_PREVIEW_COMMAND]);
    // The first page and its semantics are read between the commit and the paint.
    const firstPage = expectBoundedRowReads(calls);
    expect(firstPage.stage).toBe("result_committed");
    expect(calls.some((call) => call.command === SEMANTICS && call.stage === "result_committed")).toBe(true);
  });
});

describe("no qualifying first paint before the Results table's first page is shown", () => {
  for (const hold of [PAGE_QUERY, SEMANTICS]) {
    it(`waits while ${hold} is pending, then paints two frames after the page renders`, async () => {
      const frames = manualFrames();
      const { calls, release } = await install("pressure", { hold, afterCommit: true });
      await runThenOpenResults();
      await waitFor(() => expect(stages()).toContain("result_committed"), { timeout: 10000 });
      await waitFor(() => expect(calls.some((call) => call.command === hold && COMMITTED.has(call.stage))).toBe(true));
      // Held: the table says it is reading; however many frames pass, no paint is marked.
      for (let frame = 0; frame < 5; frame += 1) { frames.frame(); await turns(); }
      expect(screen.getByTestId("result-page-reading")).toBeTruthy();
      expect(rowShown()).toBe(false);
      expect(stages()).not.toContain("first_paint");
      expect(stages().at(-1)).toBe("result_committed");
      // Released: the rows render; the paint is the second frame after that commit.
      release();
      await waitFor(() => expect(rowShown()).toBe(true), { timeout: 10000 });
      expect(stages()).not.toContain("first_paint");
      frames.frame();
      expect(stages()).not.toContain("first_paint");
      frames.frame();
      expect(stages()).toEqual(ORDER);
      expectBoundedRowReads(calls);
    });
  }

  it("never marks a qualifying paint for a refused first page", async () => {
    const frames = manualFrames();
    await install("pressure", { refuse: PAGE_QUERY, afterCommit: true });
    await runThenOpenResults();
    await waitFor(() => expect(screen.getByTestId("result-query-refused")).toBeTruthy(), { timeout: 10000 });
    for (let frame = 0; frame < 5; frame += 1) { frames.frame(); await turns(); }
    expect(frames.pending()).toBe(0);
    expect(rowShown()).toBe(false);
    expect(stages()).toContain("result_committed");
    expect(stages()).not.toContain("first_paint");
  });

  it("waits while Results is dormant, then paints two frames after it is opened and its page renders", async () => {
    const frames = manualFrames();
    const { calls } = await install("pressure");
    menu("analyze.run");
    await waitFor(() => expect(stages()).toContain("result_committed"), { timeout: 10000 });
    for (let frame = 0; frame < 5; frame += 1) { frames.frame(); await turns(); }
    expect(stages()).not.toContain("first_paint");
    expect(calls.some((call) => call.command === PAGE_QUERY)).toBe(false);
    menu("view.section.results");
    await waitFor(() => expect(rowShown()).toBe(true), { timeout: 10000 });
    expect(stages()).not.toContain("first_paint");
    frames.frame();
    frames.frame();
    expect(stages()).toEqual(ORDER);
  });
});

describe("a qualifying first page hidden or replaced before the browser paints it (F2)", () => {
  /** Analyze → Run with Results open, up to the first page rendered and its frames queued. */
  async function firstPageArmed() {
    const frames = manualFrames();
    const { calls } = await install("pressure");
    await runThenOpenResults();
    await waitFor(() => expect(rowShown()).toBe(true), { timeout: 10000 });
    expect(stages().at(-1)).toBe("result_committed");
    expect(frames.pending()).toBeGreaterThan(0);
    return { frames, calls };
  }
  async function noPaintAcross(frames: ReturnType<typeof manualFrames>, count = 5) {
    for (let frame = 0; frame < count; frame += 1) { frames.frame(); await turns(); }
    expect(stages()).not.toContain("first_paint");
  }

  for (const hideAfter of [0, 1]) {
    it(`marks nothing when Results is hidden ${hideAfter ? "between the two frames" : "before the first frame"}, then paints two frames after it is shown again`, async () => {
      const { frames } = await firstPageArmed();
      for (let frame = 0; frame < hideAfter; frame += 1) frames.frame();
      menu("view.section.solve");
      expect(results().classList.contains("inactive")).toBe(true);
      await noPaintAcross(frames);
      expect(frames.pending()).toBe(0);
      menu("view.section.results");
      expect(results().classList.contains("inactive")).toBe(false);
      await turns();
      expect(stages()).not.toContain("first_paint");
      frames.frame();
      expect(stages()).not.toContain("first_paint");
      frames.frame();
      expect(stages()).toEqual(ORDER);
    });
  }

  it("marks nothing for a page replaced by a filter change before the first frame; the filtered page paints", async () => {
    const { frames, calls } = await firstPageArmed();
    const before = calls.filter((call) => call.command === PAGE_QUERY).length;
    fireEvent.change(screen.getByLabelText("Filter result rows"), { target: { value: "no-such-row-zz" } });
    // The old page's frames run while (or after) the filtered page is read: they mark nothing.
    frames.frame();
    expect(stages()).not.toContain("first_paint");
    await waitFor(() => expect(calls.filter((call) => call.command === PAGE_QUERY).length).toBeGreaterThan(before), { timeout: 10000 });
    await waitFor(() => expect(screen.getByTestId("result-filter-empty")).toBeTruthy(), { timeout: 10000 });
    frames.frame();
    expect(stages()).not.toContain("first_paint");
    frames.frame();
    expect(stages()).toEqual(ORDER);
  });

  it("marks nothing for a page replaced by a page change before the first frame; the new page paints", async () => {
    const { frames, calls } = await firstPageArmed();
    const before = calls.filter((call) => call.command === PAGE_QUERY).length;
    const size = screen.getByTestId("result-page-size") as HTMLSelectElement;
    const next = [...size.options].map((option) => Number(option.value)).find((value) => value !== DEFAULT_RESULT_PAGE_SIZE)!;
    fireEvent.change(size, { target: { value: String(next) } });
    frames.frame();
    expect(stages()).not.toContain("first_paint");
    await waitFor(() => expect(calls.some((call, index) => index >= before && call.command === PAGE_QUERY && (call.args as { limit?: unknown }).limit === next)).toBe(true), { timeout: 10000 });
    await waitFor(() => expect(rowShown()).toBe(true), { timeout: 10000 });
    await turns();
    await noPaintAcross(frames, 0);
    frames.frame();
    expect(stages()).not.toContain("first_paint");
    frames.frame();
    expect(stages()).toEqual(ORDER);
    expectBoundedRowReads(calls);
  });
});

describe("the reader_verdict_end → analysis_run_built window", () => {
  it("is the awaited native analysis-run preview, journalled as run_preview: holding it reproduces the attempt's state", async () => {
    const events: (NativeReaderBoundary & { stage: SolveStage | null })[] = [];
    globalThis.__SWBPIPE_NATIVE_SMOKE_READER_PHASE__ = async (event) => { events.push({ ...event, stage: readSolveTimeline()?.marks.at(-1)?.stage ?? null }); };
    const { calls, release } = await install("pressure", { hold: ANALYSIS_RUN_PREVIEW_COMMAND });
    await runThenOpenResults();
    await waitFor(() => expect(calls.map((call) => call.command)).toContain(ANALYSIS_RUN_PREVIEW_COMMAND), { timeout: 10000 });
    // Some turns later the session still waits on the preview alone: no further mark, no other call.
    const issued = calls.length;
    await turns();
    expect(stages().at(-1)).toBe("reader_verdict_end");
    expect(stages()).not.toContain("analysis_run_built");
    expect(calls.length).toBe(issued);
    // The journal names the pending await: run_preview entered after reader_verdict_end, no exit.
    expect(events.map(({ phase, edge }) => `${phase}:${edge}`)).toEqual([
      "header:enter", "header:exit", "registration:enter", "registration:exit", "standing:enter", "standing:exit",
      "summary:enter", "summary:exit", "run_preview:enter"
    ]);
    expect(events.at(-1)).toMatchObject({ layer: "web", phase: "run_preview", edge: "enter", status: "pending", stage: "reader_verdict_end" });
    release();
    await waitFor(() => expect(stages()).toContain("first_paint"), { timeout: 10000 });
    expect(stages()).toEqual(ORDER);
    expect(events.at(-1)).toMatchObject({ phase: "run_preview", edge: "exit", status: "ok", stage: "reader_verdict_end" });
    expect(events.filter((event) => event.phase === "run_preview")).toHaveLength(2);
  });
});
