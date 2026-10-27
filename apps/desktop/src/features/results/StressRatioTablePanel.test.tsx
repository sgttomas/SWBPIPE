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
  STRESS_RATIO_BOUNDARY_TEXT,
  STRESS_RATIO_PAGE,
  STRESS_RATIO_PINNED_TEXT,
  StressRatioTablePanel
} from "./StressRatioTablePanel";
import { STRESS_RATIO_NO_RESULT, STRESS_RATIO_NO_RUN, THRESHOLD_INVALID_NOTE, stressRatioRunText } from "./stressRatioTable";
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
    expect(screen.getByTestId("stress-ratio-summary").textContent).toBe(`Showing ${STRESS_RATIO_PAGE + 7} of ${STRESS_RATIO_PAGE + 7} rows.`);
    fireEvent.click(screen.getByTestId("stress-ratio-more"));
    expect(bodyRows()).toHaveLength(STRESS_RATIO_PAGE + 7);
    expect(screen.queryByTestId("stress-ratio-more")).toBeNull();
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
    expect(screen.getByTestId(`stress-ratio-reason-${P2_L2}`).textContent).toBe("Missing input: in_p2. RULE_SOLVER_RESULT_ROW_MISSING (in_p2): no row matched");
    // 4.5 / 3 = 1.5; 1 / 3 to six significant digits.
    expect(screen.getByTestId(`stress-ratio-value-${P2_L1}`).textContent).toBe("1.5");
    expect(screen.getByTestId(`stress-ratio-value-${P1}`).textContent).toBe("0.333333");
    expect(screen.getByTestId(`stress-ratio-value-${P1}`).querySelector("span")!.getAttribute("title")).toBe(`0.333333 (computed ÷ limit; ${P1})`);
    expect(view.container.innerHTML).not.toContain("0.3333333");
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
