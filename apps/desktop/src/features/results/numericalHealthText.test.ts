// DEL-07-05 numerical health §3, §5 and §7: every literal of numericalHealthText.ts
// pinned to the design's wording, and the module's vocabulary fence.
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
      checks_passed: "not verified",
      sensitive: "sensitive",
      quality_unresolved: "quality unresolved",
      solve_failed: "solve failed",
      not_assessed: "not assessed"
    });
    expect(text.numericalHealthHeadline("checks_passed")).toBe("Numerical health: not verified");
    expect(text.numericalHealthHeadline("sensitive")).toBe("Numerical health: sensitive");
    expect(text.numericalHealthHeadline("quality_unresolved")).toBe("Numerical health: quality unresolved");
    expect(text.numericalHealthHeadline("solve_failed")).toBe("Numerical health: solve failed");
    expect(text.numericalHealthHeadline("not_assessed")).toBe("Numerical health: not assessed");
    expect(text.ORDINARY_SOLVE_PREFIX).toBe("Ordinary solve —");
    expect(text.numericalHealthHeadline("sensitive", true)).toBe("Ordinary solve — Numerical health: sensitive");
  });

  it("pins the header count form", () => {
    expect(text.numericalHealthCountHeadline([["solve_failed", 0], ["quality_unresolved", 0], ["sensitive", 2], ["not_assessed", 0], ["checks_passed", 1]]))
      .toBe("Numerical health: 2 sensitive, 1 not verified");
    expect(text.numericalHealthCountHeadline([["solve_failed", 0], ["quality_unresolved", 0], ["sensitive", 0], ["not_assessed", 0], ["checks_passed", 3]]))
      .toBe("Numerical health: not verified");
    expect(text.numericalHealthCountHeadline([["solve_failed", 1], ["quality_unresolved", 1], ["sensitive", 1], ["not_assessed", 1], ["checks_passed", 1]], true))
      .toBe("Ordinary solve — Numerical health: 1 solve failed, 1 quality unresolved, 1 sensitive, 1 not assessed, 1 not verified");
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
    expect(text.checksPassedText("LC1")).toBe("The solver's own checks found no problem with LC1, but SWBPIPE has not verified an error bound for its values.");
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
      "Inherited from its load cases: LC1: sensitive; LC2: not verified."
    );
    expect(text.conditioningDetailText("10.5")).toBe(
      "Estimated sensitivity of the scaled linear solve: rounding can be magnified by roughly 10.5 of the about 16 decimal digits the computer carries (the solver marks a case sensitive at about 7.8). This is an estimate; it does not estimate the error in individual results."
    );
    expect(text.DIGITS_BELOW_FIRST_DECIMAL).toBe("below 0.1");
    expect(text.nodeDofLocationText("N1", "UX")).toBe("node N1, UX");
    expect(text.SHOW_SOLVER_DIAGNOSTIC).toBe("Show solver diagnostic");
    expect(text.LOCATED_CAUSES_LABEL).toBe("Named locations");
    expect(text.DETAIL_SECTION_LABEL).toBe("Numerical health by load case");
    expect(text.reportLineText("checks_passed")).toBe("Numerical health of the ordinary solve: not verified.");
  });

  it("pins the module's exports, so a new literal comes with a pin", () => {
    expect(Object.keys(text).sort()).toEqual([
      "BOUNDARY_SENTENCE", "CONDITIONING_CAUSE_TEXT", "DETAIL_SECTION_LABEL", "DIGITS_BELOW_FIRST_DECIMAL",
      "ELIGIBLE_RECOVERY_TEXT", "KEPT_FOR_INSPECTION_TEXT", "LOCATED_CAUSES_LABEL", "MECHANISM_TEXT",
      "NEGATIVE_ENERGY_TEXT", "NUMERICAL_HEALTH_LABEL", "NUMERICAL_HEALTH_STATE_TEXT", "ORDINARY_SOLVE_PREFIX",
      "QUALITY_UNRESOLVED_CLOSING_TEXT", "RECOVERY_BASIS_UNQUALIFIED_TEXT", "SEE_DIAGNOSTIC_TEXT",
      "SENSITIVE_CLOSING_TEXT", "SHOW_SOLVER_DIAGNOSTIC", "checksPassedText", "combinationText",
      "conditioningDetailText", "formationCauseText", "formationRecoveryCauseText", "loadFidelityCauseText",
      "nodeDofLocationText", "notAssessedText", "numericalHealthCountHeadline", "numericalHealthHeadline",
      "outputSentence", "qualityUnresolvedLeadText", "reportLineText", "sensitiveLeadText", "solveFailedLeadText"
    ].sort());
  });
});

