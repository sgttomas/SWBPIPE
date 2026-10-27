// Invented fixtures for the key-results tests (A6; DEL-07-05 Design `key-results-panel.md` §7): the
// three-member overlay model with an anchor, an equipment connection and a plain support; stress rows
// at stations for two load cases and a range-envelope combination; nonlinear support-state rows; and
// an invented rule pack whose run has a placed category, a category on two pipes, a category only on
// the range envelope and a declared category with no outcome. No value has engineering meaning.

import type { MechanicsResult, PreviewModel } from "../types";
import type { SessionRuleCheckRun } from "../features/report/renderableReportInput";
import type { RuleCheckOutcome, RuleCheckStatus } from "../services/ruleCheckService";
import { overlayModel, overlayResult, row } from "./resultOverlayFixtures";

type Row = MechanicsResult["results"][number];

export const L1 = { ref_type: "load_case", ref_id: "load:L-1" } as const;
export const L2 = { ref_type: "load_case", ref_id: "load:L-2" } as const;
export const RANGE = { ref_type: "combination", ref_id: "comb:R" } as const;
/** A combination the invented health view states no state for, so the standing withholds it. */
export const COMBINED = { ref_type: "combination", ref_id: "comb:C" } as const;
type Basis = typeof L1 | typeof L2 | typeof RANGE;

export function keyModel(options: { rulePackRefs?: number } = {}): PreviewModel {
  const model = overlayModel() as PreviewModel & Record<string, unknown>;
  model.schema_version = "0.7.0";
  model.document_kind = "pipe_stress_model";
  model.data_boundary = {};
  model.project = { id: "project:invented", name: "Invented", description: "invented", units: { length: "m", force: "N" } };
  model.analysis_status = { mechanics: "not_run", rule_check: "not_run", professional_acceptance: "not_claimed" };
  model.materials = [];
  model.sections = [];
  model.diagnostics = [];
  model.supports = [
    { id: "support:S-A", node: "node:A", label: "Anchor A", family: "anchor" },
    { id: "support:S-C", node: "node:C", label: "Nozzle C", family: "guide",
      boundary_association: { boundary_id: "b:C", kind: "equipment_nozzle", equipment_reference: "EQ-1", nozzle_reference: "N1", coordinate_system: "global" } },
    { id: "support:S-D", node: "node:D", label: "Rest D", family: "vertical_support" }
  ] as PreviewModel["supports"];
  if (options.rulePackRefs) {
    model.rule_pack_refs = Array.from({ length: options.rulePackRefs }, (_, index) => ({ rule_pack_id: `invented_pack_${index}` })) as unknown as PreviewModel["rule_pack_refs"];
  }
  return model;
}

const station = (location: string, basis = "invented") =>
  ({ component: "sigma_plus", coordinate_system: "pipe_section", location, basis, sign_convention: "invented" });
const state = (support: string, basis: Basis, value: number): Row =>
  row(`state:${support}:${basis.ref_id}`, "nonlinear_support_active_set_state_code", support, value, "state_code", {
    basis_ref: basis,
    metadata: { component: "active_set_state_code", coordinate_system: "solver_iteration", location: "uy", basis: "invented", sign_convention: "0=inactive, 1=active, 2=sticking, 3=sliding" }
  });

export function keyRows(): Row[] {
  return [
    row("s:P-1:i", "element_local_normal_stress_extreme", "pipe:P-1", 40, "MPa", { metadata: station("end_i") }),
    row("s:P-2:i", "element_local_normal_stress_extreme", "pipe:P-2", 80, "MPa", { metadata: station("end_i") }),
    row("s:P-1:i:L2", "element_local_normal_stress_extreme", "pipe:P-1", 50, "MPa", { basis_ref: L2, metadata: station("end_i") }),
    row("s:P-1:i:R", "element_local_normal_stress_extreme", "pipe:P-1", 90, "MPa", { basis_ref: RANGE, metadata: station("end_i", "explicit_user_range_envelope") }),
    row("d:A", "displacement_magnitude", "node:A", 0, "mm"),
    row("d:B", "displacement_magnitude", "node:B", 2, "mm"),
    row("d:B:L2", "displacement_magnitude", "node:B", 3, "mm", { basis_ref: L2 }),
    state("support:S-D", L1, 1),
    state("support:S-D", L2, 0)
  ];
}

