// DEL-07-05 numerical health §3, §4 and §7: the Results panel's health header, the
// output sentence, the per-case detail, row badges, located-cause navigation and the
// solver-diagnostic open. The indicator is a hand-written fixture of the Rust
// `numerical_health_json` output shape, presented through `numericalHealthView`.
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import { lineHealthState, ResultsPanel } from "./ResultsPanel";
import { numericalHealthLead, ordinarySolveLead, recoveredCaseIds } from "./integrityStatus";
import { openMethodAndLimits, shownStandingSentence } from "./resultsStageTestSupport";
import { METHOD_AND_LIMITS_LABEL, RESULTS_RUN_LABELS } from "./ResultsStageHeader";
import { checksPassedText, ELIGIBLE_RECOVERY_TEXT, NUMERICAL_HEALTH_STATE_TEXT } from "./numericalHealthText";
import { numericalHealthView, parseNumericalHealth, type NumericalHealthView } from "./numericalHealth";
import { resolveLocatedCauseSelection } from "./resultInterpretation";
import type { MechanicsResult, PreviewModel } from "../../types";
import unicodeSparse from "../../../../../fixtures/results/preview_physics_unicode_ids_sparse.json";

afterEach(cleanup);

function view(raw: unknown, options: Parameters<typeof numericalHealthView>[1]): NumericalHealthView {
  const health = parseNumericalHealth(raw);
  expect(health).not.toBeNull();
  return numericalHealthView(health, options)!;
}
const genericResult = () => structuredClone(unicodeSparse) as unknown as MechanicsResult;
const SENSITIVE = {
  case_id: "load:é", state: "sensitive", code: "NUMERICAL_INTEGRITY_SENSITIVE", integrity_diagnostic_id: "diagnostic:numerical-integrity:load:é",
  rcond: 1e-11, digits: 11, conditioning_sensitive: true,
  causes: [
    { kind: "formation", diagnostic_id: "diagnostic:numerical-integrity:load:é", locations: [{ kind: "node_dof", node: "node:α", dof: "UX" }, { kind: "node_dof", node: "node:missing", dof: "RZ" }] },
    { kind: "curved_bend_short", diagnostic_id: "diagnostic:bend", locations: [{ kind: "ref", ref: "pipe:α-β" }, { kind: "text", text: "member end forces" }] }
  ]
};
const PASSED = { case_id: "load:𝔫", state: "checks_passed", code: "NUMERICAL_INTEGRITY_CHECKS_PASSED", integrity_diagnostic_id: "diagnostic:numerical-integrity:load:𝔫", rcond: 0.01, digits: 2, conditioning_sensitive: false, causes: [] };
const genericHealth = (verdictOnly: boolean) => ({
  route: "generic", verdict_only: verdictOnly, cases: [SENSITIVE, PASSED],
  combinations: [{ combination_id: "combination:Σ", state: "sensitive", cases: ["load:é", "load:𝔫"], withheld: false }]
});

