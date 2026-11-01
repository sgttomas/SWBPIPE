// The two sides of a run comparison (W-12 S3b; DEL-14-01 Design §4.2): a recorded
// run, read through the store's metered, integrity-checked read and checked again
// in the webview, or the session's current run. Each side's numerical standing is
// `numericalResultStanding` for its result and its snapshot (the solved model its
// input manifest holds):
// - a recorded run is evaluated from its stored result and stored snapshot, after
//   the stored-hash checks; nothing registers it, so a run on the pressure-source
//   or retained route reads `needs_recompute` (live invocation custody is never
//   reconstructed from saved hashes);
// - the current run uses the session's live verdict (the same call
//   `resultsSessionState.ts` makes).
import { isNativeAnalysisRun, isNativeResult } from "../../services/nativeResult";
import type { MechanicsResult } from "../../types";
import { csvAnalysisRoute, csvRecordedModelHash, csvResultHash } from "../result-export/csvProvenance";
import type { RunSideCsvProvenance } from "./runComparisonCsv";
import { recordableRun, runDescription, type AnalysisRunRead, type AnalysisRunSummary, type CurrentRunSlot } from "../../services/analysisRunService";
import { buildRecordedRunContext, recordedRunBindingFindings } from "../results/HistoricalRunContext";
import { numericalResultStanding, type NumericalStanding } from "../results/resultStanding";
import { jsonTextKeepingNegativeZero } from "../../services/jsonText";
import type { CurrentSessionInputManifest } from "../../services/inputManifestService";
import {
  COMPARISON_RUN_NOT_EXACT,
  COMPARISON_RUN_RECORD_MISMATCH,
  flattenRunResult,
  resultSemanticIdentity,
  RunComparisonRefusal,
  runContext,
  runDiagnostics,
  type RunComparisonInput
} from "./runComparison";

/** The selection id of the session's current run in the History panel. */
export const CURRENT_RUN_ID = "current-run";

/** One side as the comparison view names it. */
export type RunComparisonSide = {
  kind: "recorded" | "current";
  /** The recorded run's id, or `CURRENT_RUN_ID`. */
  id: string;
  /** `"<name>" (<recording id>)`, or the current run's description. */
  description: string;
  /** The recording time, or null for the current run. */
  createdAt: string | null;
  /** The saved state a recorded run is tied to (`"<name>" (<state id>)`), or null. */
  state: string | null;
  standing: NumericalStanding;
  input: RunComparisonInput;
  /** RT-R6: what the side states for a CSV preamble (absent sides state `not_recorded`). */
  csv?: RunSideCsvProvenance;
};

export const CURRENT_RUN_DESCRIPTION = "the current run (this session's solve, not recorded)";

/** The binding findings that mean the webview's decoded values are not the store's:
 * a hash the store verified over its exact bytes that the webview's decoded copy does
 * not reproduce. */
const NOT_EXACT_FINDINGS: Readonly<Record<string, string>> = {
  HISTORICAL_RESULT_HASH_MISMATCH: "result",
  RECORDED_RUN_MODEL_STATE_HASH_MISMATCH: "snapshot",
  RECORDED_RUN_INPUT_MANIFEST_HASH_MISMATCH: "input manifest",
  HISTORICAL_ANALYSIS_HASH_MISMATCH: "record"
};

/** Why a received recorded run whose decoded copy does not hash as the store's is not
 * used (as `notExactReason` says of a saved state). */
export function notExactRunReason(run: Pick<AnalysisRunSummary, "run_name" | "recording_id">, parts: string[]): string {
  const named = parts.length ? parts.join(" and ") : "result";
  return `the recorded run ${runDescription(run)} holds values the desktop cannot represent exactly (its ${named} as received ${parts.length > 1 ? "do" : "does"} not hash to the stored hash)`;
}

function standingOf(result: MechanicsResult, model: unknown): NumericalStanding {
  try {
    return numericalResultStanding(result, model as Parameters<typeof numericalResultStanding>[1]);
  } catch (error) {
    return { contract: "unsupported", status: "needs_recompute", eligible: false, findings: [error instanceof Error ? error.message.split(":")[0] : "RESULT_READER_UNAVAILABLE"] };
  }
}

function sideInput(result: MechanicsResult, manifest: CurrentSessionInputManifest, record: Parameters<typeof runDiagnostics>[0], standing: NumericalStanding): RunComparisonInput {
  return {
    rows: flattenRunResult(result),
    standing,
    semantic_identity: resultSemanticIdentity(result),
    context: runContext(manifest),
    diagnostics: runDiagnostics(record)
  };
}

