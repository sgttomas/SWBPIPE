// W-12 S3b (DEL-14-01 Design `desktop-states-runs-comparison.md` §4.2, §4.3):
// Compare… of two runs. The session's flow (the one-owner check, the busy
// refusal, the native size admission before either read, the metered reads, the
// webview's exactness check, the S2-era refusal) runs against a fake of the native
// commands over recorded revisions built by the session's own builders from
// committed producer results. The History panel offers two recorded runs, or one
// and the current run, and disables Compare… with the stated reason; the run
// comparison view shows each side's standing, the counts, the context deltas, the
// diagnostics, and a filterable, paged table. All data are invented.
import { act, cleanup, fireEvent, render, renderHook, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { createHash } from "node:crypto";
import type { AnalysisRunRead, AnalysisRunSummary, CurrentRunSlot } from "../../services/analysisRunService";
import type { ModelStateSummary } from "../../services/modelStateService";
import { recordedAnalysisRunRevision } from "../../services/analysisRunCompatibility";
import { canonicalJsonString, canonicalSha256HexCheckedV1, computeModelHash } from "../../services/hashService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { buildAnalysisRunPreview } from "../../services/previewService";
import { jsonTextKeepingNegativeZero } from "../../services/jsonText";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { WorkspaceSessionProvider } from "../workspace/WorkspaceSessionContext";
import type { WorkspaceSession } from "../workspace/workspaceSession";
import { compareRunInputs, type FlatRunRow, type RunRowIdentity } from "../comparison/runComparison";
import { CURRENT_RUN_ID, currentRunBytes, type RunComparisonSide } from "../comparison/runComparisonSides";
import { RunComparisonView } from "../comparison/RunComparisonView";
import { COMPARISON_PAGE_ROWS } from "../comparison/StateComparisonView";
import { HistoryPanel } from "./HistoryPanel";
import { COMPARISON_CURRENT_RUN_CHANGED, runCompareUnavailableReason, useModelStateHistory, type ModelStateHistoryDeps, type RunComparisonResult } from "./useModelStateHistory";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as Json).__TAURI_INTERNALS__;
});

const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8")) as Json;
const sha = (text: string) => createHash("sha256").update(text, "utf8").digest("hex");

const MODEL = json("fixtures/product_preview/invented_preview_model.json") as PreviewModel;
const PROJECT = MODEL.project.id;
const SPARSE = json("fixtures/results/preview_physics_invented_sparse.json") as MechanicsResult;
const DENSE = json("fixtures/results/preview_physics_invented_dense.json") as MechanicsResult;
const STATE_ID = `state:${"a".repeat(32)}`;
const SETTINGS = { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: true };

type Solve = { model: PreviewModel; source: MechanicsResult; mode: "sparse_interactive" | "dense_scrutiny" };

/** The session's transient record and manifest of a solve, as `workspaceSession.ts` builds them. */
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

function stateSummary(modelHash: string): ModelStateSummary {
  return {
    project_id: PROJECT, state_id: STATE_ID, state_kind: "post_solve_snapshot", created_at: "2026-10-25T09:00:00.000Z", parent_state_id: null,
    model_hash: modelHash, record_hash: "r", labels_updated_at: "2026-10-25T09:00:00.000Z", child_state_ids: [], recorded_run_count: 2,
    analysis_status: ["HUMAN_REVIEW_REQUIRED"], state_name: "After solve", tags: [], notes: [], assumptions: []
  };
}

function recordHashOf(record: AnalysisRunEnvelope): string {
  return record.analysis_run.hashes.find((hash) => hash.payload_scope === "analysis_run_record")!.value;
}

/** A recorded run as `read_analysis_run` returns it: the recorded revision the store
 * writes (the TypeScript builder gives the store's witness), the result's text as
 * stored, the manifest's canonical text and the state's summary. */
async function recordedRead(solve: Solve, createdAt: string, name: string, options: { record?: (record: AnalysisRunEnvelope) => Promise<AnalysisRunEnvelope>; resultText?: string } = {}): Promise<AnalysisRunRead> {
  const { manifest, record } = await transientOf(solve);
  let recorded = await recordedAnalysisRunRevision(record, { stateId: STATE_ID, createdAt, inputManifest: manifest.manifest });
  if (options.record) recorded = await options.record(recorded);
  const hash = recordHashOf(recorded);
  const modelHash = (await computeModelHash(solve.model))!.value;
  const run: AnalysisRunSummary = {
    project_id: PROJECT, recording_id: `runrec:${hash.slice(0, 32)}`, state_id: STATE_ID, producer_run_id: recorded.analysis_run.run_id, created_at: createdAt,
    run_record_hash: hash, result_hash: "r", labels_updated_at: createdAt, analysis_status: recorded.analysis_run.analysis_status,
    run_name: name, tags: [], notes: [], assumptions: []
  };
  return { run, state: stateSummary(modelHash), record: recorded, result_json: options.resultText ?? jsonTextKeepingNegativeZero(solve.source), input_manifest_canonical: manifest.canonical_bytes };
}

