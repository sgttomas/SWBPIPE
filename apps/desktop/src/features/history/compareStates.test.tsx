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
import { MAX_VISIBLE_COMPARISON_ROWS, StateComparisonView } from "../comparison/StateComparisonView";
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
    expect(compare).toHaveAttribute("title", "Select two saved states to compare.");
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
    expect(within(view).queryByRole("button", { name: /export|download|save/i })).toBeNull();
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

  it("renders at most a bounded number of rows and says how many match", () => {
    const left = structuredClone(everySlotModel) as Json;
    left.nodes = Array.from({ length: MAX_VISIBLE_COMPARISON_ROWS + 50 }, (_, index) => ({ id: `node:M-${String(index).padStart(4, "0")}`, position: { x: index, y: 0, z: 0 } }));
    render(<StateComparisonView result={{ projectId: PROJECT, left: FIRST, right: SECOND, comparison: compareModelDocuments(left, everySlotModel) }} onClose={() => {}} />);
    expect(screen.getByTestId("state-comparison-table").querySelectorAll("tbody tr").length).toBe(MAX_VISIBLE_COMPARISON_ROWS);
    expect(screen.getByTestId("state-comparison-shown")).toHaveTextContent(`Showing the first ${MAX_VISIBLE_COMPARISON_ROWS} of`);
  });
});

/** The session's flow, against a fake of the native commands. */
function nativeStore(options: { admission?: () => unknown } = {}) {
  (window as Json).__TAURI_INTERNALS__ = {};
  const snapshots: Record<string, Json> = { [FIRST.state_id]: LEFT_MODEL, [SECOND.state_id]: RIGHT_MODEL };
  invokeMock.mockImplementation(async (command: string, args: Record<string, unknown>) => {
    switch (command) {
      case "list_model_states": return [SECOND, FIRST, { unreadable: UNREADABLE }];
      case "list_analysis_runs": return [];
      case "admit_model_state_comparison":
        if (options.admission) return options.admission();
        return { left_snapshot_bytes: 10, right_snapshot_bytes: 12, combined_bytes: 22, budget_bytes: 12 * 1024 ** 3 };
      case "read_model_state": {
        const id = args.stateId as string;
        const summary = [FIRST, SECOND].find((candidate) => candidate.state_id === id)!;
        return { state: summary, record: {}, model: structuredClone(snapshots[id]), model_document_version: { status: "current" } };
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
    nativeStore();
    const { result: hook, deps } = sessionHistory();
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    let compared = false;
    await act(async () => { compared = await hook.current.compareStates(SECOND.state_id, FIRST.state_id); });
    expect(compared).toBe(true);
    expect(commands()).toEqual(["admit_model_state_comparison", "read_model_state", "read_model_state"]);
    const admission = invokeMock.mock.calls.find(([command]) => command === "admit_model_state_comparison")![1];
    expect(admission).toEqual({ projectId: PROJECT, leftStateId: FIRST.state_id, rightStateId: SECOND.state_id });
    expect(hook.current.stateComparison).toMatchObject({ left: FIRST, right: SECOND });
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
    nativeStore({ admission: () => { throw refusal; } });
    const { result: hook } = sessionHistory();
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    await act(async () => { await hook.current.compareStates(FIRST.state_id, SECOND.state_id); });
    expect(commands()).toEqual(["admit_model_state_comparison"]);
    expect(hook.current.stateComparison).toBeNull();
    expect(hook.current.historyMessage).toBe(`Not compared: ${refusal}`);
    expect(hook.current.historyOperation).toBe("compare_states_failed");
  });

  it("starts nothing while a prompt holds the session or another operation owns it", async () => {
    nativeStore();
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
    nativeStore();
    const { result: hook } = sessionHistory();
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    for (const [first, second] of [[FIRST.state_id, UNREADABLE.id], [FIRST.state_id, FIRST.state_id]]) {
      await act(async () => { await hook.current.compareStates(first, second); });
      expect(hook.current.historyMessage).toBe("Not compared: select two different saved states of this project.");
    }
    expect(commands()).toEqual([]);
  });

  it("shows a document the comparison cannot hold as its named refusal", async () => {
    nativeStore();
    const odd = structuredClone(RIGHT_MODEL);
    odd.invented_future_member = [];
    const { result: hook } = sessionHistory();
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    const base = invokeMock.getMockImplementation()!;
    invokeMock.mockImplementation(async (command: string, args: Record<string, unknown>) => {
      if (command === "read_model_state" && args.stateId === SECOND.state_id) return { state: SECOND, record: {}, model: odd, model_document_version: { status: "current" } };
      return base(command, args);
    });
    await act(async () => { await hook.current.compareStates(FIRST.state_id, SECOND.state_id); });
    expect(hook.current.historyMessage).toMatch(/^Not compared: COMPARISON-MEMBER-UNASSIGNED: the right model holds member `invented_future_member`/);
    expect(hook.current.stateComparison).toBeNull();
  });
});
