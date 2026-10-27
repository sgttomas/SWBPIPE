// A3, the sorted stress and ratio table panel (DEL-07-05 Design `stress-ratio-table.md` §5 to §8,
// §10): the display rule, the toggle, the threshold, pinned incomplete checks, highlighting and
// paging. Invented data only (`resultOverlayFixtures` and an invented rule pack).
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { healthStanding, overlayModel, overlayResult, row } from "../../test-support/resultOverlayFixtures";
import type { MechanicsResult } from "../../types";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";
import type { RuleCheckOutcome, RuleCheckStatus } from "../../services/ruleCheckService";
import {
  RATIO_DISABLED_EMITTED,
  RATIO_DISABLED_NO_RUN,
  RATIO_DIRECTION_TEXT,
  STRESS_RATIO_BOUNDARY_TEXT,
  STRESS_RATIO_KEY_TEXT,
  STRESS_RATIO_KEY_TEXT_UNSORTED,
  STRESS_RATIO_PAGE,
  STRESS_RATIO_PINNED_TEXT,
  StressRatioTablePanel
} from "./StressRatioTablePanel";
import { INCOMPLETE_MESSAGES_NOTE, RATIO_NOTE_COMPUTED_NOT_POSITIVE, STRESS_RATIO_NO_RESULT, SUBJECT_UNDECLARED_STATUS, STRESS_RATIO_NO_RUN, THRESHOLD_INVALID_NOTE, stressRatioRunText, unitsDifferNote } from "./stressRatioTable";
import { supportLoadWithheldStateText } from "./supportLoadSummary";

afterEach(cleanup);

type Row = MechanicsResult["results"][number];
const L1 = { ref_type: "load_case", ref_id: "load:L-1" } as const;
const L2 = { ref_type: "load_case", ref_id: "load:L-2" } as const;
const PASSING = healthStanding({ eligible: true });
const meta = (location: string) => ({ component: "sigma_plus", coordinate_system: "pipe_section", location, basis: "invented", sign_convention: "invented" });

function stressRows(): Row[] {
  return [
    row("s:P-1:i", "element_local_normal_stress_extreme", "pipe:P-1", 1234.5678, "MPa", { metadata: meta("end_i") }),
    row("s:P-2:i", "element_local_normal_stress_extreme", "pipe:P-2", -2345.6789, "MPa", { metadata: meta("end_i") }),
    row("s:P-2:L2", "element_local_normal_stress_extreme", "pipe:P-2", 7, "MPa", { basis_ref: L2, metadata: meta("end_i") })
  ];
}
const maximumRow = () => row("m:P-1", "pipe_elastic_normal_stress_maximum_v2", "pipe:P-1", 123456789.5, "Pa", { metadata: meta("governing_station") });

function rulePack(): SessionRuleCheckRun["rulePackDocument"] {
  const solver = (id: string, entity: string) => ({
    input_id: id, name: id, source_kind: "solver_result", quantity_intent: { dimension: "stress", unit_ref: "MPa" },
    solver_result_ref: { result_kind: "element_local_normal_stress_extreme", entity_ref: entity, location: "end_i", component: "sigma_plus" }
  });
  const check = (id: string, input: string) => ({ check_id: id, category_ref: { ref_type: "stress_category", ref_id: "cat_a" }, required_input_refs: [{ ref_type: "required_input", ref_id: input }] });
  return {
    required_inputs: [solver("in_p1", "pipe:P-1"), solver("in_p2", "pipe:P-2")],
    stress_categories: [{ category_id: "cat_a", name: "Invented A" }],
    check_definitions: [check("chk_p1", "in_p1"), check("chk_p2", "in_p2")]
  };
}

const quantity = (value: number, unit_ref = "MPa") => ({ value, unit_ref, dimension: "stress" });
function outcome(check: string, input: string, resultId: string | null, status: RuleCheckStatus, basis: typeof L1 | typeof L2, extra: Partial<RuleCheckOutcome> = {}): RuleCheckOutcome {
  return {
    check_id: check, category_ref: "cat_a", basis_ref: basis, status,
    acceptability_relation: status === "RULE_INPUTS_INCOMPLETE" ? "none" : "less_than_or_equal",
    bound_inputs: [{ input_id: input, source_kind: "solver_result", supplied: resultId !== null, ...(resultId ? { value: 1, unit: "MPa", result_id: resultId } : {}) }],
    completeness_findings: [], evaluator_findings: [], diagnostic_codes: [],
    ...extra
  };
}

