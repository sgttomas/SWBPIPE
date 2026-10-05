/** RV91 (independent review of U6d): reviewer-written probes. Scratch only:
 * copied into the candidate lane, run, removed. Never committed.
 * Observations are written to RV91_REVIEW_OUT as JSON (one object per probe);
 * hard expectations are the reviewer's own reading of plan 3/4, D2 4.7/4.9. */
import { afterAll, afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, render } from "@testing-library/react";
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
const u7 = vi.hoisted(() => ({ simulate: false }));
vi.mock("./retainedPrecision", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./retainedPrecision")>();
  return {
    ...actual,
    validateRetainedPrecision: async (source: unknown, invocation?: unknown) => {
      const v = await actual.validateRetainedPrecision(source, invocation);
      return u7.simulate && v.invocation_bound ? Object.freeze({ ...v, numerical_eligible: true, standing: "eligible" as const }) : v;
    },
  };
});
import * as nrq from "./numericalResultQuality";
import * as rps from "./retainedPrecisionStanding";
import * as ksl from "./knownSemanticLimitations";
import * as lro from "./loadReferenceOutputAvailability";
import { decodeBinary64, validateRetainedPrecision } from "./retainedPrecision";
import { ResultsPanel } from "./ResultsPanel";
import { buildHistoricalRunContext } from "./HistoricalRunContext";
import * as ps from "../../services/previewService";
import * as arc from "../../services/analysisRunCompatibility";
import * as rcs from "../../services/ruleCheckService";
import { canonicalSha256HexCheckedV1, computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const OUT = process.env.RV91_REVIEW_OUT!;
const root = resolve(__dirname, "../../../../../");
const caseFile = JSON.parse(readFileSync(resolve(root, "fixtures/results/retained_precision_carrier_cases.json"), "utf8"));
const extraCases = JSON.parse(readFileSync(process.env.RV91_EXTRA_CASES!, "utf8"));
const obs: Record<string, unknown> = {};
afterAll(() => writeFileSync(OUT, JSON.stringify(obs, null, 1)));
const MODES = ["sparse_interactive", "dense_scrutiny"] as const;
type Mode = typeof MODES[number];
function fixture(mode: Mode) {
  const spec = caseFile.fixtures[`milestone_${mode}`];
  const bytes = readFileSync(resolve(root, spec.path));
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(spec.sha256);
  const doc = JSON.parse(bytes.toString("utf8"));
  return { source: doc.source as Json, invocation: doc.invocation as Json };
}
async function direct(source: Json, model: Json, mode: string) {
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async (command: string) => { if (command !== "run_preview_mechanics_with_solver_mode") throw new Error(`unexpected ${command}`); return structuredClone(source); });
  return ps.runPreviewMechanics(model, mode as Mode);
}
const withCases = (model: Json, ids: string[]) => ({ ...model, load_cases: ids.map(id => ({ ...model.load_cases[0], id })) });
function edit(target: Json, path: (string | number)[], op: string, value?: unknown) {
  let at = target; for (const k of path.slice(0, -1)) at = at[k];
  if (op === "set") at[path[path.length - 1]] = value; else if (op === "delete") delete at[path[path.length - 1]]; else throw new Error(op);
}
beforeEach(() => { u7.simulate = false; });
afterEach(() => { cleanup(); invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__; u7.simulate = false; });

