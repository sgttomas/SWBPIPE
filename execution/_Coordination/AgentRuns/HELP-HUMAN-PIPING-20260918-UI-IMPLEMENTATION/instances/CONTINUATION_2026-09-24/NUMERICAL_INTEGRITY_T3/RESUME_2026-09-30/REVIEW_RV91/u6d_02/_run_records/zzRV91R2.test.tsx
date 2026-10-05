/** RV91 round 2 (confirmation of I67's U6d repair round). Scratch only:
 * copied into the lane, run, removed. Never committed. */
import { afterAll, afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, render } from "@testing-library/react";
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import * as nrq from "./numericalResultQuality";
import * as rps from "./retainedPrecisionStanding";
import { ResultsPanel } from "./ResultsPanel";
import * as ps from "../../services/previewService";
import * as arc from "../../services/analysisRunCompatibility";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const OUT = process.env.RV91_REVIEW_OUT!;
const root = resolve(__dirname, "../../../../../");
const obs: Record<string, unknown> = {};
afterAll(() => writeFileSync(OUT, JSON.stringify(obs, null, 1)));
afterEach(() => { cleanup(); invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__; });
const pins: Record<string, string> = { sparse_interactive: "ac6986b0680e0df9d88c33a5bf4635372fc3b83cbb9080da44e6d32dbdca59dc", dense_scrutiny: "6cd1d249e5352aaffbd2b7d7349c74a1d0e0572df35500f66be49c3cad95c9b5" };
function fixture(mode: string) {
  const bytes = readFileSync(resolve(root, `fixtures/results/retained_precision_milestone_successor_${mode}.json`));
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(pins[mode]);
  const doc = JSON.parse(bytes.toString("utf8"));
  return { source: doc.source as Json, invocation: doc.invocation as Json };
}
async function direct(source: Json, model: Json, mode: string) {
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async () => structuredClone(source));
  return ps.runPreviewMechanics(model, mode as Json);
}
const manifestFor = (s: Json) => ({ manifest_ref: { object_type: "InputManifest", ref: "manifest:rv91r2" }, manifest_sha256: "3".repeat(64), manifest: { model_basis: { model_ref: s.model_ref }, solver_basis: { solver_name: "x", solver_version: "0", solver_build_ref: "rv91" } } });

describe("RV91 r2: SF-1, the historical v0.2 builder", () => {
  it("refuses a receipt member (object, null) and token rows on both legacy schema versions; controls unchanged", async () => {
    const { source } = fixture("sparse_interactive");
    const legacy = (version: string) => {
      const s = structuredClone(source); s.schema_version = version;
      for (const k of ["producer", "numerical_quality", "formulation_basis", "contract_evidence", "retained_precision"]) delete s[k];
      for (const r of s.results) delete r.recovery_method;
      return s;
    };
    const out: Record<string, string> = {};
    const attempt = async (label: string, s: Json) => { try { const r = await arc.buildAnalysisRunV02(s, manifestFor(s) as Json); out[label] = `built:${Object.hasOwn(r.analysis_run, "retained_precision") ? "with" : "without"}-receipt`; } catch (e) { out[label] = (e as Error).message; } };
    for (const version of ["0.1.0", "0.2.0"]) {
      await attempt(`${version}:control`, legacy(version));
      const obj = legacy(version); obj.retained_precision = structuredClone(source.retained_precision); await attempt(`${version}:member object`, obj);
      const nul = legacy(version); nul.retained_precision = null; await attempt(`${version}:member null`, nul);
      const empty = legacy(version); empty.retained_precision = {}; await attempt(`${version}:member {}`, empty);
      const first = legacy(version); first.results[0].recovery_method = "contribution_preserving_multiprecision_v1"; await attempt(`${version}:token first row`, first);
      const last = legacy(version); last.results[last.results.length - 1].recovery_method = "contribution_preserving_multiprecision_v1"; await attempt(`${version}:token last row`, last);
      const other = legacy(version); other.results[5].recovery_method = "some_other_method"; await attempt(`${version}:other method string`, other);
    }
    await attempt("successor itself", structuredClone(source));
    obs.sf1 = out;
    for (const version of ["0.1.0", "0.2.0"]) {
      expect(out[`${version}:control`]).toBe("built:without-receipt");
      for (const k of ["member object", "member null", "member {}", "token first row", "token last row"]) expect(out[`${version}:${k}`], `${version}:${k}`).toBe(arc.ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN);
    }
    expect(out["successor itself"]).toBe("HISTORICAL_ANALYSIS_SOURCE_UNSUPPORTED");
  });
});

describe("RV91 r2: N-4 and RV88 N-3, the standing text", () => {
  it.each(["sparse_interactive", "dense_scrutiny"])("%s: the count only from a validated registration; refused text truthful", async (mode) => {
    const { source, invocation } = fixture(mode);
    const model = invocation.request.model;
    const panelText = (r: Json) => { const { container, unmount } = render(<ResultsPanel result={r} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} />); const t = container.querySelector("[data-testid=numerical-result-standing]")?.textContent ?? ""; unmount(); return t; };
    const registered = await direct(source, model, mode);
    const unregistered = structuredClone(source);
    const edited = structuredClone(source); edited.results[2].value = 12346;
    const refused = await direct(edited, model, mode);
    const foreign = await direct(source, model, mode === "sparse_interactive" ? "dense_scrutiny" : "sparse_interactive");
    // A two-case corpus statement, unregistered then registered.
    const corpus = JSON.parse(readFileSync(resolve(root, "fixtures/results/retained_precision_cases.json"), "utf8"));
    const entry = corpus.cases.find((c: Json) => c.id === "two_case_facade_after_certificate_synthetic");
    const two = structuredClone(entry.source);
    const twoUnregistered = rps.retainedPrecisionStandingText(two);
    await rps.registerRetainedPrecision(two, structuredClone(entry.invocation));
    const texts = {
      registered: rps.retainedPrecisionStandingText(registered),
      unregistered: rps.retainedPrecisionStandingText(unregistered),
      refused_this_session: rps.retainedPrecisionStandingText(refused),
      refused_foreign_mode: rps.retainedPrecisionStandingText(foreign),
      two_case_unregistered: twoUnregistered,
      two_case_registered: rps.retainedPrecisionStandingText(two),
      panel_registered: panelText(registered), panel_unregistered: panelText(unregistered), panel_refused: panelText(refused),
      standing_refused: nrq.numericalResultStanding(refused, model).findings,
    };
    obs[`texts_${mode}`] = texts;
    expect(texts.registered).toContain("Selected cases: 1 of 1.");
    expect(texts.two_case_registered).toContain("Selected cases: 1 of 2.");
    for (const k of ["unregistered", "refused_this_session", "refused_foreign_mode", "two_case_unregistered", "panel_unregistered", "panel_refused"] as const) expect(texts[k], k).not.toContain("Selected cases");
    for (const k of ["refused_this_session", "refused_foreign_mode"] as const) { expect(texts[k]).toContain("unsupported, values shown for inspection only."); expect(texts[k]).not.toMatch(/historical/i); }
    expect(texts.refused_this_session).toContain("(RETAINED_PRECISION_RECEIPT_MISMATCH)");
    expect(texts.refused_foreign_mode).toContain("(RETAINED_PRECISION_INVOCATION_MISMATCH)");
    expect(texts.panel_registered).toBe(texts.registered);
    expect(texts.panel_refused).toBe(texts.refused_this_session);
    for (const t of Object.values(texts).filter(v => typeof v === "string") as string[]) expect(t).not.toMatch(/stop-rule|stop rule|enclos|sharper/i);
  });
});
