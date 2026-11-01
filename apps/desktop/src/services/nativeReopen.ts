/** C1 slice 6 (DEL-04-05 `c1-native-custody-plan.md`): saved and recorded runs reopen into native
 * custody (`src-tauri/src/stored_custody.rs`). The stored result text and its stored v0.3 record stay
 * in the desktop process; the webview receives a restored handle, the historical findings the native
 * port of `buildHistoricalRunContext` / `buildRecordedRunContext` computed on the stored texts, the
 * record's header and the project's or run's small members.
 *
 * - **Delivery.** Both commands reply with a JSON body, which Tauri delivers parsed; the decoders
 *   accept exactly that and refuse text, bytes and every other shape by name
 *   (`NATIVE-REOPEN-MALFORMED`).
 * - **Origin.** A restored handle is never a fresh solve: its `NativeResult` carries the restored
 *   origin, no capture or registration, no binding to a model's standing, and S read with no model
 *   and no standing (as the historical panel's `ResultsPanel` read the rows). Fresh persistence and
 *   reports refuse it natively; the session never makes it the current result.
 * - **C2 consumers.** A saved run re-saved with its project, and any other whole-value consumer, load
 *   the stored result and record whole only at or under 64 MiB (`read_preview_mechanics_job_rows_all`,
 *   `read_restored_result_record_all`); above it they are refused by name
 *   (`RESULT-DETAIL-WEBVIEW-LIMIT`), the run staying held natively. */
import { invoke } from "@tauri-apps/api/core";
import { invokeWithIngress } from "./nativeIngress";
import { nativeInvokeError } from "./memoryPool";
import { disposePreviewMechanicsResult } from "./previewService";
import {
  createNativeResult,
  forgetNativeJob,
  isNativeJobLive,
  NATIVE_FULL_COPY_COMMAND,
  NATIVE_FULL_COPY_RELEASE_COMMAND,
  parseFullResultRefusal,
  readNativeSummary,
  readNativeVerdict,
  queryNativeRecovery,
  type NativeOrigin,
  type NativeResult
} from "./nativeResult";
import type { AnalysisRunEnvelope, LocalProjectEnvelope, MechanicsResult } from "../types";
import type { AnalysisRunSummary } from "./analysisRunService";
import type { ModelStateSummary } from "./modelStateService";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

export const NATIVE_OPEN_PROJECT_COMMAND = "open_local_project_in_native_custody";
export const NATIVE_READ_RUN_COMMAND = "read_analysis_run_in_native_custody";
export const NATIVE_RESTORED_RECORD_COPY_COMMAND = "read_restored_result_record_all";
export const NATIVE_REOPEN_MALFORMED = "NATIVE-REOPEN-MALFORMED";
export const STORED_RESULT_RESTORE_REFUSED = "STORED-RESULT-RESTORE-REFUSED";
/** The prefix of a restored handle (a solve's is `backend-solve-job-`). */
export const RESTORED_HANDLE_PREFIX = "restored-stored-result-";

/** The native findings of a reopened run (`HistoricalFindings`). */
export type RestoredFindings = Readonly<{
  findings: readonly string[];
  run_id: string;
  renderable: boolean;
  mechanics: string | null;
  envelope_payload_hash: string | null;
  model_hash: Json;
}>;

/** A stored run held natively (`RestoredRun`). */
export type RestoredRun = Readonly<{
  handle: string;
  origin: Exclude<NativeOrigin, "fresh_job">;
  source: Readonly<{ kind: string; project_id: string; recording_id: string | null; result_sha256: string; record_sha256: string | null }>;
  result_bytes: number;
  record_bytes: number;
  result_present: boolean;
  record_present: boolean;
  findings: RestoredFindings;
  /** `{record, result_ref_count, diagnostic_count, carried_members}`: the stored record without its
   * row-scale members, or null with `record_header_bytes` above the header limit. */
  record_header: Json;
  record_header_bytes: number | null;
}>;

/** The project's stored run as the native open found it. */
export type NativeStoredRun =
  | Readonly<{ state: "none" }>
  | (Readonly<{ state: "restored" }> & RestoredRun)
  | Readonly<{ state: "refused"; code: string; message: string; result_bytes: number; analysis_run_bytes: number }>;

/** `open_local_project`'s envelope without its run, and the run (`NativeProjectEnvelope`). */
export type NativeProjectEnvelope = Omit<LocalProjectEnvelope, "mechanics_result" | "analysis_run" | "stored_result_withheld"> & { stored_run: NativeStoredRun };

