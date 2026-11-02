// The cases of C1 slice 2's parity (DEL-04-05 `c1-native-custody-plan.md`, slice 2): each result
// fixture the TypeScript builders' tests, the demos and the native mechanics replay use, with its
// model and a reader standing, and invented cases for the paths the fixtures do not reach (withheld
// and non-eligible standings, missing members, tied and signed extrema, mixed units, other frames,
// row order, duplicate ids, the per-row combination evidence). Written to
// `fixtures/results/result_summary_cases.json`, which the desktop's Rust test reads to build and
// commit the native summaries (`result_summary_tests.rs`). Invented data; no engineering meaning.

import type { MechanicsResult, PreviewModel } from "../types";
import { jsonTextKeepingNegativeZero } from "../services/jsonText";
import { overlayModel, overlayResult, reaction as overlayReaction, row as overlayRow, standardRows } from "./resultOverlayFixtures";
import { keyModel, keyRows } from "./keyResultsFixtures";

type Row = MechanicsResult["results"][number];
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

/** A text: a file's own (`pointer` empty), a member of a file by JSON pointer, or inline. */
export type CaseText = Readonly<{ path: string; pointer: string } | { text: string }>;
/** `findings`: the native command's standing (health read natively); `health`: an invented health. */
export type CaseStanding = null | Readonly<{ eligible: boolean; findings: readonly string[] }> | Readonly<{ eligible: boolean; health: Json }>;
export type SummaryCase = Readonly<{ name: string; result: CaseText; model: CaseText; standing: CaseStanding }>;

const ELIGIBLE: CaseStanding = Object.freeze({ eligible: true, findings: [] });
const NOT_ELIGIBLE: CaseStanding = Object.freeze({ eligible: false, findings: [] });
const file = (path: string, pointer = ""): CaseText => ({ path, pointer });
/** The overlay fixtures' row helpers, taking the partial or ill-typed members the invented rows state. */
const row = (id: string, kind: string, entity: string, value: number, unit: string, extra: Json = {}): Row => overlayRow(id, kind, entity, value, unit, extra);
const reaction = (support: string, component: string, value: number, unit: string, extra: Json = {}): Row => overlayReaction(support, component, value, unit, extra);
const inline = (value: unknown): CaseText => ({ text: jsonTextKeepingNegativeZero(value) });

type Standings = readonly ("eligible" | "none" | "not_eligible")[];
const STANDINGS = { eligible: ELIGIBLE, none: null, not_eligible: NOT_ELIGIBLE } as const;

