/** RV91 probes, part 2 (scratch; copied into the candidate lane, run, removed).
 * - TS per-row binding refusal and classification summary on both milestones,
 *   registered and unregistered, exported for comparison with Python U6b;
 * - registration through mocked job IPC;
 * - a cancellation accepted while the reader awaits (registration ordering). */
import { afterAll, afterEach, describe, expect, it, vi } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
const u7 = vi.hoisted(() => ({ simulate: false }));
vi.mock("./retainedPrecision", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./retainedPrecision")>();
  return { ...actual, validateRetainedPrecision: async (s: unknown, i?: unknown) => { const v = await actual.validateRetainedPrecision(s, i); return u7.simulate && v.invocation_bound ? Object.freeze({ ...v, numerical_eligible: true, standing: "eligible" as const }) : v; } };
});
import * as nrq from "./numericalResultQuality";
import * as rps from "./retainedPrecisionStanding";
import * as ksl from "./knownSemanticLimitations";
import * as ps from "../../services/previewService";
import * as rcs from "../../services/ruleCheckService";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const OUT = process.env.RV91_REVIEW_OUT!;
const root = resolve(__dirname, "../../../../../");
const obs: Record<string, unknown> = {};
afterAll(() => writeFileSync(OUT, JSON.stringify(obs, null, 1)));
afterEach(() => { invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__; u7.simulate = false; });
const pins: Record<string, string> = { sparse_interactive: "ac6986b0680e0df9d88c33a5bf4635372fc3b83cbb9080da44e6d32dbdca59dc", dense_scrutiny: "6cd1d249e5352aaffbd2b7d7349c74a1d0e0572df35500f66be49c3cad95c9b5" };
function fixture(mode: string) {
  const bytes = readFileSync(resolve(root, `fixtures/results/retained_precision_milestone_successor_${mode}.json`));
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(pins[mode]);
  const doc = JSON.parse(bytes.toString("utf8"));
  return { source: doc.source as Json, invocation: doc.invocation as Json };
}

describe("RV91 part 2", () => {
  it.each(["sparse_interactive", "dense_scrutiny"])("%s: binding and summary, registered (job IPC) and unregistered", async (mode) => {
    const { source, invocation } = fixture(mode);
    const model = invocation.request.model;
    (window as Json).__TAURI_INTERNALS__ = {};
    const jobId = `rv91:${mode}`;
    invokeMock.mockImplementation(async (command: string) => {
      if (command === "start_preview_mechanics_job_with_solver_mode") return { job_id: jobId, backend_cancellation_token: "t", state: "queued", cancellation_scope: "rv91" };
      if (command === "poll_preview_mechanics_job") return { job_id: jobId, state: "completed", cancellation_requested: false, cancellation_status: "not_requested", cancellation_scope: "rv91", result: structuredClone(source), error_message: null };
      throw new Error(command);
    });
    await ps.startPreviewMechanicsJob(model, mode as Json);
    const got = (await ps.pollPreviewMechanicsJob(jobId)).result!;
    expect(ps.hasNativeMechanicsInvocation(got, model, mode)).toBe(true);
    expect(rps.retainedPrecisionRegistration(got)?.validation?.invocation_bound).toBe(true);
    const unregistered = structuredClone(source);
    obs[mode] = {
      binding_registered: Object.fromEntries(got.results.map((r: Json) => [r.id, ksl.ruleBindingRefusal(got, r)])),
      binding_unregistered: Object.fromEntries(unregistered.results.map((r: Json) => [r.id, ksl.ruleBindingRefusal(unregistered, r)])),
      summary_registered: rps.classificationSummary(got, model),
      summary_unregistered: rps.classificationSummary(unregistered, model),
      standing_job: nrq.numericalResultStanding(got, model),
    };
  });

  it("a cancellation accepted while the reader awaits: native registration void, retained registration kept", async () => {
    const mode = "sparse_interactive";
    const { source, invocation } = fixture(mode);
    const model = invocation.request.model;
    (window as Json).__TAURI_INTERNALS__ = {};
    const jobId = "rv91:cancel";
    let release: () => void = () => {};
    const gate = new Promise<void>(r => { release = r; });
    invokeMock.mockImplementation(async (command: string) => {
      if (command === "start_preview_mechanics_job_with_solver_mode") return { job_id: jobId, backend_cancellation_token: "t", state: "queued", cancellation_scope: "rv91" };
      if (command === "poll_preview_mechanics_job") return { job_id: jobId, state: "completed", cancellation_requested: false, cancellation_status: "not_requested", cancellation_scope: "rv91", result: structuredClone(source), error_message: null };
      if (command === "cancel_preview_mechanics_job") { await gate; return { job_id: jobId, accepted: true, cancellation_status: "requested", job_state: "completed", cancellation_scope: "rv91", cancellation_success_claimed: false }; }
      throw new Error(command);
    });
    u7.simulate = true;
    await ps.startPreviewMechanicsJob(model, mode);
    const polling = ps.pollPreviewMechanicsJob(jobId);
    const cancelling = ps.cancelPreviewMechanicsJob(jobId, "t");
    // Let the poll reach validation (the reader awaits), then accept the cancellation.
    await new Promise(r => setTimeout(r, 0));
    release();
    await cancelling;
    const got = (await polling).result!;
    const outcome = {
      native: ps.hasNativeMechanicsInvocation(got, model, mode),
      retained_registered: rps.retainedPrecisionRegistration(got) !== null,
      standing_post_u7: nrq.numericalResultStanding(got, model),
      rule_gate: await rcs.runRuleChecks({ rulePackDocument: {} as Json, model, solvedEnvelope: got }).then(() => "ran", e => (e as Error).message.split(":")[0]),
    };
    obs.cancel_during_validation = outcome;
  });
});
