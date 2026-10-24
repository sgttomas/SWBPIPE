/// <reference types="vite/client" />
import { isRetainedIdentity, isRetainedRoute, PRESSURE_SOURCE_CONTRACT_ID } from '../features/results/numericalResultQuality';
import { hasCurrentSourceContract, readerValidate, rememberReceivedText, sourceContract } from '../features/results/resultReader';
import { registerSourceRecovery, sameModelData } from '../features/results/resultStanding';
import { registerRetainedPrecision } from '../features/results/retainedPrecisionStanding';
import { invoke } from "@tauri-apps/api/core";
import { invokeWithIngress } from "./nativeIngress";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  CanonicalResultDimension,
  DesignKnowledge,
  MechanicsResult,
  ObjectRef,
  PreviewModel,
  PreviewComparison,
  ResourceRefusal,
  SelectedReviewTarget,
  SolveResourceGrant,
} from "../types";
import {
  verifyCurrentSessionInputManifest,
  type CurrentSessionInputManifestEvidence,
} from "./inputManifestService";
import { buildAnalysisRunV03, modelLoadBasisRefs } from "./analysisRunCompatibility";
import { canonicalSha256HexCheckedV1, checkedJsonText } from "./hashService";
import { nativeInvokeError, parseResourceRefusal, parseSolveResourceGrant, ResourceUnavailableError } from "./memoryPool";
export { configureMemoryPool, describeResourceRefusal, getMemoryPoolStatus, ResourceUnavailableError } from "./memoryPool";

async function invokeOrFixture<T>(
  command: string,
  fixture: () => Promise<T>,
  args?: Record<string, unknown>,
): Promise<T> {
  if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) {
    return fixture();
  }
  try {
    return await invoke<T>(command, args);
  } catch {
    return fixture();
  }
}

export async function loadPreviewModel(): Promise<PreviewModel> {
  return invokeOrFixture("load_preview_model", loadModelFixture);
}

export async function loadDesignKnowledge(): Promise<DesignKnowledge> {
  return invokeOrFixture("load_design_knowledge", loadKnowledgeFixture);
}

/** One solve through the native job path: start, wait for the terminal state,
 * take the producer's exact bytes and register them as `takePreviewMechanicsJobResult`
 * does. The returned result is in the app's custody: the caller releases it with
 * `disposeDeliveredMechanicsResult` once it drops the result. A refused job
 * rejects with the typed `ResourceUnavailableError`. */
export async function runPreviewMechanics(
  model?: PreviewModel | null,
  solverMode: PreviewSolverMode = "sparse_interactive",
  options: { onStatus?: (status: BackendSolveJobStatus) => void } = {},
): Promise<MechanicsResult> {
  const receipt = await startPreviewMechanicsJob(model, solverMode);
  if (receipt.mode !== "backend_job") throw new Error("SOLVE_BACKEND_INVOCATION_REQUIRED: a reference-only receipt is not a solve.");
  // From here the started job's result is either taken (and then in the
  // caller's custody) or released natively: a step that fails abandons it.
  let terminal: BackendSolveJobStatus;
  try {
    terminal = await awaitPreviewMechanicsJob(receipt.job_id, options.onStatus);
  } catch (error) {
    void abandonPreviewMechanicsJob(receipt.job_id, receipt.backend_cancellation_token);
    throw error;
  }
  if (terminal.state !== "completed") throw terminalJobError(terminal);
  try {
    return await takePreviewMechanicsJobResult(receipt.job_id);
  } catch (error) {
    // A refused or failed take is not retried: release the job's result unread.
    void discardPreviewMechanicsJobResult(receipt.job_id);
    throw error;
  }
}

/** The error a non-completed terminal job state stands for. */
export function terminalJobError(status: BackendSolveJobStatus): Error {
  if (status.state === "refused") {
    return status.resource_refusal
      ? new ResourceUnavailableError(status.resource_refusal, null)
      : new Error(`RESOURCE_UNAVAILABLE: backend solve job ${status.job_id} was refused without a readable reason.`);
  }
  if (status.state === "cancelled") return new Error(`SOLVE_JOB_CANCELLED: backend solve job ${status.job_id} stopped (${status.cancellation_status}); no result was published.`);
  return new Error(status.error_message ?? `backend solve job ${status.job_id} ended as ${status.state} without a result`);
}

export type PreviewSolverMode = "sparse_interactive" | "dense_scrutiny";

export type NativeMechanicsInvocation = {
  request: { model: PreviewModel; materials: [] };
  solver_mode: PreviewSolverMode;
};
type CapturedNativeInvocation = {
  invocation: NativeMechanicsInvocation;
  fingerprint: string;
  // Caller representation and the exact JSON payload sent to native are two
  // separate facts. Preserve the original zero signs and detect later edits.
  callerModel: PreviewModel;
  callerFingerprint: string;
  invalidated: boolean;
  terminalClaimed: boolean;
};
type NativeSourceRegistration = { capture: CapturedNativeInvocation; sourceFingerprint: string };
const nativeSourceInvocations = new WeakMap<MechanicsResult, NativeSourceRegistration>();
const receivedBytes = new WeakMap<MechanicsResult, { bytes: string; fingerprint: string }>();

/** b7-plan R1, "Exact bytes to the desktop": the native solve job hands over the
 * producer's exact serialized bytes, never a re-encoded value. The result is
 * read from them once, and they are kept beside it. */
function receiveResultBytes(bytes: unknown): MechanicsResult {
  if (typeof bytes !== "string") throw new Error("PREVIEW-SOLVE-RESULT-BYTES-REQUIRED: the native solve hands over the producer's serialized bytes");
  const result = JSON.parse(bytes) as MechanicsResult;
  try { receivedBytes.set(result, { bytes, fingerprint: nativeContentFingerprint(result) }); } catch { /* not checked JSON: no bytes are kept for it */ }
  // R2: the base-route readers read these bytes too, while the result is unchanged.
  rememberReceivedText(result, bytes);
  return result;
}
/** The producer's exact bytes this result was read from, while the result is unchanged
 * since; null for any other result (a copy, a saved or edited one). Query only. */
export function receivedResultBytes(result: MechanicsResult | null | undefined): string | null {
  const received = result ? receivedBytes.get(result) : undefined;
  if (!received) return null;
  try { return nativeContentFingerprint(result) === received.fingerprint ? received.bytes : null; } catch { return null; }
}
const jobInvocations = new Map<string, CapturedNativeInvocation>();

