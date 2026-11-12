// The two comparison views state the registered comparison statement and no review sentence
// of their own. The History panel that contains them states that human review is still
// required; this test holds both views inside that panel, with that sentence, so the
// statement on the views stays complete only while the sentence is kept. Invented data only.
import { cleanup, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import everySlotModel from "../../../../../fixtures/model_document/invented_every_slot_model.json";
import type { ModelStateSummary } from "../../services/modelStateService";
import { compareModelDocuments } from "../comparison/stateComparison";
import { compareRunInputs } from "../comparison/runComparison";
import type { RunComparisonSide } from "../comparison/runComparisonSides";
import { WorkspaceSessionProvider } from "../workspace/WorkspaceSessionContext";
import type { WorkspaceSession } from "../workspace/workspaceSession";
import { HistoryPanel } from "./HistoryPanel";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(cleanup);

const PROJECT = (everySlotModel as Json).project.id as string;
const COMPARISON_STATEMENT = "A comparison shows differences between two selected model states, internal analysis runs, or load cases or combinations within a run. It is a review aid and does not establish that either side is correct.";
const REVIEW_SENTENCE = "Human review is still required for any use of them.";

function state(n: number): ModelStateSummary {
  return {
    project_id: PROJECT, state_id: `state:${String(n).padStart(32, "0")}`, state_kind: "design_snapshot", created_at: `2026-10-24T10:00:0${n}.000Z`,
    parent_state_id: null, model_hash: `sha256:${String(n).repeat(64)}`, record_hash: "r", labels_updated_at: `2026-10-24T10:00:0${n}.000Z`,
    child_state_ids: [], recorded_run_count: 0, analysis_status: ["HUMAN_REVIEW_REQUIRED"], state_name: `Invented ${n}`, tags: [], notes: [], assumptions: []
  };
}

function side(name: string): RunComparisonSide {
  const standing = { contract: "preview_physics", status: "integrity_checked", eligible: true, findings: [] } as const;
  return {
    kind: "recorded", id: `runrec:${name}`, description: `"${name}" (runrec:${name})`, createdAt: "2026-10-25T10:00:00.000Z", state: null,
    standing: { ...standing, findings: [] }, recordedTableRead: null,
    input: { rows: [], standing: { ...standing, findings: [] }, semantic_identity: { route: "preview_physics", contract_id: "c", contract_sha256: "s" }, context: {}, diagnostics: [] }
  } as unknown as RunComparisonSide;
}

describe("the History panel's review sentence covers both comparison views", () => {
  it("renders the saved-state and run comparison views inside the panel that states human review is still required", () => {
    const model = structuredClone(everySlotModel) as Json;
    const changed = structuredClone(everySlotModel) as Json;
    changed.nodes.push({ id: "node:N-9", label: "Invented", position: { x: 1, y: 0, z: 0 } });
    const left = side("left");
    const right = side("right");
    const history = {
      states: [state(1), state(2)], runs: [], unreadable: { states: [], runs: [] }, runCheck: null, recordRunUnavailableReason: "solve first", recordRunState: null,
      historyMessage: "", historyOperation: "", historyBusy: false, basisStateId: null, refreshStates: vi.fn(), saveState: vi.fn(), openState: vi.fn(), editLabels: vi.fn(),
      deleteState: vi.fn(), recordRun: vi.fn(), editRunLabels: vi.fn(), deleteRun: vi.fn(), checkRun: vi.fn(), deleteUnreadable: vi.fn(),
      stateComparison: { projectId: PROJECT, left: state(1), right: state(2), comparison: compareModelDocuments(model, changed) }, compareStates: vi.fn(), closeStateComparison: vi.fn(),
      runComparison: { projectId: PROJECT, left, right, comparison: compareRunInputs(left.input, right.input) }, compareRuns: vi.fn(), closeRunComparison: vi.fn(),
      currentRunUnavailableReason: "Solve the model to compare the current run.", refusedRuns: {}
    } as unknown as WorkspaceSession["project"]["history"];
    const session = { model: { model: { project: { id: PROJECT } } }, project: { history, projectBusy: false } } as unknown as WorkspaceSession;
    render(<WorkspaceSessionProvider session={session}><HistoryPanel /></WorkspaceSessionProvider>);
    const panel = screen.getByTestId("model-history-panel");
    expect(within(panel).getByTestId("model-history-review-note")).toHaveTextContent(REVIEW_SENTENCE);
    for (const id of ["state-comparison", "run-comparison"]) {
      const view = within(panel).getByTestId(id);
      expect(view).toHaveTextContent(COMPARISON_STATEMENT);
      expect(view.textContent).not.toContain("human review is still required");
    }
  });
});
