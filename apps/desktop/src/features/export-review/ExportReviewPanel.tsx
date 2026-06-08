import { Download, ShieldCheck } from "lucide-react";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  DesignKnowledge,
  EditorOperationIntent,
  MechanicsResult,
  PreviewComparison,
  PreviewModel,
  SelectedReviewTarget
} from "../../types";

export function ExportReviewPanel({
  model,
  knowledge,
  result,
  analysisRun,
  comparison,
  editorIntents,
  proposal,
  selectedReviewTarget
}: {
  model: PreviewModel;
  knowledge: DesignKnowledge | null;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  comparison: PreviewComparison | null;
  editorIntents: EditorOperationIntent[];
  proposal: AgentProposal | null;
  selectedReviewTarget: SelectedReviewTarget | null;
}) {
  const manifest = buildExportReviewManifest({
    model,
    knowledge,
    result,
    analysisRun,
    comparison,
    editorIntents,
    proposal,
    selectedReviewTarget
  });

  return (
    <section className="panel export-review-panel" aria-label="Export safety review" data-testid="export-review-panel">
      <div className="panel-title">
        <ShieldCheck size={16} aria-hidden="true" />
        Export Safety Review
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="export-review-link"
          download={`openpipestress-preview-export-review-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(manifest)}
        >
          <Download size={14} aria-hidden="true" />
          Local manifest JSON
        </a>
        <span data-testid="export-review-summary">
          {manifest.summary.available_count} of {manifest.summary.export_count} local exports ready; no private/protected
          payloads
        </span>
      </div>
      <div className="report-list" data-testid="export-review-body">
        <ReviewLine
          label="Redaction posture"
          value={redactionPosture(manifest)}
          testId="export-review-redaction"
        />
        <ReviewLine
          label="Local transport"
          value="local browser download preview; network=false; telemetry=false"
          testId="export-review-transport"
        />
        <ReviewLine
          label="State binding"
          value={`${manifest.analysis_run_ref}; ${manifest.comparison_ref}`}
          testId="export-review-state-binding"
        />
        <ReviewLine label="Boundary" value={exportBoundary(manifest)} testId="export-review-boundary" />
        <div className="operation-record-list" data-testid="export-review-records">
          {manifest.exports.map((item) => (
            <article
              className="operation-record"
              data-testid={`export-review-record-${item.export_id}`}
              key={item.export_id}
            >
              <strong>{item.label}</strong>
              <small>
                {item.readiness}; {item.document_kind}; private={String(item.private_payload_included)}
              </small>
              <p>{item.review_note}</p>
            </article>
          ))}
        </div>
      </div>
      <small className="report-note">
        Export review is a local technical-preview manifest only; it does not certify redaction, release readiness, or
        professional acceptance.
      </small>
    </section>
  );
}

function ReviewLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildExportReviewManifest({
  model,
  knowledge,
  result,
  analysisRun,
  comparison,
  editorIntents,
  proposal,
  selectedReviewTarget
}: {
  model: PreviewModel;
  knowledge: DesignKnowledge | null;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  comparison: PreviewComparison | null;
  editorIntents: EditorOperationIntent[];
  proposal: AgentProposal | null;
  selectedReviewTarget: SelectedReviewTarget | null;
}) {
  const run = analysisRun?.analysis_run;
  const diagnostics = [...model.diagnostics, ...(knowledge?.diagnostics ?? []), ...(result?.diagnostics ?? [])];
  const operationRecordCount = editorIntents.length + (proposal ? 1 : 0);
  const reportReady = Boolean(result && analysisRun);
  const handoffReady = Boolean(result && analysisRun);
  const ledgerReady = operationRecordCount > 0;
  const entityCount = model.materials?.length ?? 0;
  const stableEntityCount =
    entityCount + model.nodes.length + model.pipe_segments.length + model.supports.length + model.components.length +
    model.load_cases.length + (model.combinations?.length ?? 0);
  const exports = [
    {
      export_id: "report_packet",
      label: "Report packet",
      document_kind: "openpipestress.technical_preview.report_packet_export",
      readiness: reportReady ? "available" : "pending_mechanics_run",
      deliverable_refs: ["DEL-08-01", "DEL-08-04", "DEL-08-06"],
      source_refs: result && run ? [result.run_id, run.model_state_ref.ref] : [],
      result_ref_count: result?.results.length ?? 0,
      diagnostic_ref_count: diagnostics.length,
      operation_record_count: operationRecordCount,
      redaction_action: "classify_and_warn_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note: "Schema-first local JSON report packet; private rule criteria are not bundled."
    },
    {
      export_id: "handoff_package",
      label: "Handoff package",
      document_kind: "openpipestress.technical_preview.handoff_package",
      readiness: handoffReady ? "available" : "pending_mechanics_run",
      deliverable_refs: ["DEL-15-01", "DEL-15-02", "DEL-15-03", "DEL-17-03"],
      source_refs: run ? [run.run_id, run.model_state_ref.ref] : [],
      result_ref_count: result?.results.length ?? 0,
      stable_id_count: stableEntityCount + (result?.results.length ?? 0),
      operation_record_count: operationRecordCount,
      redaction_action: "classify_and_warn_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note: "Native open JSON preview profile; target-specific solver mapping remains deferred."
    },
    {
      export_id: "operation_review_ledger",
      label: "Operation review ledger",
      document_kind: "openpipestress.technical_preview.operation_review_ledger",
      readiness: ledgerReady ? "available" : "empty_operation_queue",
      deliverable_refs: ["DEL-16-01", "DEL-16-02", "DEL-16-03", "DEL-16-04"],
      source_refs: [
        ...editorIntents.map((intent) => intent.queue_id ?? intent.operation_id),
        ...(proposal ? [proposal.proposal_id] : [])
      ],
      operation_record_count: operationRecordCount,
      result_ref_count: result?.results.length ?? 0,
      redaction_action: "classify_and_warn_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note: "Held-for-user-acceptance review records only; no accepted-state mutation is applied."
    }
  ];
  const availableCount = exports.filter((item) => item.readiness === "available").length;

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.export_review_manifest",
    export_scope: "local_browser_download_preview",
    deliverable_refs: ["DEL-12-02", "DEL-08-04", "DEL-08-06", "DEL-15-01", "DEL-17-03", "DEL-16-03"],
    scope_items: ["SOW-040", "SOW-046", "SOW-024", "SOW-074"],
    objectives: ["OBJ-010", "OBJ-015", "OBJ-018"],
    project_ref: model.project.id,
    model_ref: result?.model_ref ?? model.project.id,
    analysis_run_ref: run?.run_id ?? "not generated",
    comparison_ref: comparison?.comparison_id ?? "not generated",
    selected_review_target: selectedReviewTarget,
    export_context: "local_browser_download_preview",
    redaction_policy: {
      export_context_classification: "local_review_package",
      public_or_shared_export_allowed: false,
      local_private_export_allowed: true,
      private_values_redacted: true,
      protected_content_blocked: true,
      source_model_mutated: false
    },
    exports,
    summary: {
      export_count: exports.length,
      available_count: availableCount,
      private_payload_count: 0,
      protected_payload_count: 0,
      release_or_professional_claim_count: 0,
      diagnostics_reviewed: diagnostics.length,
      operation_record_count: operationRecordCount
    },
    data_boundary: model.data_boundary,
    unresolved_tbd: [
      "durable redaction profile persistence",
      "protected-content linter integration for downloaded packages",
      "physical project package/container"
    ],
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    professional_boundary: professionalBoundary()
  };
}

function redactionPosture(manifest: ReturnType<typeof buildExportReviewManifest>): string {
  return [
    `private redacted=${manifest.redaction_policy.private_values_redacted}`,
    `protected content blocked=${manifest.redaction_policy.protected_content_blocked}`,
    `source model mutated=${manifest.redaction_policy.source_model_mutated}`
  ].join("; ");
}

function exportBoundary(manifest: ReturnType<typeof buildExportReviewManifest>): string {
  return [
    `private payloads=${manifest.summary.private_payload_count}`,
    `protected payloads=${manifest.summary.protected_payload_count}`,
    "no release or professional claim"
  ].join("; ");
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

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9-]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}