describe("RV91 parity: the 14 shared cases and RV91's extra cases, TS side", () => {
  it("records the TS standing token and dispatch for every case", async () => {
    const rows: Json[] = [];
    for (const c of [...caseFile.cases, ...extraCases]) {
      const mode: Mode = c.fixture === "milestone_sparse_interactive" ? "sparse_interactive" : "dense_scrutiny";
      const { source, invocation } = fixture(mode);
      for (const e of c.edits) edit(e.target === "source" ? source : invocation, e.path, e.op, e.value);
      const model = invocation.request.model;
      const standingModel = c.requested === "invocation" ? fixture(mode).invocation.request.model : withCases(model, c.requested.map((r: Json) => r.ref_id));
      // invocation "fixture": captured through mocked direct IPC with the (edited) invocation; null: delivered without a captured model.
      const delivered = c.invocation === null ? structuredClone(source) : await direct(source, model, invocation.solver_mode);
      invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__;
      const route = nrq.sourceContract(delivered);
      const standing = route === "unsupported" ? "unsupported" : route === "retained_preview_physics" ? rps.retainedPrecisionStanding(delivered, standingModel).standing : `route:${route}`;
      let dispatch: string;
      if (route === "unsupported") dispatch = nrq.numericalResultStanding(delivered).findings[0];
      else { try { await validateRetainedPrecision(delivered); dispatch = "ok"; } catch (error) { dispatch = (error as Error).message; } }
      const ns = nrq.numericalResultStanding(delivered, standingModel);
      rows.push({ id: c.id, standing, dispatch, ts_status: ns.status, ts_findings: ns.findings, expected_standing: c.expected_standing ?? null, expected_dispatch: c.expected_dispatch ?? null });
    }
    obs.parity = rows;
    for (const r of rows.slice(0, 14)) { expect(r.standing, r.id).toBe(r.expected_standing); expect(r.dispatch, r.id).toBe(r.expected_dispatch); }
  });
});

