import { ClipboardCheck, Download, Trash2 } from "lucide-react";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  EditorOperationIntent,
  PreviewModel,
  SelectedReviewTarget
} from "../../types";

export function OperationLedgerPanel({
  model,
  analysisRun,
  editorIntents,
  proposal,
  selectedReviewTarget,
  onClearReviewQueue
}: {
  model: PreviewModel;
  analysisRun: AnalysisRunEnvelope | null;
  editorIntents: EditorOperationIntent[];
  proposal: AgentProposal | null;
  selectedReviewTarget: SelectedReviewTarget | null;
  onClearReviewQueue: () => void;
}) {
  const ledger = buildOperationReviewLedger({
    model,
    analysisRun,
    editorIntents,
    proposal,
    selectedReviewTarget
  });
  const hasRecords = ledger.records.length > 0;

  return (
    <section className="panel operation-ledger-panel" aria-label="Operation review ledger" data-testid="operation-ledger-panel">
      <div className="panel-title">
        <ClipboardCheck size={16} aria-hidden="true" />
        Operation Review Ledger
      </div>
      {hasRecords ? (
        <>
          <div className="report-actions">
            <a
              className="report-export-link"
              data-testid="operation-ledger-export-link"
              download={`openpipestress-preview-operation-ledger-${safeFileToken(ledger.project_ref)}.json`}
              href={jsonDataHref(ledger)}
            >
              <Download size={14} aria-hidden="true" />
              Local ledger JSON
            </a>
            <span data-testid="operation-ledger-export-summary">
              {ledger.records.length} review {ledger.records.length === 1 ? "record" : "records"};{" "}
              {ledger.decision_counts.held_for_user_acceptance} held; no accepted-state mutation
            </span>
            <button
              className="secondary-action"
              data-testid="clear-operation-review-queue"
              onClick={onClearReviewQueue}
              type="button"
            >
              <Trash2 size={14} aria-hidden="true" />
              Clear pending operations
            </button>
          </div>
          <div className="report-list" data-testid="operation-ledger-body">
            <LedgerLine
              label="Decision counts"
              value={formatDecisionCounts(ledger.decision_counts)}
              testId="operation-ledger-decision-counts"
            />
            <LedgerLine
              label="State binding"
              value={`${ledger.model_state_ref.ref}; ${ledger.analysis_run_ref.ref}`}
              testId="operation-ledger-state-binding"
            />
            <LedgerLine
              label="Latest operation"
              value={formatLatestOperation(ledger)}
              testId="operation-ledger-latest"
            />
            <LedgerLine
              label="Unit policy"
              value={operationLedgerUnitPolicySummary(ledger)}
              testId="operation-ledger-unit-policy"
            />
            <LedgerLine label="Boundary" value={ledgerBoundary(ledger)} testId="operation-ledger-boundary" />
            <div className="operation-record-list" data-testid="operation-ledger-records">
              {ledger.records.map((record) => (
                <article
                  className="operation-record"
                  data-testid={`operation-ledger-record-${safeFileToken(record.operation_id)}`}
                  key={record.audit_record_id}
                >
                  <strong>{record.operation_id}</strong>
                  <small>
                    {record.record_source}; {record.decision.status}; {applicationStatus(record.validation_outcome)}
                  </small>
                  <p>{record.affected_entities.map((item) => item.ref).join(", ")}</p>
                </article>
              ))}
            </div>
          </div>
        </>
      ) : (
        <p className="muted" data-testid="operation-ledger-empty">
          No structured operations are queued for review. GUI and agent changes must remain proposed operations until explicit user acceptance.
        </p>
      )}
      <small className="report-note">
        Operation ledger records are local review evidence only; this panel does not apply operations. Application runs
        through the structured apply seam (Apply Operations panel) and never creates professional acceptance.
      </small>
    </section>
  );
}

function LedgerLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildOperationReviewLedger({
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
  const records = [
    ...editorIntents.map((intent, index) => editorIntentRecord({ intent, index, analysisRun, selectedReviewTarget })),
    ...(proposal ? [proposalRecord({ proposal, analysisRun, selectedReviewTarget, index: editorIntents.length })] : [])
  ];
  const unitPolicyEvidence = operationLedgerUnitPolicyEvidence(records);
  const decisionCounts = records.reduce(
    (counts, record) => {
      counts[record.decision.status] += 1;
      return counts;
    },
    { accepted: 0, rejected: 0, held_for_user_acceptance: 0 }
  );

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.operation_review_ledger",
    export_scope: "local_browser_download_preview",
    deliverable_refs: ["DEL-16-01", "DEL-16-02", "DEL-16-03", "DEL-16-04"],
    scope_items: ["SOW-069", "SOW-070"],
    objectives: ["OBJ-015", "OBJ-018"],
    project_ref: model.project.id,
    model_state_ref: run?.model_state_ref ?? { object_type: "ModelState", ref: "not generated" },
    analysis_run_ref: run ? { object_type: "AnalysisRun", ref: run.run_id } : { object_type: "AnalysisRun", ref: "not generated" },
    audit_posture: {
      default_user_acceptance_required: true,
      accepted_records_require_explicit_user_signal: true,
      preview_records_do_not_apply_operations: true,
      rejected_records_mutate_accepted_state: false
    },
    unit_policy_evidence: unitPolicyEvidence,
    decision_counts: decisionCounts,
    records,
    accepted_model_state_unchanged: true,
    selected_review_target: selectedReviewTarget,
    unresolved_tbd: [
      "durable operation audit persistence container",
      "final actor identity model",
      "durable applied-operation receipt persistence (session receipts only until the model-document persistence tranche)"
    ],
    data_boundary: model.data_boundary,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    professional_boundary: professionalBoundary()
  };
}

