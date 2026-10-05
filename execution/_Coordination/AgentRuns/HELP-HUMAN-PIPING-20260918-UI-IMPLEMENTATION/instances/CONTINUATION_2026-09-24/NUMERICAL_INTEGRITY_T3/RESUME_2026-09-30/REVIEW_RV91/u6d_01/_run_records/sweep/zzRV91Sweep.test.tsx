/** RV91 (independent review of U6d): existing-identity sweep.
 * Walks every JSON document under a fixed input root (the candidate archive's
 * P tree, identical inputs for both lanes), finds every embedded mechanics
 * envelope (not only top-level ones), and records desktop carrier outcomes
 * computed by THIS lane's modules. Run once in the base lane and once in the
 * candidate lane; a separate script compares the two outputs.
 * Scratch only: copied into the lane, run, removed. Never committed. */
import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, render } from "@testing-library/react";
import { createHash } from "node:crypto";
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import * as nrq from "./numericalResultQuality";
import * as ksl from "./knownSemanticLimitations";
import * as rs from "./resultSemantics";
import * as lro from "./loadReferenceOutputAvailability";
import { ResultsPanel } from "./ResultsPanel";
import { buildHistoricalRunContext } from "./HistoricalRunContext";
import * as ps from "../../services/previewService";
import * as arc from "../../services/analysisRunCompatibility";
import * as rcs from "../../services/ruleCheckService";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const INPUT_ROOT = process.env.RV91_INPUT_ROOT!;
const OUT = process.env.RV91_SWEEP_OUT!;
const DIRS = ["fixtures", "core", "examples", "validation", "apps/desktop/e2e", "tools"];
const h = (v: unknown) => createHash("sha256").update(typeof v === "string" ? v : JSON.stringify(v)).digest("hex").slice(0, 16);
function files(dir: string, out: string[] = []): string[] {
  let entries: string[] = [];
  try { entries = readdirSync(dir); } catch { return out; }
  for (const e of entries) {
    if (e === "node_modules" || e === "target") continue;
    const p = join(dir, e);
    const st = statSync(p);
    if (st.isDirectory()) files(p, out);
    else if (e.endsWith(".json") && st.size < 30_000_000) out.push(p);
  }
  return out;
}
const isEnvelope = (o: Json) => o && typeof o === "object" && !Array.isArray(o) && Array.isArray(o.results) && o.status && typeof o.status === "object" && typeof o.status.mechanics === "string" && typeof o.run_id === "string";
const isModel = (o: Json) => o && typeof o === "object" && !Array.isArray(o) && o.project && typeof o.project.id === "string" && Array.isArray(o.load_cases) && Array.isArray(o.nodes);
function walk(o: Json, path: string, env: { path: string; v: Json }[], models: Json[]) {
  if (!o || typeof o !== "object") return;
  if (isEnvelope(o)) env.push({ path, v: o });
  if (isModel(o)) models.push(o);
  if (Array.isArray(o)) o.forEach((x, i) => walk(x, `${path}/${i}`, env, models));
  else for (const [k, x] of Object.entries(o)) walk(x, `${path}/${k}`, env, models);
}
const tryS = (f: () => unknown) => { try { const v = f(); return v === undefined ? "undefined" : v; } catch (e) { return `ERR:${(e as Error).message}`; } };
const tryA = async (f: () => Promise<unknown>) => { try { const v = await f(); return v === undefined ? "undefined" : v; } catch (e) { return `ERR:${(e as Error).message}`; } };
function ownModel(source: Json): Json {
  const ids: string[] = [];
  for (const c of source.numerical_quality?.cases ?? []) if (c?.basis_ref?.ref_type === "load_case" && !ids.includes(c.basis_ref.ref_id)) ids.push(c.basis_ref.ref_id);
  if (!ids.length) for (const r of source.results ?? []) if (r?.basis_ref?.ref_type === "load_case" && !ids.includes(r.basis_ref.ref_id)) ids.push(r.basis_ref.ref_id);
  return { load_cases: ids.map(id => ({ id })) };
}
const manifest = (source: Json) => ({ manifest_ref: { object_type: "InputManifest", ref: "manifest:rv91" }, manifest_sha256: "0".repeat(64), manifest: { model_basis: { model_ref: source.model_ref }, solver_basis: { solver_name: source.producer?.component_name ?? "rv91", solver_version: source.producer?.component_version ?? "0", solver_build_ref: "rv91" } } });

afterEach(() => { cleanup(); invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__; });

