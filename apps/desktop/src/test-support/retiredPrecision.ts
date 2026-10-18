/** B1 (b7-plan): precision-1 is retired. A stored precision-1 statement for tests:
 * the connected preview-physics-2 producer output relabelled precision-1, its
 * preview evidence namespace removed. The Rust reader refuses it by name
 * (`SOURCE_PRECISION_1_RETIRED`), so the desktop reads it as unsupported. */
import connectedSparse from "../../../../fixtures/results/preview_physics_connected_sparse.json";
import type { MechanicsResult } from "../types";

export const RETIRED_PRECISION_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/precision-1";
export const SOURCE_PRECISION_1_RETIRED = "SOURCE_PRECISION_1_RETIRED";

export function retiredPrecisionStatement(): MechanicsResult {
  const source = structuredClone(connectedSparse) as unknown as MechanicsResult;
  source.producer = { ...source.producer!, semantic_contract_id: RETIRED_PRECISION_CONTRACT_ID };
  delete (source as { contract_evidence?: unknown }).contract_evidence;
  return source;
}

import previewTable from "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_2.json";

/** The readable statement the precision-1 synthetic header used to provide for
 * transport tests: a solved preview-physics-2 statement with invented rows, the
 * table's profile limitations, one empty complete preview case per numerical
 * case and no headline (no displacement-magnitude rows). The Rust preview reader
 * admits it; it makes no solver, physics or accuracy claim. */
export function minimalPreviewStatement(args: {
  run_id: string; model_ref: string; rows: MechanicsResult["results"];
  cases: NonNullable<MechanicsResult["numerical_quality"]>["cases"]; diagnostics?: MechanicsResult["diagnostics"];
}): MechanicsResult {
  const { cases } = args;
  return {
    schema_version: "0.2.0", document_kind: "openpipestress.product_preview.mechanics_result", run_id: args.run_id, model_ref: args.model_ref,
    producer: { component_name: "open_pipe_stress_product_physics", component_version: "0.2.0", semantic_contract_id: "openpipestress.result_semantics/0.3.0/preview-physics-2" },
    numerical_quality: { value_representation: "finite_binary64", publication_quantization: "none", integrity_policy: "M03-INTEGRITY-v1",
      status: cases.length && cases.every(c => c.solve_quality === "checks_passed") ? "checks_passed" : "not_assessed", cases },
    formulation_basis: { profile_id: "product_preview_mechanics_v1", limitations: [...previewTable.supported_profile_limitations] },
    contract_evidence: {
      preview_cases: cases.map(c => ({ load_case_id: c.basis_ref.ref_id, pipe_stress_extrema: [], stress_maximum_coverage: { complete: true, unavailable_pipe_ids: [], outside_domain_pipe_ids: [] }, support_attribution: { attributed_support_ids: [], withheld: [] }, intensified_measures: [] })),
      combination_gates: [],
    },
    status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE", professional_acceptance: "NOT_PROVIDED" },
    summary: { max_displacement: null, max_open_formula_stress: null },
    results: args.rows, diagnostics: args.diagnostics ?? [],
  } as unknown as MechanicsResult;
}

/** One empty, complete preview case per numerical case, in its order: the preview
 * reader's case coverage for a synthetic statement whose cases a test edits. */
export function syncPreviewCases(result: MechanicsResult): MechanicsResult {
  (result as unknown as { contract_evidence: { preview_cases: unknown[]; combination_gates: unknown[] } }).contract_evidence = {
    preview_cases: (result.numerical_quality?.cases ?? []).map(c => ({ load_case_id: c.basis_ref.ref_id, pipe_stress_extrema: [], stress_maximum_coverage: { complete: true, unavailable_pipe_ids: [], outside_domain_pipe_ids: [] }, support_attribution: { attributed_support_ids: [], withheld: [] }, intensified_measures: [] })),
    combination_gates: [],
  };
  return result;
}
