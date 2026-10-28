import { describe, expect, it } from "vitest";
import { healthStanding, overlayModel, overlayResult, reaction, row } from "../../test-support/resultOverlayFixtures";
import { loadBundledMechanicsReference } from "../../services/previewService";
import type { MechanicsResult, PreviewModel } from "../../types";
import { numericalHealthView, parseNumericalHealth, type ResultOverlayStanding } from "./numericalHealth";
import {
  SUPPORT_LOAD_COLUMNS,
  SUPPORT_LOAD_NO_CASES,
  SUPPORT_LOAD_NO_RESULT,
  SUPPORT_LOAD_NO_SUPPORTS,
  SUPPORT_LOAD_NOT_SOLVED,
  SUPPORT_LOAD_WITHHELD_NO_HEALTH,
  SUPPORT_LOAD_WITHHELD_NO_STATE,
  SUPPORT_LOAD_WITHHELD_NOT_ELIGIBLE,
  buildSupportLoadSummaries,
  supportLoadWithheldStateText,
  type SupportLoadSummary
} from "./supportLoadSummary";

// A2, the per-support load summary (DEL-07-05 Design `support-load-summary.md` §3, §4, §8). All
// data are invented (`resultOverlayFixtures`) or the bundled invented reference.

type Row = MechanicsResult["results"][number];
const L1 = { ref_type: "load_case", ref_id: "load:L-1" } as const;
const L2 = { ref_type: "load_case", ref_id: "load:L-2" } as const;
const C1 = { ref_type: "combination", ref_id: "combination:C-1" } as const;
const ENV = { ref_type: "combination", ref_id: "combination:ENV" } as const;
const PASSING = standingWith({ "load:L-1": "checks_passed", "load:L-2": "checks_passed" }, { "combination:C-1": "checks_passed" });

function model(): PreviewModel {
  return {
    ...overlayModel(),
    combinations: [
      { id: "combination:C-1", label: "Sum", basis: "mechanics", terms: [], provenance: "invented" },
      { id: "combination:ENV", label: "Range", basis: "range_envelope", terms: [], provenance: "invented" }
    ]
  } as PreviewModel;
}

/** Six components, both magnitudes and the node's six movements of one support and case. */
function caseRows(support: string, node: string, basis: Row["basis_ref"], values: { F: [number, number, number]; M: [number, number, number]; u?: [number, number, number]; r?: [number, number, number]; fMag?: number; mMag?: number }, extra: Partial<Row> = {}): Row[] {
  const key = `${basis!.ref_type}:${basis!.ref_id}`;
  const components = (["Fx", "Fy", "Fz", "Mx", "My", "Mz"] as const).map((component, index) =>
    reaction(support, component, index < 3 ? values.F[index] : values.M[index - 3], index < 3 ? "N" : "N*m", { id: `r:${support}:${component}:${key}`, basis_ref: basis, ...extra }));
  const frame = (component: string) => ({ component, coordinate_system: "global", location: "node", basis: "invented", sign_convention: "support-on-pipe" });
  const magnitudes = [
    ...(values.fMag === undefined ? [] : [row(`fm:${support}:${key}`, "support_reaction_force_magnitude_v2", support, values.fMag, "N", { basis_ref: basis, metadata: frame("force_magnitude"), ...extra })]),
    ...(values.mMag === undefined ? [] : [row(`mm:${support}:${key}`, "support_reaction_moment_magnitude_v2", support, values.mMag, "N*m", { basis_ref: basis, metadata: frame("moment_magnitude"), ...extra })])
  ];
  const movement = (prefix: "displacement" | "rotation", unit: string, triple?: [number, number, number]) => triple
    ? (["x", "y", "z"] as const).map((axis, index) => row(`${prefix}:${node}:${axis}:${key}`, `global_nodal_${prefix}_${axis}`, node, triple[index], unit, { basis_ref: basis, ...extra }))
    : [];
  return [...components, ...magnitudes, ...movement("displacement", "mm", values.u), ...movement("rotation", "rad", values.r)];
}

