/** RV91 round 3: an independent TS consumer of the shared case file v2 and its
 * declared_differences (typescript expectations), plus the unit table. Scratch
 * only: copied into the lane, run, removed. Never committed. */
import { afterAll, afterEach, describe, expect, it, vi } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import * as nrq from "./numericalResultQuality";
import * as rps from "./retainedPrecisionStanding";
import * as ksl from "./knownSemanticLimitations";
import { validateRetainedPrecision, validateRetainedPrecisionTransport } from "./retainedPrecision";
import * as ps from "../../services/previewService";
import * as rcs from "../../services/ruleCheckService";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const OUT = process.env.RV91_REVIEW_OUT!;
const root = resolve(__dirname, "../../../../../");
const file = JSON.parse(readFileSync(resolve(root, "fixtures/results/retained_precision_carrier_cases.json"), "utf8"));
const obs: Record<string, unknown> = {};
afterAll(() => writeFileSync(OUT, JSON.stringify(obs, null, 1)));
afterEach(() => { invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__; });
function load(id: string): { source: Json; invocation: Json | null } {
  const spec = file.fixtures[id];
  const bytes = readFileSync(resolve(root, spec.path));
  expect(createHash("sha256").update(bytes).digest("hex"), id).toBe(spec.sha256);
  const doc = JSON.parse(bytes.toString("utf8"));
  return spec.shape === "milestone" ? { source: doc.source, invocation: doc.invocation } : { source: doc, invocation: null };
}
function apply(e: Json, source: Json, invocation: Json) {
  let at = e.target === "source" ? source : invocation;
  for (const k of e.path.slice(0, -1)) at = at[k];
  const last = e.path[e.path.length - 1] === -1 ? at.length - 1 : e.path[e.path.length - 1];
  if (e.op === "set") at[last] = e.value; else if (e.op === "delete") delete at[last]; else throw new Error(e.op);
}
async function deliver(source: Json, invocation: Json | null) {
  if (invocation === null) return structuredClone(source);
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async () => structuredClone(source));
  const got = await ps.runPreviewMechanics(invocation.request.model, invocation.solver_mode);
  invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__;
  return got;
}
const modelFor = (requested: Json, base: Json | null) => requested === "invocation" ? (base ? base.request.model : { load_cases: [] }) : { load_cases: requested.map((r: Json) => ({ id: r.ref_id })) };

