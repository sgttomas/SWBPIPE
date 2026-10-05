// RV88 (U6d review): independent checks of the TypeScript successor carriers.
// Candidate lane only (scratch; never committed). Oracles are RV88's own: the
// receipt's selection lists, D2 4.9.9's notice texts, exact BigInt rational
// arithmetic, the shared case file's expectations, and fail-closed properties.
// Facts that are judgements, not pass/fail, are written to RV88_U6D_OUT.
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup } from "@testing-library/react";
import { createHash } from "node:crypto";
import { appendFileSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
// Post-U7 simulation (test-only): the real reader's result, with eligibility set on
// an invocation-bound validation; optionally a hook that runs during validation.
const sim = vi.hoisted(() => ({ eligible: false, during: null as null | (() => void) }));
vi.mock("./retainedPrecision", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./retainedPrecision")>();
  return {
    ...actual,
    validateRetainedPrecision: async (source: unknown, invocation?: unknown) => {
      const validation = await actual.validateRetainedPrecision(source, invocation);
      sim.during?.();
      return sim.eligible && validation.invocation_bound ? Object.freeze({ ...validation, numerical_eligible: true, standing: "eligible" as const }) : validation;
    },
  };
});
import type { LocalProjectEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { numericalResultStanding, sourceContract } from "./numericalResultQuality";
import { decodeBinary64, validateRetainedPrecision } from "./retainedPrecision";
import { retainedPrecisionRegistration, retainedPrecisionStanding, retainedPrecisionStandingText, retainedRowClasses, retainedStandingFrom } from "./retainedPrecisionStanding";
import { N_RP_ABSOLUTE, N_RP_NOT_COVERED, N_RP_UNVALIDATED, RULE_QUANTITY_BELOW_VERIFIED_FLOOR, RULE_QUANTITY_NOT_COVERED, isFreshSemanticResult, knownSemanticNotices, resultRowLabel, ruleBindingRefusal, upwardBoundText } from "./knownSemanticLimitations";
import { loadReferenceOutputRefusal } from "./loadReferenceOutputAvailability";
import { buildHistoricalRunContext } from "./HistoricalRunContext";
import { hasNativeMechanicsInvocation, runPreviewMechanics, type PreviewSolverMode } from "../../services/previewService";
import { ruleBindingPrecheck, runRuleChecks } from "../../services/ruleCheckService";
import { buildAnalysisRunV03, modelLoadBasisRefs, validateAnalysisRunV03 } from "../../services/analysisRunCompatibility";
import { computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import { deriveResultDocument, resultDigest, validateResultDocument } from "../result-export/resultExportAdapter";
import { reportPackageUnavailableReason } from "../report/reportPackageRequest";
import type { RulePackDocument } from "../../services/rulePackService";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const OUT = process.env.RV88_U6D_OUT ?? "";
const fact = (key: string, value: unknown) => { if (OUT) appendFileSync(OUT, `${key}\t${JSON.stringify(value)}\n`); };
const MODES = ["sparse_interactive", "dense_scrutiny"] as const;
const PINS: Record<string, string> = { sparse_interactive: "ac6986b0680e0df9d88c33a5bf4635372fc3b83cbb9080da44e6d32dbdca59dc", dense_scrutiny: "6cd1d249e5352aaffbd2b7d7349c74a1d0e0572df35500f66be49c3cad95c9b5" };
function milestone(mode: string) {
  const bytes = readFileSync(resolve(root, `fixtures/results/retained_precision_milestone_successor_${mode}.json`));
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(PINS[mode]);
  const doc = JSON.parse(bytes.toString("utf8"));
  return { source: doc.source as MechanicsResult, invocation: doc.invocation as Json, model: doc.invocation.request.model as PreviewModel };
}
async function deliver(source: MechanicsResult, model: PreviewModel | null, mode: PreviewSolverMode): Promise<MechanicsResult> {
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async (command: string) => {
    if (command === "run_preview_mechanics_with_solver_mode") return structuredClone(source);
    if (command === "run_rule_checks") return { document_kind: "rule_check_run", aggregate_status: "RULE_INPUTS_INCOMPLETE", checks: [] };
    throw new Error(`unexpected ${command}`);
  });
  return runPreviewMechanics(model, mode);
}
/** A real derive attempt: canonical base document, the source's own model id and carrier digest. */
async function derive(source: Json) {
  const base = JSON.parse(readFileSync(resolve(root, "fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json"), "utf8"));
  const digest = { algorithm: "sha256", canonicalization: "openpipestress_jcs_ijson_v1", payload_scope: "attested_headless_producer_carrier", payload_ref: { ref_type: "test_carrier", ref_id: "rv88" }, value: await resultDigest(source) };
  const origin = { origin_id: "rv88", origin_class: "attested_headless_producer", authentic_producer_available: true, received_carrier_checksum: digest, original_producer_checksum: digest };
  return deriveResultDocument(base, { project: { id: source.model_ref } } as Json, source, origin);
}
const D2_ABSOLUTE = "Uncovered quantity: verified only to an absolute bound of ±{b} {unit}, below the relative accuracy floor for this body. It is shown for inspection; rule checks cannot bind to it.";
const D2_NOT_COVERED = "Uncovered quantity: no verified accuracy for this quantity kind. It is shown for inspection; rule checks cannot bind to it.";
const SI: Record<string, string> = { mm: "m", kN: "N", "kN*m": "N*m", MPa: "Pa" }; // the reader's normalization (RS retained_precision.rs normalized())

beforeEach(() => { sim.eligible = false; sim.during = null; invokeMock.mockReset(); });
afterEach(() => cleanup());

describe.each(MODES)("%s: registration is bound to the exact bytes and the captured invocation", (mode) => {
  it("registers only a direct delivery with a captured model; copies, edits and model-less deliveries never stand", async () => {
    const { source, model } = milestone(mode);
    const received = await deliver(source, model, mode);
    expect(retainedPrecisionRegistration(received)?.validation).toBeTruthy();
    expect(numericalResultStanding(received, model)).toMatchObject({ status: "needs_recompute", eligible: false, findings: ["RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE"] });
    expect(hasNativeMechanicsInvocation(received, model, mode)).toBe(true);
    expect(retainedPrecisionStanding(structuredClone(received), model).findings).toStrictEqual(["RETAINED_PRECISION_VALIDATION_REQUIRED"]);
    const noModel = await deliver(source, null, mode);
    expect(retainedPrecisionStanding(noModel, model).findings).toStrictEqual(["RETAINED_PRECISION_VALIDATION_REQUIRED"]);
    // Every edit voids; reverting restores (the registration is fingerprinted, not frozen).
    const edits: [string, (s: Json) => () => void][] = [
      ["row value next double", s => { const v = s.results[7].value; s.results[7].value = v + Math.abs(v) * Number.EPSILON || Number.MIN_VALUE; return () => { s.results[7].value = v; }; }],
      ["unknown top-level key", s => { s.rv88 = 1; return () => { delete s.rv88; }; }],
      ["receipt field", s => { const v = s.retained_precision.body.work.charged; s.retained_precision.body.work.charged = v + 1; return () => { s.retained_precision.body.work.charged = v; }; }],
      ["numerical_quality status", s => { const v = s.numerical_quality.status; s.numerical_quality.status = "checks_passed"; return () => { s.numerical_quality.status = v; }; }],
      ["zero to negative zero", s => { const i = s.results.findIndex((r: Json) => Object.is(r.value, 0)); const v = s.results[i].value; s.results[i].value = -0; return () => { s.results[i].value = v; }; }],
      ["token removed from one row", s => { const row = structuredClone(s.results[3]); delete s.results[3].recovery_method; return () => { s.results[3] = row; }; }],
    ];
    for (const [label, edit] of edits) {
      const revert = edit(received as Json);
      expect(retainedPrecisionStanding(received, model).findings, label).toStrictEqual(["RETAINED_PRECISION_VALIDATION_REQUIRED"]);
      expect(ruleBindingRefusal(received, received.results[0]), label).toBe(RULE_QUANTITY_NOT_COVERED);
      revert();
      expect(retainedPrecisionStanding(received, model).findings, `${label} reverted`).toStrictEqual(["RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE"]);
    }
    // Key order alone: the fingerprint is checked JSON text, so a reordered copy of the same object is the same bytes.
    const r = received as Json; const status = r.status; delete r.status; r.status = status;
    fact(`${mode}/key_reorder_keeps_registration`, retainedPrecisionStanding(received, model).findings);
    // A refused delivery records the reader's code and never registers natively.
    const edited = structuredClone(source); edited.results[0].value = 12345;
    const refused = await deliver(edited, model, mode);
    expect(numericalResultStanding(refused, model)).toMatchObject({ status: "needs_recompute", eligible: false });
    const st = retainedPrecisionStanding(refused, model);
    expect(st.standing).toBe("unsupported");
    expect(st.findings).toStrictEqual(["RETAINED_PRECISION_RECEIPT_MISMATCH"]);
    expect(hasNativeMechanicsInvocation(refused, model, mode)).toBe(false);
    fact(`${mode}/refused_delivery_numericalResultStanding`, numericalResultStanding(refused, model));
    fact(`${mode}/refused_delivery_standing_text`, retainedPrecisionStandingText(refused));
  });

  it("standing never reads numerical_quality (seam), and a foreign mode is refused", async () => {
    const { source, model, invocation } = milestone(mode);
    const refs = model.load_cases.map(c => ({ ref_type: "load_case", ref_id: c.id }));
    const eligible = { invocation_bound: true, numerical_eligible: true };
    expect(retainedStandingFrom(eligible, source, refs)).toBe("numerically_eligible");
    for (const nq of [{}, null, { status: "checks_passed", cases: [] }, { ...source.numerical_quality, status: "failed" }]) {
      const s = { ...structuredClone(source), numerical_quality: nq } as Json;
      expect(retainedStandingFrom(eligible, s, refs)).toBe("numerically_eligible");
      expect(retainedStandingFrom({ invocation_bound: true, numerical_eligible: false }, s, refs)).toBe("needs_recompute");
    }
    // Case order: a two-case receipt with the requested refs reversed is not eligible.
    const two = structuredClone(source) as Json;
    two.retained_precision.body.cases.push({ ...two.retained_precision.body.cases[0], basis_ref: { ref_type: "load_case", ref_id: "second" } });
    const forward = [...refs, { ref_type: "load_case", ref_id: "second" }];
    expect(retainedStandingFrom(eligible, two, forward)).toBe("numerically_eligible");
    expect(retainedStandingFrom(eligible, two, [...forward].reverse())).toBe("needs_recompute");
    const other: PreviewSolverMode = mode === "sparse_interactive" ? "dense_scrutiny" : "sparse_interactive";
    const foreign = await deliver(source, model, other);
    expect(retainedPrecisionStanding(foreign, model)).toMatchObject({ standing: "unsupported", findings: ["RETAINED_PRECISION_INVOCATION_MISMATCH"] });
    void invocation;
  });

  it("post-U7 (simulated): standing is not bound to the current model the way source-block standing is", async () => {
    const { source, model } = milestone(mode);
    sim.eligible = true;
    const received = await deliver(source, model, mode);
    const eligible = retainedPrecisionStanding(received, model);
    fact(`${mode}/u7sim/standing_same_model`, eligible);
    // The user edits the model (same load case ids) after the solve.
    const changed = structuredClone(model) as Json;
    const node = changed.nodes.find((n: Json) => n.position && typeof n.position.x === "number");
    node.position.x += 1;
    fact(`${mode}/u7sim/standing_changed_model`, retainedPrecisionStanding(received, changed));
    fact(`${mode}/u7sim/hasNative_changed_model`, hasNativeMechanicsInvocation(received, changed, mode));
    fact(`${mode}/u7sim/numericalResultStanding_changed_model`, numericalResultStanding(received, changed));
    let gate: string;
    try { await runRuleChecks({ rulePackDocument: {} as RulePackDocument, model: changed, solvedEnvelope: received }); gate = "reached backend"; } catch (e) { gate = (e as Error).message.split(":")[0]; }
    fact(`${mode}/u7sim/rule_gate_changed_model`, gate);
    // The caller's model object changes while the reader validates: the native capture
    // is not registered, but the retained registration is.
    const callerModel = structuredClone(model) as Json;
    sim.during = () => { callerModel.project.name = `${callerModel.project.name ?? "m"}-edited`; sim.during = null; };
    const raced = await deliver(source, callerModel, mode);
    fact(`${mode}/u7sim/raced_hasNative`, hasNativeMechanicsInvocation(raced, callerModel, mode));
    fact(`${mode}/u7sim/raced_retained_standing`, retainedPrecisionStanding(raced, callerModel));
    let raceGate: string;
    try { await runRuleChecks({ rulePackDocument: {} as RulePackDocument, model: callerModel, solvedEnvelope: raced }); raceGate = "reached backend"; } catch (e) { raceGate = (e as Error).message.split(":")[0]; }
    fact(`${mode}/u7sim/raced_rule_gate`, raceGate);
  });
});

describe("the 14 shared carrier cases, by RV88's own TS mapping", () => {
  it("each case's TS standing and dispatch equals the shared expectation (which Rust U6a asserts)", async () => {
    const file = JSON.parse(readFileSync(resolve(root, "fixtures/results/retained_precision_carrier_cases.json"), "utf8"));
    let n = 0;
    for (const c of file.cases) {
      const mode = file.fixtures[c.fixture].path.includes("sparse") ? "sparse_interactive" : "dense_scrutiny";
      const { source, invocation } = milestone(mode);
      const s = structuredClone(source) as Json, inv = structuredClone(invocation);
      for (const e of c.edits) { let at = e.target === "source" ? s : inv; for (const k of e.path.slice(0, -1)) at = at[k]; at[e.path[e.path.length - 1]] = e.value; }
      const requestedModel = { ...inv.request.model, load_cases: c.requested === "invocation" ? inv.request.model.load_cases : c.requested.map((r: Json) => ({ ...inv.request.model.load_cases[0], id: r.ref_id })) };
      const received = c.invocation === null ? await deliver(s, null, inv.solver_mode) : await deliver(s, inv.request.model, inv.solver_mode).catch(() => s);
      const st = retainedPrecisionStanding(received, requestedModel);
      const standing = sourceContract(received) === "unsupported" ? "unsupported" : st.standing;
      let dispatch = "ok";
      if (sourceContract(received) === "unsupported") dispatch = numericalResultStanding(received, requestedModel).findings[0];
      else { try { await validateRetainedPrecision(received); } catch (e) { dispatch = (e as Error).message; } }
      fact(`case/${c.id}`, { standing, dispatch, findings: st.findings, expected: [c.expected_standing, c.expected_dispatch] });
      expect([standing, dispatch], c.id).toStrictEqual([c.expected_standing, c.expected_dispatch]);
      n += 1;
    }
    expect(n).toBe(14);
  });
});

describe.each(MODES)("%s: downgrade guards and refusal codes", (mode) => {
  it("every relabel form is unsupported, not fresh, and refused by every builder and output", async () => {
    const { source, model } = milestone(mode);
    const ids = ["openpipestress.result_semantics/0.3.0/preview-physics-1", "openpipestress.result_semantics/0.3.0/physics-1", "openpipestress.result_semantics/0.3.0/physics-source-1",
      "openpipestress.result_semantics/0.3.0/precision-1", "openpipestress.result_semantics/0.3.0/load-reference-1", "openpipestress.result_semantics/0.3.0/load-reference-source-1",
      "openpipestress.result_semantics/0.3.0/source-blocks-1", "openpipestress.result_semantics/0.3.0/unknown-rv88"];
    const tally: Record<string, number> = {};
    for (const id of ids) for (const keepProfile of [false, true]) {
      const base = structuredClone(source) as Json;
      base.producer.semantic_contract_id = id;
      if (!keepProfile) base.formulation_basis.profile_id = "product_preview_mechanics_v1";
      const projection = (() => { const p = structuredClone(base); delete p.retained_precision; for (const r of p.results) delete r.recovery_method; return p; })();
      const forms: [string, Json][] = [
        ["receipt", base], ["null", { ...structuredClone(base), retained_precision: null }], ["empty", { ...structuredClone(base), retained_precision: {} }],
        ["tokens_only", (() => { const p = structuredClone(base); delete p.retained_precision; return p; })()],
        ["one_token_last", (() => { const p = structuredClone(projection); p.results[p.results.length - 1].recovery_method = "contribution_preserving_multiprecision_v1"; return p; })()],
        ["legacy_with_receipt", (() => { const p = structuredClone(projection); p.schema_version = "0.1.0"; for (const k of ["producer", "numerical_quality", "formulation_basis", "contract_evidence"]) delete p[k]; p.retained_precision = structuredClone(source.retained_precision); return p; })()],
      ];
      for (const [form, s] of forms) {
        expect(sourceContract(s), `${id} ${form}`).toBe("unsupported");
        expect(isFreshSemanticResult(s)).toBe(false);
        const finding = numericalResultStanding(s, model).findings[0];
        tally[finding] = (tally[finding] ?? 0) + 1;
        expect(finding).toBe("RETAINED_PRECISION_DOWNGRADE_FORBIDDEN");
        await expect(buildAnalysisRunV03(s, { manifest_ref: { object_type: "InputManifest", ref: "m" }, manifest_sha256: "3".repeat(64), manifest: { model_basis: { model_ref: s.model_ref }, solver_basis: { solver_name: "x", solver_version: "x", solver_build_ref: "rv88" } } } as Json)).rejects.toThrow();
        await expect(derive(s)).rejects.toThrow("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
      }
    }
    fact(`${mode}/relabel_findings`, tally);
  });

  it("AnalysisRun copies the receipt whole and refuses every mismatch; reopen revalidates; outputs refuse", async () => {
    const { source, model } = milestone(mode);
    const received = await deliver(source, model, mode);
    const manifest = { manifest_ref: { object_type: "InputManifest", ref: "manifest:rv88" }, manifest_sha256: "4".repeat(64), manifest: { model_basis: { model_ref: received.model_ref }, solver_basis: { solver_name: received.producer!.component_name, solver_version: received.producer!.component_version, solver_build_ref: "rv88-unit-replay" } } };
    const refs = modelLoadBasisRefs(model);
    const record = await buildAnalysisRunV03(received, manifest as Json, undefined, refs);
    expect(JSON.stringify((record.analysis_run as Json).retained_precision)).toBe(JSON.stringify(source.retained_precision));
    await validateAnalysisRunV03(record, received, refs);
    const other = milestone(mode === "sparse_interactive" ? "dense_scrutiny" : "sparse_interactive").source;
    const mutations: [string, (r: Json) => void][] = [
      ["dropped", r => { delete r.analysis_run.retained_precision; }],
      ["null", r => { r.analysis_run.retained_precision = null; }],
      ["sha", r => { r.analysis_run.retained_precision.receipt_sha256 = "0".repeat(64); }],
      ["body", r => { r.analysis_run.retained_precision.body.work.charged += 1; }],
      ["extra key", r => { r.analysis_run.retained_precision.rv88 = true; }],
      ["other mode", r => { r.analysis_run.retained_precision = structuredClone(other.retained_precision); }],
    ];
    for (const [label, m] of mutations) {
      const r = structuredClone(record) as Json; m(r);
      await expect(validateAnalysisRunV03(r, received, refs), label).rejects.toThrow("ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH");
    }
    // A base (preview-physics-1) record carrying a receipt.
    const projection = structuredClone(source) as Json; delete projection.retained_precision; projection.producer.semantic_contract_id = "openpipestress.result_semantics/0.3.0/preview-physics-1"; projection.formulation_basis.profile_id = "product_preview_mechanics_v1"; for (const row of projection.results) delete row.recovery_method;
    let base: Json = null;
    try { base = await buildAnalysisRunV03(projection, manifest as Json, undefined, refs); } catch (e) { fact(`${mode}/projection_record_build`, (e as Error).message); }
    if (base) {
      const withReceipt = structuredClone(base); withReceipt.analysis_run.retained_precision = structuredClone(source.retained_precision);
      await expect(validateAnalysisRunV03(withReceipt, projection, refs)).rejects.toThrow("ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN");
      const withNull = structuredClone(base); withNull.analysis_run.retained_precision = null;
      await expect(validateAnalysisRunV03(withNull, projection, refs)).rejects.toThrow("ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN");
    }
    // Reopen: saved JSON text; nothing registers; the reader's code is a finding.
    async function saved(edit?: (e: Json) => void): Promise<LocalProjectEnvelope> {
      const envelope: Json = JSON.parse(JSON.stringify({ model, mechanics_result: received, analysis_run: record, model_hash: await computeModelHash(model), editor_intents: [], proposal: null, selected_review_target: null }));
      edit?.(envelope); envelope.project_envelope_hash = await computeProjectEnvelopeHash(envelope); return envelope;
    }
    sim.eligible = true;
    const plain = (await buildHistoricalRunContext(await saved()))!;
    fact(`${mode}/reopen_plain_findings`, plain.findings);
    expect(numericalResultStanding(plain.mechanicsResult!, model).eligible).toBe(false);
    const bad = (await buildHistoricalRunContext(await saved(e => { e.mechanics_result.results[0].value = 12345; })))!;
    fact(`${mode}/reopen_mutated_findings`, bad.findings);
    fact(`${mode}/reopen_mutated_standing`, numericalResultStanding(bad.mechanicsResult!, model));
    expect(bad.findings).toContain("RETAINED_PRECISION_RECEIPT_MISMATCH");
    const copy = (await buildHistoricalRunContext(await saved(e => { e.analysis_run.analysis_run.retained_precision.body.work.charged += 1; })))!;
    expect(copy.findings).toContain("ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH");
    sim.eligible = false;
    // The rule-check gate refuses before any backend call while eligibility is held.
    invokeMock.mockClear();
    await expect(runRuleChecks({ rulePackDocument: {} as RulePackDocument, model, solvedEnvelope: received })).rejects.toThrow("RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE");
    expect(invokeMock.mock.calls.filter(c => c[0] === "run_rule_checks")).toHaveLength(0);
    // T6 outputs refuse through the shared function; the report package through its fresh refusal.
    expect(loadReferenceOutputRefusal(received)).toMatch(/^RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE: /);
    await expect(derive(received)).rejects.toThrow("RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE");
    await expect(validateResultDocument({ result_envelope: {} } as Json, received)).rejects.toThrow("RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE");
    expect(reportPackageUnavailableReason(received)).toMatch(/^REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE/);
  });
});

// Exact rationals: value = num / den with BigInt.
function doubleExact(x: number): [bigint, bigint] {
  const view = new DataView(new ArrayBuffer(8)); view.setFloat64(0, x);
  const bits = view.getBigUint64(0); const e = Number((bits >> 52n) & 0x7ffn); let m = bits & ((1n << 52n) - 1n);
  let exp: number; if (e === 0) exp = -1074; else { m |= 1n << 52n; exp = e - 1075; }
  return exp >= 0 ? [m << BigInt(exp), 1n] : [m, 1n << BigInt(-exp)];
}
function textExact(t: string): [bigint, bigint] {
  const [mant, ex] = t.split("e"); const p = Number(ex ?? 0); const [i, f = ""] = mant.split(".");
  const digits = BigInt(i + f); const shift = p - f.length;
  return shift >= 0 ? [digits * 10n ** BigInt(shift), 1n] : [digits, 10n ** BigInt(-shift)];
}
const geq = (a: [bigint, bigint], b: [bigint, bigint]) => a[0] * b[1] >= b[0] * a[1];
/** The least 3-significant-digit decimal >= x (x > 0), as [digits, power] meaning digits*10^power. */
function ceil3(x: number): [bigint, number] {
  const [n, d] = doubleExact(x);
  let p = Math.floor(Math.log10(x)) - 2;
  for (;;) {
    const [sn, sd] = p >= 0 ? [n, d * 10n ** BigInt(p)] : [n * 10n ** BigInt(-p), d];
    if (sn > 1000n * sd) { p += 1; continue; }
    if (sn < 100n * sd) { p -= 1; continue; }
    const c = (sn + sd - 1n) / sd;
    return c === 1000n ? [100n, p + 1] : [c, p];
  }
}

describe("F6: new product text claims nothing beyond the receipt's bound", () => {
  it("N_RP_ABSOLUTE and N_RP_NOT_COVERED are D2 4.9.9's texts verbatim", () => {
    expect(N_RP_ABSOLUTE).toBe(D2_ABSOLUTE);
    expect(N_RP_NOT_COVERED).toBe(D2_NOT_COVERED);
    fact("text/N_RP_UNVALIDATED", N_RP_UNVALIDATED);
  });
  it("upwardBoundText is never below b, and is the least 3-significant-digit decimal >= b or one step above it", () => {
    const view = new DataView(new ArrayBuffer(8));
    let seed = 0x9e3779b97f4a7c15n;
    const next = () => { seed ^= seed << 13n; seed &= (1n << 64n) - 1n; seed ^= seed >> 7n; seed ^= seed << 17n; seed &= (1n << 64n) - 1n; return seed; };
    const samples: number[] = [Number.MIN_VALUE, 2 ** -1074 * 3, 1e-300, 9.995e-5, 9.994e-5, 9.9949999e-5, 1.25e-5, 1, 0.125, 999.5, 9.999e300, Number.MAX_VALUE, 5.421552765963047e-24, 2.6020852139652104e-22];
    for (let i = 0; i < 20000; i += 1) { view.setBigUint64(0, next() & 0x7fefffffffffffffn); const x = view.getFloat64(0); if (x > 0 && Number.isFinite(x)) samples.push(x); }
    let least = 0, oneAbove = 0, worse = 0; const odd: unknown[] = [];
    for (const b of samples) {
      const t = upwardBoundText(b);
      expect(geq(textExact(t), doubleExact(b)), `${b} -> ${t}`).toBe(true);
      const [c, p] = ceil3(b);
      const [tn, td] = textExact(t);
      const cExact: [bigint, bigint] = p >= 0 ? [c * 10n ** BigInt(p), 1n] : [c, 10n ** BigInt(-p)];
      if (tn * cExact[1] === cExact[0] * td) least += 1;
      else { const up: [bigint, bigint] = p >= 0 ? [(c + 1n) * 10n ** BigInt(p), 1n] : [c + 1n, 10n ** BigInt(-p)]; if (tn * up[1] === up[0] * td || (c === 999n)) oneAbove += 1; else { worse += 1; if (odd.length < 5) odd.push([b, t]); } }
    }
    fact("text/upwardBoundText", { samples: samples.length, least, oneAbove, worse, odd });
    expect(worse).toBe(0);
  });
  it.each(MODES)("%s: every absolute row's label prints the receipt's bound upward in the reader's SI unit", async (mode) => {
    const { source, model } = milestone(mode);
    const received = await deliver(source, model, mode);
    const listed = new Map((source as Json).retained_precision.body.cases[0].selection.absolute_verified.map((x: Json) => [x.result_id, x.bound]));
    const classes = retainedRowClasses(received)!;
    let n = 0; const units: Record<string, number> = {};
    for (const row of received.results) {
      const label = resultRowLabel(row, received);
      if (listed.has(row.id)) {
        n += 1;
        expect(classes.get(row.id)!.class).toBe("absolute_verified");
        const b = decodeBinary64(listed.get(row.id) as string);
        const unit = SI[row.unit] ?? row.unit;
        units[`${row.unit}->${unit}`] = (units[`${row.unit}->${unit}`] ?? 0) + 1;
        expect(label).toContain(D2_ABSOLUTE.replace("{b}", upwardBoundText(b)).replace("{unit}", unit));
        expect(label).not.toMatch(/stop|enclos|interval/i);
        expect(ruleBindingRefusal(received, row)).toBe(RULE_QUANTITY_BELOW_VERIFIED_FLOOR);
      } else {
        expect(label ?? "").not.toContain("Uncovered quantity");
      }
    }
    expect(n).toBe(69);
    fact(`${mode}/label_units`, units);
    fact(`${mode}/notices_registered`, knownSemanticNotices(received).map(x => [x.id, x.text]));
    fact(`${mode}/notices_unregistered`, knownSemanticNotices(structuredClone(received)).map(x => [x.id, x.text]));
    fact(`${mode}/standing_text_registered`, retainedPrecisionStandingText(received));
    fact(`${mode}/standing_text_unregistered`, retainedPrecisionStandingText(structuredClone(received)));
    fact(`${mode}/output_refusal_text`, loadReferenceOutputRefusal(received));
    // F2: the display-only precheck.
    const plan = { solverInputs: received.results.map(r => ({ input_id: `in:${r.id}`, solver_result_ref: { result_id: r.id } })), valueInputs: [], valueSlots: [], libraryInputs: [] };
    const reg = ruleBindingPrecheck(received, plan as Json);
    const unreg = ruleBindingPrecheck(structuredClone(received), plan as Json);
    fact(`${mode}/precheck_registered`, { refused: (reg as Json).length ?? reg, reasons: [...new Set((Array.isArray(reg) ? reg : (reg as Json).findings ?? []).map((f: Json) => f.reason))] });
    fact(`${mode}/precheck_unregistered`, { refused: (unreg as Json).length ?? unreg, notices: [...new Set((Array.isArray(unreg) ? unreg : (unreg as Json).findings ?? []).map((f: Json) => f.notice))] });
  });
});