/** The recorded run's reopen (`NativeRunRead`). */
export type NativeRunRead = Readonly<{ run: AnalysisRunSummary; state: ModelStateSummary; input_manifest_canonical: string; restored: RestoredRun }>;

function malformed(reason: string): never {
  throw new Error(`${NATIVE_REOPEN_MALFORMED}: ${reason}`);
}
function plain(value: unknown): value is Record<string, Json> {
  return !!value && typeof value === "object" && !Array.isArray(value) && !(value instanceof ArrayBuffer) && !ArrayBuffer.isView(value);
}
const isText = (value: unknown): value is string => typeof value === "string";
const isCount = (value: unknown): value is number => typeof value === "number" && Number.isSafeInteger(value) && value >= 0;

function decodeRestored(value: unknown, origin: RestoredRun["origin"]): RestoredRun {
  if (!plain(value)) malformed("the restored run is not an object");
  if (!isText(value.handle) || !value.handle.startsWith(RESTORED_HANDLE_PREFIX)) malformed("the restored handle is not a restored result's");
  if (value.origin !== origin) malformed(`the restored origin is ${String(value.origin)}, not ${origin}`);
  const source = value.source;
  if (!plain(source) || !isText(source.kind) || !isText(source.project_id) || !isText(source.result_sha256)) malformed("the restored source is not readable");
  if (!isCount(value.result_bytes) || !isCount(value.record_bytes) || typeof value.result_present !== "boolean" || typeof value.record_present !== "boolean") malformed("the restored lengths are not readable");
  const found = value.findings;
  if (!plain(found) || !Array.isArray(found.findings) || !found.findings.every(isText) || !isText(found.run_id) || typeof found.renderable !== "boolean") malformed("the findings are not readable");
  if (!(value.record_header === null ? isCount(value.record_header_bytes) : plain(value.record_header) && value.record_header_bytes === null)) malformed("the record header is not readable");
  // The header never carries a row-scale member.
  const record = plain(value.record_header) ? value.record_header.record : null;
  if (plain(record) && plain(record.analysis_run) && ["result_refs", "diagnostics", "contract_evidence", "source_block_recovery", "retained_precision"].some((member) => member in record.analysis_run)) malformed("the record header carries a row-scale member");
  return Object.freeze(value as RestoredRun);
}

/** Reads the project open's delivery: a parsed JSON body (or null: no project). */
export function decodeNativeProjectOpen(delivered: unknown): NativeProjectEnvelope | null {
  if (delivered === null) return null;
  if (!plain(delivered)) malformed(`the delivery is ${typeof delivered === "string" ? "text" : delivered instanceof ArrayBuffer || ArrayBuffer.isView(delivered) ? "bytes" : "not an object"}, not the parsed envelope`);
  if ("mechanics_result" in delivered || "analysis_run" in delivered) malformed("the envelope carries its run whole");
  if (!plain(delivered.summary) || !plain(delivered.model)) malformed("the envelope has no summary or model");
  const run = delivered.stored_run;
  if (!plain(run)) malformed("the envelope has no stored run");
  if (run.state === "restored") decodeRestored(run, "restored_saved_run");
  else if (run.state === "refused") {
    if (!isText(run.code) || !isText(run.message) || !isCount(run.result_bytes) || !isCount(run.analysis_run_bytes)) malformed("the refusal is not readable");
  } else if (run.state !== "none") malformed(`the stored run's state ${String(run.state)} is not known`);
  return delivered as NativeProjectEnvelope;
}

/** Reads the recorded run's delivery: a parsed JSON body. */
export function decodeNativeRunRead(delivered: unknown): NativeRunRead {
  if (!plain(delivered)) malformed(`the delivery is ${typeof delivered === "string" ? "text" : "not an object"}, not the parsed read`);
  if ("result_json" in delivered || "record" in delivered) malformed("the read carries its run whole");
  if (!plain(delivered.run) || !plain(delivered.state) || !isText(delivered.input_manifest_canonical)) malformed("the read has no run, state or manifest");
  decodeRestored(delivered.restored, "restored_recorded_run");
  return delivered as NativeRunRead;
}

/** The native project open (`stillCurrent` retires a superseded request before it is sent). */
export async function openLocalProjectNative(projectId: string | null, options: { stillCurrent?: () => boolean } = {}): Promise<NativeProjectEnvelope | null> {
  return decodeNativeProjectOpen(await invokeWithIngress<unknown>(NATIVE_OPEN_PROJECT_COMMAND, { projectId }, options));
}