// Private equality witness only, not another public hash/canonicalization profile.
// The checked guard rejects lossy/non-JSON inputs, and the extra paths retain
// negative-zero identity that JSON text alone deliberately normalizes.
function nativeContentFingerprint(value: unknown): string {
  const checked = checkedJsonText(value);
  const negativeZeros: string[] = [];
  const visit = (item: unknown, path: string) => {
    if (typeof item === "number" && Object.is(item, -0)) negativeZeros.push(path);
    else if (item && typeof item === "object") {
      for (const [key, child] of Object.entries(item)) visit(child, `${path}/${key.replaceAll("~", "~0").replaceAll("/", "~1")}`);
    }
  };
  visit(value, "");
  return `${canonicalJson(JSON.parse(checked))}\nnegative_zero_paths=${JSON.stringify(negativeZeros.sort())}`;
}
function captureNativeInvocation(model: PreviewModel | null | undefined, solverMode: PreviewSolverMode): CapturedNativeInvocation | null {
  if (!model || typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) return null;
  try {
    // Mirror the native API's actual {model, materials:[]} request and send this
    // captured JSON model, not a later caller mutation or a reconstructed result.
    const invocation = JSON.parse(checkedJsonText({ request: { model, materials: [] }, solver_mode: solverMode })) as NativeMechanicsInvocation;
    return { invocation, fingerprint: nativeContentFingerprint(invocation), callerModel: model, callerFingerprint: nativeContentFingerprint(model), invalidated: false, terminalClaimed: false };
  } catch { return null; } // Raw native diagnostics remain available; no registration.
}
async function validateCapturedSource(source: MechanicsResult, capture: CapturedNativeInvocation | null): Promise<void> {
  if (!capture || capture.invalidated) return;
  try {
    if (source.model_ref !== capture.invocation.request.model.project.id || nativeContentFingerprint(capture.invocation) !== capture.fingerprint || nativeContentFingerprint(capture.callerModel) !== capture.callerFingerprint) return;
    const sourceFingerprint = nativeContentFingerprint(source);
    // Registration is reachable only after actual direct IPC or a completed
    // known job. Method-specific source-block validation composes here when its
    // separately reviewed join is selected; none is inferred from these bytes.
    const sourceRoute = sourceContract(source);
    if (sourceRoute === "pressure_source" || source.producer?.semantic_contract_id === PRESSURE_SOURCE_CONTRACT_ID) {
      // G3 (b7-plan R2): the caller's model is the one dispatched to native; a
      // normalization between them registers nothing.
      if (!sameModelData(JSON.parse(checkedJsonText(capture.callerModel)), capture.invocation.request.model)) throw new Error("CALLER_DISPATCH_NORMALIZATION_MISMATCH");
      // A claimed pressure-source identity whose Rust header refuses also reaches
      // this registration boundary, so it cannot acquire native registration unread.
      await registerSourceRecovery(source, capture.invocation, capture.callerModel,
        model => hasNativeMechanicsInvocation(source, model as PreviewModel, capture.invocation.solver_mode));
    }
    // A preview-physics-2 source registers only when its closed reader checks pass.
    if (sourceContract(source) === "preview_physics") readerValidate("preview-physics", source, capture.invocation.request.model);
    // T1, on the current exact route (B7b): a pressure-2 source, with or without the
    // load/reference-state group, registers only when its reader checks pass with the
    // captured model. A joined pressure-source-1 source registered above with its
    // invocation; its standing stays needs_recompute (never numerically eligible in T1).
    if (sourceContract(source) === "pressure") readerValidate("pressure", source, capture.invocation.request.model);
    // U6d: a preview successor registers only after the accepted reader passes with
    // this captured invocation; its outcome is recorded against the exact bytes.
    // U7 slice T: its standing is bound to this live native capture and the current model.
    // The reader reads the producer's bytes as received (b7-plan R1).
    // A successor identity the header dispatch refuses (its transport checks failed) is still read by
    // the accepted reader, so its refusal is recorded and no native registration follows.
    if (isRetainedRoute(sourceContract(source)) || isRetainedIdentity(source.producer?.semantic_contract_id)) await registerRetainedPrecision(source, capture.invocation, model => hasNativeMechanicsInvocation(source, model as PreviewModel), receivedResultBytes(source));
    await canonicalSha256HexCheckedV1(source);
    if (capture.invalidated || nativeContentFingerprint(source) !== sourceFingerprint || nativeContentFingerprint(capture.invocation) !== capture.fingerprint || nativeContentFingerprint(capture.callerModel) !== capture.callerFingerprint) return;
    nativeSourceInvocations.set(source, { capture, sourceFingerprint });
  } catch { /* Preserve received source for inspection, without qualified standing. */ }
}
/** Query only: headers, hashes, clones and saved records cannot mint registration. */
export function hasNativeMechanicsInvocation(
  source: MechanicsResult | null | undefined,
  model: PreviewModel | null | undefined,
  solverMode?: string,
): boolean {
  if (!source || !model) return false;
  const registered = nativeSourceInvocations.get(source);
  if (!registered) return false;
  try {
    const invocation = registered.capture.invocation;
    return !registered.capture.invalidated
      && (solverMode === undefined || solverMode === invocation.solver_mode)
      && nativeContentFingerprint(invocation) === registered.capture.fingerprint
      && nativeContentFingerprint(source) === registered.sourceFingerprint
      && nativeContentFingerprint(registered.capture.callerModel) === registered.capture.callerFingerprint
      && (nativeContentFingerprint(model) === registered.capture.callerFingerprint
        || nativeContentFingerprint(model) === nativeContentFingerprint(invocation.request.model));
  } catch { return false; }
}
export function retainedNativeMechanicsInvocation(source: MechanicsResult, model: PreviewModel): NativeMechanicsInvocation | null {
  return hasNativeMechanicsInvocation(source, model)
    ? structuredClone(nativeSourceInvocations.get(source)!.capture.invocation) : null;
}

function assertPreviewSolverMode(value: unknown): asserts value is PreviewSolverMode {
  if (value !== "sparse_interactive" && value !== "dense_scrutiny") {
    throw new Error(`PREVIEW_SOLVER_MODE_UNSUPPORTED: ${String(value)}`);
  }
}