const EARLY = "2026-10-25T10:00:00.000Z";
const LATE = "2026-10-25T11:00:00.000Z";

async function nativeStore(reads: AnalysisRunRead[], options: { admission?: () => unknown; read?: (id: string) => unknown } = {}) {
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async (command: string, args: Record<string, unknown>) => {
    switch (command) {
      case "list_model_states": return [reads[0]?.state].filter(Boolean);
      case "list_analysis_runs": return reads.map((read) => read.run);
      case "admit_analysis_run_comparison":
        if (options.admission) return options.admission();
        return { runs: [], current_run_bytes: 0, combined_bytes: 10, budget_bytes: 12 * 1024 ** 3 };
      case "read_analysis_run": {
        const id = args.recordingId as string;
        if (options.read) { const replaced = options.read(id); if (replaced !== undefined) return replaced; }
        return structuredClone(reads.find((read) => read.run.recording_id === id)!);
      }
      default: throw new Error(`unexpected ${command}`);
    }
  });
}

const EMPTY_SLOT: CurrentRunSlot = { result: null, analysisRun: null, inputManifest: null, restored: false };

function sessionHistory(overrides: Partial<ModelStateHistoryDeps> = {}, projectId = PROJECT) {
  const model = { project: { id: projectId } };
  const owner = Symbol("owner");
  const deps = {
    model, currentModel: { current: model }, projectSessionGeneration: 0, projectSessionGenerationRef: { current: 0 },
    runSlot: EMPTY_SLOT, projectSessionBusy: () => false, acquireProjectOperation: vi.fn(() => owner), releaseProjectOperation: vi.fn(),
    ...overrides
  } as unknown as ModelStateHistoryDeps;
  return { deps, ...renderHook(() => useModelStateHistory(deps)) };
}

const commands = () => invokeMock.mock.calls.map(([command]) => command as string).filter((command) => !command.startsWith("list_"));

