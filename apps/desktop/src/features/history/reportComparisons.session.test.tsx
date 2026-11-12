// DEL-08-06 REQ-015, AC-009: the saved-state and run comparisons a person includes in the
// report. The session holds at most one of each kind, by its two identifiers, whichever
// comparison is shown; the project changing removes both; a run comparison with a current
// side leaves the report when that run changes. The included comparisons become records
// (`comparisonRecords.ts`) whose sections join the package's section set. The flows run
// against fakes of the native commands over committed producer results and an invented
// model document; all data are invented.
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
import { canonicalJsonString, canonicalSha256Hex, computeModelHash } from "../../services/hashService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { buildAnalysisRunPreview } from "../../services/previewService";
import { jsonTextKeepingNegativeZero } from "../../services/jsonText";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { includedRunComparison, includedStateComparison, reportComparisonRecords, runComparisonRecord, stateComparisonRecord } from "../comparison/comparisonRecords";
import { CURRENT_RUN_ID } from "../comparison/runComparisonSides";
import { StateComparisonView } from "../comparison/StateComparisonView";
import { buildComparisonSectionSet } from "../report/comparisonSections";
import { useModelStateHistory, type ModelStateHistoryDeps } from "./useModelStateHistory";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as Json).__TAURI_INTERNALS__;
});

const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8")) as Json;

// ---------------------------------------------------------------------------
// Saved states
// ---------------------------------------------------------------------------
const STATE_PROJECT = (everySlotModel as Json).project.id as string;
const LEFT_MODEL = structuredClone(everySlotModel) as Json;
const RIGHT_MODEL = (() => {
  const model = structuredClone(everySlotModel) as Json;
  model.materials[0].elastic_modulus = { value: 200000, unit: "MPa" };
  model.nodes.push({ id: "node:N-4", label: "Invented added node", position: { x: 9, y: 0, z: 0 } });
  model.lines.pop();
  return model;
})();

function state(n: number, name: string): ModelStateSummary {
  return {
    project_id: STATE_PROJECT, state_id: `state:${String(n).padStart(32, "0")}`, state_kind: "design_snapshot", created_at: `2026-10-24T10:00:0${n}.000Z`,
    parent_state_id: null, model_hash: `sha256:${String(n).repeat(64)}`, record_hash: "r", labels_updated_at: `2026-10-24T10:00:0${n}.000Z`,
    child_state_ids: [], recorded_run_count: 0, analysis_status: ["HUMAN_REVIEW_REQUIRED"], state_name: name, tags: [], notes: [{ statement: "Invented private note" }] as Json, assumptions: []
  };
}
const FIRST = state(1, "Invented label Basis");
const SECOND = state(2, "Invented label Revision");

async function stateStore() {
  (window as Json).__TAURI_INTERNALS__ = {};
  const hashed = async (model: Json) => ({ text: await canonicalJsonString(model), model_hash: (await computeModelHash(model))!.value });
  const reads: Record<string, { text: string; model_hash: string }> = { [FIRST.state_id]: await hashed(LEFT_MODEL), [SECOND.state_id]: await hashed(RIGHT_MODEL) };
  invokeMock.mockImplementation(async (command: string, args: Record<string, unknown>) => {
    switch (command) {
      case "list_model_states": return [SECOND, FIRST];
      case "list_analysis_runs": return [];
      case "admit_model_state_comparison": return { left_snapshot_bytes: 10, right_snapshot_bytes: 12, combined_bytes: 22, budget_bytes: 12 * 1024 ** 3 };
      case "read_model_state_for_comparison": {
        const id = args.stateId as string;
        const summary = [FIRST, SECOND].find((candidate) => candidate.state_id === id)!;
        return { state: { ...summary, model_hash: reads[id].model_hash }, record: {}, model: JSON.parse(reads[id].text), model_document_version: { status: "current" } };
      }
      default: throw new Error(`unexpected ${command}`);
    }
  });
}

