import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";
import { healthStanding, overlayModel, overlayResult, row } from "../../test-support/resultOverlayFixtures";
import type { MechanicsResult, PreviewModel } from "../../types";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";
import type { RuleCheckOutcome, RuleCheckStatus } from "../../services/ruleCheckService";
import type { ResultOverlayStanding } from "./numericalHealth";
import { SUPPORT_LOAD_WITHHELD_NO_HEALTH, SUPPORT_LOAD_WITHHELD_NOT_ELIGIBLE, supportLoadWithheldStateText } from "./supportLoadSummary";
import {
  INCOMPLETE_NO_REASON,
  RATIO_NOTE_LIMIT_NOT_POSITIVE,
  RATIO_NOTE_LOWER_BOUND,
  RATIO_NOTE_MISSING,
  RATIO_NOTE_PREDICATE,
  STRESS_KINDS,
  STRESS_RATIO_NO_RESULT,
  STRESS_RATIO_NO_RUN,
  STRESS_RATIO_NOT_SOLVED,
  STRESS_RATIO_NOTHING,
  STRESS_RATIO_RUN_NO_OUTCOMES,
  STRESS_RATIO_RUN_NOT_THIS_RESULT,
  THRESHOLD_INVALID_NOTE,
  buildStressRatioTable,
  ratioNoteUnitsDiffer,
  stressRatioLocationText,
  stressRatioRunText,
  stressRatioView,
  thresholdInStoredUnit,
  unitsDifferNote,
  type StressRatioGroup,
  type StressRatioMode,
  type StressRatioTable,
  type StressRatioView
} from "./stressRatioTable";

// A3, the sorted stress and ratio table (DEL-07-05 Design `stress-ratio-table.md` §2 to §5, §10).
// All data are invented (`resultOverlayFixtures` and the invented rule pack below); no value has
// engineering meaning.

type Row = MechanicsResult["results"][number];
const L1 = { ref_type: "load_case", ref_id: "load:L-1" } as const;
const L2 = { ref_type: "load_case", ref_id: "load:L-2" } as const;
const PASSING = healthStanding({ eligible: true });
const SENSITIVE_L2 = healthStanding({ eligible: true, states: { "load:L-1": "checks_passed", "load:L-2": "sensitive" } });

const station = (location: string, component = "sigma_plus", basis = "invented") =>
  ({ component, coordinate_system: "pipe_section", location, basis, sign_convention: "invented" });

function table(rows: Row[], standing: ResultOverlayStanding | null = PASSING, ruleCheckRun: SessionRuleCheckRun | null = null, model: PreviewModel = overlayModel()): StressRatioTable {
  return buildStressRatioTable({ model, result: overlayResult(rows), standing, ruleCheckRun });
}

function group(built: StressRatioTable, id: string): StressRatioGroup {
  const found = built.groups.find((item) => item.id === id);
  expect(found, id).toBeDefined();
  return found!;
}

function view(built: StressRatioTable, id: string, mode: StressRatioMode = "ratio", threshold = ""): StressRatioView {
  return stressRatioView(group(built, id), { mode, threshold });
}

const rankedKeys = (shown: StressRatioView) => shown.ranked.filter((item) => !item.hidden).map((item) => item.row.key);

// --- An invented rule pack and run --------------------------------------------------------------