/** The native recorded-run reopen. */
export async function readAnalysisRunNative(projectId: string, recordingId: string): Promise<NativeRunRead> {
  return decodeNativeRunRead(await invokeWithIngress<unknown>(NATIVE_READ_RUN_COMMAND, { projectId, recordingId }));
}

/** S's model text for a restored run: none, as the historical panel read the rows without a model. */
export const RESTORED_SUMMARY_MODEL_TEXT = "null";

/** The `NativeResult` of a restored, renderable run: S with no model and no standing, the header
 * dispatch, and the recovery facts where the route carries them. Its standing binding is empty, so
 * `numericalResultStanding` on it reads as not bound (fail closed), never as eligible. */
export async function restoredNativeResult(restored: RestoredRun): Promise<NativeResult> {
  const jobId = restored.handle;
  const summary = await readNativeSummary(jobId, RESTORED_SUMMARY_MODEL_TEXT, null);
  const verdict = await readNativeVerdict(jobId, {});
  const recovery = verdict.route === "pressure_source" || summary.header.omitted_members.includes("source_block_recovery") ? await queryNativeRecovery(jobId, 0, 1) : null;
  return createNativeResult({
    jobId,
    origin: restored.origin,
    resultBytes: restored.result_bytes,
    summary,
    verdict,
    // Never bound to a model's standing: a restored run has no current standing.
    binding: { standingModelText: "\u0000restored", invocationText: null, summaryModelText: RESTORED_SUMMARY_MODEL_TEXT, standing: null },
    recovery
  });
}

/** Ends a restored handle's custody once (later calls send nothing). */
export function disposeRestoredRun(restored: RestoredRun | null | undefined): Promise<boolean> {
  if (!restored || !isNativeJobLive(restored.handle)) return Promise.resolve(false);
  forgetNativeJob(restored.handle);
  return disposePreviewMechanicsResult(restored.handle);
}

/** A whole stored run for a C2 consumer (at or under 64 MiB each). */
export type LoadedRestoredRun = Readonly<{ result: MechanicsResult | null; analysisRun: AnalysisRunEnvelope | null }>;

/** C2 interim: the stored result and record, copied whole and parsed (each at or under 64 MiB; above,
 * `FullResultRefused` by name). The result copy's custody lease is released once parsed. */
export async function loadRestoredRun(restored: RestoredRun): Promise<LoadedRestoredRun> {
  const jobId = restored.handle;
  if (!isNativeJobLive(jobId)) throw new Error(`NATIVE-RESULT-DISPOSED: ${jobId}'s native custody has ended.`);
  const text = (delivered: unknown): string => {
    if (delivered instanceof ArrayBuffer || ArrayBuffer.isView(delivered)) {
      const view = delivered instanceof ArrayBuffer ? new Uint8Array(delivered) : new Uint8Array(delivered.buffer, delivered.byteOffset, delivered.byteLength);
      return new TextDecoder("utf-8", { fatal: true }).decode(view);
    }
    if (Object.prototype.toString.call(delivered) === "[object ArrayBuffer]") return new TextDecoder("utf-8", { fatal: true }).decode(new Uint8Array(delivered as ArrayBuffer));
    malformed("a whole copy is not bytes");
  };
  const refused = (error: unknown) => parseFullResultRefusal(error) ?? nativeInvokeError(error);
  let resultText: string;
  try { resultText = text(await invoke(NATIVE_FULL_COPY_COMMAND, { jobId })); } catch (error) { throw refused(error); }
  try {
    let recordText: string;
    try { recordText = text(await invoke(NATIVE_RESTORED_RECORD_COPY_COMMAND, { jobId })); } catch (error) { throw refused(error); }
    return Object.freeze({ result: JSON.parse(resultText) as MechanicsResult | null, analysisRun: JSON.parse(recordText) as AnalysisRunEnvelope | null });
  } finally {
    try { await invoke(NATIVE_FULL_COPY_RELEASE_COMMAND, { jobId }); } catch { /* best effort: disposal releases it */ }
  }
}

/** A restored record's rule-pack reference diagnostics (its header's), as the run's references read them. */
export function restoredRecordDiagnostics(restored: RestoredRun): { analysis_run: { diagnostics: readonly unknown[] } } {
  const listed = restored.record_header?.rule_pack_ref_diagnostics;
  return { analysis_run: { diagnostics: Array.isArray(listed) ? listed : [] } };
}

/** The person-facing text of a refused restoration (the model opened for editing). */
export function describeRestoreRefused(run: Extract<NativeStoredRun, { state: "refused" }>): string {
  return run.message;
}
