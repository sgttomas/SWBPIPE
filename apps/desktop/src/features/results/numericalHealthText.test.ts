// DEL-07-05 numerical health §3, §5 and §7: every literal of numericalHealthText.ts
// pinned to the design's wording, the module's vocabulary fence, and the indicator's headlines as rendered.
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { createElement } from "react";
import { afterEach, describe, expect, it } from "vitest";
import * as text from "./numericalHealthText";
import * as status from "./integrityStatus";
import { numericalHealthView, parseNumericalHealth, type NumericalHealthView } from "./numericalHealth";
import { ResultsPanel } from "./ResultsPanel";
import { ShellStatusBar } from "../workspace/shell/ShellStatusBar";
import { WorkspaceSessionProvider } from "../workspace/WorkspaceSessionContext";
import type { WorkspaceSession } from "../workspace/workspaceSession";
import type { MechanicsResult } from "../../types";

afterEach(cleanup);

describe("numerical health text pins", () => {
  it("pins the label, states, headlines and prefix", () => {
    expect(text.NUMERICAL_HEALTH_LABEL).toBe("Numerical health");
    expect(text.NUMERICAL_HEALTH_STATE_TEXT).toEqual({
      checks_passed: "solver checks found no problem",
      sensitive: "sensitive",
      quality_unresolved: "quality unresolved",
      solve_failed: "solve failed",
      not_assessed: "not assessed"
    });
    expect(text.numericalHealthHeadline("checks_passed")).toBe("Numerical health: solver checks found no problem");
    expect(text.numericalHealthHeadline("checks_passed", true)).toBe("Ordinary solve — Numerical health: solver checks found no problem");
    expect(text.numericalHealthHeadline("sensitive")).toBe("Numerical health: sensitive");
    expect(text.numericalHealthHeadline("quality_unresolved")).toBe("Numerical health: quality unresolved");
    expect(text.numericalHealthHeadline("solve_failed")).toBe("Numerical health: solve failed");
    expect(text.numericalHealthHeadline("not_assessed")).toBe("Numerical health: not assessed");
    expect(text.ORDINARY_SOLVE_PREFIX).toBe("Ordinary solve —");
    expect(text.numericalHealthHeadline("sensitive", true)).toBe("Ordinary solve — Numerical health: sensitive");
  });

  it("pins the header count form", () => {
    expect(text.numericalHealthCountHeadline([["solve_failed", 0], ["quality_unresolved", 0], ["sensitive", 2], ["not_assessed", 0], ["checks_passed", 1]]))
      .toBe("Numerical health: 2 sensitive, 1 no problem found");
    expect(text.numericalHealthCountHeadline([["solve_failed", 0], ["quality_unresolved", 0], ["sensitive", 0], ["not_assessed", 0], ["checks_passed", 3]]))
      .toBe("Numerical health: solver checks found no problem");
    expect(text.numericalHealthCountHeadline([["solve_failed", 1], ["quality_unresolved", 1], ["sensitive", 1], ["not_assessed", 1], ["checks_passed", 1]], true))
      .toBe("Ordinary solve — Numerical health: 1 solve failed, 1 quality unresolved, 1 sensitive, 1 not assessed, 1 no problem found");
  });

  it("pins the count form's word for each state", () => {
    expect(text.numericalHealthCountText("checks_passed")).toBe("no problem found");
    for (const state of ["sensitive", "quality_unresolved", "solve_failed", "not_assessed"] as const) expect(text.numericalHealthCountText(state)).toBe(text.NUMERICAL_HEALTH_STATE_TEXT[state]);
  });

  it("pins the pressure-source, output and boundary sentences", () => {
    expect(text.ELIGIBLE_RECOVERY_TEXT).toBe(
      "The values shown come from the producer's recovery of this case, which passed its own checks. This badge describes the ordinary solve only."
    );
    expect(text.outputSentence(2)).toBe(
      "Rule checks, the result JSON and stress-neutral exports and the report package do not use this run while 2 load case(s) are sensitive, quality unresolved, solve failed or not assessed; results tables, CSV export and, for an exact-route result, the rendered report carry each case's standing."
    );
    expect(text.BOUNDARY_SENTENCE).toBe(
      "These checks look for numerical trouble; they do not measure the error, and they say nothing about whether the model, its inputs or its design are right."
    );
  });

  it("pins every detail text", () => {
    expect(text.checksPassedText("LC1")).toBe("The solver's residual and conditioning checks found no problem with LC1.");
    expect(text.sensitiveLeadText("LC1")).toBe("The solver's checks flagged LC1 as numerically sensitive.");
    expect(text.KEPT_FOR_INSPECTION_TEXT).toBe("Its values are kept for inspection.");
    expect(text.CONDITIONING_CAUSE_TEXT).toBe("The scaled stiffness is poorly conditioned.");
    expect(text.loadFidelityCauseText("load:a")).toBe(
      "The solver could not confirm that every applied load was carried at full precision (load:a)."
    );
    expect(text.formationCauseText("node N1, UX")).toBe(
      "The solver's formation check could not confirm some stiffness or load terms at full precision (node N1, UX)."
    );
    expect(text.formationRecoveryCauseText("member forces")).toBe(
      "The solver could not confirm some recovered member or support quantities at full precision (member forces)."
    );
    expect(text.loadFidelityCauseText("")).toBe("The solver could not confirm that every applied load was carried at full precision.");
    expect(text.formationCauseText("")).toBe("The solver's formation check could not confirm some stiffness or load terms at full precision.");
    expect(text.formationRecoveryCauseText("")).toBe("The solver could not confirm some recovered member or support quantities at full precision.");
    expect(text.SEE_DIAGNOSTIC_TEXT).toBe("See the solver diagnostic for the reason.");
    expect(text.SENSITIVE_CLOSING_TEXT).toBe(
      "Solving the unchanged model again in the same solver mode gives the same result. A very short or very stiff element next to flexible ones, stiffnesses that differ by many orders of magnitude, or a weakly restrained direction are common causes."
    );
    expect(text.qualityUnresolvedLeadText("LC1")).toBe("The solver could not confirm the numerical quality of LC1 at working precision.");
    expect(text.QUALITY_UNRESOLVED_CLOSING_TEXT).toBe("A near-mechanism or extreme stiffness contrast are common causes.");
    expect(text.RECOVERY_BASIS_UNQUALIFIED_TEXT).toBe(
      "Its support-state solution is kept for inspection, but its numerical quality could not be confirmed."
    );
    expect(text.solveFailedLeadText("LC1")).toBe("The solver could not complete a solution for LC1.");
    expect(text.MECHANISM_TEXT).toBe("The model can move without resistance (a mechanism). Review connectivity, member releases and supports.");
    expect(text.NEGATIVE_ENERGY_TEXT).toBe("The assembled stiffness is not positive in some direction. Review element and support properties.");
    expect(text.notAssessedText("LC1")).toBe("No numerical quality is stated for LC1.");
    expect(text.combinationText([["LC1", "sensitive"], ["LC2", "checks_passed"]])).toBe(
      "Inherited from its load cases: LC1: sensitive; LC2: solver checks found no problem."
    );
    expect(text.conditioningDetailText("10.5")).toBe(
      "Estimated sensitivity of the scaled linear solve: rounding can be magnified by roughly 10.5 of the about 16 decimal digits the computer carries (the solver marks a case sensitive at about 7.8). This is an estimate; it does not estimate the error in individual results."
    );
    expect(text.DIGITS_BELOW_FIRST_DECIMAL).toBe("below 0.1");
    expect(text.nodeDofLocationText("N1", "UX")).toBe("node N1, UX");
    expect(text.SHOW_SOLVER_DIAGNOSTIC).toBe("Show solver diagnostic");
    expect(text.LOCATED_CAUSES_LABEL).toBe("Named locations");
    expect(text.DETAIL_SECTION_LABEL).toBe("Numerical health by load case");
    expect(text.reportLineText("checks_passed")).toBe("Numerical health of the ordinary solve: solver checks found no problem.");
  });

  it("pins the module's exports, so a new literal comes with a pin", () => {
    expect(Object.keys(text).sort()).toEqual([
      "BOUNDARY_SENTENCE", "CONDITIONING_CAUSE_TEXT", "DETAIL_SECTION_LABEL", "DIGITS_BELOW_FIRST_DECIMAL",
      "ELIGIBLE_RECOVERY_TEXT", "KEPT_FOR_INSPECTION_TEXT", "LOCATED_CAUSES_LABEL", "MECHANISM_TEXT",
      "NEGATIVE_ENERGY_TEXT", "NUMERICAL_HEALTH_LABEL", "NUMERICAL_HEALTH_STATE_TEXT", "ORDINARY_SOLVE_PREFIX",
      "QUALITY_UNRESOLVED_CLOSING_TEXT", "RECOVERY_BASIS_UNQUALIFIED_TEXT", "SEE_DIAGNOSTIC_TEXT",
      "SENSITIVE_CLOSING_TEXT", "SHOW_SOLVER_DIAGNOSTIC", "checksPassedText", "combinationText",
      "conditioningDetailText", "formationCauseText", "formationRecoveryCauseText", "loadFidelityCauseText",
      "nodeDofLocationText", "notAssessedText", "numericalHealthCountHeadline", "numericalHealthCountText", "numericalHealthHeadline",
      "outputSentence", "qualityUnresolvedLeadText", "reportLineText", "sensitiveLeadText", "solveFailedLeadText"
    ].sort());
  });
});

