// W-12 S3a (DEL-14-01 Design `desktop-states-runs-comparison.md` §4.1, §4.3):
// Compare… of two saved states. The History panel offers it for two selected
// readable states and disables it, with the stated reason, while the session is
// busy, before two are selected, and for a row the store cannot read. The
// session's flow (the one-owner check, the busy refusal, the native size
// admission before either read, the two metered reads) runs against a fake of
// the native commands, and the comparison view shows the summary, the
// filterable table and the field paths with their units. All data are invented.
import { act, cleanup, fireEvent, render, renderHook, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import everySlotModel from "../../../../../fixtures/model_document/invented_every_slot_model.json";
import type { ModelStateSummary } from "../../services/modelStateService";
import { compareModelDocuments } from "../comparison/stateComparison";
import { COMPARISON_PAGE_ROWS, StateComparisonView } from "../comparison/StateComparisonView";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { createHash } from "node:crypto";
import { WorkspaceSessionProvider } from "../workspace/WorkspaceSessionContext";
import type { WorkspaceSession } from "../workspace/workspaceSession";
import { HistoryPanel } from "./HistoryPanel";
import { useModelStateHistory, type ModelStateHistoryDeps, type StateComparisonResult } from "./useModelStateHistory";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as Json).__TAURI_INTERNALS__;
});

const PROJECT = (everySlotModel as Json).project.id as string;
const LEFT_MODEL = structuredClone(everySlotModel) as Json;
const RIGHT_MODEL = (() => {
  const model = structuredClone(everySlotModel) as Json;
  model.materials[0].elastic_modulus = { value: 200000, unit: "MPa" };
  model.supports[0].label = "Invented relabelled support";
  model.nodes.push({ id: "node:N-4", label: "Invented added node", position: { x: 9, y: 0, z: 0 } });
  model.lines.pop();
  return model;
})();

function state(n: number, name: string, overrides: Partial<ModelStateSummary> = {}): ModelStateSummary {
  return {
    project_id: PROJECT, state_id: `state:${String(n).padStart(32, "0")}`, state_kind: "design_snapshot", created_at: `2026-10-24T10:00:0${n}.000Z`,
    parent_state_id: null, model_hash: `sha256:${String(n).repeat(64)}`, record_hash: "r", labels_updated_at: `2026-10-24T10:00:0${n}.000Z`,
    child_state_ids: [], recorded_run_count: 0, analysis_status: ["HUMAN_REVIEW_REQUIRED"], state_name: name, tags: [], notes: [], assumptions: [],
    ...overrides
  };
}
const FIRST = state(1, "Basis");
const SECOND = state(2, "Revision");
const THIRD = state(3, "Later");
const UNREADABLE = { id: "state:00000000000000000000000000000009", handle: "state-row:9:38:0123456789abcdef", references: 0, refusal: "MODEL-STATE-INTEGRITY: saved state `state:00000000000000000000000000000009`: the row's created_at differs from its verified record; it is listed as unreadable and is not opened." };

function result(): StateComparisonResult {
  return { projectId: PROJECT, left: FIRST, right: SECOND, comparison: compareModelDocuments(LEFT_MODEL, RIGHT_MODEL) };
}

function slice(overrides: Partial<WorkspaceSession["project"]["history"]> = {}) {
  return {
    states: [FIRST, SECOND, THIRD], runs: [], unreadable: { states: [UNREADABLE], runs: [] }, runCheck: null, recordRunUnavailableReason: "RUN-RECORD-NO-RUN: solve first.", recordRunState: null,
    historyMessage: "", historyOperation: "", historyBusy: false, basisStateId: null, refreshStates: vi.fn(), saveState: vi.fn(), openState: vi.fn(), editLabels: vi.fn(),
    deleteState: vi.fn(), recordRun: vi.fn(), editRunLabels: vi.fn(), deleteRun: vi.fn(), checkRun: vi.fn(), deleteUnreadable: vi.fn(),
    stateComparison: null, compareStates: vi.fn(async () => true), closeStateComparison: vi.fn(),
    ...overrides
  } as unknown as WorkspaceSession["project"]["history"];
}

