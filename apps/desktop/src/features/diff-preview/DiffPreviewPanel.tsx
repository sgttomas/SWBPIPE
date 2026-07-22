import { Download, GitCompare } from "lucide-react";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  EditorOperationIntent,
  PreviewModel,
  SelectedReviewTarget
} from "../../types";
import { buildExportUnitSystemDisclosure, unitDisclosureSummary } from "../exportUnitDisclosure";

type DiffChangeRow = {
  change_id: string;
  change_kind: string;
  target_ref: string;
  field_path: string;
  field_label: string;
  before: string;
  after: string;
  unit: string;
  dimension: string;
  source_note: string;
};

export function DiffPreviewPanel({
  model,
  analysisRun,
  editorIntents,
  proposal,
  selectedReviewTarget
}: {
  model: PreviewModel;
  analysisRun: AnalysisRunEnvelope | null;
  editorIntents: EditorOperationIntent[];
  proposal: AgentProposal | null;
  selectedReviewTarget: SelectedReviewTarget | null;
}) {
  const packet = buildDiffPreviewPacket({ model, analysisRun, editorIntents, proposal, selectedReviewTarget });
  const hasRows = packet.previews.length > 0;

  return (
    <section className="panel diff-preview-panel" aria-label="Operation diff preview" data-testid="diff-preview-panel">
      <div className="panel-title">
        <GitCompare size={16} aria-hidden="true" />
        Operation Diff Preview
      </div>
      {hasRows ? (
        <>
          <div className="report-actions">
        <ControlledExportLink
              className="report-export-link"
              data-testid="diff-preview-export-link"
              download={`openpipestress-preview-operation-diff-${safeFileToken(model.project.id)}.json`}
              href={jsonDataHref(packet)}
            >
              <Download size={14} aria-hidden="true" />
              Local diff JSON
        </ControlledExportLink>
            <span data-testid="diff-preview-summary">
              {packet.summary.operation_count} operations; {packet.summary.diff_row_count} diff rows; accepted_state_mutated=
              {String(packet.summary.accepted_model_state_mutated)}
            </span>
          </div>
          <div className="report-list" data-testid="diff-preview-body">
            <DiffLine
              label="State binding"
              value={`${packet.model_state_ref.ref}; ${packet.analysis_run_ref.ref}`}
              testId="diff-preview-state-binding"
            />
            <DiffLine
              label="Units"
              value={`${packet.unit_system_disclosure.unit_system_ref.ref}; ${unitDisclosureSummary(
                packet.unit_system_disclosure
              )}`}
              testId="diff-preview-units"
            />
            <DiffLine
              label="Unit witnesses"
              value={`count=${packet.unit_preservation_witnesses.length}; policy=preserve_diff_change_units; conversion=${String(
                packet.unit_preservation_witnesses.some((item) => item.conversion_performed)
              )}`}
              testId="diff-preview-unit-witnesses"
            />
            <DiffLine label="Validation" value={diffValidation(packet)} testId="diff-preview-validation" />
            <DiffLine label="Boundary" value={diffBoundary(packet)} testId="diff-preview-boundary" />
            <div className="operation-record-list" data-testid="diff-preview-records">
              {packet.previews.map((preview) => (
                <article
                  className="operation-record"
                  data-testid={`diff-preview-record-${safeFileToken(preview.operation_id)}`}
                  key={preview.preview_id}
                >
                  <strong>{preview.operation_id}</strong>
                  <small>
                    {preview.record_source}; {preview.application_status}; preview={preview.diff_preview_status}
                  </small>
                  <p>{preview.changes.map(formatChange).join("; ")}</p>
                </article>
              ))}
            </div>
          </div>
        </>
      ) : (
        <p className="muted" data-testid="diff-preview-empty">
          No operation diffs are queued. GUI and agent operations must produce reviewable before/after context before
          acceptance.
        </p>
      )}
      <small className="report-note">
        Diff previews are local review evidence only; this panel does not apply operations or mutate accepted model
        state. Acceptance and professional judgment remain with the responsible engineer.
      </small>
    </section>
  );
}

function DiffLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildDiffPreviewPacket({
  model,
  analysisRun,
  editorIntents,
  proposal,
  selectedReviewTarget
}: {
  model: PreviewModel;
  analysisRun: AnalysisRunEnvelope | null;
  editorIntents: EditorOperationIntent[];
  proposal: AgentProposal | null;
  selectedReviewTarget: SelectedReviewTarget | null;
}) {
  const run = analysisRun?.analysis_run;
  const previews = [
    ...editorIntents.map((intent, index) => editorIntentPreview({ intent, index })),
    ...(proposal ? [proposalPreview({ proposal, index: editorIntents.length })] : [])
  ];
  const diffRowCount = previews.reduce((total, preview) => total + preview.changes.length, 0);
  const heldCount = previews.filter((preview) => preview.review_decision === "held_for_user_acceptance").length;
  const unitPreservationWitnesses = operationDiffUnitPreservationWitnesses(previews);
  const unitSystemDisclosure = buildExportUnitSystemDisclosure({
    model,
    targetExportUnits: {
      diff_preview_change_rows: "per_change_declared_unit"
    },
    conversionPolicy: "operation_diff_preview_preserves_declared_change_units_no_preview_conversion",
    conversionPerformed: false,
    conversionScope: [],
    sourceLocation: "apps/desktop/src/features/diff-preview/DiffPreviewPanel.tsx"
  });

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.operation_diff_preview",
    export_scope: "local_browser_download_preview",
    deliverable_refs: ["DEL-16-01", "DEL-16-02", "DEL-16-03", "DEL-16-04"],
    scope_items: ["SOW-069", "SOW-070"],
    objectives: ["OBJ-015", "OBJ-018"],
    project_ref: model.project.id,
    model_state_ref: run?.model_state_ref ?? { object_type: "ModelState", ref: "not generated" },
    analysis_run_ref: run ? { object_type: "AnalysisRun", ref: run.run_id } : { object_type: "AnalysisRun", ref: "not generated" },
    selected_review_target: selectedReviewTarget,
    summary: {
      operation_count: previews.length,
      diff_row_count: diffRowCount,
      held_for_user_acceptance_count: heldCount,
      accepted_model_state_mutated: false,
      hash_bound_diff_preview_count: 0,
      local_visual_diff_preview_count: diffRowCount,
      unit_preservation_witness_count: unitPreservationWitnesses.length
    },
    unit_system_disclosure: unitSystemDisclosure,
    unit_witness_policy: "preserve_operation_diff_change_value_unit_and_dimension_per_unit_bearing_row",
    unit_preservation_witnesses: unitPreservationWitnesses,
    previews,
    unresolved_tbd: [
      "hash-bound backend diff preview reference for GUI editor intents",
      "controlled operation application outside technical preview",
      "durable accepted/rejected decision persistence"
    ],
    data_boundary: model.data_boundary,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    professional_boundary: professionalBoundary()
  };
}

function operationDiffUnitPreservationWitnesses(previews: Array<{ preview_id: string; operation_id: string; changes: DiffChangeRow[] }>) {
  return previews.flatMap((preview, previewIndex) =>
    preview.changes
      .map((change, changeIndex) => ({ change, changeIndex }))
      .filter(({ change }) => isUnitBearingDiffChange(change))
      .map(({ change, changeIndex }) => ({
        witness_id: `operation-diff-unit:${safeRefToken(`${preview.operation_id}-${change.change_id}`)}`,
        source_operation_ref: reference("Operation", preview.operation_id),
        source_change_ref: reference("OperationChange", change.change_id),
        source_field_path: `previews[${previewIndex}].changes[${changeIndex}]`,
        source_quantity: {
          before: change.before,
          after: change.after,
          unit: change.unit,
          dimension: change.dimension
        },
        target_preview_ref: reference("DiffPreview", preview.preview_id),
        target_field_path: `previews[${previewIndex}].changes[${changeIndex}]`,
        target_quantity: {
          before: change.before,
          after: change.after,
          unit: change.unit,
          dimension: change.dimension
        },
        target_quantity_policy: "diff_preview_preserves_declared_operation_change_value_unit_and_dimension",
        export_unit_policy: "preserve_diff_change_row_unit_and_dimension",
        conversion_performed: false,
        decision_basis_refs: [
          reference("Decision", "DEC-018"),
          reference("Deliverable", "DEL-02-02"),
          reference("Deliverable", "DEL-16-02"),
          reference("Deliverable", "DEL-16-03")
        ],
        provenance: {
          source_name: "OpenPipeStress desktop operation diff preview",
          source_location: "apps/desktop/src/features/diff-preview/DiffPreviewPanel.tsx",
          source_license: "project-governed",
          contributor: "OpenPipeStress app integration tranche",
          contributor_certification: "Operation diff unit metadata only; no protected standards or private payloads.",
          redistribution_status: "public_permissive",
          review_status: "desktop_preview",
          privacy_classification: "public_metadata"
        }
      }))
  );
}

function isUnitBearingDiffChange(change: DiffChangeRow): boolean {
  return change.unit !== "none" && change.unit !== "TBD" && change.dimension !== "dimensionless" && change.dimension !== "TBD";
}

