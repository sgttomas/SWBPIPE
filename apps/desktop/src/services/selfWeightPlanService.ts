import { invoke } from "@tauri-apps/api/core";
import type { PreviewModel } from "../types";
import { loadSelfWeightEngine } from "./wasmEngine/loadSelfWeightEngine";
export type SelfWeightRequest = {
  case_id: string;
  label: string;
  pipe_refs: string[];
  gravity: {
    value: number;
    unit: string;
    axis: string;
  };
  provenance: string;
  source_model_hash: string;
};
export type SelfWeightOperationDraft = {
  object_type: string;
  target_ref: string;
  operation_kind: string;
  change_kind: string;
  field_label: string;
  field_path: string;
  before: string;
  after: string;
  unit: string;
  dimension: string;
  source_note: string;
};
export type SelfWeightOperationPlan = {
  source_model_hash: string;
  changes: SelfWeightOperationDraft[];
  source_evidence: unknown[];
  scope_label: string;
};
export type SelfWeightPlanOutcome = {
  document_kind: "openpipestress.desktop.self_weight_plan_outcome";
  status: "ready" | "blocked";
  input_model_unchanged: true;
  source_model_hash: string | null;
  plan: SelfWeightOperationPlan | null;
  diagnostics: {
    code: string;
    severity: "blocking";
    message: string;
    remediation: string;
    affected_objects: string[];
  }[];
  mutation_route: "structured_operations_only";
  requires_user_acceptance: true;
};
export async function generateSelfWeightPlan(
  model: PreviewModel,
  request: SelfWeightRequest
): Promise<SelfWeightPlanOutcome> {
  if (typeof window !== "undefined" && "__TAURI_INTERNALS__" in window) {
    return invoke("generate_self_weight_operation_plan", { model, request });
  }
  const engine = await loadSelfWeightEngine();
  return JSON.parse(engine.generateSelfWeightPlanJson(
    JSON.stringify(model),
    JSON.stringify(request)
  )) as SelfWeightPlanOutcome;
}
