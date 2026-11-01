/** Unit transport replay only. These are actual producer input/output bytes,
 * replayed through mocked IPC; this helper is NOT a native UI qualification.
 * It never installs a runtime, registers a source, or edits producer evidence. */
import { isDeepStrictEqual } from "node:util";
import modelFixture from "../../../../fixtures/model_operations/exact_pressure_authoring_model.json";
import sparseFixture from "../../../../fixtures/results/pressure_connected_ui_mechanics_sparse.json?raw";
import denseFixture from "../../../../fixtures/results/pressure_connected_ui_mechanics_dense.json?raw";
import precisionModelFixture from "../../../../fixtures/model_operations/precision_connected_ui_model.json";
import previewSparseFixture from "../../../../fixtures/results/preview_physics_connected_sparse.json?raw";
import previewDenseFixture from "../../../../fixtures/results/preview_physics_connected_dense.json?raw";
import { jsonTextKeepingNegativeZero } from "../services/jsonText";
import type { MechanicsResult, PreviewModel } from "../types";
import type { PreviewSolverMode } from "../services/previewService";
import { nativeReplayReply } from "./nativeReplayStore";
import { decodeIngressBody } from "./nativeIngress";

/** The bytes a mocked native solve hands over for a result value (b7-plan R1: the solve commands
 * and jobs hand over serialized bytes): its JSON text, negative zeros kept, unchecked. The reader
 * module writes parsed values with the same writer. */
export const solveBytes = jsonTextKeepingNegativeZero;

export const NATIVE_MECHANICS_REPLAY_NOTICE = "Unit transport replay of captured producer bytes; NOT actual native UI qualification";
/** `preview` is the fresh non-exact route (preview-physics-2); `pressure` the exact route
 * (pressure-2, 3.0.0/exact_pressure_v3 since B7b retired the v2 physics-1 pair); since
 * epoch 8 its result JSON and stress-neutral exports and rule check admit an eligible result. */
export type NativeMechanicsReplayProfile = "pressure" | "preview";
const captureRef = "execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/ENGINE_INTEGRATION/UI_PRODUCER_CAPTURE.json";
function replayMode(mode: unknown): asserts mode is PreviewSolverMode {
  if (mode !== "sparse_interactive" && mode !== "dense_scrutiny") throw new Error("NATIVE_REPLAY_MODE_UNSUPPORTED");
}
export function nativeMechanicsReplayPair(mode: PreviewSolverMode = "sparse_interactive", options: { profile?: NativeMechanicsReplayProfile } = {}) {
  replayMode(mode);
  const profile = options.profile ?? "pressure";
  if (profile !== "pressure" && profile !== "preview") throw new Error("NATIVE_REPLAY_PROFILE_UNSUPPORTED");
  const sparse = mode === "sparse_interactive";
  const model = profile === "pressure" ? modelFixture : precisionModelFixture;
  // The producer's bytes, as the native solve hands them over (b7-plan R1, exact bytes).
  const bytes = profile === "pressure" ? (sparse ? sparseFixture : denseFixture)
    : (sparse ? previewSparseFixture : previewDenseFixture);
  return {
    model: structuredClone(model) as PreviewModel,
    source: JSON.parse(bytes) as MechanicsResult,
    bytes,
    mode,
    profile,
    provenance: {
      kind: "unit_transport_replay" as const,
      actual_native_ui_qualification: false as const,
      notice: NATIVE_MECHANICS_REPLAY_NOTICE,
      capture_ref: profile === "pressure" ? captureRef : "fixtures/results/preview_physics_connected_{sparse,dense}.json (T0R S2a producer output)",
      model_fixture_ref: profile === "pressure" ? "fixtures/model_operations/exact_pressure_authoring_model.json" : "fixtures/model_operations/precision_connected_ui_model.json",
      source_fixture_ref: profile === "preview" ? `fixtures/results/preview_physics_connected_${sparse ? "sparse" : "dense"}.json` : `fixtures/results/pressure_connected_ui_mechanics_${sparse ? "sparse" : "dense"}.json`,
    },
  };
}

let instanceSequence = 0;
/** The resource grant a replayed job reports: the qualified 12 GiB default as
 * both budget and pool, the ordinary solve in a worker capped at its grant.
 * Replay data, not a native reading. */