describe("the session's Compare… of two runs", () => {
  it("admits the size, reads both recorded runs and compares them, the earlier-recorded on the left", async () => {
    const later = await recordedRead({ model: MODEL, source: DENSE, mode: "dense_scrutiny" }, LATE, "Dense");
    const earlier = await recordedRead({ model: MODEL, source: SPARSE, mode: "sparse_interactive" }, EARLY, "Sparse");
    await nativeStore([later, earlier]);
    const { result: hook, deps } = sessionHistory();
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    let compared = false;
    await act(async () => { compared = await hook.current.compareRuns(later.run.recording_id, earlier.run.recording_id); });
    expect(compared, hook.current.historyMessage).toBe(true);
    expect(commands()).toEqual(["admit_analysis_run_comparison", "read_analysis_run", "read_analysis_run"]);
    const admission = invokeMock.mock.calls.find(([command]) => command === "admit_analysis_run_comparison")![1];
    expect(admission).toEqual({ projectId: PROJECT, recordingIds: [earlier.run.recording_id, later.run.recording_id] });
    const shown = hook.current.runComparison!;
    expect([shown.left.id, shown.right.id]).toEqual([earlier.run.recording_id, later.run.recording_id]);
    // Both recorded runs are read from their stored result and snapshot: integrity_checked.
    expect([shown.left.standing.status, shown.right.standing.status]).toEqual(["integrity_checked", "integrity_checked"]);
    expect(shown.comparison.run_gate).toEqual([]);
    expect(shown.comparison.summary.compared_nonzero).toBeGreaterThan(0);
    // The context: the solver mode differs; the settings and load basis do not.
    expect(shown.comparison.context_deltas).toEqual([{ key: "solver_mode", left: "sparse_interactive", right: "dense_scrutiny", kind: "changed" }]);
    const { compared: pairs, not_compared, unmatched_left, unmatched_right } = shown.comparison.summary;
    expect(hook.current.historyMessage).toBe(`Compared "Sparse" (${earlier.run.recording_id}) with "Dense" (${later.run.recording_id}): ${pairs} row pairs compared, ${not_compared} not compared, ${unmatched_left} only in the left run, ${unmatched_right} only in the right run. Neither run changes.`);
    expect(deps.acquireProjectOperation).toHaveBeenCalledTimes(1);
    expect(deps.releaseProjectOperation).toHaveBeenCalledTimes(1);
    act(() => hook.current.closeRunComparison());
    expect(hook.current.runComparison).toBeNull();
  });

  // Owner ruling, 2026-10-26, "R-c1 A, 0.8.0 a2" (DEL-02-01 Design model-document-0.8.0.md §3.2):
  // a run recorded on a 0.7.0 model is listed for the project id and compares with a 0.8.0 run
  // exactly as two 0.8.0 runs do. No model-document-version gate is added for runs; the
  // existing eligibility gate decides.
  it("compares a run recorded on a 0.7.0 model with a 0.8.0 run under the existing gate alone", async () => {
    const retired = { ...MODEL, schema_version: "0.7.0" } as PreviewModel;
    const earlier = await recordedRead({ model: retired, source: SPARSE, mode: "sparse_interactive" }, EARLY, "Sparse");
    const later = await recordedRead({ model: MODEL, source: DENSE, mode: "dense_scrutiny" }, LATE, "Dense");
    expect(JSON.parse(earlier.input_manifest_canonical).model_basis.model_payload.schema_version).toBe("0.7.0");
    expect(JSON.parse(later.input_manifest_canonical).model_basis.model_payload.schema_version).toBe(MODEL.schema_version);
    const shownFor = async (reads: AnalysisRunRead[]) => {
      invokeMock.mockReset();
      await nativeStore(reads);
      const { result: hook } = sessionHistory();
      await act(async () => { await hook.current.refreshStates(PROJECT); });
      let compared = false;
      await act(async () => { compared = await hook.current.compareRuns(reads[0].run.recording_id, reads[1].run.recording_id); });
      expect(compared, hook.current.historyMessage).toBe(true);
      return hook.current.runComparison!;
    };
    const shown = await shownFor([later, earlier]);
    expect([shown.left.id, shown.right.id]).toEqual([earlier.run.recording_id, later.run.recording_id]);
    expect([shown.left.standing.status, shown.right.standing.status]).toEqual(["integrity_checked", "integrity_checked"]);
    expect(shown.comparison.run_gate).toEqual([]);
    const current = await recordedRead({ model: MODEL, source: SPARSE, mode: "sparse_interactive" }, EARLY, "Sparse");
    const control = await shownFor([later, current]);
    expect(shown.comparison).toEqual(control.comparison);
  });

  it("compares a recorded run with the current run, on the right with its live verdict, counting its bytes at admission", async () => {
    const earlier = await recordedRead({ model: MODEL, source: SPARSE, mode: "sparse_interactive" }, EARLY, "Sparse");
    const current = await transientOf({ model: MODEL, source: DENSE, mode: "dense_scrutiny" });
    const runSlot: CurrentRunSlot = { result: DENSE, analysisRun: current.record, inputManifest: current.manifest, restored: false };
    await nativeStore([earlier]);
    const { result: hook } = sessionHistory({ runSlot });
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    expect(hook.current.currentRunUnavailableReason).toBeNull();
    await act(async () => { await hook.current.compareRuns(CURRENT_RUN_ID, earlier.run.recording_id); });
    expect(commands()).toEqual(["admit_analysis_run_comparison", "read_analysis_run"]);
    const admission = invokeMock.mock.calls.find(([command]) => command === "admit_analysis_run_comparison")![1];
    const bytes = currentRunBytes(runSlot);
    expect(bytes).toBe(Buffer.byteLength(jsonTextKeepingNegativeZero(DENSE)) + Buffer.byteLength(current.manifest.canonical_bytes));
    expect(admission).toEqual({ projectId: PROJECT, recordingIds: [earlier.run.recording_id], currentRunBytes: bytes });
    const shown = hook.current.runComparison!;
    expect([shown.left.kind, shown.right.kind]).toEqual(["recorded", "current"]);
    expect(shown.right.standing.eligible).toBe(true);
    expect(shown.comparison.summary.compared_nonzero).toBeGreaterThan(0);
  });

  it("refuses a comparison whose current run is retired while it is prepared: during the admission and during the reads", async () => {
    const earlier = await recordedRead({ model: MODEL, source: SPARSE, mode: "sparse_interactive" }, EARLY, "Sparse");
    const current = await transientOf({ model: MODEL, source: DENSE, mode: "dense_scrutiny" });
    const live = (): CurrentRunSlot => ({ result: DENSE, analysisRun: current.record, inputManifest: current.manifest, restored: false });
    for (const stage of ["admission", "read"] as const) {
      const { result: hook, deps } = sessionHistory({ runSlot: live() });
      const retire = () => { (deps as Json).runSlot = stage === "admission" ? EMPTY_SLOT : { ...live(), result: structuredClone(DENSE) }; };
      await nativeStore([earlier], {
        admission: () => { if (stage === "admission") retire(); return { runs: [], current_run_bytes: 0, combined_bytes: 10, budget_bytes: 12 * 1024 ** 3 }; },
        read: () => { if (stage === "read") retire(); return undefined; }
      });
      await act(async () => { await hook.current.refreshStates(PROJECT); });
      let compared = true;
      await act(async () => { compared = await hook.current.compareRuns(CURRENT_RUN_ID, earlier.run.recording_id); });
      expect(compared, stage).toBe(false);
      expect(hook.current.runComparison, stage).toBeNull();
      expect(hook.current.historyMessage, stage).toBe(`Not compared: ${COMPARISON_CURRENT_RUN_CHANGED}: the current run changed (an edit, another solve or a cleared result) while the comparison was prepared, so nothing was compared; compare again.`);
      expect(deps.releaseProjectOperation, stage).toHaveBeenCalledTimes(1);
      cleanup();
      invokeMock.mockReset();
    }
  });

  it("closes a shown comparison with the current run when the slot changes, and keeps a recorded-only one", async () => {
    const earlier = await recordedRead({ model: MODEL, source: SPARSE, mode: "sparse_interactive" }, EARLY, "Sparse");
    const later = await recordedRead({ model: MODEL, source: DENSE, mode: "dense_scrutiny" }, LATE, "Dense");
    const current = await transientOf({ model: MODEL, source: DENSE, mode: "dense_scrutiny" });
    const live: CurrentRunSlot = { result: DENSE, analysisRun: current.record, inputManifest: current.manifest, restored: false };
    for (const change of ["cleared", "another solve", "edited model"] as const) {
      await nativeStore([earlier, later]);
      const { result: hook, deps, rerender } = sessionHistory({ runSlot: live });
      await act(async () => { await hook.current.refreshStates(PROJECT); });
      await act(async () => { await hook.current.compareRuns(CURRENT_RUN_ID, earlier.run.recording_id); });
      expect(hook.current.runComparison?.right.kind, change).toBe("current");
      // A comparison of two recorded runs is not affected by the slot.
      const { result: recordedOnly, deps: recordedDeps, rerender: rerenderRecorded } = sessionHistory({ runSlot: live });
      await act(async () => { await recordedOnly.current.refreshStates(PROJECT); });
      await act(async () => { await recordedOnly.current.compareRuns(earlier.run.recording_id, later.run.recording_id); });
      expect(recordedOnly.current.runComparison, change).not.toBeNull();
      const next: CurrentRunSlot = change === "cleared" ? EMPTY_SLOT
        : change === "another solve" ? { ...live, result: structuredClone(DENSE), analysisRun: structuredClone(current.record) }
        : { ...live, inputManifest: structuredClone(current.manifest) };
      (deps as Json).runSlot = next;
      (recordedDeps as Json).runSlot = next;
      rerender();
      rerenderRecorded();
      expect(hook.current.runComparison, change).toBeNull();
      expect(hook.current.historyMessage, change).toBe("Run comparison closed: the current run it compared changed (an edit, another solve or a cleared result); compare again to see the current run.");
      expect(hook.current.historyOperation, change).toBe("compare_runs_current_changed");
      expect(recordedOnly.current.runComparison, change).not.toBeNull();
      expect(recordedOnly.current.historyOperation, change).toBe("compare_runs");
      cleanup();
      invokeMock.mockReset();
    }
  });

  it("is refused by name before either run is read when their stored inputs exceed the budget", async () => {
    const earlier = await recordedRead({ model: MODEL, source: SPARSE, mode: "sparse_interactive" }, EARLY, "Sparse");
    const later = await recordedRead({ model: MODEL, source: DENSE, mode: "dense_scrutiny" }, LATE, "Dense");
    const refusal = `COMPARISON-RUNS-TOO-LARGE: comparing recorded runs \`${earlier.run.recording_id}\` and \`${later.run.recording_id}\` needs their results and input manifests, 30 bytes together, above the per-invocation budget of 20 bytes; nothing was read and nothing was compared.`;
    await nativeStore([earlier, later], { admission: () => { throw refusal; } });
    const { result: hook } = sessionHistory();
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    await act(async () => { await hook.current.compareRuns(earlier.run.recording_id, later.run.recording_id); });
    expect(commands()).toEqual(["admit_analysis_run_comparison"]);
    expect(hook.current.runComparison).toBeNull();
    expect(hook.current.historyMessage).toBe(`Not compared: ${refusal}`);
    expect(hook.current.historyOperation).toBe("compare_runs_failed");
  });

  it("starts nothing while a prompt holds the session or another operation owns it, or for runs it cannot compare", async () => {
    const earlier = await recordedRead({ model: MODEL, source: SPARSE, mode: "sparse_interactive" }, EARLY, "Sparse");
    const later = await recordedRead({ model: MODEL, source: DENSE, mode: "dense_scrutiny" }, LATE, "Dense");
    await nativeStore([earlier, later]);
    const busy = sessionHistory({ projectSessionBusy: () => true });
    await act(async () => { await busy.result.current.refreshStates(PROJECT); });
    await act(async () => { await busy.result.current.compareRuns(earlier.run.recording_id, later.run.recording_id); });
    expect(busy.result.current.historyOperation).toBe("compare_runs_busy");
    expect(busy.deps.acquireProjectOperation).not.toHaveBeenCalled();
    const owned = sessionHistory({ acquireProjectOperation: vi.fn(() => null) });
    await act(async () => { await owned.result.current.refreshStates(PROJECT); });
    await act(async () => { await owned.result.current.compareRuns(earlier.run.recording_id, later.run.recording_id); });
    expect(owned.result.current.historyOperation).toBe("busy");
    // The same run twice, an unlisted run, and the current run without a solve.
    const { result: hook } = sessionHistory();
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    for (const [first, second] of [[earlier.run.recording_id, earlier.run.recording_id], [earlier.run.recording_id, "runrec:unlisted"], [earlier.run.recording_id, CURRENT_RUN_ID]]) {
      await act(async () => { await hook.current.compareRuns(first, second); });
      expect(hook.current.historyOperation).toBe("compare_runs_refused");
      expect(hook.current.historyMessage).toMatch(/^Not compared: select two different runs of this project/);
    }
    expect(hook.current.historyMessage).toContain("Solve the model to compare the current run.");
    expect(commands()).toEqual([]);
  });

  it("refuses a recorded run whose stored values the webview cannot represent exactly (an integer beyond 2^53)", async () => {
    // The result's checked profile admits only binary64-exact numbers, so the gap is in the
    // snapshot: the store holds the solved model (in the input manifest) with
    // 9007199254740993 and hashed that exact text, and the webview decodes it to
    // 9007199254740992. The record names the exact manifest hash, resealed coherently.
    const probe = structuredClone(MODEL) as Json;
    probe.nodes[0].position.x = 9007199254740992;
    const decoded = await recordedRead({ model: probe, source: SPARSE, mode: "sparse_interactive" }, EARLY, "Exact");
    const exactManifest = decoded.input_manifest_canonical.replace("9007199254740992", "9007199254740993");
    expect(exactManifest).not.toBe(decoded.input_manifest_canonical);
    expect(JSON.parse(exactManifest)).toEqual(JSON.parse(decoded.input_manifest_canonical));
    const decodedHash = sha(decoded.input_manifest_canonical);
    const recordText = JSON.stringify(decoded.record).split(decodedHash).join(sha(exactManifest));
    const record = JSON.parse(recordText) as AnalysisRunEnvelope;
    const index = record.analysis_run.hashes.findIndex((hash) => hash.payload_scope === "analysis_run_record");
    record.analysis_run.hashes[index].value = await canonicalSha256HexCheckedV1({ ...record, analysis_run: { ...record.analysis_run, hashes: record.analysis_run.hashes.filter((_, at) => at !== index) } });
    const exactModelHash = `sha256:${sha((await canonicalJsonString(probe)).replace("9007199254740992", "9007199254740993"))}`;
    const exact: AnalysisRunRead = {
      ...decoded, record, input_manifest_canonical: exactManifest,
      run: { ...decoded.run, run_record_hash: recordHashOf(record), recording_id: `runrec:${recordHashOf(record).slice(0, 32)}` },
      state: { ...decoded.state, model_hash: exactModelHash }
    };
    const later = await recordedRead({ model: MODEL, source: DENSE, mode: "dense_scrutiny" }, LATE, "Dense");
    await nativeStore([exact, later]);
    const { result: hook } = sessionHistory();
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    let compared = true;
    await act(async () => { compared = await hook.current.compareRuns(exact.run.recording_id, later.run.recording_id); });
    expect(compared).toBe(false);
    expect(hook.current.runComparison).toBeNull();
    expect(hook.current.historyMessage).toBe(
      `Not compared: COMPARISON-RUN-NOT-EXACT: the recorded run "Exact" (${exact.run.recording_id}) holds values the desktop cannot represent exactly (its snapshot and input manifest as received do not hash to the stored hash), so it is not compared.`
    );
    // The same run with the representable value compares.
    await nativeStore([decoded, later]);
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    await act(async () => { compared = await hook.current.compareRuns(decoded.run.recording_id, later.run.recording_id); });
    expect(compared, hook.current.historyMessage).toBe(true);
  });

  it("refuses a run recorded before rule-pack resolution by name and keeps it shown as not comparable", async () => {
    const earlier = await recordedRead({ model: MODEL, source: SPARSE, mode: "sparse_interactive" }, EARLY, "Sparse");
    const later = await recordedRead({ model: MODEL, source: DENSE, mode: "dense_scrutiny" }, LATE, "S2-era");
    const refusal = `ANALYSIS-RUN-RULE-PACKS-NOT-RESOLVED: recorded run \`${later.run.recording_id}\`: recorded before rule-pack resolution; re-record this run.`;
    await nativeStore([earlier, later], { read: (id) => { if (id === later.run.recording_id) throw refusal; return undefined; } });
    const { result: hook } = sessionHistory();
    await act(async () => { await hook.current.refreshStates(PROJECT); });
    await act(async () => { await hook.current.compareRuns(earlier.run.recording_id, later.run.recording_id); });
    expect(hook.current.historyMessage).toBe(`Not compared: ${refusal}`);
    expect(hook.current.refusedRuns).toEqual({ [later.run.recording_id]: refusal });
    expect(runCompareUnavailableReason([earlier.run.recording_id, later.run.recording_id], hook.current.runs, null, hook.current.refusedRuns)).toBe(
      `The recorded run ${later.run.recording_id} cannot be compared: ${refusal}`
    );
  });

  it("compares no row pair of recorded runs on the pressure-source route: they read needs_recompute", async () => {
    const model = json("fixtures/product_preview/pressure_source/physics_source/fields.request.json").model as PreviewModel;
    const source = (mode: string) => json(`fixtures/product_preview/pressure_source/physics_source/fields-${mode}.raw.json`) as MechanicsResult;
    const project = model.project.id;
    const read = async (mode: "sparse_interactive" | "dense_scrutiny", at: string) => {
      const solved = await recordedRead({ model, source: source(mode), mode }, at, mode);
      return { ...solved, run: { ...solved.run, project_id: project }, state: { ...solved.state, project_id: project } };
    };
    const [left, right] = [await read("sparse_interactive", EARLY), await read("dense_scrutiny", LATE)];
    await nativeStore([left, right]);
    const { result: hook } = sessionHistory({}, project);
    await act(async () => { await hook.current.refreshStates(project); });
    await act(async () => { await hook.current.compareRuns(left.run.recording_id, right.run.recording_id); });
    const shown = hook.current.runComparison;
    expect(shown, hook.current.historyMessage).not.toBeNull();
    expect([shown!.left.standing.status, shown!.right.standing.status]).toEqual(["needs_recompute", "needs_recompute"]);
    expect(shown!.left.standing.contract).toBe("pressure_source");
    expect(shown!.comparison.run_gate).toEqual(["left_run_needs_recompute", "right_run_needs_recompute"]);
    expect(shown!.comparison.summary.compared).toBe(0);
    render(<RunComparisonView result={shown!} onClose={() => {}} />);
    expect(screen.getByTestId("run-comparison-gate")).toHaveTextContent("a run on the pressure-source or retained route reads needs_recompute");
  });
});

