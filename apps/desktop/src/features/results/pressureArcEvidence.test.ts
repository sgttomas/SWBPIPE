// T4-U2: the TypeScript pressure-1 reader admits realized-arc envelopes over
// the shared arc corpus (the Rust and Python readers read the same bytes),
// refuses each broken arc binding by name, refuses arc evidence under v2, and
// covers a model whose only pipe is a replaced span by its connector
// (T4-RV23 NOTE-4).
import { describe, expect, it } from "vitest";
import corpus from "../../../../../fixtures/results/pressure_v3_arc_reader_corpus.json";
import type { MechanicsResult } from "../../types";
import { validatePhysicsEvidence } from "./physicsResultEvidence";
import { validateLoadReferenceEvidence } from "./loadReferenceEvidence";
import { PRESSURE_CONTRACT_ID, validateExactPressureEvidence, validatePressureEvidence } from "./pressureContractEvidence";

type Probe = Record<string, any>;
const cases: [string, Probe][] = (corpus as Probe).cases.map((c: Probe): [string, Probe] => [c.label, c.envelope]);
const arcCases = cases.filter(([, e]) => e.contract_evidence.pressure.some((r: Probe) => Object.hasOwn(r, "bend_adjacent_junctions")));
const copy = (source: Probe) => structuredClone(source) as unknown as MechanicsResult;
const loadState = (source: Probe) => Object.hasOwn(source.contract_evidence, "load_reference_states");
const row = (e: Probe, kind: string, entity: string, location: string) =>
  e.results.find((r: Probe) => r.kind === kind && r.entity_ref === entity && r.metadata?.location === location);
const region = (e: Probe) => e.contract_evidence.pressure[0];
const member = (items: Probe[], pipe: string) => items.find(i => i.pipe_id === pipe)!;

// [label, mutation, reason]: the same mutations as the Rust and Python tests.
const mutations: [string, (e: Probe) => void, string][] = [
  ["arc sign", e => { row(e, "pipe_wall_axial_force_v2", "pipe:BEND", "midspan").metadata.sign_convention = "tension-positive material wall section resultant Nw"; }, "ROW_SEMANTICS"],
  ["arc hoop row", e => { e.results.push({ ...structuredClone(row(e, "pipe_lame_hoop_stress_v2", "pipe:S1", "midspan")), entity_ref: "pipe:BEND", id: "result:arc-hoop" }); }, "ARC_WITHHELD"],
  ["arc maximum not withheld", e => { const c = e.contract_evidence.exact_cases[0].stress_maximum_coverage; c.unavailable_pipe_ids = []; c.complete = true; }, "ARC_WITHHELD"],
  ["arc region key removed", e => { delete region(e).tangency_rule; }, "(PRESSURE|REGION)_SHAPE"],
  ["arc approximation", e => { region(e).approximation = "long_straight_annulus_small_strain_v2"; }, "ARC_REGION_PROFILE"],
  ["arc withheld reason", e => { region(e).withheld_on_arcs.reason = "withheld"; }, "ARC_REGION_PROFILE"],
  ["arc member kind", e => { member(region(e).geometry, "pipe:BEND").member_kind = "realized_mitre"; }, "ARC_GEOMETRY"],
  ["arc strain sign", e => { const l = member(region(e).applied_loads, "pipe:BEND"); l.arc_pressure_strain = -l.arc_pressure_strain; }, "ARC_APPLIED_LOAD"],
  ["arc cap removal", e => { const l = member(region(e).applied_loads, "pipe:BEND"); l.bend_cap_pair_removed_global_n[0][0] = 2 * l.bend_cap_pair_removed_global_n[0][0] + 1; }, "ARC_APPLIED_LOAD"],
  ["junction dropped", e => { region(e).bend_adjacent_junctions.pop(); }, "ARC_JUNCTIONS"],
  ["junction theta", e => { region(e).bend_adjacent_junctions[0].theta_rad = 2e-3; }, "ARC_JUNCTIONS"],
  ["junction tangent", e => { region(e).bend_adjacent_junctions[0].t_in_global[1] += 1e-3; }, "ARC_JUNCTION_TANGENT"],
  ["arc end row missing", e => {
    const id = row(e, "pipe_wall_endpoint_action_v2", "pipe:BEND", "end_j").id;
    e.results = e.results.filter((r: Probe) => r.id !== id);
    region(e).result_ids = region(e).result_ids.filter((r: string) => r !== id);
  }, "PRESSURE_ROW_COVERAGE"],
  ["bend term on a straight", e => {
    const terms = e.contract_evidence.exact_cases[0].pressure_rhs_assembly.groups.flatMap((g: Probe) => g.terms);
    terms.find((t: Probe) => t.kind === "terminal_cap").kind = "bend_cap_removed";
  }, "ARC_RHS_TERM"],
  ["case-free quantity row", e => { delete row(e, "global_nodal_displacement_x", "node:B", "node").basis_ref; }, "ROW_CASE_REFERENCE"],
];

it("covers both document shapes, three arc regions and the replaced-span-only model", () => {
  expect(cases).toHaveLength(5);
  expect(arcCases).toHaveLength(3);
  expect(new Set(arcCases.map(([, s]) => loadState(s)))).toEqual(new Set([true, false]));
});

describe.each(cases)("%s", (_label, source) => {
  it("is admitted by the pressure-1 reader and dispatch, without mutation", () => {
    const before = structuredClone(source);
    expect(source.producer.semantic_contract_id).toBe(PRESSURE_CONTRACT_ID);
    expect(() => validatePressureEvidence(source as unknown as MechanicsResult)).not.toThrow();
    expect(() => validateExactPressureEvidence(source as unknown as MechanicsResult)).not.toThrow();
    expect(source).toEqual(before);
  });
});

describe.each(arcCases)("%s", (_label, source) => {
  it.each(mutations)("refuses a broken arc binding: %s", (_what, mutate, reason) => {
    const broken = structuredClone(source);
    mutate(broken);
    expect(() => validatePressureEvidence(copy(broken))).toThrow(new RegExp(`${reason}$`));
  });
  it("is refused under v2", () => {
    const asV2 = structuredClone(source);
    for (const c of asV2.contract_evidence.exact_cases) c.profile_mode = "exact_straight_pressure_v2";
    for (const r of asV2.contract_evidence.pressure) { r.profile_version = "2.0.0"; r.profile_mode = "exact_straight_pressure_v2"; }
    const reader = loadState(source) ? validateLoadReferenceEvidence : validatePhysicsEvidence;
    expect(() => reader(copy(asV2))).toThrow(/ARC_UNSUPPORTED$/);
  });
});

it("needs the connector of a replaced-span-only model", () => {
  const [, source] = cases.find(([label]) => label.startsWith("replaced span only"))!;
  expect(source.contract_evidence.exact_cases[0].pipe_sections).toEqual([]);
  const broken = structuredClone(source);
  broken.contract_evidence.connector = [];
  expect(() => validatePressureEvidence(copy(broken))).toThrow(/CASE_MEMBER_SCOPE$/);
});