export const REPLAY_RESOURCE_GRANT = Object.freeze({
  invocation_budget_bytes: 12 * 1024 ** 3,
  pool_bytes: 12 * 1024 ** 3,
  route_pricing: "ordinary_worker_capped" as const,
  worker_grant_bytes: null,
  worker_peak_grant_bytes: null,
  worker_peak_footprint_bytes: null,
});
/** The peak worker grant and peak footprint a replayed finished job reports
 * (invented figures, not measurements); its worker has exited, so it holds no
 * current grant. */
export const REPLAY_WORKER_PEAK_GRANT_BYTES = 256 * 1024 ** 2;
export const REPLAY_WORKER_PEAK_FOOTPRINT_BYTES = 192 * 1024 ** 2;
/** The UTF-8 bytes and solver-mode header a start command sends. */
export function startRequestModel(args: unknown, options: unknown): { model: unknown; mode: unknown; text: string } {
  if (!(args instanceof Uint8Array) && !ArrayBuffer.isView(args)) throw new Error("NATIVE_REPLAY_REQUEST_INVALID");
  const view = args as Uint8Array;
  const text = new TextDecoder("utf-8", { fatal: true }).decode(view);
  const headers = (options as { headers?: Record<string, unknown> } | undefined)?.headers;
  if (!headers || typeof headers !== "object") throw new Error("NATIVE_REPLAY_REQUEST_INVALID");
  return { model: JSON.parse(text), mode: headers["x-swbpipe-solver-mode"], text };
}
/** The bytes a take command hands over for a producer's text. */
export function resultArrayBuffer(text: string): ArrayBuffer {
  const bytes = new TextEncoder().encode(text);
  return bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength) as ArrayBuffer;
}
/** C1: the native result commands (raw-body ingress), answered with the actual Rust commands'
 * recorded replies (`nativeReplayStore.ts`). */
export const NATIVE_RESULT_COMMANDS = Object.freeze([
  "read_preview_mechanics_job_verdict",
  "read_preview_mechanics_job_summary",
  "query_preview_mechanics_job_rows",
  "lookup_preview_mechanics_job_rows",
  "read_preview_mechanics_job_row_semantics",
  "query_preview_mechanics_job_source_recovery",
  "query_preview_mechanics_job_navigator",
  "read_preview_mechanics_job_analysis_run_preview",
]);
export const SOLVE_JOB_COMMANDS = Object.freeze([
  "start_preview_mechanics_job",
  "poll_preview_mechanics_job",
  "take_preview_mechanics_job_result",
  "dispose_preview_mechanics_result",
  "cancel_preview_mechanics_job",
  "read_preview_mechanics_job_rows_all",
  "release_preview_mechanics_job_full_copy",
  ...NATIVE_RESULT_COMMANDS,
]);
/** The native host's limit on a C2 full copy (PR #51's 64 MiB). */
export const FULL_COPY_LIMIT_BYTES = 64 * 1024 * 1024;

/** The session-start release of every retained or delivered result. */
export const DISPOSE_ALL_COMMAND = "dispose_all_preview_mechanics_results";

/** A mocked native solve-job transport speaking the job contract (raw-bytes
 * start with the solver-mode header, poll without result, take, dispose,
 * cancel). `answer` supplies the producer bytes for each accepted
 * start; a thrown answer rejects the start. Unit simulation only. */