function ruleRun(checks: RuleCheckOutcome[] = [
  outcome("chk_p1", "in_p1", "s:P-1:i", "USER_RULE_CHECKED", L1, { computed_value: quantity(1), limit_value: quantity(3) }),
  outcome("chk_p2", "in_p2", null, "RULE_INPUTS_INCOMPLETE", L2, { completeness_findings: [{ code: "RULE_SOLVER_RESULT_ROW_MISSING", severity: "blocking", subject_id: "in_p2", message: "no row matched" }] }),
  outcome("chk_p2", "in_p2", "s:P-2:i", "USER_RULE_FAILED", L1, { computed_value: quantity(4.5), limit_value: quantity(3) })
]): SessionRuleCheckRun {
  return {
    rulePackDocument: rulePack(),
    result: { document_kind: "rule_check_run_result", rule_pack_id: "invented_pack", grammar_version: "1.0.0", aggregate_status: "RULE_INPUTS_INCOMPLETE", checks, professional_boundary_notice: "invented" }
  };
}

function renderPanel(props: Partial<Parameters<typeof StressRatioTablePanel>[0]> = {}) {
  return render(<StressRatioTablePanel model={overlayModel()} result={overlayResult(stressRows())} standing={PASSING} {...props} />);
}

const bodyRows = () => within(screen.getByTestId("stress-ratio-table")).getAllByRole("row").slice(1);
const P1 = JSON.stringify(["chk_p1", "load_case", "load:L-1"]);
const P2_L1 = JSON.stringify(["chk_p2", "load_case", "load:L-1"]);
const P2_L2 = JSON.stringify(["chk_p2", "load_case", "load:L-2"]);