// The panel and the view, with the session's history faked.
const RUN_A = { project_id: PROJECT, recording_id: "runrec:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", state_id: STATE_ID, producer_run_id: "run:p", created_at: EARLY, run_record_hash: "a".repeat(64), result_hash: "r", labels_updated_at: EARLY, analysis_status: ["HUMAN_REVIEW_REQUIRED"], run_name: "Run A", tags: [], notes: [], assumptions: [] };
const RUN_B = { ...RUN_A, recording_id: "runrec:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", created_at: LATE, run_record_hash: "b".repeat(64), run_name: "Run B" };
const RUN_C = { ...RUN_A, recording_id: "runrec:cccccccccccccccccccccccccccccccc", created_at: "2026-10-25T12:00:00.000Z", run_record_hash: "c".repeat(64), run_name: "Run C" };
const UNREADABLE_RUN = { id: "runrec:dddddddddddddddddddddddddddddddd", handle: "run-row:9:39:0123456789abcdef", references: 0, refusal: "ANALYSIS-RUN-INTEGRITY: recorded run `runrec:dddddddddddddddddddddddddddddddd`: the stored record hashes to x; it is listed as unreadable and is not read." };

function slice(overrides: Partial<WorkspaceSession["project"]["history"]> = {}) {
  return {
    states: [stateSummary("sha256:m")], runs: [RUN_A, RUN_B, RUN_C], unreadable: { states: [], runs: [UNREADABLE_RUN] }, runCheck: null,
    recordRunUnavailableReason: "RUN-RECORD-NO-RUN: solve first.", recordRunState: null, historyMessage: "", historyOperation: "", historyBusy: false, basisStateId: null,
    refreshStates: vi.fn(), saveState: vi.fn(), openState: vi.fn(), editLabels: vi.fn(), deleteState: vi.fn(), recordRun: vi.fn(), editRunLabels: vi.fn(),
    deleteRun: vi.fn(), checkRun: vi.fn(), deleteUnreadable: vi.fn(), stateComparison: null, compareStates: vi.fn(async () => true), closeStateComparison: vi.fn(),
    runComparison: null, compareRuns: vi.fn(async () => true), closeRunComparison: vi.fn(), currentRunUnavailableReason: "Solve the model to compare the current run.", refusedRuns: {},
    ...overrides
  } as unknown as WorkspaceSession["project"]["history"];
}

