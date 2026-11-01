// Analysis runs recorded against model states (W-12 S2; DEL-14-01 Design
// `desktop-states-runs-comparison.md` §3; FR-CMP-002 and FR-MOD-006's run level).
// The native store (`apps/desktop/src-tauri/src/analysis_runs.rs`) is the only
// store of recorded runs: it checks the binding of the solve's transient record,
// result and input manifest to a saved state, allocates the recording time and
// computes the recorded revision and its hash. The browser preview keeps no
// recorded runs (SQLite-only history, design §7), so every call there is refused
// by name.
import { invokeWithIngress } from "./nativeIngress";
import { partitionListed, type UnreadableRow } from "./historyListEntries";
import { jsonTextKeepingNegativeZero } from "./jsonText";
import { isNativeResult, type SessionAnalysisRun, type SessionResult } from "./nativeResult";
import type { CurrentSessionInputManifestEvidence } from "./inputManifestService";
import { MODEL_STATE_STORE_UNAVAILABLE, type ModelStateSummary, type StateAssumption, type StateLabels } from "./modelStateService";
import type { AnalysisRunEnvelope, MechanicsResult } from "../types";

/** A recorded run's editable labels: kept outside the record hash. */
export type RunLabels = {
  run_name: string;
  tags: string[];
  notes: string[];
  /** Run-level assumptions and open issues (FR-MOD-006). analysis_run.v0.3 has no
   * assumption slot: they are shown with the run, not written into its record. */
  assumptions: StateAssumption[];
};

/** One recorded run as the store lists it (columns, labels and its record's status). */
export type AnalysisRunSummary = RunLabels & {
  project_id: string;
  /** `runrec:` and the first 32 hex characters of the record hash. */
  recording_id: string;
  state_id: string;
  /** The producer's run id, kept as data (every solve carries the same one today). */
  producer_run_id: string;
  created_at: string;
  run_record_hash: string;
  result_hash: string;
  labels_updated_at: string;
  analysis_status: string[];
};

export type RecordRunOutcome = {
  run: AnalysisRunSummary;
  state: ModelStateSummary;
  /** `existing`: tied to the project's state of the solved model; `created`: a new post-solve snapshot. */
  state_outcome: "existing" | "created";
  message: string;
};

export type AnalysisRunRead = {
  run: AnalysisRunSummary;
  state: ModelStateSummary;
  /** The recorded analysis_run.v0.3 revision. */
  record: AnalysisRunEnvelope;
  /** The received result's JSON text, as stored. */
  result_json: string;
  /** The input manifest's canonical JCS text. */
  input_manifest_canonical: string;
};

export type AnalysisRunDeleteReceipt = { project_id: string; recording_id: string; deleted: boolean; message: string };

export const LABELS_DO_NOT_CHANGE_RUN = "Renaming, tagging or adding notes does not change the recorded run.";

/** A current-run slot restored from the project row has no input manifest, so its binding to a state cannot be checked. */
export const RUN_RECORD_MANIFEST_UNAVAILABLE = "RUN-RECORD-MANIFEST-UNAVAILABLE: re-solve to record this run (a run restored from the saved project has no input manifest).";
export const RUN_RECORD_NO_RUN = "RUN-RECORD-NO-RUN: solve the model to record a run.";

/** The session's current-run slot, as Record Run reads it. */
export type CurrentRunSlot = {
  result: SessionResult | null;
  analysisRun: SessionAnalysisRun | null;
  inputManifest: CurrentSessionInputManifestEvidence | null;
  /** The slot holds a run restored from the project row (a historical saved run). */
  restored: boolean;
  /** C1 interim: a native result's C2 full copy and record, for Record Run. */
  loadFull?: RecordRunInput["loadFull"];
};

export type RecordableRun = { result: SessionResult; analysisRun: SessionAnalysisRun; inputManifest: CurrentSessionInputManifestEvidence };

/** Whether the slot holds a solve this session can record: its result, its transient
 * record and the input manifest the record names, all of one project. */
export function recordableRun(slot: CurrentRunSlot): { run: RecordableRun; reason: null } | { run: null; reason: string } {
  const { result, analysisRun, inputManifest } = slot;
  if (result && analysisRun && inputManifest) {
    const record = analysisRun.analysis_run;
    const bound = inputManifest.manifest.model_basis.model_ref === result.model_ref
      && record.run_id === result.run_id
      && record.created_at == null
      && record.reproducibility.input_manifest_hashes[0]?.value === inputManifest.manifest_sha256;
    if (bound) return { run: { result, analysisRun, inputManifest }, reason: null };
  }
  if (slot.restored || result || analysisRun) return { run: null, reason: RUN_RECORD_MANIFEST_UNAVAILABLE };
  return { run: null, reason: RUN_RECORD_NO_RUN };
}

function hasTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

function requireNativeStore(): void {
  if (!hasTauriRuntime()) throw new Error(MODEL_STATE_STORE_UNAVAILABLE);
}

