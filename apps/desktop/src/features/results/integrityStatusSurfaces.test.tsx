// Release exit criterion 18 (LIM-026): no desktop surface a person reads presents a result the
// solver reports checks passed for as passed, integrity checked or established. One walk over the
// rendered text of the results panel (its results line, health indicator and detail), the issues
// list and its detail, and the run comparison, for a linear passing fixture and for the invented
// demo, whose two nonlinear-support cases the solver reports confirmed ("established at 128-bit
// precision"). The results are read through the actual wasm reader.
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import { ResultsPanel } from "./ResultsPanel";
import { numericalHealthView, readNumericalHealth } from "./numericalHealth";
import { numericalResultStanding, reportStanding } from "./resultStanding";
import { SOLVER_REPORT_NOTE, SUPPORT_STATE_ESTABLISHED_TEXT, supportStatesByCase } from "./integrityStatus";
import { DiagnosticsPanel } from "../diagnostics/DiagnosticsPanel";
import { RunComparisonView } from "../comparison/RunComparisonView";
import { compareRunInputs } from "../comparison/runComparison";
import type { RunComparisonSide } from "../comparison/runComparisonSides";
import type { RunComparisonResult } from "../history/useModelStateHistory";
import type { MechanicsResult, PreviewModel } from "../../types";

afterEach(cleanup);

