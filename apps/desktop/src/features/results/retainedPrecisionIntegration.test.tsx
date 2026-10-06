/** U6d: TypeScript carriers and standing for the F2a preview successor
 * (plan 1d and 3; D2 4.7, 4.9.4 and 4.9.9).
 *
 * Inputs are PP's pinned milestone successor bytes (D-U6-5; checked by sha256)
 * with their invocation, delivered through mocked direct and job IPC. This is a
 * unit transport replay, NOT a native witness: Tauri never delivers a successor
 * in the milestone domain (plan F-1; D-U6-3 qualification limit).
 *
 * Since U7 (D-U7-5) the reader's eligibility is on: the unedited successor with
 * its captured invocation reads `numerically_eligible` through the real reader.
 * The pure seams (`retainedStandingFrom`, `classificationSummaryFrom`) are
 * exercised directly. A test-only wrapper of the accepted reader has two uses:
 * `u7.simulate` marks one row `not_covered` (no available statement has one;
 * U6a F3) or reclassifies the absolute rows, to reach the summary branches; and
 * `u7.held` returns the reader's validation as it was before U7 (never eligible,
 * standing `needs_recompute`, every gate and class unchanged), so the carriers'
 * and gates' not-eligible branches stay exercised. With both false the wrapper
 * returns the reader's own frozen result unchanged.
 */
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, render, screen } from "@testing-library/react";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
const u7 = vi.hoisted(() => ({ simulate: false, held: false, notCovered: null as string | null, noAbsolute: false }));
vi.mock("./retainedPrecision", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./retainedPrecision")>();
  return {
    ...actual,
    validateRetainedPrecision: async (source: unknown, invocation?: unknown) => {
      const validation = await actual.validateRetainedPrecision(source, invocation);
      // The pre-U7 reader (its flag false): the same gates and classes, never eligible.
      if (u7.held) return Object.freeze({ ...validation, numerical_eligible: false, standing: "needs_recompute" as const });
      if (!u7.simulate || !validation.invocation_bound) return validation;
      return Object.freeze({
        ...validation, numerical_eligible: true, standing: "eligible" as const,
        classifications: validation.classifications.map(row => row.result_id === u7.notCovered ? { ...row, class: "not_covered" as const, scale_bits: null, bound_bits: null }
          : u7.noAbsolute && row.class === "absolute_verified" ? { ...row, class: "relative_verified" as const, bound_bits: null } : row),
      });
    },
  };
});
// U7 slice T (RV91 N-5): test-only stand-ins for the two T6 panels' refusal points,
// off by default. `throwless` makes the shared refusal stop throwing inside builders
// while the panels' gate still reads it; `noRefusal` removes it entirely (the
// positive control: what a panel would do without its gate); `builderDoc` makes the
// result-export builder return a document instead of refusing.
const t6 = vi.hoisted(() => ({ throwless: false, noRefusal: false, builderDoc: null as unknown }));
vi.mock("./loadReferenceOutputAvailability", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./loadReferenceOutputAvailability")>();
  return {
    ...actual,
    loadReferenceOutputRefusal: (source: Parameters<typeof actual.loadReferenceOutputRefusal>[0]) => t6.noRefusal ? null : actual.loadReferenceOutputRefusal(source),
    refuseLoadReferenceOutput: (source: Parameters<typeof actual.refuseLoadReferenceOutput>[0]) => t6.noRefusal || t6.throwless ? undefined : actual.refuseLoadReferenceOutput(source),
  };
});
vi.mock("../result-export/resultExportAdapter", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../result-export/resultExportAdapter")>();
  return { ...actual, buildCurrentResultExport: async (args: Parameters<typeof actual.buildCurrentResultExport>[0]) => t6.builderDoc ?? actual.buildCurrentResultExport(args) };
});
import type { LocalProjectEnvelope, MechanicsResult, PreviewModel } from "../../types";
// Import order matters for the wrapper above: the carriers load first, so their
// import of the reader resolves to the wrapped module, not through the cycle.
import {
  PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256, PREVIEW_PHYSICS_RETAINED_PROFILE,
  RETAINED_PRECISION_DOWNGRADE_FORBIDDEN, currentSemanticContract, hasCurrentSourceContract, numericalResultStanding,
  RETAINED_STANDING_STATUS, ordinaryCaseEligible, retainedPrecisionDowngrade, sourceContract, sourceContractTransport, sourceSemanticBinding,
} from "./numericalResultQuality";
import {
  RETAINED_METHOD, RETAINED_PRECISION_ID, RETAINED_PRECISION_PROFILE, decodeBinary64, validateRetainedPrecision, validateRetainedPrecisionTransport,
  type RetainedPrecisionValidation, type RowClassification,
} from "./retainedPrecision";
import {
  RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED, RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE, RETAINED_PRECISION_VALIDATION_REQUIRED, classificationSummary, classificationSummaryFrom,
  registerRetainedPrecision, retainedPrecisionInvocation, retainedPrecisionRegistration, retainedPrecisionStanding, retainedPrecisionStandingText, retainedRowClasses, retainedStandingFrom,
} from "./retainedPrecisionStanding";
import {
  FRESH_SEMANTIC_CONTRACT_IDS, N_HEADLINE, N_RP_ABSOLUTE, N_RP_NOT_COVERED, N_RP_UNVALIDATED, N_SB, RULE_QUANTITY_BELOW_VERIFIED_FLOOR, RULE_QUANTITY_NOT_COVERED,
  classBindingRefusal, isFreshSemanticResult, knownSemanticNotices, resultRowLabel, retainedAbsoluteNotice, ruleBindingRefusal, standingReason, upwardBoundText,
} from "./knownSemanticLimitations";
import { semanticContractForSource, previewPhysicsSemanticContract } from "./resultSemantics";
import { ResultsPanel } from "./ResultsPanel";
import { buildHistoricalRunContext } from "./HistoricalRunContext";
import { hasNativeMechanicsInvocation, pollPreviewMechanicsJob, runPreviewMechanics, startPreviewMechanicsJob, type PreviewSolverMode } from "../../services/previewService";
import { ruleBindingPrecheck, runRuleChecks, type RuleCheckBindingPlan } from "../../services/ruleCheckService";
import { ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH, buildAnalysisRunV03, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import { cancelPreviewMechanicsJob } from "../../services/previewService";
import { ResultExportPanel } from "../result-export/ResultExportPanel";
import { StressNeutralExportPanel, liveStressBinding } from "../stress-neutral/StressNeutralExportPanel";
import { N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE } from "./outputPolicy";
import type { CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import type { RulePackDocument } from "../../services/rulePackService";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const caseFile = JSON.parse(readFileSync(resolve(root, "fixtures/results/retained_precision_carrier_cases.json"), "utf8"));
const MODES = ["sparse_interactive", "dense_scrutiny"] as const;
const sha256 = (bytes: Buffer | string) => createHash("sha256").update(bytes).digest("hex");
/** A shared-file fixture by id, checked by sha256 (D-U6-5). Format v2: a
 * `milestone` file is {id, invocation, source}; a `raw` file is the source itself
 * and has no invocation. */
function sharedFixture(id: string): { source: MechanicsResult; invocation: Json | null; model: PreviewModel | null } {
  const entry = caseFile.fixtures[id];
  const bytes = readFileSync(resolve(root, entry.path));
  expect(sha256(bytes)).toBe(entry.sha256);
  const doc = JSON.parse(bytes.toString("utf8"));
  if (entry.shape === "milestone") return { source: doc.source, invocation: doc.invocation, model: doc.invocation.request.model };
  expect(entry.shape).toBe("raw");
  return { source: doc, invocation: null, model: null };
}
/** The pinned successor and its invocation, checked by sha256 (D-U6-5). */
function milestone(mode: PreviewSolverMode) {
  const { source, invocation, model } = sharedFixture(`milestone_${mode}`);
  return { source, invocation: invocation as Json, model: model! };
}
/** The shared file's closed field sets (v4), as Python's and Rust's FORM_FIELDS:
 * any other field fails. */
const CASE_FIELDS = ["edits", "expected_dispatch", "expected_standing", "fixture", "id", "invocation", "requested"];
const FORM_FIELDS = ["capture", "current_model_edits", "edits", "expected", "fixtures", "invocation", "label", "requested", "subject"];
/** A shared case or declared-difference form applied to a fixture, delivered as
 * TS would receive it: a `"fixture"` invocation (edited if the form says so) is a
 * capture of it through mocked IPC; no invocation is a delivery without a capture;
 * a literal invocation object (v3, such as `{}`), which no IPC capture can carry,
 * is offered to the product registration after a capture-less delivery. The
 * standing model carries the requested load cases. v4: `capture: "none"` delivers
 * the same bytes without a capture and registers them with the fixture invocation
 * and no live native capture; `current_model_edits` set-edit a copy of the
 * invocation's model, which is then the session's current model. */
async function applyShared(c: Json, fixtureId: string) {
  const label = String(c.id ?? c.label);
  const fields = Object.hasOwn(c, "label") ? FORM_FIELDS : CASE_FIELDS;
  expect(Object.keys(c).filter(k => !fields.includes(k)), label).toStrictEqual([]);
  const doc = sharedFixture(fixtureId);
  const literal = c.invocation !== null && c.invocation !== "fixture";
  const source = structuredClone(doc.source) as Json, invocation = structuredClone(literal ? c.invocation : doc.invocation);
  for (const edit of c.edits) {
    expect(edit.op).toBe("set");
    setPath(edit.target === "source" ? source : invocation, edit.path, edit.value);
  }
  if (Object.hasOwn(c, "capture")) expect([c.capture, c.invocation], label).toStrictEqual(["none", "fixture"]);
  const fromFixture = c.invocation === "fixture", captured = fromFixture && !Object.hasOwn(c, "capture");
  const solveMode = (fromFixture ? invocation.solver_mode : doc.invocation?.solver_mode ?? "sparse_interactive") as PreviewSolverMode;
  const received = await deliverDirect(source, captured ? invocation.request.model : null, solveMode);
  if (literal) await registerRetainedPrecision(received, invocation).catch(() => undefined);
  // v4 `capture: "none"`: the product registration with the actual invocation, and no live capture.
  else if (fromFixture && !captured) await registerRetainedPrecision(received, invocation);
  const requestedIds: string[] = c.requested === "invocation" ? caseIds(doc.model!) : c.requested.map((r: Json) => r.ref_id);
  // The session's current model is the invocation's when it comes from the fixture
  // and the requested refs are its cases (U7 slice T binds standing to the captured
  // model); otherwise a model carrying just the requested load cases.
  let model = fromFixture && c.requested === "invocation" ? invocation.request.model as PreviewModel : { load_cases: requestedIds.map(id => ({ id })) } as unknown as PreviewModel;
  if (Object.hasOwn(c, "current_model_edits")) {
    expect(fromFixture && c.requested === "invocation", label).toBe(true);
    const before = model;
    model = structuredClone(model);
    for (const edit of c.current_model_edits as Json[]) {
      expect([Object.keys(edit).sort(), edit.op], label).toStrictEqual([["op", "path", "value"], "set"]);
      let at = model as Json;
      for (const key of edit.path.slice(0, -1)) at = at[key];
      expect(Object.hasOwn(at, edit.path[edit.path.length - 1]), label).toBe(true);
      at[edit.path[edit.path.length - 1]] = edit.value;
    }
    // The edits change the model but keep its load-case ids.
    expect(model, label).not.toStrictEqual(before);
    expect(caseIds(model), label).toStrictEqual(caseIds(before));
  }
  return { received, model };
}
const OTHER: Record<PreviewSolverMode, PreviewSolverMode> = { sparse_interactive: "dense_scrutiny", dense_scrutiny: "sparse_interactive" };
let jobSequence = 0;
/** Mocked direct IPC: the native command returns these bytes for this mode. */
async function deliverDirect(source: MechanicsResult, model: PreviewModel | null, mode: PreviewSolverMode): Promise<MechanicsResult> {
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async (command: string, args: Json) => {
    expect(command).toBe("run_preview_mechanics_with_solver_mode");
    expect(args.solverMode).toBe(mode);
    return structuredClone(source);
  });
  return runPreviewMechanics(model, mode);
}
/** Mocked job IPC: start, then one completed poll delivering these bytes. */
async function deliverJob(source: MechanicsResult, model: PreviewModel, mode: PreviewSolverMode): Promise<MechanicsResult> {
  (window as Json).__TAURI_INTERNALS__ = {};
  const jobId = `unit-transport-replay:retained:${++jobSequence}`;
  invokeMock.mockImplementation(async (command: string, args: Json) => {
    if (command === "start_preview_mechanics_job_with_solver_mode") {
      expect(args.solverMode).toBe(mode);
      return { job_id: jobId, backend_cancellation_token: `${jobId}:token`, state: "queued", cancellation_scope: "unit_transport_replay_not_native_ui_qualification" };
    }
    expect(command).toBe("poll_preview_mechanics_job");
    expect(args.jobId).toBe(jobId);
    return { job_id: jobId, state: "completed", cancellation_requested: false, cancellation_status: "not_requested", cancellation_scope: "unit_transport_replay_not_native_ui_qualification", result: structuredClone(source), error_message: null };
  });
  await startPreviewMechanicsJob(model, mode);
  return (await pollPreviewMechanicsJob(jobId)).result!;
}
function setPath(target: Json, path: (string | number)[], value: unknown) {
  let at = target;
  for (const key of path.slice(0, -1)) at = at[key];
  at[path[path.length - 1]] = value;
}
const caseIds = (model: PreviewModel) => model.load_cases.map(c => c.id);
const withLoadCases = (model: PreviewModel, ids: string[]) => ({ ...model, load_cases: ids.map(id => ({ ...model.load_cases[0], id })) }) as PreviewModel;

beforeEach(() => { u7.simulate = false; u7.held = false; u7.notCovered = null; u7.noAbsolute = false; t6.throwless = false; t6.noRefusal = false; t6.builderDoc = null; });
afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as Json).__TAURI_INTERNALS__;
  u7.simulate = false; u7.held = false; u7.notCovered = null; u7.noAbsolute = false; t6.throwless = false; t6.noRefusal = false; t6.builderDoc = null;
});