export type RecordRunInput = {
  /** The stored envelope hash value the session last opened or saved (its base). */
  expectedBase: string;
  run: RecordableRun;
  /** C1 interim: a native result is recorded through its C2 full copy and full record (at or
   * under 64 MiB, the bytes today's path stores); the native Record Run lands with slice 5. */
  loadFull?: (result: SessionResult) => Promise<{ result: MechanicsResult; analysisRun: AnalysisRunEnvelope | null }>;
  /** The saved state the session expects (the one with the solved model's hash), when it listed one. */
  stateId: string | null;
  /** The post-solve snapshot to write when no state holds the solved model. */
  newState: { labels: StateLabels; parentStateId: string | null } | null;
  labels: RunLabels;
};

/** Records the slot's run against its state. The session's slot is not changed. */
export async function recordAnalysisRun(input: RecordRunInput): Promise<RecordRunOutcome> {
  requireNativeStore();
  let result: MechanicsResult, record: AnalysisRunEnvelope | null;
  if (isNativeResult(input.run.result)) {
    if (!input.loadFull) throw new Error("RUN-RECORD-NATIVE-RESULT-UNAVAILABLE: no full copy route for the native result; nothing was recorded.");
    ({ result, analysisRun: record } = await input.loadFull(input.run.result));
    if (!record) throw new Error("RUN-RECORD-NATIVE-RECORD-UNAVAILABLE: the native result's full record could not be built; nothing was recorded.");
  } else {
    result = input.run.result;
    record = input.run.analysisRun as AnalysisRunEnvelope;
  }
  return invokeWithIngress<RecordRunOutcome>("record_analysis_run", {
    request: {
      project_id: result.model_ref,
      expected_base: input.expectedBase,
      run_record: record,
      // The received result's text as the solve handed it over (negative zeros kept).
      result_json: jsonTextKeepingNegativeZero(result),
      input_manifest_canonical: input.run.inputManifest.canonical_bytes,
      state_id: input.stateId,
      new_state: input.newState ? { labels: input.newState.labels, parent_state_id: input.newState.parentStateId } : null,
      run_labels: input.labels
    }
  });
}

/** The project's recorded runs: the readable summaries and, separately, each row the
 * store cannot read, by its id and refusal only (`historyListEntries.ts`). */
export async function listAnalysisRuns(projectId: string): Promise<{ runs: AnalysisRunSummary[]; unreadable: UnreadableRow[] }> {
  requireNativeStore();
  const { readable, unreadable } = partitionListed(await invokeWithIngress<Array<AnalysisRunSummary | { unreadable: UnreadableRow }>>("list_analysis_runs", { projectId }));
  return { runs: readable, unreadable };
}

export async function readAnalysisRun(projectId: string, recordingId: string): Promise<AnalysisRunRead> {
  requireNativeStore();
  return invokeWithIngress<AnalysisRunRead>("read_analysis_run", { projectId, recordingId });
}

export async function editAnalysisRunLabels(projectId: string, recordingId: string, labels: RunLabels, expectedUpdatedAt: string | null): Promise<AnalysisRunSummary> {
  requireNativeStore();
  return invokeWithIngress<AnalysisRunSummary>("edit_analysis_run_labels", { projectId, recordingId, labels, expectedUpdatedAt });
}

export async function deleteAnalysisRun(projectId: string, recordingId: string): Promise<AnalysisRunDeleteReceipt> {
  requireNativeStore();
  return invokeWithIngress<AnalysisRunDeleteReceipt>("delete_analysis_run", { projectId, recordingId });
}

/** Deletes the run row an unreadable list entry names, by its handle (never its shown id). */
export async function deleteUnreadableAnalysisRun(projectId: string, unreadableHandle: string): Promise<AnalysisRunDeleteReceipt> {
  requireNativeStore();
  return invokeWithIngress<AnalysisRunDeleteReceipt>("delete_analysis_run", { projectId, unreadableHandle });
}

/** The stored lengths a run comparison reads (W-12 S3b): each recorded run's result and
 * input manifest, in the order named, and the session's current run's as counted here. */
export type AnalysisRunComparisonAdmission = {
  runs: Array<{ result_bytes: number; manifest_bytes: number }>;
  current_run_bytes: number;
  combined_bytes: number;
  budget_bytes: number;
};

/** The size admission of a run comparison (W-12 S3b; DEL-14-01 Design §4.2, Memory): the
 * store verifies each named run as its list does and refuses by name
 * (`COMPARISON-RUNS-TOO-LARGE`), before either run is read, when the runs' stored results
 * and input manifests, with `currentRunBytes` (the session's current run when it is the
 * other side), exceed the per-invocation budget. A 1x cap on input size, not a bound on
 * the webview's memory. */
export async function admitAnalysisRunComparison(projectId: string, recordingIds: string[], currentRunBytes: number | null): Promise<AnalysisRunComparisonAdmission> {
  requireNativeStore();
  return invokeWithIngress<AnalysisRunComparisonAdmission>("admit_analysis_run_comparison", currentRunBytes === null ? { projectId, recordingIds } : { projectId, recordingIds, currentRunBytes });
}

export function runDescription(run: Pick<AnalysisRunSummary, "run_name" | "recording_id">): string {
  return `"${run.run_name}" (${run.recording_id})`;
}