// ---------------------------------------------------------------------------
// Runs
// ---------------------------------------------------------------------------
const RUN_MODEL = json("fixtures/product_preview/invented_preview_model.json") as PreviewModel;
const RUN_PROJECT = RUN_MODEL.project.id;
const SPARSE = json("fixtures/results/preview_physics_invented_sparse.json") as MechanicsResult;
const DENSE = json("fixtures/results/preview_physics_invented_dense.json") as MechanicsResult;
const RUN_STATE_ID = `state:${"a".repeat(32)}`;
const SETTINGS = { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: true };

async function transientOf(source: MechanicsResult, mode: "sparse_interactive" | "dense_scrutiny") {
  const manifest = await buildCurrentSessionInputManifest({
    model: RUN_MODEL,
    solver: { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: `${source.producer!.component_name}@${source.producer!.component_version}`, solver_mode: mode, settings: SETTINGS },
    active_rule_packs: [],
    external_assets: []
  });
  return { manifest, record: await buildAnalysisRunPreview(source, { inputManifest: manifest }) };
}

async function recordedRead(source: MechanicsResult, mode: "sparse_interactive" | "dense_scrutiny", createdAt: string): Promise<AnalysisRunRead> {
  const { manifest, record } = await transientOf(source, mode);
  const recorded: AnalysisRunEnvelope = await recordedAnalysisRunRevision(record, { stateId: RUN_STATE_ID, createdAt, inputManifest: manifest.manifest });
  const hash = recorded.analysis_run.hashes.find((item) => item.payload_scope === "analysis_run_record")!.value;
  const modelHash = (await computeModelHash(RUN_MODEL))!.value;
  const run: AnalysisRunSummary = {
    project_id: RUN_PROJECT, recording_id: `runrec:${hash.slice(0, 32)}`, state_id: RUN_STATE_ID, producer_run_id: recorded.analysis_run.run_id, created_at: createdAt,
    run_record_hash: hash, result_hash: "e".repeat(64), labels_updated_at: createdAt, analysis_status: recorded.analysis_run.analysis_status,
    run_name: "Invented run label", tags: [], notes: [], assumptions: []
  };
  const summary: ModelStateSummary = {
    project_id: RUN_PROJECT, state_id: RUN_STATE_ID, state_kind: "post_solve_snapshot", created_at: "2026-10-25T09:00:00.000Z", parent_state_id: null,
    model_hash: modelHash, record_hash: "r", labels_updated_at: "2026-10-25T09:00:00.000Z", child_state_ids: [], recorded_run_count: 1,
    analysis_status: ["HUMAN_REVIEW_REQUIRED"], state_name: "After solve", tags: [], notes: [], assumptions: []
  };
  return { run, state: summary, record: recorded, result_json: jsonTextKeepingNegativeZero(source), input_manifest_canonical: manifest.canonical_bytes };
}

async function runStore(reads: AnalysisRunRead[]) {
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async (command: string, args: Record<string, unknown>) => {
    switch (command) {
      case "list_model_states": return [reads[0].state];
      case "list_analysis_runs": return reads.map((read) => read.run);
      case "admit_analysis_run_comparison": return { runs: [], current_run_bytes: 0, combined_bytes: 10, budget_bytes: 12 * 1024 ** 3 };
      case "read_analysis_run": return structuredClone(reads.find((read) => read.run.recording_id === args.recordingId)!);
      default: throw new Error(`unexpected ${command}`);
    }
  });
}

const EMPTY_SLOT: CurrentRunSlot = { result: null, analysisRun: null, inputManifest: null, restored: false };

function sessionHistory(projectId: string, overrides: Partial<ModelStateHistoryDeps> = {}) {
  const model = { project: { id: projectId } };
  const owner = Symbol("owner");
  const deps = {
    model, currentModel: { current: model }, projectSessionGeneration: 0, projectSessionGenerationRef: { current: 0 },
    runSlot: EMPTY_SLOT, projectSessionBusy: () => false, acquireProjectOperation: vi.fn(() => owner), releaseProjectOperation: vi.fn(),
    ...overrides
  } as unknown as ModelStateHistoryDeps;
  return { deps, ...renderHook(() => useModelStateHistory(deps)) };
}