describe("the inputs and the pinned identity", () => {
  it("uses PP's byte-identical successors and the shared 20-case file (format v4)", () => {
    expect(caseFile.format).toBe("I66-U6-CARRIER-CASES-v4");
    expect(caseFile.cases).toHaveLength(20);
    expect(Object.keys(caseFile.fixtures).sort()).toStrictEqual(["legacy_preview_0_1", "milestone_dense_scrutiny", "milestone_sparse_interactive", "preview_physics_1_invented_sparse", "source_blocks_n05_sparse"]);
    for (const mode of MODES) expect(milestone(mode).source.producer!.semantic_contract_id).toBe(RETAINED_PRECISION_ID);
  });
  it("the raw fixtures' guard cases are refused only through their edit (as Rust asserts)", () => {
    for (const [id, route] of [["legacy_preview_0_1", "legacy"], ["preview_physics_1_invented_sparse", "preview_physics"], ["source_blocks_n05_sparse", "source_blocks"]] as const) {
      const { source, invocation } = sharedFixture(id);
      expect(invocation).toBeNull();
      expect(sourceContract(source)).toBe(route);
      expect(retainedPrecisionDowngrade(source)).toBe(false);
      expect(numericalResultStanding(source, null).status).toBe("needs_recompute");
    }
  });
  it("dispatch constants equal the accepted reader's and the pinned table, whose rows are preview-physics-1's", () => {
    expect(PREVIEW_PHYSICS_RETAINED_CONTRACT_ID).toBe(RETAINED_PRECISION_ID);
    expect(PREVIEW_PHYSICS_RETAINED_PROFILE).toBe(RETAINED_PRECISION_PROFILE);
    const bytes = readFileSync(resolve(root, "fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json"));
    expect(sha256(bytes)).toBe(PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256);
    const { source } = milestone("sparse_interactive");
    const table = semanticContractForSource(source) as Json;
    expect(table.semantic_contract_id).toBe(RETAINED_PRECISION_ID);
    expect(table.rows).toStrictEqual(previewPhysicsSemanticContract.rows);
    expect(sourceSemanticBinding(source)).toStrictEqual({ id: RETAINED_PRECISION_ID, sha256: PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256 });
    expect(FRESH_SEMANTIC_CONTRACT_IDS).toContain(RETAINED_PRECISION_ID);
  });
});