describe("StressRatioTablePanel: emitted stresses", () => {
  it("sorts by magnitude under the display rule, with the unit in the header and stress only", () => {
    const view = renderPanel({ result: overlayResult([...stressRows(), maximumRow()]) });
    expect(screen.getByTestId("stress-ratio-run").textContent).toBe(STRESS_RATIO_NO_RUN);
    // The first kind of the list opens first; a Pa row is shown in MPa.
    expect((screen.getByTestId("stress-ratio-group") as HTMLSelectElement).value).toBe("stress:pipe_elastic_normal_stress_maximum_v2");
    expect(screen.getByTestId("stress-ratio-value-m:P-1").textContent).toBe("123.457");
    expect(screen.getByRole("columnheader", { name: "Stress (MPa)" })).toBeTruthy();
    expect(screen.getByText("MPa", { exact: false, selector: "label" })).toBeTruthy();
    fireEvent.change(screen.getByTestId("stress-ratio-group"), { target: { value: "stress:element_local_normal_stress_extreme" } });
    const table = screen.getByRole("table", { name: "Section normal-stress extreme σ±: sorted by stress magnitude" });
    const header = within(table).getByRole("columnheader", { name: "Stress (MPa)" });
    expect(header.getAttribute("aria-sort")).toBe("descending");
    expect(bodyRows().map((item) => item.getAttribute("data-testid"))).toEqual(["stress-ratio-row-s:P-2:i", "stress-ratio-row-s:P-1:i", "stress-ratio-row-s:P-2:L2"]);
    expect(screen.getByTestId("stress-ratio-value-s:P-2:i").textContent).toBe("-2345.68");
    expect(screen.getByTestId("stress-ratio-value-s:P-1:i").textContent).toBe("1234.57");
    expect(screen.getByTestId("stress-ratio-value-s:P-1:i").querySelector("span")!.getAttribute("title")).toBe("1234.57 MPa (s:P-1:i)");
    const ratio = screen.getByTestId("stress-ratio-mode-ratio") as HTMLInputElement;
    expect(ratio.disabled).toBe(true);
    expect(screen.getByTestId("stress-ratio-mode-reason").textContent).toBe(RATIO_DISABLED_NO_RUN);
    expect((screen.getByTestId("stress-ratio-mode-stress") as HTMLInputElement).checked).toBe(true);
    // No stored value appears unrounded anywhere in the markup.
    for (const raw of ["1234.5678", "2345.6789", "123456789.5", "123.4567895"]) expect(view.container.innerHTML).not.toContain(raw);
    expect(screen.getByText(STRESS_RATIO_BOUNDARY_TEXT)).toBeTruthy();
  });

  it("applies a threshold at or above and states the hidden count", () => {
    renderPanel();
    fireEvent.change(screen.getByTestId("stress-ratio-threshold"), { target: { value: "1234.5678" } });
    expect(bodyRows().map((item) => item.getAttribute("data-testid"))).toEqual(["stress-ratio-row-s:P-2:i", "stress-ratio-row-s:P-1:i"]);
    expect(screen.getByTestId("stress-ratio-summary").textContent).toBe("Showing 2 of 3 rows; 1 below the threshold hidden.");
    fireEvent.change(screen.getByTestId("stress-ratio-threshold"), { target: { value: "x" } });
    expect(screen.getByTestId("stress-ratio-threshold-note").textContent).toBe(THRESHOLD_INVALID_NOTE);
    expect(bodyRows()).toHaveLength(3);
  });

  it("states a withheld case once, with its reason, and lists no value of it", () => {
    const view = renderPanel({ standing: healthStanding({ eligible: true, states: { "load:L-1": "checks_passed", "load:L-2": "sensitive" } }) });
    const withheld = screen.getByTestId("stress-ratio-withheld-load_case:load:L-2");
    expect(withheld.textContent).toContain(`Invented two (load:L-2): 1 row withheld. ${supportLoadWithheldStateText("sensitive")}`);
    expect(screen.queryByTestId("stress-ratio-row-s:P-2:L2")).toBeNull();
    expect(view.container.innerHTML).not.toContain("s:P-2:L2");
  });

  it("highlights a row's location by its button and marks the selected row", () => {
    const onSelectLocation = vi.fn();
    renderPanel({ onSelectLocation, selectedResultId: "s:P-1:i" });
    const button = screen.getByRole("button", { name: "Highlight pipe:P-2 · end_i · sigma_plus, Invented one (load:L-1)" });
    expect(button.getAttribute("type")).toBe("button");
    fireEvent.click(button);
    expect(onSelectLocation).toHaveBeenCalledWith({ resultId: "s:P-2:i", entityRef: "pipe:P-2" });
    expect(screen.getByTestId("stress-ratio-row-s:P-1:i").getAttribute("aria-current")).toBe("true");
    expect(screen.getByTestId("stress-ratio-row-s:P-1:i").className).toBe("selected-result-row");
    expect(screen.getByTestId("stress-ratio-row-s:P-2:i").getAttribute("aria-current")).toBeNull();
  });

  it("lists the ranked rows in pages", () => {
    const many = Array.from({ length: STRESS_RATIO_PAGE + 7 }, (_, index) => row(`a:${index}`, "element_local_axial_normal_stress", "pipe:P-1", index, "MPa"));
    renderPanel({ result: overlayResult(many) });
    expect(bodyRows()).toHaveLength(STRESS_RATIO_PAGE);
    // The live line counts the rows listed on this page, apart from those a later page lists.
    expect(screen.getByTestId("stress-ratio-summary").textContent).toBe(`Showing ${STRESS_RATIO_PAGE} of ${STRESS_RATIO_PAGE + 7} rows; 7 more not yet listed.`);
    fireEvent.click(screen.getByTestId("stress-ratio-more"));
    expect(bodyRows()).toHaveLength(STRESS_RATIO_PAGE + 7);
    expect(screen.getByTestId("stress-ratio-summary").textContent).toBe(`Showing ${STRESS_RATIO_PAGE + 7} of ${STRESS_RATIO_PAGE + 7} rows.`);
    expect(screen.queryByTestId("stress-ratio-more")).toBeNull();
  });

  it("says source order, not largest first, when a kind's units differ", () => {
    renderPanel({ result: overlayResult([
      row("a:mpa", "element_local_axial_normal_stress", "pipe:P-1", 1, "MPa"),
      row("a:pa", "element_local_axial_normal_stress", "pipe:P-2", 5e6, "Pa")
    ]) });
    fireEvent.change(screen.getByTestId("stress-ratio-group"), { target: { value: "stress:element_local_axial_normal_stress" } });
    const table = screen.getByRole("table", { name: "Axial normal stress: in source order, stress magnitude not sorted (units differ)" });
    expect(within(table).getByTestId("stress-ratio-column-stress").getAttribute("aria-sort")).toBe("none");
    expect(screen.getByTestId("stress-ratio-key").textContent).toBe(STRESS_RATIO_KEY_TEXT_UNSORTED.emitted_stress);
    expect(screen.getByTestId("stress-ratio-unit-note").textContent).toBe(unitsDifferNote(["MPa", "Pa"]));
    expect(screen.getByTestId("stress-ratio-key").textContent).not.toContain("largest first");
    expect(table.getAttribute("aria-label")).not.toContain("sorted by");
    // The first column states a position in source order, not a rank.
    expect(within(table).getAllByRole("columnheader")[0].textContent).toBe("Position");
  });

  it("states the unavailable state under a custom test-id prefix", () => {
    renderPanel({ result: null, testIdPrefix: "kr-stress" });
    expect(screen.getByTestId("kr-stress-unavailable").textContent).toBe(STRESS_RATIO_NO_RESULT);
    expect(screen.queryByTestId("stress-ratio-panel")).toBeNull();
  });
});