describe("including a saved-state comparison in the report", () => {
  it("includes the comparison of two identifiers made in this session, keeps it when another is shown or it is closed, and removes it", async () => {
    await stateStore();
    const { result: hook } = sessionHistory(STATE_PROJECT);
    await act(async () => { await hook.current.refreshStates(STATE_PROJECT); });
    // Nothing compared yet: refused, with the reason.
    let included = true;
    act(() => { included = hook.current.includeComparisonInReport("state", FIRST.state_id, SECOND.state_id); });
    expect(included).toBe(false);
    expect(hook.current.historyOperation).toBe("report_comparison_refused");
    expect(hook.current.reportComparisons).toEqual({ state: null, run: null });
    await act(async () => { await hook.current.compareStates(FIRST.state_id, SECOND.state_id); });
    const shown = hook.current.stateComparison!;
    // Either order of the two identifiers; another pair is refused.
    act(() => { included = hook.current.includeComparisonInReport("state", FIRST.state_id, `state:${"9".repeat(32)}`); });
    expect(included).toBe(false);
    act(() => { included = hook.current.includeComparisonInReport("state", SECOND.state_id, FIRST.state_id); });
    expect(included).toBe(true);
    expect(hook.current.historyOperation).toBe("report_comparison_included");
    const held = { kind: "state", projectId: STATE_PROJECT, leftId: FIRST.state_id, rightId: SECOND.state_id, currentRun: null, record: stateComparisonRecord(shown) };
    expect(hook.current.reportComparisons.state).toEqual(held);
    expect(hook.current.reportComparisonsNow().state).toEqual(held);
    // Only identifiers and the record are held: no side's snapshot or rows.
    expect(Object.keys(hook.current.reportComparisons.state!).sort()).toEqual(["currentRun", "kind", "leftId", "projectId", "record", "rightId"]);
    // Closing the view keeps it in the report.
    act(() => hook.current.closeStateComparison());
    expect(hook.current.stateComparison).toBeNull();
    expect(hook.current.reportComparisons.state).toEqual(held);
    // Once closed, it cannot be included again without comparing again.
    let again = true;
    act(() => { again = hook.current.includeComparisonInReport("state", FIRST.state_id, SECOND.state_id); });
    expect(again).toBe(false);
    expect(hook.current.reportComparisons.state).toEqual(held);
    let removed = false;
    act(() => { removed = hook.current.removeComparisonFromReport("state"); });
    expect(removed).toBe(true);
    expect(hook.current.reportComparisons.state).toBeNull();
    act(() => { removed = hook.current.removeComparisonFromReport("state"); });
    expect(removed).toBe(false);
  });

  it("removes every included comparison when the project changes", async () => {
    await stateStore();
    const { result: hook, deps, rerender } = sessionHistory(STATE_PROJECT);
    await act(async () => { await hook.current.refreshStates(STATE_PROJECT); });
    await act(async () => { await hook.current.compareStates(FIRST.state_id, SECOND.state_id); });
    act(() => { hook.current.includeComparisonInReport("state", FIRST.state_id, SECOND.state_id); });
    expect(hook.current.reportComparisons.state).not.toBeNull();
    const other = { project: { id: "project:invented-other" } };
    (deps as Json).model = other;
    (deps as Json).currentModel.current = other;
    await act(async () => { rerender(); });
    expect(hook.current.reportComparisons).toEqual({ state: null, run: null });
    expect(hook.current.reportComparisonsNow()).toEqual({ state: null, run: null });
    // An earlier comparison of the old project cannot be included afterwards.
    let included = true;
    act(() => { included = hook.current.includeComparisonInReport("state", FIRST.state_id, SECOND.state_id); });
    expect(included).toBe(false);
  });

  it("restates the comparison as a record of identifiers, hashes, paths and units, with no label or value", async () => {
    await stateStore();
    const { result: hook } = sessionHistory(STATE_PROJECT);
    await act(async () => { await hook.current.refreshStates(STATE_PROJECT); });
    await act(async () => { await hook.current.compareStates(FIRST.state_id, SECOND.state_id); });
    const shown = hook.current.stateComparison!;
    const record = stateComparisonRecord(shown) as Json;
    const text = JSON.stringify(record);
    for (const label of ["Invented label", "Invented private note", "Invented added node", "200000"]) expect(text).not.toContain(label);
    expect(record.comparison_id).toBe(`model-state-comparison:${FIRST.state_id}:${SECOND.state_id}`);
    expect(record.left_state_ref).toEqual({ object_type: "ModelState", ref: FIRST.state_id, model_hash: shown.left.model_hash });
    expect(shown.left.model_hash).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(record.entities.map((row: Json) => row.classification).sort()).toEqual(
      shown.comparison.rows.filter((row) => row.classification !== "unchanged").map((row) => row.classification).sort()
    );
    expect(record.summary.unchanged).toBe(shown.comparison.summary.unchanged);
    expect(record.entities.find((row: Json) => row.left_ref?.object_type === "material").changes).toEqual([
      expect.objectContaining({ path: "elastic_modulus", change_kind: expect.any(String), left_unit: expect.any(String), right_unit: "MPa" })
    ]);
    expect(record.settings.tolerance).toBe("not_tolerance_checked");
    const { model_state_comparisons: [hashed] } = await reportComparisonRecords({ state: includedStateComparison(shown), run: null });
    const { hashes, ...body } = hashed as Json;
    expect(body).toEqual(record);
    expect(hashes).toEqual([{ algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_ref: { object_type: "ModelStateComparison", ref: record.comparison_id }, payload_scope: "comparison_record_without_hashes", value: await canonicalSha256Hex(record) }]);
  });

  it("offers Include in report on the view, by keyboard-operable button, with its state in words", async () => {
    await stateStore();
    const { result: hook } = sessionHistory(STATE_PROJECT);
    await act(async () => { await hook.current.refreshStates(STATE_PROJECT); });
    await act(async () => { await hook.current.compareStates(FIRST.state_id, SECOND.state_id); });
    const shown = hook.current.stateComparison!;
    const include = vi.fn();
    const remove = vi.fn();
    const view = render(<StateComparisonView result={shown} onClose={() => {}} reportInclusion={{ included: false, include, remove }} />);
    const toggle = screen.getByTestId("state-comparison-report-toggle");
    expect(toggle.tagName).toBe("BUTTON");
    expect(toggle).toHaveAttribute("aria-pressed", "false");
    expect(toggle).toHaveTextContent("Include in report");
    expect(screen.getByTestId("state-comparison-report-state")).toHaveTextContent("Not included in a report package.");
    fireEvent.click(toggle);
    expect(include).toHaveBeenCalledTimes(1);
    view.rerender(<StateComparisonView result={shown} onClose={() => {}} reportInclusion={{ included: true, include, remove }} />);
    expect(screen.getByTestId("state-comparison-report-toggle")).toHaveAttribute("aria-pressed", "true");
    expect(screen.getByTestId("state-comparison-report-state")).toHaveTextContent("Included: the next local private report package saved holds this saved-state comparison.");
    fireEvent.click(screen.getByTestId("state-comparison-report-toggle"));
    expect(remove).toHaveBeenCalledTimes(1);
    // Without a session that holds the report's comparisons, no control is offered.
    view.rerender(<StateComparisonView result={shown} onClose={() => {}} />);
    expect(screen.queryByTestId("state-comparison-report")).toBeNull();
  });
});

