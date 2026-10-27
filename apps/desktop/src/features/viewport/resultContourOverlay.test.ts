import { describe, expect, it } from "vitest";
import { healthStanding, overlayModel, overlayResult, row } from "../../test-support/resultOverlayFixtures";
import { loadBundledMechanicsReference } from "../../services/previewService";
import { buildStressRatioTable } from "../results/stressRatioTable";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";
import type { RuleCheckOutcome } from "../../services/ruleCheckService";
import type { MechanicsResult } from "../../types";
import { entityKey } from "../workspace/selectionState";
import {
  availableResultColourQuantities,
  buildResultColourOverlay,
  RESULT_COLOUR_HELD_REASON,
  RESULT_COLOUR_QUANTITIES,
  RESULT_STATION_FRACTIONS,
  resultColourTable,
  resultOverlayCases,
  stationFraction,
  type ResultColourResolution
} from "./resultColourOverlay";
import { contourValueLabels, interpolateStations } from "./resultContour";
import { resultNoValueHex, resultScaleHex } from "./viewportPalette";

// Result colour along members (A4; DEL-07-05 Design `result-contour.md` §2, §3, §6): values from
// the stations the rows state, the standing rule per station, rule ratios from the session's run,
// and the threshold and numbers display of the member table. Invented data only.

type Row = MechanicsResult["results"][number];
const PASSING = healthStanding({ eligible: true });
const L1 = { ref_type: "load_case", ref_id: "load:L-1" } as const;
const at = (location: string, component = "sigma_plus") =>
  ({ component, coordinate_system: "pipe_section", location, basis: "invented", sign_convention: "invented" });
const quantity = (id: string) => RESULT_COLOUR_QUANTITIES.find((item) => item.id === id)!;

function stationRows(): Row[] {
  return [
    row("s:P-1:i", "element_local_normal_stress_extreme", "pipe:P-1", -4, "MPa", { metadata: at("end_i") }),
    row("s:P-1:i-", "element_local_normal_stress_extreme", "pipe:P-1", 1, "MPa", { metadata: at("end_i", "sigma_minus") }),
    row("s:P-1:m", "element_local_normal_stress_extreme", "pipe:P-1", 2, "MPa", { metadata: at("midspan") }),
    row("s:P-1:j", "element_local_normal_stress_extreme", "pipe:P-1", 6, "MPa", { metadata: at("end_j") }),
    row("s:P-1:g", "element_local_normal_stress_extreme", "pipe:P-1", 9, "MPa", { metadata: at("governing_station") }),
    // P-2: one stated station only, and a row with no station.
    row("s:P-2:m", "element_local_normal_stress_extreme", "pipe:P-2", 3, "MPa", { metadata: at("midspan") }),
    row("s:P-2:x", "element_local_normal_stress_extreme", "pipe:P-2", 5, "MPa"),
    // P-3: a non-finite row at its midspan.
    row("s:P-3:i", "element_local_normal_stress_extreme", "pipe:P-3", 1, "MPa", { metadata: at("end_i") }),
    row("s:P-3:m", "element_local_normal_stress_extreme", "pipe:P-3", Number.NaN, "MPa", { metadata: at("midspan") }),
    row("s:P-3:j", "element_local_normal_stress_extreme", "pipe:P-3", 2, "MPa", { metadata: at("end_j") }),
    row("d:A", "displacement_magnitude", "node:A", 0, "mm"),
    row("d:B", "displacement_magnitude", "node:B", 2, "mm"),
    row("d:C", "displacement_magnitude", "node:C", 1, "mm")
  ];
}

function overlayFor(id: string, resolution: ResultColourResolution, rows = stationRows(), standing = PASSING) {
  const model = overlayModel();
  const result = overlayResult(rows);
  const overlayCase = resultOverlayCases(model, result)[0];
  return { model, overlay: buildResultColourOverlay({ model, result, standing, quantity: quantity(id), overlayCase, resolution }) };
}

const stationsOf = (overlay: ReturnType<typeof overlayFor>["overlay"], member: string) =>
  overlay.members.get(member)?.stations.map((station) => [station.fraction, station.value]);

