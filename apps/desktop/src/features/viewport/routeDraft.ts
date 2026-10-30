import type { OperationBatch, OperationBatchOutcome } from "../../services/operationBatchService";
import type {
  EditorOperationIntent,
  ModelHashEvidence,
  OperationOutcome,
  PreviewModel
} from "../../types";

export type RouteEndMode = "existing" | "new";

export type RouteDraft = {
  startNodeId: string;
  endMode: RouteEndMode;
  existingEndNodeId: string;
  newEnd: {
    id: string;
    label: string;
    x: string;
    y: string;
    z: string;
    coordinateUnit: string;
    provenance: string;
  };
  pipe: {
    id: string;
    label: string;
    materialId: string;
    outsideDiameter: string;
    wallThickness: string;
    lengthUnit: string;
    yReferenceX: string;
    yReferenceY: string;
    yReferenceZ: string;
    provenance: string;
  };
};

export type NodeCreationDraft = {
  id: string;
  label: string;
  x: string;
  y: string;
  z: string;
  coordinateUnit: string;
  provenance: string;
};

export type DraftSubmission =
  | { kind: "single"; intent: EditorOperationIntent }
  | { kind: "batch"; batch: OperationBatch };

export type DraftValidationOutcome = OperationOutcome | OperationBatchOutcome;

export type FrozenDraftReview = {
  reviewId: string;
  generation: number;
  basisRevision: number;
  basisEpoch: number;
  basisHash: ModelHashEvidence;
  submission: DraftSubmission;
  outcome: DraftValidationOutcome;
};

export type DraftBuildResult =
  | { ok: true; submission: DraftSubmission }
  | { ok: false; errors: string[] };

const SOURCE = {
  source_ref: "apps/desktop/src/features/viewport/PipeViewport.tsx",
  source_channel: "local_desktop_preview",
  source_role: "viewport_editor"
} as const;

export class DraftReviewGate {
  private generation = 0;

  current(): number {
    return this.generation;
  }

  invalidate(): number {
    this.generation += 1;
    return this.generation;
  }

  isCurrent(generation: number): boolean {
    return generation === this.generation;
  }
}

export function buildNodeCreationSubmission(
  model: PreviewModel,
  pending: ReadonlyArray<unknown>,
  draft: NodeCreationDraft,
  sequence: number,
  unitValidation: string
): DraftBuildResult {
  const errors = validateNodeDraft(model, pending, draft);
  if (errors.length > 0) return { ok: false, errors };
  return {
    ok: true,
    submission: {
      kind: "single",
      intent: createNodeIntent(draft, sequence, unitValidation)
    }
  };
}

export function buildRouteSubmission(
  model: PreviewModel,
  pending: ReadonlyArray<unknown>,
  draft: RouteDraft,
  sequence: number,
  unitValidation: string
): DraftBuildResult {
  const errors = validateRouteDraft(model, pending, draft);
  if (errors.length > 0) return { ok: false, errors };

  const pipeIntent = connectPipeIntent(draft, sequence, unitValidation);
  if (draft.endMode === "existing") {
    return { ok: true, submission: { kind: "single", intent: pipeIntent } };
  }

  const nodeIntent = createNodeIntent(draft.newEnd, sequence, unitValidation);
  return {
    ok: true,
    submission: {
      kind: "batch",
      batch: {
        batch_id: `batch:viewport-straight-route-${safeToken(draft.pipe.id)}-${sequence.toString().padStart(3, "0")}`,
        operations: [nodeIntent, pipeIntent]
      }
    }
  };
}

export function validateNodeDraft(
  model: PreviewModel,
  pending: ReadonlyArray<unknown>,
  draft: NodeCreationDraft
): string[] {
  const errors: string[] = [];
  if (!draft.id.trim()) errors.push("Enter a node ID.");
  if (!draft.label.trim()) errors.push("Enter a node label.");
  if (![draft.x, draft.y, draft.z].every(isFiniteInput)) errors.push("Enter finite X, Y, and Z coordinates.");
  if (!validUnit(draft.coordinateUnit)) errors.push("Select a coordinate unit.");
  if (!draft.provenance.trim()) errors.push("Enter node provenance.");
  if (draft.id.trim() && reservedIds(model, pending).has(draft.id.trim())) errors.push("The node ID is already reserved.");
  return errors;
}

