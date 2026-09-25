/** Unit transport replay only. These are actual producer input/output bytes,
 * replayed through mocked IPC; this helper is NOT a native UI qualification.
 * It never installs a runtime, registers a source, or edits producer evidence. */
import { isDeepStrictEqual } from "node:util";
import modelFixture from "../../../../fixtures/model_operations/exact_pressure_authoring_model.json";
import sparseFixture from "../../../../fixtures/results/physics_connected_ui_mechanics_sparse.json";
import denseFixture from "../../../../fixtures/results/physics_connected_ui_mechanics_dense.json";
import precisionModelFixture from "../../../../fixtures/model_operations/precision_connected_ui_model.json";
import precisionSparseFixture from "../../../../fixtures/results/precision_connected_ui_mechanics_sparse.json";
import precisionDenseFixture from "../../../../fixtures/results/precision_connected_ui_mechanics_dense.json";
import type { MechanicsResult, PreviewModel } from "../types";
import type { PreviewSolverMode } from "../services/previewService";

export const NATIVE_MECHANICS_REPLAY_NOTICE = "Unit transport replay of captured producer bytes; NOT actual native UI qualification";
export type NativeMechanicsReplayProfile = "physics" | "precision";
const captureRef = "execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/ENGINE_INTEGRATION/UI_PRODUCER_CAPTURE.json";
function replayMode(mode: unknown): asserts mode is PreviewSolverMode {
  if (mode !== "sparse_interactive" && mode !== "dense_scrutiny") throw new Error("NATIVE_REPLAY_MODE_UNSUPPORTED");
}
export function nativeMechanicsReplayPair(mode: PreviewSolverMode = "sparse_interactive", options: { profile?: NativeMechanicsReplayProfile } = {}) {
  replayMode(mode);
  const profile = options.profile ?? "physics";
  if (profile !== "physics" && profile !== "precision") throw new Error("NATIVE_REPLAY_PROFILE_UNSUPPORTED");
  const model = profile === "physics" ? modelFixture : precisionModelFixture;
  const source = profile === "physics" ? (mode === "sparse_interactive" ? sparseFixture : denseFixture) : (mode === "sparse_interactive" ? precisionSparseFixture : precisionDenseFixture);
  return {
    model: structuredClone(model) as PreviewModel,
    source: structuredClone(source) as unknown as MechanicsResult,
    mode,
    profile,
    provenance: {
      kind: "unit_transport_replay" as const,
      actual_native_ui_qualification: false as const,
      notice: NATIVE_MECHANICS_REPLAY_NOTICE,
      capture_ref: profile === "physics" ? captureRef : captureRef.replace("UI_PRODUCER_CAPTURE.json", "PRECISION_UI_COMPANION.json"),
      model_fixture_ref: profile === "physics" ? "fixtures/model_operations/exact_pressure_authoring_model.json" : "fixtures/model_operations/precision_connected_ui_model.json",
      source_fixture_ref: `fixtures/results/${profile}_connected_ui_mechanics_${mode === "sparse_interactive" ? "sparse" : "dense"}.json`,
    },
  };
}

let instanceSequence = 0;
/** A bounded invokeMock implementation for the captured model and two modes.
 * A caller may wrap this invoke to delay delivery or simulate failures. Unknown
 * commands/jobs/models/modes fail rather than silently replaying another source. */
export function createNativeMechanicsReplay(options: { jobIdPrefix?: string; profile?: NativeMechanicsReplayProfile } = {}) {
  const profile = options.profile ?? "physics";
  nativeMechanicsReplayPair("sparse_interactive", { profile });
  const prefix = options.jobIdPrefix ?? `unit-native-replay:${++instanceSequence}`;
  const jobs = new Map<string, { source: MechanicsResult; cancelled: boolean }>();
  let sequence = 0;
  const requestPair = (args: unknown) => {
    if (!args || typeof args !== "object" || Array.isArray(args)) throw new Error("NATIVE_REPLAY_REQUEST_INVALID");
    const request = args as Record<string, unknown>;
    if (Object.keys(request).length !== 2 || !Object.hasOwn(request, "model") || !Object.hasOwn(request, "solverMode")) throw new Error("NATIVE_REPLAY_REQUEST_INVALID");
    replayMode(request.solverMode);
    const pair = nativeMechanicsReplayPair(request.solverMode, { profile });
    if (!isDeepStrictEqual(request.model, pair.model)) throw new Error("NATIVE_REPLAY_MODEL_MISMATCH");
    return pair;
  };
  const invoke = async (command: string, args?: unknown): Promise<any> => {
    if (command === "load_preview_model") {
      if (args !== undefined) throw new Error("NATIVE_REPLAY_REQUEST_INVALID");
      return nativeMechanicsReplayPair("sparse_interactive", { profile }).model;
    }
    if (command === "run_preview_mechanics_with_solver_mode") return requestPair(args).source;
    if (command === "start_preview_mechanics_job_with_solver_mode") {
      const pair = requestPair(args), job_id = `${prefix}:${++sequence}`;
      jobs.set(job_id, { source: pair.source, cancelled: false });
      return { job_id, backend_cancellation_token: `${job_id}:token`, state: "queued", cancellation_scope: "unit_transport_replay_not_native_ui_qualification" };
    }
    if (command === "poll_preview_mechanics_job" || command === "cancel_preview_mechanics_job") {
      const request = args as { jobId?: unknown; cancellationToken?: unknown } | undefined;
      const job = typeof request?.jobId === "string" ? jobs.get(request.jobId) : undefined;
      if (!job) throw new Error("NATIVE_REPLAY_JOB_UNKNOWN");
      const jobId = request!.jobId as string;
      if (command === "cancel_preview_mechanics_job") {
        if (request!.cancellationToken !== `${jobId}:token`) throw new Error("NATIVE_REPLAY_CANCELLATION_TOKEN_MISMATCH");
        job.cancelled = true;
        return { job_id: jobId, accepted: true, cancellation_status: "cancelled", job_state: "cancelled", cancellation_scope: "unit_transport_replay_not_native_ui_qualification", cancellation_success_claimed: false };
      }
      return { job_id: jobId, state: job.cancelled ? "cancelled" : "completed", cancellation_requested: job.cancelled,
        cancellation_status: job.cancelled ? "cancelled" : "not_requested", cancellation_scope: "unit_transport_replay_not_native_ui_qualification",
        result: job.cancelled ? null : structuredClone(job.source), error_message: null };
    }
    throw new Error(`NATIVE_REPLAY_COMMAND_UNSUPPORTED: ${command}`);
  };
  return { invoke, notice: NATIVE_MECHANICS_REPLAY_NOTICE };
}
