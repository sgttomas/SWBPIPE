// The comparison overlay's session functions (DEL-07-08 REQ-011): `showComparisonOverlay(kind,
// first, second)` turns the overlay on from identifiers alone, comparing first when that comparison
// is not the one shown, and `hideComparisonOverlay()` turns it off, with no pointer event; a
// comparison closed, replaced or made stale takes its overlay with it; showing and leaving it
// changes nothing of the session's model. The History panel's control turns it on and off by
// keyboard, and the overlay's activation of an entity shows that entity's row in the table. The
// native store is faked as in `compareStates.test.tsx` and `compareRuns.test.tsx`. All data are invented.
import { act, cleanup, fireEvent, render, renderHook, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import everySlotModel from "../../../../../fixtures/model_document/invented_every_slot_model.json";
import type { AnalysisRunRead, AnalysisRunSummary, CurrentRunSlot } from "../../services/analysisRunService";
import type { ModelStateSummary } from "../../services/modelStateService";
import { recordedAnalysisRunRevision } from "../../services/analysisRunCompatibility";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { buildAnalysisRunPreview } from "../../services/previewService";
import { jsonTextKeepingNegativeZero } from "../../services/jsonText";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { CURRENT_RUN_ID } from "../comparison/runComparisonSides";
import { StateComparisonView } from "../comparison/StateComparisonView";
import { useModelStateHistory, type ModelStateHistoryDeps } from "./useModelStateHistory";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as Json).__TAURI_INTERNALS__;
});