function editorIntentRecord({
  intent,
  index,
  analysisRun,
  selectedReviewTarget
}: {
  intent: EditorOperationIntent;
  index: number;
  analysisRun: AnalysisRunEnvelope | null;
  selectedReviewTarget: SelectedReviewTarget | null;
}) {
  const targetRef = { object_type: intent.target.object_type, ref: intent.target.ref };
  return {
    audit_record_id: `audit:held:${safeRefToken(intent.operation_id)}:${index + 1}`,
    record_source: "gui_editor_intent_queue",
    source_ref: intent.queue_id ?? `editor-intent:${index + 1}`,
    operation_id: intent.operation_id,
    proposal_ref: null,
    operation_kind: intent.operation_kind,
    operation_status: intent.operation_status,
    decision: heldDecision(intent.rationale),
    actor: {
      actor_type: intent.author_type,
      actor_ref: "local-preview-user",
      source_role: intent.source?.source_role ?? "gui_editor"
    },
    source: {
      source_ref: intent.source?.source_ref ?? "apps/desktop/property-inspector",
      source_channel: intent.source?.source_channel ?? "local_desktop_preview"
    },
    affected_entities: [targetRef],
    operation_history: {
      target_refs: [targetRef],
      changes: [intent.change],
      validation: intent.validation,
      assumptions: visibleEditorAssumptions(intent),
      provenance: intent.change.source_note
    },
    validation_outcome: {
      ...intent.validation,
      status: "held_for_user_acceptance"
    },
    diff_preview_ref: {
      object_type: "DiffPreview",
      ref: intent.validation.diff_preview_status === "not_generated" ? "not generated" : "preview:TBD",
      hash: "TBD"
    },
    selected_review_target: selectedReviewTarget,
    accepted_model_state_ref: analysisRun?.analysis_run.model_state_ref ?? { object_type: "ModelState", ref: "not generated" },
    diagnostics: [
      diagnostic("AUDIT-EXPLICIT-USER-ACCEPTANCE-REQUIRED", "warning", "Explicit user acceptance has not been recorded."),
      diagnostic("AUDIT-DIFF-PREVIEW-REF-TBD", "info", "Hash-bound diff preview reference is not generated in this preview ledger.")
    ],
    professional_boundary: intent.professional_boundary,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false
  };
}

function proposalRecord({
  proposal,
  analysisRun,
  selectedReviewTarget,
  index
}: {
  proposal: AgentProposal;
  analysisRun: AnalysisRunEnvelope | null;
  selectedReviewTarget: SelectedReviewTarget | null;
  index: number;
}) {
  return {
    audit_record_id: `audit:held:${safeRefToken(proposal.operation.operation_id)}:${index + 1}`,
    record_source: "agent_proposal",
    source_ref: proposal.proposal_id,
    operation_id: proposal.operation.operation_id,
    proposal_ref: proposal.proposal_id,
    operation_kind: proposal.operation.operation_kind,
    operation_status: proposal.operation.operation_status,
    decision: heldDecision(proposal.rationale),
    actor: {
      actor_type: "agent",
      actor_ref: "agent:local-preview-proposal",
      source_role: "review_proposal_generator"
    },
    source: {
      source_ref: "apps/desktop/agent-proposal-panel",
      source_channel: "local_desktop_preview"
    },
    affected_entities: proposal.operation.affected_entity_ids.map((refValue) => ({
      object_type: "ReviewTarget",
      ref: refValue
    })),
    operation_history: {
      target_refs: proposal.operation.affected_entity_ids.map((refValue) => ({
        object_type: "ReviewTarget",
        ref: refValue
      })),
      changes: proposal.operation.changes,
      validation: proposal.validation,
      assumptions: [
        ...proposal.assumptions,
        "explicit user acceptance not recorded",
        "operation application disabled in technical preview"
      ],
      provenance: proposal.prompt
    },
    validation_outcome: {
      ...proposal.validation,
      application_status: proposal.validation.application_status ?? "not_applied",
      status: "held_for_user_acceptance"
    },
    diff_preview_ref: {
      object_type: "DiffPreview",
      ref: proposal.validation.diff_preview_status,
      hash: "TBD"
    },
    selected_review_target: selectedReviewTarget,
    accepted_model_state_ref: analysisRun?.analysis_run.model_state_ref ?? { object_type: "ModelState", ref: "not generated" },
    diagnostics: [
      diagnostic("AUDIT-EXPLICIT-USER-ACCEPTANCE-REQUIRED", "warning", "Agent proposal requires explicit user acceptance."),
      diagnostic("AUDIT-ACCEPTANCE-REVIEW-ONLY", "info", "Proposal acceptance is represented as review-only in this preview.")
    ],
    professional_boundary: proposal.professional_boundary,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false
  };
}