function renderPanel(history: WorkspaceSession["project"]["history"], projectBusy = false) {
  const session = { model: { model: { project: { id: PROJECT } } }, project: { history, projectBusy } } as unknown as WorkspaceSession;
  return render(<WorkspaceSessionProvider session={session}><HistoryPanel /></WorkspaceSessionProvider>);
}

const select = (target: ModelStateSummary) => fireEvent.click(screen.getByTestId(`model-state-${target.state_id}-compare-select`));

describe("Compare… in the History panel", () => {
  it("is enabled for two selected readable states and compares them", () => {
    const history = slice();
    renderPanel(history);
    const compare = screen.getByTestId("model-state-compare");
    expect(compare).toBeDisabled();
    // Since S3b the one Compare… takes two saved states or two runs.
    expect(compare).toHaveAttribute("title", "Select two saved states, or two runs, to compare.");
    select(SECOND);
    expect(compare).toBeDisabled();
    select(FIRST);
    expect(compare).toBeEnabled();
    expect(compare).not.toHaveAttribute("title");
    // A third cannot be added while two are selected; the reason is stated.
    const third = screen.getByTestId(`model-state-${THIRD.state_id}-compare-select`);
    expect(third).toBeDisabled();
    expect(third).toHaveAttribute("title", "Two saved states are selected; clear one to choose another.");
    fireEvent.click(compare);
    expect(history.compareStates).toHaveBeenCalledWith(SECOND.state_id, FIRST.state_id);
    // Clearing one frees the third.
    select(SECOND);
    expect(third).toBeEnabled();
    expect(compare).toBeDisabled();
  });

  it("is disabled with the busy reason while another operation or prompt holds the session", () => {
    for (const [overrides, projectBusy] of [[{ historyBusy: true }, false], [{}, true]] as const) {
      const history = slice(overrides);
      renderPanel(history, projectBusy);
      select(FIRST);
      select(SECOND);
      const compare = screen.getByTestId("model-state-compare");
      expect(compare).toBeDisabled();
      expect(compare).toHaveAttribute("title", "Another project operation or prompt is in progress.");
      fireEvent.click(compare);
      expect(history.compareStates).not.toHaveBeenCalled();
      cleanup();
    }
  });

  it("offers no comparison of a row the store cannot read, and says why", () => {
    renderPanel(slice());
    const item = screen.getByTestId(`unreadable-state-${UNREADABLE.handle}`);
    const box = within(item).getByTestId(`unreadable-state-${UNREADABLE.handle}-compare-select`);
    expect(box).toBeDisabled();
    expect(box).toHaveAttribute("title", "This stored row cannot be read, so it cannot be compared.");
    expect(item).toHaveTextContent("Not compared: This stored row cannot be read, so it cannot be compared.");
    // Delete stays its only action.
    expect(within(item).getAllByRole("button").map((button) => button.textContent)).toEqual(["Delete…"]);
  });

  it("shows the comparison below the lists, view-only, with no wording of standing", () => {
    const history = slice({ stateComparison: result() });
    renderPanel(history);
    const view = screen.getByTestId("state-comparison");
    // RT-R6 (DEL-08-04 Design `csv-export.md` §1 C1): the one export is the table's local-private
    // CSV; the comparison envelope export stays later (FR-CMP-010).
    expect(within(view).getAllByRole("button", { name: /export|download|save/i }).map((button) => button.getAttribute("aria-label"))).toEqual(["Export CSV: Saved state comparison"]);
    expect(screen.getByTestId("model-history-panel").textContent).not.toMatch(/certif|accept|approv/i);
    fireEvent.click(within(view).getByTestId("state-comparison-close"));
    expect(history.closeStateComparison).toHaveBeenCalled();
  });
});

