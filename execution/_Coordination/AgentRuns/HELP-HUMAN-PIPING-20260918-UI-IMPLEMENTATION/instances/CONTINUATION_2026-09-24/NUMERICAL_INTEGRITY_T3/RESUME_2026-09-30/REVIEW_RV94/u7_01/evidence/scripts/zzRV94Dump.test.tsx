/** RV94 scratch-only dumper (never committed). Reads RV94_INPUTS (JSONL) and writes RV94_TS_OUT.
 * (1) the reader; (2) the standing seam: a registration of these bytes with the record's
 * invocation, read with a model carrying the requested load cases, with a stand-in live
 * capture that holds only for that model object, and again with no live capture;
 * (3) mocked-IPC deliveries (direct and job) of the live milestone successors and of the
 * 07j not_required entry, with RV94's hostile TS variants. */
import { describe, expect, it, vi } from "vitest";
import { readFileSync, writeFileSync } from "node:fs";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { numericalResultStanding, hasCurrentSourceContract } from "./numericalResultQuality";
import { validateRetainedPrecision } from "./retainedPrecision";
import { classificationSummary, registerRetainedPrecision, retainedPrecisionInvocation, retainedPrecisionStanding } from "./retainedPrecisionStanding";
import { hasNativeMechanicsInvocation, pollPreviewMechanicsJob, runPreviewMechanics, startPreviewMechanicsJob } from "../../services/previewService";
import * as SN from "../stress-neutral/StressNeutralExportPanel";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const clone = <T,>(v: T): T => structuredClone(v);
const withheld = (s: Json[]) => s.map(x => x.withheld);

async function reader(source: Json, inv: Json) {
  try {
    const v = await validateRetainedPrecision(clone(source), inv == null ? undefined : clone(inv));
    const counts: Record<string, number> = {};
    for (const c of v.classifications) counts[c.class] = (counts[c.class] ?? 0) + 1;
    return { ok: true, invocation_bound: v.invocation_bound, numerical_eligible: v.numerical_eligible, standing: v.standing, publication_sha256: v.publication_sha256, class_counts: counts };
  } catch (e) { return { ok: false, gate: (e as Json).gate, code: (e as Json).message }; }
}
async function seam(source: Json, inv: Json, model: Json, live: boolean) {
  const s = clone(source) as MechanicsResult;
  await registerRetainedPrecision(s, inv == null ? undefined : clone(inv), (live ? ((m: unknown) => m === model) : null) as Json).catch(() => undefined);
  const st = retainedPrecisionStanding(s, model);
  return { standing: st.standing, eligible: st.eligible, findings: st.findings, status: numericalResultStanding(s, model).status, summary: withheld(classificationSummary(s, model)) };
}
async function deliverDirect(source: Json, model: Json, mode: string): Promise<MechanicsResult> {
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async () => clone(source));
  return runPreviewMechanics(model, mode as Json);
}
let job = 0;
async function deliverJob(source: Json, model: Json, mode: string): Promise<MechanicsResult> {
  (window as Json).__TAURI_INTERNALS__ = {};
  const jobId = `rv94:${++job}`;
  invokeMock.mockImplementation(async (command: string) => command === "start_preview_mechanics_job_with_solver_mode"
    ? { job_id: jobId, backend_cancellation_token: `${jobId}:t`, state: "queued", cancellation_scope: "rv94" }
    : { job_id: jobId, state: "completed", cancellation_requested: false, cancellation_status: "not_requested", cancellation_scope: "rv94", result: clone(source), error_message: null });
  await startPreviewMechanicsJob(model, mode as Json);
  return (await pollPreviewMechanicsJob(jobId)).result!;
}
function view(received: MechanicsResult, model: Json) {
  const st = retainedPrecisionStanding(received, model);
  const inv = retainedPrecisionInvocation(received, model);
  const binding = typeof (SN as Json).liveStressBinding === "function" ? (SN as Json).liveStressBinding(model, received, {}) : "n/a";
  return { standing: st.standing, findings: st.findings, status: numericalResultStanding(received, model).status, summary: withheld(classificationSummary(received, model)),
    invocation: inv === null ? null : "present", live: hasNativeMechanicsInvocation(received, model), current_contract: hasCurrentSourceContract(received), stress_binding: binding === null ? null : binding === "n/a" ? "n/a" : "OPEN" };
}

