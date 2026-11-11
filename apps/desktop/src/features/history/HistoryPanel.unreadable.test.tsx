// W-12 (DEL-14-01 Design §2, §3): the History panel shows each stored row the
// native store cannot read by its id, its named refusal and what to do, apart
// from the readable rows, with nothing that reads it offered (only delete);
// a run recorded before rule-pack resolution says to re-record it. Against a
// fake of the session's history slice, and the list services against a fake of
// the native commands. All data are invented.
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { listAnalysisRuns, type AnalysisRunSummary } from "../../services/analysisRunService";
import { listModelStates, type ModelStateSummary } from "../../services/modelStateService";
import { RULE_PACKS_NOT_RESOLVED } from "../../services/historyListEntries";
import { WorkspaceSessionProvider } from "../workspace/WorkspaceSessionContext";
import type { WorkspaceSession } from "../workspace/workspaceSession";
import { HistoryPanel } from "./HistoryPanel";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as Json).__TAURI_INTERNALS__;
});

const PROJECT = "project:invented-history";
const STATE: ModelStateSummary = {
  project_id: PROJECT, state_id: "state:00000000000000000000000000000001", state_kind: "post_solve_snapshot", created_at: "2026-10-24T10:00:01.000Z",
  parent_state_id: null, model_hash: `sha256:${"a".repeat(64)}`, record_hash: "r", labels_updated_at: "2026-10-24T10:00:01.000Z", child_state_ids: [],
  recorded_run_count: 0, analysis_status: ["HUMAN_REVIEW_REQUIRED"], state_name: "Readable", tags: [], notes: [], assumptions: []
};
const RUN: AnalysisRunSummary = {
  project_id: PROJECT, recording_id: "runrec:00000000000000000000000000000001", state_id: STATE.state_id, producer_run_id: "run:preview-linear-static-001",
  created_at: "2026-10-24T10:00:02.000Z", run_record_hash: "h", result_hash: "r", labels_updated_at: "2026-10-24T10:00:02.000Z",
  analysis_status: ["HUMAN_REVIEW_REQUIRED", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE"], run_name: "Readable run", tags: [], notes: [], assumptions: []
};
const TAMPERED_STATE = { id: "state:00000000000000000000000000000009", handle: "state-row:9:38:0123456789abcdef", references: 0, refusal: "MODEL-STATE-INTEGRITY: saved state `state:00000000000000000000000000000009`: the row's created_at differs from its verified record; it is listed as unreadable and is not opened." };
const TAMPERED_RUN = { id: "runrec:00000000000000000000000000000009", handle: "run-row:9:39:fedcba9876543210", references: 0, refusal: "ANALYSIS-RUN-INTEGRITY: recorded run `runrec:00000000000000000000000000000009`: the row's result hash differs from its verified record; it is listed as unreadable and is not read." };
const S2_REFUSAL = `${RULE_PACKS_NOT_RESOLVED}: recorded run \`${RUN.recording_id}\`: recorded before rule-pack resolution; re-record this run.`;

function history(overrides: Partial<WorkspaceSession["project"]["history"]> = {}) {
  return {
    states: [STATE], runs: [RUN], unreadable: { states: [TAMPERED_STATE], runs: [TAMPERED_RUN] }, runCheck: null, recordRunUnavailableReason: null, recordRunState: STATE,
    historyMessage: "", historyOperation: "", historyBusy: false, basisStateId: null, refreshStates: vi.fn(), saveState: vi.fn(), openState: vi.fn(), editLabels: vi.fn(),
    deleteState: vi.fn(), recordRun: vi.fn(async () => true), editRunLabels: vi.fn(async () => true), deleteRun: vi.fn(async () => true), checkRun: vi.fn(async () => true),
    deleteUnreadable: vi.fn(async () => true),
    ...overrides
  } as unknown as WorkspaceSession["project"]["history"];
}

function renderPanel(slice: WorkspaceSession["project"]["history"]) {
  const session = { model: { model: { project: { id: PROJECT } } }, project: { history: slice, projectBusy: false } } as unknown as WorkspaceSession;
  render(<WorkspaceSessionProvider session={session}><HistoryPanel /></WorkspaceSessionProvider>);
}

describe("unreadable history rows in the History panel (W-12)", () => {
  it("lists each by id and named refusal only, apart from the readable rows, which stay usable", () => {
    renderPanel(history());
    // The readable rows are listed as usual.
    expect(screen.getByTestId(`model-state-${STATE.state_id}`)).toBeInTheDocument();
    expect(screen.getByTestId(`recorded-run-${RUN.recording_id}`)).toBeInTheDocument();
    const block = screen.getByTestId("model-history-unreadable");
    for (const [kind, row] of [["state", TAMPERED_STATE], ["run", TAMPERED_RUN]] as const) {
      const item = within(block).getByTestId(`unreadable-${kind}-${row.handle}`);
      expect(item).toHaveTextContent(row.id);
      expect(within(item).getByTestId(`unreadable-${kind}-${row.handle}-refusal`)).toHaveTextContent(row.refusal);
      expect(within(item).getByTestId(`unreadable-${kind}-${row.handle}-guidance`)).toHaveTextContent("The stored row does not verify. It is not read; it can be deleted.");
      // Nothing that reads the row is offered: only delete.
      expect(within(item).getAllByRole("button").map((button) => button.textContent)).toEqual(["Delete…"]);
    }
    expect(block).toHaveTextContent("nothing of them is shown, opened, checked or compared");
    expect(screen.getByTestId("model-history-panel").textContent).not.toMatch(/certif|accept|approv/i);
  });

  it("deletes an unreadable row only after confirming", () => {
    const slice = history();
    renderPanel(slice);
    // By its handle, never its shown id (a cut id may name another row).
    fireEvent.click(screen.getByTestId(`unreadable-run-${TAMPERED_RUN.handle}-delete`));
    fireEvent.click(screen.getByTestId(`unreadable-run-${TAMPERED_RUN.handle}-delete-yes`));
    expect(slice.deleteUnreadable).toHaveBeenCalledWith("run", TAMPERED_RUN.handle);
    fireEvent.click(screen.getByTestId(`unreadable-state-${TAMPERED_STATE.handle}-delete`));
    fireEvent.click(screen.getByTestId(`unreadable-state-${TAMPERED_STATE.handle}-delete-yes`));
    expect(slice.deleteUnreadable).toHaveBeenCalledWith("state", TAMPERED_STATE.handle);
  });

  it("an unreadable list with no readable state shows no empty-history sentence", () => {
    renderPanel(history({ states: [], runs: [] }));
    expect(screen.queryByTestId("model-history-empty")).toBeNull();
    expect(screen.getByTestId("model-history-unreadable")).toBeInTheDocument();
  });

  it("a run recorded before rule-pack resolution says to re-record it", () => {
    renderPanel(history({ unreadable: { states: [], runs: [{ id: RUN.recording_id, refusal: S2_REFUSAL, handle: "run-row:4:39:00000000000000aa", references: 0 }] }, runCheck: { recordingId: RUN.recording_id, context: null, failure: S2_REFUSAL } }));
    expect(screen.getByTestId("unreadable-run-run-row:4:39:00000000000000aa-guidance")).toHaveTextContent("Re-record this run");
    const check = screen.getByTestId(`recorded-run-${RUN.recording_id}-check`);
    expect(check).toHaveTextContent(`Not read: ${S2_REFUSAL}`);
    expect(within(check).getByTestId(`recorded-run-${RUN.recording_id}-re-record`)).toHaveTextContent("Re-record this run: solve the model again and record the new run");
  });
});

describe("a readable run whose state cannot be read (W-12)", () => {
  it("is listed in its own block with check and delete; its unreadable state is deletable once nothing names it", () => {
    const orphan: AnalysisRunSummary = { ...RUN, recording_id: "runrec:00000000000000000000000000000007", state_id: TAMPERED_STATE.id, run_name: "Orphaned run" };
    const blockedState = { ...TAMPERED_STATE, references: 1 };
    const slice = history({ runs: [RUN, orphan], unreadable: { states: [blockedState], runs: [] } });
    renderPanel(slice);
    const block = screen.getByTestId("model-history-orphan-runs");
    const row = within(block).getByTestId(`recorded-run-${orphan.recording_id}`);
    expect(row).toHaveTextContent("Orphaned run");
    // Both rows are listed; the state's delete waits for the run.
    const stateDelete = screen.getByTestId(`unreadable-state-${blockedState.handle}-delete`);
    expect(stateDelete).toBeDisabled();
    expect(screen.getByTestId(`unreadable-state-${blockedState.handle}-references`)).toHaveTextContent("1 stored row names it; delete that row first.");
    fireEvent.click(within(row).getByTestId(`recorded-run-${orphan.recording_id}-delete`));
    fireEvent.click(within(row).getByTestId(`recorded-run-${orphan.recording_id}-delete-yes`));
    expect(slice.deleteRun).toHaveBeenCalledWith(orphan.recording_id);
    cleanup();
    // After the run's delete the list names no row of the state: it can be deleted.
    renderPanel(history({ runs: [RUN], unreadable: { states: [TAMPERED_STATE], runs: [] } }));
    expect(screen.queryByTestId("model-history-orphan-runs")).toBeNull();
    expect(screen.getByTestId(`unreadable-state-${TAMPERED_STATE.handle}-delete`)).toBeEnabled();
  });

  it("a readable state named by rows that cannot be read says so and keeps its delete blocked", () => {
    renderPanel(history({ states: [{ ...STATE, unreadable_references: 1 }], runs: [] }));
    expect(screen.getByTestId(`model-state-${STATE.state_id}-delete-blocked-unreadable`)).toHaveTextContent("1 stored row that cannot be read names it");
    expect(screen.getByTestId(`model-state-${STATE.state_id}-delete`)).toBeDisabled();
  });
});

describe("the history list services (W-12)", () => {
  it("split the native list into the readable summaries and the unreadable rows", async () => {
    (window as Json).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(async (command: string) => {
      if (command === "list_analysis_runs") return [RUN, { unreadable: TAMPERED_RUN }];
      if (command === "list_model_states") return [{ unreadable: TAMPERED_STATE }, STATE];
      throw new Error(`unexpected ${command}`);
    });
    expect(await listAnalysisRuns(PROJECT)).toEqual({ runs: [RUN], unreadable: [TAMPERED_RUN] });
    expect(await listModelStates(PROJECT)).toEqual({ states: [STATE], unreadable: [TAMPERED_STATE] });
  });
});