describe("the comparison view", () => {
  it("shows the counts, a filterable table and field paths with both values and units", () => {
    const shown = result();
    render(<StateComparisonView result={shown} onClose={() => {}} />);
    const { summary } = shown.comparison;
    expect(summary).toMatchObject({ added: 1, removed: 1, changed: 3 });
    for (const name of ["added", "removed", "changed", "unchanged"] as const) {
      expect(screen.getByTestId(`state-comparison-count-${name}`)).toHaveTextContent(String(summary[name]));
    }
    expect(screen.getByTestId("state-comparison-count-unit-changed")).toHaveTextContent("1");
    expect(screen.getByTestId("state-comparison-sides")).toHaveTextContent(`Left: "Basis" (${FIRST.state_id})`);
    // Differences only by default.
    const rows = () => screen.getByTestId("state-comparison-table").querySelectorAll("tbody tr");
    expect([...rows()].map((row) => row.getAttribute("data-classification"))).toEqual(["changed", "added", "changed", "changed", "removed"]);
    const material = screen.getByTestId("state-comparison-row-material-material:steel");
    expect(material).toHaveTextContent("elastic_modulus unit changed (not converted): left 200000000000 Pa; right 200000 MPa");
    expect(screen.getByTestId("state-comparison-row-support-support:S-1")).toHaveTextContent('label changed: left "Anchor support"; right "Invented relabelled support"');
    expect(screen.getByTestId("state-comparison-row-node-node:N-4")).toHaveTextContent("Only in the right state.");
    expect(screen.getByTestId("state-comparison-row-line-line:L-200")).toHaveTextContent("Only in the left state.");
    // Filters: classification, family and id.
    fireEvent.change(screen.getByTestId("state-comparison-filter-classification"), { target: { value: "unchanged" } });
    expect(rows().length).toBe(summary.unchanged);
    fireEvent.change(screen.getByTestId("state-comparison-filter-classification"), { target: { value: "all" } });
    expect(rows().length).toBe(summary.total);
    fireEvent.change(screen.getByTestId("state-comparison-filter-family"), { target: { value: "node" } });
    expect([...rows()].map((row) => row.getAttribute("data-testid"))).toEqual(["node:N-1", "node:N-2", "node:N-3", "node:N-4"].map((id) => `state-comparison-row-node-${id}`));
    fireEvent.change(screen.getByTestId("state-comparison-filter-id"), { target: { value: "N-4" } });
    expect(rows().length).toBe(1);
    fireEvent.change(screen.getByTestId("state-comparison-filter-id"), { target: { value: "no such id" } });
    expect(screen.getByTestId("state-comparison-shown")).toHaveTextContent("No entity matches the filters.");
  });

  it("pages through every matching row, at most a page at once, back to the first page on a filter change", () => {
    // 201 removed `A-N-1-xxx` nodes sort before the removed `N-1`, which is on the second page.
    const minimal = (nodes: Json[]) => ({ schema_version: "0.9.0", document_kind: "openpipestress.product_preview.model", project: { id: PROJECT }, nodes });
    const removed = [...Array.from({ length: 201 }, (_, index) => ({ id: `A-N-1-${String(index).padStart(3, "0")}` })), { id: "N-1" }];
    render(<StateComparisonView result={{ projectId: PROJECT, left: FIRST, right: SECOND, comparison: compareModelDocuments(minimal(removed), minimal([])) }} onClose={() => {}} />);
    const ids = () => [...screen.getByTestId("state-comparison-table").querySelectorAll("tbody tr td:nth-child(2)")].map((cell) => cell.textContent);
    fireEvent.change(screen.getByTestId("state-comparison-filter-classification"), { target: { value: "removed" } });
    expect(COMPARISON_PAGE_ROWS).toBeLessThanOrEqual(200);
    expect(ids()).toHaveLength(COMPARISON_PAGE_ROWS);
    expect(ids()).not.toContain("N-1");
    expect(screen.getByTestId("state-comparison-shown")).toHaveTextContent(`Rows 1–${COMPARISON_PAGE_ROWS} of 202.`);
    expect(screen.getByTestId("state-comparison-page-previous")).toBeDisabled();
    fireEvent.click(screen.getByTestId("state-comparison-page-next"));
    expect(ids()).toContain("N-1");
    expect(screen.getByTestId("state-comparison-shown")).toHaveTextContent(`Rows ${COMPARISON_PAGE_ROWS + 1}–202 of 202.`);
    expect(screen.getByTestId("state-comparison-page-next")).toBeDisabled();
    fireEvent.click(screen.getByTestId("state-comparison-page-previous"));
    expect(screen.getByTestId("state-comparison-shown")).toHaveTextContent(`Rows 1–${COMPARISON_PAGE_ROWS} of 202.`);
    // A filter change returns to the first page.
    fireEvent.click(screen.getByTestId("state-comparison-page-next"));
    fireEvent.change(screen.getByTestId("state-comparison-filter-id"), { target: { value: "N-1" } });
    expect(screen.getByTestId("state-comparison-shown")).toHaveTextContent(`Rows 1–${COMPARISON_PAGE_ROWS} of 202.`);
    fireEvent.change(screen.getByTestId("state-comparison-filter-id"), { target: { value: "A-N-1-2" } });
    expect(ids()).toEqual(["A-N-1-200"]);
  });
});