describe("the station grid", () => {
  it("places only the five stated station tokens, from end_i at 0 to end_j at 1", () => {
    expect(RESULT_STATION_FRACTIONS).toEqual({ end_i: 0, quarter_1: 0.25, midspan: 0.5, quarter_3: 0.75, end_j: 1 });
    for (const other of ["governing_station", "node", "summary", "", undefined, null, "toString"]) expect(stationFraction(other), String(other)).toBeNull();
  });

  it("is the grid the bundled invented reference states for its member rows", async () => {
    const { source } = await loadBundledMechanicsReference();
    const locations = new Set(source.results.filter((item) => item.kind === "element_local_bending_moment_y").map((item) => item.metadata?.location));
    expect([...locations].sort()).toEqual(Object.keys(RESULT_STATION_FRACTIONS).sort());
  });
});

describe("result colour along members", () => {
  it("draws a member from its stated stations: the largest magnitude at each, in order", () => {
    const { overlay } = overlayFor("normal-stress-extreme", "station");
    expect(overlay.state).toBe("available");
    expect(overlay.resolution).toBe("station");
    expect(stationsOf(overlay, "pipe:P-1")).toEqual([[0, 4], [0.5, 2], [1, 6]]);
    // The member's value is the largest drawn value, not the governing row that states no station.
    expect(overlay.members.get("pipe:P-1")).toMatchObject({ value: 6, rowId: "s:P-1:j" });
    expect(overlay.unplacedRowCount).toBe(1);
  });

  it("shows a member with fewer than two stated stations as one value, and says how many", () => {
    const { overlay } = overlayFor("normal-stress-extreme", "station");
    expect(overlay.members.get("pipe:P-2")).toMatchObject({ value: 5, stations: [] });
    expect(overlay.stationMemberCount).toBe(2);
  });

  it("leaves a station with a non-finite row without a value, and never colours beside it", () => {
    const { overlay } = overlayFor("normal-stress-extreme", "station");
    expect(stationsOf(overlay, "pipe:P-3")).toEqual([[0, 1], [0.5, null], [1, 2]]);
    expect(overlay.gapStationCount).toBe(1);
  });

  it("places a nodal quantity at the member's ends: From at 0, To at 1", () => {
    const { overlay } = overlayFor("displacement", "station");
    expect(stationsOf(overlay, "pipe:P-1")).toEqual([[0, 0], [1, 2]]);
    expect(stationsOf(overlay, "pipe:P-2")).toEqual([[0, 2], [1, 1]]);
    // P-3 has a row at node C only: one stated value.
    expect(stationsOf(overlay, "pipe:P-3")).toEqual([]);
    expect(overlay.members.get("pipe:P-3")).toMatchObject({ value: 1, stations: [] });
  });

  it("keeps one value per member as the special case: each member's value is the largest of all its rows", () => {
    const { overlay } = overlayFor("normal-stress-extreme", "member");
    expect(overlay.resolution).toBe("member");
    expect([...overlay.members].map(([id, member]) => [id, member.value, member.stations.length])).toEqual([
      ["pipe:P-1", 9, 0], ["pipe:P-2", 5, 0], ["pipe:P-3", 2, 0]
    ]);
    expect(overlay.stationMemberCount).toBe(0);
    expect(overlay.gapStationCount).toBe(0);
  });
});

