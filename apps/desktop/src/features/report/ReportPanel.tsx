import { Download, FileText } from "lucide-react";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  DesignKnowledge,
  Diagnostic,
  EditorOperationIntent,
  LocalProjectSummary,
  LocalStorageCapability,
  MechanicsResult,
  PreviewModel,
  PreviewComparison,
  SelectedReviewTarget
} from "../../types";

export function ReportPanel({
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
  const resultRefs = result ? selectedResultRefs(result) : [];
  const diagnostics = result ? reportDiagnostics({ model, knowledge, result }) : [];
  const severitySummary = countDiagnosticsBySeverity(diagnostics);
  const run = analysisRun?.analysis_run;
  const resultHashCount = run?.result_refs.reduce((count, item) => count + item.hash_refs.length, 0) ?? 0;
  const envelopeHash = run?.hashes.find((item) => item.payload_scope === "result_envelope");
  const loadBasisRefs = run?.load_basis_refs?.map((item) => item.ref).join(", ") ?? "not generated";
  const persistenceEvidence = reportPersistenceEvidence({
    model,
    result,
    analysisRun,
    editorIntents,
    projectOperation,
    projectSummary,
    proposal,
    storageCapability
  });
  const exportPacket = result
    ? reportExportPacket({
        model,
        result,
        analysisRun,
        comparison,
        editorIntents,
        persistenceEvidence,
        proposal,
        selectedReviewTarget,
        resultRefs,
        diagnostics
      })
    : null;
  return (
    <section className="panel report-panel" aria-label="Report packet" data-testid="report-panel">
      <div className="panel-title">
        <FileText size={16} />
        Report Packet
      </div>
      {result ? (
        <>
          <div className="report-actions">
            <a
              className="report-export-link"
              data-testid="report-export-link"
              download={`openpipestress-preview-report-${safeFileToken(result.run_id)}.json`}
              href={exportPacket ? jsonDataHref(exportPacket) : "#"}
            >
              <Download size={14} aria-hidden="true" />
              Local JSON
            </a>
            <span data-testid="report-export-summary">
              {exportPacket
                ? `${exportPacket.selected_result_refs.length} refs; ${exportPacket.diagnostic_refs.length} diagnostics; no private payload`
                : "not available"}
            </span>
          </div>
          <div className="report-list" data-testid="report-packet-body">
            <ReportLine label="Model" value={result.model_ref} />
            <ReportLine label="Mechanics" value={result.status.mechanics.replaceAll("_", " ")} />
            <ReportLine label="Rule check" value={result.status.rule_check.replaceAll("_", " ")} />
            <ReportLine label="Professional acceptance" value={result.status.professional_acceptance.replaceAll("_", " ")} />
            <ReportLine label="Selected result refs" value={resultRefs.join(", ")} testId="report-selected-result-refs" />
            <ReportLine
              label="Selected review target"
              value={formatSelectedReviewTarget(selectedReviewTarget)}
              testId="report-selected-review-target"
            />
            <ReportLine
              label="Diagnostic scope"
              value="model, design knowledge, and computed mechanics findings"
              testId="report-diagnostic-scope"
            />
            <ReportLine label="Diagnostics" value={`${diagnostics.length} review finding${diagnostics.length === 1 ? "" : "s"}`} />
            <ReportLine
              label="Diagnostic severity"
              value={formatDiagnosticSeverity(severitySummary)}
              testId="report-diagnostic-summary"
            />
            {run ? (
              <>
                <ReportLine label="Analysis run" value={`${analysisRun.deliverable_id}; ${run.run_id}`} testId="report-analysis-run" />
                <ReportLine label="Load basis refs" value={loadBasisRefs} testId="report-load-basis-refs" />
                <ReportLine label="Run immutability" value={run.immutability_policy.mutation_policy.replaceAll("_", " ")} />
                <ReportLine label="Result hashes" value={`${resultHashCount} result value hash${resultHashCount === 1 ? "" : "es"}`} />
                <ReportLine label="Envelope hash" value={envelopeHash ? `${envelopeHash.algorithm}; ${envelopeHash.payload_scope}` : "not available"} />
                <ReportLine label="Run boundary" value={boundarySummary(run.professional_boundary)} />
              </>
            ) : null}
            {comparison ? (
              <ReportLine
                label="Comparison"
                value={formatComparisonSummary(comparison)}
                testId="report-comparison-summary"
              />
            ) : null}
            <ReportLine
              label="Project persistence"
              value={formatPersistenceSummary(persistenceEvidence)}
              testId="report-project-persistence"
            />
            <ReportLine
              label="Export readiness"
              value={formatExportInventorySummary(persistenceEvidence)}
              testId="report-export-readiness"
            />
            <ReportLine
              label="Storage boundary"
              value={formatStorageBoundary(persistenceEvidence)}
              testId="report-storage-boundary"
            />
            <ReportLine
              label="Editor intents"
              value={formatEditorIntentSummary(editorIntents)}
              testId="report-editor-intent-summary"
            />
            {editorIntents[0] ? (
              <>
                <ReportLine
                  label="Latest editor intent"
                  value={formatEditorIntentOperation(editorIntents[0])}
                  testId="report-editor-intent-operation"
                />
                <ReportLine
                  label="Editor intent boundary"
                  value={editorIntentBoundarySummary(editorIntents[0])}
                  testId="report-editor-intent-boundary"
                />
              </>
            ) : null}
            <ReportLine label="Proposal" value={proposal?.proposal_id ?? "not generated"} />
            {proposal ? (
              <>
                <ReportLine
                  label="Proposal operation"
                  value={formatProposalOperation(proposal)}
                  testId="report-proposal-operation"
                />
                <ReportLine
                  label="Proposal boundary"
                  value={proposalBoundarySummary(proposal)}
                  testId="report-proposal-boundary"
                />
              </>
            ) : null}
            <ReportLine label="Boundary" value="technical preview only; human review required; no compliance or professional approval claim" />
          </div>
        </>
      ) : (
        <p className="muted">
          Run the bounded preview mechanics path to assemble a report packet from computed result and diagnostic IDs.
        </p>
      )}
      <small className="report-note">
        Uses invented or cleared preview data for {model.project.id}; private rule criteria and professional acceptance are not bundled.
      </small>
    </section>
  );
}

function ReportLine({ label, value, testId }: { label: string; value: string; testId?: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function selectedResultRefs(result: MechanicsResult): string[] {
  return [
    result.summary.max_displacement?.result_ref,
    result.summary.max_open_formula_stress?.result_ref,
    result.results.find((item) => item.id === "result:force:pipe-P-120:axial")?.id,
    result.results.find((item) => item.id === "result:force:pipe-P-120:axial:end-j")?.id,
    result.results.find((item) => item.id === "result:force:pipe-P-120:midspan:axial")?.id,
    result.results.find((item) => item.id === "result:force:pipe-P-120:quarter-1:shear-y")?.id,
    result.results.find((item) => item.id === "result:force:pipe-P-120:shear-y")?.id,
    result.results.find((item) => item.id === "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial")?.id,
    result.results.find((item) => item.id === "result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y")?.id,
    result.results.find((item) => item.id === "result:stress:pipe-P-120:end-j:torsional-shear")?.id,
    result.results.find((item) => item.id === "result:stress:pipe-P-120:quarter-1:torsional-shear")?.id
  ].filter((value): value is string => Boolean(value));
}

function reportDiagnostics({
  model,
  knowledge,
  result
}: {
  model: PreviewModel;
  knowledge: DesignKnowledge | null;
  result: MechanicsResult;
}): Diagnostic[] {
  return [...model.diagnostics, ...(knowledge?.diagnostics ?? []), ...result.diagnostics];
}

function countDiagnosticsBySeverity(diagnostics: Diagnostic[]): Record<Diagnostic["severity"], number> {
  return diagnostics.reduce(
    (counts, item) => {
      counts[item.severity] += 1;
      return counts;
    },
    { blocking: 0, error: 0, warning: 0, info: 0 }
  );
}

function formatDiagnosticSeverity(counts: Record<Diagnostic["severity"], number>): string {
  return `${counts.warning} warning${counts.warning === 1 ? "" : "s"}; ${counts.info} info; ${counts.error} error${counts.error === 1 ? "" : "s"}; ${counts.blocking} blocking`;
}

function formatSelectedReviewTarget(selectedReviewTarget: SelectedReviewTarget | null): string {
  if (!selectedReviewTarget) return "not selected";
  return `${selectedReviewTarget.target_type}: ${selectedReviewTarget.id}`;
}

function boundarySummary(boundary: Record<string, boolean>): string {
  if (
    boundary.human_review_required &&
    !boundary.software_makes_compliance_claim &&
    !boundary.software_makes_certification_claim &&
    !boundary.software_makes_sealing_claim &&
    !boundary.software_makes_approval_claim
  ) {
    return "human review required; no compliance, certification, sealing, or approval claim";
  }
  return "review boundary requires attention";
}

function formatProposalOperation(proposal: AgentProposal): string {
  return [
    proposal.operation.operation_id,
    proposal.operation.operation_kind,
    proposal.operation.operation_status,
    proposal.validation.application_status
  ].join("; ");
}

function proposalBoundarySummary(proposal: AgentProposal): string {
  if (
    proposal.audit_boundary.requires_user_acceptance &&
    !proposal.audit_boundary.mutates_accepted_model_state &&
    proposal.audit_boundary.acceptance_recorded_as_review_only &&
    proposal.professional_boundary.human_review_required &&
    !proposal.professional_boundary.software_makes_compliance_claim &&
    !proposal.professional_boundary.software_makes_approval_claim
  ) {
    return "review-only; requires user acceptance; does not mutate accepted model state; no compliance or professional approval claim";
  }
  return "proposal boundary requires attention";
}

function reportExportPacket({
  model,
  result,
  analysisRun,
  comparison,
  editorIntents,
  persistenceEvidence,
  proposal,
  selectedReviewTarget,
  resultRefs,
  diagnostics
}: {
  model: PreviewModel;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope | null;
  comparison: PreviewComparison | null;
  editorIntents: EditorOperationIntent[];
  persistenceEvidence: ReturnType<typeof reportPersistenceEvidence>;
  proposal: AgentProposal | null;
  selectedReviewTarget: SelectedReviewTarget | null;
  resultRefs: string[];
  diagnostics: Diagnostic[];
}) {
  const run = analysisRun?.analysis_run;
  const diagnosticSummary = countDiagnosticsBySeverity(diagnostics);
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.report_packet_export",
    export_scope: "local_browser_download_preview",
    deliverable_refs: [
      "DEL-08-01",
      "DEL-08-03",
      "DEL-08-04",
      "DEL-08-06",
      "DEL-10-03",
      "DEL-10-04",
      "DEL-10-05",
      "DEL-15-04",
      "DEL-17-04",
      "DEL-17-05",
      "DEL-17-06",
      "DEL-17-07",
      "DEL-17-08",
      "DEL-17-09",
      "DEL-02-05",
      "DEL-12-01",
      "DEL-12-02"
    ],
    model_ref: result.model_ref,
    project_ref: model.project.id,
    analysis_run_ref: run?.run_id ?? "not generated",
    deliverable_ref: analysisRun?.deliverable_id ?? "not generated",
    selected_result_refs: resultRefs,
    selected_review_target: selectedReviewTarget,
    diagnostic_refs: diagnostics.map((item) => item.id ?? item.code),
    diagnostic_summary: {
      total: diagnostics.length,
      by_severity: diagnosticSummary
    },
    load_basis_refs: run?.load_basis_refs.map((item) => item.ref) ?? [],
    hash_refs: {
      result_value_count: run?.result_refs.reduce((count, item) => count + item.hash_refs.length, 0) ?? 0,
      envelope_hash_scopes: run?.hashes.map((item) => item.payload_scope) ?? []
    },
    run_audit: run ? runAuditExport({ analysisRun, result }) : null,
    comparison_ref: comparison?.comparison_id ?? "not generated",
    comparison_summary: comparison ? comparisonExportSummary(comparison) : null,
    comparison_top_deltas: comparison?.result_deltas.slice(0, 8) ?? [],
    comparison_professional_boundary: comparison?.professional_boundary ?? null,
    persistence_evidence: persistenceEvidence,
    editor_intent_refs: editorIntents.map((intent) => intent.operation_id),
    editor_intent_summary: editorIntentExportSummary(editorIntents),
    editor_operation_intents: editorIntents.map(editorIntentExport),
    proposal_ref: proposal?.proposal_id ?? "not generated",
    proposal_operation: proposal ? proposalOperationExport(proposal) : null,
    data_boundary: model.data_boundary,
    professional_boundary: run?.professional_boundary ?? {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false
  };
}

function reportPersistenceEvidence({
  model,
  result,
  analysisRun,
  editorIntents,
  projectOperation,
  projectSummary,
  proposal,
  storageCapability
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  editorIntents: EditorOperationIntent[];
  projectOperation: string;
  projectSummary: LocalProjectSummary | null;
  proposal: AgentProposal | null;
  storageCapability: LocalStorageCapability | null;
}) {
  const run = analysisRun?.analysis_run;
  const operationRecordCount = editorIntents.length + (proposal ? 1 : 0);
  const mechanicsExportsReady = Boolean(result && run);
  const storageMode = projectSummary?.storage_mode ?? "not_persisted_this_session";
  const validationStatus = projectSummary ? "preview_current" : "preview_not_persisted";
  const versionCheckStatus =
    model.schema_version === "0.1.0" ? "supported_current_schema" : "unsupported_schema_review_required";
  const readinessByExportId = {
    project_storage_audit: "available",
    project_validation_preflight: "available",
    build_package_readiness: "available",
    result_envelope: mechanicsExportsReady ? "available" : "pending_mechanics_run",
    stress_neutral_csv_json_package: mechanicsExportsReady ? "available" : "pending_mechanics_run",
    headless_runner_envelope: "available",
    adapter_framework_envelope: "available",
    local_fea_handoff_package: mechanicsExportsReady ? "available" : "pending_mechanics_run",
    external_prover_boundary_metadata: "available",
    review_geometry_export: "available",
    conservative_pcf_export: "available",
    caepipe_mbf_export: "available",
    caepipe_external_run_evidence: "available",
    export_adapter_sdk_registry: "available",
    native_json_package: mechanicsExportsReady ? "available" : "pending_mechanics_run",
    report_packet: mechanicsExportsReady ? "available" : "pending_mechanics_run",
    report_protected_content_lint: "available",
    handoff_package: mechanicsExportsReady ? "available" : "pending_mechanics_run",
    operation_review_ledger: operationRecordCount > 0 ? "available" : "empty_operation_queue"
  };
  const availableCount = Object.values(readinessByExportId).filter((value) => value === "available").length;

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.report_persistence_export_context",
    source_export_manifest_ref: "openpipestress.technical_preview.export_review_manifest",
    storage_audit: {
      document_kind: "openpipestress.technical_preview.local_project_persistence_audit",
      deliverable_refs: ["DEL-02-05", "DEL-12-01", "DEL-12-02"],
      project_ref: model.project.id,
      persisted_project_ref: projectSummary?.project_id ?? "not_persisted_this_session",
      last_operation: projectOperation,
      storage_engine: storageCapability?.engine ?? "storage_check_pending",
      storage_mode: storageMode,
      pending_operation_count: editorIntents.length,
      copied_external_files: Boolean(projectSummary?.copied_external_files),
      accepted_model_state_mutated: false
    },
    validation_preflight: {
      document_kind: "openpipestress.technical_preview.project_validation_preflight",
      deliverable_refs: ["DEL-02-05", "DEL-12-01"],
      validation_status: validationStatus,
      version_check_status: versionCheckStatus,
      migration_status: projectSummary?.migration_status ?? "not_persisted_this_session",
      round_trip_status: "semantic_categories_declared",
      accepted_model_state_mutated: false
    },
    export_inventory: {
      expected_export_count: Object.keys(readinessByExportId).length,
      available_count: availableCount,
      readiness_by_export_id: readinessByExportId,
      operation_record_count: operationRecordCount
    },
    boundary: {
      export_scope: "local_browser_download_preview",
      local_only_project_store: true,
      repository_default_private_write: false,
      external_file_copy_performed: Boolean(projectSummary?.copied_external_files),
      network_required: Boolean(storageCapability?.network_required),
      daemon_required: Boolean(storageCapability?.daemon_required),
      telemetry_enabled: Boolean(storageCapability?.telemetry_enabled),
      accepted_model_state_mutated: false,
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false
    }
  };
}

function formatPersistenceSummary(evidence: ReturnType<typeof reportPersistenceEvidence>): string {
  return [
    `storage=${evidence.storage_audit.storage_mode}`,
    `validation=${evidence.validation_preflight.validation_status}`,
    `round_trip=${evidence.validation_preflight.round_trip_status}`
  ].join("; ");
}

function formatExportInventorySummary(evidence: ReturnType<typeof reportPersistenceEvidence>): string {
  return [
    `${evidence.export_inventory.available_count} of ${evidence.export_inventory.expected_export_count} local exports ready`,
    `storage=${evidence.export_inventory.readiness_by_export_id.project_storage_audit}`,
    `validation=${evidence.export_inventory.readiness_by_export_id.project_validation_preflight}`
  ].join("; ");
}

function formatStorageBoundary(evidence: ReturnType<typeof reportPersistenceEvidence>): string {
  return [
    `network=${String(evidence.boundary.network_required)}`,
    `telemetry=${String(evidence.boundary.telemetry_enabled)}`,
    `private/protected payload=false`,
    `accepted_state_mutated=${String(evidence.boundary.accepted_model_state_mutated)}`
  ].join("; ");
}

function runAuditExport({
  analysisRun,
  result
}: {
  analysisRun: AnalysisRunEnvelope | null;
  result: MechanicsResult;
}) {
  const run = analysisRun?.analysis_run;
  if (!run) return null;
  return {
    deliverable_id: analysisRun.deliverable_id,
    package_id: analysisRun.package_id,
    model_state_ref: run.model_state_ref,
    analysis_run_ref: {
      object_type: "AnalysisRun",
      ref: run.run_id
    },
    run_kind: run.run_kind,
    analysis_status: run.analysis_status,
    result_row_count: result.results.length,
    result_ref_count: run.result_refs.length,
    result_value_hash_count: run.result_refs.reduce((count, item) => count + item.hash_refs.length, 0),
    hash_scopes: run.hashes.map((item) => item.payload_scope),
    input_manifest_refs: run.reproducibility.input_manifest_refs,
    determinism_notes: run.reproducibility.determinism_notes,
    unresolved_tbd: run.reproducibility.unresolved_tbd,
    immutability_policy: run.immutability_policy,
    professional_boundary: run.professional_boundary
  };
}

function proposalOperationExport(proposal: AgentProposal) {
  return {
    proposal_id: proposal.proposal_id,
    prompt: proposal.prompt,
    operation_id: proposal.operation.operation_id,
    operation_kind: proposal.operation.operation_kind,
    operation_status: proposal.operation.operation_status,
    affected_entity_ids: proposal.operation.affected_entity_ids,
    changes: proposal.operation.changes,
    rationale: proposal.rationale,
    assumptions: proposal.assumptions,
    validation: proposal.validation,
    audit_boundary: proposal.audit_boundary,
    professional_boundary: proposal.professional_boundary
  };
}

function formatEditorIntentSummary(editorIntents: EditorOperationIntent[]): string {
  if (editorIntents.length === 0) return "none queued";
  const latest = editorIntents[0];
  return `${editorIntents.length} queued; latest=${latest.operation_id}; ${latest.validation.application_status}`;
}

function formatEditorIntentOperation(intent: EditorOperationIntent): string {
  return [
    intent.queue_id ?? "unqueued",
    intent.operation_id,
    intent.operation_kind,
    intent.operation_status,
    `${intent.target.object_type}:${intent.target.ref}`,
    `${intent.change.field_path}=${intent.change.after}`,
    intent.validation.application_status
  ].join("; ");
}

function editorIntentBoundarySummary(intent: EditorOperationIntent): string {
  if (
    intent.audit_boundary.requires_user_acceptance &&
    !intent.audit_boundary.direct_model_mutation_allowed &&
    !intent.audit_boundary.mutates_accepted_model_state &&
    intent.professional_boundary.human_review_required &&
    !intent.professional_boundary.software_makes_compliance_claim &&
    !intent.professional_boundary.software_makes_approval_claim
  ) {
    return "review-only; requires user acceptance; does not mutate accepted model state; no compliance or professional approval claim";
  }
  return "editor intent boundary requires attention";
}

function editorIntentExportSummary(editorIntents: EditorOperationIntent[]) {
  return {
    queued_count: editorIntents.length,
    latest_operation_id: editorIntents[0]?.operation_id ?? "not queued",
    application_statuses: [...new Set(editorIntents.map((intent) => intent.validation.application_status))],
    requires_user_acceptance: editorIntents.some((intent) => intent.audit_boundary.requires_user_acceptance),
    mutates_accepted_model_state: editorIntents.some((intent) => intent.audit_boundary.mutates_accepted_model_state),
    direct_model_mutation_allowed: editorIntents.some((intent) => intent.audit_boundary.direct_model_mutation_allowed),
    release_or_professional_claim: false
  };
}

function editorIntentExport(intent: EditorOperationIntent) {
  return {
    queue_id: intent.queue_id ?? "unqueued",
    operation_id: intent.operation_id,
    operation_kind: intent.operation_kind,
    operation_status: intent.operation_status,
    author_type: intent.author_type,
    target: intent.target,
    change: intent.change,
    validation: intent.validation,
    audit_boundary: intent.audit_boundary,
    professional_boundary: intent.professional_boundary,
    rationale: intent.rationale,
    accepted_model_state_mutated: false,
    protected_content_included: false,
    private_payload_included: false,
    release_or_professional_claim: false
  };
}

function formatComparisonSummary(comparison: PreviewComparison): string {
  return [
    comparison.comparison_id,
    `${comparison.summary.comparable_result_pairs} mapped pairs`,
    `${comparison.summary.unmatched_left_results} reference-only`,
    `${comparison.summary.unmatched_right_results} target-only`,
    comparison.summary.tolerance_status,
    `profile=${comparison.summary.tolerance_profile_ref}`
  ].join("; ");
}

function comparisonExportSummary(comparison: PreviewComparison) {
  return {
    comparison_id: comparison.comparison_id,
    deliverable_id: comparison.deliverable_id,
    package_id: comparison.package_id,
    comparison_kind: comparison.comparison_kind,
    left_basis_ref: comparison.left.basis_ref.ref,
    right_basis_ref: comparison.right.basis_ref.ref,
    comparable_result_pairs: comparison.summary.comparable_result_pairs,
    unmatched_left_results: comparison.summary.unmatched_left_results,
    unmatched_right_results: comparison.summary.unmatched_right_results,
    mapping_basis: comparison.summary.mapping_basis,
    tolerance_status: comparison.summary.tolerance_status,
    tolerance_profile_ref: comparison.summary.tolerance_profile_ref,
    diagnostic_refs: comparison.diagnostics.map((item) => item.id ?? item.code),
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false
  };
}

function jsonDataHref(value: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(value, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9_-]+/g, "-").replace(/^-+|-+$/g, "") || "preview";
}