describe("numerical health vocabulary", () => {
  // DEL-07-05 REQ-023 and AC-008, as changed for release exit criterion 18 (LIM-026): no text of the
  // indicator uses certified, guaranteed, valid, approved, compliant, correct or reliable; it uses
  // verified and bound only to deny them, in the phrases "not verified" and "has not verified an error
  // bound". Each permitted denial is taken out only as a whole phrase at word boundaries, so "a knot
  // verified" or "unbound" keeps its word. The note ("Reported by the solver; not verified by
  // SWBPIPE.") denies by the first phrase.
  const DENIALS = [/\bhas not verified an error bound\b/gi, /\bnot verified\b/gi];
  const FENCED = ["verified", "bound", "certified", "guaranteed", "valid", "approved", "compliant", "correct", "reliable"];
  function fencedWords(value: string): string[] {
    const rest = DENIALS.reduce((left, denial) => left.replace(denial, " "), value);
    return FENCED.filter((word) => new RegExp(`\\b${word}\\b`, "i").test(rest));
  }
  const STATES = Object.keys(text.NUMERICAL_HEALTH_STATE_TEXT) as text.NumericalHealthState[];
  const OUTCOMES = Object.keys(status.CONFIRMATION_STATEMENTS);
  const shown = (value: unknown): string[] => typeof value === "string" ? [value] : value && typeof value === "object" ? Object.values(value).flatMap(shown) : [];
  /** Every string each export of the indicator's two text modules can show, by export: a constant
   * as it is, a record's values, a function called over every state, outcome and form it takes. */
  const INDICATOR: Record<string, () => string[]> = {
    // numericalHealthText.ts: every export is indicator text.
    ...Object.fromEntries(Object.entries(text).filter(([, value]) => typeof value !== "function").map(([name, value]) => [`text.${name}`, () => shown(value)])),
    "text.numericalHealthHeadline": () => STATES.flatMap((state) => [text.numericalHealthHeadline(state), text.numericalHealthHeadline(state, true)]),
    "text.numericalHealthCountHeadline": () => [true, false].flatMap((prefix) => [text.numericalHealthCountHeadline(STATES.map((state) => [state, 1] as const), prefix), text.numericalHealthCountHeadline([["checks_passed", 2]], prefix)]),
    "text.outputSentence": () => [text.outputSentence(2)],
    "text.checksPassedText": () => [text.checksPassedText("LC1")],
    "text.sensitiveLeadText": () => [text.sensitiveLeadText("LC1")],
    "text.loadFidelityCauseText": () => [text.loadFidelityCauseText(""), text.loadFidelityCauseText("load:a")],
    "text.formationCauseText": () => [text.formationCauseText(""), text.formationCauseText("node N1, UX")],
    "text.formationRecoveryCauseText": () => [text.formationRecoveryCauseText(""), text.formationRecoveryCauseText("member forces")],
    "text.qualityUnresolvedLeadText": () => [text.qualityUnresolvedLeadText("LC1")],
    "text.solveFailedLeadText": () => [text.solveFailedLeadText("LC1")],
    "text.notAssessedText": () => [text.notAssessedText("LC1")],
    "text.combinationText": () => [text.combinationText(STATES.map((state) => ["LC1", state] as const))],
    "text.conditioningDetailText": () => [text.conditioningDetailText("10.5")],
    "text.nodeDofLocationText": () => [text.nodeDofLocationText("N1", "UX")],
    "text.reportLineText": () => STATES.map(text.reportLineText),
    // integrityStatus.ts: the indicator's note, support-state lines and results-line leads.
    "status.SOLVER_REPORT_NOTE": () => [status.SOLVER_REPORT_NOTE],
    "status.CONFIRMATION_STATEMENTS": () => shown(status.CONFIRMATION_STATEMENTS),
    "status.SUPPORT_STATES_LEAD": () => [status.SUPPORT_STATES_LEAD],
    "status.DETAIL_NOT_CARRIED_TEXT": () => [status.DETAIL_NOT_CARRIED_TEXT],
    "status.CONFIRMATION_RECORD_UNREAD_STATEMENT": () => [status.CONFIRMATION_RECORD_UNREAD_STATEMENT],
    "status.SUPPORT_STATES_UNREAD_LINE": () => [status.SUPPORT_STATES_UNREAD_LINE.text],
    "status.supportStatesText": () => OUTCOMES.flatMap((outcome) => shown(status.supportStatesText(outcome))),
    "status.supportStatesLine": () => OUTCOMES.flatMap((outcome) => [null, "x"].flatMap((detail) => { const line = status.supportStatesLine(outcome, detail); return line ? [line.text, ...shown(line.detailNote)] : []; })),
    "status.numericalQualityStatusText": () => [...STATES, "checks_passed", "unresolved", "failed"].map(status.numericalQualityStatusText),
    "status.numericalHealthLead": () => ["checks_passed", "sensitive"].map((value) => status.numericalHealthLead({ numerical_quality: { status: value } } as never)),
    "status.ordinarySolveLead": () => ["checks_passed", "sensitive"].map((value) => status.ordinarySolveLead({ numerical_quality: { status: value } } as never))
  };
  /** The exports of integrityStatus.ts that are not indicator text, with what they are. */
  const NOT_INDICATOR: Record<string, string> = {
    CHECKS_PASSED_CODE: "the solver's code, matched, never shown as the desktop's words",
    SUPPORT_STATE_ESTABLISHED_TEXT: "the solver's own sentence, matched in its text",
    SOLVER_CLAIM_PHRASES: "the solver's own phrases, matched in its text",
    solverTextClaim: "the classification of a code",
    needsSolverReportNote: "the predicate",
    limitationsWithNotes: "the solver's limitation texts with the note",
    withLimitationsNote: "the solver's limitation texts with the note",
    standingStatusText: "the reader's standing status words, outside the indicator",
    INTEGRITY_BOUND_NOT_VERIFIED_TEXT: "the reader verdict's bound member in words, outside the indicator",
    integrityBoundText: "the reader verdict's bound member in words, outside the indicator",
    supportStatesByCase: "reads records into supportStatesLine",
    SUPPORT_STATES_NONE: "no line on any case: holds no text",
    SUPPORT_STATES_UNREAD: "SUPPORT_STATES_UNREAD_LINE on every case",
    supportStatesOf: "picks a case's line",
    nonlinearSupportKnowledge: "reads a model's supports",
    supportStatesFor: "decides the lines from a model and a record",
    nativeSupportStates: "decides the lines from a native summary",
    SOURCE_RECOVERY_METHOD: "a method token, matched",
    recoveredCaseIds: "reads a result's recorded methods"
  };

  it("classifies every export of both modules, so a new string comes with a check", () => {
    const named = new Set([...Object.keys(INDICATOR), ...Object.keys(NOT_INDICATOR).map((name) => `status.${name}`)]);
    const exported = [...Object.keys(text).map((name) => `text.${name}`), ...Object.keys(status).map((name) => `status.${name}`)];
    expect(exported.filter((name) => !named.has(name)), "unclassified exports").toEqual([]);
    expect([...named].filter((name) => !exported.includes(name)), "classified names that are not exports").toEqual([]);
  });

  it.each(Object.keys(INDICATOR))("%s uses no fenced word but in the two denials", (name) => {
    const strings = INDICATOR[name]();
    expect(strings.length, name).toBeGreaterThan(0);
    for (const value of strings) expect(fencedWords(value), `${name}: ${value}`).toEqual([]);
  });

  it("finds a fenced word that only looks like a denial", () => {
    expect(fencedWords("a knot verified")).toEqual(["verified"]);
    expect(fencedWords("not verifiedly bound")).toEqual(["bound"]);
    expect(fencedWords("an error bound is verified")).toEqual(["verified", "bound"]);
    expect(fencedWords("checks passed and verified")).toEqual(["verified"]);
    expect(fencedWords("it is correct")).toEqual(["correct"]);
    expect(fencedWords("Reported by the solver; not verified by SWBPIPE.")).toEqual([]);
    expect(fencedWords("SWBPIPE has not verified an error bound for its values.")).toEqual([]);
  });

  // The indicator as rendered, so a sentence written inline in a component is read too: the health
  // header and output sentence, its detail, the row badges, and the status bar's chip and popover,
  // for each of the five states and for the pressure-source route. The solver's own quoted text
  // (a support-state line's quoted reason) is not the indicator's and is left out.
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
    const detail = screen.getByTestId("numerical-health-detail").cloneNode(true) as HTMLElement;
    for (const quoted of detail.querySelectorAll("blockquote")) quoted.remove();
    shown.detail = textOf(detail);
    shown.rows = [...document.querySelectorAll("[data-testid^='result-row-health-']")].map(textOf).join(" | ");
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

  it.each(RENDERED)("the rendered indicator ($name) uses no fenced word but in the two denials", ({ result, view }) => {
    const shown = renderedIndicator(result(), view());
    expect(shown.header.length).toBeGreaterThan(0);
    expect(shown.detail.length).toBeGreaterThan(0);
    // Not vacuous: the status bar shows the indicator's headline.
    const shownView = view();
    expect(shown.statusPopover, "the status bar's popover").toContain(shownView.headline);
    expect(shown.header).toContain(shownView.headline);
    for (const [where, value] of Object.entries(shown)) expect(fencedWords(value), `${where}: ${value}`).toEqual([]);
  });

  it("finds a fenced word written into the rendered indicator", () => {
    const view = stateView("generic", "checks_passed", ["load:é", "load:𝔫"], true);
    const inline = { ...view, headline: "Numerical health: verified", cases: view.cases.map((c) => ({ ...c, sentences: [...c.sentences, "The bound is reliable."] })) } as NumericalHealthView;
    const shown = renderedIndicator(fixture(UNICODE), inline);
    expect(fencedWords(shown.header), shown.header).toEqual(["verified"]);
    expect(fencedWords(shown.detail), shown.detail).toEqual(["bound", "reliable"]);
    expect(fencedWords(shown.statusPopover), shown.statusPopover).toEqual(["verified"]);
  });
});