/** The result fixtures, each with its model (`base_reader_outcomes.rs`'s pairs where it has one). */
const FILE_CASES: readonly [string, CaseText, CaseText, Standings][] = [
  ["results/preview_physics_connected_sparse", file("fixtures/results/preview_physics_connected_sparse.json"), file("fixtures/model_operations/precision_connected_ui_model.json"), ["eligible", "none", "not_eligible"]],
  ["results/preview_physics_connected_dense", file("fixtures/results/preview_physics_connected_dense.json"), file("fixtures/model_operations/precision_connected_ui_model.json"), ["eligible"]],
  ["results/preview_physics_invented_sparse", file("fixtures/results/preview_physics_invented_sparse.json"), file("core/product_physics/tests/fixtures/preview_physics_invented_model.json"), ["eligible"]],
  ["results/preview_physics_unicode_ids_sparse", file("fixtures/results/preview_physics_unicode_ids_sparse.json"), file("core/product_physics/tests/fixtures/preview_physics_unicode_ids_model.json"), ["eligible"]],
  ["results/pressure_connected_ui_mechanics_sparse", file("fixtures/results/pressure_connected_ui_mechanics_sparse.json"), file("fixtures/model_operations/exact_pressure_authoring_model.json"), ["eligible", "none"]],
  ["results/pressure_connected_ui_mechanics_dense", file("fixtures/results/pressure_connected_ui_mechanics_dense.json"), file("fixtures/model_operations/exact_pressure_authoring_model.json"), ["eligible"]],
  ["results/pressure_thermal_ui_mechanics_sparse", file("fixtures/results/pressure_thermal_ui_mechanics_sparse.json"), file("fixtures/model_operations/physics_thermal_ui_model.json"), ["eligible"]],
  ["results/pressure_u4b_arc_sparse", file("fixtures/results/pressure_u4b_arc_sparse.json"), file("core/product_physics/tests/fixtures/u4b_l90_p_model.json"), ["eligible"]],
  ["results/preview_physics_u4b_arc_sparse", file("fixtures/results/preview_physics_u4b_arc_sparse.json"), file("core/product_physics/tests/fixtures/u4b_z_preview_rev_model.json"), ["eligible"]],
  ["results/preview_physics_range_envelope_sparse", file("fixtures/results/preview_physics_range_envelope_sparse.json"), file("core/product_physics/tests/fixtures/preview_physics_range_envelope_model.json"), ["eligible"]],
  ["results/preview_physics_timoshenko_arc_sparse", file("fixtures/results/preview_physics_timoshenko_arc_sparse.json"), file("validation/references/pf_timoshenko/requests/PFT-ARC-TIP-TIMOSHENKO.request.json", "/model"), ["eligible"]],
  ["results/preview_physics_timoshenko_partial_span_blocked_sparse", file("fixtures/results/preview_physics_timoshenko_partial_span_blocked_sparse.json"), file("validation/references/pf_timoshenko/requests/PFT-SB-L1-CM0-TIMOSHENKO.request.json", "/model"), ["eligible"]],
  ["results/invented_demo_result_preview_physics_1_sparse", file("fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json"), file("fixtures/product_preview/invented_demo_model.json"), ["eligible", "not_eligible"]],
  ["results/invented_demo_result_preview_physics_1_dense", file("fixtures/product_preview/invented_demo_result_preview_physics_1_dense.json"), file("fixtures/product_preview/invented_demo_model.json"), ["eligible"]],
  ["results/invented_mechanics_result_preview_physics_1_sparse", file("fixtures/product_preview/invented_mechanics_result_preview_physics_1_sparse.json"), file("fixtures/product_preview/invented_preview_model.json"), ["eligible"]],
  ...[0, 1, 2, 3].map((i): [string, CaseText, CaseText, Standings] => [
    `nonlinear_operands/${i}`,
    file("fixtures/results/preview_physics_nonlinear_operands_corpus.json", `/documents/${i}/document`),
    file("fixtures/results/preview_physics_nonlinear_operands_corpus.json", `/documents/${i}/model`),
    ["eligible"]
  ]),
  ...[0, 3].map((i): [string, CaseText, CaseText, Standings] => [
    `combination/${i}`,
    file("fixtures/results/pressure_v3_combination_reader_corpus.json", `/cases/${i}/envelope`),
    file("fixtures/results/pressure_v3_combination_reader_corpus.json", `/cases/${i}/model`),
    ["eligible"]
  ]),
  ["pressure_source/n05-sparse_interactive", file("fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json"), file("fixtures/product_preview/pressure_source/physics_source/n05.request.json", "/model"), ["eligible"]]
];

/** An invented health of the generic route (or another), as `healthStanding` builds it. */
function health(states: Record<string, string>, options: { route?: string; combinations?: Record<string, string | null>; withheld?: readonly string[] } = {}): Json {
  return {
    route: options.route ?? "generic",
    verdict_only: false,
    cases: Object.entries(states).map(([caseId, state]) => ({
      case_id: caseId, state, code: null, integrity_diagnostic_id: null, rcond: null, digits: null, conditioning_sensitive: false, causes: []
    })),
    combinations: Object.entries(options.combinations ?? {}).map(([id, state]) => ({
      combination_id: id, state, cases: Object.keys(states), withheld: (options.withheld ?? []).includes(id)
    }))
  };
}
const PASSED = { "load:L-1": "checks_passed", "load:L-2": "checks_passed" };

const L1 = { ref_type: "load_case", ref_id: "load:L-1" } as const;
const L2 = { ref_type: "load_case", ref_id: "load:L-2" } as const;
const C1 = { ref_type: "combination", ref_id: "comb:C1" } as const;
const RANGE = { ref_type: "combination", ref_id: "comb:R" } as const;
const ENVELOPE_META = { coordinate_system: "global", basis: "explicit_user_range_envelope" };
const global = (extra: Record<string, unknown> = {}) => ({ coordinate_system: "global", ...extra });

