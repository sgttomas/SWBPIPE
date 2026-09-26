import { physicsSourceReceiptShape, physicsSourceStanding } from "./physicsSourceRecovery";
import { SOURCE_BLOCKS_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_SHA256, sourceBlockReceiptShape, sourceBlockStanding } from "./sourceBlockRecovery";
import { validatePhysicsEvidence } from "./physicsResultEvidence";
import { validatePreviewPhysicsEvidence } from "./previewPhysicsEvidence";
import type { MechanicsResult, PreviewModel } from "../../types";
export const PRECISION_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/precision-1";
export const PRECISION_CONTRACT_SHA256 = "d75aacee175e178dbdeb256d89a65f4b375265f7da077725ee635af33df51d7e";
export const PHYSICS_SOURCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/physics-source-1";
export const PHYSICS_SOURCE_CONTRACT_SHA256 = "ba13f2aefd7a38bd725e5f111e6ec30144bc8776aa957c6278ee7b1178298ba1";
export const PHYSICS_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/physics-1";
export const PHYSICS_CONTRACT_SHA256 = "9a2cf6268b57bd5265a1a115497c07450819dd4d03cd5ab618097bd9d19da8cc";
export const PREVIEW_PHYSICS_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-1";
export const PREVIEW_PHYSICS_CONTRACT_SHA256 = "ae55503d44a4750714a35c423623e38cf4132099134097193024d1635bfbc88a";
/** T1 load/reference-state identities (0.4.0 exact route only). Members of the
 * static fresh set; the desktop readers for them arrive with T1 WP2. */