describe("rv94", () => {
  it("dumps", async () => {
    const out: string[] = [];
    const lines = readFileSync(process.env.RV94_INPUTS!, "utf8").split("\n").filter(Boolean);
    for (const line of lines) {
      const r = JSON.parse(line);
      const rec: Json = { id: r.id, reader: await reader(r.source, r.invocation) };
      if (r.requested.every((x: Json) => x.ref_type === "load_case")) {
        const invRefs = (r.invocation?.request?.model?.load_cases ?? []).map((c: Json) => c.id);
        const ids = r.requested.map((x: Json) => x.ref_id);
        const model = r.invocation && JSON.stringify(invRefs) === JSON.stringify(ids) ? clone(r.invocation.request.model) : { load_cases: ids.map((id: string) => ({ id })) };
        rec.seam_live = await seam(r.source, r.invocation, model, true);
        rec.seam_nolive = await seam(r.source, r.invocation, model, false);
      } else rec.seam = "inexpressible";
      out.push(JSON.stringify(rec));
    }
    // IPC scenarios on the live successors and on 07j.
    const ipc: Json[] = [];
    const byId = new Map(lines.map(l => { const r = JSON.parse(l); return [r.id, r]; }));
    const scen = [["ms:sparse_interactive:inv", "sparse_interactive"], ["ms:dense_scrutiny:inv", "dense_scrutiny"], ["mp:not_required_second_case_checks_passed", null], ["nr:desktop_shaped_invocation", null]] as const;
    for (const [id, m] of scen) {
      const r: Json = byId.get(id); const mode = m ?? r.invocation.solver_mode; const model = clone(r.invocation.request.model);
      const capturedInvocationEqualsRecord = JSON.stringify({ request: { model, materials: [] }, solver_mode: mode }) === JSON.stringify(r.invocation);
      const a = await deliverDirect(r.source, model, mode);
      ipc.push({ id, scenario: "direct:captured_model", captured_equals_record_invocation: capturedInvocationEqualsRecord, ...view(a, model) });
      ipc.push({ id, scenario: "direct:content_equal_clone_model", ...view(a, clone(model)) });
      const stale = clone(model); stale.nodes[0].position.x = (stale.nodes[0].position.x ?? 0) + 1;
      ipc.push({ id, scenario: "direct:stale_model_same_case_ids", ...view(a, stale) });
      ipc.push({ id, scenario: "direct:case_ids_only_model", ...view(a, { load_cases: model.load_cases.map((c: Json) => ({ id: c.id })) }) });
      const other = clone(model); other.load_cases = [...other.load_cases, { ...other.load_cases[0], id: "rv94-extra" }];
      ipc.push({ id, scenario: "direct:extra_case_model", ...view(a, other) });
      const b = await deliverJob(r.source, model, mode);
      ipc.push({ id, scenario: "job:captured_model", ...view(b, model) });
      // the caller's model object mutated after capture voids the capture
      const callerModel = clone(model);
      const c = await deliverDirect(r.source, callerModel, mode);
      const before = view(c, callerModel);
      callerModel.nodes[0].position.x = (callerModel.nodes[0].position.x ?? 0) + 1;
      ipc.push({ id, scenario: "direct:caller_model_mutated_after_capture", before: before.standing, ...view(c, model) });
      // the received bytes mutated after registration
      const d = await deliverDirect(r.source, model, mode);
      (d as Json).run_id = `${(d as Json).run_id}-rv94`;
      ipc.push({ id, scenario: "direct:bytes_mutated_after_registration", ...view(d, model) });
      // no capture (browser-style delivery), then the product registration with the actual invocation and no live capture
      const e = await deliverDirect(r.source, null, mode);
      await registerRetainedPrecision(e, clone(r.invocation)).catch(() => undefined);
      ipc.push({ id, scenario: "nocapture:registered_without_live", ...view(e, model) });
    }
    writeFileSync(process.env.RV94_TS_OUT!, out.join("\n") + "\n");
    writeFileSync(process.env.RV94_TS_OUT! + ".ipc.jsonl", ipc.map(x => JSON.stringify(x)).join("\n") + "\n");
    expect(out.length).toBe(lines.length);
  }, 1_800_000);
});