export function keyResult(rows: Row[] = keyRows()): MechanicsResult {
  return overlayResult(rows);
}

/** The pack: `cat_a` on P-1 (placed), `cat_b` on P-1 and P-2 together (not placed), `cat_r` on the
 * range envelope only, and `cat_none` declared with no check. */
export function keyPack(): SessionRuleCheckRun["rulePackDocument"] {
  const solver = (id: string, entity: string) => ({
    input_id: id, name: id, source_kind: "solver_result",
    quantity_intent: { dimension: "stress", unit_ref: "MPa" },
    solver_result_ref: { result_kind: "element_local_normal_stress_extreme", entity_ref: entity, location: "end_i", component: "sigma_plus" }
  });
  const check = (id: string, category: string, inputs: string[]) => ({
    check_id: id, category_ref: { ref_type: "stress_category", ref_id: category },
    required_input_refs: inputs.map((ref) => ({ ref_type: "required_input", ref_id: ref }))
  });
  return {
    required_inputs: [solver("in_p1", "pipe:P-1"), solver("in_p2", "pipe:P-2"),
      { input_id: "limit_value", name: "limit", source_kind: "user_supplied_rule_value", quantity_intent: { dimension: "stress", unit_ref: "MPa" } }],
    stress_categories: [
      { category_id: "cat_a", name: "Invented A" },
      { category_id: "cat_b", name: "Invented B" },
      { category_id: "cat_r", name: "Invented range" },
      { category_id: "cat_none", name: "Invented unused" }
    ],
    required_load_cases: [],
    load_case_mappings: [],
    check_definitions: [
      check("chk_a", "cat_a", ["in_p1", "limit_value"]),
      check("chk_b", "cat_b", ["in_p1", "in_p2", "limit_value"]),
      check("chk_r", "cat_r", ["in_p1", "limit_value"])
    ]
  } as unknown as SessionRuleCheckRun["rulePackDocument"];
}

const quantity = (value: number) => ({ value, unit_ref: "MPa", dimension: "stress" });

export function keyOutcome(check: string, category: string, basis: Basis, status: RuleCheckStatus, bound: Record<string, string | undefined>): RuleCheckOutcome {
  const incomplete = status === "RULE_INPUTS_INCOMPLETE";
  return {
    check_id: check, category_ref: category, basis_ref: basis, status,
    acceptability_relation: incomplete ? "none" : "less_than_or_equal",
    bound_inputs: Object.entries(bound).map(([input, resultId]) => ({
      input_id: input, source_kind: "solver_result", supplied: resultId !== undefined, ...(resultId ? { value: 1, unit: "MPa", result_id: resultId } : {})
    })),
    completeness_findings: [], evaluator_findings: [], diagnostic_codes: [],
    ...(incomplete ? {} : { computed_value: quantity(40), limit_value: quantity(100) })
  } as RuleCheckOutcome;
}

/** A run of `keyPack`: `cat_a` checked in L-1 and incomplete in L-2, `cat_b` on two pipes, and
 * `cat_r` on the range envelope. */
export function keyRun(checks: RuleCheckOutcome[] = keyOutcomes()): SessionRuleCheckRun {
  return {
    rulePackDocument: keyPack(),
    result: {
      document_kind: "rule_check_run_result", rule_pack_id: "invented_pack", grammar_version: "1.0.0",
      aggregate_status: checks.some((item) => item.status === "RULE_INPUTS_INCOMPLETE") ? "RULE_INPUTS_INCOMPLETE" : "USER_RULE_CHECKED",
      checks, professional_boundary_notice: "invented"
    }
  };
}

export function keyOutcomes(): RuleCheckOutcome[] {
  return [
    keyOutcome("chk_a", "cat_a", L1, "USER_RULE_CHECKED", { in_p1: "s:P-1:i" }),
    keyOutcome("chk_a", "cat_a", L2, "RULE_INPUTS_INCOMPLETE", { in_p1: undefined }),
    keyOutcome("chk_b", "cat_b", L1, "USER_RULE_CHECKED", { in_p1: "s:P-1:i", in_p2: "s:P-2:i" }),
    keyOutcome("chk_r", "cat_r", RANGE, "USER_RULE_CHECKED", { in_p1: "s:P-1:i:R" })
  ];
}