describe("RV91 registration binding", () => {
  it("binds the exact bytes; every edit voids it; a copy never registers; revert restores", async () => {
    const { source, invocation } = fixture("sparse_interactive");
    const model = invocation.request.model;
    const got = await direct(source, model, "sparse_interactive");
    expect(rps.retainedPrecisionRegistration(got)?.validation?.invocation_bound).toBe(true);
    expect(ps.hasNativeMechanicsInvocation(got, model, "sparse_interactive")).toBe(true);
    const voids: Record<string, unknown> = {};
    const probes: [string, (s: Json) => () => void][] = [
      ["row value", s => { const v = s.results[3].value; s.results[3].value = v + 1; return () => { s.results[3].value = v; }; }],
      ["row value sign of zero", s => { const i = s.results.findIndex((r: Json) => r.value === 0 && !Object.is(r.value, -0)); if (i < 0) return () => {}; s.results[i].value = -0; return () => { s.results[i].value = 0; }; }],
      ["new top-level member", s => { s.rv91 = 1; return () => { delete s.rv91; }; }],
      ["numerical_quality.status", s => { const v = s.numerical_quality.status; s.numerical_quality.status = v === "checks_passed" ? "failed" : "checks_passed"; return () => { s.numerical_quality.status = v; }; }],
      ["receipt body member", s => { const v = s.retained_precision.body.receipt_version; s.retained_precision.body.receipt_version = 99; return () => { s.retained_precision.body.receipt_version = v; }; }],
      ["row method token removed", s => { const v = s.results[0].recovery_method; delete s.results[0].recovery_method; return () => { s.results[0].recovery_method = v; }; }],
      ["diagnostic edited", s => { const v = s.diagnostics[0].message; s.diagnostics[0].message = `${v}!`; return () => { s.diagnostics[0].message = v; }; }],
      ["contract_evidence edited", s => { const k = Object.keys(s.contract_evidence)[0]; const v = s.contract_evidence[k]; s.contract_evidence[k] = null; return () => { s.contract_evidence[k] = v; }; }],
    ];
    for (const [label, mutate] of probes) {
      const revert = mutate(got);
      const after = { reg: rps.retainedPrecisionRegistration(got), standing: nrq.numericalResultStanding(got, model).findings, native: ps.hasNativeMechanicsInvocation(got, model) };
      revert();
      const reverted = { reg: !!rps.retainedPrecisionRegistration(got), standing: nrq.numericalResultStanding(got, model).findings };
      voids[label] = { voided: after.reg === null, standing: after.standing, native: after.native, reverted };
      expect(after.reg, label).toBeNull();
      expect(after.standing, label).toEqual([rps.RETAINED_PRECISION_VALIDATION_REQUIRED]);
    }
    obs.registration_voids = voids;
    const copy = structuredClone(got);
    expect(rps.retainedPrecisionRegistration(copy)).toBeNull();
    expect(nrq.numericalResultStanding(copy, model).findings).toEqual([rps.RETAINED_PRECISION_VALIDATION_REQUIRED]);
    // Key order: same JSON content, different member order.
    const reordered = Object.fromEntries(Object.entries(got).reverse());
    obs.registration_key_reorder_object_is_new = rps.retainedPrecisionRegistration(reordered) === null;
    // Direct registration call on bytes, then the caller mutates during the await.
    const fresh = structuredClone(source);
    const pending = rps.registerRetainedPrecision(fresh, structuredClone(invocation));
    fresh.results[3].value += 1; // edited while the reader awaits
    let outcome: string;
    try { await pending; outcome = "validated"; } catch (e) { outcome = (e as Error).message; }
    const whileEdited = rps.retainedPrecisionRegistration(fresh);
    fresh.results[3].value -= 1;
    const afterRevert = rps.retainedPrecisionRegistration(fresh);
    obs.registration_edit_during_await = { outcome, while_edited: whileEdited === null ? null : "registered", after_revert: afterRevert === null ? null : (afterRevert.error ?? "validation") };
    expect(outcome).toBe("validated"); // reader snapshots synchronously
    expect(whileEdited).toBeNull();
    expect(afterRevert?.validation?.invocation_bound).toBe(true); // the validation is of exactly these (reverted) bytes
  });

  it("standing never reads numerical_quality for a selected case (seam), and only the registration (post-U7 simulation)", async () => {
    const { source, invocation } = fixture("dense_scrutiny");
    const refs = invocation.request.model.load_cases.map((c: Json) => ({ ref_type: "load_case", ref_id: c.id }));
    const bad = structuredClone(source);
    bad.numerical_quality.status = "failed";
    for (const q of bad.numerical_quality.cases) { q.solve_quality = "failed"; q.structural_status = "rv91"; q.evidence_refs = []; }
    const v = { invocation_bound: true, numerical_eligible: true };
    expect(rps.retainedStandingFrom(v, bad, refs)).toBe("numerically_eligible");
    expect(rps.retainedStandingFrom({ ...v, invocation_bound: false }, source, refs)).toBe("needs_recompute");
    // Post-U7 simulation through the product path.
    u7.simulate = true;
    const got = await direct(source, invocation.request.model, "dense_scrutiny");
    const eligible = nrq.numericalResultStanding(got, invocation.request.model);
    expect(eligible.eligible).toBe(true);
    // Model divergence: same load-case ids, a different model (a moved node). Does successor standing bind the model?
    const moved = structuredClone(invocation.request.model);
    moved.nodes[0].x = (moved.nodes[0].x ?? 0) + 1;
    const diverged = nrq.numericalResultStanding(got, moved);
    obs.model_divergence_post_u7 = {
      successor_standing_eligible_with_other_model: diverged.eligible,
      native_invocation_with_other_model: ps.hasNativeMechanicsInvocation(got, moved),
      rule_check_gate: await rcs.runRuleChecks({ rulePackDocument: {} as Json, model: moved, solvedEnvelope: got }).then(() => "ran", e => (e as Error).message.split(":")[0]),
    };
    u7.simulate = false;
  });
});