export function validateRouteDraft(
  model: PreviewModel,
  pending: ReadonlyArray<unknown>,
  draft: RouteDraft
): string[] {
  const errors: string[] = [];
  const start = draft.startNodeId.trim();
  const end = routeEndNodeId(draft);
  const ids = reservedIds(model, pending);
  if (!model.nodes.some((node) => node.id === start)) errors.push("Choose an existing start node.");
  if (draft.endMode === "existing") {
    if (!model.nodes.some((node) => node.id === end)) errors.push("Choose an existing end node.");
  } else {
    errors.push(...validateNodeDraft(model, pending, draft.newEnd));
  }
  if (start && end && start === end) errors.push("Start and end nodes must be different.");
  if (!draft.pipe.id.trim()) errors.push("Enter a pipe ID.");
  if (!draft.pipe.label.trim()) errors.push("Enter a pipe label.");
  if (draft.pipe.id.trim() && ids.has(draft.pipe.id.trim())) errors.push("The pipe ID is already reserved.");
  if (draft.endMode === "new" && draft.newEnd.id.trim() && draft.newEnd.id.trim() === draft.pipe.id.trim()) {
    errors.push("The new endpoint and pipe IDs must be different.");
  }
  if (!draft.pipe.materialId.trim() || !(model.materials ?? []).some((material) => material.id === draft.pipe.materialId.trim())) {
    errors.push("Choose an existing material ID.");
  }
  if (![draft.pipe.outsideDiameter, draft.pipe.wallThickness].every(isPositiveInput)) {
    errors.push("Enter positive outside diameter and wall thickness values.");
  } else if (Number(draft.pipe.wallThickness) * 2 >= Number(draft.pipe.outsideDiameter)) {
    errors.push("Wall thickness must be less than half the outside diameter.");
  }
  if (!validUnit(draft.pipe.lengthUnit)) errors.push("Select a pipe length unit.");
  if (![draft.pipe.yReferenceX, draft.pipe.yReferenceY, draft.pipe.yReferenceZ].every(isFiniteInput) ||
      ![draft.pipe.yReferenceX, draft.pipe.yReferenceY, draft.pipe.yReferenceZ].some((value) => Number(value) !== 0)) {
    errors.push("Enter a finite, nonzero y-reference vector.");
  }
  if (!draft.pipe.provenance.trim()) errors.push("Enter pipe provenance.");
  return Array.from(new Set(errors));
}

export function routeEndNodeId(draft: RouteDraft): string {
  return draft.endMode === "existing" ? draft.existingEndNodeId.trim() : draft.newEnd.id.trim();
}

export function submissionOperationIds(submission: DraftSubmission): string[] {
  return submission.kind === "single"
    ? [submission.intent.operation_id]
    : submission.batch.operations.map((intent) => intent.operation_id);
}

export function validationMatchesSubmission(
  submission: DraftSubmission,
  outcome: DraftValidationOutcome
): boolean {
  if (outcome.mode !== "validate_only" || hasWarningOrBlockingDiagnostic(outcome.diagnostics)) return false;
  if (submission.kind === "single") {
    if (!("operation_id" in outcome)) return false;
    return outcome.operation_id === submission.intent.operation_id &&
      outcome.change_id === submission.intent.change.change_id &&
      outcome.target_ref === submission.intent.target.ref &&
      outcome.validation.schema_validation === "passed" &&
      outcome.validation.diff_preview_status === "generated" &&
      outcome.validation.application_status === "not_applied" &&
      diffPreviewMatchesIntent(outcome.diff_preview, submission.intent);
  }
  if (!("batch_id" in outcome)) return false;
  return outcome.batch_id === submission.batch.batch_id &&
    sameIntentList(outcome.submitted_operations, submission.batch.operations) &&
    outcome.validation.schema_validation === "passed" &&
    outcome.validation.batch_validation_status === "passed" &&
    outcome.validation.diff_preview_status === "generated" &&
    outcome.validation.application_status === "not_applied" &&
    outcome.simulation_disposition === "validation_only_discarded" &&
    outcome.operation_outcomes.length === submission.batch.operations.length &&
    outcome.operation_outcomes.every((step, index) =>
      step.index === index &&
      step.operation_id === submission.batch.operations[index]?.operation_id &&
      step.change_id === submission.batch.operations[index]?.change.change_id &&
      step.simulation_status === "validated_on_temporary_state" &&
      diffPreviewMatchesIntent(step.diff_preview, submission.batch.operations[index]) &&
      !hasWarningOrBlockingDiagnostic(step.diagnostics)
    );
}