export type SolveJobStartReceipt =
  | {
      mode: "backend_job";
      job_id: string;
      backend_cancellation_token: string;
      state: string;
      cancellation_scope: string;
      /** The grant the native ledger made for this job; null when not reported. */
      resource: SolveResourceGrant | null;
    }
  | { mode: "browser_fixture_no_backend_job" };

export type BackendSolveJobState = "queued" | "running" | "completed" | "cancelled" | "failed" | "refused";

/** A job's status. The poll never carries the result: a completed job's bytes
 * are taken separately (`takePreviewMechanicsJobResult`). */
export type BackendSolveJobStatus = {
  job_id: string;
  state: BackendSolveJobState;
  /** Why a queued job is waiting, as the native ledger reports it. */
  queue_reason: ResourceRefusal | null;
  cancellation_requested: boolean;
  cancellation_status: string;
  cancellation_scope: string;
  result_available: boolean;
  result_bytes: number | null;
  error_message: string | null;
  /** The typed refusal of a `refused` job. */
  resource_refusal: ResourceRefusal | null;
  resource: SolveResourceGrant | null;
};

export type BackendSolveJobCancellationReceipt = {
  job_id: string;
  accepted: boolean;
  cancellation_status: string;
  job_state: string;
  cancellation_scope: string;
  cancellation_success_claimed: boolean;
};

/** The solver-mode header of the raw-bytes start command. */
export const SOLVER_MODE_HEADER = "x-swbpipe-solver-mode";
const BACKEND_JOB_STATES: readonly BackendSolveJobState[] = ["queued", "running", "completed", "cancelled", "failed", "refused"];
const TERMINAL_JOB_STATES: readonly BackendSolveJobState[] = ["completed", "cancelled", "failed", "refused"];
export const SOLVE_JOB_POLL_INTERVAL_MS = 50;

export async function startPreviewMechanicsJob(
  model?: PreviewModel | null,
  solverMode: PreviewSolverMode = "sparse_interactive",
): Promise<SolveJobStartReceipt> {
  assertPreviewSolverMode(solverMode);
  if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) {
    throw await browserSolveUnavailable(model);
  }
  if (!model) throw new Error("PREVIEW-SOLVE-MODEL-REQUIRED: a solve carries the session's model document.");
  // The session's startup release of earlier webview custody completes before any start.
  if (sessionCustodyReset) await sessionCustodyReset;
  const capture = captureNativeInvocation(model, solverMode);
  // The request body is the UTF-8 JSON text of exactly the captured model; the
  // solver mode travels as a header. Budget policy is never part of the model.
  // An uncaptured (unchecked) model is still sent as its JSON text and registers nothing.
  const text = capture ? checkedJsonText(capture.invocation.request.model) : JSON.stringify(model);
  let receipt: Record<string, unknown>;
  try {
    receipt = await invoke<Record<string, unknown>>(
      "start_preview_mechanics_job",
      new TextEncoder().encode(text),
      { headers: { [SOLVER_MODE_HEADER]: solverMode } },
    );
  } catch (error) {
    throw nativeInvokeError(error);
  }
  if (typeof receipt?.job_id !== "string" || !receipt.job_id) throw new Error("SOLVE-JOB-RECEIPT-INVALID: the native start receipt names no job.");
  if (capture) jobInvocations.set(receipt.job_id, capture);
  return {
    mode: "backend_job",
    job_id: receipt.job_id,
    backend_cancellation_token: String(receipt.backend_cancellation_token ?? ""),
    state: String(receipt.state ?? "queued"),
    cancellation_scope: String(receipt.cancellation_scope ?? ""),
    resource: parseSolveResourceGrant(receipt.resource),
  };
}

export async function pollPreviewMechanicsJob(
  jobId: string,
): Promise<BackendSolveJobStatus> {
  let native: Record<string, unknown>;
  try { native = await invoke<Record<string, unknown>>("poll_preview_mechanics_job", { jobId }); }
  catch (error) { throw nativeInvokeError(error); }
  const capture = jobInvocations.get(jobId) ?? null;
  if (native?.job_id !== jobId) {
    if (capture) capture.invalidated = true;
    jobInvocations.delete(jobId);
    throw new Error("SOLVE-JOB-IDENTITY-MISMATCH");
  }
  if (!BACKEND_JOB_STATES.includes(native.state as BackendSolveJobState)) throw new Error(`SOLVE-JOB-STATE-UNSUPPORTED: ${String(native.state)}`);
  const status: BackendSolveJobStatus = {
    job_id: jobId,
    state: native.state as BackendSolveJobState,
    queue_reason: parseResourceRefusal(native.queue_reason),
    cancellation_requested: native.cancellation_requested === true,
    cancellation_status: String(native.cancellation_status ?? ""),
    cancellation_scope: String(native.cancellation_scope ?? ""),
    result_available: native.result_available === true,
    result_bytes: typeof native.result_bytes === "number" ? native.result_bytes : null,
    error_message: typeof native.error_message === "string" ? native.error_message : null,
    resource_refusal: parseResourceRefusal(native.resource_refusal),
    resource: parseSolveResourceGrant(native.resource),
  };
  // A job that ends without a result retires its capture; a completed job's
  // capture waits for the take, which registers the bytes it delivers.
  if (status.state !== "completed" && TERMINAL_JOB_STATES.includes(status.state) && capture && !capture.terminalClaimed) {
    capture.terminalClaimed = true;
    capture.invalidated = true;
    if (jobInvocations.get(jobId) === capture) jobInvocations.delete(jobId);
  }
  return status;
}

/** Polls until the job reaches a terminal state; `onStatus` sees every status read. */
export async function awaitPreviewMechanicsJob(
  jobId: string,
  onStatus?: (status: BackendSolveJobStatus) => void,
): Promise<BackendSolveJobStatus> {
  let status = await pollPreviewMechanicsJob(jobId);
  onStatus?.(status);
  while (!TERMINAL_JOB_STATES.includes(status.state)) {
    await new Promise((resolve) => setTimeout(resolve, SOLVE_JOB_POLL_INTERVAL_MS));
    status = await pollPreviewMechanicsJob(jobId);
    onStatus?.(status);
  }
  return status;
}