// W-17: Open State applies the desktop's open check for the `project.units` map
// (`MODEL-FILE-SHAPE`), as File > Open Model Document does; a missing key alone opens.
describe("Open State and project.units", () => {
  async function openWith(mutate: (units: Json, project: Json) => void, extra: Partial<ModelStateHistoryDeps> = {}) {
    const sha = (text: string) => `sha256:${createHash("sha256").update(text, "utf8").digest("hex")}`;
    const changed = structuredClone(RIGHT_MODEL);
    mutate(changed.project.units, changed.project);
    const text = await canonicalJsonString(changed);
    const open = sessionHistory({
      captureProjectBinding: () => ({ base: null, summary: null, envelopeHash: null }),
      confirmReplacingEdits: async () => "proceed",
      adoptStateSnapshot: vi.fn(async () => ""),
      ...extra
    } as Partial<ModelStateHistoryDeps>);
    await nativeStore({ reads: { [SECOND.state_id]: { text, model_hash: sha(text) } } });
    await act(async () => { await open.result.current.refreshStates(PROJECT); });
    await act(async () => { await open.result.current.openState(SECOND.state_id); });
    return open;
  }

  it("refuses a saved state without the map by name, and adopts nothing", async () => {
    const open = await openWith((_units, project) => { delete project.units; });
    expect(open.result.current.historyOperation).toBe("open_state_failed");
    expect(open.result.current.historyMessage).toBe(
      "Not opened: Error: Model document refused: MODEL-FILE-SHAPE at project.units: project.units is missing (object required)."
    );
    expect(open.deps.adoptStateSnapshot).not.toHaveBeenCalled();
  });

  it("opens a saved state missing one key", async () => {
    const open = await openWith((units) => { delete units.angle; });
    expect(open.result.current.historyOperation).toBe("open_state");
    expect(open.deps.adoptStateSnapshot).toHaveBeenCalledTimes(1);
    expect(open.result.current.refusedStateExport).toBeNull();
  });

  // The recovery export: the refused state can be saved to a file as stored,
  // through the session's dialog; its outcome is the history message.
  it("offers the refused state's recovery export, which adopts nothing", async () => {
    const requestStoredDocumentExport = vi.fn((_target: unknown, done: (message: string, operation: string) => void) => {
      done("Saved the saved state to invented.json exactly as stored.", "export_stored_document");
      return true;
    });
    const open = await openWith((_units, project) => { delete project.units; }, { requestStoredDocumentExport });
    const target = open.result.current.refusedStateExport;
    expect(target).toEqual({
      projectId: PROJECT,
      stateId: SECOND.state_id,
      description: `the saved state "Revision" (${SECOND.state_id})`,
      refusal: "Model document refused: MODEL-FILE-SHAPE at project.units: project.units is missing (object required)."
    });
    let offered = false;
    act(() => { offered = open.result.current.exportRefusedState(); });
    expect(offered).toBe(true);
    expect(requestStoredDocumentExport).toHaveBeenCalledWith(target, expect.any(Function));
    expect(open.result.current.historyMessage).toBe("Saved the saved state to invented.json exactly as stored.");
    expect(open.deps.adoptStateSnapshot).not.toHaveBeenCalled();
    // Only the list and the one read reached the native store: nothing was written.
    expect(commands()).toEqual(["read_model_state"]);
  });

  it("shows Save the stored document to a file… in the History panel for the refused state", () => {
    const exportRefusedState = vi.fn(() => true);
    renderPanel(slice({
      refusedStateExport: { projectId: PROJECT, stateId: SECOND.state_id, description: `the saved state "Revision" (${SECOND.state_id})`, refusal: "invented" },
      exportRefusedState
    } as Json));
    const button = screen.getByTestId("model-history-export-stored-state");
    expect(button).toHaveTextContent("Save the stored document to a file…");
    expect(screen.getByTestId("model-history-refused-state-export")).toHaveTextContent(`Saves the saved state "Revision" (${SECOND.state_id}) exactly as stored, without opening it`);
    fireEvent.click(button);
    expect(exportRefusedState).toHaveBeenCalledTimes(1);
    cleanup();
    renderPanel(slice({ refusedStateExport: null } as Json));
    expect(screen.queryByTestId("model-history-export-stored-state")).toBeNull();
  });
});

