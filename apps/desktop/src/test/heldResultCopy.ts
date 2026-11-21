// A solve held natively, as the session holds its own (C1 custody), and the C2 full copy of its
// exact bytes, registered to the native job that holds them (`loadNativeFullResult`). A rule check
// on that copy sends the job's handle, which the native command reads; a result taken into the
// webview is no longer held natively, and the native command refuses it. Through the caller's
// mocked transport; unit transport replay, NOT native UI qualification.
import {
  awaitPreviewMechanicsJob,
  loadNativeFullResult,
  readNativeFreshResult,
  startPreviewMechanicsJob,
  type PreviewSolverMode
} from "../services/previewService";
import type { MechanicsResult, PreviewModel } from "../types";

export async function solveHeldCopy(model: PreviewModel, mode: PreviewSolverMode = "sparse_interactive"): Promise<MechanicsResult> {
  const started = await startPreviewMechanicsJob(model, mode, { custody: "native" });
  if (started.mode !== "backend_job") throw new Error("not a backend job");
  const terminal = await awaitPreviewMechanicsJob(started.job_id);
  if (terminal.state !== "completed") throw new Error(`the solve did not complete: ${terminal.state}`);
  const { result } = await readNativeFreshResult(started.job_id, terminal.result_bytes ?? 0, { standingModel: model, summaryModel: model });
  return loadNativeFullResult(result);
}