export function applyResultMatchesSubmission(
  submission: DraftSubmission,
  outcome: unknown,
  basisHash: ModelHashEvidence
): boolean {
  if (!isRecord(outcome) || outcome.mode !== "apply" || outcome.input_model_unchanged !== true) return false;
  if (!cleanDiagnostics(outcome.diagnostics) || !completeAcceptance(outcome.acceptance)) return false;
  if (!isRecord(outcome.applied_model) || !nonemptyString(outcome.applied_model_backend_hash)) return false;
  if (!professionalBoundaryMatches(outcome.professional_boundary)) return false;

  if (submission.kind === "single") {
    const intent = submission.intent;
    return outcome.document_kind === "openpipestress.desktop.operation_outcome" &&
      outcome.schema_version === "0.1.0" &&
      jsonValuesEqual(outcome.deliverable_refs, ["DEL-16-02", "DEL-16-03"]) &&
      (outcome.application_route === "tauri_backend_apply" || outcome.application_route === "local_wasm_engine") &&
      outcome.operation_id === intent.operation_id &&
      outcome.change_id === intent.change.change_id &&
      outcome.operation_kind === intent.operation_kind &&
      outcome.change_kind === intent.change.change_kind &&
      outcome.target_object_type === intent.target.object_type &&
      outcome.target_ref === intent.target.ref &&
      successfulSingleValidation(outcome.validation) &&
      diffPreviewMatchesIntent(outcome.diff_preview, intent) &&
      modelBasisMatches(outcome.model_basis, basisHash) &&
      auditBoundaryMatches(outcome.audit_boundary, false);
  }

  const operations = submission.batch.operations;
  if (
    outcome.document_kind !== "openpipestress.desktop.operation_batch_outcome" ||
    outcome.schema_version !== "0.1.0" ||
    outcome.batch_id !== submission.batch.batch_id ||
    (outcome.application_route !== "structured_operation_batch" && outcome.application_route !== "local_wasm_engine") ||
    !successfulBatchValidation(outcome.validation) ||
    outcome.simulation_disposition !== "committed_as_one_batch" ||
    !engineInitialModelHashMatches(outcome.initial_model_hash, basisHash) ||
    outcome.initial_backend_hash !== basisHash.value ||
    outcome.input_backend_hash !== basisHash.value ||
    !nonemptyString(outcome.batch_hash) ||
    !sameIntentList(outcome.submitted_operations, operations) ||
    outcome.submitted_operations_trust !== "untrusted_submitted_metadata_not_validation_evidence" ||
    !jsonValuesEqual(outcome.submitted_initial_model_hash, basisHash) ||
    !auditBoundaryMatches(outcome.audit_boundary, true) ||
    !Array.isArray(outcome.operation_outcomes) ||
    outcome.operation_outcomes.length !== operations.length
  ) return false;
  return outcome.operation_outcomes.every((candidate, index) => {
    const intent = operations[index];
    if (!isRecord(candidate) || !intent) return false;
    return candidate.index === index &&
      candidate.operation_id === intent.operation_id &&
      candidate.change_id === intent.change.change_id &&
      candidate.target_object_type === intent.target.object_type &&
      candidate.target_ref === intent.target.ref &&
      candidate.change_kind === intent.change.change_kind &&
      candidate.simulation_status === "validated_on_temporary_state" &&
      successfulBatchStepValidation(candidate.validation) &&
      diffPreviewMatchesIntent(candidate.diff_preview, intent) &&
      cleanDiagnostics(candidate.diagnostics);
  });
}

export function sameSubmission(left: DraftSubmission, right: DraftSubmission): boolean {
  return jsonValuesEqual(left, right);
}

function sameIntentList(left: unknown, right: EditorOperationIntent[]): boolean {
  return jsonValuesEqual(left, right);
}

function diffPreviewMatchesIntent(
  rows: unknown,
  intent: EditorOperationIntent | undefined
): boolean {
  if (!intent || !Array.isArray(rows) || rows.length !== 1 || !isRecord(rows[0])) return false;
  const row = rows[0];
  return row.entity_ref === intent.target.ref &&
    row.object_type === intent.target.object_type &&
    row.field_path === intent.change.field_path &&
    row.before === intent.change.before &&
    row.after === intent.change.after &&
    row.unit === intent.change.unit &&
    row.dimension === intent.change.dimension &&
    row.change_kind === intent.change.change_kind;
}

function successfulSingleValidation(value: unknown): boolean {
  return isRecord(value) &&
    value.schema_validation === "passed" &&
    value.reference_validation === "passed" &&
    value.unit_validation === "passed" &&
    value.before_state_validation === "passed" &&
    value.diff_preview_status === "generated" &&
    value.application_status === "applied_to_session_model";
}

function successfulBatchValidation(value: unknown): boolean {
  return isRecord(value) &&
    value.schema_validation === "passed" &&
    value.batch_validation_status === "passed" &&
    value.diff_preview_status === "generated" &&
    value.application_status === "applied_to_session_model";
}