/** Solver inputs at P-1 end_i and at P-2 end_i (a selector only), and one limit slot. */
function pack(): SessionRuleCheckRun["rulePackDocument"] {
  const solver = (id: string, entity: string, location: string) => ({
    input_id: id, name: id, source_kind: "solver_result",
    quantity_intent: { dimension: "stress", unit_ref: "MPa" },
    solver_result_ref: { result_kind: "element_local_normal_stress_extreme", entity_ref: entity, location, component: "sigma_plus" }
  });
  const check = (id: string, category: string, inputs: string[]) => ({
    check_id: id, category_ref: { ref_type: "stress_category", ref_id: category },
    required_input_refs: inputs.map((ref) => ({ ref_type: "required_input", ref_id: ref }))
  });
  return {
    required_inputs: [solver("in_p1", "pipe:P-1", "end_i"), solver("in_p2", "pipe:P-2", "end_i"),
      { input_id: "limit_value", name: "limit", source_kind: "user_supplied_rule_value", quantity_intent: { dimension: "stress", unit_ref: "MPa" } }],
    stress_categories: [{ category_id: "cat_b", name: "Invented B" }, { category_id: "cat_a", name: "Invented A" }],
    required_load_cases: [
      { required_load_case_id: "rl1", name: "one", basis_ref: L1 },
      { required_load_case_id: "rl2", name: "two", basis_ref: L2 }
    ],
    load_case_mappings: [
      { category_ref: { ref_type: "stress_category", ref_id: "cat_a" }, required_load_case_refs: [{ ref_type: "required_load_case", ref_id: "rl1" }, { ref_type: "required_load_case", ref_id: "rl2" }] },
      { category_ref: { ref_type: "stress_category", ref_id: "cat_b" }, required_load_case_refs: [{ ref_type: "required_load_case", ref_id: "rl1" }] }
    ],
    check_definitions: [
      check("chk_p1", "cat_a", ["in_p1", "limit_value"]),
      check("chk_p2", "cat_a", ["in_p2", "limit_value"]),
      check("chk_both", "cat_b", ["in_p1", "in_p2", "limit_value"])
    ]
  };
}

const quantity = (value: number, unit_ref = "MPa", dimension = "stress") => ({ value, unit_ref, dimension });

function outcome(check: string, category: string, basis: typeof L1 | typeof L2 | null, status: RuleCheckStatus, extra: Partial<RuleCheckOutcome> = {}): RuleCheckOutcome {
  return {
    check_id: check, category_ref: category, basis_ref: basis, status,
    acceptability_relation: status === "RULE_INPUTS_INCOMPLETE" ? "none" : "less_than_or_equal",
    bound_inputs: [], completeness_findings: [], evaluator_findings: [], diagnostic_codes: [],
    ...extra
  };
}

/** An outcome of check `chk_p1` (input `in_p1`) bound to `resultId`. */
function bound(resultId: string | undefined, extra: Partial<RuleCheckOutcome> & { status: RuleCheckStatus; basis?: typeof L1 | typeof L2; check?: string; category?: string; input?: string }): RuleCheckOutcome {
  const { status, basis = L1, check = "chk_p1", category = "cat_a", input = "in_p1", ...rest } = extra;
  return outcome(check, category, basis, status, {
    bound_inputs: [{ input_id: input, source_kind: "solver_result", supplied: resultId !== undefined, ...(resultId ? { value: 1, unit: "MPa", result_id: resultId } : {}) }],
    ...rest
  });
}

function run(checks: RuleCheckOutcome[]): SessionRuleCheckRun {
  return {
    rulePackDocument: pack(),
    result: {
      document_kind: "rule_check_run_result", rule_pack_id: "invented_pack", grammar_version: "1.0.0",
      aggregate_status: checks.some((item) => item.status === "RULE_INPUTS_INCOMPLETE") ? "RULE_INPUTS_INCOMPLETE" : "USER_RULE_CHECKED",
      checks, professional_boundary_notice: "invented"
    }
  };
}

const stressRows = (): Row[] => [
  row("s:P-1:i", "element_local_normal_stress_extreme", "pipe:P-1", -40, "MPa", { metadata: station("end_i") }),
  row("s:P-1:j", "element_local_normal_stress_extreme", "pipe:P-1", 30, "MPa", { metadata: station("end_j") }),
  row("s:P-2:i", "element_local_normal_stress_extreme", "pipe:P-2", 80, "MPa", { metadata: station("end_i") }),
  row("s:P-2:i:L2", "element_local_normal_stress_extreme", "pipe:P-2", 50, "MPa", { basis_ref: L2, metadata: station("end_i") }),
  row("d:A", "displacement_magnitude", "node:A", 9, "mm")
];

describe("stress ratio table: stress kinds", () => {
  it("lists every kind a pinned semantic contract gives the stress family, once each", () => {
    const dir = path.join(process.cwd(), "..", "..", "fixtures", "results");
    const files = readdirSync(dir).filter((name) => /^semantic_contract_v0_3_.*\.json$/.test(name));
    expect(files.length).toBeGreaterThanOrEqual(5);
    const contract = new Set<string>();
    for (const name of files) {
      const parsed = JSON.parse(readFileSync(path.join(dir, name), "utf8")) as { rows?: Array<{ kind: string; family?: string }> };
      for (const entry of parsed.rows ?? []) if (entry.family === "stress") contract.add(entry.kind);
    }
    const listed = STRESS_KINDS.map((item) => item.kind);
    expect(new Set(listed).size).toBe(listed.length);
    expect([...listed].sort()).toEqual([...contract].sort());
  });
});