function editorIntentPreview({ intent, index }: { intent: EditorOperationIntent; index: number }) {
  return {
    preview_id: `diff-preview:gui:${safeRefToken(intent.operation_id)}:${index + 1}`,
    record_source: "gui_editor_intent_queue",
    source_ref: intent.queue_id ?? `editor-intent:${index + 1}`,
    operation_id: intent.operation_id,
    operation_kind: intent.operation_kind,
    operation_status: intent.operation_status,
    affected_entities: [intent.target.ref],
    validation: intent.validation,
    diff_preview_status: intent.validation.diff_preview_status,
    application_status: intent.validation.application_status,
    review_decision: "held_for_user_acceptance",
    explicit_user_acceptance: false,
    accepted_model_state_mutated: false,
    hash_bound_diff_preview: false,
    changes: [
      {
        change_id: intent.change.change_id,
        change_kind: intent.change.change_kind,
        target_ref: intent.target.ref,
        field_path: intent.change.field_path,
        field_label: intent.change.field_label,
        before: intent.change.before,
        after: intent.change.after,
        unit: intent.change.unit,
        dimension: intent.change.dimension,
        source_note: intent.change.source_note
      }
    ],
    diagnostics: [
      diagnostic("DIFF-PREVIEW-HASH-TBD", "info", "Local visual diff is available; hash-bound backend diff reference is TBD."),
      diagnostic("DIFF-PREVIEW-NOT-APPLIED", "warning", "Operation is review-only and has not been applied.")
    ],
    professional_boundary: intent.professional_boundary,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false
  };
}

function proposalPreview({ proposal, index }: { proposal: AgentProposal; index: number }) {
  const applicationStatus = proposal.validation.application_status ?? "not_applied";
  const diffPreviewStatus = proposal.validation.diff_preview_status ?? "TBD";
  return {
    preview_id: `diff-preview:proposal:${safeRefToken(proposal.operation.operation_id)}:${index + 1}`,
    record_source: "agent_proposal",
    source_ref: proposal.proposal_id,
    operation_id: proposal.operation.operation_id,
    operation_kind: proposal.operation.operation_kind,
    operation_status: proposal.operation.operation_status,
    affected_entities: proposal.operation.affected_entity_ids,
    validation: proposal.validation,
    diff_preview_status: diffPreviewStatus,
    application_status: applicationStatus,
    review_decision: "held_for_user_acceptance",
    explicit_user_acceptance: false,
    accepted_model_state_mutated: false,
    hash_bound_diff_preview: false,
    changes: proposal.operation.changes.map((change) => ({
      change_id: change.change_id,
      change_kind: change.change_kind,
      target_ref: change.target_ref,
      field_path: change.target_ref,
      field_label: change.change_kind,
      before: change.before,
      after: change.after,
      unit: "TBD",
      dimension: "TBD",
      source_note: proposal.prompt
    })),
    diagnostics: [
      diagnostic("DIFF-PREVIEW-PROPOSAL-REVIEW-ONLY", "warning", "Agent proposal requires explicit user acceptance."),
      diagnostic("DIFF-PREVIEW-HASH-TBD", "info", "Hash-bound backend diff reference is TBD.")
    ],
    professional_boundary: proposal.professional_boundary,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false
  };
}

function formatChange(change: DiffChangeRow): string {
  const unitSuffix = change.unit === "none" || change.unit === "TBD" ? "" : ` ${change.unit}`;
  return `${change.target_ref} ${change.field_path}: ${change.before} to ${change.after}${unitSuffix}`;
}

function diffValidation(packet: ReturnType<typeof buildDiffPreviewPacket>): string {
  return [
    `${packet.summary.local_visual_diff_preview_count} local rows`,
    `${packet.summary.hash_bound_diff_preview_count} hash-bound rows`,
    `${packet.summary.held_for_user_acceptance_count} held`
  ].join("; ");
}

function diffBoundary(packet: ReturnType<typeof buildDiffPreviewPacket>): string {
  return [
    `accepted model mutated=${String(packet.summary.accepted_model_state_mutated)}`,
    `private payload=${String(packet.private_payload_included)}`,
    `protected content=${String(packet.protected_content_included)}`,
    "acceptance and professional judgment remain with the responsible engineer"
  ].join("; ");
}

function diagnostic(code: string, severity: "info" | "warning", message: string) {
  return {
    code,
    severity,
    message,
    source: "apps/desktop/src/features/diff-preview/DiffPreviewPanel.tsx"
  };
}

function professionalBoundary() {
  return {
    human_review_required: true,
    software_makes_compliance_claim: false,
    software_makes_certification_claim: false,
    software_makes_sealing_claim: false,
    software_makes_approval_claim: false,
    software_makes_authentication_claim: false
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeRefToken(value: string): string {
  return value.replace(/[^a-z0-9]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}

function safeFileToken(value: string): string {
  return safeRefToken(value);
}

function reference(objectType: string, ref: string) {
  return { object_type: objectType, ref };
}
import { ControlledExportLink } from "../redaction-controls/ControlledExportLink";