export const LOAD_REFERENCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/load-reference-1";
export const LOAD_REFERENCE_SOURCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/load-reference-source-1";
export type SourceContract = "legacy" | "precision" | "physics" | "source_blocks" | "physics_source" | "preview_physics" | "unsupported";
export function sourceSemanticBinding(source: MechanicsResult) {
  const route = sourceContract(source);
  if (route === "physics_source") return { id: PHYSICS_SOURCE_CONTRACT_ID, sha256: PHYSICS_SOURCE_CONTRACT_SHA256 };
  if (route === "source_blocks") return { id: SOURCE_BLOCKS_CONTRACT_ID, sha256: SOURCE_BLOCKS_CONTRACT_SHA256 };
  if (route === "physics") return { id: PHYSICS_CONTRACT_ID, sha256: PHYSICS_CONTRACT_SHA256 };
  if (route === "preview_physics") return { id: PREVIEW_PHYSICS_CONTRACT_ID, sha256: PREVIEW_PHYSICS_CONTRACT_SHA256 };
  if (route === "precision") return { id: PRECISION_CONTRACT_ID, sha256: PRECISION_CONTRACT_SHA256 };
  throw new Error("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
}
/** Exact known (readable) contract binding. This does not authenticate a
 * producer, mint numerical eligibility or admit a historical identity as fresh
 * (see `isFreshSemanticResult`). Each supported method retains its separate
 * evidence requirements. */
export function currentSemanticContract(source: MechanicsResult | null | undefined): { id: string; sha256: string } | null {
  if (!source) return null;
  try { return sourceSemanticBinding(source); } catch { return null; }
}
export function hasCurrentSourceContract(source: MechanicsResult | null | undefined): boolean {
  return currentSemanticContract(source) !== null;
}

function keys(value: unknown, expected: string[]): boolean {
  return !!value && typeof value === "object" && !Array.isArray(value) && Object.keys(value).length === expected.length && expected.every(k => Object.hasOwn(value, k));
}
const statuses = ["not_assessed", "checks_passed", "sensitive", "unresolved", "failed"];
/** Dispatch is explicit. A header never authenticates its claimed producer. */
export function sourceContract(source: MechanicsResult): SourceContract {
  if (Object.hasOwn(source, "carrier_evidence")) return "unsupported";
  if (source.schema_version === "0.1.0") return ["producer", "numerical_quality", "formulation_basis", "contract_evidence", "source_block_recovery"].some(key => Object.hasOwn(source, key)) ? "unsupported" : "legacy";
  const p = source.producer, q = source.numerical_quality, f = source.formulation_basis;
  const blocks = p?.semantic_contract_id === SOURCE_BLOCKS_CONTRACT_ID;
  const composite = p?.semantic_contract_id === PHYSICS_SOURCE_CONTRACT_ID;
  const preview = p?.semantic_contract_id === PREVIEW_PHYSICS_CONTRACT_ID;
  if (composite ? !physicsSourceReceiptShape(source.source_block_recovery) : blocks ? !sourceBlockReceiptShape(source.source_block_recovery) : Object.hasOwn(source, "source_block_recovery")) return "unsupported";
  return source.schema_version === "0.2.0"
    && keys(p, ["component_name", "component_version", "semantic_contract_id"])
    && keys(q, ["value_representation", "publication_quantization", "integrity_policy", "status", "cases"])
    && keys(f, ["profile_id", "limitations"])
    && p?.component_name === "open_pipe_stress_product_physics"
    && p.component_version === "0.2.0" && [PRECISION_CONTRACT_ID, PHYSICS_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID].includes(p.semantic_contract_id)
    && q?.value_representation === "finite_binary64" && q.publication_quantization === "none"
    && q.integrity_policy === "M03-INTEGRITY-v1" && Array.isArray(q.cases)
    && statuses.includes(q.status)
    && q.cases.every(c => keys(c, ["basis_ref", "structural_status", "solve_quality", "model_matrix_fidelity", "accuracy_evidence", "evidence_refs"])
      && keys(c.basis_ref, ["ref_type", "ref_id"]) && typeof c.basis_ref.ref_type === "string" && !!c.basis_ref.ref_type && typeof c.basis_ref.ref_id === "string" && !!c.basis_ref.ref_id
      && ["passive_model_basis", "physical_mechanism_witnessed", "negative_energy_witnessed", "numerically_unresolved"].includes(c.structural_status)
      && statuses.includes(c.solve_quality) && ["represented_equations_retained", "assembly_loss_detected", "assembly_uncertainty", "not_assessed"].includes(c.model_matrix_fidelity)
      && ["not_claimed", "reference_verified", "unresolved"].includes(c.accuracy_evidence)
      && Array.isArray(c.evidence_refs) && c.evidence_refs.every(r => typeof r === "string" && !!r))
    && f !== undefined && ([PHYSICS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID].includes(p.semantic_contract_id) ? f?.profile_id === "exact_straight_pressure_v2" : preview ? f?.profile_id === "product_preview_mechanics_v1" && !!source.contract_evidence && typeof source.contract_evidence === "object" && !Array.isArray(source.contract_evidence) : f?.profile_id === "product_preview_mechanics_v1" && source.contract_evidence == null) && Array.isArray(f.limitations)
    && f.limitations.length > 0 && f.limitations.every(x => typeof x === "string" && x.length > 0)
    ? (composite ? "physics_source" : blocks ? "source_blocks" : preview ? "preview_physics" : p.semantic_contract_id === PHYSICS_CONTRACT_ID ? "physics" : "precision") : "unsupported";
}
export function numericalResultStanding(source: MechanicsResult, model?: (Pick<PreviewModel, "load_cases"> & Partial<Pick<PreviewModel, "pipe_segments" | "supports">>) | null) {
  const contract = sourceContract(source);
  const findings: string[] = [];
  if (contract === "legacy") findings.push("LEGACY_ABSOLUTE_ROUNDING_INTEGRITY_NOT_ASSESSED");
  else if (contract === "unsupported") findings.push("SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED");
  else if (contract === "physics_source") findings.push(...physicsSourceStanding(source, model).findings);
  else if (contract === "source_blocks") findings.push(...sourceBlockStanding(source, model).findings);
  else {
    const q = source.numerical_quality!;
    if (contract === "physics") {
      try { validatePhysicsEvidence(source, model ?? undefined); }
      catch (error) { findings.push(error instanceof Error ? error.message : "PHYSICS_EVIDENCE_INVALID"); }
    }
    if (contract === "preview_physics") {
      try { validatePreviewPhysicsEvidence(source, model ?? undefined); }
      catch (error) { findings.push(error instanceof Error ? error.message : "SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID"); }
    }
    // T0R: precision-1 stays readable and verifiable, but is historical only and
    // never Current, rule-, report- or Current-export-eligible.
    // A2 10: dispatch above already validated the precision-1 header (a malformed or
    // tampered carrier is "unsupported" and never reaches this reason).
    if (contract === "precision") findings.push("PRECISION_1_HISTORICAL_SEMANTICS");
    // Sensitive backward-error evidence does not establish source-answer accuracy.
    // Preserve its raw contract for inspection while withholding qualified use.
    if (q.status !== "checks_passed") findings.push("NUMERICAL_INTEGRITY_NOT_QUALIFIED");
    // The product solves every supplied load case. Result rows cannot prove request completeness.
    const requested = model?.load_cases.map(c => c.id) ?? [];
    const emittedIds = [...source.results.map(r => r.id), ...source.diagnostics.flatMap(d => d.id ? [d.id] : [])];
    const emitted = new Set(emittedIds);
    if (source.results.some(r => typeof r.id !== "string" || !r.id) || source.diagnostics.some(d => typeof d.id !== "string" || !d.id) || emitted.size !== emittedIds.length) findings.push("NUMERICAL_EVIDENCE_ID_AMBIGUOUS");
    if (!requested.length || new Set(requested).size !== requested.length) findings.push("REQUESTED_NUMERICAL_BASIS_UNAVAILABLE");
    const cases = q.cases;
    if (cases.length !== requested.length || requested.some(id => cases.filter(c => c?.basis_ref?.ref_type === "load_case" && c.basis_ref.ref_id === id).length !== 1)) findings.push("NUMERICAL_CASE_COVERAGE_INCOMPLETE");
    if (cases.some(c => !c || c.structural_status !== "passive_model_basis" || c.solve_quality !== "checks_passed"
      || c.model_matrix_fidelity !== "represented_equations_retained" || !["not_claimed", "reference_verified"].includes(c.accuracy_evidence)
      || !Array.isArray(c.evidence_refs) || !c.evidence_refs.length || c.evidence_refs.some(id => !emitted.has(id)))) findings.push("NUMERICAL_CASE_EVIDENCE_INCOMPLETE");
    const aggregateOrder = ["checks_passed", "sensitive", "not_assessed", "unresolved", "failed"];
    const aggregate = cases.length
      ? aggregateOrder[cases.reduce((worst, c) => Math.max(worst, aggregateOrder.indexOf(c.solve_quality)), 0)]
      : "not_assessed";
    if (q.status !== aggregate) findings.push("NUMERICAL_AGGREGATE_CONTRADICTION");
  }
  if (source.status.mechanics !== "MECHANICS_SOLVED") findings.push("MECHANICS_NOT_SOLVED");
  return { contract, status: findings.length ? "needs_recompute" as const : "integrity_checked" as const, eligible: findings.length === 0, findings };
}
