import { cleanup, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { SolvePanel } from "./SolvePanel";
import { INTEGRITY_CHECKED, NEEDS_RECOMPUTE_NOT_CURRENT } from "./LoadReferenceStatesBlock";
import { initialSolveJob } from "../workspace/solveJobAudit";
import { LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE, SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED, numericalResultStanding, registerSourceRecovery } from "../results/resultStanding";
import { PRESSURE_2_OUTPUT_REFUSAL, PRESSURE_SOURCE_OUTPUT_REFUSAL } from "../results/loadReferenceOutputAvailability";
import type { LoadStateContractEvidence, MechanicsResult, PreviewModel } from "../../types";
import type { PreviewSolverMode } from "../../services/previewService";
import { receivedFromText } from "../../test-support/receivedText";

// B7b: a load/reference-state result is published under the current exact identities,
// pressure-2 (ordinary; fixtures/product_preview/pressure/load_reference/) and
// pressure-source-1 (joined; fixtures/product_preview/pressure_source/load_reference_source/),
// the v3 twins of the retired load-reference-1 and load-reference-source-1. Committed
// producer raws and the committed sensitive ordinary raw; all values are invented
// fixture data. Read from their bytes, as the desktop receives a result (b7-plan R2:
// the readers read received text).
const raws = import.meta.glob<string>([
  "../../../../../fixtures/product_preview/pressure/load_reference/*.json",
  "../../../../../fixtures/product_preview/pressure_source/load_reference_source/*.json",
  "../../../../../core/reporting/result_export/tests/fixtures/load_reference_fallback_uz*.json"
], { query: "?raw", import: "default", eager: true });
function fixture(path: string): unknown {
  const key = Object.keys(raws).find(k => k.endsWith(`/${path}`));
  if (!key) throw new Error(`fixture ${path} not found`);
  return receivedFromText(raws[key]);
}
const MODES: PreviewSolverMode[] = ["sparse_interactive", "dense_scrutiny"];
function pair(dir: "load_reference" | "load_reference_source", name: string, mode: PreviewSolverMode) {
  const request = fixture(`${dir}/${name}.request.json`) as { model: PreviewModel; materials: unknown[] };
  const result = fixture(`${dir}/${name}-${mode}.raw.json`) as MechanicsResult;
  return { request, model: request.model, result };
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
const records = (result: MechanicsResult) => (result.contract_evidence as unknown as LoadStateContractEvidence).load_reference_states;
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
/** Read only; SolvePanel's solve-job download gate is intact with the route's own reason. */
function expectReadOnly(reason: string) {
  const block = screen.getByTestId("load-reference-states-block");
  expect(block.querySelectorAll("a, button, input, select, textarea, [download]")).toHaveLength(0);
  expect(screen.queryByTestId("solve-job-export-link")).toBeNull();
  expect(screen.getByTestId("solve-job-load-reference-output-unavailable")).toHaveTextContent(reason);
}
afterEach(cleanup);

describe("resolved-state block: ordinary pressure-2 with load/reference states", () => {
  for (const mode of MODES) for (const name of ["connected", "pressure"]) {
    it(`${name} ${mode}: admitted by the reader, labelled from Rust's standing, published values only`, () => {
      const { model, result } = pair("load_reference", name, mode);
      const before = JSON.stringify(result);
      renderSolve(model, result, mode);
      const standing = numericalResultStanding(result, model);
      expect(standing).toMatchObject({ contract: "pressure", status: "integrity_checked", eligible: true });
      expect(screen.getByTestId("load-reference-states-route")).toHaveTextContent("Route: ordinary pressure-2");
      expect(screen.getByTestId("load-reference-states-standing")).toHaveTextContent("Result standing: integrity_checked");
      for (const record of records(result)) {
        expect(screen.getByTestId(`load-reference-state-standing-${record.load_case_id}`)).toHaveTextContent(`Numerical standing: ${INTEGRITY_CHECKED} (published solve quality: checks_passed).`);
        expect(screen.getByTestId(`load-reference-state-${record.load_case_id}`)).toHaveTextContent(`mode ${mode}`);
      }
      expect(screen.getByTestId("load-reference-states-block")).not.toHaveTextContent(NEEDS_RECOMPUTE_NOT_CURRENT);
      expectPublishedValues(result);
      expectReadOnly(PRESSURE_2_OUTPUT_REFUSAL);
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
    expect(screen.getByTestId("load-reference-states-route")).toHaveTextContent("Route: ordinary pressure-2");
    expect(screen.getByTestId("load-reference-states-standing")).toHaveTextContent("NUMERICAL_INTEGRITY_NOT_QUALIFIED");
    for (const record of records(result))
      expect(screen.getByTestId(`load-reference-state-standing-${record.load_case_id}`)).toHaveTextContent(`Numerical standing: ${NEEDS_RECOMPUTE_NOT_CURRENT}`);
    expect(screen.getByTestId("load-reference-states-block")).not.toHaveTextContent(INTEGRITY_CHECKED);
    expectPublishedValues(result);
    expectReadOnly(PRESSURE_2_OUTPUT_REFUSAL);
  });

  it("shows no value when the reader refuses a tampered publication", () => {
    const { model, result } = pair("load_reference", "connected", "sparse_interactive");
    records(result)[0].members[0].thermal_strain = 0.5;
    renderSolve(model, result);
    expect(screen.getByTestId("load-reference-states-refused")).toHaveTextContent("SOURCE_LOAD_REFERENCE_STRETCH_BINDING");
    expect(screen.queryByTestId("load-reference-members-case:cold")).toBeNull();
    expect(screen.queryByTestId("load-reference-state-standing-case:cold")).toBeNull();
  });
});

describe("resolved-state block: joined pressure-source-1 with load/reference states", () => {
  for (const mode of MODES) for (const name of ["eigen_motion", "fields", "mixed", "n05", "n06"]) {
    it(`${name} ${mode}: always needs recompute, never Current or integrity checked`, async () => {
      const { request, model, result } = pair("load_reference_source", name, mode);
      renderSolve(model, result, mode);
      // The joined reader admits the publication; without a registered invocation its
      // standing names the missing capture.
      expect(screen.getByTestId(`load-reference-members-${records(result)[0].load_case_id}`)).toBeInTheDocument();
      expect(numericalResultStanding(result, model)).toMatchObject({ contract: "pressure_source", status: "needs_recompute", eligible: false, findings: [SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED] });
      expect(screen.getByTestId("load-reference-states-route")).toHaveTextContent(`Route: joined pressure-source-1; ${NEEDS_RECOMPUTE_NOT_CURRENT} in T1`);
      expect(screen.getByTestId("load-reference-states-standing")).toHaveTextContent(SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED);
      cleanup();
      // With its captured invocation registered, Rust's standing states the declared T1 reason.
      await registerSourceRecovery(result, { request, solver_mode: mode }, model);
      expect(numericalResultStanding(result, model)).toMatchObject({ contract: "pressure_source", status: "needs_recompute", eligible: false });
      expect(numericalResultStanding(result, model).findings).toContain(LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE);
      renderSolve(model, result, mode);
      expect(screen.getByTestId("load-reference-states-standing")).toHaveTextContent(LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE);
      for (const record of records(result))
        expect(screen.getByTestId(`load-reference-state-standing-${record.load_case_id}`)).toHaveTextContent(`Numerical standing: ${NEEDS_RECOMPUTE_NOT_CURRENT}`);
      expect(screen.getByTestId("load-reference-states-block")).not.toHaveTextContent(INTEGRITY_CHECKED);
      expectPublishedValues(result);
      expectReadOnly(PRESSURE_SOURCE_OUTPUT_REFUSAL);
    });
  }

  it("a joined case whose published solve quality is checks_passed is still not labelled integrity checked", async () => {
    const { request, model, result } = pair("load_reference_source", "mixed", "sparse_interactive");
    await registerSourceRecovery(result, { request, solver_mode: "sparse_interactive" }, model);
    renderSolve(model, result);
    expect(screen.getByTestId("load-reference-state-standing-case:ordinary-pressure")).toHaveTextContent(`Numerical standing: ${NEEDS_RECOMPUTE_NOT_CURRENT} (published solve quality: checks_passed).`);
  });

  it("a tampered joined publication is refused and shows no value", () => {
    const { model, result } = pair("load_reference_source", "eigen_motion", "sparse_interactive");
    records(result)[0].members[0].thermal_strain = 0.25;
    renderSolve(model, result);
    expect(screen.getByTestId("load-reference-states-refused")).toHaveTextContent("SOURCE_LOAD_REFERENCE_STRETCH_BINDING");
    expect(screen.queryByTestId("load-reference-members-case:join")).toBeNull();
  });
});

it("no block for other routes or no result", () => {
  const { model } = pair("load_reference", "connected", "sparse_interactive");
  renderSolve(model, null);
  expect(screen.queryByTestId("load-reference-states-block")).toBeNull();
  // A statement relabelled to a retired v2 identity reads unsupported: no block.
  const { result } = pair("load_reference", "connected", "sparse_interactive");
  result.producer = { ...result.producer!, semantic_contract_id: "openpipestress.result_semantics/0.3.0/load-reference-1" };
  cleanup();
  renderSolve(model, result);
  expect(screen.queryByTestId("load-reference-states-block")).toBeNull();
});
