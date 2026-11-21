// Issue 588: the engine publishes one fixed run id for distinct runs, so a results view's
// transient state (page, kind filter, navigator entity, selected row) is held with the run's view
// identity, never its run id. Swapping directly from run A to run B with the same run id, without
// passing through no result, shows run B's rows with run B's own default view; a harmless re-render
// of run A keeps run A's view. Invented and fixture data only.
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { buildHistoricalRunContext, HistoricalRunPanel, type HistoricalRunContext } from "./HistoricalRunContext";
import { ResultsPanel, resultViewIdentity, type ResultsViewRequest } from "./ResultsPanel";
import { SUPPORT_STATE_ROW_KIND } from "./keyResults";
import { keyResult, keyRows } from "../../test-support/keyResultsFixtures";
import { row } from "../../test-support/resultOverlayFixtures";
import { buildAnalysisRunV03 } from "../../services/analysisRunCompatibility";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import savedModel from "../../../../../fixtures/model_operations/precision_connected_ui_model.json";
import savedResult from "../../../../../fixtures/results/preview_physics_connected_sparse.json";
import type { LocalProjectEnvelope, MechanicsResult, PreviewModel } from "../../types";

afterEach(cleanup);

const pageSummary = () => screen.getByTestId("result-page-summary").textContent ?? "";

/** A saved project's envelope over the connected fixture (89 rows: two pages of 50), with the first
 * row's value moved by `shift` so two saves are distinct runs under the same engine run id. */
async function savedEnvelope(shift: number): Promise<LocalProjectEnvelope> {
  const model = structuredClone(savedModel) as unknown as PreviewModel;
  const mechanics_result = structuredClone(savedResult) as unknown as MechanicsResult;
  mechanics_result.results[0].value += shift;
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: mechanics_result.producer!.component_name, solver_version: mechanics_result.producer!.component_version, solver_build_ref: "synthetic@1", solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] });
  const analysis_run = await buildAnalysisRunV03(mechanics_result, inputManifest);
  const model_hash = await computeModelHash(model);
  const payload = { model, mechanics_result, analysis_run, model_hash, editor_intents: [], proposal: null, selected_review_target: null };
  return { ...payload, project_envelope_hash: await computeProjectEnvelopeHash(payload) } as unknown as LocalProjectEnvelope;
}

/** The app's mounting of a historical run (`App.tsx`): keyed on its view identity. */
function Shown({ context }: { context: HistoricalRunContext }) {
  return <HistoricalRunPanel key={context.viewKey} context={context} />;
}

describe("a historical run's view identity", () => {
  it("is the same for the same saved run and distinct for two saved runs sharing a run id", async () => {
    const a = (await buildHistoricalRunContext(await savedEnvelope(0)))!;
    const again = (await buildHistoricalRunContext(await savedEnvelope(0)))!;
    const b = (await buildHistoricalRunContext(await savedEnvelope(1)))!;
    expect(a.runId).toBe(b.runId);
    expect(a.viewKey).toMatch(/^saved-result-sha256:[0-9a-f]{64}$/u);
    expect(again.viewKey).toBe(a.viewKey);
    expect(b.viewKey).not.toBe(a.viewKey);
  });

  it("falls back to the envelope's hashes for a malformed saved result, and never collides", async () => {
    const saved = await savedEnvelope(0);
    (saved as unknown as { mechanics_result: unknown }).mechanics_result = { run_id: savedResult.run_id, results: [{ value: Number.NaN }] };
    const malformed = (await buildHistoricalRunContext(saved))!;
    expect(malformed.viewKey).toMatch(/^saved-envelope-payload-sha256:/u);
    expect(malformed.viewKey).not.toBe((await buildHistoricalRunContext(await savedEnvelope(0)))!.viewKey);
  });

  it("HistoricalRunPanel: run B, swapped in directly with run A's run id, opens on its own first page", async () => {
    const a = (await buildHistoricalRunContext(await savedEnvelope(0)))!;
    const b = (await buildHistoricalRunContext(await savedEnvelope(1)))!;
    const { rerender } = render(<Shown context={a} />);
    fireEvent.click(screen.getByTestId("next-result-page"));
    expect(pageSummary()).toMatch(/page 2 of 2$/u);
    rerender(<Shown context={a} />);
    expect(pageSummary()).toMatch(/page 2 of 2$/u);
    rerender(<Shown context={b} />);
    expect(pageSummary()).toMatch(/page 1 of 2$/u);
    expect(screen.getByTestId(`result-row-${savedResult.results[0].id}`)).toHaveTextContent(String(savedResult.results[0].value + 1));
  });
});