describe("RV91 sweep", () => {
  it("records carrier outcomes for every embedded envelope", async () => {
    const lines: string[] = [];
    const all = DIRS.flatMap(d => files(join(INPUT_ROOT, d))).sort();
    let n = 0;
    for (const file of all) {
      let doc: Json;
      try { doc = JSON.parse(readFileSync(file, "utf8")); } catch { continue; }
      const env: { path: string; v: Json }[] = [], models: Json[] = [];
      walk(doc, "", env, models);
      for (const { path, v } of env) {
        n++;
        const source = structuredClone(v) as Json;
        const key = `${relative(INPUT_ROOT, file)}#${path}`;
        const paired = models.find(m => m.project.id === source.model_ref) ?? null;
        const own = ownModel(source);
        const o: Record<string, unknown> = {};
        o.identity = source.producer?.semantic_contract_id ?? null;
        o.has_rp = Object.hasOwn(source, "retained_precision");
        o.route = tryS(() => nrq.sourceContract(source));
        o.binding = tryS(() => nrq.sourceSemanticBinding(source));
        o.current = tryS(() => nrq.currentSemanticContract(source));
        o.hasCurrent = tryS(() => nrq.hasCurrentSourceContract(source));
        o.fresh = tryS(() => ksl.isFreshSemanticResult(source));
        o.reason = tryS(() => ksl.standingReason(source));
        o.standing_none = tryS(() => nrq.numericalResultStanding(source));
        o.standing_own = tryS(() => nrq.numericalResultStanding(source, own));
        o.standing_paired = paired ? tryS(() => nrq.numericalResultStanding(source, paired)) : "no-model";
        o.notices = tryS(() => ksl.knownSemanticNotices(source));
        o.output_refusal = tryS(() => lro.loadReferenceOutputRefusal(source));
        o.is_load_ref_route = tryS(() => lro.isLoadReferenceRoute(source));
        o.table = tryS(() => (rs.semanticContractForSource(source) as Json).semantic_contract_id ?? h(rs.semanticContractForSource(source)));
        o.rows = h((source.results as Json[]).map((r: Json) => [r?.id, tryS(() => ksl.resultRowLabel(r, source)), tryS(() => ksl.ruleBindingRefusal(source, r)), tryS(() => h(rs.resultSemantics(r, source))), tryS(() => h(arc.analysisRowSemantics(r, source)))]));
        o.row_labels = (source.results as Json[]).map((r: Json) => tryS(() => ksl.resultRowLabel(r, source))).filter((x: unknown) => x !== null).length;
        o.row_refusals = [...new Set((source.results as Json[]).map((r: Json) => tryS(() => ksl.ruleBindingRefusal(source, r))))];
        const plan = { solverInputs: (source.results as Json[]).map((r: Json, i: number) => ({ input_id: `in${i}`, solver_result_ref: { result_id: r?.id } })), valueInputs: [], valueSlots: [], libraryInputs: [] } as Json;
        o.precheck = tryS(() => h(rcs.ruleBindingPrecheck(source, plan)));
        o.bind_dims = tryS(() => { const b = ps.bindSourceResultDimensions(source); return b === source ? "identity" : h(b); });
        o.ar_v03 = await tryA(async () => canonicalSha256HexCheckedV1(await arc.buildAnalysisRunV03(structuredClone(source), manifest(source) as Json, null)));
        o.ar_v02 = await tryA(async () => canonicalSha256HexCheckedV1(await arc.buildAnalysisRunV02(structuredClone(source), manifest(source) as Json, null)));
        // Rendering: the standing text and a digest of all panel text.
        o.panel = tryS(() => {
          const { container, unmount } = render(<ResultsPanel result={source} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} />);
          const text = container.querySelector("[data-testid=numerical-result-standing]")?.textContent ?? null;
          const all = h(container.textContent ?? "");
          unmount();
          return { text, all };
        });
        cleanup();
        // Registration through mocked direct IPC with a paired (or minimal) model.
        const model = paired ?? { project: { id: source.model_ref }, ...own };
        o.ipc = await tryA(async () => {
          (window as Json).__TAURI_INTERNALS__ = {};
          invokeMock.mockImplementation(async () => structuredClone(v));
          const delivered = await ps.runPreviewMechanics(structuredClone(model), "sparse_interactive");
          return { native: ps.hasNativeMechanicsInvocation(delivered, model), native_mode: ps.hasNativeMechanicsInvocation(delivered, model, "sparse_interactive"), standing: tryS(() => nrq.numericalResultStanding(delivered, model)) };
        });
        delete (window as Json).__TAURI_INTERNALS__;
        invokeMock.mockReset();
        // Reopen (saved record, no AnalysisRun).
        o.reopen = await tryA(async () => {
          const ctx = await buildHistoricalRunContext({ model: structuredClone(model), mechanics_result: structuredClone(v), analysis_run: null } as Json);
          return ctx ? h(JSON.parse(JSON.stringify(ctx, (_k, x) => (typeof x === "function" ? undefined : x)))) : null;
        });
        // Probes of the guard on an existing identity: the deliberate change.
        const withMember = { ...structuredClone(v), retained_precision: {} };
        o.probe_member = { route: tryS(() => nrq.sourceContract(withMember)), standing: tryS(() => (nrq.numericalResultStanding(withMember, own) as Json).findings) };
        lines.push(JSON.stringify({ key, o }));
      }
    }
    writeFileSync(OUT, lines.join("\n") + "\n");
    expect(n).toBeGreaterThan(0);
  }, 1_800_000);
});