describe.each(MODES)("%s: registration through mocked IPC", (mode) => {
  it.each([["direct", deliverDirect], ["job", deliverJob]] as const)("%s IPC registers only after the accepted reader passes with the captured invocation", async (_route, deliver) => {
    const { source, invocation, model } = milestone(mode);
    const received = await deliver(source, model, mode);
    expect(received).toStrictEqual(source);
    expect(sourceContract(received)).toBe("retained_preview_physics");
    expect(hasNativeMechanicsInvocation(received, model, mode)).toBe(true);
    const outcome = retainedPrecisionRegistration(received)!;
    expect(outcome.error).toBeNull();
    expect(outcome.validation!.invocation_bound).toBe(true);
    expect(outcome.validation!.numerical_eligible).toBe(true);
    expect(outcome.validation!.classifications).toHaveLength(source.results.length);
    // Synchronous standing reads only the registration: since U7 the live capture with
    // its captured model is eligible, and the rule-check route receives the invocation.
    expect(numericalResultStanding(received, model)).toStrictEqual({ contract: "retained_preview_physics", status: "integrity_checked", eligible: true, findings: [] });
    expect(retainedPrecisionStanding(received, model).standing).toBe("numerically_eligible");
    expect(retainedPrecisionInvocation(received, model)).toStrictEqual(invocation);
    // Fresh by membership (D-U6-6), never by standing.
    expect(isFreshSemanticResult(received)).toBe(true);
    expect(currentSemanticContract(received)).toStrictEqual({ id: RETAINED_PRECISION_ID, sha256: PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256 });
    expect(standingReason(received)).toBeNull();
    // The registration binds the actual invocation, which the reader's G8 checked.
    expect((await validateRetainedPrecision(received, invocation)).invocation_bound).toBe(true);
  });

  it("copies, saved bytes, headers and a model-less solve never register", async () => {
    const { source, model } = milestone(mode);
    const received = await deliverDirect(source, model, mode);
    for (const copy of [structuredClone(received), JSON.parse(JSON.stringify(received)) as MechanicsResult, source]) {
      expect(retainedPrecisionRegistration(copy)).toBeNull();
      expect(numericalResultStanding(copy, model)).toStrictEqual({ contract: "retained_preview_physics", status: "needs_recompute", eligible: false, findings: [RETAINED_PRECISION_VALIDATION_REQUIRED] });
      expect(hasNativeMechanicsInvocation(copy, model, mode)).toBe(false);
    }
    // No model, no captured invocation: delivered bytes are kept but nothing registers.
    const uncaptured = await deliverDirect(source, null, mode);
    expect(retainedPrecisionRegistration(uncaptured)).toBeNull();
    expect(numericalResultStanding(uncaptured, model).findings).toStrictEqual([RETAINED_PRECISION_VALIDATION_REQUIRED]);
    // Browser preview (no Tauri runtime) refuses before any delivery.
    delete (window as Json).__TAURI_INTERNALS__;
    await expect(runPreviewMechanics(model, mode)).rejects.toThrow();
  });

  it("a later edit of the received bytes voids the registration, including a sign of zero", async () => {
    const { source, model } = milestone(mode);
    const received = await deliverDirect(source, model, mode);
    const original = received.results[3].value;
    received.results[3].value = original + 1;
    expect(retainedPrecisionRegistration(received)).toBeNull();
    expect(numericalResultStanding(received, model).findings).toStrictEqual([RETAINED_PRECISION_VALIDATION_REQUIRED]);
    expect(hasNativeMechanicsInvocation(received, model, mode)).toBe(false);
    expect(ruleBindingRefusal(received, received.results[3])).toBe(RULE_QUANTITY_NOT_COVERED);
    received.results[3].value = original;
    expect(retainedPrecisionRegistration(received)!.error).toBeNull();
    const zero = received.results.findIndex(row => Object.is(row.value, 0));
    expect(zero).toBeGreaterThanOrEqual(0);
    received.results[zero].value = -0;
    expect(retainedPrecisionRegistration(received)).toBeNull();
    // Bytes outside the checked profile void it, and so does a receipt edit.
    received.results[zero].value = NaN;
    expect(retainedPrecisionRegistration(received)).toBeNull();
    received.results[zero].value = 0;
    (received.retained_precision as Json).receipt_sha256 = "0".repeat(64);
    expect(retainedPrecisionRegistration(received)).toBeNull();
  });

  it("bytes outside the checked profile never register, even when registration is called directly", async () => {
    const { source, invocation } = milestone(mode);
    const unchecked = structuredClone(source); unchecked.results[3].value = NaN;
    await expect(registerRetainedPrecision(unchecked, invocation)).rejects.toThrow("RETAINED_PRECISION_RECEIPT_MISMATCH");
    expect(retainedPrecisionRegistration(unchecked)).toBeNull();
    unchecked.results[3].value = source.results[3].value;
    expect(retainedPrecisionRegistration(unchecked)).toBeNull();
  });

  it("bytes outside the checked profile never register, even after a later repair", async () => {
    const { source, model } = milestone(mode);
    const unchecked = structuredClone(source); unchecked.results[3].value = NaN;
    const received = await deliverDirect(unchecked, model, mode);
    expect(retainedPrecisionRegistration(received)).toBeNull();
    received.results[3].value = source.results[3].value;
    expect(retainedPrecisionRegistration(received)).toBeNull();
    expect(numericalResultStanding(received, model).findings).toStrictEqual([RETAINED_PRECISION_VALIDATION_REQUIRED]);
    expect(hasNativeMechanicsInvocation(received, model, mode)).toBe(false);
  });

  it.each([
    ["an edited covered row", (s: Json) => { s.results[0].value = 12345; }, "RETAINED_PRECISION_RECEIPT_MISMATCH"],
    ["a hash-consistent-looking quality claim of checks_passed", (s: Json) => { s.numerical_quality.status = "checks_passed"; s.numerical_quality.cases[0].solve_quality = "checks_passed"; }, "RETAINED_PRECISION_RECEIPT_MISMATCH"],
    ["an altered receipt", (s: Json) => { s.retained_precision.body.receipt_version = 1; s.retained_precision.receipt_sha256 = "f".repeat(64); }, "RETAINED_PRECISION_RECEIPT_MISMATCH"],
  ] as const)("a delivery with %s records the reader's first code: unsupported, never registered natively", async (_label, edit, code) => {
    const { source, model } = milestone(mode);
    const edited = structuredClone(source); edit(edited);
    const received = await deliverDirect(edited, model, mode);
    expect(retainedPrecisionRegistration(received)).toStrictEqual({ validation: null, error: code });
    expect(retainedPrecisionStanding(received, model)).toStrictEqual({ standing: "unsupported", eligible: false, findings: [code] });
    expect(numericalResultStanding(received, model)).toStrictEqual({ contract: "retained_preview_physics", status: "needs_recompute", eligible: false, findings: [code] });
    expect(hasNativeMechanicsInvocation(received, model, mode)).toBe(false);
    expect(retainedRowClasses(received)).toBeNull();
  });

  it("numerical_quality never contributes, even with the post-U7 reader and a fully passing ordinary claim", async () => {
    u7.simulate = true;
    const { source, model } = milestone(mode);
    const claimed = structuredClone(source) as Json;
    claimed.numerical_quality.status = "checks_passed";
    claimed.numerical_quality.cases[0].solve_quality = "checks_passed";
    // The generic branch would accept this ordinary claim; the successor branch refuses it at G1.
    expect(claimed.numerical_quality.cases.every((c: Json) => ordinaryCaseEligible(c, new Set([...claimed.results, ...claimed.diagnostics].map((x: Json) => x.id))))).toBe(true);
    const received = await deliverDirect(claimed, model, mode);
    expect(numericalResultStanding(received, model)).toStrictEqual({ contract: "retained_preview_physics", status: "needs_recompute", eligible: false, findings: ["RETAINED_PRECISION_RECEIPT_MISMATCH"] });
  });

  it("a registration without an invocation is never eligible, even with the post-U7 reader, and says so", async () => {
    u7.simulate = true;
    const { source, model } = milestone(mode);
    const own = structuredClone(source);
    const validation = await registerRetainedPrecision(own, undefined);
    expect(validation.invocation_bound).toBe(false);
    expect(retainedPrecisionStanding(own, model)).toStrictEqual({ standing: "needs_recompute", eligible: false, findings: [RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE] });
    expect(retainedPrecisionStandingText(own)).toContain("validated by the retained-precision reader without an invocation");
    expect(retainedPrecisionInvocation(own, model)).toBeNull();
  });

  it("registration binds the bytes captured before the reader's await: an edit made while it awaits never registers (RV91 N-3, RV09)", async () => {
    const { source, invocation } = milestone(mode);
    const fresh = structuredClone(source);
    const pending = registerRetainedPrecision(fresh, structuredClone(invocation));
    const original = fresh.results[3].value;
    fresh.results[3].value = original + 1; // edited while the reader awaits
    // The reader validates the bytes it snapshotted synchronously.
    expect((await pending).invocation_bound).toBe(true);
    expect(retainedPrecisionRegistration(fresh)).toBeNull();
    expect(numericalResultStanding(fresh, null).findings).toStrictEqual([RETAINED_PRECISION_VALIDATION_REQUIRED]);
    fresh.results[3].value = original;
    // The validation belongs to exactly these (reverted) bytes.
    expect(retainedPrecisionRegistration(fresh)?.validation?.invocation_bound).toBe(true);
  });

  it("the registered invocation is a private copy: neither the caller's object nor a returned copy can alter it (RV03, RV04)", async () => {
    const { source, invocation, model } = milestone(mode);
    const own = structuredClone(source), callers = structuredClone(invocation);
    // A live-capture predicate stands in for previewService's native capture (U7 slice T).
    await registerRetainedPrecision(own, callers, () => true);
    callers.solver_mode = OTHER[mode];
    const first = retainedPrecisionInvocation(own, model) as Json;
    expect(first).toStrictEqual(invocation);
    first.solver_mode = OTHER[mode];
    expect(retainedPrecisionInvocation(own, model)).toStrictEqual(invocation);
  });

  it("a foreign solver mode is refused at the reader's invocation binding", async () => {
    const { source, model } = milestone(mode);
    const received = await deliverDirect(source, model, OTHER[mode]);
    expect(retainedPrecisionRegistration(received)).toStrictEqual({ validation: null, error: "RETAINED_PRECISION_INVOCATION_MISMATCH" });
    expect(retainedPrecisionStanding(received, model).standing).toBe("unsupported");
    expect(hasNativeMechanicsInvocation(received, model)).toBe(false);
  });
});

describe("the post-U7 standing rules (seams, independent of the reader's flag)", () => {
  const { source } = milestone("sparse_interactive");
  const requested = [{ ref_type: "load_case", ref_id: "case" }];
  const bound = { invocation_bound: true, numerical_eligible: true };
  /** A two-case statement: the milestone's selected case plus a not_required one. */
  function twoCase(edit?: (s: Json) => void): MechanicsResult {
    const s = structuredClone(source) as Json;
    s.retained_precision.body.cases.push({ basis_ref: { ref_type: "load_case", ref_id: "second" }, status: "not_required" });
    s.numerical_quality.cases.push({ basis_ref: { ref_type: "load_case", ref_id: "second" }, structural_status: "passive_model_basis", solve_quality: "checks_passed", model_matrix_fidelity: "represented_equations_retained", accuracy_evidence: "not_claimed", evidence_refs: [s.diagnostics[0].id] });
    edit?.(s);
    return s;
  }
  const two = [...requested, { ref_type: "load_case", ref_id: "second" }];
  it("is numerically_eligible only with every conjunct", () => {
    expect(retainedStandingFrom(bound, source, requested)).toBe("numerically_eligible");
    expect(retainedStandingFrom({ ...bound, invocation_bound: false }, source, requested)).toBe("needs_recompute");
    expect(retainedStandingFrom({ ...bound, numerical_eligible: false }, source, requested)).toBe("needs_recompute");
    expect(retainedStandingFrom(bound, source, [])).toBe("needs_recompute");
    expect(retainedStandingFrom(bound, source, [{ ref_type: "load_case", ref_id: "other" }])).toBe("needs_recompute");
    expect(retainedStandingFrom(bound, source, [...requested, ...requested])).toBe("needs_recompute");
    expect(retainedStandingFrom(bound, { ...source, status: { ...source.status, mechanics: "MODEL_INCOMPLETE" } }, requested)).toBe("needs_recompute");
    // Key order is not significant (serde_json Value equality).
    expect(retainedStandingFrom(bound, source, [{ ref_id: "case", ref_type: "load_case" }])).toBe("numerically_eligible");
  });
  it("applies the base ordinary-eligibility predicate to not_required cases (F-7) and refuses other statuses", () => {
    expect(retainedStandingFrom(bound, twoCase(), two)).toBe("numerically_eligible");
    expect(retainedStandingFrom(bound, twoCase(), [two[1], two[0]])).toBe("needs_recompute");
    for (const [field, value] of [["solve_quality", "sensitive"], ["structural_status", "numerically_unresolved"], ["model_matrix_fidelity", "assembly_uncertainty"], ["accuracy_evidence", "unresolved"], ["evidence_refs", []], ["evidence_refs", ["missing"]], ["evidence_refs", [source.diagnostics[0].id!, "missing"]], ["basis_ref", { ref_type: "load_case", ref_id: "elsewhere" }]] as const) {
      expect(retainedStandingFrom(bound, twoCase(s => { s.numerical_quality.cases[1][field] = value; }), two), field).toBe("needs_recompute");
    }
    expect(retainedStandingFrom(bound, twoCase(s => { s.numerical_quality.cases.pop(); }), two)).toBe("needs_recompute");
    expect(retainedStandingFrom(bound, twoCase(s => { s.retained_precision.body.cases[1].status = "unavailable"; }), two)).toBe("needs_recompute");
    expect(retainedStandingFrom(bound, twoCase(s => { s.diagnostics.push(structuredClone(s.diagnostics[0])); }), two)).toBe("needs_recompute");
    expect(retainedStandingFrom(bound, twoCase(s => { delete s.diagnostics[1].id; }), two)).toBe("needs_recompute");
    expect(retainedStandingFrom(bound, twoCase(s => { s.results[1].id = ""; }), two)).toBe("needs_recompute");
  });
  it("counts withheld rows by standing (Rust classification_summary_from)", async () => {
    const validation = await validateRetainedPrecision(source);
    const notCurrent = classificationSummaryFrom(validation, source, requested);
    expect(notCurrent).toStrictEqual([{ case_id: "case", relative_verified: 25, absolute_verified: 69, interval_bindable: 0, not_covered: 0, input_derived: 3, non_quantity: 1, withheld: 97 }]);
    const current = classificationSummaryFrom({ ...validation, invocation_bound: true, numerical_eligible: true }, source, requested);
    expect(current[0].withheld).toBe(69);
    const uncovered: RetainedPrecisionValidation = { ...validation, invocation_bound: true, numerical_eligible: true, classifications: validation.classifications.map((row, i) => i === 1 ? { ...row, class: "not_covered" } : row) };
    expect(classificationSummaryFrom(uncovered, source, requested)[0]).toMatchObject({ not_covered: 1, withheld: 69 + 1 - (validation.classifications[1].class === "absolute_verified" ? 1 : 0) });
    // Rows count only toward their own case.
    const split: RetainedPrecisionValidation = { ...validation, classifications: validation.classifications.map((row, i) => i < 10 ? { ...row, basis_ref: { ref_type: "load_case", ref_id: "second" } } : row) };
    const counts = classificationSummaryFrom(split, twoCase(), two).map(c => [c.case_id, c.relative_verified + c.absolute_verified + c.not_covered + c.input_derived + c.non_quantity]);
    expect(counts).toStrictEqual([["case", validation.classifications.length - 10], ["second", 10]]);
  });
});