/** The session's flow, against a fake of the native commands. */
/** Each read returns the snapshot as the webview decodes the native response, with
 * the state's stored model hash (the native store's hash of its exact text). */
async function nativeStore(options: { admission?: () => unknown; reads?: Record<string, { text: string; model_hash: string }> } = {}) {
  (window as Json).__TAURI_INTERNALS__ = {};
  const hashed = async (model: Json) => ({ text: await canonicalJsonString(model), model_hash: (await computeModelHash(model))!.value });
  const reads: Record<string, { text: string; model_hash: string }> = {
    [FIRST.state_id]: await hashed(LEFT_MODEL), [SECOND.state_id]: await hashed(RIGHT_MODEL), ...options.reads
  };
  invokeMock.mockImplementation(async (command: string, args: Record<string, unknown>) => {
    switch (command) {
      case "list_model_states": return [SECOND, FIRST, { unreadable: UNREADABLE }];
      case "list_analysis_runs": return [];
      case "admit_model_state_comparison":
        if (options.admission) return options.admission();
        return { left_snapshot_bytes: 10, right_snapshot_bytes: 12, combined_bytes: 22, budget_bytes: 12 * 1024 ** 3 };
      case "read_model_state":
      case "read_model_state_for_comparison": {
        const id = args.stateId as string;
        const summary = [FIRST, SECOND].find((candidate) => candidate.state_id === id)!;
        const read = reads[id];
        return { state: { ...summary, model_hash: read.model_hash }, record: {}, model: JSON.parse(read.text), model_document_version: { status: "current" } };
      }
      default: throw new Error(`unexpected ${command}`);
    }
  });
}

function sessionHistory(overrides: Partial<ModelStateHistoryDeps> = {}) {
  const model = { project: { id: PROJECT } };
  const owner = Symbol("owner");
  const deps = {
    model, currentModel: { current: model }, projectSessionGeneration: 0, projectSessionGenerationRef: { current: 0 },
    runSlot: { result: null, analysisRun: null, inputManifest: null, restored: false },
    projectSessionBusy: () => false, acquireProjectOperation: vi.fn(() => owner), releaseProjectOperation: vi.fn(),
    ...overrides
  } as unknown as ModelStateHistoryDeps;
  return { deps, ...renderHook(() => useModelStateHistory(deps)) };
}

const commands = () => invokeMock.mock.calls.map(([command]) => command as string).filter((command) => !command.startsWith("list_"));

