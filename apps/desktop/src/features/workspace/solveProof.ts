// Solve-proof evidence and its status text. `solveProofStatus` returns text only
// while the model, the model hash, the result and the solve job all match the
// recorded proof, and null otherwise. No React.

import type {
  MechanicsResult,
  ModelHashEvidence,
  PreviewModel,
  SolveJobAuditState
} from "../../types";

export type SolveProofEvidence = {
  state: "completed";
  run_generation: number;
  job_id: string;
  backend_job_seam: SolveJobAuditState["backend_job_seam"];
  project_ref: string;
  model_sha256: string;
  input_manifest_sha256: string;
  result_run_id: string;
  result_model_ref: string;
  result_row_count: number;
};

export function solveProofStatus(
  model: PreviewModel,
  modelHash: ModelHashEvidence | null,
  result: MechanicsResult | null,
  solveJob: SolveJobAuditState,
  proof: SolveProofEvidence | null
): string | null {
  if (
    !modelHash ||
    !result ||
    result.status.mechanics !== "MECHANICS_SOLVED" ||
    !proof ||
    solveJob.state !== "completed" ||
    proof.state !== "completed" ||
    solveJob.job_id !== proof.job_id ||
    solveJob.backend_job_seam !== proof.backend_job_seam ||
    model.project.id !== proof.project_ref ||
    modelHash.value !== proof.model_sha256 ||
    result.run_id !== proof.result_run_id ||
    result.model_ref !== proof.result_model_ref ||
    result.results.length !== proof.result_row_count ||
    result.model_ref !== model.project.id
  ) {
    return null;
  }
  return [
    `seam=${proof.backend_job_seam}`,
    `project=${model.project.id}`,
    `result_model=${result.model_ref}`,
    "identity=match",
    `rows=${result.results.length}`,
    `generation=${proof.run_generation}`,
    `job=${proof.job_id}`,
    `model_sha256=${proof.model_sha256}`,
    `input_manifest_sha256=${proof.input_manifest_sha256}`
  ].join("; ");
}