describe("ResultsPanel's view identity", () => {
  /** Run A and run B: distinct results (distinct objects, one row's value apart) with the same run id. */
  function runs(): [MechanicsResult, MechanicsResult] {
    const rows = [...keyRows(), ...Array.from({ length: 120 }, (_, i) => row(`d:extra:${i}`, "displacement_magnitude", `node:X${i}`, i, "mm"))];
    const a = keyResult(rows);
    const b = keyResult(rows.map((item, i) => (i === 0 ? { ...item, value: item.value + 1 } : item)));
    expect(b.run_id).toBe(a.run_id);
    return [a, b];
  }
  const view = (result: MechanicsResult, request: ResultsViewRequest | null = null, runIdentityKey?: string) => (
    <ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => undefined} viewRequest={request} runIdentityKey={runIdentityKey} />
  );

  it("derives a stable identity per result and never the bare run id", () => {
    const [a, b] = runs();
    expect(resultViewIdentity(a)).toBe(resultViewIdentity(a));
    expect(resultViewIdentity(b)).not.toBe(resultViewIdentity(a));
    expect(resultViewIdentity(a)).not.toContain(a.run_id);
    expect(resultViewIdentity(a, "recording:R-1")).toBe(resultViewIdentity(b, "recording:R-1"));
    expect(resultViewIdentity(null)).toBeNull();
  });

  it("resets the page index when swapped directly from run A to run B, and keeps it across a re-render of run A", () => {
    const [a, b] = runs();
    const { rerender } = render(view(a));
    fireEvent.click(screen.getByTestId("next-result-page"));
    expect(pageSummary()).toMatch(/page 2 of 3$/u);
    rerender(view(a));
    expect(pageSummary()).toMatch(/page 2 of 3$/u);
    rerender(view(b));
    expect(pageSummary()).toMatch(/page 1 of 3$/u);
  });

  it("resets the kind filter when swapped directly to run B, even with run A's request still given", () => {
    const [a, b] = runs();
    const request: ResultsViewRequest = { serial: 1, runId: a.run_id, kind: SUPPORT_STATE_ROW_KIND, withheld: {} };
    const { rerender } = render(view(a, request));
    expect(screen.getByTestId("results-kind-filter")).toBeInTheDocument();
    rerender(view(a, request));
    expect(screen.getByTestId("results-kind-filter")).toBeInTheDocument();
    rerender(view(b, request));
    expect(screen.queryByTestId("results-kind-filter")).toBeNull();
    expect(pageSummary()).toMatch(/page 1 of 3$/u);
    // A new request, made for run B, applies to it.
    rerender(view(b, { ...request, serial: 2 }));
    expect(screen.getByTestId("results-kind-filter")).toBeInTheDocument();
  });

  it("holds the view with a caller's stored identity: a new object for the same stored run keeps it, another run's resets it", () => {
    const [a, b] = runs();
    const { rerender } = render(view(a, null, "recording:R-1"));
    fireEvent.click(screen.getByTestId("next-result-page"));
    rerender(view({ ...a }, null, "recording:R-1"));
    expect(pageSummary()).toMatch(/page 2 of 3$/u);
    rerender(view(b, null, "recording:R-2"));
    expect(pageSummary()).toMatch(/page 1 of 3$/u);
  });
});