describe("RV91 AnalysisRun, reopen and the gate", () => {
  const manifestFor = (s: Json) => ({ manifest_ref: { object_type: "InputManifest", ref: "manifest:rv91" }, manifest_sha256: "2".repeat(64), manifest: { model_basis: { model_ref: s.model_ref }, solver_basis: { solver_name: s.producer?.component_name ?? "x", solver_version: s.producer?.component_version ?? "0", solver_build_ref: "rv91" } } });
  it.each(MODES)("%s: builds, copies whole, validates equality; every receipt change is refused with the ruled code", async (mode) => {
    const { source, invocation } = fixture(mode);
    const model = invocation.request.model;
    const record = await arc.buildAnalysisRunV03(structuredClone(source), manifestFor(source) as Json, undefined, arc.modelLoadBasisRefs(model));
    expect(JSON.stringify(record.analysis_run.retained_precision)).toBe(JSON.stringify(source.retained_precision));
    expect(Object.hasOwn(record.analysis_run, "contract_evidence")).toBe(false);
    expect(Object.hasOwn(record.analysis_run, "source_block_recovery")).toBe(false);
    writeFileSync(`${OUT}.analysis_run_${mode}.json`, JSON.stringify(record));
    const codes: Record<string, string> = {};
    const tryValidate = async (label: string, r: Json, s: Json = source) => { try { await arc.validateAnalysisRunV03(r, s, arc.modelLoadBasisRefs(model)); codes[label] = "ok"; } catch (e) { codes[label] = (e as Error).message; } };
    await tryValidate("unchanged", record);
    const dropped = structuredClone(record); delete dropped.analysis_run.retained_precision; await tryValidate("dropped", dropped);
    const nulled = structuredClone(record); (nulled.analysis_run as Json).retained_precision = null; await tryValidate("null", nulled);
    const altered = structuredClone(record); (altered.analysis_run.retained_precision as Json).receipt_sha256 = "0".repeat(64); await tryValidate("receipt_sha256 altered", altered);
    const body = structuredClone(record); (body.analysis_run.retained_precision as Json).body.cases[0].selection.floor = "3ff0000000000000"; await tryValidate("body altered", body);
    const extra = structuredClone(record); (extra.analysis_run.retained_precision as Json).rv91 = true; await tryValidate("extra member", extra);
    const reordered = structuredClone(record); (reordered.analysis_run as Json).retained_precision = Object.fromEntries(Object.entries(record.analysis_run.retained_precision as Json).reverse()); await tryValidate("key order reversed", reordered);
    // A record of another identity carrying a receipt (validated against the reader's own projection).
    const base = structuredClone(source); delete base.retained_precision; base.producer.semantic_contract_id = nrq.PREVIEW_PHYSICS_CONTRACT_ID; base.formulation_basis.profile_id = "product_preview_mechanics_v1";
    for (const r of base.results) delete r.recovery_method;
    let baseRecord: Json = null;
    try { baseRecord = await arc.buildAnalysisRunV03(structuredClone(base), manifestFor(base) as Json, undefined, arc.modelLoadBasisRefs(model)); } catch (e) { codes["projection build"] = (e as Error).message; }
    if (baseRecord) { baseRecord.analysis_run.retained_precision = structuredClone(source.retained_precision); await tryValidate("receipt on preview-physics-1 record", baseRecord, base); }
    // Validate a successor record against an edited source.
    const editedSource = structuredClone(source); editedSource.results[0].value = 12345; await tryValidate("edited source", record, editedSource);
    obs[`analysis_run_${mode}`] = codes;
    expect(codes.unchanged).toBe("ok");
    expect(codes["key order reversed"]).toBe("ok");
    for (const k of ["dropped", "null", "receipt_sha256 altered", "body altered", "extra member"]) expect(codes[k], k).toBe(arc.ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH);
    if (baseRecord) expect(codes["receipt on preview-physics-1 record"]).toBe(arc.ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN);
    expect(codes["edited source"]).toBe("RETAINED_PRECISION_RECEIPT_MISMATCH");
  });

  it("I66 F-U6b-3: the TS v0.2 (legacy) builder and a source carrying retained_precision", async () => {
    const { source } = fixture("sparse_interactive");
    const out: Record<string, unknown> = {};
    // A legacy-shaped source (no producer, quality, formulation or evidence) carrying the receipt member.
    const legacy = structuredClone(source); legacy.schema_version = "0.1.0";
    for (const k of ["producer", "numerical_quality", "formulation_basis", "contract_evidence"]) delete legacy[k];
    for (const r of legacy.results) delete r.recovery_method;
    out.legacy_route = nrq.sourceContract(legacy);
    try {
      const r = await arc.buildAnalysisRunV02(structuredClone(legacy), manifestFor(legacy) as Json);
      out.v02_built = true; out.v02_record_has_receipt = Object.hasOwn(r.analysis_run, "retained_precision");
      out.v02_received_result_hash_binds_receipt = r.analysis_run.hashes.find(h => h.payload_scope === "received_result")!.value === await canonicalSha256HexCheckedV1(legacy);
    } catch (e) { out.v02_built = (e as Error).message; }
    // The same with retained_precision: null, and with only token rows.
    const nulled = structuredClone(legacy); nulled.retained_precision = null;
    try { const r = await arc.buildAnalysisRunV02(nulled, manifestFor(nulled) as Json); out.v02_null_member_built = !Object.hasOwn(r.analysis_run, "retained_precision"); } catch (e) { out.v02_null_member_built = (e as Error).message; }
    const tokens = structuredClone(legacy); delete tokens.retained_precision; tokens.results[0].recovery_method = "contribution_preserving_multiprecision_v1";
    out.token_route = nrq.sourceContract(tokens);
    try { await arc.buildAnalysisRunV02(tokens, manifestFor(tokens) as Json); out.v02_token_rows_built = true; } catch (e) { out.v02_token_rows_built = (e as Error).message; }
    try { await arc.buildAnalysisRunV02(structuredClone(source), manifestFor(source) as Json); out.v02_successor = "built"; } catch (e) { out.v02_successor = (e as Error).message; }
    obs.f_u6b_3 = out;
  });

  it("reopen: a rehashed altered copy is caught by the copy check alone; standing never mints", async () => {
    const mode = "sparse_interactive" as const;
    const { source, invocation } = fixture(mode);
    const model = invocation.request.model;
    u7.simulate = true;
    const received = await direct(source, model, mode);
    const ar = await arc.buildAnalysisRunV03(received, manifestFor(received) as Json, undefined, arc.modelLoadBasisRefs(model));
    const project = async (mutate?: (e: Json) => void, rehashRecord = false) => {
      const env: Json = JSON.parse(JSON.stringify({ model, mechanics_result: received, analysis_run: ar, model_hash: await computeModelHash(model), editor_intents: [], proposal: null, selected_review_target: null }));
      mutate?.(env);
      if (rehashRecord) {
        const hashes = env.analysis_run.analysis_run.hashes.filter((h: Json) => h.payload_scope === "analysis_run_record");
        hashes[0].value = await canonicalSha256HexCheckedV1(arc.analysisRecordProjection(env.analysis_run));
      }
      env.project_envelope_hash = await computeProjectEnvelopeHash(env);
      return env;
    };
    const plain = await buildHistoricalRunContext(await project());
    const altered = await buildHistoricalRunContext(await project(e => { e.analysis_run.analysis_run.retained_precision.body.receipt_version = 7; }, true));
    const mutatedRow = await buildHistoricalRunContext(await project(e => { e.mechanics_result.results[2].value = 12346; }));
    obs.reopen = { plain: plain!.findings, altered_rehashed: altered!.findings, mutated_row: mutatedRow!.findings,
      plain_standing_eligible: nrq.numericalResultStanding(plain!.mechanicsResult!, model).eligible };
    expect(plain!.findings).toEqual(["HISTORICAL_INPUT_MANIFEST_MISSING", rps.RETAINED_PRECISION_VALIDATION_REQUIRED]);
    expect(altered!.findings).toContain(arc.ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH);
    expect(altered!.findings).not.toContain("HISTORICAL_ANALYSIS_HASH_MISMATCH");
    expect(mutatedRow!.findings).toContain("RETAINED_PRECISION_RECEIPT_MISMATCH");
    expect(nrq.numericalResultStanding(plain!.mechanicsResult!, model).eligible).toBe(false);
  });

  it.each(MODES)("%s: the rule-check gate refuses before the backend; post-U7 it passes the registered invocation", async (mode) => {
    const { source, invocation } = fixture(mode);
    const model = invocation.request.model;
    const got = await direct(source, model, mode);
    const calls: string[] = [];
    invokeMock.mockImplementation(async (command: string, args: Json) => { calls.push(command); return { command, args }; });
    const held = await rcs.runRuleChecks({ rulePackDocument: {} as Json, model, solvedEnvelope: got }).then(() => "ran", e => (e as Error).message.split(":")[0]);
    const copy = await rcs.runRuleChecks({ rulePackDocument: {} as Json, model, solvedEnvelope: structuredClone(got) }).then(() => "ran", e => (e as Error).message.split(":")[0]);
    expect(held).toBe(rps.RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE);
    expect(copy).toBe("RULE_NATIVE_INVOCATION_REQUIRED");
    expect(calls).toEqual([]);
    u7.simulate = true;
    const eligible = await direct(source, model, mode);
    const sent: Json[] = [];
    invokeMock.mockImplementation(async (command: string, args: Json) => { sent.push({ command, args }); return {}; });
    await rcs.runRuleChecks({ rulePackDocument: {} as Json, model, solvedEnvelope: eligible });
    expect(sent).toHaveLength(1);
    expect(sent[0].command).toBe("run_rule_checks");
    expect(sent[0].args.sourceBlockInvocation).toEqual(invocation);
    obs[`gate_${mode}`] = { held, copy, backend_calls_when_held: calls.length, post_u7_invocation_equals_captured: JSON.stringify(sent[0].args.sourceBlockInvocation) === JSON.stringify(invocation) };
  });
});