describe.each(MODES)("%s: the post-U7 path through the real carriers", (mode) => {
  it("standing becomes eligible only for the requested cases, and the rule-check gate passes the registered invocation", async () => {
    const { source, invocation, model } = milestone(mode);
    const received = await deliverDirect(source, model, mode);
    expect(numericalResultStanding(received, model)).toStrictEqual({ contract: "retained_preview_physics", status: "integrity_checked", eligible: true, findings: [] });
    expect(numericalResultStanding(received, withLoadCases(model, ["other"])).findings).toStrictEqual([RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE]);
    expect(numericalResultStanding(received, null).eligible).toBe(false);
    expect(retainedPrecisionInvocation(received, model)).toStrictEqual(invocation);
    expect(classificationSummary(received, model)[0].withheld).toBe(69);
    expect(classificationSummary(received, null)[0].withheld).toBe(97);
    invokeMock.mockReset();
    invokeMock.mockImplementation(async (command: string, args: Json) => {
      expect(command).toBe("run_rule_checks");
      expect(args.sourceBlockInvocation).toStrictEqual(invocation);
      expect(args.solvedEnvelope).toBe(received);
      return { document_kind: "rule_check_run", rule_pack_id: "invented", grammar_version: "1.0.0", aggregate_status: "RULE_INPUTS_INCOMPLETE", checks: [], professional_boundary_notice: "unit simulation" };
    });
    const route = await runRuleChecks({ rulePackDocument: {} as RulePackDocument, model, solvedEnvelope: received });
    expect(route.route).toBe("tauri_backend");
    expect(invokeMock).toHaveBeenCalledTimes(1);
    // Saved or copied bytes still never register.
    expect(numericalResultStanding(structuredClone(received), model).findings).toStrictEqual([RETAINED_PRECISION_VALIDATION_REQUIRED]);
  });
  it("a not_covered row is labelled, refused and counted (fabricated class; no statement has one)", async () => {
    u7.simulate = true;
    const { source, model } = milestone(mode);
    const target = source.results.find(row => row.kind === "global_nodal_rotation_x")!;
    u7.notCovered = target.id;
    const received = await deliverDirect(source, model, mode);
    expect(ruleBindingRefusal(received, target)).toBe(RULE_QUANTITY_NOT_COVERED);
    expect(resultRowLabel(target, received)).toBe(N_RP_NOT_COVERED);
    expect(knownSemanticNotices(received).map(n => n.id)).toContain("retained-precision-not-covered:case");
    expect(ruleBindingPrecheck(received, { solverInputs: [{ input_id: "x", name: "x", dimension: "angle", unit_ref: "rad", solver_result_ref: { result_id: target.id } }], valueInputs: [], valueSlots: [], libraryInputs: [] })).toStrictEqual([{ input_id: "x", result_id: target.id, reason: RULE_QUANTITY_NOT_COVERED, notice: N_RP_NOT_COVERED }]);
    expect(classificationSummary(received, model)[0].not_covered).toBe(1);
  });
  it("without absolute rows there is no absolute summary notice", async () => {
    u7.simulate = true; u7.noAbsolute = true;
    const { source, model } = milestone(mode);
    const received = await deliverDirect(source, model, mode);
    expect(classificationSummary(received, model)[0]).toMatchObject({ absolute_verified: 0, not_covered: 0 });
    expect(knownSemanticNotices(received).map(n => n.id)).toStrictEqual(["headline-label"]);
    expect(ruleBindingRefusal(received, received.results[3])).toBeNull();
  });
});

describe.each(MODES)("%s: the rule-check gate and binding refusals", (mode) => {
  const pack = {} as RulePackDocument;
  it("passes a registered successor to the rule backend with its invocation (U7); copies and refused bytes are not native; the held reader's needs_recompute is refused before any backend call", async () => {
    const { source, invocation, model } = milestone(mode);
    const received = await deliverDirect(source, model, mode);
    invokeMock.mockReset();
    invokeMock.mockImplementation(async (command: string, args: Json) => {
      expect(command).toBe("run_rule_checks");
      expect(args.sourceBlockInvocation).toStrictEqual(invocation);
      return { document_kind: "rule_check_run", rule_pack_id: "invented", grammar_version: "1.0.0", aggregate_status: "RULE_INPUTS_INCOMPLETE", checks: [], professional_boundary_notice: "unit simulation" };
    });
    expect((await runRuleChecks({ rulePackDocument: pack, model, solvedEnvelope: received })).route).toBe("tauri_backend");
    expect(invokeMock).toHaveBeenCalledTimes(1);
    invokeMock.mockReset();
    await expect(runRuleChecks({ rulePackDocument: pack, model, solvedEnvelope: structuredClone(received) })).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    const edited = structuredClone(source); edited.results[0].value = 12345;
    const refused = await deliverDirect(edited, model, mode);
    invokeMock.mockReset();
    await expect(runRuleChecks({ rulePackDocument: pack, model, solvedEnvelope: refused })).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    expect(invokeMock).not.toHaveBeenCalled();
    // The held reader (u7.held, the pre-U7 output): a live registration that reads
    // needs_recompute is refused by the gate itself.
    u7.held = true;
    const held = await deliverDirect(source, model, mode);
    invokeMock.mockReset();
    await expect(runRuleChecks({ rulePackDocument: pack, model, solvedEnvelope: held })).rejects.toThrow(`${RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE}: the retained-precision result is not numerically eligible for rule checks.`);
    expect(invokeMock).not.toHaveBeenCalled();
  });
  it("binds rows by validated class; a headline is refused as the row it names; unregistered rows are all refused", async () => {
    const { source, model } = milestone(mode);
    const received = await deliverDirect(source, model, mode);
    const classes = retainedRowClasses(received)!;
    const byClass = (cls: string) => received.results.filter(row => classes.get(row.id)!.class === cls);
    expect(byClass("absolute_verified")).toHaveLength(69);
    for (const row of byClass("absolute_verified")) expect(ruleBindingRefusal(received, row)).toBe(RULE_QUANTITY_BELOW_VERIFIED_FLOOR);
    for (const cls of ["relative_verified", "input_derived", "non_quantity"]) {
      expect(byClass(cls).length).toBeGreaterThan(0);
      for (const row of byClass(cls)) expect(ruleBindingRefusal(received, row), cls).toBeNull();
    }
    expect(ruleBindingRefusal(received, { id: "result:absent", kind: "x" })).toBeNull();
    const headline = received.summary.max_open_formula_stress!.result_ref;
    expect(classes.get(headline)!.class).toBe("absolute_verified");
    const plan: RuleCheckBindingPlan = { solverInputs: [{ input_id: "headline", name: "h", dimension: "stress", unit_ref: "Pa", solver_result_ref: { result_id: headline } }, { input_id: "relative", name: "r", dimension: "force", unit_ref: "N" }], valueInputs: [], valueSlots: [], libraryInputs: [] };
    const relative = byClass("relative_verified")[0];
    const findings = ruleBindingPrecheck(received, plan, [{ input_id: "relative", result_id: relative.id }]);
    const label = retainedAbsoluteNotice(classes.get(headline)!.bound_bits!, "Pa");
    expect(findings).toStrictEqual([{ input_id: "headline", result_id: headline, reason: RULE_QUANTITY_BELOW_VERIFIED_FLOOR, notice: label }]);
    expect(label).not.toBe(N_SB);
    const copy = structuredClone(received);
    expect(ruleBindingRefusal(copy, relative)).toBe(RULE_QUANTITY_NOT_COVERED);
    expect(ruleBindingPrecheck(copy, plan, [{ input_id: "relative", result_id: relative.id }]).map(f => [f.reason, f.notice])).toStrictEqual([[RULE_QUANTITY_NOT_COVERED, N_RP_UNVALIDATED], [RULE_QUANTITY_NOT_COVERED, N_RP_UNVALIDATED]]);
    expect(classBindingRefusal("not_covered")).toBe(RULE_QUANTITY_NOT_COVERED);
    expect(classBindingRefusal("absolute_verified")).toBe(RULE_QUANTITY_BELOW_VERIFIED_FLOOR);
    expect(classBindingRefusal(undefined)).toBeNull();
  });
  it("summarizes classes per case over the registration only", async () => {
    const { source, model } = milestone(mode);
    const received = await deliverDirect(source, model, mode);
    // U7: Current for the captured model, so only the absolute rows are withheld.
    expect(classificationSummary(received, model)).toStrictEqual([{ case_id: "case", relative_verified: 25, absolute_verified: 69, interval_bindable: 0, not_covered: 0, input_derived: 3, non_quantity: mode === "sparse_interactive" ? 1 : 2, withheld: 69 }]);
    expect(classificationSummary(structuredClone(received), model)).toStrictEqual([]);
    expect(classificationSummary(null)).toStrictEqual([]);
  });
});