describe("the rendered indicator", () => {
  const STATES = Object.keys(text.NUMERICAL_HEALTH_STATE_TEXT) as text.NumericalHealthState[];
  // The health header, its detail and the status bar's chip and popover, rendered for each of the
  // five states and for the pressure-source route.
  const root = resolve(__dirname, "../../../../..");
  const fixture = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8")) as MechanicsResult;
  const UNICODE = "fixtures/results/preview_physics_unicode_ids_sparse.json";
  const PRESSURE_SOURCE = "fixtures/product_preview/pressure_source/physics_source/mixed-sparse_interactive.raw.json";
  const CAUSES: Record<text.NumericalHealthState, { code: string; causes: unknown[] }[]> = {
    checks_passed: [{ code: "NUMERICAL_INTEGRITY_CHECKS_PASSED", causes: [] }],
    sensitive: [{ code: "NUMERICAL_INTEGRITY_SENSITIVE", causes: [{ kind: "formation", diagnostic_id: "d", locations: [{ kind: "node_dof", node: "node:α", dof: "UX" }] }, { kind: "load_fidelity", diagnostic_id: "d", locations: [{ kind: "ref", ref: "load:a" }] }, { kind: "formation_recovery", diagnostic_id: "d", locations: [{ kind: "text", text: "member end forces" }] }] }, { code: "NUMERICAL_INTEGRITY_SENSITIVE", causes: [] }],
    quality_unresolved: [{ code: "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", causes: [] }, { code: "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED", causes: [] }],
    solve_failed: [{ code: "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM", causes: [{ kind: "mechanism", diagnostic_id: "d", locations: [] }] }, { code: "NUMERICAL_INTEGRITY_NEGATIVE_ENERGY", causes: [{ kind: "negative_energy", diagnostic_id: "d", locations: [] }] }],
    not_assessed: [{ code: "NUMERICAL_INTEGRITY_INSPECTION_ONLY", causes: [] }, { code: "NUMERICAL_INTEGRITY_INSPECTION_ONLY", causes: [] }]
  };
  function stateView(route: "generic" | "pressure_source", state: text.NumericalHealthState, caseIds: readonly string[], eligible: boolean): NumericalHealthView {
    const variants = CAUSES[state];
    const health = parseNumericalHealth({
      route, verdict_only: !eligible && route === "generic",
      cases: caseIds.map((case_id, index) => ({ case_id, state, code: variants[index % variants.length].code, integrity_diagnostic_id: `diagnostic:numerical-integrity:${case_id}`, rcond: state === "checks_passed" || state === "sensitive" ? 1e-11 : null, digits: state === "checks_passed" || state === "sensitive" ? 11 : null, conditioning_sensitive: state === "sensitive", causes: variants[index % variants.length].causes })),
      combinations: []
    });
    expect(health, `${route} ${state}`).not.toBeNull();
    return numericalHealthView(health, { eligible, verdictHold: !eligible && route === "generic", recoveredCases: route === "pressure_source" ? status.recoveredCaseIds(fixture(PRESSURE_SOURCE)) : undefined })!;
  }
  /** An element's text, each text node read on its own (adjacent elements' text is not run together). */
  const textOf = (element: Element): string => {
    const parts: string[] = [];
    const walker = document.createTreeWalker(element, NodeFilter.SHOW_TEXT);
    for (let node = walker.nextNode(); node; node = walker.nextNode()) parts.push(node.textContent ?? "");
    return parts.join(" ");
  };
  /** Every text of the rendered indicator, by where it is shown. */
  function renderedIndicator(result: MechanicsResult, view: NumericalHealthView): Record<string, string> {
    render(createElement(ResultsPanel, { result, knowledge: null, analysisRun: null, selectedResultId: null, onSelectResult: () => {}, numericalHealth: view }));
    const shown: Record<string, string> = {};
    const header = screen.getByTestId("numerical-health-header-line");
    shown.header = textOf(header);
    shown.detail = textOf(screen.getByTestId("numerical-health-detail"));
    cleanup();
    const hold = { headline: view.headline, shortHeadline: view.shortHeadline, outputSentence: view.outputSentence ?? text.outputSentence(1) };
    const session = {
      model: { model: { analysis_status: { mechanics: "MECHANICS_SOLVED" } } },
      results: { result: { status: { mechanics: "MECHANICS_SOLVED", professional_acceptance: "NOT_PROVIDED" } }, currentSolvedResult: null, numericalHealthHold: hold, historicalRun: null, solveJob: { state: "completed" }, ruleCheckAggregate: null },
      selection: { selection: null, orderedSelection: { orderedKeys: [] } },
      chrome: { activeSection: "results", stageSurface: "results", auditDrawerOpen: false, setAuditDrawerOpen() {}, issuesDrawerOpen: false, setIssuesDrawerOpen() {} }
    } as unknown as WorkspaceSession;
    render(createElement(WorkspaceSessionProvider, { session, children: createElement(ShellStatusBar, { issueCount: 0 }) }));
    const chip = screen.getByTestId("status-pill-mechanics");
    shown.statusBar = `${textOf(chip)} ${chip.getAttribute("title")}`;
    fireEvent.click(chip);
    shown.statusPopover = textOf(screen.getByTestId("status-pill-mechanics-popover"));
    cleanup();
    return shown;
  }
  const RENDERED = [
    ...STATES.map((state) => ({ name: `generic, ${state}`, result: () => fixture(UNICODE), view: () => stateView("generic", state, ["load:é", "load:𝔫"], state === "checks_passed") })),
    { name: "pressure-source, eligible", result: () => fixture(PRESSURE_SOURCE), view: () => stateView("pressure_source", "checks_passed", ["case", "case:ordinary-pressure"], true) },
    { name: "pressure-source, sensitive", result: () => fixture(PRESSURE_SOURCE), view: () => stateView("pressure_source", "sensitive", ["case", "case:ordinary-pressure"], false) }
  ];

  it.each(RENDERED)("shows its headline in the header and the status bar ($name)", ({ result, view }) => {
    const shownView = view();
    const shown = renderedIndicator(result(), shownView);
    expect(shown.header).toContain(shownView.headline);
    expect(shown.statusPopover, "the status bar's popover").toContain(shownView.headline);
    expect(shown.detail).toContain(text.BOUNDARY_SENTENCE);
  });

  it("the five states have five headlines", () => {
    expect(new Set(STATES.map((state) => stateView("generic", state, ["load:é"], state === "checks_passed").headline)).size).toBe(5);
  });
});