/** The delivered results the app holds, by result, with their native job. */
const deliveredResults = new WeakMap<MechanicsResult, { jobId: string; disposed: boolean }>();

function decodeResultBytes(raw: unknown): string {
  let bytes: Uint8Array;
  if (Object.prototype.toString.call(raw) === "[object ArrayBuffer]") bytes = new Uint8Array(raw as ArrayBuffer);
  else if (ArrayBuffer.isView(raw)) bytes = new Uint8Array(raw.buffer, raw.byteOffset, raw.byteLength);
  else throw new Error("PREVIEW-SOLVE-RESULT-BYTES-REQUIRED: the native solve hands over the producer's serialized bytes");
  try { return new TextDecoder("utf-8", { fatal: true }).decode(bytes); }
  catch { throw new Error("PREVIEW-SOLVE-RESULT-UTF8-INVALID: the native result bytes are not UTF-8"); }
}

/** Takes a completed job's result: the producer's exact bytes, read once and
 * registered against the captured invocation as a completed job always was. The
 * result is then in the app's custody until `disposeDeliveredMechanicsResult`.
 * A refused take (for example `custody_full`) leaves the result with the native
 * host, and the job's capture stays available for a later take. */
export async function takePreviewMechanicsJobResult(jobId: string): Promise<MechanicsResult> {
  let raw: unknown;
  try { raw = await invoke<unknown>("take_preview_mechanics_job_result", { jobId }); }
  catch (error) { throw nativeInvokeError(error); }
  const capture = jobInvocations.get(jobId) ?? null;
  let result: MechanicsResult;
  try {
    result = receiveResultBytes(decodeResultBytes(raw));
  } catch (error) {
    // The app received bytes it cannot read: retire the capture and release custody.
    if (capture) { capture.terminalClaimed = true; capture.invalidated = true; }
    if (jobInvocations.get(jobId) === capture) jobInvocations.delete(jobId);
    await disposePreviewMechanicsResult(jobId);
    throw error;
  }
  deliveredResults.set(result, { jobId, disposed: false });
  if (capture && !capture.terminalClaimed) {
    capture.terminalClaimed = true;
    try {
      await validateCapturedSource(result, capture);
    } finally {
      if (jobInvocations.get(jobId) === capture) jobInvocations.delete(jobId);
    }
  }
  return result;
}

/** Jobs whose result or custody the native host reported released: a job's
 * result is released at most once, so a later discard of the same job (for
 * example after its take already released an unreadable delivery) sends nothing. */
const releasedJobs = new Set<string>();

/** Releases a delivered result from the native custody ledger. Best effort: a
 * failure is reported as false and never thrown. */
export async function disposePreviewMechanicsResult(jobId: string): Promise<boolean> {
  if (releasedJobs.has(jobId)) return false;
  try {
    const receipt = await invoke<{ released?: unknown } | null>("dispose_preview_mechanics_result", { jobId });
    if (receipt?.released === true) releasedJobs.add(jobId);
    return true;
  } catch {
    return false;
  }
}

/** Releases the delivered result the app is dropping (replaced, cleared or used
 * up). Results that were not delivered by a native job, or were already
 * released, are ignored. */
export function disposeDeliveredMechanicsResult(result: MechanicsResult | null | undefined): Promise<boolean> {
  const delivered = result ? deliveredResults.get(result) : undefined;
  if (!delivered || delivered.disposed) return Promise.resolve(false);
  delivered.disposed = true;
  return disposePreviewMechanicsResult(delivered.jobId);
}

/** Query only: the native job a held result was delivered by, while it is held. */
export function deliveredMechanicsResultJob(result: MechanicsResult | null | undefined): string | null {
  const delivered = result ? deliveredResults.get(result) : undefined;
  return delivered && !delivered.disposed ? delivered.jobId : null;
}

/** A completed job whose result the app will not take (its run was superseded,
 * or its take was refused and the session does not retry): retire its capture
 * and release the retained result natively, without copying it to the app. */
export async function discardPreviewMechanicsJobResult(jobId: string): Promise<boolean> {
  retireJobCapture(jobId);
  return disposePreviewMechanicsResult(jobId);
}

function retireJobCapture(jobId: string): void {
  const capture = jobInvocations.get(jobId);
  if (capture) { capture.terminalClaimed = true; capture.invalidated = true; jobInvocations.delete(jobId); }
}

/** A started job whose terminal state the app did not establish (a poll
 * failed, its identity did not match, or its run was superseded before it was
 * awaited) and whose result the app will not take: request its
 * cancellation, so the native host drops a result it has not yet published,
 * then release whatever it already retains. Best effort; never throws. A
 * result neither step reaches stays with the native host until the next
 * session start releases it (`beginSessionResultCustodyReset`). */
export async function abandonPreviewMechanicsJob(jobId: string, cancellationToken: string): Promise<boolean> {
  retireJobCapture(jobId);
  try { await invoke("cancel_preview_mechanics_job", { jobId, cancellationToken }); } catch { /* best effort */ }
  return disposePreviewMechanicsResult(jobId);
}

/** What `dispose_all_preview_mechanics_results` reported releasing. */
export type NativeCustodyReset = { released_results: number; released_bytes: number; abandoned_jobs: number };

/** Releases every result the native host retains or counts as held by this
 * webview, untaken or delivered, and abandons every queued or running job (they
 * end cancelled without publishing; their owners release when the worker
 * finishes). Only meaningful when no result is held: a
 * fresh webview holds none, so the session calls it once at start. Best
 * effort: null when unavailable or unreadable; never throws. */
export async function disposeAllPreviewMechanicsResults(): Promise<NativeCustodyReset | null> {
  if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) return null;
  try {
    const reported = await invoke<Record<string, unknown>>("dispose_all_preview_mechanics_results");
    const results = reported?.released_results, bytes = reported?.released_bytes, abandoned = reported?.abandoned_jobs;
    return Number.isSafeInteger(results) && Number.isSafeInteger(bytes) && Number.isSafeInteger(abandoned)
      ? { released_results: results as number, released_bytes: bytes as number, abandoned_jobs: abandoned as number }
      : null;
  } catch {
    return null;
  }
}

let sessionCustodyReset: Promise<NativeCustodyReset | null> | null = null;

