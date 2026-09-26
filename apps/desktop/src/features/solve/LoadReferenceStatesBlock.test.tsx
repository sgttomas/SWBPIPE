import { cleanup, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { SolvePanel } from "./SolvePanel";
import { INTEGRITY_CHECKED, NEEDS_RECOMPUTE_NOT_CURRENT } from "./LoadReferenceStatesBlock";
import { initialSolveJob } from "../workspace/solveJobAudit";
import { numericalResultStanding } from "../results/numericalResultQuality";
import { validateLoadReferenceSourceEvidence } from "../results/loadReferenceSourceEvidence";
import { LOAD_REFERENCE_OUTPUT_REFUSAL } from "../results/loadReferenceOutputAvailability";
import type { LoadReferenceContractEvidence, MechanicsResult, PreviewModel } from "../../types";
import type { PreviewSolverMode } from "../../services/previewService";

// Committed producer raws (fixtures/product_preview/load_reference*/) and the
// committed sensitive ordinary raw. All values are invented fixture data.
const raws = import.meta.glob<{ default: unknown }>([
  "../../../../../fixtures/product_preview/load_reference/*.json",
  "../../../../../fixtures/product_preview/load_reference_source/*.json",
  "../../../../../core/reporting/result_export/tests/fixtures/load_reference_fallback_uz*.json"
], { eager: true });
function fixture(path: string): unknown {
  const key = Object.keys(raws).find(k => k.endsWith(path));
  if (!key) throw new Error(`fixture ${path} not found`);
  return structuredClone(raws[key].default);
}
const MODES: PreviewSolverMode[] = ["sparse_interactive", "dense_scrutiny"];
function pair(dir: string, name: string, mode: PreviewSolverMode) {
  const request = fixture(`${dir}/${name}.request.json`) as { model: PreviewModel };
  const result = fixture(`${dir}/${name}-${mode}.raw.json`) as MechanicsResult;
  return { model: request.model, result };
}
const coreFallback = (mode: PreviewSolverMode) => {
  const request = fixture("load_reference_fallback_uz.request.json") as { model: PreviewModel };
  return { model: request.model, result: fixture(`load_reference_fallback_uz-${mode}.raw.json`) as MechanicsResult };
};
function renderSolve(model: PreviewModel, result: MechanicsResult | null, solverMode: PreviewSolverMode = "sparse_interactive") {
  // The committed request models are rendered unmodified (the two ordinary ones
  // carry no `diagnostics` array; SolvePanel treats that as none).
  return render(<SolvePanel analysisRun={null} model={model} result={result} running={false} solveJob={initialSolveJob()} solverMode={solverMode} onCancel={() => {}} onRun={() => {}} onSolverModeChange={() => {}} />);
}
const records = (result: MechanicsResult) => (result.contract_evidence as unknown as LoadReferenceContractEvidence).load_reference_states;
const text = (value: unknown) => value === null ? "—" : String(value);

/** Every published per-case value in the block is the published value, as text. */
function expectPublishedValues(result: MechanicsResult) {
  for (const record of records(result)) {
    const id = record.load_case_id;
    const members = within(screen.getByTestId(`load-reference-members-${id}`)).getAllByRole("row").slice(1);
    expect(members).toHaveLength(record.members.length);
    record.members.forEach((m, i) => {
      const cells = within(members[i]).getAllByRole("cell").map(c => c.textContent);
      expect(cells).toEqual([m.pipe_id, m.material_id, m.material_selection_kind, text(m.operating_temperature_k), text(m.material_selection_temperature_k),
        text(m.installation_temperature_k), m.thermal_definition, text(m.expansion_law_id), text(m.thermal_strain), m.fit_kind, text(m.fit_strain),
        text(m.total_eigenstrain), text(m.selected_E_pa), text(m.selected_nu)]);
    });
    const supports = within(screen.getByTestId(`load-reference-supports-${id}`)).getAllByRole("row").slice(1);
    expect(supports.map(r => within(r).getAllByRole("cell").map(c => c.textContent)))
      .toEqual(record.support_components.map(s => [s.support_id, s.node_id, s.dof, String(s.prescribed_value), s.unit, s.meaning]));
    const sources = within(screen.getByTestId(`load-reference-sources-${id}`)).getAllByRole("row").slice(1);
    expect(sources).toHaveLength(record.contributions.length + record.excluded_sources.length);
    record.contributions.forEach((c, i) => {
      const cells = within(sources[i]).getAllByRole("cell").map(cell => cell.textContent);
      expect(cells.slice(0, 3)).toEqual([c.source_id, c.owner_kind, c.classification]);
      if ("factor" in c) expect(cells[3]).toBe(text(c.factor));
      if ("applied_magnitude" in c) expect(cells[4]).toBe(`${c.applied_magnitude} (${c.dimension})`);
      else if ("value" in c) expect(cells[4]).toBe(String(c.value));
    });
  }
}
function expectReadOnly() {
  const block = screen.getByTestId("load-reference-states-block");
  expect(block.querySelectorAll("a, button, input, select, textarea, [download]")).toHaveLength(0);
  // SolvePanel's existing solve-job download gate is intact.
  expect(screen.queryByTestId("solve-job-export-link")).toBeNull();
  expect(screen.getByTestId("solve-job-load-reference-output-unavailable")).toHaveTextContent(LOAD_REFERENCE_OUTPUT_REFUSAL);
}
afterEach(cleanup);

describe("resolved-state block: ordinary load-reference-1", () => {
  for (const mode of MODES) for (const name of ["connected", "pressure"]) {
    it(`${name} ${mode}: admitted by the reader, labelled from WP2 standing, published values only`, () => {
      const { model, result } = pair("load_reference", name, mode);
      const before = JSON.stringify(result);
      renderSolve(model, result, mode);
      const standing = numericalResultStanding(result, model);
      expect(standing).toMatchObject({ contract: "load_reference", status: "integrity_checked", eligible: true });
      expect(screen.getByTestId("load-reference-states-route")).toHaveTextContent("Route: ordinary load-reference-1");
      expect(screen.getByTestId("load-reference-states-standing")).toHaveTextContent("Result standing: integrity_checked");
      for (const record of records(result)) {
        expect(screen.getByTestId(`load-reference-state-standing-${record.load_case_id}`)).toHaveTextContent(`Numerical standing: ${INTEGRITY_CHECKED} (published solve quality: checks_passed).`);
        expect(screen.getByTestId(`load-reference-state-${record.load_case_id}`)).toHaveTextContent(`mode ${mode}`);
      }
      expect(screen.getByTestId("load-reference-states-block")).not.toHaveTextContent(NEEDS_RECOMPUTE_NOT_CURRENT);
      expectPublishedValues(result);
      expectReadOnly();
      expect(JSON.stringify(result)).toBe(before);
    });
  }

  for (const mode of MODES) it(`a sensitive ordinary result (${mode}) is never labelled integrity checked`, () => {
    const { model, result } = coreFallback(mode);
    expect(result.numerical_quality?.status).toBe("sensitive");
    renderSolve(model, result, mode);
    const standing = numericalResultStanding(result, model);
    expect(standing.status).toBe("needs_recompute");
    expect(standing.findings).toContain("NUMERICAL_INTEGRITY_NOT_QUALIFIED");
    expect(screen.getByTestId("load-reference-states-route")).toHaveTextContent("Route: ordinary load-reference-1");
    expect(screen.getByTestId("load-reference-states-standing")).toHaveTextContent("NUMERICAL_INTEGRITY_NOT_QUALIFIED");
    for (const record of records(result))
      expect(screen.getByTestId(`load-reference-state-standing-${record.load_case_id}`)).toHaveTextContent(`Numerical standing: ${NEEDS_RECOMPUTE_NOT_CURRENT}`);
    expect(screen.getByTestId("load-reference-states-block")).not.toHaveTextContent(INTEGRITY_CHECKED);
    expectPublishedValues(result);
    expectReadOnly();
  });

  it("shows no value when the reader refuses a tampered publication", () => {
    const { model, result } = pair("load_reference", "connected", "sparse_interactive");
    records(result)[0].members[0].thermal_strain = 0.5;
    renderSolve(model, result);
    expect(screen.getByTestId("load-reference-states-refused")).toHaveTextContent("SOURCE_LOAD_REFERENCE_");
    expect(screen.queryByTestId("load-reference-members-case:cold")).toBeNull();
    expect(screen.queryByTestId("load-reference-state-standing-case:cold")).toBeNull();
  });
});

describe("resolved-state block: joined load-reference-source-1", () => {
  for (const mode of MODES) for (const name of ["eigen_motion", "fields", "mixed", "n05", "n06"]) {
    it(`${name} ${mode}: always needs recompute, never Current or integrity checked`, async () => {
      const { model, result } = pair("load_reference_source", name, mode);
      renderSolve(model, result, mode);
      // Nothing is shown until the joined reader has admitted the publication.
      expect(screen.queryByTestId(`load-reference-members-${records(result)[0].load_case_id}`)).toBeNull();
      await waitFor(() => expect(screen.getByTestId(`load-reference-members-${records(result)[0].load_case_id}`)).toBeInTheDocument(), { timeout: 20_000 });
      expect(numericalResultStanding(result, model)).toMatchObject({ contract: "load_reference_source", status: "needs_recompute", eligible: false });
      expect(screen.getByTestId("load-reference-states-route")).toHaveTextContent(`Route: joined load-reference-source-1; ${NEEDS_RECOMPUTE_NOT_CURRENT} in T1`);
      expect(screen.getByTestId("load-reference-states-standing")).toHaveTextContent("LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1");
      for (const record of records(result))
        expect(screen.getByTestId(`load-reference-state-standing-${record.load_case_id}`)).toHaveTextContent(`Numerical standing: ${NEEDS_RECOMPUTE_NOT_CURRENT}`);
      expect(screen.getByTestId("load-reference-states-block")).not.toHaveTextContent(INTEGRITY_CHECKED);
      expectPublishedValues(result);
      expectReadOnly();
    });
  }

  it("a joined case whose published solve quality is checks_passed is still not labelled integrity checked", async () => {
    const { model, result } = pair("load_reference_source", "mixed", "sparse_interactive");
    await validateLoadReferenceSourceEvidence(result);
    renderSolve(model, result);
    expect(screen.getByTestId("load-reference-state-standing-case:ordinary-pressure")).toHaveTextContent(`Numerical standing: ${NEEDS_RECOMPUTE_NOT_CURRENT} (published solve quality: checks_passed).`);
  });

  it("a tampered joined publication is refused and shows no value", async () => {
    const { model, result } = pair("load_reference_source", "eigen_motion", "sparse_interactive");
    records(result)[0].members[0].thermal_strain = 0.25;
    renderSolve(model, result);
    await waitFor(() => expect(screen.getByTestId("load-reference-states-refused")).toHaveTextContent("SOURCE_LOAD_REFERENCE"), { timeout: 20_000 });
    expect(screen.queryByTestId("load-reference-members-case:join")).toBeNull();
  });
});

it("no block for other routes or no result", () => {
  const { model } = pair("load_reference", "connected", "sparse_interactive");
  renderSolve(model, null);
  expect(screen.queryByTestId("load-reference-states-block")).toBeNull();
});
