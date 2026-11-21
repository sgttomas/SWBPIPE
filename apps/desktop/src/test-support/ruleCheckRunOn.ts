// An invented rule-check run on a result's own rows, for tests of the displays that read a run
// (the stress and ratio table, the key results, the viewport's rule ratios). Invented data only.
import type { SessionRuleCheckRun } from "../features/report/renderableReportInput";
import { STRESS_KINDS } from "../features/results/stressRatioTable";
import type { RuleCheckOutcome } from "../services/ruleCheckService";
import type { MechanicsResult } from "../types";

/** An invented rule-check run on `source`'s own rows: a check bound to each of up to three emitted
 * stress rows, or of any finite rows on a load case or combination where it emits none (alternating
 * two categories, one with no ratio), a selector-only input left unbound,
 * and, with `foreign`, an outcome bound to an id that is not a row of this result. */
export function ruleCheckRunOn(source: MechanicsResult, foreign: string | null = null): SessionRuleCheckRun {
  const stressKinds = new Set(STRESS_KINDS.map((item) => item.kind));
  const bindable = (row: MechanicsResult["results"][number]) => Number.isFinite(row.value) && (row.basis_ref?.ref_type === "load_case" || row.basis_ref?.ref_type === "combination");
  // Emitted stress rows first; a result without one binds any finite row on a load case or combination.
  const stresses = source.results.filter((row) => stressKinds.has(row.kind) && bindable(row));
  const chosen = (stresses.length > 0 ? stresses : source.results.filter(bindable)).slice(0, 3);
  const location = (row: MechanicsResult["results"][number]) => (row.metadata as Record<string, unknown> | undefined)?.location ?? undefined;
  const solver = (id: string, kind: string, entity: string, at: unknown) => ({
    input_id: id, name: id, source_kind: "solver_result", quantity_intent: { dimension: "stress", unit_ref: "MPa" },
    solver_result_ref: { result_kind: kind, entity_ref: entity, ...(at === undefined ? {} : { location: at }) }
  });
  const check = (id: string, category: string, inputs: string[]) => ({
    check_id: id, category_ref: { ref_type: "stress_category", ref_id: category },
    required_input_refs: inputs.map((ref) => ({ ref_type: "required_input", ref_id: ref }))
  });
  const bases = [...new Map(chosen.map((row) => [`${row.basis_ref!.ref_type}:${row.basis_ref!.ref_id}`, row.basis_ref!] as const)).values()];
  const basis = (bases[0] ?? { ref_type: "load_case", ref_id: "invented" }) as RuleCheckOutcome["basis_ref"];
  const outcome = (id: string, category: string, extra: Partial<RuleCheckOutcome>): RuleCheckOutcome => ({
    check_id: id, category_ref: category, basis_ref: basis, status: "USER_RULE_CHECKED", acceptability_relation: "less_than_or_equal",
    bound_inputs: [], completeness_findings: [], evaluator_findings: [], diagnostic_codes: [], ...extra
  });
  const checks: RuleCheckOutcome[] = chosen.map((row, i) => outcome(`chk_${i}`, i % 2 === 0 ? "cat_a" : "cat_b", {
    basis_ref: row.basis_ref as RuleCheckOutcome["basis_ref"],
    ...(i === 1 ? {} : { computed_value: { value: row.value, unit_ref: row.unit ?? "MPa", dimension: "stress" }, limit_value: { value: 2 * Math.abs(row.value) + 1, unit_ref: row.unit ?? "MPa", dimension: "stress" } }),
    bound_inputs: [{ input_id: `in_${i}`, source_kind: "solver_result", supplied: true, value: row.value, unit: row.unit ?? "MPa", result_id: row.id }]
  }));
  checks.push(outcome("chk_sel", "cat_a", { status: "RULE_INPUTS_INCOMPLETE", acceptability_relation: "none", bound_inputs: [{ input_id: "in_sel", source_kind: "solver_result", supplied: false }] }));
  if (foreign) checks.push(outcome("chk_foreign", "cat_b", { bound_inputs: [{ input_id: "in_foreign", source_kind: "solver_result", supplied: true, value: 1, unit: "MPa", result_id: foreign }] }));
  return {
    rulePackDocument: {
      required_inputs: [
        ...chosen.map((row, i) => solver(`in_${i}`, row.kind, row.entity_ref, location(row))),
        solver("in_sel", "element_local_normal_stress_extreme", "pipe:invented", "end_i"),
        ...(foreign ? [solver("in_foreign", "element_local_normal_stress_extreme", "pipe:invented", "end_j")] : []),
        { input_id: "limit_value", name: "limit", source_kind: "user_supplied_rule_value", quantity_intent: { dimension: "stress", unit_ref: "MPa" } }
      ],
      stress_categories: [{ category_id: "cat_b", name: "Invented B" }, { category_id: "cat_a", name: "Invented A" }],
      required_load_cases: bases.map((item, i) => ({ required_load_case_id: `rl${i}`, name: item.ref_id, basis_ref: item })),
      load_case_mappings: [],
      check_definitions: [
        ...chosen.map((_, i) => check(`chk_${i}`, i % 2 === 0 ? "cat_a" : "cat_b", [`in_${i}`, "limit_value"])),
        check("chk_sel", "cat_a", ["in_sel", "limit_value"]),
        ...(foreign ? [check("chk_foreign", "cat_b", ["in_foreign", "limit_value"])] : [])
      ]
    },
    result: {
      document_kind: "rule_check_run_result", rule_pack_id: "invented_pack", grammar_version: "1.0.0",
      aggregate_status: "RULE_INPUTS_INCOMPLETE", checks, professional_boundary_notice: "invented"
    }
  };
}