function renderPanel(history: WorkspaceSession["project"]["history"], projectBusy = false) {
  const session = { model: { model: { project: { id: PROJECT } } }, project: { history, projectBusy } } as unknown as WorkspaceSession;
  return render(<WorkspaceSessionProvider session={session}><HistoryPanel /></WorkspaceSessionProvider>);
}

const box = (id: string) => screen.getByTestId(`${id === CURRENT_RUN_ID ? "current-run" : `recorded-run-${id}`}-compare-select`);

describe("Compare… of runs in the History panel", () => {
  it("is enabled for two selected runs, says why a third cannot be added, and keeps states and runs apart", () => {
    const history = slice();
    renderPanel(history);
    const compare = screen.getByTestId("model-state-compare");
    expect(compare).toHaveAttribute("title", "Select two saved states, or two runs, to compare.");
    fireEvent.click(box(RUN_B.recording_id));
    expect(compare).toBeDisabled();
    fireEvent.click(box(RUN_A.recording_id));
    expect(compare).toBeEnabled();
    expect(box(RUN_C.recording_id)).toBeDisabled();
    expect(box(RUN_C.recording_id)).toHaveAttribute("title", "Two runs are selected; clear one to choose another.");
    fireEvent.click(compare);
    expect(history.compareRuns).toHaveBeenCalledWith(RUN_B.recording_id, RUN_A.recording_id);
    expect(history.compareStates).not.toHaveBeenCalled();
    // Selecting a state clears the runs.
    fireEvent.click(screen.getByTestId(`model-state-${STATE_ID}-compare-select`));
    expect(box(RUN_A.recording_id)).not.toBeChecked();
    expect(box(RUN_C.recording_id)).toBeEnabled();
  });

  it("offers the current run when it can be compared, and says why not otherwise", () => {
    renderPanel(slice());
    expect(box(CURRENT_RUN_ID)).toBeDisabled();
    expect(box(CURRENT_RUN_ID)).toHaveAttribute("title", "Not compared: Solve the model to compare the current run.");
    cleanup();
    const history = slice({ currentRunUnavailableReason: null });
    renderPanel(history);
    fireEvent.click(box(CURRENT_RUN_ID));
    fireEvent.click(box(RUN_A.recording_id));
    fireEvent.click(screen.getByTestId("model-state-compare"));
    expect(history.compareRuns).toHaveBeenCalledWith(CURRENT_RUN_ID, RUN_A.recording_id);
  });

  it("is disabled with the busy reason, and for a run the store refused or cannot read", () => {
    for (const [overrides, projectBusy] of [[{ historyBusy: true }, false], [{}, true]] as const) {
      const history = slice(overrides);
      renderPanel(history, projectBusy);
      fireEvent.click(box(RUN_A.recording_id));
      fireEvent.click(box(RUN_B.recording_id));
      const compare = screen.getByTestId("model-state-compare");
      expect(compare).toBeDisabled();
      expect(compare).toHaveAttribute("title", "Another project operation or prompt is in progress.");
      cleanup();
    }
    const refusal = `ANALYSIS-RUN-RULE-PACKS-NOT-RESOLVED: recorded run \`${RUN_B.recording_id}\`: recorded before rule-pack resolution; re-record this run.`;
    renderPanel(slice({ refusedRuns: { [RUN_B.recording_id]: refusal } }));
    expect(box(RUN_B.recording_id)).toBeDisabled();
    expect(box(RUN_B.recording_id)).toHaveAttribute("title", `Not compared: ${refusal}`);
    expect(screen.getByTestId(`recorded-run-${RUN_B.recording_id}`)).toHaveTextContent(`Not compared: ${refusal}`);
    const item = screen.getByTestId(`unreadable-run-${UNREADABLE_RUN.handle}`);
    const unreadable = within(item).getByTestId(`unreadable-run-${UNREADABLE_RUN.handle}-compare-select`);
    expect(unreadable).toBeDisabled();
    expect(unreadable).toHaveAttribute("title", "This stored row cannot be read, so it cannot be compared.");
    expect(within(item).getAllByRole("button").map((button) => button.textContent)).toEqual(["Delete…"]);
  });
});