const root = resolve(__dirname, "../../../../..");
const read = <T,>(path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8")) as T;
const FIXTURES: ReadonlyArray<{ name: string; result: string; model: string; nonlinear: boolean }> = [
  { name: "linear passing", result: "fixtures/results/preview_physics_connected_sparse.json", model: "fixtures/model_operations/precision_connected_ui_model.json", nonlinear: false },
  { name: "nonlinear confirmed", result: "fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json", model: "fixtures/product_preview/invented_demo_model.json", nonlinear: true }
];

/** The solver's own words inside its checks-passed message, quoted as the solver wrote them; each
 * is allowed only where the note stands beside it (checked below). */
const SOLVER_QUOTED = ["separate zero count/cap/contact/sliding checks passed"];
/** The text a person reads, with the decided wordings that name the solver's report, and the
 * solver's quoted words, taken out. */
function claims(text: string): string {
  return ["solver checks passed, not verified", ...SOLVER_QUOTED].reduce((rest, words) => rest.split(words).join(""), text);
}
function expectNoClaim(container: HTMLElement, what: string) {
  const text = container.textContent ?? "";
  expect(claims(text), `${what}: checks passed`).not.toMatch(/checks passed/i);
  expect(text, `${what}: integrity checked`).not.toMatch(/integrity[ _]checked/i);
  expect(text, `${what}: inverse-norm`).not.toMatch(/inverse-norm/i);
  expect(text, `${what}: confirmed`).not.toMatch(/(?<![Nn]ot )\bconfirmed\b/);
  // Every element that shows the solver's established sentence or its checks-passed code shows the note with it.
  for (const element of container.querySelectorAll("*")) {
    const own = [...element.childNodes].filter((node) => node.nodeType === node.TEXT_NODE).map((node) => node.textContent).join("");
    if (![SUPPORT_STATE_ESTABLISHED_TEXT, "NUMERICAL_INTEGRITY_CHECKS_PASSED", ...SOLVER_QUOTED].some((words) => own.includes(words))) continue;
    const holder = element.closest("button, li, article, dl, p, span") ?? element;
    const scope = holder.parentElement ?? holder;
    expect(scope.textContent, `${what}: the note beside "${own.slice(0, 60)}"`).toContain(SOLVER_REPORT_NOTE);
  }
}

describe.each(FIXTURES)("no unannotated passed or established claim ($name)", ({ result: resultPath, model: modelPath, nonlinear }) => {
  const result = () => read<MechanicsResult>(resultPath);
  const model = () => read<PreviewModel>(modelPath);

  it("the results panel: results line, health header and detail", () => {
    const source = result(), bound = model();
    const standing = numericalResultStanding(source, bound);
    expect(standing.status).toBe("solver_checks_passed");
    expect(standing.integrity_bound).toEqual({ status: "not_verified", reason: "inverse_norm_bound_not_verified" });
    const supportStates = supportStatesByCase(reportStanding(source, bound).numerical_standing);
    const health = numericalHealthView(readNumericalHealth(source, standing.findings), { eligible: standing.eligible, verdictHold: false, supportStates })!;
    expect(health.cases.length).toBeGreaterThan(0);
    expect(health.cases.every((c) => c.state === "checks_passed")).toBe(true);
    expect(health.badge).toBe("grey");
    render(<ResultsPanel result={source} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={health} />);
    expect(screen.getByTestId("numerical-result-standing").textContent).toMatch(/^Numerical health: not verified\./);
    expect(screen.getByTestId("numerical-health-header")).toHaveTextContent(/^Numerical health: not verified$/);
    for (const c of health.cases) {
      expect(screen.getByTestId(`numerical-health-case-text-${c.caseId}`).textContent).toBe(
        `The solver's own checks found no problem with ${c.caseId}, but SWBPIPE has not verified an error bound for its values.`);
      const line = screen.queryByTestId(`numerical-health-case-support-states-${c.caseId}`);
      if (nonlinear) expect(line?.textContent).toMatch(/^Support states: not confirmed\. Not confirmed: /);
    }
    if (nonlinear) expect(screen.getAllByText(/^Support states: not confirmed/).length).toBeGreaterThan(0);
    expectNoClaim(document.body, "results panel");
  });

  it("the issues list and the diagnostic detail", () => {
    const source = result(), bound = model();
    const integrity = source.diagnostics.filter((d) => d.code === "NUMERICAL_INTEGRITY_CHECKS_PASSED");
    expect(integrity.length).toBeGreaterThan(0);
    if (nonlinear) expect(integrity.some((d) => d.message.includes(SUPPORT_STATE_ESTABLISHED_TEXT))).toBe(true);
    let selected: string | null = null;
    const { rerender } = render(<DiagnosticsPanel model={bound} knowledge={null} result={source} selectedDiagnosticId={null} onSelectDiagnostic={(id) => { selected = id; }} />);
    expect(screen.getAllByTestId("diagnostic-solver-report-note-NUMERICAL_INTEGRITY_CHECKS_PASSED")).toHaveLength(integrity.length);
    fireEvent.click(screen.getAllByTestId("diagnostic-NUMERICAL_INTEGRITY_CHECKS_PASSED")[0]);
    rerender(<DiagnosticsPanel model={bound} knowledge={null} result={source} selectedDiagnosticId={selected} onSelectDiagnostic={() => {}} />);
    expect(screen.getByTestId("selected-diagnostic-solver-report-note")).toHaveTextContent(SOLVER_REPORT_NOTE);
    expectNoClaim(document.body, "issues list");
  });

  it("the run comparison", () => {
    const source = result(), bound = model();
    const standing = numericalResultStanding(source, bound);
    const side = (name: string, diagnostics: MechanicsResult["diagnostics"]): RunComparisonSide => ({
      kind: "recorded", id: `runrec:${name}`, description: name, createdAt: null, state: null, standing, recordedTableRead: null,
      input: { rows: [], standing, semantic_identity: { route: standing.contract, contract_id: "c", contract_sha256: "s" }, context: {}, diagnostics }
    } as unknown as RunComparisonSide);
    const diagnostics = source.diagnostics.map((d) => ({ code: d.code, severity: d.severity ?? null, affected_refs: d.affected_refs ?? [], message: d.message ?? null }));
    const left = side("left", []), right = side("right", diagnostics as unknown as MechanicsResult["diagnostics"]);
    const shown: RunComparisonResult = { projectId: "project", left, right, comparison: compareRunInputs(left.input, right.input) };
    render(<RunComparisonView result={shown} onClose={() => {}} />);
    expect(screen.getByTestId("run-comparison-side-left")).toHaveTextContent("reads solver checks passed, not verified (route");
    expect(screen.getAllByTestId("run-comparison-diagnostic-solver-report-note").length).toBeGreaterThan(0);
    expectNoClaim(document.body, "run comparison");
  });
});
