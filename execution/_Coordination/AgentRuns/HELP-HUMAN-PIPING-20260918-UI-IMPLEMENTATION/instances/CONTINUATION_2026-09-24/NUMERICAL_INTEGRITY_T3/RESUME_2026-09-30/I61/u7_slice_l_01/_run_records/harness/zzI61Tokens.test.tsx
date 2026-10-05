/** I61 U7 slice L (scratch lane only): TypeScript standing tokens on the live successors
 * (PP's actual Direct entry's bytes), delivered through the mocked direct and job IPC with a
 * live capture, as the product's preview service receives them. No reader is wrapped. */
import { describe, expect, it, vi } from "vitest";
import { readFileSync, writeFileSync } from "node:fs";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { numericalResultStanding } from "./numericalResultQuality";
import { validateRetainedPrecision } from "./retainedPrecision";
import { classificationSummary, registerRetainedPrecision, retainedPrecisionStanding } from "./retainedPrecisionStanding";
import { hasNativeMechanicsInvocation, pollPreviewMechanicsJob, runPreviewMechanics, startPreviewMechanicsJob, type PreviewSolverMode } from "../../services/previewService";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const live = process.env.I61_LIVE, out = process.env.I61_OUT;
let seq = 0;
async function deliverDirect(source: MechanicsResult, model: PreviewModel, mode: PreviewSolverMode) {
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async (command: string, args: Json) => {
    expect(command).toBe("run_preview_mechanics_with_solver_mode"); expect(args.solverMode).toBe(mode);
    return structuredClone(source);
  });
  return runPreviewMechanics(model, mode);
}
async function deliverJob(source: MechanicsResult, model: PreviewModel, mode: PreviewSolverMode) {
  (window as Json).__TAURI_INTERNALS__ = {};
  const jobId = `i61-u7l:${++seq}`;
  invokeMock.mockImplementation(async (command: string, args: Json) => {
    if (command === "start_preview_mechanics_job_with_solver_mode") { expect(args.solverMode).toBe(mode); return { job_id: jobId, backend_cancellation_token: `${jobId}:t`, state: "queued", cancellation_scope: "unit_transport_replay_not_native_ui_qualification" }; }
    expect(command).toBe("poll_preview_mechanics_job");
    return { job_id: jobId, state: "completed", cancellation_requested: false, cancellation_status: "not_requested", cancellation_scope: "unit_transport_replay_not_native_ui_qualification", result: structuredClone(source), error_message: null };
  });
  await startPreviewMechanicsJob(model, mode);
  return (await pollPreviewMechanicsJob(jobId)).result!;
}
const token = (s: MechanicsResult, m: PreviewModel) => { const r = retainedPrecisionStanding(s, m); return { token: r.standing, findings: r.findings, status: numericalResultStanding(s, m).status }; };

describe.skipIf(!live || !out)("I61 U7 slice L tokens (TypeScript)", () => {
  it("records the standing tokens on the live bytes", async () => {
    const all: Json = {};
    for (const mode of ["sparse_interactive", "dense_scrutiny"] as const) {
      const doc = JSON.parse(readFileSync(`${live}/u3g2_successor_${mode}.json`, "utf8"));
      const source = doc.source as MechanicsResult, inv = doc.invocation, model = inv.request.model as PreviewModel;
      const reader = await validateRetainedPrecision(structuredClone(source), structuredClone(inv));
      const reader0 = await validateRetainedPrecision(structuredClone(source));
      // With the invocation: the live capture through direct and job IPC.
      const direct = await deliverDirect(source, model, mode);
      const job = await deliverJob(source, model, mode);
      // Without it: the product registration of these bytes with no invocation.
      const own = structuredClone(source); await registerRetainedPrecision(own, undefined);
      // D-U7-4 (a): the actual invocation registered with no IPC capture.
      const nocap = structuredClone(source); await registerRetainedPrecision(nocap, structuredClone(inv));
      // D-U7-4 (b): the live capture, read against a stale current model with the same case ids.
      const stale = structuredClone(model) as Json; stale.nodes[0].position.x = 1.0;
      all[mode] = {
        reader: { with_invocation: { numerical_eligible: reader.numerical_eligible, standing: reader.standing }, without_invocation: { numerical_eligible: reader0.numerical_eligible, standing: reader0.standing } },
        native_capture: { direct: hasNativeMechanicsInvocation(direct, model, mode), job: hasNativeMechanicsInvocation(job, model, mode), stale_model: hasNativeMechanicsInvocation(direct, stale, mode) },
        token: {
          with_invocation: token(direct, model), with_invocation_job: token(job, model),
          without_invocation: token(own, model),
          d_u7_4_no_native_capture: token(nocap, model),
          d_u7_4_stale_current_model: token(direct, stale),
        },
        withheld: { with_invocation: classificationSummary(direct, model).map(s => s.withheld), without_invocation: classificationSummary(own, model).map(s => s.withheld),
          d_u7_4_no_native_capture: classificationSummary(nocap, model).map(s => s.withheld), d_u7_4_stale_current_model: classificationSummary(direct, stale).map(s => s.withheld) },
      };
    }
    writeFileSync(out!, JSON.stringify(all, null, 1));
  }, 600_000);
});