export function createSolveJobTransport(
  answer: (request: { model: unknown; mode: unknown; text: string }) => string | Promise<string>,
  options: { jobIdPrefix?: string; cancellationScope?: string; fullCopyLimitBytes?: number } = {},
) {
  const fullCopyLimit = options.fullCopyLimitBytes ?? FULL_COPY_LIMIT_BYTES;
  const prefix = options.jobIdPrefix ?? `unit-solve-job:${++instanceSequence}`;
  const scope = options.cancellationScope ?? "unit_transport_replay_not_native_ui_qualification";
  const jobs = new Map<string, { bytes: string; ingress: number; cancelled: boolean; taken: boolean; disposed: boolean; native: boolean; fullCopy: boolean }>();
  const calls: string[] = [];
  let sequence = 0, disposeAllCalls = 0;
  // A queued job's worker holds no grant yet; a finished job's worker has exited, leaving its peak.
  const grant = (ingress: number, finished: boolean) => ({
    ...REPLAY_RESOURCE_GRANT,
    ingress_reserved_bytes: ingress,
    worker_peak_grant_bytes: finished ? REPLAY_WORKER_PEAK_GRANT_BYTES : null,
    worker_peak_footprint_bytes: finished ? REPLAY_WORKER_PEAK_FOOTPRINT_BYTES : null,
  });
  const handles = (command: string) => SOLVE_JOB_COMMANDS.includes(command);
  const invoke = async (command: string, args?: unknown, invokeOptions?: unknown): Promise<any> => {
    calls.push(command);
    if (command === "start_preview_mechanics_job") {
      const request = startRequestModel(args, invokeOptions);
      const bytes = await answer(request);
      const ingress = new TextEncoder().encode(request.text).byteLength, job_id = `${prefix}:${++sequence}`;
      const native = (invokeOptions as { headers?: Record<string, string> } | undefined)?.headers?.["x-swbpipe-result-custody"] === "native";
      jobs.set(job_id, { bytes, ingress, cancelled: false, taken: false, disposed: false, native, fullCopy: false });
      return { job_id, backend_cancellation_token: `${job_id}:token`, state: "queued", cancellation_scope: scope, resource: grant(ingress, false) };
    }
    if (command === DISPOSE_ALL_COMMAND) {
      // As the native host at session start: every retained or delivered result is released.
      let released_results = 0, released_bytes = 0;
      for (const job of jobs.values()) {
        if (job.disposed || job.cancelled) continue;
        job.disposed = true;
        released_results += 1;
        released_bytes += new TextEncoder().encode(job.bytes).byteLength;
      }
      disposeAllCalls += 1;
      return { released_results, released_bytes, abandoned_jobs: 0 };
    }
    if (!handles(command)) throw new Error(`NATIVE_REPLAY_COMMAND_UNSUPPORTED: ${command}`);
    if (NATIVE_RESULT_COMMANDS.includes(command)) {
      // As the native host: a raw-body request on a held native-custody result, answered by the
      // actual command's recorded reply for these exact arguments and bytes.
      // A raw body is decoded here; `ingressDecodingInvoke` hands over the arguments it decoded
      // (after asserting the raw body and header).
      const decoded = ArrayBuffer.isView(args) ? decodeIngressBody(command, args, invokeOptions as never) : args as Record<string, unknown>;
      const jobId = decoded.jobId as string;
      const held = jobs.get(jobId);
      if (!held) throw { code: "INVALID_REQUEST", message: `unknown solve job: ${jobId}` };
      if (held.cancelled || held.disposed || held.taken) throw { code: "INVALID_REQUEST", message: `PREVIEW-SOLVE-RESULT-UNAVAILABLE: ${jobId} is ${held.taken ? "delivered" : "released"}` };
      return nativeReplayReply(command, decoded, jobId, held.bytes);
    }
    const request = args as { jobId?: unknown; cancellationToken?: unknown } | undefined;
    const job = typeof request?.jobId === "string" ? jobs.get(request.jobId) : undefined;
    if (!job) throw new Error("NATIVE_REPLAY_JOB_UNKNOWN");
    const jobId = request!.jobId as string;
    if (command === "cancel_preview_mechanics_job") {
      if (request!.cancellationToken !== `${jobId}:token`) throw new Error("NATIVE_REPLAY_CANCELLATION_TOKEN_MISMATCH");
      job.cancelled = true;
      return { job_id: jobId, accepted: true, cancellation_status: "cancelled", job_state: "cancelled", cancellation_scope: scope, cancellation_success_claimed: false };
    }
    if (command === "read_preview_mechanics_job_rows_all") {
      // As `SolveJobRegistry::copy_result_within`: a native-custody result's exact bytes, at or
      // under the limit, one copy at a time; above it refused by name before anything is copied.
      if (!job.native) throw { code: "INVALID_REQUEST", message: `PREVIEW-SOLVE-RESULT-NOT-NATIVE: ${jobId} is a webview-custody result; take it` };
      if (job.cancelled || job.disposed) throw { code: "INVALID_REQUEST", message: `PREVIEW-SOLVE-RESULT-UNAVAILABLE: ${jobId} is released` };
      const length = new TextEncoder().encode(job.bytes).byteLength;
      if (length > fullCopyLimit) throw { code: "INVALID_REQUEST", message: `RESULT-DETAIL-WEBVIEW-LIMIT (result_bytes=${length}, limit_bytes=${fullCopyLimit}): ${jobId}'s result is ${length} bytes, above the ${fullCopyLimit} bytes the webview can hold whole; nothing was copied, and the result stays held natively.` };
      if (job.fullCopy) throw { code: "INVALID_REQUEST", message: `RESULT-DETAIL-FULL-COPY-HELD: ${jobId}'s full copy is already held; release it first.` };
      job.fullCopy = true;
      return resultArrayBuffer(job.bytes);
    }
    if (command === "release_preview_mechanics_job_full_copy") {
      const released = job.fullCopy;
      job.fullCopy = false;
      return released;
    }
    if (command === "take_preview_mechanics_job_result") {
      if (job.native) throw { code: "INVALID_REQUEST", message: `PREVIEW-SOLVE-RESULT-NATIVE-CUSTODY: ${jobId} is held natively; read it through the native result commands.` };
      if (job.cancelled || job.taken || job.disposed) throw { code: "INVALID_REQUEST", message: `NATIVE_REPLAY_RESULT_UNAVAILABLE: ${jobId}` };
      job.taken = true;
      return resultArrayBuffer(job.bytes);
    }
    if (command === "dispose_preview_mechanics_result") {
      // As the native host: releases a delivered copy or a retained, untaken result; a repeat releases nothing.
      const released = !job.disposed && !job.cancelled;
      job.disposed = true;
      return { job_id: jobId, released, released_bytes: released ? new TextEncoder().encode(job.bytes).byteLength : 0, delivered: job.taken };
    }
    const available = !job.cancelled && !job.taken && !job.disposed;
    return { job_id: jobId, state: job.cancelled ? "cancelled" : "completed", queue_reason: null, cancellation_requested: job.cancelled,
      cancellation_status: job.cancelled ? "cancelled" : "not_requested", cancellation_scope: scope,
      result_available: available, result_bytes: available ? new TextEncoder().encode(job.bytes).byteLength : null, error_message: null,
      resource_refusal: null, resource: grant(job.ingress, !job.cancelled) };
  };
  /** Jobs whose delivered results are held by the app (taken, not disposed). */
  const custody = () => [...jobs].filter(([, job]) => job.taken && !job.disposed).map(([id]) => id);
  /** Jobs whose delivered results the app released. */
  const disposed = () => [...jobs].filter(([, job]) => job.disposed).map(([id]) => id);
  /** How many times the session-start release of every result was requested. */
  const disposeAllCount = () => disposeAllCalls;
  /** Jobs held in native custody (not disposed). */
  const nativeHeld = () => [...jobs].filter(([, job]) => job.native && !job.disposed && !job.cancelled).map(([id]) => id);
  /** Every command the transport answered, in order. */
  const commands = () => [...calls];
  /** A native-custody job's held bytes while it is held (the native persistence fake reads them). */
  const heldBytes = (jobId: string) => {
    const job = jobs.get(jobId);
    return job && job.native && !job.disposed && !job.cancelled ? job.bytes : null;
  };
  return { invoke, handles: (command: string) => handles(command) || command === DISPOSE_ALL_COMMAND, custody, disposed, disposeAllCount, nativeHeld, commands, heldBytes };
}