function summaryFor(rows: Row[], standing: ResultOverlayStanding | null = PASSING, evidence?: Record<string, unknown>, supportId = "support:S-A"): SupportLoadSummary {
  const summaries = buildSupportLoadSummaries({ model: model(), result: overlayResult(rows, evidence), standing });
  expect(summaries.state).toBe("available");
  return summaries.supports.find((item) => item.supportId === supportId)!;
}

function extreme(summary: SupportLoadSummary, column: string) {
  return summary.extremes.find((item) => item.column === column)!;
}

/** A standing with explicit case and combination states (the overlays' input). */
function standingWith(cases: Record<string, "checks_passed" | "sensitive">, combinations: Record<string, "checks_passed" | "sensitive" | null>, eligible = true): ResultOverlayStanding {
  const parsed = parseNumericalHealth({
    route: "generic",
    verdict_only: false,
    cases: Object.entries(cases).map(([caseId, state]) => ({ case_id: caseId, state, code: null, integrity_diagnostic_id: null, rcond: null, digits: null, conditioning_sensitive: false, causes: [] })),
    combinations: Object.entries(combinations).map(([id, state]) => ({ combination_id: id, state, cases: Object.keys(cases), withheld: state === null }))
  });
  return { eligible, health: numericalHealthView(parsed, { eligible, verdictHold: false }) };
}