function side(name: string, overrides: Partial<RunComparisonSide> = {}): RunComparisonSide {
  return {
    kind: "recorded", id: `runrec:${name}`, description: `"${name}" (runrec:${name})`, createdAt: EARLY, state: `"After solve" (${STATE_ID})`,
    standing: { contract: "preview_physics", status: "integrity_checked", eligible: true, findings: [] },
    input: { rows: [], standing: { contract: "preview_physics", status: "integrity_checked", eligible: true, findings: [] }, semantic_identity: { route: "preview_physics", contract_id: "c", contract_sha256: "s" }, context: {}, diagnostics: [] },
    ...overrides
  };
}

function identity(id: string, overrides: Partial<RunRowIdentity> = {}): RunRowIdentity {
  return { result_id: id, family: "stress", object_ref: "pipe:P-1", basis_ref: { ref_type: "load_case", ref_id: "case:L-1" }, dimension: "stress", component: "c", location: "l", basis: "b", coordinate_system: "cs", sign_convention: "sc", ...overrides };
}
const flat = (id: string, value: number, unit = "MPa", overrides: Partial<RunRowIdentity> = {}): FlatRunRow => ({ identity: identity(id, overrides), value, unit, row_index: 0 });

function viewResult(leftRows: FlatRunRow[], rightRows: FlatRunRow[], extra: { context?: [Record<string, unknown>, Record<string, unknown>] } = {}): RunComparisonResult {
  const left = side("left");
  const right = side("right");
  left.input = { ...left.input, rows: leftRows, context: extra.context?.[0] ?? {}, diagnostics: [{ code: "LOAD_CATEGORY_PREVIEW_MAPPED", severity: "warning", affected_refs: ["load:x"], message: "mapped" }] };
  right.input = { ...right.input, rows: rightRows, context: extra.context?.[1] ?? {}, diagnostics: [{ code: "NUMERICAL_INTEGRITY_CHECKS_PASSED", severity: "info", affected_refs: ["case:L-1"], message: null }] };
  return { projectId: PROJECT, left, right, comparison: compareRunInputs(left.input, right.input) };
}