describe("StressRatioTablePanel: rule-check outcomes", () => {
  it("opens on the pack's category in ratio mode, pins the incomplete check and sorts by ratio", () => {
    const view = renderPanel({ ruleCheckRun: ruleRun() });
    expect(screen.getByTestId("stress-ratio-run").textContent).toBe(stressRatioRunText("invented_pack", 3));
    expect((screen.getByTestId("stress-ratio-group") as HTMLSelectElement).value).toBe("rule:cat_a");
    expect(screen.getByTestId("stress-ratio-mode").getAttribute("data-mode")).toBe("ratio");
    expect(screen.getByTestId("stress-ratio-pinned-note").textContent).toBe(STRESS_RATIO_PINNED_TEXT);
    const table = screen.getByRole("table", { name: "Invented A (cat_a): sorted by ratio" });
    expect(within(table).getByRole("columnheader", { name: "Ratio" }).getAttribute("aria-sort")).toBe("descending");
    expect(bodyRows().map((item) => [item.getAttribute("data-part"), item.getAttribute("data-status")])).toEqual([
      ["pinned", "RULE_INPUTS_INCOMPLETE"], ["ranked", "USER_RULE_FAILED"], ["ranked", "USER_RULE_CHECKED"]
    ]);
    expect(screen.getByTestId(`stress-ratio-status-${P2_L2}`).textContent).toBe("Incomplete (RULE_INPUTS_INCOMPLETE)");
    expect(screen.getByTestId(`stress-ratio-reason-${P2_L2}`).textContent).toBe(`Missing input: in_p2. Completeness finding RULE_SOLVER_RESULT_ROW_MISSING (in_p2). ${INCOMPLETE_MESSAGES_NOTE}`);
    // 4.5 / 3 = 1.5; 1 / 3 to six significant digits.
    expect(screen.getByTestId(`stress-ratio-value-${P2_L1}`).textContent).toBe("1.5");
    expect(screen.getByTestId(`stress-ratio-value-${P1}`).textContent).toBe("0.333333");
    expect(screen.getByTestId(`stress-ratio-value-${P1}`).querySelector("span")!.getAttribute("title")).toBe(`0.333333 (computed ÷ limit; ${P1})`);
    expect(view.container.innerHTML).not.toContain("0.3333333");
  });

  // HELP_HUMAN, 2026-10-27: "label each ratio's direction" (Design §3.1, §7).
  it("labels each ratio's direction, sorts both directions together and pins the incomplete check", () => {
    const LB = JSON.stringify(["chk_lb", "load_case", "load:L-1"]);
    const LB_NONE = JSON.stringify(["chk_lb", "load_case", "load:L-2"]);
    const view = renderPanel({ ruleCheckRun: ruleRun([
      outcome("chk_p1", "in_p1", "s:P-1:i", "USER_RULE_CHECKED", L1, { computed_value: quantity(1), limit_value: quantity(3) }),
      outcome("chk_p2", "in_p2", null, "RULE_INPUTS_INCOMPLETE", L2),
      outcome("chk_p2", "in_p2", "s:P-2:i", "USER_RULE_FAILED", L1, { computed_value: quantity(4.5), limit_value: quantity(3) }),
      outcome("chk_lb", "in_p1", null, "USER_RULE_FAILED", L1, { acceptability_relation: "greater_than_or_equal", computed_value: quantity(3), limit_value: quantity(7) }),
      outcome("chk_lb", "in_p1", null, "USER_RULE_CHECKED", L2, { acceptability_relation: "greater_than", computed_value: quantity(-2), limit_value: quantity(7) })
    ]) });
    expect(screen.getByTestId("stress-ratio-key").textContent).toBe(STRESS_RATIO_KEY_TEXT.ratio);
    const table = screen.getByRole("table", { name: "Invented A (cat_a): sorted by ratio" });
    expect(within(table).getAllByRole("columnheader").map((item) => item.textContent)).toEqual(["Rank", "Location", "Case", "Check", "Status", "Ratio", "Direction", "Relation"]);
    // 7 / 3 (lower) above 4.5 / 3 (upper) above 1 / 3 (upper); the lower bound over -2 forms none.
    expect(bodyRows().map((item) => [item.getAttribute("data-testid"), item.getAttribute("data-part")])).toEqual([
      [`stress-ratio-row-${P2_L2}`, "pinned"], [`stress-ratio-row-${LB}`, "ranked"], [`stress-ratio-row-${P2_L1}`, "ranked"],
      [`stress-ratio-row-${P1}`, "ranked"], [`stress-ratio-row-${LB_NONE}`, "unranked"]
    ]);
    expect(screen.getByTestId(`stress-ratio-value-${LB}`).textContent).toBe("2.33333");
    expect(screen.getByTestId(`stress-ratio-value-${LB}`).querySelector("span")!.getAttribute("title")).toBe(`2.33333 (limit ÷ computed; ${LB})`);
    expect(screen.getByTestId(`stress-ratio-value-${P2_L1}`).querySelector("span")!.getAttribute("title")).toBe(`1.5 (computed ÷ limit; ${P2_L1})`);
    expect(screen.getByTestId(`stress-ratio-direction-${LB}`).textContent).toBe("Lower bound: limit ÷ computed");
    expect(screen.getByTestId(`stress-ratio-direction-${P2_L1}`).textContent).toBe("Upper bound: computed ÷ limit");
    expect(screen.getByTestId(`stress-ratio-direction-${P1}`).textContent).toBe(RATIO_DIRECTION_TEXT.upper);
    expect(screen.getByTestId(`stress-ratio-value-${LB_NONE}`).textContent).toBe(RATIO_NOTE_COMPUTED_NOT_POSITIVE);
    expect(within(screen.getByTestId(`stress-ratio-direction-${LB_NONE}`)).getByText("none")).toBeTruthy();
    // The pinned row's reason spans the ratio, direction and relation columns.
    expect(screen.getByTestId(`stress-ratio-reason-${P2_L2}`).getAttribute("colspan")).toBe("3");
    expect(view.container.innerHTML).not.toContain("2.333333");
    // A threshold of 2.4 hides no failed check of either direction.
    fireEvent.change(screen.getByTestId("stress-ratio-threshold"), { target: { value: "2.4" } });
    expect(bodyRows().map((item) => item.getAttribute("data-testid"))).toEqual([
      `stress-ratio-row-${P2_L2}`, `stress-ratio-row-${LB}`, `stress-ratio-row-${P2_L1}`, `stress-ratio-row-${LB_NONE}`
    ]);
    // Stress mode has no direction column.
    fireEvent.click(screen.getByTestId("stress-ratio-mode-stress"));
    expect(screen.queryByTestId("stress-ratio-column-direction")).toBeNull();
    expect(screen.getByTestId(`stress-ratio-reason-${P2_L2}`).getAttribute("colspan")).toBe("2");
  });

  it("keeps the incomplete and failed checks under a threshold above every ratio", () => {
    renderPanel({ ruleCheckRun: ruleRun() });
    fireEvent.change(screen.getByTestId("stress-ratio-threshold"), { target: { value: "1000" } });
    expect(bodyRows().map((item) => item.getAttribute("data-status"))).toEqual(["RULE_INPUTS_INCOMPLETE", "USER_RULE_FAILED"]);
    expect(screen.getByTestId("stress-ratio-summary").textContent).toBe("Showing 2 of 3 rows; 1 below the threshold hidden.");
  });

  it("toggles to stress, which shows the computed value and limit and keeps its own threshold", () => {
    renderPanel({ ruleCheckRun: ruleRun() });
    fireEvent.change(screen.getByTestId("stress-ratio-threshold"), { target: { value: "1" } });
    fireEvent.click(screen.getByTestId("stress-ratio-mode-stress"));
    expect(screen.getByTestId("stress-ratio-mode").getAttribute("data-mode")).toBe("stress");
    expect((screen.getByTestId("stress-ratio-threshold") as HTMLInputElement).value).toBe("");
    expect(screen.getByRole("columnheader", { name: "Computed (MPa)" }).getAttribute("aria-sort")).toBe("descending");
    expect(screen.getByRole("columnheader", { name: "Limit (MPa)" })).toBeTruthy();
    expect(screen.getByTestId(`stress-ratio-value-${P2_L1}`).textContent).toBe("4.5");
    expect(screen.getByTestId(`stress-ratio-limit-${P2_L1}`).textContent).toBe("3");
    expect(bodyRows()[0].getAttribute("data-status")).toBe("RULE_INPUTS_INCOMPLETE");
    fireEvent.click(screen.getByTestId("stress-ratio-mode-ratio"));
    expect((screen.getByTestId("stress-ratio-threshold") as HTMLInputElement).value).toBe("1");
  });

  it("shows a withheld outcome with its reason and no status or value", () => {
    const view = renderPanel({ ruleCheckRun: ruleRun(), standing: healthStanding({ eligible: true, states: { "load:L-1": "sensitive", "load:L-2": "checks_passed" } }) });
    expect(bodyRows().map((item) => [item.getAttribute("data-part"), item.getAttribute("data-standing")])).toEqual([
      ["pinned", "admitted"], ["pinned", "withheld"], ["pinned", "withheld"]
    ]);
    expect(screen.getByTestId(`stress-ratio-status-${P1}`).textContent).toBe("Withheld");
    expect(screen.getByTestId(`stress-ratio-reason-${P1}`).textContent).toBe(supportLoadWithheldStateText("sensitive"));
    expect(view.container.textContent).not.toContain("USER_RULE_FAILED");
    expect(view.container.textContent).not.toContain("USER_RULE_CHECKED");
    expect(view.container.innerHTML).not.toContain("0.333333");
    // No pass or fail status in any attribute, title or class of a withheld row; incomplete stays stated.
    expect(view.container.innerHTML).not.toContain("USER_RULE");
    for (const key of [P1, P2_L1]) {
      const held = screen.getByTestId(`stress-ratio-row-${key}`);
      expect(held.hasAttribute("data-status")).toBe(false);
      expect(held.outerHTML).not.toMatch(/fail|checked|pass/i);
    }
    expect(screen.getByTestId(`stress-ratio-row-${P2_L2}`).getAttribute("data-status")).toBe("RULE_INPUTS_INCOMPLETE");
  });

  it("never shows a quantity from a finding's message, withheld or admitted (#1655)", () => {
    const tableMessage = "table argument 123456789.5 is outside the table range [0.5, 100.25]";
    const checks = [
      outcome("chk_p1", "in_p1", "s:P-1:i", "USER_RULE_CHECKED", L1, { computed_value: quantity(1), limit_value: quantity(3) }),
      outcome("chk_p2", "in_p2", null, "RULE_INPUTS_INCOMPLETE", L2, {
        computed_value: quantity(123456789.5, "Pa"), limit_value: quantity(98765432.1, "Pa"),
        completeness_findings: [{ code: "RULE_SOLVER_RESULT_ROW_MISSING", severity: "blocking", subject_id: "in_p2", message: "no row matched at 0.5" }],
        evaluator_findings: [{ code: "TableOutOfRange", severity: "blocking", subject_id: "chk_p2", message: tableMessage }]
      })
    ];
    const reason = `Missing input: in_p2. Completeness finding RULE_SOLVER_RESULT_ROW_MISSING (in_p2). Evaluator finding TableOutOfRange (chk_p2). ${INCOMPLETE_MESSAGES_NOTE}`;
    const withheldReason = `Missing input: in_p2. Completeness finding RULE_SOLVER_RESULT_ROW_MISSING. Evaluator finding TableOutOfRange. ${INCOMPLETE_MESSAGES_NOTE}`;
    const quantities = ["123456789", "123.457", "1.23457", "123457", "98765432", "98.7654", "100.25", "0.5", "table argument", "no row matched"];
    const withheld = healthStanding({ eligible: true, states: { "load:L-1": "checks_passed", "load:L-2": "sensitive" } });
    for (const mode of ["ratio", "stress"] as const) {
      // Withheld: the cause (codes, missing input) and the incomplete status stay, with no finding
      // subject (§3.2); no digit of a value anywhere.
      const held = renderPanel({ ruleCheckRun: ruleRun(checks), standing: withheld });
      fireEvent.click(screen.getByTestId(`stress-ratio-mode-${mode}`));
      const heldRow = screen.getByTestId(`stress-ratio-row-${P2_L2}`);
      expect([heldRow.getAttribute("data-standing"), heldRow.getAttribute("data-status")]).toEqual(["withheld", "RULE_INPUTS_INCOMPLETE"]);
      expect(screen.getByTestId(`stress-ratio-status-${P2_L2}`).textContent).toBe("Incomplete (RULE_INPUTS_INCOMPLETE)");
      expect(screen.getByTestId(`stress-ratio-reason-${P2_L2}`).textContent).toBe(`${withheldReason} ${supportLoadWithheldStateText("sensitive")}`);
      for (const fragment of quantities) expect(held.container.innerHTML).not.toContain(fragment);
      held.unmount();
      // Admitted: the structured reason with its subjects; the message's numbers are not shown in any form.
      const admitted = renderPanel({ ruleCheckRun: ruleRun(checks) });
      fireEvent.click(screen.getByTestId(`stress-ratio-mode-${mode}`));
      expect(screen.getByTestId(`stress-ratio-reason-${P2_L2}`).textContent).toBe(reason);
      for (const fragment of quantities) expect(admitted.container.innerHTML).not.toContain(fragment);
      admitted.unmount();
    }
  });

  describe("a downgraded check's undeclared status in a finding subject (§3.2, §4)", () => {
    // The runner's `enforce_declared` puts the evaluated status in a STATUS_NOT_DECLARED finding's
    // subject and reports the outcome RULE_INPUTS_INCOMPLETE.
    const downgraded = (evaluated: string) => [outcome("chk_p2", "in_p2", null, "RULE_INPUTS_INCOMPLETE", L2, {
      computed_value: quantity(4.5), limit_value: quantity(3),
      evaluator_findings: [{ code: "STATUS_NOT_DECLARED", severity: "blocking", subject_id: evaluated, message: "computed status is not in the check's declared result_statuses" }]
    })];
    const withheld = healthStanding({ eligible: true, states: { "load:L-1": "checks_passed", "load:L-2": "sensitive" } });
    const STATUS_TOKEN = /USER_RULE_(FAILED|CHECKED)|user-rule (failed|checked)|(failed|checked) by your rule/i;

    for (const evaluated of ["USER_RULE_FAILED", "USER_RULE_CHECKED"]) {
      for (const mode of ["ratio", "stress"] as const) {
        it(`${evaluated}, ${mode} mode: no status token in a withheld row; admitted reads "${SUBJECT_UNDECLARED_STATUS}"`, () => {
          const held = renderPanel({ ruleCheckRun: ruleRun(downgraded(evaluated)), standing: withheld });
          fireEvent.click(screen.getByTestId(`stress-ratio-mode-${mode}`));
          const heldRow = screen.getByTestId(`stress-ratio-row-${P2_L2}`);
          expect([heldRow.getAttribute("data-standing"), heldRow.getAttribute("data-status")]).toEqual(["withheld", "RULE_INPUTS_INCOMPLETE"]);
          expect(screen.getByTestId(`stress-ratio-status-${P2_L2}`).textContent).toBe("Incomplete (RULE_INPUTS_INCOMPLETE)");
          expect(screen.getByTestId(`stress-ratio-reason-${P2_L2}`).textContent)
            .toBe(`Missing input: in_p2. Evaluator finding STATUS_NOT_DECLARED. ${INCOMPLETE_MESSAGES_NOTE} ${supportLoadWithheldStateText("sensitive")}`);
          // Text, attributes, titles and aria of the row, and of the whole panel.
          expect(heldRow.textContent).not.toMatch(STATUS_TOKEN);
          expect(heldRow.outerHTML).not.toMatch(STATUS_TOKEN);
          for (const element of [heldRow, ...Array.from(heldRow.querySelectorAll("*"))]) {
            for (const attribute of Array.from(element.attributes)) expect(attribute.value, attribute.name).not.toMatch(STATUS_TOKEN);
          }
          expect(held.container.innerHTML).not.toMatch(STATUS_TOKEN);
          expect(held.container.innerHTML).not.toContain(SUBJECT_UNDECLARED_STATUS);
          held.unmount();

          const admitted = renderPanel({ ruleCheckRun: ruleRun(downgraded(evaluated)) });
          fireEvent.click(screen.getByTestId(`stress-ratio-mode-${mode}`));
          const admittedRow = screen.getByTestId(`stress-ratio-row-${P2_L2}`);
          expect([admittedRow.getAttribute("data-standing"), admittedRow.getAttribute("data-status")]).toEqual(["admitted", "RULE_INPUTS_INCOMPLETE"]);
          expect(screen.getByTestId(`stress-ratio-status-${P2_L2}`).textContent).toBe("Incomplete (RULE_INPUTS_INCOMPLETE)");
          expect(screen.getByTestId(`stress-ratio-reason-${P2_L2}`).textContent)
            .toBe(`Missing input: in_p2. Evaluator finding STATUS_NOT_DECLARED (${SUBJECT_UNDECLARED_STATUS}). ${INCOMPLETE_MESSAGES_NOTE}`);
          expect(admitted.container.innerHTML).not.toMatch(STATUS_TOKEN);
          admitted.unmount();
        });
      }
    }
  });

  it("keeps a failed outcome on the page when the mode moves it past the page length", () => {
    // Fifty checked outcomes with computed -100 .. -149 against a limit of 1, and one failed 2 / 1.
    const checked = Array.from({ length: STRESS_RATIO_PAGE + 7 }, (_, index) =>
      outcome(`chk_${index}`, "in_p1", null, "USER_RULE_CHECKED", L1, { bound_inputs: [], computed_value: quantity(-100 - index), limit_value: quantity(1) }));
    const failed = outcome("chk_failed", "in_p1", null, "USER_RULE_FAILED", L1, { bound_inputs: [], computed_value: quantity(2), limit_value: quantity(1) });
    const failedKey = JSON.stringify(["chk_failed", "load_case", "load:L-1"]);
    for (const count of [STRESS_RATIO_PAGE, STRESS_RATIO_PAGE + 7]) {
      const view = renderPanel({ ruleCheckRun: ruleRun([...checked.slice(0, count), failed]) });
      expect(bodyRows()[0].getAttribute("data-testid")).toBe(`stress-ratio-row-${failedKey}`);
      fireEvent.click(screen.getByTestId("stress-ratio-mode-stress"));
      const failedRow = screen.getByTestId(`stress-ratio-row-${failedKey}`);
      expect([failedRow.getAttribute("data-part"), failedRow.getAttribute("data-status")]).toEqual(["ranked", "USER_RULE_FAILED"]);
      // Its rank is its own place in the stress order, past the page.
      expect(within(failedRow).getByRole("rowheader").textContent).toBe(String(count + 1));
      expect(bodyRows()).toHaveLength(STRESS_RATIO_PAGE + 1);
      expect(screen.getByTestId("stress-ratio-summary").textContent).toBe(count === STRESS_RATIO_PAGE
        ? `Showing ${count + 1} of ${count + 1} rows.`
        : `Showing ${STRESS_RATIO_PAGE + 1} of ${count + 1} rows; ${count - STRESS_RATIO_PAGE} more not yet listed.`);
      expect(screen.queryByTestId("stress-ratio-more") !== null).toBe(count > STRESS_RATIO_PAGE);
      // A threshold above every value keeps it, alone.
      fireEvent.change(screen.getByTestId("stress-ratio-threshold"), { target: { value: "1000" } });
      expect(bodyRows().map((item) => item.getAttribute("data-status"))).toEqual(["USER_RULE_FAILED"]);
      view.unmount();
    }
  });

  it("highlights an outcome by its bound row, or by the selector's entity", () => {
    const onSelectLocation = vi.fn();
    renderPanel({ ruleCheckRun: ruleRun(), onSelectLocation });
    const rows = bodyRows();
    fireEvent.click(within(rows[0]).getByRole("button"));
    expect(onSelectLocation).toHaveBeenLastCalledWith({ resultId: null, entityRef: "pipe:P-2" });
    fireEvent.click(within(rows[2]).getByRole("button", { name: "Highlight pipe:P-1 · end_i · sigma_plus, Invented one (load:L-1)" }));
    expect(onSelectLocation).toHaveBeenLastCalledWith({ resultId: "s:P-1:i", entityRef: "pipe:P-1" });
  });

  it("disables ratio for an emitted-stress category while a run is held", () => {
    renderPanel({ ruleCheckRun: ruleRun() });
    fireEvent.change(screen.getByTestId("stress-ratio-group"), { target: { value: "stress:element_local_normal_stress_extreme" } });
    expect((screen.getByTestId("stress-ratio-mode-ratio") as HTMLInputElement).disabled).toBe(true);
    expect(screen.getByTestId("stress-ratio-mode-reason").textContent).toBe(RATIO_DISABLED_EMITTED);
    expect(screen.queryByTestId("stress-ratio-pinned-note")).toBeNull();
  });
});