function heldDecision(rationale: string) {
  return {
    status: "held_for_user_acceptance" as const,
    explicit_user_acceptance: false,
    acceptance_required: true,
    accepted_model_state_mutated: false,
    decided_at: "TBD",
    rationale
  };
}

function visibleEditorAssumptions(intent: EditorOperationIntent): string[] {
  return [
    `${intent.validation.schema_validation} schema validation`,
    `${intent.validation.constraint_validation} constraint validation`,
    `${intent.validation.unit_validation} unit validation`,
    `${intent.validation.diff_preview_status} diff preview`,
    "explicit user acceptance not recorded"
  ];
}

function diagnostic(code: string, severity: "info" | "warning", message: string) {
  return {
    code,
    severity,
    message,
    source: "apps/desktop/src/features/operations/OperationLedgerPanel.tsx"
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

function formatDecisionCounts(counts: { accepted: number; rejected: number; held_for_user_acceptance: number }): string {
  return `${counts.held_for_user_acceptance} held_for_user_acceptance; ${counts.accepted} accepted; ${counts.rejected} rejected`;
}

function formatLatestOperation(ledger: ReturnType<typeof buildOperationReviewLedger>): string {
  const latest = ledger.records[ledger.records.length - 1];
  if (!latest) return "none";
  return `${latest.operation_id}; ${latest.record_source}; ${latest.decision.status}; ${applicationStatus(latest.validation_outcome)}`;
}

type UnitPolicyLedgerRecord = {
  operation_history: {
    changes: Array<{ change_id: string; unit?: string }>;
  };
  validation_outcome: {
    status: string;
    unit_validation?: string;
  };
};

function operationLedgerUnitPolicyEvidence(records: UnitPolicyLedgerRecord[]) {
  const changeRecords = records.flatMap((record) => record.operation_history.changes);
  const unitBearingChangeCount = changeRecords.filter(
    (change) => typeof change.unit === "string" && change.unit !== "none"
  ).length;
  const dimensionlessChangeCount = changeRecords.length - unitBearingChangeCount;
  const validationStatuses = Array.from(
    new Set(
      records
        .map((record) => record.validation_outcome.unit_validation)
        .filter((status): status is string => typeof status === "string" && status.trim().length > 0)
    )
  ).sort();

  return {
    evidence_id: "unit-policy-evidence:operation-review-ledger",
    unit_policy: "ledger_preserves_operation_unit_metadata_without_conversion",
    record_count: records.length,
    change_count: changeRecords.length,
    unit_bearing_change_count: unitBearingChangeCount,
    dimensionless_change_count: dimensionlessChangeCount,
    unit_validation_statuses: validationStatuses,
    conversion_performed: false,
    applied_receipt_units: "not_serialized_in_review_ledger",
    source: "queued_operation_intents_and_agent_proposals"
  };
}

function operationLedgerUnitPolicySummary(ledger: ReturnType<typeof buildOperationReviewLedger>): string {
  const evidence = ledger.unit_policy_evidence;
  return [
    `records=${evidence.record_count}`,
    `unit_bearing_changes=${evidence.unit_bearing_change_count}`,
    `dimensionless_changes=${evidence.dimensionless_change_count}`,
    `unit_validations=${evidence.unit_validation_statuses.length > 0 ? evidence.unit_validation_statuses.join(",") : "none"}`,
    `receipt_units=${evidence.applied_receipt_units}`,
    `conversion=${String(evidence.conversion_performed)}`
  ].join("; ");
}

function applicationStatus(validationOutcome: { application_status?: string }): string {
  return validationOutcome.application_status ?? "TBD";
}

function ledgerBoundary(ledger: ReturnType<typeof buildOperationReviewLedger>): string {
  if (
    ledger.audit_posture.default_user_acceptance_required &&
    ledger.audit_posture.preview_records_do_not_apply_operations &&
    ledger.accepted_model_state_unchanged &&
    !ledger.private_payload_included &&
    !ledger.protected_content_included &&
    !ledger.release_or_professional_claim &&
    ledger.professional_boundary.human_review_required &&
    !ledger.professional_boundary.software_makes_compliance_claim &&
    !ledger.professional_boundary.software_makes_approval_claim
  ) {
    return "review-only; requires explicit user acceptance; does not mutate accepted model state; no compliance or professional approval claim";
  }
  return "operation ledger boundary requires attention";
}

function jsonDataHref(value: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(value, null, 2))}`;
}

function safeRefToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9:_-]+/g, "-").replace(/^-+|-+$/g, "") || "operation";
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9_-]+/g, "-").replace(/^-+|-+$/g, "") || "preview";
}