function successfulBatchStepValidation(value: unknown): boolean {
  return isRecord(value) &&
    value.schema_validation === "passed" &&
    value.reference_validation === "passed" &&
    value.unit_validation === "passed" &&
    value.before_state_validation === "passed" &&
    value.diff_preview_status === "generated";
}

function modelBasisMatches(value: unknown, basisHash: ModelHashEvidence): boolean {
  return isRecord(value) &&
    value.claimed_model_hash === basisHash.value &&
    value.claimed_hash_canonicalization === basisHash.canonicalization &&
    value.backend_model_hash === basisHash.value &&
    value.backend_canonicalization === basisHash.canonicalization &&
    value.binding_status === "claimed_model_hash_matches_current_backend_model";
}

function engineInitialModelHashMatches(value: unknown, basisHash: ModelHashEvidence): boolean {
  return isRecord(value) &&
    value.algorithm === basisHash.algorithm &&
    value.canonicalization === basisHash.canonicalization &&
    value.payload_scope === basisHash.payload_scope &&
    value.payload_ref === "model:local_batch_input" &&
    value.value === basisHash.value &&
    value.hash_status === basisHash.hash_status;
}

function completeAcceptance(value: unknown): boolean {
  return isRecord(value) &&
    value.acceptance_basis === "user_initiated_apply_in_local_session" &&
    value.acceptance_is_professional_approval === false &&
    value.persistence_status === "session_state_only_not_yet_saved";
}

function professionalBoundaryMatches(value: unknown): boolean {
  return isRecord(value) &&
    value.human_review_required === true &&
    value.software_makes_compliance_claim === false &&
    value.software_makes_certification_claim === false &&
    value.software_makes_sealing_claim === false &&
    value.software_makes_approval_claim === false &&
    value.software_makes_authentication_claim === false;
}

function auditBoundaryMatches(value: unknown, batch: boolean): boolean {
  return isRecord(value) &&
    value.mutation_route === "structured_operations_only" &&
    value.direct_model_mutation_allowed === false &&
    value.requires_user_acceptance === true &&
    value.input_model_mutated_in_place === false &&
    value.applied_model_is_new_document === true &&
    (!batch || (
      value.agent_runtime_binding === "held_D58" &&
      value.source_identity_verification === "not_performed_asserted_metadata_only"
    ));
}

function cleanDiagnostics(value: unknown): boolean {
  return Array.isArray(value) && value.every((diagnostic) =>
    isRecord(diagnostic) &&
    diagnostic.severity === "info" &&
    nonemptyString(diagnostic.id) &&
    nonemptyString(diagnostic.code) &&
    nonemptyString(diagnostic.message) &&
    nonemptyString(diagnostic.remediation) &&
    Array.isArray(diagnostic.affected_refs) &&
    diagnostic.affected_refs.every(nonemptyString) &&
    nonemptyString(diagnostic.source)
  );
}

function nonemptyString(value: unknown): value is string {
  return typeof value === "string" && Boolean(value.trim());
}

function createNodeIntent(
  draft: NodeCreationDraft,
  sequence: number,
  unitValidation: string
): EditorOperationIntent {
  const id = draft.id.trim();
  const unit = draft.coordinateUnit.trim();
  return baseIntent({
    operationId: `op:viewport-create-node-${safeToken(id)}-${sequence.toString().padStart(3, "0")}`,
    operationKind: "create",
    objectType: "Node",
    targetRef: id,
    changeId: `change:viewport:create-node:${safeToken(id)}`,
    changeKind: "create_node",
    fieldLabel: "Explicit node geometry",
    fieldPath: "nodes",
    after: JSON.stringify({
      id,
      label: draft.label.trim(),
      position: { x: Number(draft.x), y: Number(draft.y), z: Number(draft.z) },
      provenance: draft.provenance.trim()
    }),
    unit,
    unitValidation,
    sourceNote: "explicit user-entered viewport node geometry and provenance",
    rationale: "explicit user-entered node geometry; requires service validation before durable model change."
  });
}

