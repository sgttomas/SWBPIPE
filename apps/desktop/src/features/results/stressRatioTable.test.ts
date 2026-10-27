import { createHash } from "node:crypto";
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
  INCOMPLETE_MESSAGES_NOTE,
  INCOMPLETE_NO_REASON,
  RATIO_NOTE_LIMIT_NOT_POSITIVE,
  RATIO_NOTE_COMPUTED_NOT_POSITIVE,
  RATIO_NOTE_MISSING,
  RATIO_NOTE_NOT_FINITE,
  RATIO_NOTE_PREDICATE,
  STRESS_KINDS,
  STRESS_RATIO_NO_RESULT,
  STRESS_RATIO_NO_RUN,
  STRESS_RATIO_NOT_SOLVED,
  STRESS_RATIO_NOTHING,
  STRESS_RATIO_RUN_NO_OUTCOMES,
  STRESS_RATIO_RUN_NOT_THIS_RESULT,
  SUBJECT_NOT_SHOWN,
  SUBJECT_UNDECLARED_STATUS,
  THRESHOLD_INVALID_NOTE,
  buildStressRatioTable,
  ratioNoteUnitsDiffer,
  incompleteReason,
  outcomeRatio,
  ratioDirection,
  stressRatioLocationText,
  stressRatioPage,
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
const SENSITIVE_L1 = healthStanding({ eligible: true, states: { "load:L-1": "sensitive", "load:L-2": "checks_passed" } });
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
    expect(shown.pinned[0].outcome!.incompleteReason).toBe(`Missing input: in_p1. Completeness finding RULE_SOLVER_RESULT_ROW_MISSING (in_p1). ${INCOMPLETE_MESSAGES_NOTE}`);
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

  it("forms a ratio only under a bound relation, in one unit, with a positive limit", () => {
    // A lower bound forms limit ÷ computed (HELP_HUMAN, 2026-10-27; Design §3.1): 4 / 3.
    const cases: Array<[Partial<RuleCheckOutcome>, number | null, string | null]> = [
      [{ computed_value: quantity(3), limit_value: quantity(4), acceptability_relation: "less_than" }, 0.75, null],
      [{ computed_value: quantity(3), limit_value: quantity(4), acceptability_relation: "greater_than_or_equal" }, 4 / 3, null],
      [{ computed_value: quantity(3), limit_value: quantity(4), acceptability_relation: "greater_than" }, 4 / 3, null],
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
    // A lower bound over a computed value that is not positive forms no ratio (Design §3.1).
    const checks = [
      bound("s:P-1:i", { status: "USER_RULE_CHECKED", computed_value: quantity(-3), limit_value: quantity(4), acceptability_relation: "greater_than_or_equal" }),
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

  it("builds an incomplete reason from structured fields only, never from a finding's message", () => {
    const numeric = bound(undefined, {
      status: "RULE_INPUTS_INCOMPLETE",
      completeness_findings: [{ code: "RULE_SOLVER_RESULT_ROW_MISSING", severity: "blocking", subject_id: "in_p1", message: "no row matched at 0.25" }],
      evaluator_findings: [{ code: "TableOutOfRange", severity: "blocking", subject_id: "chk_p1", message: "table argument 123456789.5 is outside the table range [0, 100]; the lookup is not clamped" }],
      diagnostic_codes: ["RULE_DIAG_7"]
    });
    const reason = incompleteReason(numeric, "admitted");
    expect(reason).toBe(`Missing input: in_p1. Completeness finding RULE_SOLVER_RESULT_ROW_MISSING (in_p1). Evaluator finding TableOutOfRange (chk_p1). ${INCOMPLETE_MESSAGES_NOTE}`);
    // A withheld row's reason keeps the codes and missing inputs but no subject (§3.2).
    const withheldReason = incompleteReason(numeric, "withheld");
    expect(withheldReason).toBe(`Missing input: in_p1. Completeness finding RULE_SOLVER_RESULT_ROW_MISSING. Evaluator finding TableOutOfRange. ${INCOMPLETE_MESSAGES_NOTE}`);
    for (const fragment of ["123456789", "0.25", "100", "table argument", "no row matched"]) {
      expect(reason).not.toContain(fragment);
      expect(withheldReason).not.toContain(fragment);
    }
    const pinnedReason = (standing: ResultOverlayStanding) => view(table(stressRows(), standing, run([numeric])), "rule:cat_a").pinned[0].outcome!.incompleteReason;
    expect(pinnedReason(PASSING)).toBe(reason);
    expect(pinnedReason(SENSITIVE_L1)).toBe(withheldReason);
    for (const standing of ["admitted", "withheld"] as const) {
      expect(incompleteReason(outcome("chk_p1", "cat_a", L1, "RULE_INPUTS_INCOMPLETE", { diagnostic_codes: ["RULE_DIAG_7", "RULE_DIAG_8"] }), standing)).toBe("RULE_DIAG_7, RULE_DIAG_8");
    }
  });

  describe("a downgraded check's undeclared status in a finding subject (§3.2)", () => {
    const STATUS_TOKENS = ["USER_RULE_FAILED", "USER_RULE_CHECKED"] as const;
    // The runner's `enforce_declared`: the evaluated status as the subject, the outcome incomplete.
    const downgraded = (evaluated: string) => bound(undefined, {
      status: "RULE_INPUTS_INCOMPLETE",
      computed_value: quantity(2), limit_value: quantity(1),
      evaluator_findings: [{ code: "STATUS_NOT_DECLARED", severity: "blocking", subject_id: evaluated, message: "computed status is not in the check's declared result_statuses" }]
    });

    for (const evaluated of STATUS_TOKENS) {
      for (const mode of ["ratio", "stress"] as const) {
        it(`${evaluated}, ${mode} mode: withheld shows no subject; admitted shows "${SUBJECT_UNDECLARED_STATUS}"`, () => {
          const withheld = view(table(stressRows(), SENSITIVE_L1, run([downgraded(evaluated)])), "rule:cat_a", mode).pinned[0];
          expect(withheld.standing).toBe("withheld");
          expect(withheld.outcome!.status).toBe("RULE_INPUTS_INCOMPLETE");
          expect(withheld.outcome!.incompleteReason).toBe(`Missing input: in_p1. Evaluator finding STATUS_NOT_DECLARED. ${INCOMPLETE_MESSAGES_NOTE}`);
          expect(withheld.outcome!.computed).toBeNull();
          expect(JSON.stringify(withheld)).not.toMatch(/USER_RULE_(FAILED|CHECKED)/);

          const admitted = view(table(stressRows(), PASSING, run([downgraded(evaluated)])), "rule:cat_a", mode).pinned[0];
          expect(admitted.standing).toBe("admitted");
          expect(admitted.outcome!.status).toBe("RULE_INPUTS_INCOMPLETE");
          expect(admitted.outcome!.incompleteReason).toBe(`Missing input: in_p1. Evaluator finding STATUS_NOT_DECLARED (${SUBJECT_UNDECLARED_STATUS}). ${INCOMPLETE_MESSAGES_NOTE}`);
          expect(admitted.outcome!.incompleteReason).not.toMatch(/USER_RULE_(FAILED|CHECKED)/);
        });
      }
    }

    it("never echoes a subject that contains a status token, in any case and under any code", () => {
      const subjects = ["USER_RULE_FAILED", "user_rule_checked", "chk_USER_RULE_FAILED_1", "RULE_INPUTS_INCOMPLETE"];
      for (const subject of subjects) {
        const item = bound(undefined, {
          status: "RULE_INPUTS_INCOMPLETE",
          completeness_findings: [{ code: "RULE_INPUT_MISSING", severity: "blocking", subject_id: subject, message: "" }],
          evaluator_findings: [{ code: "STATUS_NOT_DECLARED", severity: "blocking", subject_id: subject, message: "" }]
        });
        const admitted = incompleteReason(item, "admitted");
        expect(admitted).toBe(`Missing input: in_p1. Completeness finding RULE_INPUT_MISSING (${SUBJECT_NOT_SHOWN}). Evaluator finding STATUS_NOT_DECLARED (${SUBJECT_UNDECLARED_STATUS}). ${INCOMPLETE_MESSAGES_NOTE}`);
        expect(admitted.toUpperCase()).not.toContain(subject.toUpperCase());
        expect(incompleteReason(item, "withheld")).toBe(`Missing input: in_p1. Completeness finding RULE_INPUT_MISSING. Evaluator finding STATUS_NOT_DECLARED. ${INCOMPLETE_MESSAGES_NOTE}`);
      }
      // Other subjects stay for an admitted row.
      const plain = bound(undefined, { status: "RULE_INPUTS_INCOMPLETE", evaluator_findings: [{ code: "STATUS_NOT_DECLARED", severity: "blocking", subject_id: "chk_p1", message: "" }] });
      expect(incompleteReason(plain, "admitted")).toContain("Evaluator finding STATUS_NOT_DECLARED (chk_p1).");
    });
  });

  it("lists a failed outcome on every page at its rank, in either mode, past the page length", () => {
    // Fifty checked outcomes with computed -100 .. -149 against a limit of 1, and one failed 2 / 1.
    const checked = (count: number) => Array.from({ length: count }, (_, index) =>
      outcome(`chk_${index}`, "cat_a", L1, "USER_RULE_CHECKED", { computed_value: quantity(-100 - index), limit_value: quantity(1) }));
    const failed = outcome("chk_failed", "cat_a", L1, "USER_RULE_FAILED", { computed_value: quantity(2), limit_value: quantity(1) });
    const failedKey = JSON.stringify(["chk_failed", "load_case", "load:L-1"]);
    for (const count of [50, 57]) {
      const built = table(stressRows(), PASSING, run([...checked(count), failed]));
      const byRatio = stressRatioPage(view(built, "rule:cat_a", "ratio"), 50);
      expect(byRatio.ranked[0]).toMatchObject({ rank: 1 });
      expect(byRatio.ranked[0].row.key).toBe(failedKey);
      const byStress = stressRatioPage(view(built, "rule:cat_a", "stress"), 50);
      expect(byStress.ranked.map((item) => item.row.key)).toContain(failedKey);
      expect(byStress.ranked.at(-1)).toMatchObject({ rank: count + 1 });
      expect(byStress.ranked.at(-1)!.row.key).toBe(failedKey);
      expect(byStress.ranked).toHaveLength(51);
      expect(byStress.notListedCount).toBe(count - 50);
      expect(byStress.listedCount).toBe(51);
      // A threshold above every value and a page of one keep it too.
      const narrow = stressRatioPage(view(built, "rule:cat_a", "stress", "1000"), 1);
      expect(narrow.ranked.map((item) => [item.row.key, item.rank])).toEqual([[failedKey, 1]]);
      expect(narrow.notListedCount).toBe(0);
      const paged = stressRatioPage(view(built, "rule:cat_a", "ratio"), 1);
      expect(paged.ranked.map((item) => item.rank)).toEqual([1, 2]);
      expect(paged.notListedCount).toBe(count - 1);
    }
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

// A lower-bound relation forms limit ÷ computed (HELP_HUMAN, 2026-10-27: "Lower-bound ratio ranking
// decided: for a greater-than relation, ratio = limit ÷ computed, same unit and dimension, positive
// computed value, finite quotient; ratio > 1 means failure in both directions; label each ratio's
// direction; rank by magnitude otherwise."; Design §3.1, §5).
describe("stress ratio table: lower-bound ratio", () => {
  const LOWER = ["greater_than", "greater_than_or_equal"] as const;
  const lower = (relation: string, computed: number | null, limit: number | null, computedUnit = "MPa", limitUnit = "MPa") => outcomeRatio({
    acceptability_relation: relation,
    computed_value: computed === null ? undefined : quantity(computed, computedUnit),
    limit_value: limit === null ? undefined : quantity(limit, limitUnit)
  });
  const lb = (check: string, basis: typeof L1 | typeof L2, status: RuleCheckStatus, relation: string, computed: number, limit: number) =>
    outcome(check, "cat_a", basis, status, { acceptability_relation: relation, computed_value: quantity(computed), limit_value: quantity(limit) });
  const id = (check: string, basis: typeof L1 | typeof L2) => JSON.stringify([check, "load_case", basis.ref_id]);

  it("names each relation's direction", () => {
    expect(["less_than", "less_than_or_equal", ...LOWER, "formula_predicate", "none", "equal", ""].map(ratioDirection))
      .toEqual(["upper", "upper", "lower", "lower", null, null, null, null]);
  });

  it("forms limit ÷ computed under either lower-bound relation", () => {
    for (const relation of LOWER) {
      expect(lower(relation, 8, 4), relation).toEqual({ ratio: 0.5, note: null });
      expect(lower(relation, 2, 4), relation).toEqual({ ratio: 2, note: null });
      expect(lower(relation, 3, 4), relation).toEqual({ ratio: 4 / 3, note: null });
      expect(lower(relation, 4, 4), relation).toEqual({ ratio: 1, note: null });
      // Dimensionless, in one unit.
      expect(outcomeRatio({ acceptability_relation: relation, computed_value: quantity(5, "1", "dimensionless"), limit_value: quantity(2, "1", "dimensionless") })).toEqual({ ratio: 0.4, note: null });
    }
  });

  it("forms no ratio over a computed value that is zero, negative or not finite", () => {
    for (const relation of LOWER) {
      for (const computed of [0, -0, -3, -Number.MIN_VALUE]) {
        expect(lower(relation, computed, 4), `${relation} ${computed}`).toEqual({ ratio: null, note: RATIO_NOTE_COMPUTED_NOT_POSITIVE });
      }
      // A value that is not finite is no value, as in the computed and limit cells (Design §3.1).
      for (const computed of [Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY]) {
        expect(lower(relation, computed, 4), `${relation} ${computed}`).toEqual({ ratio: null, note: RATIO_NOTE_MISSING });
      }
      expect(lower(relation, null, 4)).toEqual({ ratio: null, note: RATIO_NOTE_MISSING });
    }
  });

  it("forms no ratio against a limit that is zero, negative or not finite", () => {
    for (const relation of LOWER) {
      for (const limit of [0, -0, -4]) expect(lower(relation, 3, limit), `${relation} ${limit}`).toEqual({ ratio: null, note: RATIO_NOTE_LIMIT_NOT_POSITIVE });
      for (const limit of [Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY]) {
        expect(lower(relation, 3, limit), `${relation} ${limit}`).toEqual({ ratio: null, note: RATIO_NOTE_MISSING });
      }
      expect(lower(relation, 3, null)).toEqual({ ratio: null, note: RATIO_NOTE_MISSING });
      // The divisor is checked first: both at zero read as the computed value.
      expect(lower(relation, 0, 0)).toEqual({ ratio: null, note: RATIO_NOTE_COMPUTED_NOT_POSITIVE });
      expect(lower(relation, -3, -4)).toEqual({ ratio: null, note: RATIO_NOTE_COMPUTED_NOT_POSITIVE });
    }
  });

  it("forms no ratio across units or dimensions, and none when the quotient overflows", () => {
    for (const relation of LOWER) {
      expect(lower(relation, 3, 4e6, "MPa", "Pa")).toEqual({ ratio: null, note: ratioNoteUnitsDiffer("MPa", "Pa") });
      expect(outcomeRatio({ acceptability_relation: relation, computed_value: quantity(3, "1", "dimensionless"), limit_value: quantity(4, "1", "length") }))
        .toEqual({ ratio: null, note: ratioNoteUnitsDiffer("1", "1") });
      // A tiny computed value against a large limit.
      expect(lower(relation, 1e-300, 1e300)).toEqual({ ratio: null, note: RATIO_NOTE_NOT_FINITE });
      expect(lower(relation, Number.MIN_VALUE, 1)).toEqual({ ratio: null, note: RATIO_NOTE_NOT_FINITE });
      expect(lower(relation, Number.MIN_VALUE, Number.MAX_VALUE)).toEqual({ ratio: null, note: RATIO_NOTE_NOT_FINITE });
      // An underflowing quotient is finite: the computed value is far above the limit.
      expect(lower(relation, 1e300, 1e-300)).toEqual({ ratio: 0, note: null });
    }
  });

  it("keeps the rule pack's status beside the ratio, at the boundary too", () => {
    // At computed = limit, greater_than does not hold and greater_than_or_equal does; both ratios are 1.
    const checks = [
      lb("lb_pass", L1, "USER_RULE_CHECKED", "greater_than", 8, 4),
      lb("lb_gte_edge", L1, "USER_RULE_CHECKED", "greater_than_or_equal", 4, 4),
      lb("lb_gt_edge", L1, "USER_RULE_FAILED", "greater_than", 4, 4),
      lb("lb_fail", L2, "USER_RULE_FAILED", "greater_than_or_equal", 2, 4)
    ];
    const built = table(stressRows(), PASSING, run(checks));
    const rows = group(built, "rule:cat_a").rows.map((item) => [item.outcome!.checkId, item.outcome!.status, item.outcome!.ratio, item.outcome!.ratioNote]);
    expect(rows).toEqual([
      ["lb_pass", "USER_RULE_CHECKED", 0.5, null],
      ["lb_gte_edge", "USER_RULE_CHECKED", 1, null],
      ["lb_gt_edge", "USER_RULE_FAILED", 1, null],
      ["lb_fail", "USER_RULE_FAILED", 2, null]
    ]);
    // Largest first; at a tie the failed outcome comes first, then run order.
    expect(view(built, "rule:cat_a").ranked.map((item) => [item.row.outcome!.checkId, item.key])).toEqual([
      ["lb_fail", 2], ["lb_gt_edge", 1], ["lb_gte_edge", 1], ["lb_pass", 0.5]
    ]);
  });

  // Upper: 1.2 failed, 0.4 checked. Lower: 2 failed, 0.5 checked, a non-positive computed value
  // (no ratio), and an incomplete outcome.
  const mixed = () => run([
    outcome("up_fail", "cat_a", L1, "USER_RULE_FAILED", { acceptability_relation: "less_than_or_equal", computed_value: quantity(120), limit_value: quantity(100) }),
    lb("lb_pass", L1, "USER_RULE_CHECKED", "greater_than", 200, 100),
    outcome("up_pass", "cat_a", L1, "USER_RULE_CHECKED", { acceptability_relation: "less_than", computed_value: quantity(40), limit_value: quantity(100) }),
    lb("lb_none", L1, "USER_RULE_CHECKED", "greater_than_or_equal", -300, 100),
    lb("lb_fail", L2, "USER_RULE_FAILED", "greater_than_or_equal", 50, 100),
    outcome("lb_incomplete", "cat_a", L2, "RULE_INPUTS_INCOMPLETE", { acceptability_relation: "greater_than", computed_value: quantity(1), limit_value: quantity(100) })
  ]);

  it("sorts upper- and lower-bound ratios together, then lists rows with no ratio, and pins incomplete ones", () => {
    const built = table(stressRows(), PASSING, mixed());
    const shown = view(built, "rule:cat_a");
    expect(shown.sorted).toBe(true);
    expect(shown.pinned.map((item) => item.key)).toEqual([id("lb_incomplete", L2)]);
    expect(shown.pinned[0].outcome!.ratio).toBeNull();
    expect(shown.ranked.map((item) => [item.row.outcome!.checkId, item.key])).toEqual([["lb_fail", 2], ["up_fail", 1.2], ["lb_pass", 0.5], ["up_pass", 0.4]]);
    expect(shown.ranked.map((item) => ratioDirection(item.row.outcome!.relation))).toEqual(["lower", "upper", "lower", "upper"]);
    expect(shown.unranked.map((item) => [item.outcome!.checkId, item.outcome!.ratioNote])).toEqual([["lb_none", RATIO_NOTE_COMPUTED_NOT_POSITIVE]]);
    // Stress mode ranks every admitted outcome by the magnitude of its computed value, either direction.
    const byStress = view(built, "rule:cat_a", "stress");
    expect(byStress.ranked.map((item) => [item.row.outcome!.checkId, item.key])).toEqual([
      ["lb_none", 300], ["lb_pass", 200], ["up_fail", 120], ["lb_fail", 50], ["up_pass", 40]
    ]);
    expect(byStress.unranked).toEqual([]);
    expect(byStress.pinned.map((item) => item.key)).toEqual([id("lb_incomplete", L2)]);
  });

  it("applies the threshold to the ratio whatever its direction, never to a failed, incomplete or unranked row", () => {
    const built = table(stressRows(), PASSING, mixed());
    const at = (threshold: string) => view(built, "rule:cat_a", "ratio", threshold);
    expect(at("0.45").ranked.filter((item) => item.hidden).map((item) => item.row.outcome!.checkId)).toEqual(["up_pass"]);
    expect(at("0.5").ranked.filter((item) => item.hidden).map((item) => item.row.outcome!.checkId)).toEqual(["up_pass"]);
    expect(at("0.6").ranked.filter((item) => item.hidden).map((item) => item.row.outcome!.checkId)).toEqual(["lb_pass", "up_pass"]);
    for (const threshold of ["0.6", "1", "1e9"]) {
      const shown = at(threshold);
      expect(rankedKeys(shown), threshold).toEqual([id("lb_fail", L2), id("up_fail", L1)]);
      expect(shown.pinned.map((item) => item.key)).toEqual([id("lb_incomplete", L2)]);
      expect(shown.unranked.map((item) => item.outcome!.checkId)).toEqual(["lb_none"]);
      expect(shown).toMatchObject({ hiddenCount: 2, shownCount: 4, totalCount: 6 });
    }
  });
});

// The upper-bound path is unchanged by the lower-bound ratio (HELP_HUMAN, 2026-10-27; Design §3.1).
// Every relation other than a lower bound, over a grid of computed values and limits, gives the same
// ratio and note as before, and a run of such outcomes builds the same table, views and pages. The
// digest is of the outputs at the code before the lower-bound ratio (commit 1618bf866b).
describe("stress ratio table: upper-bound outputs pinned", () => {
  const RELATIONS = ["less_than", "less_than_or_equal", "formula_predicate", "none", "equal", ""];
  const VALUES = [3, 4, 0, -0, -3, Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY, 1e-308, 1e308, 5e-324];
  const json = (value: unknown) => JSON.stringify(value, (_, item) => (typeof item === "number" && !Number.isFinite(item) ? String(item) : Object.is(item, -0) ? "-0" : item));
  function grid(): unknown[] {
    const out: unknown[] = [];
    for (const relation of RELATIONS) {
      for (const computed of VALUES) {
        for (const limit of VALUES) out.push([relation, computed, limit, outcomeRatio({ acceptability_relation: relation, computed_value: quantity(computed), limit_value: quantity(limit) })]);
      }
      out.push([relation, "units", outcomeRatio({ acceptability_relation: relation, computed_value: quantity(3, "MPa"), limit_value: quantity(4e6, "Pa") })]);
      out.push([relation, "dimension", outcomeRatio({ acceptability_relation: relation, computed_value: quantity(3, "1", "dimensionless"), limit_value: quantity(4, "1", "length") })]);
      out.push([relation, "no computed", outcomeRatio({ acceptability_relation: relation, limit_value: quantity(4) })]);
      out.push([relation, "no limit", outcomeRatio({ acceptability_relation: relation, computed_value: quantity(3) })]);
    }
    return out;
  }
  function views(): unknown[] {
    const checks = [
      bound("s:P-1:i", { status: "USER_RULE_CHECKED", computed_value: quantity(40), limit_value: quantity(100), acceptability_relation: "less_than" }),
      bound("s:P-2:i", { status: "USER_RULE_FAILED", input: "in_p2", check: "chk_p2", computed_value: quantity(120), limit_value: quantity(100) }),
      bound("s:P-2:i:L2", { status: "USER_RULE_CHECKED", basis: L2, input: "in_p2", check: "chk_p2", computed_value: quantity(-70), limit_value: quantity(0) }),
      bound("s:P-1:i", { status: "USER_RULE_FAILED", basis: L2, computed_value: quantity(3, "MPa"), limit_value: quantity(4e6, "Pa") }),
      bound("s:P-1:i", { status: "USER_RULE_CHECKED", check: "chk_both", category: "cat_b", computed_value: quantity(1e-300), limit_value: quantity(1e300) }),
      bound("s:P-2:i", { status: "USER_RULE_FAILED", check: "chk_both", category: "cat_b", input: "in_p2", computed_value: quantity(1e300), limit_value: quantity(1e-300) }),
      bound(undefined, { status: "RULE_INPUTS_INCOMPLETE", basis: L2 }),
      bound("s:P-1:i", { status: "USER_RULE_CHECKED", basis: L2, check: "chk_both", category: "cat_b", acceptability_relation: "formula_predicate" })
    ];
    const out: unknown[] = [];
    for (const standing of [PASSING, SENSITIVE_L2]) {
      const built = table(stressRows(), standing, run(checks));
      out.push(built);
      for (const item of built.groups) {
        for (const mode of ["ratio", "stress"] as const) {
          for (const threshold of ["", "0.5", "1", "50", "junk"]) {
            const shown = stressRatioView(item, { mode, threshold });
            out.push(shown, stressRatioPage(shown, 1), stressRatioPage(shown, 50));
          }
        }
      }
    }
    return out;
  }
  const sha = (text: string) => createHash("sha256").update(text).digest("hex");

  it("gives the same ratio and note for every relation other than a lower bound", () => {
    expect(sha(json(grid()))).toBe("0b3f0b9dbfe91cbb0efa8d3d3cea23b5b0d22fa3083bb09d0cee8bd89282a8e6");
  });

  it("builds the same table, views and pages for a run of upper-bound outcomes", () => {
    expect(sha(json(views()))).toBe("331b222e3dc5aefcedffb45162fb41cdb6799a78dd248d3cdba8b0683b1625c0");
  });
});