describe("support load summary: rows and extremes", () => {
  const rows = [
    ...caseRows("support:S-A", "node:A", L1, { F: [10, -20, 30], M: [1, -2, 3], u: [0.5, -0.25, 0], r: [0.001, 0, -0.002], fMag: 37.4, mMag: 3.7 }),
    ...caseRows("support:S-A", "node:A", L2, { F: [-40, 5, 30], M: [-4, 2, 0], u: [-1, 0.75, 0], r: [0, 0.003, 0.001], fMag: 50.2, mMag: 4.5 }),
    ...caseRows("support:S-A", "node:A", C1, { F: [-30, -15, 60], M: [-3, 0, 3], fMag: 69, mMag: 4.2 })
  ];

  it("lists every case and combination in the viewport's order with the model's labels", () => {
    const summary = summaryFor(rows);
    expect(summary.rows.map((item) => item.case.label)).toEqual(["Invented one (load:L-1)", "Invented two (load:L-2)", "Sum (combination:C-1)"]);
    expect(summary.nodeId).toBe("node:A");
    expect(summary.signConvention).toMatch(/^support-on-pipe/);
    expect(summary.rows.every((item) => item.standing === "admitted" && item.reactionAbsence === null && item.inExtremes)).toBe(true);
  });

  it("keeps stored values, units and row ids; resultants are the emitted magnitude rows", () => {
    const first = summaryFor(rows).rows[0];
    expect(first.values.Fy).toEqual({ value: -20, unit: "N", resultId: "r:support:S-A:Fy:load_case:load:L-1" });
    expect(first.values.F).toEqual({ value: 37.4, unit: "N", resultId: "fm:support:S-A:load_case:load:L-1" });
    expect(first.values.M?.value).toBe(3.7);
    expect(first.values.ux).toEqual({ value: 0.5, unit: "mm", resultId: "displacement:node:A:x:load_case:load:L-1" });
    expect(first.values.rz?.unit).toBe("rad");
  });

  it("never forms a resultant from components", () => {
    const summary = summaryFor(caseRows("support:S-A", "node:A", L1, { F: [3, 4, 0], M: [0, 0, 0] }));
    expect(summary.rows[0].values.F).toBeUndefined();
    expect(summary.rows[0].values.M).toBeUndefined();
    expect(extreme(summary, "F").max).toBeNull();
  });

  it("takes signed maximum and minimum per column with the governing case", () => {
    const summary = summaryFor(rows);
    expect(extreme(summary, "Fx").max).toMatchObject({ value: 10, caseKey: "load_case:load:L-1" });
    expect(extreme(summary, "Fx").min).toMatchObject({ value: -40, caseKey: "load_case:load:L-2", caseLabel: "Invented two (load:L-2)" });
    expect(extreme(summary, "Fz").max).toMatchObject({ value: 60, caseKey: "combination:combination:C-1" });
    expect(extreme(summary, "Fy").max).toMatchObject({ value: 5, resultId: "r:support:S-A:Fy:load_case:load:L-2" });
    expect(extreme(summary, "My").min).toMatchObject({ value: -2 });
    // All-negative and the node's movements.
    expect(extreme(summary, "ux").max).toMatchObject({ value: 0.5, caseKey: "load_case:load:L-1" });
    expect(extreme(summary, "ux").min).toMatchObject({ value: -1, caseKey: "load_case:load:L-2" });
    expect(summary.extremes.map((item) => item.column)).toEqual(SUPPORT_LOAD_COLUMNS.map((item) => item.id));
    expect(summary.extremeRowCount).toBe(3);
  });

  it("names the first governing case in row order on a tie, and 0 ties -0", () => {
    const summary = summaryFor([
      ...caseRows("support:S-A", "node:A", L1, { F: [7, -0, 0], M: [0, 0, 0] }),
      ...caseRows("support:S-A", "node:A", L2, { F: [7, 0, -0], M: [0, 0, 0] })
    ]);
    expect(extreme(summary, "Fx").max?.caseKey).toBe("load_case:load:L-1");
    expect(extreme(summary, "Fx").min?.caseKey).toBe("load_case:load:L-1");
    expect(extreme(summary, "Fy").max?.caseKey).toBe("load_case:load:L-1");
    expect(extreme(summary, "Fz").min?.caseKey).toBe("load_case:load:L-1");
  });

  it("forms no extreme across different units and says so", () => {
    const summary = summaryFor([
      ...caseRows("support:S-A", "node:A", L1, { F: [1, 1, 1], M: [0, 0, 0] }),
      row("kN", "support_reaction_component_v2", "support:S-A", 2, "kN", { basis_ref: L2, metadata: { component: "Fx", coordinate_system: "global", location: "node", basis: "invented", sign_convention: "support-on-pipe" } })
    ]);
    expect(extreme(summary, "Fx")).toMatchObject({ max: null, min: null, note: "Units differ (N, kN); no extreme is formed" });
    expect(extreme(summary, "Fy").max).toMatchObject({ value: 1 });
  });
});