/** Once per webview session, before any solve starts: a new webview (first
 * launch or reload) holds no delivered result, so whatever the native host
 * still retains or counts as webview custody belongs to a page that is gone
 * and would otherwise fill the retention and custody limits for good. Starts
 * the release at most once (Tauri runtime only) and returns the same promise;
 * `startPreviewMechanicsJob` waits for it. It never rejects, so a failed
 * release never blocks startup or a solve. */
export function beginSessionResultCustodyReset(): Promise<NativeCustodyReset | null> {
  if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) return Promise.resolve(null);
  sessionCustodyReset ??= disposeAllPreviewMechanicsResults();
  return sessionCustodyReset;
}

/** Test support only: forget the session's startup release and the jobs
 * already released (mocked transports reuse job ids across tests). */
export function resetSessionResultCustodyForTests(): void {
  sessionCustodyReset = null;
  releasedJobs.clear();
}

export async function cancelPreviewMechanicsJob(
  jobId: string,
  cancellationToken: string,
): Promise<BackendSolveJobCancellationReceipt> {
  const receipt = await invoke<BackendSolveJobCancellationReceipt>(
    "cancel_preview_mechanics_job",
    {
      jobId,
      cancellationToken,
    },
  );
  if (receipt.accepted) {
    const capture = jobInvocations.get(jobId);
    if (capture) capture.invalidated = true;
    jobInvocations.delete(jobId);
  }
  return receipt;
}

// The three frozen automatic rule-check statuses a GUI rule-check run may
// produce (mirrors core/runner/headless `analysis_status_for_rule_check`).
// Anything outside this set is NOT silently coerced (CONTRACT
// no-silent-defaults) — the analysis-run record falls back to the solve
// envelope's own rule_check rather than trusting an unrecognized aggregate.
const RULE_CHECK_RUN_STATUSES = new Set([
  "RULE_INPUTS_INCOMPLETE",
  "USER_RULE_CHECKED",
  "USER_RULE_FAILED",
]);

// Resolve the rule-check status the app-held analysis-run record should carry.
// A recognized GUI rule-check aggregate (incomplete-first over the checks the user
// ran against THIS solve) supersedes the solve envelope's `rule_check`, which a
// plain solve always leaves at RULE_INPUTS_INCOMPLETE because the solve runs no
// user rule checks. An absent or unrecognized aggregate falls back to the solve
// envelope's own rule_check — never a false pass.
export function appliedRuleCheckStatus(
  solveRuleCheck: string,
  ruleCheckAggregate?: string | null,
): string {
  if (ruleCheckAggregate && RULE_CHECK_RUN_STATUSES.has(ruleCheckAggregate)) {
    return ruleCheckAggregate;
  }
  return solveRuleCheck;
}

