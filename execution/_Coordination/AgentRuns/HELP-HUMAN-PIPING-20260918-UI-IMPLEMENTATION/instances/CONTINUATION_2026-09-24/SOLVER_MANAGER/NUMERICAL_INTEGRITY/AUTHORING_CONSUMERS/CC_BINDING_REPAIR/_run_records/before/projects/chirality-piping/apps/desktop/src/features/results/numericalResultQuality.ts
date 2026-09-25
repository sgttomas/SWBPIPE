import type { MechanicsResult, PreviewModel } from "../../types";
export const PRECISION_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/precision-1";
export const PRECISION_CONTRACT_SHA256 = "d75aacee175e178dbdeb256d89a65f4b375265f7da077725ee635af33df51d7e";
export type SourceContract = "legacy" | "precision" | "unsupported";
function keys(value: unknown, expected: string[]): boolean {
  return !!value && typeof value === "object" && !Array.isArray(value) && Object.keys(value).length === expected.length && expected.every(k => Object.hasOwn(value, k));
}
const statuses = ["not_assessed", "checks_passed", "sensitive", "unresolved", "failed"];
/** Dispatch is explicit. A header never authenticates its claimed producer. */
export function sourceContract(source: MechanicsResult): SourceContract {
  if (source.schema_version === "0.1.0") return ["producer", "numerical_quality", "formulation_basis"].some(key => Object.hasOwn(source, key)) ? "unsupported" : "legacy";
  const p = source.producer, q = source.numerical_quality, f = source.formulation_basis;
  return source.schema_version === "0.2.0"
    && keys(p, ["component_name", "component_version", "semantic_contract_id"])
    && keys(q, ["value_representation", "publication_quantization", "integrity_policy", "status", "cases"])
    && keys(f, ["profile_id", "limitations"])
    && p?.component_name === "open_pipe_stress_product_physics"
    && p.component_version === "0.2.0" && p.semantic_contract_id === PRECISION_CONTRACT_ID
    && q?.value_representation === "finite_binary64" && q.publication_quantization === "none"
    && q.integrity_policy === "M03-INTEGRITY-v1" && Array.isArray(q.cases)
    && statuses.includes(q.status)
    && q.cases.every(c => keys(c, ["basis_ref", "structural_status", "solve_quality", "model_matrix_fidelity", "accuracy_evidence", "evidence_refs"])
      && keys(c.basis_ref, ["ref_type", "ref_id"]) && ["load_case", "combination"].includes(c.basis_ref.ref_type) && typeof c.basis_ref.ref_id === "string" && !!c.basis_ref.ref_id
      && ["passive_model_basis", "physical_mechanism_witnessed", "negative_energy_witnessed", "numerically_unresolved"].includes(c.structural_status)
      && statuses.includes(c.solve_quality) && ["represented_equations_retained", "assembly_loss_detected", "assembly_uncertainty", "not_assessed"].includes(c.model_matrix_fidelity)
      && ["not_claimed", "reference_verified", "unresolved"].includes(c.accuracy_evidence)
      && Array.isArray(c.evidence_refs) && c.evidence_refs.every(r => typeof r === "string" && !!r))
    && f?.profile_id === "product_preview_mechanics_v1" && Array.isArray(f.limitations)
    && f.limitations.length > 0 && f.limitations.every(x => typeof x === "string" && x.length > 0)
    ? "precision" : "unsupported";
}
export function numericalResultStanding(source: MechanicsResult, model?: Pick<PreviewModel, "load_cases"> | null) {
  const contract = sourceContract(source);
  const findings: string[] = [];
  if (contract === "legacy") findings.push("LEGACY_ABSOLUTE_ROUNDING_INTEGRITY_NOT_ASSESSED");
  else if (contract === "unsupported") findings.push("SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED");
  else {
    const q = source.numerical_quality!;
    if (!["checks_passed", "sensitive"].includes(q.status)) findings.push("NUMERICAL_INTEGRITY_NOT_QUALIFIED");
    // The product solves every supplied load case. Result rows cannot prove request completeness.
    const requested = model?.load_cases.map(c => c.id) ?? [];
    const emittedIds = [...source.results.map(r => r.id), ...source.diagnostics.flatMap(d => d.id ? [d.id] : [])];
    const emitted = new Set(emittedIds);
    if (source.results.some(r => typeof r.id !== "string" || !r.id) || source.diagnostics.some(d => typeof d.id !== "string" || !d.id) || emitted.size !== emittedIds.length) findings.push("NUMERICAL_EVIDENCE_ID_AMBIGUOUS");
    if (!requested.length || new Set(requested).size !== requested.length) findings.push("REQUESTED_NUMERICAL_BASIS_UNAVAILABLE");
    const cases = q.cases;
    if (cases.length !== requested.length || requested.some(id => cases.filter(c => c?.basis_ref?.ref_type === "load_case" && c.basis_ref.ref_id === id).length !== 1)) findings.push("NUMERICAL_CASE_COVERAGE_INCOMPLETE");
    if (cases.some(c => !c || c.structural_status !== "passive_model_basis" || !["checks_passed", "sensitive"].includes(c.solve_quality)
      || c.model_matrix_fidelity !== "represented_equations_retained" || !["not_claimed", "reference_verified"].includes(c.accuracy_evidence)
      || !Array.isArray(c.evidence_refs) || !c.evidence_refs.length || c.evidence_refs.some(id => !emitted.has(id)))) findings.push("NUMERICAL_CASE_EVIDENCE_INCOMPLETE");
    if (cases.length && q.status !== (cases.some(c => c?.solve_quality === "sensitive") ? "sensitive" : "checks_passed")) findings.push("NUMERICAL_AGGREGATE_CONTRADICTION");
  }
  if (source.status.mechanics !== "MECHANICS_SOLVED") findings.push("MECHANICS_NOT_SOLVED");
  return { contract, status: findings.length ? "needs_recompute" as const : "integrity_checked" as const, eligible: findings.length === 0, findings };
}