describe("RV91 text", () => {
  // Exact rational comparison of a 3-significant-digit decimal with a binary64 value.
  function exact(b: number): { m: bigint; e: number } { // b = m * 2^e
    const dv = new DataView(new ArrayBuffer(8)); dv.setFloat64(0, b);
    const bits = dv.getBigUint64(0), exp = Number((bits >> 52n) & 0x7ffn), frac = bits & ((1n << 52n) - 1n);
    return exp === 0 ? { m: frac, e: -1074 } : { m: frac | (1n << 52n), e: exp - 1075 };
  }
  function cmpDecimal(text: string, b: number): number { // sign(decimal - b)
    const [mant, ex] = text.split("e"); const M = BigInt(mant.replace(".", "")); const E = Number(ex) - 2;
    const { m, e } = exact(b);
    let L = M, Rr = m; // compare M*10^E with m*2^e
    if (E >= 0) L *= 10n ** BigInt(E); else Rr *= 10n ** BigInt(-E);
    if (e >= 0) Rr *= 2n ** BigInt(e); else L *= 2n ** BigInt(-e);
    return L > Rr ? 1 : L < Rr ? -1 : 0;
  }
  const lower = (text: string) => { const [mant, ex] = text.split("e"); let d = Math.round(Number(mant) * 100) - 1, p = Number(ex); if (d < 100) { d = 999; p -= 1; } return `${(d / 100).toFixed(2)}e${p < 0 ? "-" : "+"}${Math.abs(p)}`; };
  it("upwardBoundText is never below b and is the tight upward 3-digit value except where the decimal equals b", () => {
    let seed = 91; const rnd = () => { seed = (seed * 1103515245 + 12345) % 2147483648; return seed / 2147483648; };
    const samples: number[] = [5e-324, 2.2250738585072014e-308, 1e-300, 1e-15, 1e-5, 1.25, 0.0625, 6.25e-2, 9.995e-5, 9.994e-5, 9.9999999e-5, 1.005, 999.5, 1, 123, 1.2345e-16, Number.MAX_VALUE];
    for (let i = 0; i < 20000; i++) samples.push(Math.pow(10, -320 + rnd() * 620) * (1 + rnd()));
    const dv = new DataView(new ArrayBuffer(8));
    for (let i = 0; i < 5000; i++) { dv.setUint32(0, Math.floor(rnd() * 0x7fefffff)); dv.setUint32(4, Math.floor(rnd() * 0xffffffff)); const x = dv.getFloat64(0); if (Number.isFinite(x) && x > 0) samples.push(x); }
    let unsound = 0, loose = 0, badFormat = 0; const looseExamples: string[] = [];
    for (const b of samples) {
      const t = ksl.upwardBoundText(b);
      if (!/^[1-9]\.\d\de[+-]\d+$/.test(t)) { badFormat++; continue; }
      if (cmpDecimal(t, b) < 0) unsound++;
      if (cmpDecimal(lower(t), b) >= 0) { loose++; if (looseExamples.length < 8) looseExamples.push(`${b} -> ${t}`); }
    }
    obs.upward_bound = { samples: samples.length, unsound, loose, badFormat, looseExamples, zero: ksl.upwardBoundText(0) };
    expect(unsound).toBe(0);
    expect(badFormat).toBe(0);
  });

  it("the milestone absolute labels print each receipt bound upward in the SI unit the reader classified in", async () => {
    const rows: Json[] = [];
    for (const mode of MODES) {
      const { source, invocation } = fixture(mode);
      const got = await direct(source, invocation.request.model, mode);
      const listed = new Map<string, string>(got.retained_precision.body.cases.flatMap((c: Json) => c.status === "selected" ? c.selection.absolute_verified.map((a: Json) => [a.result_id, a.bound]) : []));
      let checked = 0, bad = 0; const units = new Set<string>();
      for (const r of got.results) {
        if (!listed.has(r.id)) continue;
        const label = ksl.resultRowLabel(r, got) ?? "";
        const m = /±(\S+) (\S+), below/.exec(label);
        const b = decodeBinary64(listed.get(r.id)!);
        const si = ({ mm: "m", kN: "N", "kN*m": "N*m", MPa: "Pa" } as Record<string, string>)[r.unit] ?? r.unit;
        checked++; units.add(`${r.unit}->${m?.[2]}`);
        // Sound (never below b) and tight (one step lower is below b, unless the decimal equals b).
        if (!m || cmpDecimal(m[1], b) < 0 || cmpDecimal(lower(m[1]), b) > 0 || m[2] !== si) bad++;
      }
      const reg = rps.retainedRowClasses(got)!;
      const classes: Record<string, number> = {};
      for (const v of reg.values()) classes[v.class] = (classes[v.class] ?? 0) + 1;
      rows.push({ mode, listed: listed.size, checked, bad, units: [...units], classes, example: ksl.resultRowLabel(got.results.find((r: Json) => listed.has(r.id))!, got) });
      expect(bad).toBe(0);
      expect(checked).toBe(listed.size);
    }
    obs.absolute_labels = rows;
  });

  it("captures every new text an unregistered, a registered and a refused successor shows", async () => {
    const { source, invocation } = fixture("sparse_interactive");
    const model = invocation.request.model;
    const texts: Record<string, unknown> = {};
    const panel = (result: Json) => { const { container, unmount } = render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} />); const t = container.textContent ?? ""; unmount(); return t; };
    const unregistered = structuredClone(source);
    texts.unregistered_standing = rps.retainedPrecisionStandingText(unregistered);
    texts.unregistered_notices = ksl.knownSemanticNotices(unregistered);
    const unregisteredPanel = panel(unregistered);
    texts.unregistered_panel_mentions = { checks_passed: unregisteredPanel.includes("checks_passed"), verified: (unregisteredPanel.match(/verified/gi) ?? []).length, integrity: unregisteredPanel.includes("Numerical integrity") };
    const got = await direct(source, model, "sparse_interactive");
    texts.registered_standing = rps.retainedPrecisionStandingText(got);
    texts.registered_notices = ksl.knownSemanticNotices(got);
    const edited = structuredClone(source); edited.results[0].value = 12345;
    const refused = await direct(edited, model, "sparse_interactive");
    texts.refused_standing = rps.retainedPrecisionStandingText(refused);
    texts.output_refusal = lro.loadReferenceOutputRefusal(got);
    texts.output_refusal_unregistered = lro.loadReferenceOutputRefusal(unregistered);
    texts.output_refusal_relabelled = lro.loadReferenceOutputRefusal({ ...structuredClone(source), producer: { ...source.producer, semantic_contract_id: nrq.PREVIEW_PHYSICS_CONTRACT_ID } });
    texts.precheck_unregistered = rcs.ruleBindingPrecheck(unregistered, { solverInputs: [{ input_id: "a", solver_result_ref: { result_id: source.results[0].id } }], valueInputs: [], valueSlots: [], libraryInputs: [] } as Json);
    obs.texts = texts;
    for (const t of [texts.unregistered_standing, texts.registered_standing, texts.refused_standing, JSON.stringify(texts.registered_notices), JSON.stringify(texts.unregistered_notices)] as string[]) {
      expect(t).not.toMatch(/stop-rule|stop rule|enclos|2\^-64|sharper/i);
    }
  });
});