// Build the app-held analysis-run envelope (DEL-14-02) for a solved result.
// When a GUI rule-check aggregate is supplied (TP-C4-APPAGG-001), the record's
// own `analysis_status` and its `analysis_run_record` hash honestly compose the
// mechanics solve with the rule-check run executed against it; the embedded
// `result_envelope` hash still binds the raw solve (which ran no rule checks),
// so the hash-bound solve envelope is never mutated. Omitting the aggregate
// reproduces the prior behavior byte-for-byte.
export async function buildAnalysisRunPreview(
  result: MechanicsResult,
  {
    inputManifest,
    ruleCheckAggregate,
  }: {
    inputManifest: CurrentSessionInputManifestEvidence;
    ruleCheckAggregate?: string | null;
  },
): Promise<AnalysisRunEnvelope> {
  await verifyCurrentSessionInputManifest(inputManifest);
  const effective = appliedRuleCheckStatus(result.status.rule_check, ruleCheckAggregate);
  if (!hasCurrentSourceContract(result)) throw new Error("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
  return buildAnalysisRunV03(result, inputManifest, effective, modelLoadBasisRefs(inputManifest.manifest.model_basis.model_payload));
}

export function declaredSourceResultDimension(
  item: Omit<MechanicsResult["results"][number], "dimension"> &
    Partial<Pick<MechanicsResult["results"][number], "dimension">>,
): CanonicalResultDimension {
  const kind = item.kind;
  if (kind === "support_reaction_component_v2") {
    if (["Fx", "Fy", "Fz"].includes(item.metadata?.component ?? "")) return "force";
    if (["Mx", "My", "Mz"].includes(item.metadata?.component ?? "")) return "moment";
    throw new Error("SUPPORT_COMPONENT_UNSUPPORTED");
  }
  const component = item.metadata?.component;
  if (kind === "constant_effort_user_input_review") {
    if (component === "constant_effort_support_constant_load") return "force";
    if (component === "constant_effort_support_travel_range") return "length";
  }
  if (kind === "spring_hanger_user_input_review") {
    if (component === "variable_spring_hanger_stiffness")
      return "linear_stiffness";
    if (
      component === "variable_spring_hanger_installed_load" ||
      component === "variable_spring_hanger_cold_load" ||
      component === "variable_spring_hanger_hot_load"
    ) {
      return "force";
    }
    if (component === "variable_spring_hanger_travel_range") return "length";
  }
  const declarations: Record<string, CanonicalResultDimension> = {
    component_user_stress_multiplier_review: "stress",
    displacement_magnitude: "length",
    element_local_axial_force: "force",
    element_local_axial_normal_stress: "stress",
    element_local_bending_moment_y: "moment",
    element_local_bending_moment_z: "moment",
    element_local_bending_normal_stress_y: "stress",
    element_local_bending_normal_stress_z: "stress",
    element_local_shear_force_y: "force",
    element_local_shear_force_z: "force",
    element_local_torsional_moment: "moment",
    element_local_torsional_shear_stress: "stress",
    global_nodal_displacement_x: "length",
    global_nodal_displacement_y: "length",
    global_nodal_displacement_z: "length",
    global_nodal_rotation_x: "angle",
    global_nodal_rotation_y: "angle",
    global_nodal_rotation_z: "angle",
    linear_solver_mode_basis: "dimensionless",
    nonlinear_support_active_set_converged_flag: "dimensionless",
    nonlinear_support_active_set_final_residual_count: "dimensionless",
    nonlinear_support_active_set_iteration_count: "dimensionless",
    nonlinear_support_active_set_state_code: "dimensionless",
    nonlinear_support_final_displacement: "length",
    nonlinear_support_final_reaction: "force",
    nonlinear_support_free_dof_work_residual: "moment",
    nonlinear_support_friction_normal_reaction_derived: "force",
    nonlinear_support_observed_free_dof_force_residual: "force",
    nonlinear_support_observed_free_dof_moment_residual: "moment",
    nonlinear_support_observed_max_force_reaction_delta: "force",
    nonlinear_support_observed_max_moment_reaction_delta: "moment",
    nonlinear_support_observed_max_rotation_delta: "angle",
    nonlinear_support_observed_max_translation_delta: "length",
    open_formula_stress_summary: "stress",
    pipe_section_pressure_hoop_stress: "stress",
    reaction_resultant: "force",
  };
  const declared = declarations[kind];
  if (!declared) {
    throw new Error(
      `ANALYSIS-RUN-RESULT-DIMENSION-UNDECLARED: ${item.id} (${item.kind}) has no source dimension declaration.`,
    );
  }
  return declared;
}

export function buildPreviewComparison({
  result,
  analysisRun,
  leftBasisRef = "load:L-100",
  rightBasisRef = "combination:C-OPER-ALT",
}: {
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
  leftBasisRef?: string;
  rightBasisRef?: string;
}): PreviewComparison {
  const run = analysisRun.analysis_run;
  const resultIndex = new Map(result.results.map((item) => [item.id, item]));
  const leftResults = result.results.filter(
    (item) => item.basis_ref?.ref_id === leftBasisRef,
  );
  const rightResults = result.results.filter(
    (item) => item.basis_ref?.ref_id === rightBasisRef,
  );
  const matchedLeftIds = new Set<string>();
  const unmatchedRightRefs: string[] = [];
  const deltas = rightResults
    .flatMap((rightResult) => {
      const leftResultId = rightResult.source_result_refs?.find((sourceRef) =>
        resultIndex.has(sourceRef),
      );
      if (!leftResultId) {
        unmatchedRightRefs.push(rightResult.id);
        return [];
      }
      const leftResult = resultIndex.get(leftResultId);
      if (!leftResult || leftResult.unit !== rightResult.unit) {
        unmatchedRightRefs.push(rightResult.id);
        return [];
      }
      matchedLeftIds.add(leftResult.id);
      return [
        {
          mapping_id: `mapping:${leftResult.id}->${rightResult.id}`,
          left_result_id: leftResult.id,
          right_result_id: rightResult.id,
          entity_ref: rightResult.entity_ref,
          result_family: resultFamily(
            rightResult,
            rightResult.dimension ??
              declaredSourceResultDimension(rightResult),
          ),
          component: rightResult.metadata?.component ?? rightResult.kind,
          location: rightResult.metadata?.location ?? "summary",
          unit: rightResult.unit,
          left_value: leftResult.value,
          right_value: rightResult.value,
          raw_delta: rightResult.value - leftResult.value,
          absolute_delta: Math.abs(rightResult.value - leftResult.value),
          classification: "not_tolerance_checked" as const,
          classification_basis:
            "governed_tolerance_profile_TBD_no_default_engineering_threshold",
        },
      ];
    })
    .sort((left, right) => {
      const byDelta = right.absolute_delta - left.absolute_delta;
      return byDelta === 0
        ? left.right_result_id.localeCompare(right.right_result_id)
        : byDelta;
    });
  const unmatchedLeftRefs = leftResults
    .map((item) => item.id)
    .filter((id) => !matchedLeftIds.has(id));
  const diagnostics = comparisonDiagnostics({
    unmatchedLeftRefs,
    unmatchedRightRefs,
  });
  const matchedResultUnits = Array.from(
    new Set(deltas.map((item) => item.unit).filter(Boolean)),
  ).sort();

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.comparison",
    deliverable_id: "DEL-14-04",
    package_id: "PKG-14",
    scope_items: ["SOW-073"],
    objectives: ["OBJ-016"],
    comparison_id: `comparison:${result.run_id}:${safeComparisonToken(leftBasisRef)}-to-${safeComparisonToken(rightBasisRef)}`,
    comparison_kind: "single_run_load_basis_review",
    left: {
      label: "Reference load-case result rows",
      basis_ref: ref("LoadCase", leftBasisRef),
      model_state_ref: run.model_state_ref,
      analysis_run_ref: ref("AnalysisRun", run.run_id),
      result_count: leftResults.length,
    },
    right: {
      label: "User-defined combination result rows",
      basis_ref: ref("Combination", rightBasisRef),
      model_state_ref: run.model_state_ref,
      analysis_run_ref: ref("AnalysisRun", run.run_id),
      result_count: rightResults.length,
    },
    summary: {
      comparable_result_pairs: deltas.length,
      unmatched_left_results: unmatchedLeftRefs.length,
      unmatched_right_results: unmatchedRightRefs.length,
      mapping_basis:
        "stable result IDs plus explicit source_result_refs from the preview mechanics result envelope",
      tolerance_status: "not_tolerance_checked",
      tolerance_profile_ref: "TBD",
    },
    unit_policy_evidence: {
      evidence_id: "unit-policy-evidence:comparison-workspace-preview",
      unit_system_ref: ref("UnitSystem", "unit-system:dec-018-si-dual-display"),
      storage_convention: "entered_units_preserved",
      comparison_unit_policy:
        "compare_only_rows_with_equal_explicit_result_units",
      matching_policy:
        "stable_result_refs_must_match_and_units_must_be_equal_before_delta",
      matched_result_units: matchedResultUnits,
      unmatched_left_result_count: unmatchedLeftRefs.length,
      unmatched_right_result_count: unmatchedRightRefs.length,
      conversion_policy:
        "comparison_workspace_preserves_result_units_without_conversion",
      conversion_performed: false,
      tolerance_profile_ref: "TBD",
      tolerance_status: "not_tolerance_checked",
      decision_basis_refs: [
        ref("Decision", "DEC-018"),
        ref("Decision", "DEC-026"),
        ref("Deliverable", "DEL-14-05"),
      ],
      protected_content_included: false,
      private_payload_included: false,
    },
    result_deltas: deltas,
    diagnostics,
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false,
    },
  };
}