/** A model with every support form the tables read: anchors, a hanger by its explicit type, a
 * nonlinear support, a snubber (model 0.9.0's family), an unknown family, none stated, a boundary
 * association and interfaces. */
function supportModel(): PreviewModel {
  const model = overlayModel() as PreviewModel & Record<string, unknown>;
  model.nodes = [...model.nodes, { id: "node:E", label: "", position: { x: 3, y: 1, z: 0 } }, { id: "node:A", label: "Again", position: { x: 9, y: 9, z: 9 } }] as PreviewModel["nodes"];
  model.supports = [
    { id: "support:S-A", node: "node:A", label: "Anchor A", family: "anchor" },
    { id: "support:S-C", node: "node:C", label: "", family: "guide",
      boundary_association: { boundary_id: "b:C", kind: "equipment_nozzle", equipment_reference: "EQ-1", coordinate_system: "global" } },
    { id: "support:S-D", node: "node:D", family: "vertical_support", hanger: { hanger_type: " spring_hanger " } },
    { id: "support:S-E", node: "node:E", label: "Lift", family: "nonlinear", nonlinear: { behavior: "lift_off", dof: "uy" } },
    { id: "support:S-F", node: "node:E", label: "Custom", family: "custom \"form\"" },
    { id: "support:S-H", node: "node:C", label: "Snub", family: "snubber" },
    { id: "support:S-G", node: "node:B" }
  ] as unknown as PreviewModel["supports"];
  model.combinations = [{ id: "comb:C1", label: "Combined" }, { id: "comb:R", label: "" }] as unknown as PreviewModel["combinations"];
  model.equipment_interfaces = [{ id: "ei:1", node_ref: "node:C", label: "Pump flange" }, { id: "ei:2", node_ref: "node:C" }] as unknown as PreviewModel["equipment_interfaces"];
  return model;
}

/** Reactions and node movements with every listed path: complete and partial triples, magnitudes,
 * another frame, an absent frame, a withheld attribution, tied, signed and mixed-unit values, a
 * duplicate row (the first is kept), an envelope combination, rows of a node outside the model,
 * rows with no basis, no unit, no metadata or a value that is not a number. */