describe("result colour along members: the stated span, the governing row and signs", () => {
  it("leaves the member's own tube in the no-value colour, so nothing outside its stated stations is coloured", () => {
    const rows: Row[] = [
      row("q1", "element_local_axial_force", "pipe:P-1", 2, "N", { metadata: at("quarter_1", "axial_force") }),
      row("q3", "element_local_axial_force", "pipe:P-1", 4, "N", { metadata: at("quarter_3", "axial_force") }),
      row("one", "element_local_axial_force", "pipe:P-2", 3, "N", { metadata: at("midspan", "axial_force") })
    ];
    const { model, overlay } = overlayFor("axial-force", "station", rows);
    expect(stationsOf(overlay, "pipe:P-1")).toEqual([[0.25, 2], [0.75, 4]]);
    const table = resultColourTable(model, overlay)!;
    const noValue = { light: resultNoValueHex("light"), dark: resultNoValueHex("dark") };
    // The sleeve spans 0.25 to 0.75; the tube beneath, ends included, is not given the member's colour.
    expect(table.get(entityKey({ type: "pipe", id: "pipe:P-1" }))).toEqual(noValue);
    // A member shown as one value keeps its colour, as with one value per member.
    expect(table.get(entityKey({ type: "pipe", id: "pipe:P-2" }))).toEqual({ light: resultScaleHex("light", 0.75), dark: resultScaleHex("dark", 0.75) });
    // One value per member is unchanged.
    const member = overlayFor("axial-force", "member", rows);
    expect(resultColourTable(member.model, member.overlay)!.get(entityKey({ type: "pipe", id: "pipe:P-1" }))).toEqual({ light: resultScaleHex("light", 1), dark: resultScaleHex("dark", 1) });
  });

  it("keeps the row that gives a station's value, not the first row stated there", () => {
    const rows: Row[] = [
      row("small", "element_local_normal_stress_extreme", "pipe:P-1", 1, "MPa", { metadata: at("end_i", "sigma_minus") }),
      row("large", "element_local_normal_stress_extreme", "pipe:P-1", 10, "MPa", { metadata: at("end_i", "sigma_plus") }),
      row("j", "element_local_normal_stress_extreme", "pipe:P-1", 2, "MPa", { metadata: at("end_j") })
    ];
    const { overlay } = overlayFor("normal-stress-extreme", "station", rows);
    expect(overlay.members.get("pipe:P-1")!.stations[0]).toMatchObject({ value: 10, rowId: "large", rowIds: ["small", "large"] });
    expect(contourValueLabels(overlay).labels.find((label) => label.fraction === 0)).toMatchObject({ value: 10, rowId: "large" });
  });

  const signedRows = (values: [number, number], conventions: [string, string] = ["section", "section"]): Row[] => [
    row("i", "element_local_bending_moment_z", "pipe:P-1", values[0], "N*m", { metadata: { ...at("quarter_1", "bending_moment_z"), sign_convention: conventions[0] } }),
    row("j", "element_local_bending_moment_z", "pipe:P-1", values[1], "N*m", { metadata: { ...at("quarter_3", "bending_moment_z"), sign_convention: conventions[1] } })
  ];

  it("marks an interval between stations of opposite sign under one convention, and does not blend across it", () => {
    const { overlay } = overlayFor("bending-moment-z", "station", signedRows([-3, 5]));
    expect(overlay.members.get("pipe:P-1")!.stations.map((station) => [station.value, station.sign])).toEqual([[3, -1], [5, 1]]);
    expect(overlay.signChangeIntervalCount).toBe(1);
    expect(interpolateStations(overlay.members.get("pipe:P-1")!.stations, 0.5)).toBeNull();
  });

  it("blends stations of the same sign, and does not compare signs stated under different conventions", () => {
    const same = overlayFor("bending-moment-z", "station", signedRows([-3, -5])).overlay;
    expect(same.signChangeIntervalCount).toBe(0);
    expect(interpolateStations(same.members.get("pipe:P-1")!.stations, 0.5)).toBe(4);
    const differing = overlayFor("bending-moment-z", "station", signedRows([-3, 5], ["i-end force", "section"])).overlay;
    expect(differing.signChangeIntervalCount).toBe(0);
    expect(interpolateStations(differing.members.get("pipe:P-1")!.stations, 0.5)).toBe(4);
  });

  it("infers no sign for an unsigned magnitude or an extremum whose governing component may switch", () => {
    expect(RESULT_COLOUR_QUANTITIES.filter((item) => item.signed).map((item) => item.id)).toEqual([
      "torsional-shear", "axial-force", "shear-force-y", "shear-force-z", "torsional-moment", "bending-moment-y", "bending-moment-z"
    ]);
    const rows: Row[] = [
      row("i", "element_local_normal_stress_extreme", "pipe:P-1", -3, "MPa", { metadata: at("end_i", "sigma_minus") }),
      row("j", "element_local_normal_stress_extreme", "pipe:P-1", 5, "MPa", { metadata: at("end_j", "sigma_plus") })
    ];
    const { overlay } = overlayFor("normal-stress-extreme", "station", rows);
    expect(overlay.members.get("pipe:P-1")!.stations.map((station) => station.sign)).toEqual([null, null]);
    expect(overlay.signChangeIntervalCount).toBe(0);
    expect(interpolateStations(overlay.members.get("pipe:P-1")!.stations, 0.5)).toBe(4);
  });
});