describe("RV91 r3: case format v2", () => {
  it("20 cases, 4 fixtures, every case's standing and dispatch in TS", async () => {
    expect(file.format).toBe("I66-U6-CARRIER-CASES-v2");
    expect(file.cases).toHaveLength(20);
    const rows: Json[] = [];
    for (const c of file.cases) {
      const { source, invocation: base } = load(c.fixture);
      const src = structuredClone(source), inv = base ? structuredClone(base) : null;
      for (const e of c.edits) apply(e, src, inv);
      const delivered = await deliver(src, c.invocation === null ? null : inv);
      const route = nrq.sourceContract(delivered);
      const model = modelFor(c.requested, base);
      const standing = route === "unsupported" ? "unsupported" : route === "retained_preview_physics" ? rps.retainedPrecisionStanding(delivered, model).standing : `route:${route}`;
      let dispatch: string;
      if (route === "unsupported") dispatch = nrq.numericalResultStanding(delivered).findings[0];
      else if (route === "retained_preview_physics") { try { await validateRetainedPrecision(delivered); dispatch = "ok"; } catch (e) { dispatch = (e as Error).message; } }
      else dispatch = `route:${route}`;
      const eligible = nrq.numericalResultStanding(delivered, model).eligible;
      rows.push({ id: c.id, standing, dispatch, eligible, ok: standing === c.expected_standing && dispatch === c.expected_dispatch && !eligible });
    }
    obs.cases = rows;
    for (const r of rows) expect(r.ok, r.id).toBe(true);
    // The raw fixtures without an edit: their own routes, no downgrade.
    obs.raw_unedited = Object.fromEntries(["legacy_preview_0_1", "preview_physics_1_invented_sparse"].map(id => [id, nrq.sourceContract(load(id).source)]));
  });

  it("declared_differences: exactly four entries; every typescript expectation holds", async () => {
    const dd = file.declared_differences;
    expect(dd.map((e: Json) => e.id)).toEqual(["I67-F1:unregistered_invalid_statement", "I67-F2:display_only_binding_precheck", "F-U6b-2:python_refuses_transport", "F5:refused_statement_binding"]);
    const results: Json[] = [];
    for (const e of dd) {
      expect(Object.keys(e.expected).sort()).toEqual(["python", "rust", "typescript"]);
      const want = e.expected.typescript;
      for (const fx of e.fixtures) {
        const { source, invocation: base } = load(fx);
        const src = structuredClone(source), inv = base ? structuredClone(base) : null;
        for (const ed of e.edits) apply(ed, src, inv);
        const delivered = await deliver(src, e.invocation === null ? null : inv);
        const model = modelFor(e.requested, base);
        const got: Json = {};
        if (e.subject === "standing") { const s = rps.retainedPrecisionStanding(delivered, model); got.standing = s.standing; got.finding = s.findings[0]; got.eligible = nrq.numericalResultStanding(delivered, model).eligible; }
        if (e.subject === "binding") {
          const reasons = delivered.results.map((r: Json) => ksl.ruleBindingRefusal(delivered, r));
          got.binding = reasons.every((x: string | null) => x === "RULE_QUANTITY_NOT_COVERED") ? "every_row:RULE_QUANTITY_NOT_COVERED" : `mixed:${[...new Set(reasons)].join(",")}`;
          const plan = { solverInputs: delivered.results.map((r: Json, i: number) => ({ input_id: `i${i}`, solver_result_ref: { result_id: r.id } })), valueInputs: [], valueSlots: [], libraryInputs: [] } as Json;
          const notices = new Set(rcs.ruleBindingPrecheck(delivered, plan).map(f => f.notice));
          got.notice = notices.size === 1 && notices.has(ksl.N_RP_UNVALIDATED) ? "N_RP_UNVALIDATED" : [...notices].join("|");
        }
        if (e.subject === "transport") {
          const route = nrq.sourceContract(delivered);
          try { if (route !== "retained_preview_physics") throw new Error(`route:${route}`); await validateRetainedPrecisionTransport(delivered); got.transport = "ok"; } catch (x) { got.transport = (x as Error).message; }
        }
        const agrees = Object.entries(want).every(([k, v]) => got[k] === v) && got.eligible !== true;
        results.push({ id: e.id, fixture: fx, want, got, agrees });
      }
    }
    obs.declared_differences = results;
    for (const r of results) expect(r.agrees, `${r.id} ${r.fixture}`).toBe(true);
  });

  it("the unit table: nine entries equal Rust's si_unit; an unknown unit claims no bound", () => {
    const bits = "3c00000000000000";
    const rust: Record<string, string> = { m: "m", mm: "m", rad: "rad", N: "N", kN: "N", "N*m": "N*m", "kN*m": "N*m", Pa: "Pa", MPa: "Pa" };
    // A Map, not an object literal: a "__proto__" key must be a key (RV91 probe fix).
    const out = new Map<string, string>();
    for (const u of [...Object.keys(rust), "degC", "mode_code", "unitless", "", "toString", "constructor", "__proto__", "MN", "m/s", "N.m", "N·m"]) out.set(u, ksl.retainedAbsoluteNotice(bits, u));
    obs.units = Object.fromEntries([...out].map(([k, v]) => [`unit:${k}`, v]));
    for (const [u, si] of Object.entries(rust)) expect(out.get(u), u).toBe(ksl.N_RP_ABSOLUTE.replace("{b}", ksl.upwardBoundText(2 ** -63)).replace("{unit}", si));
    for (const u of ["degC", "mode_code", "unitless", "", "toString", "constructor", "__proto__", "MN", "m/s", "N.m", "N·m"]) expect(out.get(u), u).toBe(ksl.N_RP_NOT_COVERED);
  });
});