describe("ResultsPanel numerical health", () => {
  it("shows the generic header with counts, the output sentence under the hold, and the per-case detail", () => {
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
      numericalHealth={view(genericHealth(true), { eligible: false, verdictHold: true })} />);
    expect(screen.getByTestId("numerical-health-header-line").textContent).toBe(
      "Numerical health: 1 sensitive, 1 no problem found. Rule checks, the result JSON and stress-neutral exports and the report package do not use this run while 1 load case(s) are sensitive, quality unresolved, solve failed or not assessed; results tables, CSV export and, for an exact-route result, the rendered report carry each case's standing."
    );
    expect(screen.getByTestId("numerical-health-header")).toHaveTextContent(/^Numerical health: 1 sensitive, 1 no problem found$/);
    expect(screen.getByTestId("numerical-health-header")).toHaveAttribute("data-badge", "red");
    expect(screen.getByTestId("numerical-health-case-text-load:é").textContent).toContain("Its values are kept for inspection.");
    expect(screen.getByTestId("numerical-health-case-conditioning-load:é").textContent).toContain("roughly 11.0 of the about 16 decimal digits");
    expect(screen.getByTestId("numerical-health-case-badge-load:𝔫")).toHaveTextContent("Numerical health: solver checks found no problem");
    expect(screen.getByTestId("numerical-health-combination-badge-combination:Σ")).toHaveTextContent("sensitive");
  });

  it("adds no output sentence without the verdict-only hold", () => {
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
      numericalHealth={view(genericHealth(false), { eligible: false, verdictHold: false })} />);
    expect(screen.getByTestId("numerical-health-header-line").textContent).toBe("Numerical health: 1 sensitive, 1 no problem found");
    expect(screen.queryByTestId("numerical-health-output-sentence")).toBeNull();
  });

  it("marks a generic row with its load case or combination state only where it differs from the standing line's, and never where the solver's checks found no problem", () => {
    const result = genericResult();
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
      numericalHealth={view(genericHealth(true), { eligible: false, verdictHold: true })} />);
    const line = lineHealthState(result.numerical_quality?.status);
    const rows = result.results.slice(0, 50);
    let marked = 0, unmarked = 0;
    for (const row of rows) {
      const badge = screen.queryByTestId(`result-row-health-${row.id}`);
      if (!row.basis_ref) { expect(badge).toBeNull(); continue; }
      const state = row.basis_ref.ref_id === "load:𝔫" ? "checks_passed" : "sensitive";
      if (state === "checks_passed" || state === line) { expect(badge).toBeNull(); unmarked += 1; continue; }
      expect(badge?.textContent).toBe(NUMERICAL_HEALTH_STATE_TEXT[state]);
      marked += 1;
    }
    // Not vacuous: the fixture's rows differ from the line somewhere.
    expect(marked).toBeGreaterThan(0);
    void unmarked;
  });

  it("marks no row whose state is the one the standing line states", () => {
    const result = genericResult();
    const passed = { route: "generic", verdict_only: false, cases: [{ ...PASSED, case_id: "load:é", integrity_diagnostic_id: "diagnostic:numerical-integrity:load:é" }, PASSED],
      combinations: [{ combination_id: "combination:Σ", state: "checks_passed", cases: ["load:é", "load:𝔫"], withheld: false }] };
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
      numericalHealth={view(passed, { eligible: true, verdictHold: false })} />);
    expect(lineHealthState(result.numerical_quality?.status)).toBe("checks_passed");
    expect(screen.getAllByTestId(/^result-row-result/).length).toBeGreaterThan(0);
    expect(screen.queryAllByTestId(/^result-row-health-/)).toHaveLength(0);
  });

  it("gives a withheld combination's rows no badge", () => {
    const raw = genericHealth(true) as { combinations: { withheld: boolean; state: string | null; cases: string[] }[] };
    Object.assign(raw.combinations[0], { withheld: true, state: null, cases: [] });
    const result = genericResult();
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={view(raw, { eligible: false, verdictHold: true })} />);
    expect(screen.queryByTestId("numerical-health-combination-badge-combination:Σ")).toBeNull();
    expect(screen.queryByTestId("numerical-health-combination-combination:Σ")).toBeNull();
    const combinationRow = result.results.slice(0, 50).find((row) => row.basis_ref?.ref_type === "combination");
    if (combinationRow) expect(screen.queryByTestId(`result-row-health-${combinationRow.id}`)).toBeNull();
  });

  it("lists located causes, navigates the resolvable ones and opens the solver diagnostic", () => {
    const model = { nodes: [{ id: "node:α" }], pipe_segments: [{ id: "pipe:α-β" }], supports: [], components: [], load_cases: [] } as unknown as PreviewModel;
    const onSelect = vi.fn(), onShow = vi.fn();
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
      numericalHealth={view(genericHealth(true), { eligible: false, verdictHold: true })}
      resolveCauseLocation={(location) => resolveLocatedCauseSelection(model, location)}
      onSelectCauseLocation={onSelect} onShowDiagnostic={onShow} />);
    const list = within(screen.getByTestId("numerical-health-case-locations-load:é"));
    expect(list.getByText("node node:α, UX").tagName).toBe("BUTTON");
    expect(list.getByText("node node:missing, RZ").tagName).toBe("SPAN");
    expect(list.getByText("pipe:α-β").tagName).toBe("BUTTON");
    expect(list.getByText("member end forces").tagName).toBe("SPAN");
    fireEvent.click(list.getByText("node node:α, UX"));
    expect(onSelect).toHaveBeenLastCalledWith({ type: "node", id: "node:α" });
    fireEvent.click(list.getByText("pipe:α-β"));
    expect(onSelect).toHaveBeenLastCalledWith({ type: "pipe", id: "pipe:α-β" });
    fireEvent.click(screen.getByTestId("numerical-health-show-diagnostic-load:é"));
    expect(onShow).toHaveBeenCalledWith("diagnostic:numerical-integrity:load:é");
  });

  it("offers no solver-diagnostic link without an integrity diagnostic", () => {
    const raw = genericHealth(false);
    (raw.cases[0] as { integrity_diagnostic_id: string | null }).integrity_diagnostic_id = null;
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
      numericalHealth={view(raw, { eligible: false, verdictHold: false })} onShowDiagnostic={() => {}} />);
    expect(screen.queryByTestId("numerical-health-show-diagnostic-load:é")).toBeNull();
    expect(screen.getByTestId("numerical-health-show-diagnostic-load:𝔫")).toHaveTextContent("Show solver diagnostic");
  });

  it("prefixes the pressure-source header, keeps the recovery text, adds the eligible wording and places no row badges", () => {
    const result = JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/product_preview/pressure_source/physics_source/n05-dense_scrutiny.raw.json"), "utf8")) as MechanicsResult;
    const health = view({ route: "pressure_source", verdict_only: false, cases: [{ ...SENSITIVE, case_id: "case", integrity_diagnostic_id: "diagnostic:numerical-integrity:case", causes: [] }], combinations: [] },
      { eligible: true, verdictHold: false, recoveredCases: recoveredCaseIds(result) });
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={health} />);
    expect(screen.getByTestId("numerical-health-header").textContent).toBe("Ordinary solve — Numerical health: 1 sensitive");
    expect(shownStandingSentence()).toMatch(/^Ordinary solve: sensitive\. Producer recovery receipt: qualified\. /);
    expect(screen.queryByTestId("numerical-health-output-sentence")).toBeNull();
    expect(screen.getByTestId("numerical-health-case-text-case").textContent).toContain(
      "The values shown come from the producer's recovery of this case, which passed its own checks. This badge describes the ordinary solve only."
    );
    expect(screen.getByTestId("numerical-health-case-text-case").textContent).not.toContain("Its values are kept for inspection.");
    expect(screen.queryAllByTestId(/^result-row-health-/)).toHaveLength(0);
    expect(screen.getByTestId("physics-source-evidence")).toBeTruthy();
  });

  it("keeps today's line when the indicator is absent or its route disagrees", () => {
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={null} />);
    expect(shownStandingSentence()).toMatch(/^Numerical health: solver checks found no problem\. /);
    expect(screen.queryByTestId("numerical-health-detail")).toBeNull();
    cleanup();
    const pressureSource = view({ route: "pressure_source", verdict_only: false, cases: [PASSED], combinations: [] }, { eligible: true, verdictHold: false });
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={pressureSource} />);
    expect(shownStandingSentence()).toMatch(/^Numerical health: solver checks found no problem\. /);
  });
});