function connectPipeIntent(draft: RouteDraft, sequence: number, unitValidation: string): EditorOperationIntent {
  const id = draft.pipe.id.trim();
  const unit = draft.pipe.lengthUnit.trim();
  return baseIntent({
    operationId: `op:viewport-connect-pipe-${safeToken(id)}-${sequence.toString().padStart(3, "0")}`,
    operationKind: "connect",
    objectType: "Element",
    targetRef: id,
    changeId: `change:viewport:connect-pipe:${safeToken(id)}`,
    changeKind: "connect_pipe_run",
    fieldLabel: "Explicit straight pipe connectivity",
    fieldPath: "pipe_segments",
    after: JSON.stringify({
      id,
      label: draft.pipe.label.trim(),
      from: draft.startNodeId.trim(),
      to: routeEndNodeId(draft),
      section: {
        outside_diameter: { value: Number(draft.pipe.outsideDiameter), unit },
        wall_thickness: { value: Number(draft.pipe.wallThickness), unit }
      },
      material: draft.pipe.materialId.trim(),
      y_reference: {
        x: Number(draft.pipe.yReferenceX),
        y: Number(draft.pipe.yReferenceY),
        z: Number(draft.pipe.yReferenceZ)
      },
      provenance: draft.pipe.provenance.trim()
    }),
    unit,
    unitValidation,
    sourceNote: "explicit user-entered straight-pipe connectivity and inline section geometry",
    rationale: "explicit user-entered straight route; requires service validation before durable model change."
  });
}

function baseIntent(input: {
  operationId: string;
  operationKind: EditorOperationIntent["operation_kind"];
  objectType: EditorOperationIntent["target"]["object_type"];
  targetRef: string;
  changeId: string;
  changeKind: EditorOperationIntent["change"]["change_kind"];
  fieldLabel: string;
  fieldPath: string;
  after: string;
  unit: string;
  unitValidation: string;
  sourceNote: string;
  rationale: string;
}): EditorOperationIntent {
  return {
    operation_id: input.operationId,
    operation_kind: input.operationKind,
    operation_status: "proposed",
    author_type: "user",
    source: SOURCE,
    target: { object_type: input.objectType, ref: input.targetRef },
    change: {
      change_id: input.changeId,
      change_kind: input.changeKind,
      field_label: input.fieldLabel,
      field_path: input.fieldPath,
      before: "not_present",
      after: input.after,
      unit: input.unit,
      dimension: "length",
      source_note: input.sourceNote
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: input.unitValidation,
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: input.rationale
  };
}

function reservedIds(model: PreviewModel, pending: ReadonlyArray<unknown>): Set<string> {
  const ids = new Set<string>();
  [
    ...(model.materials ?? []),
    ...(model.sections ?? []),
    ...model.nodes,
    ...model.pipe_segments,
    ...model.supports,
    ...model.components,
    ...model.load_cases,
    ...(model.combinations ?? [])
  ].forEach((entity) => ids.add(entity.id));
  for (const candidate of pending) {
    const ref = reservationRef(candidate);
    if (ref) ids.add(ref);
  }
  return ids;
}

function reservationRef(candidate: unknown): string | null {
  if (!isRecord(candidate)) return null;
  const kind = candidate.operation_kind;
  if (kind !== "create" && kind !== "connect" && kind !== "insert") return null;
  const target = candidate.target;
  if (
    !isRecord(target) ||
    typeof target.object_type !== "string" ||
    !SUPPORTED_RESERVATION_OBJECT_TYPES.has(target.object_type) ||
    typeof target.ref !== "string" ||
    !target.ref.trim()
  ) return null;
  return target.ref;
}

const SUPPORTED_RESERVATION_OBJECT_TYPES = new Set([
  "Material", "Section", "Node", "Element", "Component", "Support", "Load", "Combination"
]);

function jsonValuesEqual(left: unknown, right: unknown): boolean {
  if (left === right) return true;
  if (Array.isArray(left) || Array.isArray(right)) {
    return Array.isArray(left) && Array.isArray(right) &&
      left.length === right.length &&
      left.every((value, index) => jsonValuesEqual(value, right[index]));
  }
  if (!isRecord(left) || !isRecord(right)) return false;
  const leftKeys = Object.keys(left).sort();
  const rightKeys = Object.keys(right).sort();
  return leftKeys.length === rightKeys.length &&
    leftKeys.every((key, index) => key === rightKeys[index] && jsonValuesEqual(left[key], right[key]));
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function isFiniteInput(value: string): boolean {
  return value.trim() !== "" && Number.isFinite(Number(value));
}

function isPositiveInput(value: string): boolean {
  return isFiniteInput(value) && Number(value) > 0;
}

function validUnit(value: string): boolean {
  return Boolean(value.trim() && value.trim() !== "TBD");
}

function hasWarningOrBlockingDiagnostic(diagnostics: Array<{ severity: string }>): boolean {
  return diagnostics.some((diagnostic) => diagnostic.severity === "warning" || diagnostic.severity === "blocking");
}

function safeToken(value: string): string {
  return value.trim().replace(/[^A-Za-z0-9:_-]+/g, "-") || "TBD";
}