describe("the run comparison view", () => {
  it("shows the sides' standing, the counts, the context, the diagnostics and a filterable table of deltas with units", () => {
    const shown = viewResult(
      [flat("r:1", 10), flat("r:2", 2, "N", { family: "force" }), flat("r:3", 1), flat("r:left", 5)],
      [flat("r:1", 12.5), flat("r:2", 2, "kN", { family: "force" }), flat("r:3", 1, "MPa", { location: "elsewhere" }), flat("r:right", 6)],
      { context: [{ solver_mode: "sparse_interactive", "load_case:case:L-1": true }, { solver_mode: "dense_scrutiny" }] }
    );
    render(<RunComparisonView result={shown} onClose={() => {}} />);
    expect(screen.getByTestId("run-comparison-side-left")).toHaveTextContent('Left: "left" (runrec:left), recorded 2026-10-25T10:00:00.000Z, against saved state "After solve"');
    expect(screen.getByTestId("run-comparison-side-left")).toHaveTextContent("reads integrity_checked (route preview_physics)");
    for (const [id, count] of [["mapped", 2], ["compared", 1], ["changed", 1], ["not-compared", 1], ["left-only", 2], ["right-only", 2], ["context", 2], ["diagnostics-added", 1], ["diagnostics-removed", 1]] as const) {
      expect(screen.getByTestId(`run-comparison-count-${id}`), id).toHaveTextContent(String(count));
    }
    expect(screen.getByTestId("run-comparison-context")).toHaveTextContent("solver_mode changed: left \"sparse_interactive\"; right \"dense_scrutiny\"");
    expect(screen.getByTestId("run-comparison-context")).toHaveTextContent("load_case:case:L-1 only in the left run");
    expect(screen.getByTestId("run-comparison-diagnostics")).toHaveTextContent("Only in the right run: NUMERICAL_INTEGRITY_CHECKS_PASSED on case:L-1");
    expect(screen.getByTestId("run-comparison-row-compared-r:1")).toHaveTextContent("10 MPa12.5 MPaΔ 2.5 MPa");
    expect(screen.getByTestId("run-comparison-row-not_compared-r:2")).toHaveTextContent("Not compared: the units differ (nothing is converted).");
    expect(screen.getByTestId("run-comparison-row-left_only-r:3")).toHaveTextContent("Unmatched: the other run's row with this result id differs in location.");
    expect(screen.getByTestId("run-comparison-row-right_only-r:right")).toHaveTextContent("Unmatched: the other run has no row with this result id.");
    const rows = () => [...screen.getByTestId("run-comparison-table").querySelectorAll("tbody tr")].map((row) => row.getAttribute("data-testid"));
    fireEvent.change(screen.getByTestId("run-comparison-filter-status"), { target: { value: "left_only" } });
    expect(rows()).toEqual(["run-comparison-row-left_only-r:3", "run-comparison-row-left_only-r:left"]);
    fireEvent.change(screen.getByTestId("run-comparison-filter-status"), { target: { value: "all" } });
    fireEvent.change(screen.getByTestId("run-comparison-filter-family"), { target: { value: "force" } });
    expect(rows()).toEqual(["run-comparison-row-not_compared-r:2"]);
    fireEvent.change(screen.getByTestId("run-comparison-filter-family"), { target: { value: "all" } });
    fireEvent.change(screen.getByTestId("run-comparison-filter-id"), { target: { value: "r:1" } });
    expect(rows()).toEqual(["run-comparison-row-compared-r:1"]);
    // View-only, and no wording of standing beyond the reader's status.
    const view = screen.getByTestId("run-comparison");
    expect(within(view).queryByRole("button", { name: /export|download|save/i })).toBeNull();
    expect(view.textContent).not.toMatch(/certif|accept|approv/i);
  });

  it("pages through every row, at most a page at once, back to the first page on a filter change", () => {
    const left = [...Array.from({ length: 201 }, (_, index) => flat(`A-${String(index).padStart(3, "0")}`, index)), flat("Z-last", 1)];
    const right = left.map((entry) => ({ ...entry, value: (entry.value as number) + 1 }));
    render(<RunComparisonView result={viewResult(left, right)} onClose={() => {}} />);
    const ids = () => [...screen.getByTestId("run-comparison-table").querySelectorAll("tbody tr td:first-child")].map((cell) => cell.textContent);
    expect(ids()).toHaveLength(COMPARISON_PAGE_ROWS);
    expect(ids()).not.toContain("Z-last");
    expect(screen.getByTestId("run-comparison-shown")).toHaveTextContent(`Rows 1–${COMPARISON_PAGE_ROWS} of 202.`);
    expect(screen.getByTestId("run-comparison-page-previous")).toBeDisabled();
    fireEvent.click(screen.getByTestId("run-comparison-page-next"));
    expect(ids()).toContain("Z-last");
    expect(screen.getByTestId("run-comparison-page-next")).toBeDisabled();
    fireEvent.change(screen.getByTestId("run-comparison-filter-status"), { target: { value: "changed" } });
    expect(screen.getByTestId("run-comparison-shown")).toHaveTextContent(`Rows 1–${COMPARISON_PAGE_ROWS} of 202.`);
  });

  it("is shown below the lists and closes", () => {
    const history = slice({ runComparison: viewResult([flat("r:1", 1)], [flat("r:1", 2)]) });
    renderPanel(history);
    fireEvent.click(within(screen.getByTestId("run-comparison")).getByTestId("run-comparison-close"));
    expect(history.closeRunComparison).toHaveBeenCalled();
    expect(screen.getByTestId("model-history-panel").textContent).not.toMatch(/certif|accept|approv/i);
  });
});