describe("the standing line beside the health header", () => {
  // Native-smoke expected.json pins this line's prefix (owner-held G10 observation): the Results
  // stage's header shows the standing line, which begins as today's text on every route; the rest of
  // the sentence is the first part of "Method and limits", so line and detail joined are exactly
  // today's text. The indicator's own header line is a separate element inside "Method and limits".
  const original = (result: MechanicsResult) => sourceContractText(result);
  function sourceContractText(result: MechanicsResult): string {
    return result.source_block_recovery
      ? `${ordinarySolveLead(result)} Producer recovery receipt: ${(result.source_block_recovery as { body: { status: string } }).body.status}. ${result.formulation_basis!.limitations.join(" ")} Current use is checked separately against the actual invocation and complete result evidence. Numerical checks do not establish engineering correctness.`
      : `${numericalHealthLead(result)} ${result.formulation_basis!.limitations.join(" ")} Numerical checks do not establish engineering correctness.`;
  }
  it.each([
    ["generic", () => genericResult(), () => view(genericHealth(true), { eligible: false, verdictHold: true })],
    ["pressure-source", () => JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/product_preview/pressure_source/physics_source/n05-dense_scrutiny.raw.json"), "utf8")) as MechanicsResult,
      () => view({ route: "pressure_source", verdict_only: false, cases: [{ ...SENSITIVE, case_id: "case", causes: [] }], combinations: [] }, { eligible: true, verdictHold: false })]
  ] as const)("keeps the %s standing text unchanged, its line in the header and the rest in Method and limits", (_, makeResult, makeHealth) => {
    const result = makeResult();
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={makeHealth()} />);
    const standing = screen.getByTestId("numerical-result-standing");
    expect(original(result).startsWith(`${standing.textContent} `)).toBe(true);
    expect(shownStandingSentence()).toBe(original(result));
    const header = screen.getByTestId("results-stage-header");
    const method = screen.getByTestId("results-method");
    expect(header).toContainElement(standing);
    expect(method).not.toContainElement(standing);
    // The indicator's header line and its per-case detail are inside "Method and limits", not in the line.
    expect(standing).not.toContainElement(screen.getByTestId("numerical-health-header"));
    expect(method).toContainElement(screen.getByTestId("numerical-health-header-line"));
    expect(method).toContainElement(screen.getByTestId("numerical-health-detail"));
    cleanup();
    // And the same text with no indicator at all.
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={null} />);
    expect(shownStandingSentence()).toBe(original(result));
    expect(screen.queryByTestId("numerical-health-header-line")).toBeNull();
    expect(screen.queryByTestId("numerical-health-detail")).toBeNull();
  });
});

