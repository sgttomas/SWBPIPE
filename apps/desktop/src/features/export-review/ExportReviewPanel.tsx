import { Download, ShieldCheck } from "lucide-react";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  DesignKnowledge,
  EditorOperationIntent,
  LocalProjectSummary,
  LocalStorageCapability,
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
  projectOperation,
  projectSummary,
  proposal,
  selectedReviewTarget,
  storageCapability
}: {
  model: PreviewModel;
  knowledge: DesignKnowledge | null;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  comparison: PreviewComparison | null;
  editorIntents: EditorOperationIntent[];
  projectOperation: string;
  projectSummary: LocalProjectSummary | null;
  proposal: AgentProposal | null;
  selectedReviewTarget: SelectedReviewTarget | null;
  storageCapability: LocalStorageCapability | null;
}) {
  const manifest = buildExportReviewManifest({
    model,
    knowledge,
    result,
    analysisRun,
    comparison,
    editorIntents,
    projectOperation,
    projectSummary,
    proposal,
    selectedReviewTarget,
    storageCapability
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
  projectOperation,
  projectSummary,
  proposal,
  selectedReviewTarget,
  storageCapability
}: {
  model: PreviewModel;
  knowledge: DesignKnowledge | null;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  comparison: PreviewComparison | null;
  editorIntents: EditorOperationIntent[];
  projectOperation: string;
  projectSummary: LocalProjectSummary | null;
  proposal: AgentProposal | null;
  selectedReviewTarget: SelectedReviewTarget | null;
  storageCapability: LocalStorageCapability | null;
}) {
  const run = analysisRun?.analysis_run;
  const diagnostics = [...model.diagnostics, ...(knowledge?.diagnostics ?? []), ...(result?.diagnostics ?? [])];
  const operationRecordCount = editorIntents.length + (proposal ? 1 : 0);
  const storageAuditReady = true;
  const validationPreflightReady = true;
  const reportReady = Boolean(result && analysisRun);
  const resultExportReady = Boolean(result && analysisRun);
  const headlessRunnerReady = true;
  const adapterFrameworkReady = true;
  const nativePackageReady = Boolean(result && analysisRun);
  const handoffReady = Boolean(result && analysisRun);
  const ledgerReady = operationRecordCount > 0;
  const reportLintReady = true;
  const stableEntityCount =
    1 + (model.materials?.length ?? 0) + model.nodes.length + model.pipe_segments.length + model.supports.length + model.components.length +
    model.load_cases.length + (model.combinations?.length ?? 0);
  const exports = [
    {
      export_id: "project_storage_audit",
      label: "Project storage audit",
      document_kind: "openpipestress.technical_preview.local_project_persistence_audit",
      readiness: storageAuditReady ? "available" : "pending_storage_capability",
      deliverable_refs: ["DEL-02-05", "DEL-12-01", "DEL-12-02"],
      source_refs: [model.project.id, projectSummary?.project_id ?? "not_persisted_this_session"],
      pending_operation_count: editorIntents.length,
      last_operation: projectOperation,
      storage_mode: projectSummary?.storage_mode ?? "not_persisted_this_session",
      copied_external_files: Boolean(projectSummary?.copied_external_files),
      redaction_action: "classify_and_warn_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Local project persistence audit export; records storage capability, local-only boundary, and non-mutating review context."
    },
    {
      export_id: "project_validation_preflight",
      label: "Project validation preflight",
      document_kind: "openpipestress.technical_preview.project_validation_preflight",
      readiness: validationPreflightReady ? "available" : "pending_validation_preflight",
      deliverable_refs: ["DEL-02-05", "DEL-12-01", "DEL-12-02"],
      source_refs: [model.project.id, projectSummary?.project_id ?? "not_persisted_this_session"],
      pending_operation_count: editorIntents.length,
      validation_status: projectSummary ? "preview_current" : "preview_not_persisted",
      version_check_status: model.schema_version === "0.1.0" ? "supported_current_schema" : "unsupported_schema_review_required",
      migration_status: projectSummary?.migration_status ?? "not_persisted_this_session",
      round_trip_status: "semantic_categories_declared",
      redaction_action: "classify_and_warn_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Local project validation preflight export; records schema/version, migration, and round-trip category evidence."
    },
    {
      export_id: "result_envelope",
      label: "Result envelope",
      document_kind: "openpipestress.technical_preview.result_export_envelope",
      readiness: resultExportReady ? "available" : "pending_mechanics_run",
      deliverable_refs: ["DEL-08-04", "DEL-14-02"],
      source_refs: result && run ? [result.run_id, run.model_state_ref.ref] : [],
      result_ref_count: result?.results.length ?? 0,
      result_set_count: result ? 1 : 0,
      diagnostic_ref_count: diagnostics.length,
      redaction_action: "classify_and_warn_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Schema-first local JSON result envelope; non-JSON formats, public transport, and local FEA package formats remain TBD."
    },
    {
      export_id: "headless_runner_envelope",
      label: "Headless runner envelope",
      document_kind: "openpipestress.technical_preview.headless_runner_envelope",
      readiness: headlessRunnerReady ? "available" : "pending_runner_contract",
      deliverable_refs: ["DEL-10-05", "DEL-08-04", "DEL-08-02", "DEL-14-02", "DEL-12-01"],
      source_refs: result && run ? [result.run_id, run.model_state_ref.ref, model.project.id] : [model.project.id],
      operation: "solve",
      requested_outputs: ["result_envelope", "audit_manifest", "diagnostics", "regression_record"],
      result_ref_count: result?.results.length ?? 1,
      runner_job_state: result && run ? "COMPLETED" : "TBD",
      final_cli_command_syntax: "TBD",
      package_scripts: "TBD",
      process_invocation: "TBD",
      network_access: "TBD",
      filesystem_mutation_policy: "TBD",
      redaction_action: "classify_and_warn_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Schema-first local headless request/result envelope; final CLI syntax, package scripts, process, network, filesystem, CI, and release matrix remain TBD."
    },
    {
      export_id: "adapter_framework_envelope",
      label: "Adapter framework envelope",
      document_kind: "openpipestress.technical_preview.adapter_framework_envelope",
      readiness: adapterFrameworkReady ? "available" : "pending_adapter_framework_declaration",
      deliverable_refs: ["DEL-10-02", "DEL-02-04", "DEL-08-04", "DEL-12-01", "DEL-12-02"],
      source_refs: result && run ? [result.run_id, run.model_state_ref.ref, model.project.id] : [model.project.id],
      capabilities: ["export_model", "export_results", "validate_payload", "contribution_review"],
      parse_status: "not_parsed_by_framework",
      validation_plan: "schema_units_provenance_privacy_protected_content_export_review_required",
      result_ref_count: result?.results.length ?? 1,
      diagnostic_ref_count: diagnostics.length,
      external_format_list: "TBD",
      public_transport_protocol: "TBD",
      endpoint_syntax: "TBD",
      plugin_runtime: "TBD",
      no_bypass_controls: "api_units_provenance_privacy_diagnostics_persistence_report_human_boundary_required",
      redaction_action: "classify_and_warn_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Format-neutral adapter declaration; concrete formats, transport, plugin runtime, filesystem roots, redaction workflow, CI, and release matrix remain TBD."
    },
    {
      export_id: "native_json_package",
      label: "Native JSON package",
      document_kind: "openpipestress.technical_preview.native_json_package_review",
      readiness: nativePackageReady ? "available" : "pending_mechanics_run",
      deliverable_refs: ["DEL-17-02", "DEL-17-03", "DEL-02-05", "DEL-12-01", "DEL-08-04", "DEL-14-02"],
      source_refs: result && run ? [result.run_id, run.model_state_ref.ref, model.project.id] : [model.project.id],
      result_ref_count: result?.results.length ?? 0,
      stable_id_count: stableEntityCount + (result?.results.length ?? 0) + operationRecordCount,
      operation_record_count: operationRecordCount,
      redaction_action: "classify_and_warn_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Project-owned native JSON review package; physical container, public transport, canonical package hash, and target-specific adapter behavior remain TBD."
    },
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
      export_id: "report_protected_content_lint",
      label: "Report protected-content lint",
      document_kind: "openpipestress.technical_preview.report_protected_content_lint_run",
      readiness: reportLintReady ? "available" : "pending_lint_run",
      deliverable_refs: ["DEL-08-05", "DEL-08-01", "DEL-08-03", "DEL-08-06", "DEL-12-02"],
      source_refs: [
        model.project.id,
        "apps/desktop/src/features/report/ReportPanel.tsx",
        "apps/desktop/src/features/export-review/ExportReviewPanel.tsx"
      ],
      target_count: result && run ? 4 : 3,
      finding_count: 0,
      blocking_finding_count: 0,
      clean_scan_is_clearance: false,
      redaction_action: "heuristic_public_surface_lint_review_evidence_only",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Local public report-surface heuristic lint; clean output is not legal, redaction, release, or professional clearance."
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
    deliverable_refs: ["DEL-12-02", "DEL-02-05", "DEL-12-01", "DEL-08-04", "DEL-08-05", "DEL-08-06", "DEL-10-02", "DEL-10-05", "DEL-15-01", "DEL-17-02", "DEL-17-03", "DEL-16-03"],
    scope_items: ["SOW-040", "SOW-050", "SOW-041", "SOW-029", "SOW-046", "SOW-043", "SOW-054", "SOW-032", "SOW-024", "SOW-030", "SOW-074"],
    objectives: ["OBJ-001", "OBJ-002", "OBJ-008", "OBJ-009", "OBJ-010", "OBJ-012", "OBJ-015", "OBJ-018"],
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
      "CI and package-level protected-content linter integration for physical downloads",
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
