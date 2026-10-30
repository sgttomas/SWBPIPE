// W-12 S2 (DEL-14-01 Design §3, §4.3): the History panel's recorded runs, under their
// states, with their labels, label editing, the record check, delete and Record Run…,
// against a fake of the session's history slice. The flows behind these controls are
// `recordRun.session.test.tsx`; the store's checks are the Rust tests.
import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import type { AnalysisRunSummary } from "../../services/analysisRunService";
import { LABELS_DO_NOT_CHANGE_RUN, RUN_RECORD_MANIFEST_UNAVAILABLE } from "../../services/analysisRunService";
import type { ModelStateSummary } from "../../services/modelStateService";
import type { HistoricalRunContext } from "../results/HistoricalRunContext";
import { WorkspaceSessionProvider } from "../workspace/WorkspaceSessionContext";
import type { WorkspaceSession } from "../workspace/workspaceSession";
import { HistoryPanel } from "./HistoryPanel";

const PROJECT = "project:invented-history";
const STATE: ModelStateSummary = {
  project_id: PROJECT, state_id: "state:00000000000000000000000000000001", state_kind: "post_solve_snapshot", created_at: "2026-10-24T10:00:01.000Z",
  parent_state_id: null, model_hash: `sha256:${"a".repeat(64)}`, record_hash: "r", labels_updated_at: "2026-10-24T10:00:01.000Z", child_state_ids: [],
  recorded_run_count: 1, analysis_status: ["HUMAN_REVIEW_REQUIRED"], state_name: "After solve", tags: [], notes: [], assumptions: []
};
const RUN: AnalysisRunSummary = {
  project_id: PROJECT, recording_id: "runrec:00000000000000000000000000000001", state_id: STATE.state_id, producer_run_id: "run:preview-linear-static-001",
  created_at: "2026-10-24T10:00:02.000Z", run_record_hash: "h", result_hash: "r", labels_updated_at: "2026-10-24T10:00:02.000Z",
  analysis_status: ["HUMAN_REVIEW_REQUIRED", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE"], run_name: "Run – rev A", tags: ["tie-in"], notes: ["Invented note."],
  assumptions: [{ assumption_id: "assumption:R-1", kind: "open_issue", status: "unresolved", statement: "Vendor stiffness pending.", affected_refs: [] }]
};

function history(overrides: Partial<WorkspaceSession["project"]["history"]> = {}) {
  return {
    states: [STATE], runs: [RUN], runCheck: null, recordRunUnavailableReason: null, recordRunState: STATE, historyMessage: "", historyOperation: "", historyBusy: false,
    basisStateId: null, refreshStates: vi.fn(), saveState: vi.fn(), openState: vi.fn(), editLabels: vi.fn(), deleteState: vi.fn(),
    recordRun: vi.fn(async () => true), editRunLabels: vi.fn(async () => true), deleteRun: vi.fn(async () => true), checkRun: vi.fn(async () => true),
    ...overrides
  } as unknown as WorkspaceSession["project"]["history"];
}

function renderPanel(slice: WorkspaceSession["project"]["history"], projectBusy = false) {
  const session = { model: { model: { project: { id: PROJECT } } }, project: { history: slice, projectBusy } } as unknown as WorkspaceSession;
  render(<WorkspaceSessionProvider session={session}><HistoryPanel /></WorkspaceSessionProvider>);
}

describe("recorded runs in the History panel (W-12 S2)", () => {
  it("lists each run under its state with its labels, and its state's delete names the run as a blocker", () => {
    renderPanel(history());
    const state = screen.getByTestId(`model-state-${STATE.state_id}`);
    const run = within(within(state).getByTestId(`model-state-${STATE.state_id}-runs`)).getByTestId(`recorded-run-${RUN.recording_id}`);
    expect(within(run).getByTestId(`recorded-run-${RUN.recording_id}-name`)).toHaveTextContent("Run – rev A");
    expect(run).toHaveTextContent("producer run run:preview-linear-static-001");
    expect(within(run).getByTestId(`recorded-run-${RUN.recording_id}-tags`)).toHaveTextContent("Tags: tie-in");
    expect(within(run).getByTestId(`recorded-run-${RUN.recording_id}-assumptions`)).toHaveTextContent("Open issue (unresolved): Vendor stiffness pending.");
    expect(within(state).getByTestId(`model-state-${STATE.state_id}-delete-blocked-runs`)).toHaveTextContent(`Delete is blocked while recorded run "Run – rev A" (${RUN.recording_id}) is tied to it.`);
    const stateDelete = within(state).getByTestId(`model-state-${STATE.state_id}-delete`);
    expect(stateDelete).toBeDisabled();
    expect(stateDelete).toHaveAttribute("title", "Recorded runs are tied to this state.");
    // No wording implies certification, acceptance or approval (F-PIP-2, F-PIP-3).
    expect(screen.getByTestId("model-history-panel").textContent).not.toMatch(/certif|accept|approv/i);
  });

  it("edits a run's labels with the sentence that they do not change the recorded run", () => {
    const slice = history();
    renderPanel(slice);
    fireEvent.click(screen.getByTestId(`recorded-run-${RUN.recording_id}-edit`));
    expect(screen.getByTestId("recorded-run-labels-note")).toHaveTextContent(LABELS_DO_NOT_CHANGE_RUN);
    expect(screen.getByTestId("recorded-run-labels-note")).toHaveTextContent("they are not part of its analysis record");
    fireEvent.change(screen.getByTestId(`recorded-run-${RUN.recording_id}-labels-name`), { target: { value: "Renamed" } });
    fireEvent.click(screen.getByTestId(`recorded-run-${RUN.recording_id}-labels-save`));
    expect(slice.editRunLabels).toHaveBeenCalledWith(RUN.recording_id, {
      run_name: "Renamed", tags: ["tie-in"], notes: ["Invented note."],
      assumptions: [{ assumption_id: "assumption:R-1", kind: "open_issue", status: "unresolved", statement: "Vendor stiffness pending.", affected_refs: [] }]
    });
  });

  it("deletes a run only after confirming, and checks its record on request", () => {
    const slice = history();
    renderPanel(slice);
    fireEvent.click(screen.getByTestId(`recorded-run-${RUN.recording_id}-delete`));
    expect(screen.getByTestId(`recorded-run-${RUN.recording_id}-delete-confirm`)).toHaveTextContent("Delete this recorded run and its labels? This cannot be undone.");
    fireEvent.click(screen.getByTestId(`recorded-run-${RUN.recording_id}-delete-yes`));
    expect(slice.deleteRun).toHaveBeenCalledWith(RUN.recording_id);
    fireEvent.click(screen.getByTestId(`recorded-run-${RUN.recording_id}-check-run`));
    expect(slice.checkRun).toHaveBeenCalledWith(RUN.recording_id);
  });

  it("shows a record check's outcome and says it establishes no current result", () => {
    const context = { findings: [], designation: "recorded_run" } as unknown as HistoricalRunContext;
    renderPanel(history({ runCheck: { recordingId: RUN.recording_id, context, failure: null } }));
    expect(screen.getByTestId(`recorded-run-${RUN.recording_id}-check`)).toHaveTextContent("The record, its stored result and input manifest, and its state's model hash agree. This is a record check only; it establishes no current result.");
  });

  it("Record Run… states why it is unavailable, and is disabled while busy with the reason", () => {
    renderPanel(history({ recordRunUnavailableReason: RUN_RECORD_MANIFEST_UNAVAILABLE }));
    const open = screen.getByTestId("record-run-open");
    expect(open).toBeDisabled();
    expect(open).toHaveAttribute("title", RUN_RECORD_MANIFEST_UNAVAILABLE);
    expect(screen.getByTestId("record-run-unavailable")).toHaveTextContent(`Record Run: ${RUN_RECORD_MANIFEST_UNAVAILABLE}`);
  });

  it("Record Run… is disabled with the busy reason while another project operation runs", () => {
    renderPanel(history(), true);
    expect(screen.getByTestId("record-run-open")).toBeDisabled();
    expect(screen.getByTestId("record-run-open")).toHaveAttribute("title", "Another project operation or prompt is in progress.");
  });

  it("records against the listed state, or asks for the post-solve snapshot's name", () => {
    const slice = history({ recordRunState: null });
    renderPanel(slice);
    fireEvent.click(screen.getByTestId("record-run-open"));
    expect(screen.getByTestId("record-run-state")).toHaveTextContent("No saved state holds the solved model.");
    fireEvent.change(screen.getByTestId("record-run-name"), { target: { value: "Run B" } });
    expect(screen.getByTestId("record-run-submit")).toBeDisabled();
    expect(screen.getByTestId("record-run-submit")).toHaveAttribute("title", "Name the post-solve snapshot the run is recorded against.");
    fireEvent.change(screen.getByTestId("record-run-snapshot-name"), { target: { value: "After solve" } });
    fireEvent.click(screen.getByTestId("record-run-submit"));
    expect(slice.recordRun).toHaveBeenCalledWith({ labels: { run_name: "Run B", tags: [], notes: [], assumptions: [] }, snapshotName: "After solve" });
  });

  it("names the listed state the run is recorded against", () => {
    const slice = history();
    renderPanel(slice);
    fireEvent.click(screen.getByTestId("record-run-open"));
    expect(screen.getByTestId("record-run-state")).toHaveTextContent(`The run is recorded against the saved state "After solve" (${STATE.state_id}), which holds the solved model.`);
    expect(screen.queryByTestId("record-run-snapshot-name")).toBeNull();
    fireEvent.change(screen.getByTestId("record-run-name"), { target: { value: "Run C" } });
    fireEvent.click(screen.getByTestId("record-run-submit"));
    expect(slice.recordRun).toHaveBeenCalledWith({ labels: { run_name: "Run C", tags: [], notes: [], assumptions: [] }, snapshotName: null });
  });
});