describe("the per-case detail disclosure", () => {
  it("is summarised as Numerical health by load case, open with a non-passing case and collapsed when all pass", () => {
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
      numericalHealth={view(genericHealth(false), { eligible: false, verdictHold: false })} />);
    const open = screen.getByTestId("numerical-health-detail");
    expect(open.tagName).toBe("DETAILS");
    expect(open.querySelector("summary")?.textContent).toBe("Numerical health by load case");
    expect(open).toHaveAttribute("open");
    cleanup();
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
      numericalHealth={view({ route: "generic", verdict_only: false, cases: [PASSED, { ...PASSED, case_id: "load:é", integrity_diagnostic_id: "diagnostic:numerical-integrity:load:é" }], combinations: [] }, { eligible: true, verdictHold: false })} />);
    expect(screen.getByTestId("numerical-health-detail")).not.toHaveAttribute("open");
  });
});

describe("the boundary sentence", () => {
  const BOUNDARY = "These checks look for numerical trouble; they do not measure the error, and they say nothing about whether the model, its inputs or its design are right.";
  const count = () => (screen.getByTestId("numerical-health-detail").textContent ?? "").split(BOUNDARY).length - 1;
  it("is carried once by every detail, whatever its cases' states (REQ-023, AC-008)", () => {
    const notAssessed = { case_id: "load:é", state: "not_assessed", code: null, integrity_diagnostic_id: null, rcond: null, digits: null, conditioning_sensitive: false, causes: [] };
    const failed = { ...notAssessed, case_id: "load:𝔫", state: "solve_failed" };
    for (const [label, cases] of [
      ["a blocked document, every case suppressed or unassessed", [notAssessed]],
      ["a blocked document with a preserved failure", [notAssessed, failed]],
      ["a passing and a sensitive case", [PASSED, SENSITIVE]],
      ["passing only", [PASSED]]
    ] as const) {
      cleanup();
      render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
        numericalHealth={view({ route: "generic", verdict_only: false, cases, combinations: [] }, { eligible: false, verdictHold: false })} />);
      expect(count(), label).toBe(1);
      expect(screen.getByTestId("numerical-health-boundary").textContent).toBe(BOUNDARY);
    }
  });
});