describe("result colour along members: the numerical standing", () => {
  it("withholds every station when the standing withholds the result: held, nothing coloured", () => {
    const { model, overlay } = overlayFor("normal-stress-extreme", "station", stationRows(), healthStanding({ eligible: false }));
    expect(overlay.state).toBe("held");
    expect(overlay.reason).toBe(RESULT_COLOUR_HELD_REASON);
    expect(overlay.members.size).toBe(0);
    expect(resultColourTable(model, overlay)).toBeNull();
  });

  it("withholds the stations of a case whose rows the results table badges other than checks passed", () => {
    const sensitiveL2 = healthStanding({ eligible: true, states: { "load:L-1": "checks_passed", "load:L-2": "sensitive" } });
    const rows: Row[] = [
      row("a:i", "element_local_axial_force", "pipe:P-1", 1, "N", { metadata: at("end_i", "axial_force") }),
      row("a:j", "element_local_axial_force", "pipe:P-1", 2, "N", { metadata: at("end_j", "axial_force") }),
      row("a:j:L2", "element_local_axial_force", "pipe:P-1", 7, "N", { basis_ref: { ref_type: "load_case", ref_id: "load:L-2" }, metadata: at("end_j", "axial_force") })
    ];
    const model = overlayModel();
    const result = overlayResult(rows);
    const second = resultOverlayCases(model, result)[1];
    const overlay = buildResultColourOverlay({ model, result, standing: sensitiveL2, quantity: quantity("axial-force"), overlayCase: second, resolution: "station" });
    expect(overlay.state).toBe("held");
    expect(overlay.heldRowCount).toBe(1);
  });
});

describe("the member table under a threshold and the numbers display", () => {
  it("colours only members above the threshold; at or below it, the plain pipe colour (no entry)", () => {
    const { model, overlay } = overlayFor("normal-stress-extreme", "member");
    const table = resultColourTable(model, overlay, { threshold: 5 })!;
    expect(table.get(entityKey({ type: "pipe", id: "pipe:P-1" }))).toEqual({ light: resultScaleHex("light", 1), dark: resultScaleHex("dark", 1) });
    expect(table.has(entityKey({ type: "pipe", id: "pipe:P-2" }))).toBe(false);
    expect(table.has(entityKey({ type: "pipe", id: "pipe:P-3" }))).toBe(false);
    // Without a threshold every member is coloured.
    expect(resultColourTable(model, overlay)!.size).toBe(3);
  });

  it("keeps a member with no value in the no-value colour under a threshold", () => {
    const rows = stationRows().filter((item) => item.entity_ref !== "pipe:P-3");
    const { model, overlay } = overlayFor("normal-stress-extreme", "member", rows);
    const table = resultColourTable(model, overlay, { threshold: 100 })!;
    expect(table.get(entityKey({ type: "pipe", id: "pipe:P-3" }))).toEqual({ light: resultNoValueHex("light"), dark: resultNoValueHex("dark") });
    expect(table.size).toBe(1);
  });

  it("leaves the figure neutral in the numbers display", () => {
    const { model, overlay } = overlayFor("normal-stress-extreme", "station");
    expect(resultColourTable(model, overlay, { display: "numbers" })).toBeNull();
  });
});

// --- Rule ratios from the session's rule-check run ---------------------------------------------

function pack(): SessionRuleCheckRun["rulePackDocument"] {
  const solver = (id: string, entity: string, location: string) => ({
    input_id: id, name: id, source_kind: "solver_result",
    quantity_intent: { dimension: "stress", unit_ref: "MPa" },
    solver_result_ref: { result_kind: "element_local_normal_stress_extreme", entity_ref: entity, location, component: "sigma_plus" }
  });
  const check = (id: string, inputs: string[]) => ({
    check_id: id, category_ref: { ref_type: "stress_category", ref_id: "cat_a" },
    required_input_refs: inputs.map((ref) => ({ ref_type: "required_input", ref_id: ref }))
  });
  return {
    required_inputs: [solver("in_i", "pipe:P-1", "end_i"), solver("in_j", "pipe:P-1", "end_j"), solver("in_p3", "pipe:P-3", "end_i"),
      { input_id: "limit_value", name: "limit", source_kind: "user_supplied_rule_value", quantity_intent: { dimension: "stress", unit_ref: "MPa" } }],
    stress_categories: [{ category_id: "cat_a", name: "Invented A" }],
    required_load_cases: [{ required_load_case_id: "rl1", name: "one", basis_ref: L1 }],
    load_case_mappings: [{ category_ref: { ref_type: "stress_category", ref_id: "cat_a" }, required_load_case_refs: [{ ref_type: "required_load_case", ref_id: "rl1" }] }],
    check_definitions: [check("chk_i", ["in_i", "limit_value"]), check("chk_j", ["in_j", "limit_value"]), check("chk_p3", ["in_p3", "limit_value"])]
  };
}