function sessionHistory(projectId: string, overrides: Partial<ModelStateHistoryDeps> = {}) {
  const model = { project: { id: projectId }, nodes: [{ id: "node:WORKING", position: { x: 0, y: 0, z: 0 } }] };
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

// --- Two saved states.

const PROJECT = (everySlotModel as Json).project.id as string;
const LEFT_MODEL = structuredClone(everySlotModel) as Json;
const RIGHT_MODEL = (() => {
  const model = structuredClone(everySlotModel) as Json;
  model.supports[0].label = "Invented relabelled support";
  model.nodes.push({ id: "node:N-4", label: "Invented added node", position: { x: 9, y: 0, z: 0 }, provenance: "invented_example" });
  return model;
})();
function state(n: number, name: string): ModelStateSummary {
  return {
    project_id: PROJECT, state_id: `state:${String(n).padStart(32, "0")}`, state_kind: "design_snapshot", created_at: `2026-10-24T10:00:0${n}.000Z`,
    parent_state_id: null, model_hash: `sha256:${String(n).repeat(64)}`, record_hash: "r", labels_updated_at: `2026-10-24T10:00:0${n}.000Z`,
    child_state_ids: [], recorded_run_count: 0, analysis_status: ["HUMAN_REVIEW_REQUIRED"], state_name: name, tags: [], notes: [], assumptions: []
  };
}
const FIRST = state(1, "Basis");
const SECOND = state(2, "Revision");
const THIRD = state(3, "Later");

async function stateStore() {
  (window as Json).__TAURI_INTERNALS__ = {};
  const hashed = async (model: Json) => ({ text: await canonicalJsonString(model), model_hash: (await computeModelHash(model))!.value });
  const reads: Record<string, { text: string; model_hash: string }> = {
    [FIRST.state_id]: await hashed(LEFT_MODEL), [SECOND.state_id]: await hashed(RIGHT_MODEL), [THIRD.state_id]: await hashed(LEFT_MODEL)
  };
  invokeMock.mockImplementation(async (command: string, args: Record<string, unknown>) => {
    switch (command) {
      case "list_model_states": return [THIRD, SECOND, FIRST];
      case "list_analysis_runs": return [];
      case "admit_model_state_comparison": return { left_snapshot_bytes: 10, right_snapshot_bytes: 12, combined_bytes: 22, budget_bytes: 12 * 1024 ** 3 };
      case "read_model_state_for_comparison": {
        const id = args.stateId as string;
        const summary = [FIRST, SECOND, THIRD].find((candidate) => candidate.state_id === id)!;
        return { state: { ...summary, model_hash: reads[id].model_hash }, record: {}, model: JSON.parse(reads[id].text), model_document_version: { status: "current" } };
      }
      default: throw new Error(`unexpected ${command}`);
    }
  });
}

describe("the comparison overlay's session functions, for two saved states", () => {
  it("turn the overlay on from the two identifiers alone, comparing first, and off again; the model is unchanged", async () => {
    await stateStore();
    const { result: hook, deps } = sessionHistory(PROJECT);
    const modelBefore = JSON.stringify(deps.model);
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    expect(hook.current.stateComparison).toBeNull();
    let shown = false;
    await act(async () => { shown = await hook.current.showComparisonOverlay("states", SECOND.state_id, FIRST.state_id); });
    expect(shown, hook.current.historyMessage).toBe(true);
    expect(commands()).toEqual(["admit_model_state_comparison", "read_model_state_for_comparison", "read_model_state_for_comparison"]);
    const overlay = hook.current.comparisonOverlay!;
    expect(overlay.kind).toBe("states");
    expect(overlay.ids).toEqual([FIRST.state_id, SECOND.state_id]);
    expect(overlay.leftText).toBe(`"Basis" (${FIRST.state_id})`);
    expect(overlay.rightText).toBe(`"Revision" (${SECOND.state_id})`);
    // The marks are the shown comparison's rows: the same result the table reads.
    expect(overlay.scene.marks.map((mark) => [mark.ref, mark.markClass])).toEqual(hook.current.stateComparison!.comparison.rows.map((row) => [row.id, row.classification]));
    const marked = overlay.scene.drawn.filter((entity) => entity.classes[0] !== "unchanged").map((entity) => [entity.ref, entity.classes.join(" ")]);
    expect(marked).toEqual([["node:N-4", "added"], [(everySlotModel as Json).supports[0].id, "changed"]]);

    // Already shown: turned on again without another read.
    act(() => hook.current.hideComparisonOverlay());
    expect(hook.current.comparisonOverlay).toBeNull();
    expect(hook.current.stateComparison).not.toBeNull();
    await act(async () => { shown = await hook.current.showComparisonOverlay("states", FIRST.state_id, SECOND.state_id); });
    expect(shown).toBe(true);
    expect(commands()).toHaveLength(3);
    expect(hook.current.comparisonOverlay).not.toBeNull();
    act(() => hook.current.hideComparisonOverlay());
    expect(hook.current.comparisonOverlay).toBeNull();
    expect(JSON.stringify(deps.model)).toBe(modelBefore);
    expect(deps.currentModel.current).toBe(deps.model);
  });

  it("is taken with its comparison when that is closed or replaced", async () => {
    await stateStore();
    const { result: hook } = sessionHistory(PROJECT);
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    await act(async () => { await hook.current.showComparisonOverlay("states", FIRST.state_id, SECOND.state_id); });
    expect(hook.current.comparisonOverlay).not.toBeNull();
    act(() => hook.current.closeStateComparison());
    expect(hook.current.comparisonOverlay).toBeNull();
    // Comparing again does not bring back the overlay of the closed comparison.
    await act(async () => { await hook.current.compareStates(FIRST.state_id, SECOND.state_id); });
    expect(hook.current.comparisonOverlay).toBeNull();
    await act(async () => { await hook.current.showComparisonOverlay("states", FIRST.state_id, SECOND.state_id); });
    expect(hook.current.comparisonOverlay).not.toBeNull();
    // Another comparison replaces the shown one, and its overlay goes with it.
    await act(async () => { await hook.current.compareStates(FIRST.state_id, THIRD.state_id); });
    expect(hook.current.stateComparison!.right.state_id).toBe(THIRD.state_id);
    expect(hook.current.comparisonOverlay).toBeNull();
  });

  it("refuses an unknown kind or an unlisted state, and shows nothing", async () => {
    await stateStore();
    const { result: hook } = sessionHistory(PROJECT);
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    let shown = true;
    await act(async () => { shown = await hook.current.showComparisonOverlay("models" as never, FIRST.state_id, SECOND.state_id); });
    expect(shown).toBe(false);
    expect(hook.current.historyOperation).toBe("comparison_overlay_refused");
    await act(async () => { shown = await hook.current.showComparisonOverlay("states", FIRST.state_id, "state:unlisted"); });
    expect(shown).toBe(false);
    expect(hook.current.historyMessage).toBe("Not compared: select two different saved states of this project.");
    expect(hook.current.comparisonOverlay).toBeNull();
    expect(commands()).toEqual([]);
  });

  it("is turned on and off by the comparison view's control, and shows an activated entity's row in the table", async () => {
    await stateStore();
    const { result: hook } = sessionHistory(PROJECT);
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    await act(async () => { await hook.current.compareStates(FIRST.state_id, SECOND.state_id); });
    const link = () => ({
      shown: hook.current.comparisonOverlay !== null,
      unavailable: null,
      onShow: () => { void hook.current.showComparisonOverlay("states", FIRST.state_id, SECOND.state_id); },
      onHide: hook.current.hideComparisonOverlay,
      rowRequest: hook.current.comparisonRowRequest,
      onRowRequestHandled: hook.current.clearComparisonRowRequest
    });
    const view = render(<StateComparisonView result={hook.current.stateComparison!} onClose={() => {}} overlay={link()} />);
    const rerender = () => view.rerender(<StateComparisonView result={hook.current.stateComparison!} onClose={() => {}} overlay={link()} />);
    const toggle = screen.getByRole("button", { name: "Show on the model" });
    expect(toggle).toHaveAttribute("aria-pressed", "false");
    // A native button: Enter and Space press it as a click does.
    expect(toggle.tagName).toBe("BUTTON");
    await act(async () => { fireEvent.click(toggle); });
    rerender();
    expect(hook.current.comparisonOverlay).not.toBeNull();
    expect(toggle).toHaveAttribute("aria-pressed", "true");

    // The overlay activates an unchanged node: its row is on the "Differences only" filter's hidden side.
    const unchanged = (everySlotModel as Json).nodes[0].id as string;
    expect(screen.queryByTestId(`state-comparison-row-node-${unchanged}`)).toBeNull();
    act(() => hook.current.requestComparisonRow(unchanged, "node"));
    rerender();
    const row = screen.getByTestId(`state-comparison-row-node-${unchanged}`);
    expect(row).toHaveAttribute("data-overlay-focus", "true");
    expect(document.activeElement).toBe(row);
    expect(screen.getByTestId("state-comparison-filter-classification")).toHaveValue("all");
    // The request is spent once shown.
    expect(hook.current.comparisonRowRequest).toBeNull();

    fireEvent.click(toggle);
    rerender();
    expect(hook.current.comparisonOverlay).toBeNull();
    expect(toggle).toHaveAttribute("aria-pressed", "false");
  });
});

// --- Two runs (the recorded revisions are built by the session's own builders, as in compareRuns.test.tsx).

const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8")) as Json;
const MODEL = json("fixtures/product_preview/invented_preview_model.json") as PreviewModel;
const RUN_PROJECT = MODEL.project.id;
const SPARSE = json("fixtures/results/preview_physics_invented_sparse.json") as MechanicsResult;
const DENSE = json("fixtures/results/preview_physics_invented_dense.json") as MechanicsResult;
const STATE_ID = `state:${"a".repeat(32)}`;
const SETTINGS = { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: true };
type Solve = { model: PreviewModel; source: MechanicsResult; mode: "sparse_interactive" | "dense_scrutiny" };

async function transientOf(solve: Solve) {
  const manifest = await buildCurrentSessionInputManifest({
    model: solve.model,
    solver: {
      solver_name: solve.source.producer!.component_name,
      solver_version: solve.source.producer!.component_version,
      solver_build_ref: `${solve.source.producer!.component_name}@${solve.source.producer!.component_version}`,
      solver_mode: solve.mode,
      settings: SETTINGS
    },
    active_rule_packs: [],
    external_assets: []
  });
  const record = await buildAnalysisRunPreview(solve.source, { inputManifest: manifest });
  return { manifest, record };
}

async function recordedRead(solve: Solve, createdAt: string, name: string): Promise<AnalysisRunRead> {
  const { manifest, record } = await transientOf(solve);
  const recorded: AnalysisRunEnvelope = await recordedAnalysisRunRevision(record, { stateId: STATE_ID, createdAt, inputManifest: manifest.manifest });
  const hash = recorded.analysis_run.hashes.find((entry) => entry.payload_scope === "analysis_run_record")!.value;
  const modelHash = (await computeModelHash(solve.model))!.value;
  const run: AnalysisRunSummary = {
    project_id: RUN_PROJECT, recording_id: `runrec:${hash.slice(0, 32)}`, state_id: STATE_ID, producer_run_id: recorded.analysis_run.run_id, created_at: createdAt,
    run_record_hash: hash, result_hash: "r", labels_updated_at: createdAt, analysis_status: recorded.analysis_run.analysis_status,
    run_name: name, tags: [], notes: [], assumptions: []
  };
  const summary: ModelStateSummary = {
    project_id: RUN_PROJECT, state_id: STATE_ID, state_kind: "post_solve_snapshot", created_at: "2026-10-25T09:00:00.000Z", parent_state_id: null,
    model_hash: modelHash, record_hash: "r", labels_updated_at: "2026-10-25T09:00:00.000Z", child_state_ids: [], recorded_run_count: 2,
    analysis_status: ["HUMAN_REVIEW_REQUIRED"], state_name: "After solve", tags: [], notes: [], assumptions: []
  };
  return { run, state: summary, record: recorded, result_json: jsonTextKeepingNegativeZero(solve.source), input_manifest_canonical: manifest.canonical_bytes };
}

async function runStore(reads: AnalysisRunRead[]) {
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async (command: string, args: Record<string, unknown>) => {
    switch (command) {
      case "list_model_states": return [reads[0]?.state].filter(Boolean);
      case "list_analysis_runs": return reads.map((read) => read.run);
      case "admit_analysis_run_comparison": return { runs: [], current_run_bytes: 0, combined_bytes: 10, budget_bytes: 12 * 1024 ** 3 };
      case "read_analysis_run": return structuredClone(reads.find((read) => read.run.recording_id === args.recordingId)!);
      default: throw new Error(`unexpected ${command}`);
    }
  });
}