describe("the downgrade guard (F-5)", () => {
  const { source } = milestone("sparse_interactive");
  const relabel = (s: Json) => { s.producer.semantic_contract_id = PREVIEW_PHYSICS_CONTRACT_ID; s.formulation_basis.profile_id = "product_preview_mechanics_v1"; return s; };
  it.each([
    ["the receipt and the token rows", (s: Json) => relabel(s)],
    ["the token rows only", (s: Json) => { delete relabel(s).retained_precision; }],
    ["a null receipt member only", (s: Json) => { relabel(s).retained_precision = null; for (const row of s.results) delete row.recovery_method; }],
    ["one token row only", (s: Json) => { delete relabel(s).retained_precision; for (const row of s.results.slice(1)) delete row.recovery_method; }],
  ] as const)("a successor relabelled as preview-physics-1 keeping %s is unsupported", (_label, edit) => {
    const s = structuredClone(source) as Json; edit(s);
    expect(retainedPrecisionDowngrade(s)).toBe(true);
    expect(sourceContract(s)).toBe("unsupported");
    expect(numericalResultStanding(s, null)).toStrictEqual({ contract: "unsupported", status: "needs_recompute", eligible: false, findings: [RETAINED_PRECISION_DOWNGRADE_FORBIDDEN] });
    expect(hasCurrentSourceContract(s)).toBe(false);
    expect(isFreshSemanticResult(s)).toBe(false);
    expect(ruleBindingRefusal(s, s.results[0])).toBeNull();
  });
  it("with neither the receipt nor a token row it is the base identity again, and the guard ignores other method strings", () => {
    const s = relabel(structuredClone(source));
    delete s.retained_precision;
    for (const row of s.results) delete row.recovery_method;
    expect(retainedPrecisionDowngrade(s)).toBe(false);
    expect(sourceContract(s)).toBe("preview_physics");
    s.results[0].recovery_method = "some_other_method";
    expect(retainedPrecisionDowngrade(s)).toBe(false);
  });
  it("applies to every other identity, including legacy 0.1.0 and an unparseable header", () => {
    const legacy = JSON.parse(readFileSync(resolve(root, "fixtures/product_preview/invented_mechanics_result.json"), "utf8"));
    expect(sourceContract(legacy)).toBe("legacy");
    expect(sourceContract({ ...legacy, retained_precision: {} })).toBe("unsupported");
    expect(numericalResultStanding({ ...legacy, retained_precision: {} }, null).findings).toStrictEqual([RETAINED_PRECISION_DOWNGRADE_FORBIDDEN]);
    const preview = JSON.parse(readFileSync(resolve(root, "fixtures/results/preview_physics_connected_sparse.json"), "utf8"));
    expect(sourceContract(preview)).toBe("preview_physics");
    expect(sourceContract({ ...preview, retained_precision: structuredClone(source.retained_precision) })).toBe("unsupported");
    expect(sourceContract({ ...preview, results: preview.results.map((row: Json, i: number) => i ? row : { ...row, recovery_method: RETAINED_METHOD }) })).toBe("unsupported");
    expect(retainedPrecisionDowngrade({ ...legacy, results: [null, 3] })).toBe(false);
    expect(retainedPrecisionDowngrade({ ...legacy, results: "rows" })).toBe(false);
    expect(numericalResultStanding({ ...legacy, producer: { semantic_contract_id: "unknown" } } as MechanicsResult, null).findings).toStrictEqual(["SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED"]);
  });
  it("the successor's own header requires its profile, its contract evidence and a receipt object, and no other receipt", () => {
    expect(sourceContract(source)).toBe("retained_preview_physics");
    for (const edit of [
      (s: Json) => { s.formulation_basis.profile_id = "product_preview_mechanics_v1"; },
      (s: Json) => { delete s.contract_evidence; },
      (s: Json) => { s.retained_precision = null; },
      (s: Json) => { s.retained_precision = [s.retained_precision]; },
      (s: Json) => { s.retained_precision = "receipt"; },
      (s: Json) => { delete s.retained_precision; },
      (s: Json) => { s.source_block_recovery = {}; },
      (s: Json) => { s.carrier_evidence = {}; },
      (s: Json) => { s.producer.component_version = "0.1.0"; },
    ]) {
      const s = structuredClone(source) as Json; edit(s);
      expect(sourceContract(s)).toBe("unsupported");
      expect(numericalResultStanding(s, null).findings).toStrictEqual(["SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED"]);
      // RV91 N-3 (RV08): binding selects the successor by producer id, as Rust does,
      // so a header-broken successor still has no validated class and every row is refused.
      for (const row of s.results.slice(0, 3)) expect(ruleBindingRefusal(s, row)).toBe(RULE_QUANTITY_NOT_COVERED);
    }
  });
});

// The ruled differences between the languages' carriers (and F5's shared semantics),
// read from the shared file's `declared_differences` (format v3: each entry lists
// one or more forms) with TypeScript's own expectations (RR "RV88 on U6a, U6c, U6b
// (and U6d)…" and "RV92 (U6f) on the whole of U6…"). Rust and Python assert theirs
// from the same forms; any other difference is a defect.
const DECLARED = ["D-U7-4:ts_requires_live_native_capture", "F-U6b-2:python_refuses_transport", "F5:refused_statement_binding", "I67-F1:unregistered_invalid_statement", "I67-F2:display_only_binding_precheck", "RV92-N2-N5:ts_refuses_token_rows_at_the_header"];
const NOTICES: Record<string, string> = { N_RP_UNVALIDATED };
/** The shared 'summary' vocabulary, counted here from the reader's classes, not through the
 * seam: per receipt case, interval_bindable 0; `withheld` is every quantity row, or when
 * Current ('by_validated_class_current', U7 repair, RV94 S-1) only the absolute and not-covered rows. */