function supportRows(): Row[] {
  const movement = (node: string, kind: string, value: number | null, basis: Json, extra: Json = {}): Row =>
    row(`m:${node}:${kind}:${basis?.ref_id ?? "none"}:${value}`, kind, node, value as number, kind.includes("rotation") ? "rad" : "mm", { basis_ref: basis, metadata: global(), ...extra });
  return [
    ...["Fx", "Fy", "Fz", "Mx", "My", "Mz"].map((c, i) => reaction("support:S-A", c, (i % 2 ? -1 : 1) * (i + 1) * 100, c.startsWith("F") ? "N" : "N*m")),
    row("r:S-A:F", "support_reaction_force_magnitude_v2", "support:S-A", 374.17, "N", { metadata: global() }),
    reaction("support:S-A", "Fx", 600, "N", { id: "r:S-A:Fx:L2", basis_ref: L2 }),
    reaction("support:S-A", "Fx", 600, "N", { id: "r:S-A:Fx:L2:dup", basis_ref: L2 }),
    reaction("support:S-A", "Fy", -0, "N", { id: "r:S-A:Fy:L2", basis_ref: L2 }),
    reaction("support:S-A", "Fx", 0.6, "kN", { id: "r:S-A:Fx:C1", basis_ref: C1 }),
    reaction("support:S-A", "Fx", 999, "N", { id: "r:S-A:Fx:R", basis_ref: RANGE, metadata: { component: "Fx", ...ENVELOPE_META } }),
    reaction("support:S-C", "Fx", 5, "N", { metadata: { component: "Fx", coordinate_system: "element_local" } }),
    reaction("support:S-C", "Fy", 6, "N", { id: "r:S-C:Fy:L2", basis_ref: L2, metadata: { component: "Fy" } }),
    reaction("support:S-D", "Fz", 7, "N", { id: "r:S-D:Fz:L2", basis_ref: L2 }),
    reaction("support:S-D", "Qx", 8, "N", { id: "r:S-D:Qx:L2", basis_ref: L2 }),
    reaction("support:S-G", "Fx", 1, "N", { id: "r:S-G:nobasis", basis_ref: undefined }),
    movement("node:A", "global_nodal_displacement_x", 1.5, L1),
    movement("node:A", "global_nodal_displacement_x", 1.5, L2),
    movement("node:A", "global_nodal_displacement_x", -1.5, C1),
    movement("node:A", "global_nodal_displacement_y", 0, L1),
    movement("node:A", "global_nodal_displacement_y", -0, L2),
    movement("node:A", "global_nodal_rotation_z", 0.001, L1),
    movement("node:A", "global_nodal_rotation_z", 0.002, L1),
    movement("node:C", "global_nodal_displacement_z", 2, L1, { metadata: { coordinate_system: "element_local" } }),
    movement("node:C", "global_nodal_displacement_z", 3, L2, { metadata: undefined }),
    movement("node:C", "global_nodal_displacement_z", 4, L1, { unit: "in" }),
    movement("node:D", "global_nodal_displacement_x", 9, RANGE, { metadata: ENVELOPE_META }),
    movement("node:D", "global_nodal_displacement_x", null, L1),
    movement("node:Z", "global_nodal_displacement_x", 3, L1),
    movement("node:E", "global_nodal_rotation_x", 0.5, L2, { unit: undefined }),
    movement("node:E", "global_nodal_rotation_y", -0.25, L1, { metadata: { coordinate_system: null } as unknown as Row["metadata"] }),
    row("d:A:L1", "displacement_magnitude", "node:A", 1.5, "mm", { metadata: global() }),
    row("d:E:L2", "displacement_magnitude", "node:E", 0.5, "mm", { basis_ref: L2 }),
    row("state:S-E:L1", "nonlinear_support_active_set_state_code", "support:S-E", 1, "state_code", { metadata: { coordinate_system: "solver_iteration" } }),
    row("state:S-E:C1", "nonlinear_support_active_set_state_code", "support:S-E", 0, "state_code", { basis_ref: C1 }),
    row("state:S-E:none", "nonlinear_support_active_set_state_code", "support:S-E", 2, "state_code", { basis_ref: undefined })
  ];
}
const ATTRIBUTION = { preview_cases: [{ load_case_id: "load:L-1", support_attribution: { withheld: [{ support_id: "support:S-D", reason: "ambiguous_attribution" }, { support_id: 7 }, { support_id: "support:S-G" }] } }, null, "x"] };

/** Stress rows with tied magnitudes, signs, envelope rows, section bases, a row whose station is
 * not a string, rows of an unknown case, and the colour quantities' station rows. */