describe("resolveLocatedCauseSelection", () => {
  const model = { nodes: [{ id: "N1" }], pipe_segments: [{ id: "P1" }], supports: [{ id: "S1" }], components: [], load_cases: [{ id: "L1" }] } as unknown as PreviewModel;
  it("maps a row label to its node and keeps the direct-id path for references", () => {
    expect(resolveLocatedCauseSelection(model, { kind: "node_dof", node: "N1", dof: "UX" })).toEqual({ type: "node", id: "N1" });
    expect(resolveLocatedCauseSelection(model, { kind: "node_dof", node: "P1", dof: "UX" })).toBeNull();
    expect(resolveLocatedCauseSelection(model, { kind: "ref", ref: "S1" })).toEqual({ type: "support", id: "S1" });
    expect(resolveLocatedCauseSelection(model, { kind: "ref", ref: "L1" })).toEqual({ type: "load", id: "L1" });
    expect(resolveLocatedCauseSelection(model, { kind: "ref", ref: "unknown" })).toBeNull();
    expect(resolveLocatedCauseSelection(model, { kind: "text" })).toBeNull();
  });
});

// The Results stage's header (DEL-07-05 REQ-011, REQ-023): the standing line with the checks-passed
// sentence as its hover text and as the first line of "Method and limits"; the per-case states beside
// it only where a case's state is not the line's; the pressure-source recovery sentence beside the
// ordinary solve's standing; "Method and limits" closed until opened. Words are read from the
// indicator's functions, never retyped here.
describe("the Results stage header", () => {
  it("carries the checks-passed sentence as the line's hover text and as the first line of Method and limits", () => {
    const allPassed = view({ route: "generic", verdict_only: false, cases: [PASSED], combinations: [] }, { eligible: true, verdictHold: false });
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={allPassed} />);
    const line = screen.getByTestId("numerical-result-standing");
    expect(line).toHaveAttribute("title", checksPassedText("load:𝔫"));
    const method = screen.getByTestId("results-method") as HTMLDetailsElement;
    expect(method.open).toBe(false);
    expect(method.querySelector("summary")!.textContent).toBe(METHOD_AND_LIMITS_LABEL);
    openMethodAndLimits();
    expect(within(method).getAllByTestId("results-method-standing-sentence").map((item) => item.textContent)).toEqual([checksPassedText("load:𝔫")]);
    expect(method.querySelector(".results-method-body")!.firstElementChild).toBe(within(method).getByTestId("results-method-standing-sentence"));
    // Every case is in the line's state: no per-case marks beside it.
    expect(screen.queryByTestId("results-stage-case-states")).toBeNull();
  });

  it("states beside the line only the cases whose state the line does not state, with the output sentence under a hold", () => {
    const health = view(genericHealth(true), { eligible: false, verdictHold: true });
    const result = genericResult();
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={health} />);
    const marks = screen.getByTestId("results-stage-case-states");
    expect(screen.getByTestId("results-stage-header")).toContainElement(marks);
    for (const state of ["sensitive", "checks_passed"] as const) expect(marks.textContent).toContain(`1 ${NUMERICAL_HEALTH_STATE_TEXT[state]}`);
    expect(marks.textContent).toContain(health.outputSentence!);
  });

  it("keeps the pressure-source recovery sentence beside the ordinary solve's standing, outside Method and limits", () => {
    const result = JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/product_preview/pressure_source/physics_source/n05-dense_scrutiny.raw.json"), "utf8")) as MechanicsResult;
    // The recovery's sentence is stated for a case whose values come from the recovery.
    const health = view({ route: "pressure_source", verdict_only: false, cases: [{ ...SENSITIVE, case_id: "case", integrity_diagnostic_id: null, causes: [] }], combinations: [] },
      { eligible: true, verdictHold: false, recoveredCases: recoveredCaseIds(result) });
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={health} />);
    const note = screen.getByTestId("results-stage-recovery-note");
    expect(note.textContent).toBe(ELIGIBLE_RECOVERY_TEXT);
    expect(screen.getByTestId("results-stage-header")).toContainElement(note);
    expect(screen.getByTestId("results-method")).not.toContainElement(note);
  });

  it("names the run and holds the case selector in the header", () => {
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={null}
      stage={{ identity: { kind: "reopened", runId: "run:r" } }} />);
    const header = screen.getByTestId("results-stage-header");
    expect(within(header).getByTestId("results-stage-run").textContent).toContain(RESULTS_RUN_LABELS.reopened);
    expect(header).toContainElement(screen.getByTestId("results-navigator-case-select"));
  });
});