const expectedSummary = (classes: readonly RowClassification[], source: Json, current: boolean) => (source.retained_precision.body.cases as Json[]).map(c => {
  const n = (k: string) => classes.filter(r => r.basis_ref.ref_id === c.basis_ref.ref_id && r.class === k).length;
  return { case_id: c.basis_ref.ref_id, relative_verified: n("relative_verified"), absolute_verified: n("absolute_verified"), interval_bindable: 0, not_covered: n("not_covered"), input_derived: n("input_derived"), non_quantity: n("non_quantity"),
    withheld: current ? n("absolute_verified") + n("not_covered") : n("relative_verified") + n("absolute_verified") + n("not_covered") + n("input_derived") };
});
const SUMMARY_KIND = "open_formula_stress_summary";
const declaredForms = (caseFile.declared_differences as Json[]).flatMap(e => (e.forms as Json[]).flatMap(form => (form.fixtures as string[]).map(f => [`${e.id} / ${form.label}`, f, form] as [string, string, Json])));
describe("the declared differences, with TypeScript's expectations", () => {
  it("are exactly the six ruled entries, each with a ruling, forms and one expectation per language, in closed field sets", () => {
    const entries = caseFile.declared_differences as Json[];
    expect(entries.map(e => e.id).sort()).toStrictEqual(DECLARED);
    const subjects = new Set<string>(), v4 = new Set<string>();
    for (const entry of entries) {
      expect(typeof entry.ruling === "string" && entry.ruling.length > 0, entry.id).toBe(true);
      expect(entry.forms.length, entry.id).toBeGreaterThan(0);
      for (const form of entry.forms) {
        expect(Object.keys(form.expected).sort(), `${entry.id} ${form.label}`).toStrictEqual(["python", "rust", "typescript"]);
        expect(Object.keys(form).filter(k => !FORM_FIELDS.includes(k)), `${entry.id} ${form.label}`).toStrictEqual([]);
        subjects.add(form.subject);
        for (const field of ["capture", "current_model_edits"]) if (Object.hasOwn(form, field)) v4.add(field);
      }
    }
    for (const c of caseFile.cases as Json[]) expect(Object.keys(c).filter(k => !CASE_FIELDS.includes(k)), c.id).toStrictEqual([]);
    expect([...subjects].sort()).toStrictEqual(["binding", "standing", "summary", "transport"]);
    // Both v4 fields TS consumes are exercised.
    expect([...v4].sort()).toStrictEqual(["capture", "current_model_edits"]);
    // D-U7-4, TS's side: without a live native capture of these bytes and the current
    // model, a valid successor with its invocation reads needs_recompute.
    const capture = entries.find(e => e.id === "D-U7-4:ts_requires_live_native_capture");
    expect(capture.forms.map((f: Json) => [f.label, f.subject, f.expected.typescript])).toStrictEqual([
      ["invocation_without_native_capture", "standing", { standing: "needs_recompute", finding: RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED }],
      ["stale_current_model_same_case_ids", "standing", { standing: "needs_recompute", finding: RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED }],
      // U7 repair (RV94 S-1): the summary follows TS's standing, so TS's summary is not Current.
      ["invocation_without_native_capture:summary", "summary", { summary: "by_validated_class" }],
      ["stale_current_model_same_case_ids:summary", "summary", { summary: "by_validated_class" }],
    ]);
    // Each summary form reads the same inputs as its standing twin.
    const inputs = (f: Json) => { const { label: _l, subject: _s, expected: _e, ...rest } = f; return rest; };
    expect(capture.forms.slice(2).map(inputs)).toStrictEqual(capture.forms.slice(0, 2).map(inputs));
  });
  it("carry the N-4 scope: differences inherited from the base carriers are not U6's", () => {
    expect(caseFile.scope).toMatch(/inherited from the base carriers are not U6 differences/);
    expect(caseFile.scope).toMatch(/G7 parity compares the reader's \(gate, code\)[\s\S]*parity there compares only accept against refuse/);
    // D-U7-6: eligibility is a property of the supplied statement and its invocation.
    expect(caseFile.scope).toMatch(/no carrier authenticates producer origin/);
    // I66 U7 slice F observation 3: a blocked envelope is refused at G7 with each language's own base code.
    expect(caseFile.scope).toMatch(/a blocked envelope is refused at G7 with each language's own base code[^.]*TS SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID/);
    // RV94 N-3 (U7 repair): an invalid enum value in a not_required case's quality, TS's G7 contract check first.
    expect(caseFile.scope).toMatch(/An invalid enum value in a not_required case's quality is refused at G7 with each language's own code[^.]*TS SOURCE_PRODUCER_CONTRACT_UNSUPPORTED/);
  });
  it.each<[string, string, Json]>(declaredForms)("%s on %s", async (_id, fixtureId, form) => {
    const expected = form.expected.typescript;
    const { received, model } = await applyShared(form, fixtureId);
    if (form.subject === "standing") {
      const standing = retainedPrecisionStanding(received, model);
      expect(standing.standing).toBe(expected.standing);
      expect(standing.findings[0]).toBe(expected.finding);
      // The token is compared; TS's status follows the pinned mapping (RV92 N-8).
      expect(numericalResultStanding(received, model)).toStrictEqual({ contract: "retained_preview_physics", status: RETAINED_STANDING_STATUS[expected.standing as keyof typeof RETAINED_STANDING_STATUS], eligible: false, findings: [expected.finding] });
    } else if (form.subject === "transport") {
      // TS's carrier transport route (RV88 and RV92 N-1).
      expect(await sourceContractTransport(received).then(() => "ok", (error: Error) => error.message)).toBe(expected.transport);
    } else if (form.subject === "summary") {
      if (expected.summary === "empty") expect(classificationSummary(received, model)).toStrictEqual([]);
      else {
        expect(["by_validated_class", "by_validated_class_current"]).toContain(expected.summary);
        expect(classificationSummary(received, model)).toStrictEqual(expectedSummary((await validateRetainedPrecision(received)).classifications, received, expected.summary === "by_validated_class_current"));
      }
    } else {
      expect(form.subject).toBe("binding");
      const rows = received.results;
      const got = rows.map(row => ruleBindingRefusal(received, row));
      const [rule, code] = String(expected.binding).split(":");
      if (rule === "by_validated_class") {
        const classes = new Map((await validateRetainedPrecision(received)).classifications.map(c => [c.result_id, c.class]));
        expect(got).toStrictEqual(rows.map(row => classBindingRefusal(classes.get(row.id))));
      } else if (rule === "source_blocks_summary") {
        const headline = received.summary.max_open_formula_stress?.result_ref;
        expect(got).toStrictEqual(rows.map(row => row.kind === SUMMARY_KIND || row.id === headline ? code : null));
      } else {
        expect(rule).toBe("every_row");
        expect(got).toStrictEqual(rows.map(() => code === "none" ? null : code));
        if (expected.notice) {
          // The display-only precheck shows the declared notice on every row.
          const plan: RuleCheckBindingPlan = { solverInputs: rows.map(row => ({ input_id: row.id, name: row.id, dimension: "x", unit_ref: "x", solver_result_ref: { result_id: row.id } })), valueInputs: [], valueSlots: [], libraryInputs: [] };
          const notices = ruleBindingPrecheck(received, plan);
          expect(notices).toHaveLength(rows.length);
          expect(new Set(notices.map(f => f.notice))).toStrictEqual(new Set([NOTICES[expected.notice]]));
        }
      }
    }
    expect(numericalResultStanding(received, model).eligible).toBe(false);
  });
  // D-U7-4 (I67 u7_slice_f_02): the summary follows TS's standing, live capture included,
  // so neither form's summary reads Current although D2 4.9.4 alone would.
  it.each<[string, string, Json]>(declaredForms.filter(([id]) => id.startsWith("D-U7-4:")))("%s on %s: the summary's withheld is the not-Current count", async (_id, fixtureId, form) => {
    const { received, model } = await applyShared(form, fixtureId);
    expect(retainedPrecisionStanding(received, model).findings).toStrictEqual([RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED]);
    const validation = retainedPrecisionRegistration(received)!.validation!;
    const requested = model.load_cases.map(c => ({ ref_type: "load_case", ref_id: c.id }));
    expect(classificationSummaryFrom(validation, received, requested)[0].withheld).toBe(69);
    const notCurrent = classificationSummaryFrom(validation, received, []);
    expect(notCurrent[0].withheld).toBe(97);
    expect(classificationSummary(received, model)).toStrictEqual(notCurrent);
  });
  // An additional TypeScript input for the same declared difference (I67-F1): a
  // rewritten ordinary quality claim, unregistered, also reads needs_recompute.
  it.each(MODES)("%s: I67-F1 also holds for numerical_quality rewritten to checks_passed", async (mode) => {
    const entry = (caseFile.declared_differences as Json[]).find(e => e.id === "I67-F1:unregistered_invalid_statement").forms[0];
    const { source, model } = milestone(mode);
    const invalid = structuredClone(source) as Json;
    invalid.numerical_quality.status = "checks_passed"; invalid.numerical_quality.cases[0].solve_quality = "checks_passed";
    await expect(validateRetainedPrecision(invalid)).rejects.toThrow("RETAINED_PRECISION_RECEIPT_MISMATCH");
    const standing = retainedPrecisionStanding(invalid, model);
    expect([standing.standing, standing.findings[0]]).toStrictEqual([entry.expected.typescript.standing, entry.expected.typescript.finding]);
    expect(ruleBindingRefusal(invalid, invalid.results[1])).toBe(RULE_QUANTITY_NOT_COVERED);
  });
});

// RV88 and RV92 N-1: TS's carrier transport route (Rust `for_source_metadata`'s twin)
// runs the reader's transport checks, so a tampered transported successor is refused
// with Rust's code (RV92's 10 probes: these five forms in both modes; the
// no-invocation twin is the same bytes, since transport reads no invocation).
describe.each(MODES)("%s: the carrier transport route runs the reader's transport checks (RV88 and RV92 N-1)", (mode) => {
  const transport = (s: MechanicsResult) => sourceContractTransport(s).then(route => `ok:${route}`, (error: Error) => error.message);
  const zeroHash = (s: Json) => { s.retained_precision.receipt_sha256 = "0".repeat(64); };
  const headerOnly = (s: Json) => { delete s.results; };
  it.each([
    ["receipt_sha_zero", zeroHash, "RETAINED_PRECISION_RECEIPT_MISMATCH"],
    ["receipt_sha_zero_no_invocation", zeroHash, "RETAINED_PRECISION_RECEIPT_MISMATCH"],
    ["receipt_body_edit (unsealed)", (s: Json) => { s.retained_precision.body.work.charged += 1; }, "RETAINED_PRECISION_RECEIPT_MISMATCH"],
    ["receipt_empty", (s: Json) => { s.retained_precision = {}; }, "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"],
    ["transport_no_results_receipt_zero", (s: Json) => { headerOnly(s); zeroHash(s); }, "RETAINED_PRECISION_RECEIPT_MISMATCH"],
  ] as const)("%s is refused with Rust's code", async (_label, edit, code) => {
    const s = structuredClone(milestone(mode).source) as Json; edit(s);
    // The header route alone admits it; the transport route does not.
    expect(sourceContract(s)).toBe("retained_preview_physics");
    expect(await transport(s)).toBe(code);
  });
  it("an untampered transport passes, with or without raw rows, and is never eligible", async () => {
    const { source } = milestone(mode);
    expect(await transport(structuredClone(source))).toBe("ok:retained_preview_physics");
    const header = structuredClone(source) as Json; headerOnly(header);
    expect(await transport(header)).toBe("ok:retained_preview_physics");
    expect((await validateRetainedPrecisionTransport(header)).numerical_eligible).toBe(false);
  });
  it("a receipt-consistent but base-inconsistent transport is refused at the reader's G7", async () => {
    const s = structuredClone(milestone(mode).source) as Json; s.contract_evidence.combination_gates = "not-an-array";
    expect(sourceContract(s)).toBe("retained_preview_physics");
    const error = await sourceContractTransport(s).then(() => null, (e: Json) => e);
    expect(error?.gate).toBe("G7");
  });
});
describe("the carrier transport route on other identities is the header route", () => {
  it("resolves each existing route, and refuses an unsupported or downgraded header with its standing code", async () => {
    const legacy = sharedFixture("legacy_preview_0_1").source, preview = sharedFixture("preview_physics_1_invented_sparse").source, blocks = sharedFixture("source_blocks_n05_sparse").source;
    expect(await sourceContractTransport(structuredClone(legacy))).toBe("legacy");
    expect(await sourceContractTransport(structuredClone(preview))).toBe("preview_physics");
    expect(await sourceContractTransport(structuredClone(blocks))).toBe("source_blocks");
    await expect(sourceContractTransport({ ...structuredClone(preview), retained_precision: null } as MechanicsResult)).rejects.toThrow(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN);
    await expect(sourceContractTransport({ ...structuredClone(preview), carrier_evidence: {} } as MechanicsResult)).rejects.toThrow("SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED");
  });
});

// U7 slice T (RV91 N-2 = RV88 U6d S-1; D-U7-4): a successor's standing is bound to
// the live native capture of these bytes and to the current model, as
// hasNativeMechanicsInvocation is. Since U7 the reader's own eligibility is used;
// the held reader (u7.held) shows the pre-U7 reading.
describe.each(MODES)("%s: standing is bound to the live native capture and the current model (U7 slice T)", (mode) => {
  const movedNode = (model: PreviewModel) => { const m = structuredClone(model) as Json; m.nodes[0].position.x += 1; return m as PreviewModel; };
  it("eligible only for the captured model; another model with the same case ids reads needs_recompute", async () => {
    const { source, model } = milestone(mode);
    const received = await deliverDirect(source, model, mode);
    expect(retainedPrecisionStanding(received, model)).toStrictEqual({ standing: "numerically_eligible", eligible: true, findings: [] });
    const other = movedNode(model);
    expect(caseIds(other)).toStrictEqual(caseIds(model));
    expect(hasNativeMechanicsInvocation(received, other)).toBe(false);
    expect(retainedPrecisionStanding(received, other)).toStrictEqual({ standing: "needs_recompute", eligible: false, findings: [RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED] });
    expect(numericalResultStanding(received, other)).toStrictEqual({ contract: "retained_preview_physics", status: "needs_recompute", eligible: false, findings: [RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED] });
    expect(retainedPrecisionInvocation(received, other)).toBeNull();
    // A model carrying only the case ids (no captured model) is not the current model either.
    expect(retainedPrecisionStanding(received, { load_cases: model.load_cases } as PreviewModel).findings).toStrictEqual([RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED]);
    expect(retainedPrecisionStanding(received, null).findings).toStrictEqual([RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE]);
  });
  it("an edit of the caller's model object after the capture voids the binding", async () => {
    const { source, model } = milestone(mode);
    const callerModel = structuredClone(model);
    const received = await deliverDirect(source, callerModel, mode);
    expect(retainedPrecisionStanding(received, callerModel).eligible).toBe(true);
    (callerModel as Json).nodes[0].position.x += 1;
    expect(retainedPrecisionStanding(received, callerModel).findings).toStrictEqual([RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED]);
    expect(retainedPrecisionStanding(received, model).findings).toStrictEqual([RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED]);
  });
  it("a job cancelled while the reader validates keeps no live capture", async () => {
    const { source, model } = milestone(mode);
    (window as Json).__TAURI_INTERNALS__ = {};
    const jobId = `unit-transport-replay:retained:cancel:${++jobSequence}`;
    invokeMock.mockImplementation(async (command: string) => {
      if (command === "start_preview_mechanics_job_with_solver_mode") return { job_id: jobId, backend_cancellation_token: "t", state: "queued", cancellation_scope: "unit" };
      if (command === "poll_preview_mechanics_job") return { job_id: jobId, state: "completed", cancellation_requested: false, cancellation_status: "not_requested", cancellation_scope: "unit", result: structuredClone(source), error_message: null };
      expect(command).toBe("cancel_preview_mechanics_job");
      return { job_id: jobId, accepted: true, cancellation_status: "accepted", job_state: "completed", cancellation_scope: "unit", cancellation_success_claimed: false };
    });
    await startPreviewMechanicsJob(model, mode);
    const polling = pollPreviewMechanicsJob(jobId);
    await cancelPreviewMechanicsJob(jobId, "t"); // accepted while the reader awaits
    const received = (await polling).result!;
    // The reader's own validation is recorded, but the capture was invalidated.
    expect(retainedPrecisionRegistration(received)?.validation?.numerical_eligible).toBe(true);
    expect(hasNativeMechanicsInvocation(received, model)).toBe(false);
    expect(retainedPrecisionStanding(received, model)).toStrictEqual({ standing: "needs_recompute", eligible: false, findings: [RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED] });
  });
  it("a registration without a native capture is never eligible", async () => {
    const { source, invocation, model } = milestone(mode);
    const own = structuredClone(source);
    expect((await registerRetainedPrecision(own, structuredClone(invocation))).numerical_eligible).toBe(true);
    expect(retainedPrecisionStanding(own, model).findings).toStrictEqual([RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED]);
    expect(retainedPrecisionInvocation(own, model)).toBeNull();
  });
  it("with the held reader (u7.held), every one of these reads as before U7 (NOT_NUMERICALLY_ELIGIBLE)", async () => {
    u7.held = true;
    const { source, invocation, model } = milestone(mode);
    const received = await deliverDirect(source, model, mode);
    for (const m of [model, movedNode(model), { load_cases: model.load_cases } as PreviewModel]) {
      expect(retainedPrecisionStanding(received, m)).toStrictEqual({ standing: "needs_recompute", eligible: false, findings: [RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE] });
    }
    const own = structuredClone(source);
    await registerRetainedPrecision(own, structuredClone(invocation));
    expect(retainedPrecisionStanding(own, model).findings).toStrictEqual([RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE]);
  });
});

// U7 slice T (I61 slice A, §1.4): knownSemanticNotices calls classificationSummary
// without the model, by design. The notices show only the absolute-verified and
// not-covered counts, which are the reader's G5c classes and do not depend on standing;
// the per-quantity refusals they state hold whether or not the envelope is Current
// (S-I has not landed). The model-dependent field, `withheld`, is not shown.
describe.each(MODES)("%s: the class notices do not depend on standing (U7 slice T)", (mode) => {
  it("an eligible successor shows exactly the notices a held one does; only `withheld` would differ", async () => {
    const { source, model } = milestone(mode);
    u7.held = true;
    const held = await deliverDirect(source, model, mode);
    expect(numericalResultStanding(held, model).eligible).toBe(false);
    const heldNotices = knownSemanticNotices(held);
    u7.held = false;
    const eligible = await deliverDirect(source, model, mode);
    expect(numericalResultStanding(eligible, model).eligible).toBe(true);
    expect(knownSemanticNotices(eligible)).toStrictEqual(heldNotices);
    const withModel = classificationSummary(eligible, model), withoutModel = classificationSummary(eligible);
    expect(withModel.map(({ withheld: _w, ...counts }) => counts)).toStrictEqual(withoutModel.map(({ withheld: _w, ...counts }) => counts));
    expect([withModel[0].withheld, withoutModel[0].withheld]).toStrictEqual([69, 97]);
  });
});

// U7 slice T (RV91 N-5), superseded by T6S-3 (RR decision 2): the result-export and
// stress-neutral panels gate a successor by their explicit entries in the output
// policy, not by a builder throwing: they admit it only at numerically eligible
// standing with the live native capture. Each test registers an eligible successor
// through mocked IPC with the pinned model and lets the result-export builder proceed
// (t6.builderDoc), so that only the gate decides: the panel withholds the packet for a
// moved model, which is not the live capture, and offers it for the captured model.
describe.each(MODES)("%s: the T6 panels gate a successor by an explicit policy entry (U7 slice T; T6S-3)", (mode) => {
  async function eligibleSuccessor() {
    const { source, model } = milestone(mode);
    const received = await deliverDirect(source, model, mode);
    expect(numericalResultStanding(received, model).eligible).toBe(true);
    expect(hasNativeMechanicsInvocation(received, model, mode)).toBe(true);
    const manifest = { manifest_ref: { object_type: "InputManifest", ref: "manifest:invented-u7-gate" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: received.model_ref, model_payload: model }, solver_basis: { solver_name: received.producer!.component_name, solver_version: received.producer!.component_version, solver_build_ref: "unit-transport-replay-not-native-witness", solver_mode: mode } } } as unknown as CurrentSessionInputManifestEvidence;
    const analysisRun = await buildAnalysisRunV03(received, manifest as Json, undefined, modelLoadBasisRefs(model));
    return { received, model, analysisRun, manifest };
  }
  const DOC = { export_format_status: { baseline_format: "invented", additional_formats: "none" }, result_envelope: { run_ref: { ref_id: "invented" }, model_ref: { ref_id: "invented" }, result_sets: [{ values: [{ unit: "N", dimension: "force" }] }], diagnostics: [], unit_preservation_witnesses: [], reproducibility: { deterministic_ordering: true, run_hashes: [] }, professional_boundary: { human_review_required: true } } };
  const movedModel = (model: PreviewModel) => { const copy = structuredClone(model) as Json; copy.nodes[0].position.x += 1; return copy as PreviewModel; };
  it("result export: no packet is offered for a moved model even when its builder would not refuse", async () => {
    const { received, model, analysisRun, manifest } = await eligibleSuccessor();
    t6.builderDoc = DOC;
    render(<ResultExportPanel model={movedModel(model)} result={received} analysisRun={analysisRun} inputManifest={manifest} />);
    await new Promise(settle => setTimeout(settle, 50));
    expect(screen.queryByTestId("result-export-summary")).toBeNull();
    cleanup();
    // The admission: the captured model is the live capture, and the panel offers the packet.
    render(<ResultExportPanel model={model} result={received} analysisRun={analysisRun} inputManifest={manifest} />);
    expect((await screen.findByTestId("result-export-summary")).textContent).toContain("available");
  });
  it("stress-neutral: the live binding is closed for a moved model, whose panel shows the standing's refusal", async () => {
    const { received, model, analysisRun } = await eligibleSuccessor();
    expect(liveStressBinding(movedModel(model), received, analysisRun)).toBeNull();
    render(<StressNeutralExportPanel model={movedModel(model)} result={received} analysisRun={analysisRun} />);
    await new Promise(settle => setTimeout(settle, 50));
    expect(screen.getByTestId("stress-neutral-load-reference-output-unavailable").textContent).toBe(`${RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED}: ${N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE}`);
    expect(screen.queryByTestId("stress-neutral-empty")).toBeNull();
    // The admission: the binding is open for the captured model.
    expect(liveStressBinding(model, received, analysisRun)).not.toBeNull();
  });
});

// U7 slice T (RV92 N-8): carriers compare standing by the carrier TOKEN
// (needs_recompute, numerically_eligible, unsupported), never by TS's status string;
// TS's token-to-status mapping is pinned separately here.
describe("standing compares by the carrier token; TS's status mapping is pinned separately (U7 slice T)", () => {
  it("pins the mapping from each token to TS's status", () => {
    expect(RETAINED_STANDING_STATUS).toStrictEqual({ numerically_eligible: "integrity_checked", needs_recompute: "needs_recompute", unsupported: "needs_recompute" });
    expect(Object.isFrozen(RETAINED_STANDING_STATUS)).toBe(true);
  });
  // Since U7, exactly the two capture-bound, requested-as-invoked, unedited milestone
  // cases are numerically_eligible (written out, not derived from the shared file).
  const ELIGIBLE_AFTER_FLIP = new Set(["sparse_interactive:invocation", "dense_scrutiny:invocation"]);
  it("the shared file's eligible cases are exactly these", () => {
    expect((caseFile.cases as Json[]).filter(c => c.expected_standing === "numerically_eligible").map(c => c.id).sort()).toStrictEqual([...ELIGIBLE_AFTER_FLIP].sort());
  });
  it.each<[string, Json]>((caseFile.cases as Json[]).map(c => [c.id, c]))("%s: token and status", async (_id, c) => {
    const { received, model } = await applyShared(c, c.fixture);
    const route = sourceContract(received);
    const token = route === "unsupported" ? "unsupported" : retainedPrecisionStanding(received, model).standing;
    const expected = ELIGIBLE_AFTER_FLIP.has(c.id) ? "numerically_eligible" : c.expected_standing;
    expect(token).toBe(expected);
    const standing = numericalResultStanding(received, model);
    expect(standing.status).toBe(RETAINED_STANDING_STATUS[token as keyof typeof RETAINED_STANDING_STATUS]);
    expect(standing.eligible).toBe(token === "numerically_eligible");
  });
});

describe("the 20 shared parity scenarios agree with Rust U6a and Python U6b", () => {
  it("covers every case of the shared file", () => {
    expect((caseFile.cases as Json[]).map(c => c.fixture).filter((f: string) => !Object.hasOwn(caseFile.fixtures, f))).toStrictEqual([]);
  });
  it.each<[string, Json]>((caseFile.cases as Json[]).map(c => [c.id, c]))("%s", async (_id, c) => {
    // TS has no synchronous reader: the invocation case is a capture of that
    // invocation through mocked IPC; no invocation is a delivery without a capture.
    const { received, model } = await applyShared(c, c.fixture);
    const route = sourceContract(received);
    const standing = route === "unsupported" ? "unsupported" : route === "retained_preview_physics" ? retainedPrecisionStanding(received, model).standing : "unexpected";
    expect(standing).toBe(c.expected_standing);
    // Raw dispatch: the header route, then the accepted reader without an invocation.
    let dispatch: string;
    if (route === "unsupported") dispatch = numericalResultStanding(received, model).findings[0];
    else dispatch = await validateRetainedPrecision(received).then(() => "ok", (error: Json) => error.code);
    expect(dispatch).toBe(c.expected_dispatch);
    expect(numericalResultStanding(received, model).eligible).toBe(c.expected_standing === "numerically_eligible");
  });
});

describe("notices, labels and the results-panel standing text", () => {
  it("the bound is printed upward with three significant digits", () => {
    expect(upwardBoundText(0)).toBe("0");
    // A printed value equal to b as a binary64 is stepped up too: never below b.
    expect(upwardBoundText(1.25)).toBe("1.26e+0");
    expect(upwardBoundText(1.2344e-20)).toBe("1.24e-20");
    expect(upwardBoundText(9.9951e-5)).toBe("1.00e-4");
    expect(upwardBoundText(9.996e3)).toBe("1.00e+4");
    // A step up from 9.99 carries into the exponent.
    expect(upwardBoundText(9.994e-5)).toBe("1.00e-4");
    expect(upwardBoundText(9.993e3)).toBe("1.00e+4");
    expect(upwardBoundText(1.2345e7)).toBe("1.24e+7");
    expect(upwardBoundText(1.23e-20)).toBe("1.24e-20");
    expect(N_RP_ABSOLUTE).toContain("±{b} {unit}");
    // b is in the SI unit the reader classified the quantity in.
    const one = "3ff0000000000000";
    // The same table as Rust derivative::si_unit (RV88 U6a S-2), every entry.
    for (const [unit, si] of [["m", "m"], ["mm", "m"], ["rad", "rad"], ["N", "N"], ["kN", "N"], ["N*m", "N*m"], ["kN*m", "N*m"], ["Pa", "Pa"], ["MPa", "Pa"]]) expect(retainedAbsoluteNotice(one, unit)).toBe(N_RP_ABSOLUTE.replace("{b}", "1.01e+0").replace("{unit}", si));
    // A unit the reader does not normalize names no bound, as in Rust (labelled uncovered).
    for (const unit of ["degC", "mode_code", "unitless", "toString", ""]) expect(retainedAbsoluteNotice(one, unit)).toBe(N_RP_NOT_COVERED);
  });
  it.each(MODES)("%s: registered rows carry their class label with an upward b in SI units; notices summarize per case", async (mode) => {
    const { source, model } = milestone(mode);
    const received = await deliverDirect(source, model, mode);
    const classes = retainedRowClasses(received)!;
    let checked = 0;
    for (const row of received.results) {
      const c = classes.get(row.id)!, label = resultRowLabel(row, received);
      if (c.class !== "absolute_verified") { expect(label === null || !label.includes("Uncovered quantity"), row.id).toBe(true); continue; }
      const match = /±(\S+) (\S+), below/.exec(label!)!;
      expect(Number(match[1])).toBeGreaterThanOrEqual(decodeBinary64(c.bound_bits!));
      expect(match[2]).toBe({ mm: "m", MPa: "Pa", kN: "N" }[row.unit] ?? row.unit);
      checked += 1;
    }
    expect(checked).toBe(69);
    const mm = received.results.find(row => row.unit === "mm" && classes.get(row.id)!.class === "absolute_verified");
    if (mm) expect(resultRowLabel(mm, received)).toContain(" m, below");
    const headline = received.results.find(row => row.id === received.summary.max_open_formula_stress!.result_ref)!;
    expect(resultRowLabel(headline, received)).toBe(`${retainedAbsoluteNotice(classes.get(headline.id)!.bound_bits!, headline.unit)} ${N_HEADLINE}`);
    const notices = knownSemanticNotices(received);
    expect(notices.map(n => n.id)).toStrictEqual(["headline-label", "retained-precision-absolute:case"]);
    expect(notices[1].text).toContain("69 quantities verified only to an absolute bound");
    // Unregistered: no class is known, so no row is presented as verified.
    const copy = structuredClone(received);
    expect(knownSemanticNotices(copy).map(n => n.id)).toStrictEqual(["headline-label", "retained-precision-unvalidated"]);
    expect(resultRowLabel(headline, copy)).toBe(N_HEADLINE);
  });
  it("the standing text comes from the registered receipt, never from numerical_quality", async () => {
    const { source, model } = milestone("sparse_interactive");
    const received = await deliverDirect(source, model, "sparse_interactive");
    expect(source.numerical_quality!.status).toBe("sensitive");
    render(<ResultsPanel result={received} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} />);
    const text = screen.getByTestId("numerical-result-standing").textContent!;
    expect(text).toBe(retainedPrecisionStandingText(received));
    expect(text).toContain("receipt validated by the retained-precision reader against the actual invocation");
    expect(text).not.toMatch(/sensitive|Numerical integrity:/);
    const absolute = received.results.find(row => retainedRowClasses(received)!.get(row.id)!.class === "absolute_verified" && row.unit === "N")!;
    expect(screen.getByTestId(`result-row-label-${absolute.id}`).textContent).toContain("Uncovered quantity: verified only to an absolute bound");
    expect(screen.getByTestId("results-notice-retained-precision-absolute:case")).toBeTruthy();
    cleanup();
    render(<ResultsPanel result={structuredClone(received)} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} />);
    expect(screen.getByTestId("numerical-result-standing").textContent).toContain("receipt not validated for these exact bytes");
    cleanup();
    const edited = structuredClone(source); edited.results[0].value = 12345;
    const refused = await deliverDirect(edited, model, "sparse_interactive");
    expect(retainedPrecisionStandingText(refused)).toContain("refused by the retained-precision reader (RETAINED_PRECISION_RECEIPT_MISMATCH); unsupported, values shown for inspection only.");
    // RV91 N-4: the receipt's case count is shown only from a validated registration.
    expect(retainedPrecisionStandingText(received)).toContain("Selected cases: 1 of 1.");
    expect(retainedPrecisionStandingText(refused)).not.toContain("Selected cases");
    expect(retainedPrecisionStandingText(structuredClone(received))).not.toContain("Selected cases");
    // D-U7-6 (I61 slice A 1.5): every standing text says the reader does not establish producer origin.
    for (const s of [received, refused, structuredClone(received)]) {
      expect(retainedPrecisionStandingText(s)).toContain("against the actual invocation and the requested cases. The reader checks these bytes and their invocation; it does not establish which producer made them. Numerical checks do not establish engineering correctness.");
    }
  });
  it("the case count is the validated receipt's selected cases over all its cases (RV91 N-4)", async () => {
    // A valid two-case statement from the shared reader corpus: one selected, one unavailable.
    const corpus = JSON.parse(readFileSync(resolve(root, "fixtures/results/retained_precision_cases.json"), "utf8"));
    const entry = corpus.cases.find((c: Json) => c.id === "two_case_facade_after_certificate_synthetic");
    const statuses = entry.source.retained_precision.body.cases.map((c: Json) => c.status);
    expect(statuses).toStrictEqual(["selected", "unavailable"]);
    const own = structuredClone(entry.source) as MechanicsResult;
    const unclaimed = retainedPrecisionStandingText(own);
    expect(unclaimed).toContain("receipt not validated for these exact bytes");
    expect(unclaimed).not.toContain("Selected cases");
    await registerRetainedPrecision(own, structuredClone(entry.invocation));
    expect(retainedPrecisionStandingText(own)).toContain("against the actual invocation for these exact bytes. Selected cases: 1 of 2.");
    expect(retainedPrecisionStanding(own, null).findings).toStrictEqual([RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE]);
  });
});