function stressRows(): Row[] {
  const station = (location: unknown, basis = "u4b; section_basis=reduced_section") => ({ component: "sigma_plus", coordinate_system: "pipe_section", location, basis, sign_convention: "j-side section action" }) as unknown as Row["metadata"];
  return [
    row("s:1", "element_local_normal_stress_extreme", "pipe:P-1", -8, "MPa", { metadata: station("end_i") }),
    row("s:2", "element_local_normal_stress_extreme", "pipe:P-1", 8, "MPa", { metadata: station("end_j", "u4b; section_basis=nominal_section") }),
    row("s:3", "element_local_normal_stress_extreme", "pipe:P-2", 8, "MPa", { basis_ref: L2, metadata: station("midspan", "none") }),
    row("s:4", "element_local_normal_stress_extreme", "pipe:P-2", 3, "MPa", { basis_ref: RANGE, metadata: { ...station("end_i"), basis: "explicit_user_range_envelope" } as Row["metadata"] }),
    row("s:5", "element_local_normal_stress_extreme", "pipe:P-3", null as unknown as number, "MPa", { metadata: station("quarter_1") }),
    row("s:6", "pipe_elastic_normal_stress_maximum_v2", "pipe:P-3", 4e6, "Pa", { metadata: station(7) }),
    row("s:7", "pipe_elastic_normal_stress_maximum_v2", "pipe:P-3", -4e6, "Pa", { basis_ref: L2, metadata: station("end_i", "x; section_basis=other") }),
    row("s:8", "element_local_torsional_shear_stress", "pipe:P-1", -2, "MPa", { metadata: { ...station("end_i"), sign_convention: "positive value follows the element-local DOF at the i-end force vector" } as Row["metadata"] }),
    row("s:9", "element_local_torsional_shear_stress", "pipe:P-1", 1, "MPa", { metadata: station("midspan") }),
    row("s:10", "element_local_torsional_shear_stress", "pipe:P-1", 1, "MPa", { metadata: station("end_j") }),
    row("s:11", "element_local_axial_force", "pipe:P-2", -100, "N", { metadata: station("end_i") }),
    row("s:12", "element_local_axial_force", "pipe:P-2", 100, "N", { metadata: station("end_j") }),
    row("s:13", "element_local_axial_force", "pipe:P-2", 50, "kN", { basis_ref: L2, metadata: station("end_j") }),
    row("s:14", "element_local_normal_stress_extreme", "pipe:P-1", 5, "MPa", { basis_ref: { ref_type: "load_case", ref_id: "load:L-9" }, metadata: station("end_i") }),
    row("s:15", "pipe_lame_hoop_stress_v2", "pipe:P-1", 12, "MPa", { basis_ref: C1, metadata: station("end_i") }),
    row("ib:1", "component_equal_factor_intensified_bending_stress_v1", "component:C-1", 7e6, "Pa", { source_result_refs: ["s:1", "missing", "s:dup"] }),
    row("s:dup", "element_local_bending_normal_stress_y", "pipe:P-2", 1, "MPa"),
    row("s:dup", "element_local_bending_normal_stress_y", "pipe:P-3", 2, "MPa"),
    row("body:1", "element_local_axial_normal_stress", "pipe:P-3", 2, "MPa", { component_body: { component_ref: "component:K-1", stiffness_scale: 1000 } as Json }),
    row("body:2", "element_local_axial_normal_stress", "pipe:P-3", 3, "MPa", { component_body: { component_ref: "component:K-2", stiffness_scale: 10 } as Json }),
    row("odd:frames:1", "element_local_axial_normal_stress", "pipe:P-1", 1, "MPa", { metadata: { coordinate_system: "Zeta frame" } as Row["metadata"] }),
    row("odd:frames:2", "element_local_axial_normal_stress", "pipe:P-1", 1, "MPa", { metadata: { coordinate_system: "alpha_frame" } as Row["metadata"] }),
    row("odd:frames:3", "element_local_axial_normal_stress", "pipe:P-1", 1, "MPa", { metadata: { coordinate_system: "\u{feff} " } as Row["metadata"] }),
    row("odd:frames:4", "element_local_axial_normal_stress", "pipe:P-1", 1, "MPa", { metadata: { coordinate_system: 3 } as unknown as Row["metadata"] }),
    row("\u{1F600}:id", "some_unknown_kind_v9", "pipe:\u{FFFD}", 1, "\u{1F600}", { basis_ref: { ref_type: "other_basis", ref_id: "x" } })
  ];
}

function stressModel(): PreviewModel {
  const model = overlayModel() as PreviewModel & Record<string, unknown>;
  model.combinations = [{ id: "comb:C1", label: "Combined" }, { id: "comb:R", label: "Range" }] as unknown as PreviewModel["combinations"];
  return model;
}

