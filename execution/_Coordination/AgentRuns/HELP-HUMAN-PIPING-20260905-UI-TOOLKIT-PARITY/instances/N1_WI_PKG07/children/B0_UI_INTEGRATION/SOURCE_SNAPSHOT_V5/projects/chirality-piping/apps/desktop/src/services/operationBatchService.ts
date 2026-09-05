import { invoke } from "@tauri-apps/api/core";
import type { EditorOperationIntent, ModelHashEvidence, PreviewModel, OperationOutcomeDiagnostic, OperationDiffPreviewRow, OperationAcceptanceRecord } from "../types";
import { loadWasmEngine } from "./wasmEngine/loadWasmEngine";
export type OperationBatch = {
  batch_id: string;
  operations: EditorOperationIntent[];
};
export type OperationBatchStep = {
  index: number;
  operation_id: string;
  change_id: string;
  target_object_type: string;
  target_ref: string;
  change_kind: string;
  validation: {
    schema_validation: string;
    reference_validation: string;
    unit_validation: string;
    before_state_validation: string;
    diff_preview_status: string;
  };
  diff_preview: OperationDiffPreviewRow[];
  diagnostics: OperationOutcomeDiagnostic[];
  simulation_status: "validated_on_temporary_state" | "blocked";
};
export type OperationBatchOutcome = {
  schema_version: string;
  document_kind: "openpipestress.desktop.operation_batch_outcome";
  batch_id: string;
  mode: "validate_only" | "apply";
  application_route: "structured_operation_batch" | "local_wasm_engine";
  input_model_unchanged: true;
  validation: {
    schema_validation: "passed" | "blocked";
    batch_validation_status: "passed" | "blocked";
    diff_preview_status: "generated" | "blocked_by_validation";
    application_status: "applied_to_session_model" | "blocked" | "not_applied";
  };
  simulation_disposition: "rolled_back_no_model_published" | "committed_as_one_batch" | "validation_only_discarded";
  diagnostics: OperationOutcomeDiagnostic[];
  operation_outcomes: OperationBatchStep[];
  initial_model_hash: ModelHashEvidence;
  initial_backend_hash: string;
  input_backend_hash: string;
  batch_hash: string;
  submitted_operations: EditorOperationIntent[];
  submitted_operations_trust: "untrusted_submitted_metadata_not_validation_evidence";
  submitted_initial_model_hash: ModelHashEvidence | null;
  applied_model: PreviewModel | null;
  applied_model_backend_hash: string | null;
  acceptance: OperationAcceptanceRecord | null;
  professional_boundary: Record<string, boolean>;
  audit_boundary: Record<string, unknown>;
};
async function run(
  model: PreviewModel,
  batch: OperationBatch,
  claimedModelHash: ModelHashEvidence | null,
  apply: boolean
): Promise<OperationBatchOutcome> {
  if (typeof window !== "undefined" && "__TAURI_INTERNALS__" in window) {
    return invoke(apply ? "apply_model_operation_batch" : "validate_model_operation_batch", {
      model,
      batch,
      claimedModelHash
    });
  }
  const engine = await loadWasmEngine();
  const execute = apply ? engine.applyOperationBatchJson : engine.validateOperationBatchJson;
  const parsed = JSON.parse(execute(
    JSON.stringify(model),
    JSON.stringify(batch),
    JSON.stringify(claimedModelHash)
  ));
  if (parsed.error) {
    throw new Error(`${parsed.error.code}: ${parsed.error.message}`);
  }
  return parsed as OperationBatchOutcome;
}

export function validateOperationBatch(
  model: PreviewModel,
  batch: OperationBatch,
  claimedModelHash: ModelHashEvidence | null
): Promise<OperationBatchOutcome> {
  return run(model, batch, claimedModelHash, false);
}

export function applyOperationBatch(
  model: PreviewModel,
  batch: OperationBatch,
  claimedModelHash: ModelHashEvidence | null
): Promise<OperationBatchOutcome> {
  return run(model, batch, claimedModelHash, true);
}