describe("stress ratio table: emitted stress groups (no rule-check run)", () => {
  it("sorts a kind's rows by magnitude, largest first, keeps signed values and offers stress only", () => {
    const built = table(stressRows());
    expect(built.state).toBe("available");
    expect(built.run).toMatchObject({ state: "none", note: STRESS_RATIO_NO_RUN });
    expect(built.groups.map((item) => item.id)).toEqual(["stress:element_local_normal_stress_extreme"]);
    const extreme = group(built, "stress:element_local_normal_stress_extreme");
    expect(extreme.modes).toEqual(["stress"]);
    const shown = stressRatioView(extreme, { mode: "ratio", threshold: "" });
    expect(shown.mode).toBe("stress");
    expect(shown.sorted).toBe(true);
    expect(shown.keyUnit).toBe("MPa");
    expect(rankedKeys(shown)).toEqual(["s:P-2:i", "s:P-2:i:L2", "s:P-1:i", "s:P-1:j"]);
    const first = shown.ranked[0].row;
    expect(first.stress).toEqual({ value: 80, unit: "MPa", resultId: "s:P-2:i" });
    expect(first.target).toEqual({ resultId: "s:P-2:i", entityRef: "pipe:P-2" });
    expect(stressRatioLocationText(first.locations[0])).toBe("pipe:P-2 · end_i · sigma_plus");
    expect(shown.ranked[2].row.stress!.value).toBe(-40);
    expect(shown.ranked[1].row.caseLabel).toBe("Invented two (load:L-2)");
    expect(shown).toMatchObject({ pinned: [], unranked: [], hiddenCount: 0, shownCount: 4, totalCount: 4 });
  });

  it("ties keep result-row order, and 0 ties -0", () => {
    const rows = [
      row("a", "element_local_axial_normal_stress", "pipe:P-1", -5, "MPa"),
      row("b", "element_local_axial_normal_stress", "pipe:P-2", 5, "MPa"),
      row("c", "element_local_axial_normal_stress", "pipe:P-3", -0, "MPa"),
      row("d", "element_local_axial_normal_stress", "pipe:P-1", 0, "MPa", { basis_ref: L2 })
    ];
    expect(rankedKeys(view(table(rows), "stress:element_local_axial_normal_stress", "stress"))).toEqual(["a", "b", "c", "d"]);
  });

  it("orders groups by the kind list, skips non-finite rows and labels range-envelope rows", () => {
    const rows = [
      row("t", "element_local_torsional_shear_stress", "pipe:P-1", 2, "MPa"),
      row("m", "pipe_elastic_normal_stress_maximum_v2", "pipe:P-1", 2e6, "Pa"),
      row("nan", "pipe_elastic_normal_stress_maximum_v2", "pipe:P-2", Number.NaN, "Pa"),
      row("env", "pipe_elastic_normal_stress_maximum_v2", "pipe:P-3", 9e6, "Pa", { basis_ref: L2, metadata: station("governing_station", "maximum_absolute_normal_stress", "explicit_user_range_envelope") })
    ];
    const built = table(rows);
    expect(built.groups.map((item) => item.label)).toEqual(["Normal-stress maximum", "Torsional shear stress"]);
    const shown = view(built, "stress:pipe_elastic_normal_stress_maximum_v2", "stress");
    expect(rankedKeys(shown)).toEqual(["env", "m"]);
    expect(shown.ranked[0].row.caseLabel).toBe("Invented two (load:L-2) (range envelope)");
  });

  it("does not sort or filter rows of one kind in more than one unit", () => {
    const rows = [
      row("p", "pipe_lame_hoop_stress_v2", "pipe:P-1", 1e6, "Pa"),
      row("q", "pipe_lame_hoop_stress_v2", "pipe:P-2", 9, "MPa"),
      row("r", "pipe_lame_hoop_stress_v2", "pipe:P-3", 4e6, "Pa")
    ];
    const shown = view(table(rows), "stress:pipe_lame_hoop_stress_v2", "stress", "100");
    expect(shown.sorted).toBe(false);
    expect(shown.keyUnit).toBeNull();
    expect(shown.unitNote).toBe(unitsDifferNote(["MPa", "Pa"]));
    expect(shown.threshold).toEqual({ state: "disabled", reason: unitsDifferNote(["MPa", "Pa"]) });
    expect(rankedKeys(shown)).toEqual(["p", "q", "r"]);
    expect(shown.hiddenCount).toBe(0);
  });

  it("compares a threshold at or above, in the display unit (MPa for Pa rows) by a decimal shift", () => {
    const rows = [
      row("a", "pipe_elastic_normal_stress_maximum_v2", "pipe:P-1", 1.1e6, "Pa"),
      row("b", "pipe_elastic_normal_stress_maximum_v2", "pipe:P-2", 1099999.9999999998, "Pa"),
      row("c", "pipe_elastic_normal_stress_maximum_v2", "pipe:P-3", -3e6, "Pa")
    ];
    const built = table(rows);
    const shown = view(built, "stress:pipe_elastic_normal_stress_maximum_v2", "stress", "1.1");
    expect(shown.threshold).toEqual({ state: "applied", text: "1.1", value: 1100000 });
    expect(rankedKeys(shown)).toEqual(["c", "a"]);
    expect(shown).toMatchObject({ hiddenCount: 1, shownCount: 2, totalCount: 3 });
    expect(view(built, "stress:pipe_elastic_normal_stress_maximum_v2", "stress", "abc").threshold).toEqual({ state: "invalid", text: "abc" });
    expect(view(built, "stress:pipe_elastic_normal_stress_maximum_v2", "stress", " ").threshold).toEqual({ state: "none" });
    expect(THRESHOLD_INVALID_NOTE).toMatch(/no threshold/);
  });

  it("reads threshold text exactly", () => {
    expect(thresholdInStoredUnit("1.1", "Pa")).toBe(1100000);
    // A floating multiply would not give the decimal: 1.0000001 * 1e6 is 1000000.1000000001.
    expect(thresholdInStoredUnit("1.0000001", "Pa")).toBe(1000000.1);
    expect(1.0000001 * 1e6).not.toBe(1000000.1);
    expect(thresholdInStoredUnit("2.5e-3", "Pa")).toBe(2500);
    expect(thresholdInStoredUnit(".5", "MPa")).toBe(0.5);
    expect(thresholdInStoredUnit("-1", null)).toBe(-1);
    expect(thresholdInStoredUnit("1,5", null)).toBeNull();
    expect(thresholdInStoredUnit("1e999", null)).toBeNull();
  });

  it("withholds rows by the overlays' standing and states each withheld case once", () => {
    const shown = view(table(stressRows(), SENSITIVE_L2), "stress:element_local_normal_stress_extreme", "stress", "1000");
    expect(rankedKeys(shown)).toEqual([]);
    expect(shown.ranked.map((item) => item.row.key)).not.toContain("s:P-2:i:L2");
    const extreme = group(table(stressRows(), SENSITIVE_L2), "stress:element_local_normal_stress_extreme");
    expect(extreme.withheldCases).toHaveLength(1);
    expect(extreme.withheldCases[0]).toMatchObject({ rowCount: 1, reason: supportLoadWithheldStateText("sensitive") });
    expect(extreme.withheldCases[0].case.key).toBe("load_case:load:L-2");
    expect(shown).toMatchObject({ hiddenCount: 3, totalCount: 4, shownCount: 1 });
    for (const [standing, reason] of [[healthStanding({ eligible: false }), SUPPORT_LOAD_WITHHELD_NOT_ELIGIBLE], [null, SUPPORT_LOAD_WITHHELD_NO_HEALTH]] as const) {
      const held = group(table(stressRows(), standing), "stress:element_local_normal_stress_extreme");
      expect(held.rows).toEqual([]);
      expect(held.withheldCases.map((item) => [item.rowCount, item.reason])).toEqual([[3, reason], [1, reason]]);
    }
  });

  it("states the unavailable cases", () => {
    expect(buildStressRatioTable({ model: overlayModel(), result: null, standing: PASSING }).reason).toBe(STRESS_RATIO_NO_RESULT);
    const unsolved = { ...overlayResult(stressRows()), status: { mechanics: "MODEL_INCOMPLETE", rule_check: "not_run", professional_acceptance: "not_claimed" } } as unknown as MechanicsResult;
    expect(buildStressRatioTable({ model: overlayModel(), result: unsolved, standing: PASSING }).reason).toBe(STRESS_RATIO_NOT_SOLVED);
    const none = table([row("d:A", "displacement_magnitude", "node:A", 9, "mm")]);
    expect(none).toMatchObject({ state: "unavailable", reason: STRESS_RATIO_NOTHING, groups: [] });
  });
});