function comparisonDiagnostics({
  unmatchedLeftRefs,
  unmatchedRightRefs,
}: {
  unmatchedLeftRefs: string[];
  unmatchedRightRefs: string[];
}) {
  const diagnostics = [];
  if (unmatchedLeftRefs.length > 0) {
    diagnostics.push({
      id: "diagnostic:comparison:reference-unmatched",
      code: "COMPARISON_REFERENCE_ROWS_UNMATCHED",
      severity: "info" as const,
      message:
        "Some reference load-case result rows do not have a matching user-defined combination row in the preview comparison.",
      source: "apps/desktop/src/services/previewService.ts",
      affected_refs: unmatchedLeftRefs.slice(0, 12),
    });
  }
  if (unmatchedRightRefs.length > 0) {
    diagnostics.push({
      id: "diagnostic:comparison:target-unmatched",
      code: "COMPARISON_TARGET_ROWS_UNMATCHED",
      severity: "warning" as const,
      message:
        "Some user-defined combination result rows could not be mapped back to source result rows for preview comparison.",
      source: "apps/desktop/src/services/previewService.ts",
      affected_refs: unmatchedRightRefs.slice(0, 12),
    });
  }
  return diagnostics;
}

function safeComparisonToken(value: string): string {
  return (
    value.replace(/[^a-zA-Z0-9_-]+/g, "-").replace(/^-+|-+$/g, "") || "basis"
  );
}

export async function loadSampleProposal(
  mechanicsResult?: MechanicsResult | null,
  selectedTarget?: SelectedReviewTarget | null,
): Promise<AgentProposal> {
  if (typeof window !== "undefined" && "__TAURI_INTERNALS__" in window) {
    try {
      return (
        await invokeWithIngress<{ proposal: AgentProposal }>("sample_agent_proposal", {
          mechanicsResult,
          selectedTarget,
        })
      ).proposal;
    } catch {
      // Fall through to the local fixture-backed proposal below.
    }
  }
  return buildProposalFromMechanics(
    mechanicsResult ?? (await loadMechanicsFixture()),
    await loadAgentProposalFixture(),
    selectedTarget,
  );
}

/** The default session model and the model of the bundled reference results:
 * the valid demo, the invented loop without the retired legacy pressure
 * primitives and the refused expansion joint, which the product solves. */
async function loadModelFixture(): Promise<PreviewModel> {
  return structuredClone((
    await import("../../../../fixtures/product_preview/invented_demo_model.json")
  ).default) as PreviewModel;
}

async function loadKnowledgeFixture(): Promise<DesignKnowledge> {
  return (
    await import("../../../../fixtures/product_preview/invented_design_knowledge.json")
  ).default as DesignKnowledge;
}

async function loadMechanicsFixture(
  solverMode: PreviewSolverMode = "sparse_interactive",
): Promise<MechanicsResult> {
  assertPreviewSolverMode(solverMode);
  const text = solverMode === "sparse_interactive"
    ? (await import("../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json?raw")).default
    : (await import("../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_dense.json?raw")).default;
  // A fresh reading per call, so one caller cannot change the bundled producer evidence
  // for subsequent invocations. No header, row, dimension or quality is manufactured.
  // The readers read the bundled bytes as received (b7-plan R2).
  const result = JSON.parse(text) as MechanicsResult;
  rememberReceivedText(result, text);
  return result;
}

/** Validate the preview-physics-2 evidence and mode evidence of a selected bundled
 * producer result, not authenticate an arbitrary caller's source/build or upgrade
 * its numerical quality. */
export function validateBrowserMechanicsFixture(
  result: MechanicsResult,
  solverMode: PreviewSolverMode,
  fixtureModel: PreviewModel,
): MechanicsResult {
  assertPreviewSolverMode(solverMode);
  if (sourceContract(result) !== "preview_physics") {
    throw new Error("BROWSER_FIXTURE_PRODUCER_CONTRACT_UNSUPPORTED");
  }
  if (result.model_ref !== fixtureModel.project.id) {
    throw new Error("BROWSER_FIXTURE_MODEL_BINDING_MISMATCH");
  }
  // A genuine blocked producer return can have no execution rows. Preserve that
  // failure as received; it supplies no completed mode/solve evidence to promote.
  if (result.status.mechanics !== "MECHANICS_SOLVED" && result.results.length === 0) {
    readerValidate("preview-physics", result, fixtureModel);
    return result;
  }
  const cases = fixtureModel.load_cases.map(item => item.id);
  const rows = result.results.filter(item => item.kind === "linear_solver_mode_basis");
  const expectedValue = solverMode === "sparse_interactive" ? 1 : 2;
  const expectedBasis = solverMode === "sparse_interactive"
    ? "sparse_structural_integrity_primary" : "dense_structural_integrity_primary";
  const field = (basis: string, key: string) => basis.split(";")
    .map(part => part.trim()).filter(part => part.startsWith(`${key}=`));
  if (!cases.length || new Set(cases).size !== cases.length || rows.length !== cases.length
    || cases.some(id => rows.filter(row => row.basis_ref?.ref_type === "load_case" && row.basis_ref.ref_id === id).length !== 1)
    || rows.some(row => row.value !== expectedValue || row.unit !== "mode_code"
      || row.metadata?.component !== "linear_solver_mode"
      || row.metadata.coordinate_system !== "reduced_system"
      || row.metadata.location !== row.basis_ref?.ref_id
      || typeof row.metadata.basis !== "string"
      || field(row.metadata.basis, "solver_mode").join() !== `solver_mode=${solverMode}`
      || field(row.metadata.basis, "solution_basis").join() !== `solution_basis=${expectedBasis}`)) {
    throw new Error(`BROWSER_FIXTURE_SOLVER_MODE_BINDING_MISMATCH: ${solverMode}`);
  }
  // The preview-physics-2 reader checks the bundled bytes as received.
  readerValidate("preview-physics", result, fixtureModel);
  return result;
}

export type BundledMechanicsReference = {
  standing: "reference_only";
  model: PreviewModel;
  source: MechanicsResult;
  provenance: {
    origin: "preserved_bundled_producer_record";
    fixture_ref: string;
    recorded_solver_mode: PreviewSolverMode;
    fresh_invocation_performed: false;
    current_use_eligible: false;
    notice: string;
  };
};

/** Explicit reference inspection, separate from the fresh solve/job APIs.
 * Provenance stays outside the unchanged producer data. Reading this record
 * does not mint a live solve invocation, Current, rule or export eligibility. */