/** A bounded invokeMock implementation for the captured model and two modes,
 * speaking the job contract. A caller may wrap this invoke to delay delivery or
 * simulate failures. Unknown commands/jobs/models/modes fail rather than
 * silently replaying another source. */
export function createNativeMechanicsReplay(options: { jobIdPrefix?: string; profile?: NativeMechanicsReplayProfile } = {}) {
  const profile = options.profile ?? "pressure";
  nativeMechanicsReplayPair("sparse_interactive", { profile });
  const transport = createSolveJobTransport((request) => {
    replayMode(request.mode);
    const pair = nativeMechanicsReplayPair(request.mode, { profile });
    if (!isDeepStrictEqual(request.model, pair.model)) throw new Error("NATIVE_REPLAY_MODEL_MISMATCH");
    return pair.bytes;
  }, { jobIdPrefix: options.jobIdPrefix ?? `unit-native-replay:${++instanceSequence}` });
  const invoke = async (command: string, args?: unknown, invokeOptions?: unknown): Promise<any> => {
    if (command === "load_preview_model") {
      if (args !== undefined) throw new Error("NATIVE_REPLAY_REQUEST_INVALID");
      return nativeMechanicsReplayPair("sparse_interactive", { profile }).model;
    }
    return transport.invoke(command, args, invokeOptions);
  };
  return { invoke, notice: NATIVE_MECHANICS_REPLAY_NOTICE, custody: transport.custody, disposed: transport.disposed, disposeAllCount: transport.disposeAllCount, nativeHeld: transport.nativeHeld, commands: transport.commands, heldBytes: transport.heldBytes };
}

/** Every command a native mechanics replay answers (the job contract, the C1 native result
 * commands and the bundled model), for tests that route commands to it. */
export const REPLAY_COMMANDS: readonly string[] = Object.freeze(["load_preview_model", ...SOLVE_JOB_COMMANDS]);