describe("stress ratio table: rule-check outcomes", () => {
  const outcomes = () => [
    bound("s:P-1:i", { status: "USER_RULE_CHECKED", computed_value: quantity(40), limit_value: quantity(100) }),
    bound("s:P-2:i:L2", { status: "USER_RULE_CHECKED", basis: L2, input: "in_p2", check: "chk_p2", computed_value: quantity(50), limit_value: quantity(100) }),
    bound(undefined, { status: "RULE_INPUTS_INCOMPLETE", basis: L2, completeness_findings: [{ code: "RULE_SOLVER_RESULT_ROW_MISSING", severity: "blocking", subject_id: "in_p1", message: "no row matched" }] }),
    bound("s:P-2:i", { status: "USER_RULE_FAILED", input: "in_p2", check: "chk_p2", computed_value: quantity(120), limit_value: quantity(100) })
  ];

  it("groups outcomes by the pack's stress categories in pack order, before the emitted stresses", () => {
    const checks = [...outcomes(), bound("s:P-1:i", { status: "USER_RULE_CHECKED", check: "chk_both", category: "cat_b", computed_value: quantity(1, "1", "dimensionless"), limit_value: quantity(2, "1", "dimensionless") })];
    const built = table(stressRows(), PASSING, run(checks));
    expect(built.run).toEqual({ state: "used", rulePackId: "invented_pack", outcomeCount: 5, note: stressRatioRunText("invented_pack", 5) });
    expect(built.groups.map((item) => [item.id, item.label, item.modes])).toEqual([
      ["rule:cat_b", "Invented B (cat_b)", ["ratio", "stress"]],
      ["rule:cat_a", "Invented A (cat_a)", ["ratio", "stress"]],
      ["stress:element_local_normal_stress_extreme", "Section normal-stress extreme σ±", ["stress"]]
    ]);
  });

  it("sorts by ratio, pins the incomplete outcome with its reason and keeps the failed one first", () => {
    const shown = view(table(stressRows(), PASSING, run(outcomes())), "rule:cat_a");
    expect(shown.pinned.map((item) => item.outcome!.status)).toEqual(["RULE_INPUTS_INCOMPLETE"]);
    expect(shown.pinned[0].outcome!.incompleteReason).toBe("Missing input: in_p1. RULE_SOLVER_RESULT_ROW_MISSING (in_p1): no row matched");
    expect(shown.pinned[0].outcome!.ratio).toBeNull();
    expect(shown.ranked.map((item) => [item.row.outcome!.checkId, item.row.case?.basis.ref_id, item.key])).toEqual([
      ["chk_p2", "load:L-1", 1.2], ["chk_p2", "load:L-2", 0.5], ["chk_p1", "load:L-1", 0.4]
    ]);
    expect(shown.ranked[0].row.outcome!.status).toBe("USER_RULE_FAILED");
    expect(shown.keyUnit).toBeNull();
  });

  it("keeps an incomplete outcome visible and pinned under every mode and threshold (FR-RULE-003)", () => {
    const group0 = group(table(stressRows(), PASSING, run(outcomes())), "rule:cat_a");
    for (const mode of ["ratio", "stress"] as const) {
      for (const threshold of ["", "0", "0.45", "1", "1e9", "-1e9", "junk"]) {
        const shown = stressRatioView(group0, { mode, threshold });
        expect(shown.pinned.map((item) => item.outcome!.status), `${mode} ${threshold}`).toEqual(["RULE_INPUTS_INCOMPLETE"]);
        expect(shown.ranked.some((item) => item.row.outcome!.status === "RULE_INPUTS_INCOMPLETE")).toBe(false);
        expect(shown.unranked.some((item) => item.outcome!.status === "RULE_INPUTS_INCOMPLETE")).toBe(false);
        // A failed outcome is never hidden by the threshold either.
        expect(shown.ranked.find((item) => item.row.outcome!.status === "USER_RULE_FAILED")!.hidden).toBe(false);
      }
    }
  });

  it("hides only checked outcomes below the threshold", () => {
    const shown = view(table(stressRows(), PASSING, run(outcomes())), "rule:cat_a", "ratio", "1.5");
    expect(shown.threshold).toEqual({ state: "applied", text: "1.5", value: 1.5 });
    expect(rankedKeys(shown)).toEqual([JSON.stringify(["chk_p2", "load_case", "load:L-1"])]);
    expect(shown).toMatchObject({ hiddenCount: 2, totalCount: 4, shownCount: 2 });
    const atRatio = view(table(stressRows(), PASSING, run(outcomes())), "rule:cat_a", "ratio", "0.5");
    expect(atRatio.hiddenCount).toBe(1);
  });

  it("sorts by the computed value's magnitude in stress mode, with the threshold in the display unit", () => {
    const checks = [
      bound("s:P-1:i", { status: "USER_RULE_CHECKED", computed_value: quantity(4e7, "Pa"), limit_value: quantity(1e8, "Pa") }),
      bound("s:P-2:i", { status: "USER_RULE_CHECKED", input: "in_p2", check: "chk_p2", computed_value: quantity(-6e7, "Pa"), limit_value: quantity(1e8, "Pa") })
    ];
    const shown = view(table(stressRows(), PASSING, run(checks)), "rule:cat_a", "stress", "50");
    expect(shown.keyUnit).toBe("Pa");
    expect(shown.ranked.map((item) => [item.row.outcome!.checkId, item.key, item.hidden])).toEqual([["chk_p2", 6e7, false], ["chk_p1", 4e7, true]]);
    const mixed = run([checks[0], bound("s:P-2:i", { status: "USER_RULE_CHECKED", input: "in_p2", check: "chk_p2", computed_value: quantity(60), limit_value: quantity(100) })]);
    const unsorted = view(table(stressRows(), PASSING, mixed), "rule:cat_a", "stress", "50");
    expect(unsorted.sorted).toBe(false);
    expect(unsorted.threshold.state).toBe("disabled");
    // The ratio is unitless, so ratio mode still sorts them.
    expect(view(table(stressRows(), PASSING, mixed), "rule:cat_a", "ratio").sorted).toBe(true);
  });

  it("forms a ratio only under an upper-bound relation, in one unit, with a positive limit", () => {
    const cases: Array<[Partial<RuleCheckOutcome>, number | null, string | null]> = [
      [{ computed_value: quantity(3), limit_value: quantity(4), acceptability_relation: "less_than" }, 0.75, null],
      [{ computed_value: quantity(3), limit_value: quantity(4), acceptability_relation: "greater_than_or_equal" }, null, RATIO_NOTE_LOWER_BOUND],
      [{ computed_value: quantity(3), limit_value: quantity(4), acceptability_relation: "greater_than" }, null, RATIO_NOTE_LOWER_BOUND],
      [{ acceptability_relation: "formula_predicate" }, null, RATIO_NOTE_PREDICATE],
      [{ computed_value: quantity(3) }, null, RATIO_NOTE_MISSING],
      [{ computed_value: quantity(3, "MPa"), limit_value: quantity(4e6, "Pa") }, null, ratioNoteUnitsDiffer("MPa", "Pa")],
      [{ computed_value: quantity(3), limit_value: quantity(0) }, null, RATIO_NOTE_LIMIT_NOT_POSITIVE],
      [{ computed_value: quantity(3), limit_value: quantity(-4) }, null, RATIO_NOTE_LIMIT_NOT_POSITIVE]
    ];
    for (const [extra, ratio, note] of cases) {
      const built = table(stressRows(), PASSING, run([bound("s:P-1:i", { status: "USER_RULE_CHECKED", ...extra })]));
      const item = group(built, "rule:cat_a").rows[0].outcome!;
      expect([item.ratio, item.ratioNote], JSON.stringify(extra)).toEqual([ratio, note]);
    }
  });

  it("lists outcomes with no ratio after the ranked ones and never hides them", () => {
    const checks = [
      bound("s:P-1:i", { status: "USER_RULE_CHECKED", computed_value: quantity(3), limit_value: quantity(4), acceptability_relation: "greater_than_or_equal" }),
      bound("s:P-2:i", { status: "USER_RULE_CHECKED", input: "in_p2", check: "chk_p2", computed_value: quantity(1), limit_value: quantity(4) })
    ];
    const shown = view(table(stressRows(), PASSING, run(checks)), "rule:cat_a", "ratio", "100");
    expect(shown.ranked.map((item) => [item.row.outcome!.checkId, item.hidden])).toEqual([["chk_p2", true]]);
    expect(shown.unranked.map((item) => item.outcome!.checkId)).toEqual(["chk_p1"]);
    expect(shown).toMatchObject({ hiddenCount: 1, shownCount: 1 });
  });

  it("puts a failed outcome before a checked one at an equal ratio, then keeps run order", () => {
    const checks = [
      bound("s:P-1:i", { status: "USER_RULE_CHECKED", computed_value: quantity(1), limit_value: quantity(1) }),
      bound("s:P-2:i", { status: "USER_RULE_CHECKED", input: "in_p2", check: "chk_p2", computed_value: quantity(2), limit_value: quantity(2) }),
      bound("s:P-2:i:L2", { status: "USER_RULE_FAILED", basis: L2, input: "in_p2", check: "chk_p2", computed_value: quantity(5), limit_value: quantity(5), acceptability_relation: "less_than" })
    ];
    const shown = view(table(stressRows(), PASSING, run(checks)), "rule:cat_a");
    expect(shown.ranked.map((item) => [item.row.outcome!.checkId, item.row.case!.basis.ref_id])).toEqual([["chk_p2", "load:L-2"], ["chk_p1", "load:L-1"], ["chk_p2", "load:L-1"]]);
  });

  it("gives one row per check and basis", () => {
    const checks = [
      bound("s:P-1:i", { status: "USER_RULE_CHECKED", computed_value: quantity(1), limit_value: quantity(10) }),
      bound(undefined, { status: "RULE_INPUTS_INCOMPLETE", basis: L2 })
    ];
    const rows = group(table(stressRows(), PASSING, run(checks)), "rule:cat_a").rows;
    expect(rows.map((item) => [item.outcome!.checkId, item.caseLabel, item.key])).toEqual([
      ["chk_p1", "Invented one (load:L-1)", JSON.stringify(["chk_p1", "load_case", "load:L-1"])],
      ["chk_p1", "Invented two (load:L-2)", JSON.stringify(["chk_p1", "load_case", "load:L-2"])]
    ]);
    expect(rows[1].outcome!.incompleteReason).toBe("Missing input: in_p1.");
  });

  it("locates an outcome by its bound row, else by the pack's selector, and lists several inputs", () => {
    const checks = [
      bound("s:P-1:i", { status: "USER_RULE_CHECKED", computed_value: quantity(1), limit_value: quantity(10) }),
      bound(undefined, { status: "RULE_INPUTS_INCOMPLETE", basis: L2, input: "in_p2", check: "chk_p2" }),
      outcome("chk_both", "cat_b", L1, "USER_RULE_CHECKED", {
        bound_inputs: [{ input_id: "in_p1", source_kind: "solver_result", supplied: false }, { input_id: "in_p2", source_kind: "solver_result", supplied: true, value: 80, unit: "MPa", result_id: "s:P-2:i" }],
        computed_value: quantity(1), limit_value: quantity(2)
      })
    ];
    const built = table(stressRows(), PASSING, run(checks));
    const [byRow, bySelector] = group(built, "rule:cat_a").rows;
    expect(byRow.locations).toEqual([{ entityRef: "pipe:P-1", station: "end_i", component: "sigma_plus", sectionBasis: null, resultId: "s:P-1:i", inputId: "in_p1" }]);
    expect(byRow.target).toEqual({ resultId: "s:P-1:i", entityRef: "pipe:P-1" });
    expect(bySelector.locations).toEqual([{ entityRef: "pipe:P-2", station: "end_i", component: "sigma_plus", sectionBasis: null, resultId: null, inputId: "in_p2" }]);
    expect(bySelector.target).toEqual({ resultId: null, entityRef: "pipe:P-2" });
    const both = group(built, "rule:cat_b").rows[0];
    expect(both.locations.map((item) => [item.entityRef, item.resultId])).toEqual([["pipe:P-1", null], ["pipe:P-2", "s:P-2:i"]]);
    expect(both.target).toEqual({ resultId: "s:P-2:i", entityRef: "pipe:P-2" });
  });

  it("names the section basis of a located row", () => {
    const rows = [row("s:red", "element_local_normal_stress_extreme", "pipe:P-1", 3, "MPa", { metadata: station("end_i", "sigma_plus", "section_extremes_from_published_stress_rows_v1; section_basis=reduced_section") })];
    const located = group(table(rows), "stress:element_local_normal_stress_extreme").rows[0].locations[0];
    expect(located.sectionBasis).toBe("reduced_section");
    expect(stressRatioLocationText(located)).toBe("pipe:P-1 · end_i · sigma_plus · reduced section");
  });

  it("withholds an outcome of a withheld case with no values, pinned after the incomplete ones", () => {
    const shown = view(table(stressRows(), SENSITIVE_L2, run(outcomes())), "rule:cat_a", "ratio", "100");
    expect(shown.pinned.map((item) => [item.outcome!.status, item.standing])).toEqual([
      ["RULE_INPUTS_INCOMPLETE", "withheld"], ["USER_RULE_CHECKED", "withheld"]
    ]);
    const held = shown.pinned[1];
    expect(held.withheldReason).toBe(supportLoadWithheldStateText("sensitive"));
    expect(held.outcome).toMatchObject({ computed: null, limit: null, ratio: null });
    expect(shown.ranked.map((item) => item.row.outcome!.checkId)).toEqual(["chk_p2", "chk_p1"]);
    for (const [standing, reason] of [[healthStanding({ eligible: false }), SUPPORT_LOAD_WITHHELD_NOT_ELIGIBLE], [null, SUPPORT_LOAD_WITHHELD_NO_HEALTH]] as const) {
      const all = view(table(stressRows(), standing, run(outcomes())), "rule:cat_a", "ratio", "100");
      expect(all.ranked).toEqual([]);
      expect(all.pinned.map((item) => item.outcome!.status)).toEqual(["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_CHECKED", "USER_RULE_FAILED"]);
      expect(all.pinned.every((item) => item.withheldReason === reason && item.outcome!.computed === null)).toBe(true);
    }
  });

  it("states an incomplete outcome with no basis and no reason", () => {
    const built = table(stressRows(), PASSING, run([outcome("chk_p1", "cat_a", null, "RULE_INPUTS_INCOMPLETE")]));
    const only = group(built, "rule:cat_a").rows[0];
    expect(only.caseLabel).toBe("No load case or combination");
    expect(only.outcome!.incompleteReason).toBe(INCOMPLETE_NO_REASON);
    expect(view(built, "rule:cat_a").pinned).toEqual([only]);
  });

  it("does not use a run that binds a row this result does not hold", () => {
    const built = table(stressRows(), PASSING, run([bound("s:other", { status: "USER_RULE_CHECKED", computed_value: quantity(1), limit_value: quantity(2) })]));
    expect(built.run).toEqual({ state: "not_this_result", rulePackId: "invented_pack", outcomeCount: 1, note: STRESS_RATIO_RUN_NOT_THIS_RESULT });
    expect(built.groups.map((item) => item.source)).toEqual(["emitted_stress"]);
  });

  it("falls back to the stress groups for a run with no outcomes", () => {
    const built = table(stressRows(), PASSING, run([]));
    expect(built.run.state).toBe("no_outcomes");
    expect(built.run.note).toBe(STRESS_RATIO_RUN_NO_OUTCOMES);
    expect(built.groups.map((item) => item.source)).toEqual(["emitted_stress"]);
  });

  it("is frozen and deterministic", () => {
    const first = table(stressRows(), PASSING, run(outcomes()));
    const second = table(stressRows(), PASSING, run(outcomes()));
    expect(second).toEqual(first);
    expect(Object.isFrozen(first)).toBe(true);
    expect(Object.isFrozen(first.groups[0].rows[0])).toBe(true);
    const shown = view(first, "rule:cat_a");
    expect(Object.isFrozen(shown) && Object.isFrozen(shown.ranked) && Object.isFrozen(shown.ranked[0])).toBe(true);
  });
});