export async function loadBundledMechanicsReference(
  solverMode: PreviewSolverMode = "sparse_interactive",
): Promise<BundledMechanicsReference> {
  assertPreviewSolverMode(solverMode);
  const model = await loadModelFixture();
  const source = validateBrowserMechanicsFixture(await loadMechanicsFixture(solverMode), solverMode, model);
  return {
    standing: "reference_only",
    model,
    source,
    provenance: {
      origin: "preserved_bundled_producer_record",
      fixture_ref: solverMode === "sparse_interactive"
        ? "fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json"
        : "fixtures/product_preview/invented_demo_result_preview_physics_1_dense.json",
      recorded_solver_mode: solverMode,
      fresh_invocation_performed: false,
      current_use_eligible: false,
      notice: "Preserved reference data; no solve was performed. This record is unavailable for Current, rule checks or qualified export.",
    },
  };
}

export const BROWSER_REFERENCE_SOLVE_DIAGNOSTIC = "BROWSER_SOLVE_BACKEND_REQUIRED_REFERENCE_ONLY: Browser mechanics requires a real solver backend. Bundled records are available only through reference inspection; use the native application to solve.";
async function browserSolveUnavailable(model?: PreviewModel | null): Promise<Error> {
  if (model && canonicalJson(model) !== canonicalJson(await loadModelFixture())) {
    return new Error("BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL: Browser reference data cannot solve an edited model; use the native backend for model-bound mechanics results.");
  }
  return new Error(BROWSER_REFERENCE_SOLVE_DIAGNOSTIC);
}

async function loadAgentProposalFixture(): Promise<AgentProposal> {
  return (
    await import("../../../../fixtures/product_preview/invented_agent_proposal.json")
  ).default as AgentProposal;
}

function ref(objectType: string, value: string): ObjectRef {
  return { object_type: objectType, ref: value };
}

function resultFamily(
  result: MechanicsResult["results"][number],
  sourceDimension: CanonicalResultDimension,
): string {
  const expectedDimension = declaredSourceResultDimension(result);
  if (sourceDimension !== expectedDimension) {
    throw new Error(
      `ANALYSIS-RUN-RESULT-FAMILY-DIMENSION-MISMATCH: ${result.id} declares ${sourceDimension}; exact kind semantics require ${expectedDimension}.`,
    );
  }
  if (
    sourceDimension === "force" &&
    (
      result.kind === "reaction_resultant" ||
      result.kind === "nonlinear_support_final_reaction" ||
      result.kind === "nonlinear_support_friction_normal_reaction_derived" ||
      result.kind === "nonlinear_support_observed_max_force_reaction_delta"
    )
  ) {
    return "reaction";
  }
  if (
    sourceDimension === "moment" &&
    result.kind === "nonlinear_support_observed_max_moment_reaction_delta"
  ) {
    return "reaction";
  }
  const families: Partial<Record<CanonicalResultDimension, string>> = {
    length: "displacement",
    angle: "rotation",
    force: "force",
    moment: "moment",
    stress: "stress",
    pressure: "stress",
    ratio: "ratio",
    dimensionless: "ratio",
  };
  return families[sourceDimension] ?? "TBD";
}

function canonicalJson(value: unknown): string {
  return JSON.stringify(sortJson(value));
}

function sortJson(value: unknown): unknown {
  if (Array.isArray(value)) {
    return value.map(sortJson);
  }
  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value)
        .sort(([left], [right]) => left.localeCompare(right))
        .map(([key, nested]) => [key, sortJson(nested)]),
    );
  }
  return value;
}

async function sha256(payload: string): Promise<string> {
  if (globalThis.crypto?.subtle) {
    const bytes = new TextEncoder().encode(payload);
    const digest = await globalThis.crypto.subtle.digest("SHA-256", bytes);
    return Array.from(new Uint8Array(digest))
      .map((byte) => byte.toString(16).padStart(2, "0"))
      .join("");
  }
  let hash = 5381;
  for (let index = 0; index < payload.length; index += 1) {
    hash = (hash * 33) ^ payload.charCodeAt(index);
  }
  return `sha256-unavailable-${(hash >>> 0).toString(16).padStart(8, "0")}`;
}

function buildProposalFromMechanics(
  result: MechanicsResult,
  agentProposalFixture: AgentProposal,
  selectedTarget?: SelectedReviewTarget | null,
): AgentProposal {
  const forceResult =
    result.results.find(
      (item) => item.id === "result:force:pipe-P-120:axial",
    ) ??
    result.results.find((item) => item.kind === "element_local_axial_force");
  const primaryDiagnostic =
    result.diagnostics.find(
      (item) => item.severity === "warning" || item.severity === "blocking",
    ) ?? result.diagnostics[0];
  const targetRef =
    selectedTarget?.id ??
    forceResult?.id ??
    primaryDiagnostic?.id ??
    primaryDiagnostic?.affected_refs?.[0] ??
    result.summary.max_displacement?.result_ref ??
    result.results[0]?.id ??
    "diagnostic:physics:context-unavailable";
  const targetKind =
    selectedTarget?.target_type.replaceAll("_", " ") ?? "computed mechanics";

  return {
    ...agentProposalFixture,
    proposal_id: "proposal:physics-diagnostic-review",
    prompt:
      "Review current computed mechanics diagnostics and suggest a non-mutating follow-up.",
    operation: {
      ...agentProposalFixture.operation,
      operation_id: "op:review-computed-diagnostic",
      operation_kind: "attach_design_knowledge",
      affected_entity_ids: [targetRef],
      changes: [
        {
          change_id: "change:add-review-note",
          change_kind: "attach_design_knowledge",
          target_ref: targetRef,
          before: `No review note attached for the selected ${targetKind} context.`,
          after: `Attach review note referencing the current computed preview ${targetKind} context.`,
        },
      ],
    },
    rationale: `Generated from current preview mechanics context; selected review reference is ${targetRef}. This narrative is review-only and does not mutate accepted model state.`,
    validation: {
      ...agentProposalFixture.validation,
      unit_validation: "not_required_metadata_review_only",
      constraint_validation: "warning_computed_context_requires_human_review",
      diff_preview_status: "generated_from_computed_context",
    },
  };
}