describe("support load summary: standing, withheld and absent rows", () => {
  const rows = [
    ...caseRows("support:S-A", "node:A", L1, { F: [1, 2, 3], M: [0, 0, 0], u: [0, 0, 0] }),
    ...caseRows("support:S-A", "node:A", L2, { F: [100, 200, 300], M: [9, 9, 9], u: [5, 5, 5] }),
    ...caseRows("support:S-A", "node:A", C1, { F: [101, 202, 303], M: [9, 9, 9] })
  ];

  it("withholds a case the standing does not admit: no values, a reason, out of the extremes", () => {
    const summary = summaryFor(rows, standingWith({ "load:L-1": "checks_passed", "load:L-2": "sensitive" }, { "combination:C-1": "sensitive" }));
    const [first, second, third] = summary.rows;
    expect(first.standing).toBe("admitted");
    expect(second).toMatchObject({ standing: "withheld", withheldReason: supportLoadWithheldStateText("sensitive"), values: {}, inExtremes: false });
    expect(third).toMatchObject({ standing: "withheld", values: {} });
    expect(extreme(summary, "Fx").max).toMatchObject({ value: 1, caseKey: "load_case:load:L-1" });
    expect(extreme(summary, "ux").max).toMatchObject({ value: 0 });
    expect(summary.exclusions).toEqual({ withheld: 2, envelope: 0, empty: 0 });
    expect(summary.extremeRowCount).toBe(1);
  });

  it("admits a combination only with its own checks-passed state", () => {
    const admitted = summaryFor(rows, standingWith({ "load:L-1": "checks_passed", "load:L-2": "checks_passed" }, { "combination:C-1": "checks_passed" }));
    expect(admitted.rows[2].standing).toBe("admitted");
    const noState = summaryFor(rows, standingWith({ "load:L-1": "checks_passed", "load:L-2": "checks_passed" }, { "combination:C-1": null }));
    expect(noState.rows[2]).toMatchObject({ standing: "withheld", withheldReason: SUPPORT_LOAD_WITHHELD_NO_STATE });
  });

  it("withholds every row when the result is not eligible or has no health view", () => {
    const ineligible = summaryFor(rows, healthStanding({ eligible: false }));
    expect(ineligible.rows.every((item) => item.standing === "withheld" && item.withheldReason === SUPPORT_LOAD_WITHHELD_NOT_ELIGIBLE)).toBe(true);
    expect(ineligible.extremes.every((item) => item.max === null && item.min === null)).toBe(true);
    const noHealth = summaryFor(rows, { eligible: true, health: null });
    expect(noHealth.rows.every((item) => item.withheldReason === SUPPORT_LOAD_WITHHELD_NO_HEALTH)).toBe(true);
    const noStanding = summaryFor(rows, null);
    expect(noStanding.rows.every((item) => item.standing === "withheld")).toBe(true);
  });

  it("admits the pressure-source route at eligible standing (no row badges there)", () => {
    const summary = summaryFor(rows, healthStanding({ eligible: true, route: "pressure_source", states: { "load:L-1": "sensitive", "load:L-2": "sensitive" } }));
    expect(summary.rows.map((item) => item.standing)).toEqual(["admitted", "admitted", "admitted"]);
  });

  it("says when the solver withheld a support's attribution, and keeps its node's movements", () => {
    const evidence = { preview_cases: [{ load_case_id: "load:L-1", support_attribution: { withheld: [{ support_id: "support:S-C", reason: "CONSTANT_EFFORT_NOT_CONSUMED" }] } }] };
    const summary = summaryFor([...rows, row("dC", "global_nodal_displacement_z", "node:C", -3, "mm", { basis_ref: L1 })], PASSING, evidence, "support:S-C");
    expect(summary.rows[0]).toMatchObject({ reactionAbsence: "attribution_withheld", reactionAbsenceDetail: "CONSTANT_EFFORT_NOT_CONSUMED", inExtremes: true });
    expect(summary.rows[0].values).toEqual({ uz: { value: -3, unit: "mm", resultId: "dC" } });
    expect(summary.rows[1]).toMatchObject({ reactionAbsence: "no_rows", values: {}, inExtremes: false });
    expect(summary.exclusions.empty).toBe(2);
  });

  it("lists a support without rows with empty rows and no extremes", () => {
    const summary = summaryFor(rows, PASSING, undefined, "support:S-D");
    expect(summary.rows.every((item) => item.reactionAbsence === "no_rows" && Object.keys(item.values).length === 0)).toBe(true);
    expect(summary.extremes.every((item) => item.max === null && item.min === null && item.note === null)).toBe(true);
    expect(summary.signConvention).toBeNull();
  });

  it("does not tabulate reaction rows in another frame", () => {
    const local = caseRows("support:S-A", "node:A", L1, { F: [1, 2, 3], M: [0, 0, 0], u: [4, 4, 4] }).map((item) =>
      item.kind === "support_reaction_component_v2" ? { ...item, metadata: { ...item.metadata!, coordinate_system: "support_local" } } : item);
    const summary = summaryFor(local);
    expect(summary.rows[0]).toMatchObject({ reactionAbsence: "frame_not_global", reactionAbsenceDetail: "support_local" });
    expect(summary.rows[0].values.Fx).toBeUndefined();
    expect(summary.rows[0].values.ux?.value).toBe(4);
  });

  it("lists a range envelope as such and keeps it out of the extremes", () => {
    const envelope = caseRows("support:S-A", "node:A", ENV, { F: [999, 999, 999], M: [0, 0, 0], fMag: 999 }).map((item) => ({
      ...item, metadata: { ...item.metadata!, basis: "explicit_user_range_envelope", sign_convention: "max mode-selected" }
    }));
    const summary = summaryFor([...rows, ...envelope]);
    const last = summary.rows[summary.rows.length - 1];
    expect(last).toMatchObject({ case: { label: "Range (combination:ENV)", envelope: true }, standing: "withheld" });
    const admitted = summaryFor([...rows, ...envelope], standingWith({ "load:L-1": "checks_passed", "load:L-2": "checks_passed" }, { "combination:C-1": "checks_passed", "combination:ENV": "checks_passed" }));
    const env = admitted.rows.find((item) => item.case.envelope)!;
    expect(env).toMatchObject({ standing: "admitted", inExtremes: false });
    expect(env.values.Fx?.value).toBe(999);
    expect(env.values.F?.value).toBe(999);
    expect(extreme(admitted, "Fx").max?.value).toBe(101);
    expect(admitted.exclusions.envelope).toBe(1);
  });
});