function invented(): SummaryCase[] {
  const overlay = overlayResult(standardRows());
  const supports = overlayResult(supportRows(), ATTRIBUTION);
  const stresses = overlayResult(stressRows());
  const notSolved = { ...overlayResult(standardRows()), status: { mechanics: "MECHANICS_BLOCKED", rule_check: "not_run", professional_acceptance: "not_claimed" } };
  const combinations = {
    ...overlayResult(standardRows(), {
      exact_combinations: [{ combination_id: "comb:C1", basis: "x", gate: { state: "open" }, operands: [{ load_case_id: "load:L-1" }], rows: [{ result_id: "s:P-1:i", sources: [] }], recoveries: [{ value: 1 }, { value: 2 }], missing_sources: [], withheld: [] }, "not a record"],
      pressure: [{ load_case_id: "load:L-1" }]
    }),
    custom_member: { kept: "out" }
  };
  const empty = { ...overlayModel(), nodes: [], supports: [] } as PreviewModel;
  const c = (name: string, result: unknown, model: unknown, standing: CaseStanding): SummaryCase => ({ name: `invented/${name}`, result: inline(result), model: inline(model), standing });
  return [
    c("overlay_standard", overlay, overlayModel(), { eligible: true, health: health(PASSED) }),
    c("overlay_standard_l2_sensitive", overlay, overlayModel(), { eligible: true, health: health({ "load:L-1": "checks_passed", "load:L-2": "sensitive" }) }),
    c("overlay_standard_not_eligible", overlay, overlayModel(), { eligible: false, health: health(PASSED) }),
    c("overlay_standard_pressure_source", overlay, overlayModel(), { eligible: true, health: health(PASSED, { route: "pressure_source" }) }),
    c("overlay_standard_retained_route", overlay, overlayModel(), { eligible: true, health: health(PASSED, { route: "retained" }) }),
    c("overlay_standard_malformed_health", overlay, overlayModel(), { eligible: true, health: { ...health(PASSED), cases: [{ case_id: "" }] } }),
    c("overlay_standard_no_standing", overlay, overlayModel(), null),
    c("key_rows", overlayResult(keyRows()), keyModel({ rulePackRefs: 2 }), { eligible: true, health: health(PASSED) }),
    c("key_rows_one_pack", overlayResult(keyRows()), keyModel({ rulePackRefs: 1 }), { eligible: true, health: health({ "load:L-1": "checks_passed", "load:L-2": "quality_unresolved" }) }),
    c("key_rows_no_pack", overlayResult(keyRows()), keyModel(), { eligible: true, health: health(PASSED, { combinations: { "comb:R": "checks_passed" } }) }),
    c("supports", supports, supportModel(), { eligible: true, health: health(PASSED, { combinations: { "comb:C1": "checks_passed", "comb:R": "checks_passed" } }) }),
    c("supports_l2_failed", supports, supportModel(), { eligible: true, health: health({ "load:L-1": "checks_passed", "load:L-2": "solve_failed" }, { combinations: { "comb:C1": "sensitive", "comb:R": null }, withheld: ["comb:R"] }) }),
    c("supports_not_eligible", supports, supportModel(), { eligible: false, health: health(PASSED) }),
    c("supports_no_standing", supports, supportModel(), null),
    c("stresses", stresses, stressModel(), { eligible: true, health: health(PASSED, { combinations: { "comb:C1": "checks_passed", "comb:R": "checks_passed" } }) }),
    c("stresses_l2_sensitive", stresses, stressModel(), { eligible: true, health: health({ "load:L-1": "checks_passed", "load:L-2": "sensitive" }) }),
    c("not_solved", notSolved, overlayModel(), { eligible: true, health: health(PASSED) }),
    c("empty_model", overlay, empty, { eligible: true, health: health(PASSED) }),
    c("combination_evidence", combinations, overlayModel(), { eligible: true, health: health(PASSED) })
  ];
}

/** Every case, in the committed order. */
export function resultSummaryCases(): SummaryCase[] {
  const files = FILE_CASES.flatMap(([name, result, model, standings]) =>
    standings.map((standing): SummaryCase => ({ name: standing === "eligible" ? name : `${name}#${standing}`, result, model, standing: STANDINGS[standing] })));
  return [...files, ...invented()];
}

/** The cases file's text, as committed. */
export function resultSummaryCasesText(): string {
  return `${JSON.stringify({
    document_kind: "swbpipe.test.result_summary_cases",
    note: "Written by resultSummaryParity.test.ts (RESULT_SUMMARY_CASES_WRITE=1) from test-support/resultSummaryCases.ts; read by result_summary_tests.rs.",
    cases: resultSummaryCases()
  }, null, 2)}\n`;
}
