import { readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";
import { healthStanding, overlayModel, overlayResult, row } from "../../test-support/resultOverlayFixtures";
import { loadBundledMechanicsReference } from "../../services/previewService";
import type { MechanicsResult, PreviewModel } from "../../types";
import { numericalHealthView, parseNumericalHealth, type ResultOverlayStanding } from "./numericalHealth";
import { SUPPORT_LOAD_WITHHELD_NO_HEALTH, SUPPORT_LOAD_WITHHELD_NOT_ELIGIBLE, supportLoadWithheldStateText } from "./supportLoadSummary";
import {
  DISPLACEMENT_COMPONENTS,
  DISPLACEMENT_NO_CASES,
  DISPLACEMENT_NO_NODES,
  DISPLACEMENT_NO_RESULT,
  DISPLACEMENT_NOT_SOLVED,
  FAMILY_NOT_STATED,
  FILTER_ALL,
  FILTER_FAMILY_NOT_STATED,
  FILTER_HANGERS,
  FILTER_SUPPORTS,
  SUPPORT_FAMILIES,
  buildDisplacementExtremes,
  displacementExtremesView,
  extremeUnitsDifferNote,
  familyFilterId,
  sortUnitsDifferNote,
  type DisplacementExtremes,
  type DisplacementSort
} from "./displacementExtremes";

// A8, displacement extremes per node (DEL-07-05 Design `displacement-extremes.md` §2 to §5, §9).
// All data are invented (`resultOverlayFixtures`) or the bundled invented reference.

type Row = MechanicsResult["results"][number];
const L1 = { ref_type: "load_case", ref_id: "load:L-1" } as const;
const L2 = { ref_type: "load_case", ref_id: "load:L-2" } as const;
const C1 = { ref_type: "combination", ref_id: "combination:C-1" } as const;
const ENV = { ref_type: "combination", ref_id: "combination:ENV" } as const;
const PASSING = standingWith({ "load:L-1": "checks_passed", "load:L-2": "checks_passed" }, { "combination:C-1": "checks_passed", "combination:ENV": "checks_passed" });

/** Nodes A to D; supports of several kinds, two of them at node C, none at node B. */
function model(): PreviewModel {
  const base = overlayModel();
  return {
    ...base,
    nodes: base.nodes.map((node) => ({ ...node, label: node.id === "node:A" ? "Alpha" : "" })),
    supports: [
      { id: "support:S-A", label: "Anchor A", node: "node:A", family: "anchor", restraints: [] },
      { id: "support:S-C", label: "Guide C", node: "node:C", family: "guide", restraints: [] },
      { id: "support:H-C", label: "Hanger C", node: "node:C", family: "variable_spring_hanger", restraints: [] },
      { id: "support:S-D", label: "", node: "node:D", restraints: [] },
      { id: "support:N-D", label: "Gap D", node: "node:D", family: "nonlinear", nonlinear: { behavior: "gap", dof: "UY" }, restraints: [] }
    ],
    combinations: [
      { id: "combination:C-1", label: "Sum", basis: "mechanics", terms: [], provenance: "invented" },
      { id: "combination:ENV", label: "Range", basis: "range_envelope", terms: [], provenance: "invented" }
    ]
  } as unknown as PreviewModel;
}

/** A node's six movements for one case; `null` omits a component. */
function movements(node: string, basis: Row["basis_ref"], u: [number | null, number | null, number | null], r: [number | null, number | null, number | null] = [null, null, null], extra: Partial<Row> = {}): Row[] {
  const key = `${basis!.ref_type}:${basis!.ref_id}`;
  const one = (prefix: "displacement" | "rotation", unit: string, triple: (number | null)[]) => (["x", "y", "z"] as const).flatMap((axis, index) =>
    triple[index] === null ? [] : [row(`${prefix}:${node}:${axis}:${key}`, `global_nodal_${prefix}_${axis}`, node, triple[index]!, unit, {
      basis_ref: basis, metadata: { component: `nodal_${prefix}_${axis}`, coordinate_system: "global", location: "node", basis: "invented", sign_convention: "invented" }, ...extra
    })]);
  return [...one("displacement", "mm", u), ...one("rotation", "rad", r)];
}

function standingWith(cases: Record<string, "checks_passed" | "sensitive">, combinations: Record<string, "checks_passed" | "sensitive" | null>, eligible = true): ResultOverlayStanding {
  const parsed = parseNumericalHealth({
    route: "generic",
    verdict_only: false,
    cases: Object.entries(cases).map(([caseId, state]) => ({ case_id: caseId, state, code: null, integrity_diagnostic_id: null, rcond: null, digits: null, conditioning_sensitive: false, causes: [] })),
    combinations: Object.entries(combinations).map(([id, state]) => ({ combination_id: id, state, cases: Object.keys(cases), withheld: state === null }))
  });
  return { eligible, health: numericalHealthView(parsed, { eligible, verdictHold: false }) };
}

function build(rows: Row[], standing: ResultOverlayStanding | null = PASSING, theModel: PreviewModel = model()): DisplacementExtremes {
  const data = buildDisplacementExtremes({ model: theModel, result: overlayResult(rows), standing });
  expect(data.state).toBe("available");
  return data;
}

function view(data: DisplacementExtremes, options: { caseKeys?: readonly string[] | null; filter?: string; sort?: DisplacementSort } = {}) {
  return displacementExtremesView(data, { caseKeys: options.caseKeys ?? null, filter: options.filter ?? FILTER_ALL, sort: options.sort ?? "model" });
}

const nodeRow = (shown: ReturnType<typeof view>, nodeId: string) => shown.rows.find((item) => item.nodeId === nodeId)!;

const ROWS: Row[] = [
  ...movements("node:A", L1, [0.5, -0.25, 0], [0.001, 0, -0.002]),
  ...movements("node:A", L2, [-1, 0.75, 0], [0, 0.003, 0.001]),
  ...movements("node:A", C1, [-0.5, 0.5, 2], [0.001, 0.003, 0]),
  ...movements("node:B", L1, [3, 0, 0]),
  ...movements("node:B", L2, [-4, 0, 0]),
  ...movements("node:C", L1, [1, 1, 1]),
  ...movements("node:D", L2, [0, -9, 0])
];

describe("displacement extremes: support-kind vocabulary", () => {
  it("lists the model document's SupportFamily values, once each", () => {
    const schema = JSON.parse(readFileSync(path.join(process.cwd(), "..", "..", "schemas", "model.schema.yaml"), "utf8")) as { $defs: { SupportFamily: { enum: string[] } } };
    const listed = SUPPORT_FAMILIES.map((item) => item.value);
    expect(new Set(listed).size).toBe(listed.length);
    expect([...listed].sort()).toEqual([...schema.$defs.SupportFamily.enum].sort());
  });
});

describe("displacement extremes: cases and values", () => {
  it("lists the cases with movement rows in the viewport's order, with the model's labels and standing", () => {
    const data = build([...ROWS, row("stress", "element_local_axial_normal_stress", "pipe:P-1", 1, "MPa", { basis_ref: { ref_type: "load_case", ref_id: "load:L-9" } })]);
    expect(data.cases.map((item) => item.label)).toEqual(["Invented one (load:L-1)", "Invented two (load:L-2)", "Sum (combination:C-1)"]);
    expect(data.cases.every((item) => item.standing === "admitted" && item.choosable)).toBe(true);
    expect(data.cases.map((item) => item.rowCount)).toEqual([6 + 3 + 3, 6 + 3 + 3, 6]);
    expect(data.nodes.map((item) => item.nodeId)).toEqual(["node:A", "node:B", "node:C", "node:D"]);
    expect(data.nodes[0].label).toBe("Alpha");
    expect(data.nodes[1].label).toBe("node:B");
  });

  it("takes signed maximum and minimum per component with the governing case and row id", () => {
    const shown = view(build(ROWS));
    const a = nodeRow(shown, "node:A");
    expect(a.extremes.ux.max).toEqual({ value: 0.5, unit: "mm", resultId: "displacement:node:A:x:load_case:load:L-1", caseKey: "load_case:load:L-1", caseLabel: "Invented one (load:L-1)" });
    expect(a.extremes.ux.min).toMatchObject({ value: -1, caseKey: "load_case:load:L-2" });
    expect(a.extremes.uz.max).toMatchObject({ value: 2, caseKey: "combination:combination:C-1", caseLabel: "Sum (combination:C-1)" });
    expect(a.extremes.rz.min).toMatchObject({ value: -0.002, unit: "rad" });
    expect(a.extremes.ry.max).toMatchObject({ value: 0.003, caseKey: "load_case:load:L-2" });
    const d = nodeRow(shown, "node:D");
    expect(d.extremes.uy.max).toMatchObject({ value: -9 });
    expect(d.extremes.uy.min).toMatchObject({ value: -9 });
    expect(d.extremes.rx).toEqual({ max: null, min: null, note: null });
    expect(shown.units).toEqual({ ux: "mm", uy: "mm", uz: "mm", rx: "rad", ry: "rad", rz: "rad" });
    expect(Object.keys(a.extremes)).toEqual(DISPLACEMENT_COMPONENTS.map((item) => item.id));
  });

  it("names the first case in case order on a tie, and 0 ties -0", () => {
    const shown = view(build([...movements("node:A", L1, [7, -0, 0]), ...movements("node:A", L2, [7, 0, -0])]));
    const a = nodeRow(shown, "node:A");
    expect(a.extremes.ux.max?.caseKey).toBe("load_case:load:L-1");
    expect(a.extremes.ux.min?.caseKey).toBe("load_case:load:L-1");
    expect(a.extremes.uy.max?.caseKey).toBe("load_case:load:L-1");
    expect(Object.is(a.extremes.uy.max?.value, -0)).toBe(true);
    expect(a.extremes.uz.min?.caseKey).toBe("load_case:load:L-1");
  });

  it("forms no extreme across different units, says so, and leaves the header unit unstated", () => {
    const shown = view(build([...movements("node:A", L1, [1, 1, 1]), row("m", "global_nodal_displacement_x", "node:A", 0.002, "m", { basis_ref: L2 })]));
    const a = nodeRow(shown, "node:A");
    expect(a.extremes.ux).toEqual({ max: null, min: null, note: extremeUnitsDifferNote(["m", "mm"]) });
    expect(a.extremes.uy.max).toMatchObject({ value: 1 });
    expect(shown.units.uy).toBe("mm");
    const mixed = view(build([...movements("node:A", L1, [1, 1, 1]), ...movements("node:B", L1, [null, null, null]), row("m", "global_nodal_displacement_x", "node:B", 2, "m")]));
    expect(mixed.units.ux).toBeNull();
  });

  it("keeps the first emitted row of a component and case; ignores rows of other frames and nodes outside the model", () => {
    const data = build([
      ...movements("node:A", L1, [1, null, null]),
      row("dup", "global_nodal_displacement_x", "node:A", 99, "mm"),
      row("local", "global_nodal_displacement_y", "node:A", 5, "mm", { metadata: { component: "nodal_displacement_y", coordinate_system: "element_local", location: "node", basis: "invented", sign_convention: "invented" } }),
      row("stray", "global_nodal_displacement_z", "node:Z", 5, "mm"),
      row("nan", "global_nodal_displacement_z", "node:A", Number.NaN, "mm")
    ]);
    const a = nodeRow(view(data), "node:A");
    expect(a.extremes.ux.max).toMatchObject({ value: 1, resultId: "displacement:node:A:x:load_case:load:L-1" });
    expect(a.extremes.uy.max).toBeNull();
    expect(a.extremes.uz.max).toBeNull();
    expect(data.unusedRows).toEqual({ otherFrame: 1, notModelNode: 1 });
  });
});

describe("displacement extremes: standing, envelopes and case choice", () => {
  it("withholds a case the standing does not admit: no values kept, a reason, counted", () => {
    const data = build(ROWS, standingWith({ "load:L-1": "checks_passed", "load:L-2": "sensitive" }, { "combination:C-1": "sensitive" }));
    const [, second, third] = data.cases;
    expect(second).toMatchObject({ standing: "withheld", withheldReason: supportLoadWithheldStateText("sensitive"), choosable: false, rowCount: 12 });
    expect(third).toMatchObject({ standing: "withheld", choosable: false });
    expect(data.nodes[0].values.ux[1]).toBeUndefined();
    expect(data.nodes[0].values.ux[2]).toBeUndefined();
    const shown = view(data);
    expect(shown.chosenCases.map((item) => item.key)).toEqual(["load_case:load:L-1"]);
    expect(shown.exclusions).toEqual({ withheld: 2, withheldRows: 18, envelope: 0, notChosen: 0 });
    expect(nodeRow(shown, "node:A").extremes.ux.min).toMatchObject({ value: 0.5, caseKey: "load_case:load:L-1" });
    expect(nodeRow(shown, "node:D").extremes.uy.max).toBeNull();
  });

  it("withholds every case when the result is not eligible, has no health view or no standing", () => {
    const ineligible = build(ROWS, healthStanding({ eligible: false }));
    expect(ineligible.cases.every((item) => item.standing === "withheld" && item.withheldReason === SUPPORT_LOAD_WITHHELD_NOT_ELIGIBLE)).toBe(true);
    expect(view(ineligible).chosenCases).toEqual([]);
    expect(view(ineligible).rows.every((item) => DISPLACEMENT_COMPONENTS.every((c) => item.extremes[c.id].max === null))).toBe(true);
    const noHealth = build(ROWS, { eligible: true, health: null });
    expect(noHealth.cases.every((item) => item.withheldReason === SUPPORT_LOAD_WITHHELD_NO_HEALTH)).toBe(true);
    expect(build(ROWS, null).cases.every((item) => item.standing === "withheld")).toBe(true);
  });

  it("admits the pressure-source route at eligible standing (no row badges there)", () => {
    const data = build(ROWS, healthStanding({ eligible: true, route: "pressure_source", states: { "load:L-1": "sensitive", "load:L-2": "sensitive" } }));
    expect(data.cases.map((item) => item.standing)).toEqual(["admitted", "admitted", "admitted"]);
  });

  it("lists a range envelope, never lets it be chosen and keeps it out of the extremes", () => {
    const envelope = movements("node:A", ENV, [999, -999, 999]).map((item) => ({ ...item, metadata: { ...item.metadata!, basis: "explicit_user_range_envelope" } }));
    const data = build([...ROWS, ...envelope]);
    const env = data.cases.find((item) => item.envelope)!;
    expect(env).toMatchObject({ label: "Range (combination:ENV)", standing: "admitted", choosable: false });
    const shown = view(data, { caseKeys: data.cases.map((item) => item.key) });
    expect(shown.chosenCases.some((item) => item.envelope)).toBe(false);
    expect(nodeRow(shown, "node:A").extremes.ux.max?.value).toBe(0.5);
    expect(nodeRow(shown, "node:A").extremes.uy.min?.value).toBe(-0.25);
    expect(shown.exclusions.envelope).toBe(1);
  });

  it("takes the extremes over the chosen cases only and counts the rest", () => {
    const data = build(ROWS);
    const shown = view(data, { caseKeys: ["load_case:load:L-2", "combination:combination:C-1", "load_case:load:NOPE"] });
    expect(shown.chosenCases.map((item) => item.key)).toEqual(["load_case:load:L-2", "combination:combination:C-1"]);
    expect(shown.exclusions.notChosen).toBe(1);
    const a = nodeRow(shown, "node:A");
    expect(a.extremes.ux.max).toMatchObject({ value: -0.5, caseKey: "combination:combination:C-1" });
    expect(nodeRow(shown, "node:C").extremes.ux.max).toBeNull();
    const none = view(data, { caseKeys: [] });
    expect(none.chosenCases).toEqual([]);
    expect(none.exclusions.notChosen).toBe(3);
  });
});

describe("displacement extremes: support-kind filter", () => {
  const data = build(ROWS);
  const listed = (filter: string) => view(data, { filter }).rows.map((item) => item.nodeId);

  it("offers all nodes, support nodes, hangers, each stated family present and family not stated, with counts", () => {
    expect(data.filters.map((item) => [item.id, item.label, item.nodeCount])).toEqual([
      [FILTER_ALL, "All nodes", 4],
      [FILTER_SUPPORTS, "Support nodes", 3],
      [FILTER_HANGERS, "Hangers", 1],
      [familyFilterId("anchor"), "Anchor", 1],
      [familyFilterId("guide"), "Guide", 1],
      [familyFilterId("variable_spring_hanger"), "Variable spring hanger", 1],
      [familyFilterId("nonlinear"), "Nonlinear support", 1],
      [FILTER_FAMILY_NOT_STATED, FAMILY_NOT_STATED, 1]
    ]);
  });

  it("lists the nodes of each kind once, and nodes without supports only under all nodes", () => {
    expect(listed(FILTER_ALL)).toEqual(["node:A", "node:B", "node:C", "node:D"]);
    expect(listed(FILTER_SUPPORTS)).toEqual(["node:A", "node:C", "node:D"]);
    expect(listed(FILTER_HANGERS)).toEqual(["node:C"]);
    expect(listed(familyFilterId("anchor"))).toEqual(["node:A"]);
    expect(listed(familyFilterId("guide"))).toEqual(["node:C"]);
    expect(listed(familyFilterId("variable_spring_hanger"))).toEqual(["node:C"]);
    expect(listed(familyFilterId("nonlinear"))).toEqual(["node:D"]);
    expect(listed(FILTER_FAMILY_NOT_STATED)).toEqual(["node:D"]);
    expect(listed("unknown")).toEqual(["node:A", "node:B", "node:C", "node:D"]);
    expect(view(data, { filter: FILTER_HANGERS }).filter.label).toBe("Hangers");
  });

  it("names each support with its kind, a nonlinear behavior and DOF included", () => {
    const c = data.nodes.find((item) => item.nodeId === "node:C")!;
    expect(c.supports.map((item) => [item.label, item.kindText, item.hanger])).toEqual([["Guide C", "Guide", false], ["Hanger C", "Variable spring hanger", true]]);
    const d = data.nodes.find((item) => item.nodeId === "node:D")!;
    expect(d.supports.map((item) => [item.label, item.family, item.kindText])).toEqual([["support:S-D", null, FAMILY_NOT_STATED], ["Gap D", "nonlinear", "Nonlinear support (gap, UY)"]]);
    expect(data.nodes.find((item) => item.nodeId === "node:B")!.supports).toEqual([]);
  });

  it("filters by every family, a hanger record, a null family and a preserved unrecognized value", () => {
    const families = SUPPORT_FAMILIES.map((item) => item.value);
    const base = model();
    const every = {
      ...base,
      nodes: [...families, "hanger-record", "null", "odd"].map((name, index) => ({ id: `node:${name}`, label: name, position: { x: index, y: 0, z: 0 } })),
      supports: [
        ...families.map((family) => ({ id: `support:${family}`, node: `node:${family}`, family, restraints: [] })),
        { id: "support:hanger-record", node: "node:hanger-record", family: "spring", hanger: { hanger_type: "constant_effort_support" }, restraints: [] },
        { id: "support:null", node: "node:null", family: null, restraints: [] },
        { id: "support:odd", node: "node:odd", family: "strut", restraints: [] }
      ]
    } as unknown as PreviewModel;
    const everyData = build([row("d", "global_nodal_displacement_x", "node:anchor", 1, "mm")], PASSING, every);
    for (const family of families) {
      const expected = family === "spring" ? ["node:spring", "node:hanger-record"] : [`node:${family}`];
      expect(view(everyData, { filter: familyFilterId(family) }).rows.map((item) => item.nodeId)).toEqual(expected);
    }
    // "Hangers" is the report's rule (`isSpringHangerSupport`), the solver's: `spring_hanger` counts.
    expect(view(everyData, { filter: FILTER_HANGERS }).rows.map((item) => item.nodeId)).toEqual(["node:variable_spring_hanger", "node:spring_hanger", "node:constant_effort_support", "node:hanger-record"]);
    expect(everyData.filters.find((item) => item.id === FILTER_HANGERS)?.nodeCount).toBe(4);
    expect(view(everyData, { filter: FILTER_FAMILY_NOT_STATED }).rows.map((item) => item.nodeId)).toEqual(["node:null"]);
    expect(view(everyData, { filter: familyFilterId("strut") }).rows.map((item) => item.nodeId)).toEqual(["node:odd"]);
    expect(everyData.filters.find((item) => item.id === familyFilterId("strut"))?.label).toBe('"strut" (as stated)');
    expect(everyData.filters.map((item) => item.id).slice(3, 3 + families.length)).toEqual(families.map(familyFilterId));
  });
});

describe("displacement extremes: sort", () => {
  it("sorts by the largest magnitude of a component, ties in model order, unranked nodes last", () => {
    const data = build([
      ...movements("node:A", L1, [2, 0, 0]),
      ...movements("node:B", L1, [-5, 0, 0]),
      ...movements("node:C", L1, [5, 0, 0]),
      ...movements("node:D", L1, [null, 1, 0])
    ]);
    const shown = view(data, { sort: "ux" });
    expect(shown.rows.map((item) => item.nodeId)).toEqual(["node:B", "node:C", "node:A", "node:D"]);
    expect(shown.rows.map((item) => item.key)).toEqual([5, 5, 2, null]);
    expect(shown.sort).toEqual({ component: "ux", sorted: true, note: null, rankedCount: 3 });
    expect(view(data).rows.map((item) => item.nodeId)).toEqual(["node:A", "node:B", "node:C", "node:D"]);
    expect(view(data).sort).toEqual({ component: null, sorted: false, note: null, rankedCount: 0 });
    // 0 ties -0.
    const zeros = view(build([...movements("node:A", L1, [null, -0, null]), ...movements("node:B", L1, [null, 0, null])]), { sort: "uy" });
    expect(zeros.rows.slice(0, 2).map((item) => item.nodeId)).toEqual(["node:A", "node:B"]);
  });

  it("keeps model order and says so when the sort component's units differ", () => {
    const data = build([...movements("node:A", L1, [1, 0, 0]), row("m", "global_nodal_displacement_x", "node:B", 2, "m")]);
    const shown = view(data, { sort: "ux" });
    expect(shown.rows.map((item) => item.nodeId)).toEqual(["node:A", "node:B", "node:C", "node:D"]);
    expect(shown.sort).toEqual({ component: "ux", sorted: false, note: sortUnitsDifferNote(["m", "mm"]), rankedCount: 0 });
  });
});

describe("displacement extremes: unavailable states", () => {
  it("states why nothing is listed", () => {
    expect(buildDisplacementExtremes({ model: model(), result: null, standing: PASSING })).toMatchObject({ state: "unavailable", reason: DISPLACEMENT_NO_RESULT, nodes: [] });
    const unsolved = { ...overlayResult([]), status: { mechanics: "MECHANICS_BLOCKED", rule_check: "not_run", professional_acceptance: "not_claimed" } };
    expect(buildDisplacementExtremes({ model: model(), result: unsolved, standing: PASSING }).reason).toBe(DISPLACEMENT_NOT_SOLVED);
    expect(buildDisplacementExtremes({ model: { ...model(), nodes: [] }, result: overlayResult(ROWS), standing: PASSING }).reason).toBe(DISPLACEMENT_NO_NODES);
    expect(buildDisplacementExtremes({ model: model(), result: overlayResult([row("d", "displacement_magnitude", "node:A", 1, "mm")]), standing: PASSING }).reason).toBe(DISPLACEMENT_NO_CASES);
    const empty = buildDisplacementExtremes({ model: model(), result: null, standing: PASSING });
    expect(displacementExtremesView(empty, { caseKeys: null, filter: FILTER_ALL, sort: "model" })).toMatchObject({ rows: [], chosenCases: [] });
  });
});

describe("displacement extremes: the bundled invented reference", () => {
  it("forms each node's extremes from the emitted rows over both load cases", async () => {
    const { model: reference, source } = await loadBundledMechanicsReference();
    const standing = healthStanding({ eligible: true, states: { "load:L-100": "checks_passed", "load:L-200": "checks_passed" } });
    const data = buildDisplacementExtremes({ model: reference, result: source, standing });
    expect(data.state).toBe("available");
    expect(data.nodes.map((item) => item.nodeId)).toEqual(reference.nodes.map((item) => item.id));
    const shown = displacementExtremesView(data, { caseKeys: null, filter: FILTER_ALL, sort: "model" });
    expect(shown.chosenCases.length).toBe(data.cases.filter((item) => item.choosable).length);
    for (const node of shown.rows) {
      for (const component of DISPLACEMENT_COMPONENTS) {
        const emitted = source.results.filter((item) => item.kind === component.kind && item.entity_ref === node.nodeId && Number.isFinite(item.value)
          && shown.chosenCases.some((chosen) => chosen.key === `${item.basis_ref?.ref_type}:${item.basis_ref?.ref_id}`));
        const extreme = node.extremes[component.id];
        if (emitted.length === 0) { expect(extreme.max).toBeNull(); continue; }
        expect(extreme.max?.value).toBe(Math.max(...emitted.map((item) => item.value)));
        expect(extreme.min?.value).toBe(Math.min(...emitted.map((item) => item.value)));
        expect(emitted.some((item) => item.id === extreme.max?.resultId && item.value === extreme.max?.value)).toBe(true);
      }
    }
    const guides = displacementExtremesView(data, { caseKeys: null, filter: familyFilterId("guide"), sort: "model" });
    expect(guides.rows.map((item) => item.nodeId)).toEqual(
      reference.nodes.map((item) => item.id).filter((id) => reference.supports.some((support) => support.node === id && support.family === "guide"))
    );
  });
});