describe("including a run comparison in the report", () => {
  it("holds its record, keeps it when the view closes, and its sections join a set without blocking diagnostics", async () => {
    const earlier = await recordedRead(SPARSE, "sparse_interactive", "2026-10-25T10:00:00.000Z");
    const later = await recordedRead(DENSE, "dense_scrutiny", "2026-10-25T11:00:00.000Z");
    await runStore([earlier, later]);
    const { result: hook } = sessionHistory(RUN_PROJECT);
    await act(async () => { await hook.current.refreshStates(RUN_PROJECT); });
    await act(async () => { await hook.current.compareRuns(earlier.run.recording_id, later.run.recording_id); });
    const shown = hook.current.runComparison!;
    let included = false;
    act(() => { included = hook.current.includeComparisonInReport("run", later.run.recording_id, earlier.run.recording_id); });
    expect(included).toBe(true);
    const record = runComparisonRecord(shown) as Json;
    expect(hook.current.reportComparisons.run).toEqual({ kind: "run", projectId: RUN_PROJECT, leftId: earlier.run.recording_id, rightId: later.run.recording_id, currentRun: null, record });
    act(() => hook.current.closeRunComparison());
    expect(hook.current.reportComparisons.run?.record).toEqual(record);
    expect(record.run_context.left).toEqual({ object_type: "AnalysisRun", ref: earlier.run.recording_id, side_kind: "recorded_run", run_record_hash: earlier.run.run_record_hash, result_hash: earlier.run.result_hash });
    expect(record.result_deltas.length).toBe(shown.comparison.summary.compared);
    expect(record.result_deltas.every((delta: Json) => delta.dimension && delta.normalized_unit && delta.classification === "not_tolerance_checked")).toBe(true);
    expect(record.run_context.not_compared.length).toBe(shown.comparison.summary.not_compared);
    expect(record.run_context.unmatched.length).toBe(shown.comparison.unmatched.length);
    expect(JSON.stringify(record)).not.toContain("Invented run label");
    for (const entry of [...record.run_context.not_compared, ...record.run_context.unmatched]) expect(entry).not.toHaveProperty("value");
    const records = await reportComparisonRecords({ state: null, run: includedRunComparison(shown) });
    const set = buildComparisonSectionSet({
      section_set_id: "invented", model_states: [], analysis_runs: [], handoff_packages: [], export_workflows: [], external_prover_metadata: [],
      source_notes: [], provenance: null, ...records
    }) as Json;
    expect(set.diagnostics.filter((item: Json) => item.severity === "blocking")).toEqual([]);
    expect(set.sections.comparison_sections[0].hash_refs).toEqual((records.analysis_run_comparisons[0] as Json).hashes);
  });

  it("leaves the report when its current run changes, by the rule that closes it", async () => {
    const earlier = await recordedRead(SPARSE, "sparse_interactive", "2026-10-25T10:00:00.000Z");
    const current = await transientOf(DENSE, "dense_scrutiny");
    const live: CurrentRunSlot = { result: DENSE, analysisRun: current.record, inputManifest: current.manifest, restored: false };
    await runStore([earlier]);
    const { result: hook, deps, rerender } = sessionHistory(RUN_PROJECT, { runSlot: live });
    await act(async () => { await hook.current.refreshStates(RUN_PROJECT); });
    await act(async () => { await hook.current.compareRuns(CURRENT_RUN_ID, earlier.run.recording_id); });
    act(() => { hook.current.includeComparisonInReport("run", CURRENT_RUN_ID, earlier.run.recording_id); });
    const record = hook.current.reportComparisons.run!.record as Json;
    expect(record.run_context.right).toMatchObject({ ref: CURRENT_RUN_ID, side_kind: "current_session_run", run_record_hash: "not_recorded" });
    (deps as Json).runSlot = { ...live, result: structuredClone(DENSE), analysisRun: structuredClone(current.record) };
    await act(async () => { rerender(); });
    expect(hook.current.reportComparisons.run).toBeNull();
    expect(hook.current.reportComparisonsNow().run).toBeNull();
    expect(hook.current.historyOperation).toBe("report_comparison_current_changed");
  });
});