describe("support load summary: unavailable states", () => {
  it("states why nothing is tabulated", () => {
    expect(buildSupportLoadSummaries({ model: model(), result: null, standing: PASSING })).toMatchObject({ state: "unavailable", reason: SUPPORT_LOAD_NO_RESULT, supports: [] });
    const unsolved = { ...overlayResult([]), status: { mechanics: "MECHANICS_BLOCKED", rule_check: "not_run", professional_acceptance: "not_claimed" } };
    expect(buildSupportLoadSummaries({ model: model(), result: unsolved, standing: PASSING }).reason).toBe(SUPPORT_LOAD_NOT_SOLVED);
    expect(buildSupportLoadSummaries({ model: { ...model(), supports: [] }, result: overlayResult([]), standing: PASSING }).reason).toBe(SUPPORT_LOAD_NO_SUPPORTS);
    expect(buildSupportLoadSummaries({ model: model(), result: overlayResult([]), standing: PASSING }).reason).toBe(SUPPORT_LOAD_NO_CASES);
  });
});

describe("support load summary: the bundled invented reference", () => {
  it("summarises every support over both load cases with the emitted rows", async () => {
    const { model: reference, source } = await loadBundledMechanicsReference();
    const standing = healthStanding({ eligible: true, states: { "load:L-100": "checks_passed", "load:L-200": "checks_passed" } });
    const summaries = buildSupportLoadSummaries({ model: reference, result: source, standing });
    expect(summaries.state).toBe("available");
    expect(summaries.supports.map((item) => item.supportId)).toEqual(reference.supports.map((item) => item.id));
    const anchor = summaries.supports.find((item) => item.supportId === "support:S-100")!;
    const l100 = anchor.rows.find((item) => item.case.basis.ref_id === "load:L-100")!;
    const fz = source.results.find((item) => item.id === "result:support-action:10:load:L-100:13:support:S-100:Fz")!;
    expect(l100.values.Fz).toEqual({ value: fz.value, unit: fz.unit, resultId: fz.id });
    expect(l100.values.F?.resultId).toBe("result:support-action:10:load:L-100:13:support:S-100:force_magnitude");
    const constantEffort = summaries.supports.find((item) => item.supportId === "support:CE-120")!;
    expect(constantEffort.rows.find((item) => item.case.basis.ref_id === "load:L-100")).toMatchObject({ reactionAbsence: "attribution_withheld", reactionAbsenceDetail: "CONSTANT_EFFORT_NOT_CONSUMED" });
  });
});