describe.each(MODES)("%s: reopen revalidates the saved statement and never mints standing", (mode) => {
  async function saved(edit?: (envelope: Json) => void): Promise<LocalProjectEnvelope> {
    const { source, model } = milestone(mode);
    const received = await deliverDirect(source, model, mode);
    const manifest = { manifest_ref: { object_type: "InputManifest", ref: "manifest:invented-retained-reopen" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: received.model_ref }, solver_basis: { solver_name: received.producer!.component_name, solver_version: received.producer!.component_version, solver_build_ref: "unit-transport-replay-not-native-witness" } } };
    const analysis_run = await buildAnalysisRunV03(received, manifest, undefined, modelLoadBasisRefs(model));
    // Persistence keeps JSON text (src-tauri mechanics_result_json, analysis_run_json).
    const envelope: Json = JSON.parse(JSON.stringify({ model, mechanics_result: received, analysis_run, model_hash: await computeModelHash(model), editor_intents: [], proposal: null, selected_review_target: null }));
    edit?.(envelope);
    envelope.project_envelope_hash = await computeProjectEnvelopeHash(envelope);
    return envelope as LocalProjectEnvelope;
  }
  it("a saved successor reads needs_recompute; its receipt revalidates and its AnalysisRun copy matches", async () => {
    u7.simulate = true; // even the post-U7 reader cannot mint standing from saved bytes
    const opened = await saved();
    const context = (await buildHistoricalRunContext(opened))!;
    expect(context.mechanicsResult).not.toBeNull();
    expect(context.findings).toStrictEqual(["HISTORICAL_INPUT_MANIFEST_MISSING", RETAINED_PRECISION_VALIDATION_REQUIRED]);
    expect(numericalResultStanding(context.mechanicsResult!, opened.model).eligible).toBe(false);
    expect(hasNativeMechanicsInvocation(context.mechanicsResult, opened.model, mode)).toBe(false);
    expect(retainedPrecisionRegistration(context.mechanicsResult)).toBeNull();
  });
  it("a saved successor without an AnalysisRun is revalidated and reports no copy mismatch", async () => {
    const opened = await saved(envelope => { envelope.analysis_run = null; });
    const findings = (await buildHistoricalRunContext(opened))!.findings;
    expect(findings).toContain(RETAINED_PRECISION_VALIDATION_REQUIRED);
    expect(findings).toContain("HISTORICAL_RUN_EVIDENCE_INCOMPLETE");
    expect(findings).not.toContain(ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH);
  });
  it("a mutated saved row reads unsupported: the reader's own code is a finding", async () => {
    const opened = await saved(envelope => { envelope.mechanics_result.results[0].value = 12345; });
    const findings = (await buildHistoricalRunContext(opened))!.findings;
    expect(findings).toEqual(expect.arrayContaining([RETAINED_PRECISION_VALIDATION_REQUIRED, "RETAINED_PRECISION_RECEIPT_MISMATCH", "HISTORICAL_RESULT_HASH_MISMATCH"]));
  });
  it.each([
    ["altered", (envelope: Json) => { envelope.analysis_run.analysis_run.retained_precision.body.receipt_version = 2; }],
    ["dropped", (envelope: Json) => { delete envelope.analysis_run.analysis_run.retained_precision; }],
  ] as const)("an %s AnalysisRun receipt copy is reported", async (_label, edit) => {
    const findings = (await buildHistoricalRunContext(await saved(edit)))!.findings;
    expect(findings).toContain(ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH);
    expect(findings).toContain("HISTORICAL_ANALYSIS_HASH_MISMATCH");
    expect(findings).not.toContain("RETAINED_PRECISION_RECEIPT_MISMATCH");
  });
});