const value = (amount: number) => ({ value: amount, unit_ref: "MPa", dimension: "stress" });

function outcome(check: string, input: string, resultId: string, extra: Partial<RuleCheckOutcome>): RuleCheckOutcome {
  return {
    check_id: check, category_ref: "cat_a", basis_ref: L1, status: "USER_RULE_CHECKED", acceptability_relation: "less_than_or_equal",
    bound_inputs: [{ input_id: input, source_kind: "solver_result", supplied: true, value: 1, unit: "MPa", result_id: resultId }],
    completeness_findings: [], evaluator_findings: [], diagnostic_codes: [],
    ...extra
  };
}

function ratioRun(): SessionRuleCheckRun {
  const checks = [
    outcome("chk_i", "in_i", "s:P-1:i", { computed_value: value(4), limit_value: value(10) }),
    outcome("chk_j", "in_j", "s:P-1:j", { computed_value: value(6), limit_value: value(5), status: "USER_RULE_FAILED" }),
    outcome("chk_p3", "in_p3", "s:P-3:i", { status: "RULE_INPUTS_INCOMPLETE", acceptability_relation: "none" })
  ];
  return {
    rulePackDocument: pack(),
    result: { document_kind: "rule_check_run_result", rule_pack_id: "invented_pack", grammar_version: "1.0.0", aggregate_status: "RULE_INPUTS_INCOMPLETE", checks, professional_boundary_notice: "invented" }
  };
}

describe("result colour: rule ratios", () => {
  function ratioOverlay(resolution: ResultColourResolution, standing = PASSING) {
    const model = overlayModel();
    const result = overlayResult(stationRows());
    const ratioTable = buildStressRatioTable({ model, result, standing, ruleCheckRun: ratioRun() });
    const quantities = availableResultColourQuantities(model, result, ratioTable);
    const ratio = quantities.find((item) => item.source === "rule_ratio")!;
    const overlayCase = resultOverlayCases(model, result)[0];
    return { model, quantities, ratio, overlay: buildResultColourOverlay({ model, result, standing, quantity: ratio, overlayCase, resolution, ratioTable }) };
  }

  it("offers a ratio quantity only for a stress category of the user's run, and none without a run", () => {
    const { quantities, ratio } = ratioOverlay("station");
    expect(ratio).toMatchObject({ id: "rule-ratio:rule:cat_a", label: "Rule ratio · Invented A (cat_a)", dimension: "ratio", kind: "cat_a" });
    expect(quantities.filter((item) => item.source === "rule_ratio")).toHaveLength(1);
    const model = overlayModel();
    const result = overlayResult(stationRows());
    expect(availableResultColourQuantities(model, result, buildStressRatioTable({ model, result, standing: PASSING })).some((item) => item.source === "rule_ratio")).toBe(false);
    expect(availableResultColourQuantities(model, result).some((item) => item.source === "rule_ratio")).toBe(false);
  });

  it("colours each located outcome by its ratio along the member; an incomplete outcome colours nothing", () => {
    const { overlay } = ratioOverlay("station");
    expect(overlay.state).toBe("available");
    expect(overlay.unit).toBe("");
    expect(stationsOf(overlay, "pipe:P-1")).toEqual([[0, 0.4], [1, 1.2]]);
    expect(overlay.members.has("pipe:P-3")).toBe(false);
    expect(overlay.noRatioCount).toBe(1);
    expect(overlay.maximum).toBe(1.2);
  });

  it("colours no ratio the numerical standing withholds", () => {
    const { overlay } = ratioOverlay("station", healthStanding({ eligible: false }));
    expect(overlay.state).toBe("held");
    expect(overlay.members.size).toBe(0);
    expect(overlay.heldRowCount).toBe(3);
  });
});