describe("the session's Compare… flow", () => {
  it("admits the size, reads both states and compares them, the earlier-saved on the left", async () => {
    await nativeStore();
    const { result: hook, deps } = sessionHistory();
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    let compared = false;
    await act(async () => { compared = await hook.current.compareStates(SECOND.state_id, FIRST.state_id); });
    expect(compared).toBe(true);
    expect(commands()).toEqual(["admit_model_state_comparison", "read_model_state_for_comparison", "read_model_state_for_comparison"]);
    const admission = invokeMock.mock.calls.find(([command]) => command === "admit_model_state_comparison")![1];
    expect(admission).toEqual({ projectId: PROJECT, leftStateId: FIRST.state_id, rightStateId: SECOND.state_id });
    expect(hook.current.stateComparison).toMatchObject({ left: { state_id: FIRST.state_id }, right: { state_id: SECOND.state_id } });
    expect(hook.current.stateComparison!.comparison).toEqual(compareModelDocuments(LEFT_MODEL, RIGHT_MODEL));
    expect(hook.current.historyMessage).toBe(`Compared "Basis" (${FIRST.state_id}) with "Revision" (${SECOND.state_id}): 1 added, 1 removed, 3 changed, ${hook.current.stateComparison!.comparison.summary.unchanged} unchanged. Neither state changes.`);
    // Through the one-owner check, released afterwards.
    expect(deps.acquireProjectOperation).toHaveBeenCalledTimes(1);
    expect(deps.releaseProjectOperation).toHaveBeenCalledTimes(1);
    act(() => hook.current.closeStateComparison());
    expect(hook.current.stateComparison).toBeNull();
  });

  it("is refused by name before either state is read when the two snapshots exceed the budget", async () => {
    const refusal = `COMPARISON-STATES-TOO-LARGE: comparing saved states \`${FIRST.state_id}\` and \`${SECOND.state_id}\` needs their two snapshots, 30 bytes together, above the per-invocation budget of 20 bytes; nothing was read and nothing was compared.`;
    await nativeStore({ admission: () => { throw refusal; } });
    const { result: hook } = sessionHistory();
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    await act(async () => { await hook.current.compareStates(FIRST.state_id, SECOND.state_id); });
    expect(commands()).toEqual(["admit_model_state_comparison"]);
    expect(hook.current.stateComparison).toBeNull();
    expect(hook.current.historyMessage).toBe(`Not compared: ${refusal}`);
    expect(hook.current.historyOperation).toBe("compare_states_failed");
  });

  it("starts nothing while a prompt holds the session or another operation owns it", async () => {
    await nativeStore();
    const busy = sessionHistory({ projectSessionBusy: () => true });
    await act(async () => { await busy.result.current.refreshStates(PROJECT); });
    await act(async () => { await busy.result.current.compareStates(FIRST.state_id, SECOND.state_id); });
    expect(busy.result.current.historyOperation).toBe("compare_states_busy");
    expect(busy.deps.acquireProjectOperation).not.toHaveBeenCalled();
    const owned = sessionHistory({ acquireProjectOperation: vi.fn(() => null) });
    await act(async () => { await owned.result.current.refreshStates(PROJECT); });
    await act(async () => { await owned.result.current.compareStates(FIRST.state_id, SECOND.state_id); });
    expect(owned.result.current.historyOperation).toBe("busy");
    expect(commands()).toEqual([]);
  });

  it("refuses an unreadable or unlisted state, and a state compared with itself", async () => {
    await nativeStore();
    const { result: hook } = sessionHistory();
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    for (const [first, second] of [[FIRST.state_id, UNREADABLE.id], [FIRST.state_id, FIRST.state_id]]) {
      await act(async () => { await hook.current.compareStates(first, second); });
      expect(hook.current.historyMessage).toBe("Not compared: select two different saved states of this project.");
    }
    expect(commands()).toEqual([]);
  });

  it("refuses a snapshot whose stored values the webview cannot represent exactly (an integer beyond 2^53)", async () => {
    // The native store keeps 9007199254740992 and 9007199254740993 apart, with
    // distinct hashes; the webview decodes both to 9007199254740992.
    const sha = (text: string) => `sha256:${createHash("sha256").update(text, "utf8").digest("hex")}`;
    const base = structuredClone(RIGHT_MODEL);
    base.nodes[0].position = { ...base.nodes[0].position, x: 9007199254740992 };
    const decodedText = await canonicalJsonString(base);
    expect(decodedText).toContain('"x":9007199254740992');
    const exactText = decodedText.replace('"x":9007199254740992', '"x":9007199254740993');
    expect(sha(exactText)).not.toBe(sha(decodedText));
    expect(JSON.parse(exactText)).toEqual(JSON.parse(decodedText));
    // The webview's hash of the decoded document is the hash of the decoded text.
    expect((await computeModelHash(JSON.parse(decodedText)))!.value).toBe(sha(decodedText));
    await nativeStore({ reads: { [SECOND.state_id]: { text: exactText, model_hash: sha(exactText) } } });
    const { result: hook } = sessionHistory();
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    let compared = true;
    await act(async () => { compared = await hook.current.compareStates(FIRST.state_id, SECOND.state_id); });
    expect(compared).toBe(false);
    expect(hook.current.stateComparison).toBeNull();
    expect(hook.current.historyMessage).toMatch(new RegExp(`^Not compared: COMPARISON-STATE-NOT-EXACT: the saved state "Revision" \\(${SECOND.state_id}\\) holds values the desktop cannot represent exactly`));
    // The same pair with the representable value compares.
    await nativeStore({ reads: { [SECOND.state_id]: { text: decodedText, model_hash: sha(decodedText) } } });
    await act(async () => { compared = await hook.current.compareStates(FIRST.state_id, SECOND.state_id); });
    expect(compared).toBe(true);
    expect(hook.current.stateComparison!.comparison.rows.find((row) => row.id === "node:N-1")!.classification).toBe("changed");
  });

  it("Open State names the cause: a snapshot not representable exactly, apart from a snapshot of another project", async () => {
    const sha = (text: string) => `sha256:${createHash("sha256").update(text, "utf8").digest("hex")}`;
    const base = structuredClone(RIGHT_MODEL);
    base.nodes[0].position = { ...base.nodes[0].position, x: 9007199254740992 };
    const exactText = (await canonicalJsonString(base)).replace('"x":9007199254740992', '"x":9007199254740993');
    const open = sessionHistory({
      captureProjectBinding: () => ({ base: null, summary: null, envelopeHash: null }),
      confirmReplacingEdits: async () => "proceed",
      adoptStateSnapshot: vi.fn(async () => "")
    } as Partial<ModelStateHistoryDeps>);
    await nativeStore({ reads: { [SECOND.state_id]: { text: exactText, model_hash: sha(exactText) } } });
    await act(async () => { await open.result.current.refreshStates(PROJECT); });
    await act(async () => { await open.result.current.openState(SECOND.state_id); });
    expect(open.result.current.historyOperation).toBe("open_state_failed");
    expect(open.result.current.historyMessage).toBe(
      `Not opened: Error: MODEL-STATE-INTEGRITY: the saved state "Revision" (${SECOND.state_id}) holds values the desktop cannot represent exactly (its snapshot as received does not hash to the stored ${sha(exactText).slice(0, 19)}…), so it is not opened.`
    );
    expect(open.deps.adoptStateSnapshot).not.toHaveBeenCalled();
    // The same wording as Compare…'s refusal.
    await act(async () => { await open.result.current.compareStates(FIRST.state_id, SECOND.state_id); });
    expect(open.result.current.historyMessage).toContain(`COMPARISON-STATE-NOT-EXACT: the saved state "Revision" (${SECOND.state_id}) holds values the desktop cannot represent exactly`);
    // A snapshot of another project, hashing correctly, has its own message.
    const stranger = structuredClone(RIGHT_MODEL);
    stranger.project = { ...stranger.project, id: "project:another-invented" };
    const strangerText = await canonicalJsonString(stranger);
    await nativeStore({ reads: { [SECOND.state_id]: { text: strangerText, model_hash: sha(strangerText) } } });
    await act(async () => { await open.result.current.openState(SECOND.state_id); });
    expect(open.result.current.historyMessage).toBe("Not opened: Error: MODEL-STATE-INTEGRITY: the opened snapshot is not of the saved state's project, so it is not opened.");
    expect(open.deps.adoptStateSnapshot).not.toHaveBeenCalled();
  });

  it("shows a document the comparison cannot hold as its named refusal", async () => {
    await nativeStore();
    const odd = structuredClone(RIGHT_MODEL);
    odd.invented_future_member = [];
    const { result: hook } = sessionHistory();
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    const base = invokeMock.getMockImplementation()!;
    invokeMock.mockImplementation(async (command: string, args: Record<string, unknown>) => {
      if (command === "read_model_state_for_comparison" && args.stateId === SECOND.state_id) return { state: { ...SECOND, model_hash: (await computeModelHash(odd))!.value }, record: {}, model: odd, model_document_version: { status: "current" } };
      return base(command, args);
    });
    await act(async () => { await hook.current.compareStates(FIRST.state_id, SECOND.state_id); });
    expect(hook.current.historyMessage).toMatch(/^Not compared: COMPARISON-MEMBER-UNASSIGNED: the right model holds member `invented_future_member`/);
    expect(hook.current.stateComparison).toBeNull();
  });
});