/** A recorded run as a comparison side. The store has verified its bytes; here its
 * result, snapshot, manifest and record, as the webview decoded them, are hashed again
 * in the store's canonical forms (`buildRecordedRunContext`): a hash that differs is
 * refused as `COMPARISON-RUN-NOT-EXACT`, any other disagreement with its stored state
 * and record as `COMPARISON-RUN-RECORD-MISMATCH`. */
export async function recordedRunSide(read: AnalysisRunRead): Promise<RunComparisonSide> {
  const context = await buildRecordedRunContext(read);
  const findings = recordedRunBindingFindings(context);
  const inexact = findings.filter((finding) => Object.hasOwn(NOT_EXACT_FINDINGS, finding));
  if (inexact.length) {
    throw new RunComparisonRefusal(COMPARISON_RUN_NOT_EXACT, `${notExactRunReason(read.run, inexact.map((finding) => NOT_EXACT_FINDINGS[finding]))}, so it is not compared.`);
  }
  let manifest: CurrentSessionInputManifest | null = null;
  try { manifest = JSON.parse(read.input_manifest_canonical) as CurrentSessionInputManifest; } catch { manifest = null; }
  const result = context.mechanicsResult;
  if (findings.length || !result || !manifest?.model_basis?.model_payload) {
    const listed = findings.length ? findings.join(", ") : "its result or input manifest cannot be read";
    throw new RunComparisonRefusal(COMPARISON_RUN_RECORD_MISMATCH, `the recorded run ${runDescription(read.run)} does not agree with its stored state and record as the desktop reads them (${listed}), so it is not compared.`);
  }
  const standing = standingOf(result, manifest.model_basis.model_payload);
  return {
    kind: "recorded",
    id: read.run.recording_id,
    description: runDescription(read.run),
    createdAt: read.run.created_at,
    state: `"${read.state.state_name}" (${read.state.state_id})`,
    standing,
    input: sideInput(result, manifest, read.record, standing),
    csv: { modelHash: csvRecordedModelHash(read.state.model_hash), model: null, resultHash: csvResultHash(result, read.record), analysisRoute: csvAnalysisRoute(result) }
  };
}

/** Why the session's current run cannot be compared, or null when it can. */
export function currentRunUnavailableReason(slot: CurrentRunSlot, projectId: string | null): string | null {
  const recordable = recordableRun(slot);
  if (!recordable.run) {
    return slot.result || slot.analysisRun || slot.restored
      ? "The current run has no input manifest in this session (a run restored from the saved project); solve the model again to compare it."
      : "Solve the model to compare the current run.";
  }
  if (recordable.run.result.model_ref !== projectId) return "The current run is a solve of another project.";
  return null;
}

/** The current run's stored-form size for the comparison's size admission: its result's
 * JSON text and its input manifest's canonical text, in UTF-8 bytes. */
export function currentRunBytes(slot: CurrentRunSlot): number {
  const recordable = recordableRun(slot);
  if (!recordable.run || isNativeResult(recordable.run.result)) return 0;
  const encoder = new TextEncoder();
  return encoder.encode(jsonTextKeepingNegativeZero(recordable.run.result)).length + encoder.encode(recordable.run.inputManifest.canonical_bytes).length;
}

/** The session's current run as a comparison side, with its live verdict: the standing
 * the session reads for it (`numericalResultStanding(result, model_payload)`). */
export function currentRunSide(slot: CurrentRunSlot): RunComparisonSide {
  const recordable = recordableRun(slot);
  if (!recordable.run) throw new RunComparisonRefusal(COMPARISON_RUN_RECORD_MISMATCH, `${recordable.reason}`);
  const { inputManifest } = recordable.run;
  // A native current run arrives here only as its C2 full copy (`useModelStateHistory`).
  if (isNativeResult(recordable.run.result) || isNativeAnalysisRun(recordable.run.analysisRun)) throw new RunComparisonRefusal(COMPARISON_RUN_RECORD_MISMATCH, "the current run is held natively; its full copy is required for a comparison");
  const result = recordable.run.result, analysisRun = recordable.run.analysisRun;
  const standing = standingOf(result, inputManifest.manifest.model_basis.model_payload);
  return {
    kind: "current",
    id: CURRENT_RUN_ID,
    description: CURRENT_RUN_DESCRIPTION,
    createdAt: null,
    state: null,
    standing,
    input: sideInput(result, inputManifest.manifest, analysisRun, standing),
    csv: { modelHash: null, model: inputManifest.manifest.model_basis.model_payload, resultHash: csvResultHash(result, analysisRun), analysisRoute: csvAnalysisRoute(result) }
  };
}