const EMPTY_SLOT: CurrentRunSlot = { result: null, analysisRun: null, inputManifest: null, restored: false };

describe("the comparison overlay's session functions, for two runs", () => {
  it("turn the overlay on from a recorded run's id and the current run's, drawing the run's solved model, and close it with a stale comparison", async () => {
    const earlier = await recordedRead({ model: MODEL, source: SPARSE, mode: "sparse_interactive" }, "2026-10-25T10:00:00.000Z", "Sparse");
    const current = await transientOf({ model: MODEL, source: DENSE, mode: "dense_scrutiny" });
    const live: CurrentRunSlot = { result: DENSE, analysisRun: current.record, inputManifest: current.manifest, restored: false };
    await runStore([earlier]);
    const { result: hook, deps, rerender } = sessionHistory(RUN_PROJECT, { runSlot: live });
    await act(async () => { await hook.current.refreshStates(RUN_PROJECT); });
    let shown = false;
    await act(async () => { shown = await hook.current.showComparisonOverlay("runs", CURRENT_RUN_ID, earlier.run.recording_id); });
    expect(shown, hook.current.historyMessage).toBe(true);
    const overlay = hook.current.comparisonOverlay!;
    expect(overlay.kind).toBe("runs");
    expect(overlay.ids).toEqual([earlier.run.recording_id, CURRENT_RUN_ID]);
    // The later run's solved model is drawn: every member of the model it is bound to.
    const drawnMembers = overlay.scene.drawn.filter((entity) => entity.family === "pipe_segment" && !entity.ghost).map((entity) => entity.ref).sort();
    expect(drawnMembers).toEqual(MODEL.pipe_segments.map((pipe) => pipe.id).sort());
    // The two solver modes give differing displacements: some member or node is marked as differing.
    expect(overlay.scene.counts.find((count) => count.markClass === "nonzero_delta")!.total).toBeGreaterThan(0);
    // A row naming no entity of either model (as the solver's) is counted, not drawn.
    expect(overlay.scene.notDrawn.every((entry) => entry.reason === "not_in_either_model")).toBe(true);

    // Another solve replaces the current run: the comparison closes, and its overlay with it.
    (deps as Json).runSlot = { ...live, result: structuredClone(DENSE) };
    rerender();
    expect(hook.current.runComparison).toBeNull();
    expect(hook.current.comparisonOverlay).toBeNull();
    (deps as Json).runSlot = EMPTY_SLOT;
  });
});
