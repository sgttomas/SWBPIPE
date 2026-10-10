/** T4-U2a: reader dispatch on the exact pressure contract identity.
 *
 * TypeScript peer of `validate_exact_pressure_evidence` in
 * `core/analysis_runs/physics_evidence.py` and of the `pressure-1` arm of
 * `core/reporting/result_export/src/semantic_contract.rs`. physics-1 and
 * load-reference-1 carry `2.0.0/exact_straight_pressure_v2` (unchanged);
 * pressure-1 carries `3.0.0/exact_pressure_v3` on 0.3.0 and 0.4.0 documents
 * (straight families only, signed p_pa). An unknown identity is refused. */
import { validatePhysicsEvidence, validatePhysicsEvidenceFor, PRESSURE_V3 } from "./physicsResultEvidence";
import { LOAD_REFERENCE_CONTRACT_ID, validateLoadReferenceEvidence, validatePressureLoadReferenceEvidence } from "./loadReferenceEvidence";
import type { MechanicsResult, PreviewModel } from "../../types";

export const PHYSICS_EVIDENCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/physics-1";
export const PRESSURE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/pressure-1";
export const PRESSURE_PROFILE = "exact_pressure_v3";
type PhysicsModel = Pick<PreviewModel, "load_cases"> & Partial<Pick<PreviewModel, "pipe_segments" | "supports">>;

/** Raw pressure-1 publication: a 0.4.0 envelope (with `load_reference_states`)
 * takes the load/reference pre-pass first; a 0.3.0 envelope the physics checks. */
export function validatePressureEvidence(source: MechanicsResult, model?: PhysicsModel): void {
  const evidence = source.contract_evidence as Record<string, unknown> | undefined;
  if (evidence && typeof evidence === "object" && Object.hasOwn(evidence, "load_reference_states")) validatePressureLoadReferenceEvidence(source, model);
  else validatePhysicsEvidenceFor(source, PRESSURE_V3, model);
}

export function validateExactPressureEvidence(source: MechanicsResult, model?: PhysicsModel): void {
  const identity = source.producer?.semantic_contract_id;
  if (identity === PHYSICS_EVIDENCE_CONTRACT_ID) validatePhysicsEvidence(source, model);
  else if (identity === LOAD_REFERENCE_CONTRACT_ID) validateLoadReferenceEvidence(source, model);
  else if (identity === PRESSURE_CONTRACT_ID) {
    if (source.formulation_basis?.profile_id !== PRESSURE_PROFILE) throw new Error("PHYSICS_EVIDENCE_PRESSURE_FORMULATION_PROFILE");
    validatePressureEvidence(source, model);
  } else throw new Error(`PHYSICS_EVIDENCE_PRESSURE_CONTRACT_UNKNOWN: ${String(identity)}`);
}
