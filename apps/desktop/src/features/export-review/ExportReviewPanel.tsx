import { Download, ShieldCheck } from "lucide-react";
import { buildExportUnitSystemDisclosure, unitDisclosureSummary } from "../exportUnitDisclosure";
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
        <ReviewLine
          label="Units"
          value={`${manifest.unit_policy_summary.unit_system_ref.ref}; covered=${manifest.unit_policy_summary.summary.unit_evidence_present_count}/${manifest.unit_policy_summary.summary.unit_evidence_required_count}; ${unitDisclosureSummary(manifest.unit_policy_summary)}`}
          testId="export-review-units"
        />
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

const UNIT_EVIDENCE_REQUIRED_EXPORT_IDS = new Set([
  "project_storage_audit",
  "project_validation_preflight",
  "security_threat_model_review",
  "editor_contract_review",
  "agent_proposal_review",
  "missing_data_warning_blocking_review",
  "rule_completeness_review",
  "result_envelope",
  "stress_neutral_csv_json_package",
  "headless_runner_envelope",
  "adapter_framework_envelope",
  "local_fea_handoff_package",
  "external_prover_boundary_metadata",
  "review_geometry_export",
  "conservative_pcf_export",
  "caepipe_mbf_export",
  "caepipe_external_run_evidence",
  "export_adapter_sdk_registry",
  "native_json_package",
  "report_packet",
  "handoff_package",
  "operation_review_ledger"
]);

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
  const telemetryBoundaryReady = true;
  const secretPrivateLibraryReady = true;
  const securityThreatModelReady = true;
  const editorContractReady = true;
  const missingDataReviewReady = true;
  const agentProposalReviewReady = Boolean(proposal);
  const accessibilityBaselineReady = true;
  const designWorkspaceReady = true;
  const buildReadinessReady = true;
  const validationEvidenceReady = true;
  const reportReady = Boolean(result && analysisRun);
  const resultExportReady = Boolean(result && analysisRun);
  const stressNeutralReady = Boolean(result && analysisRun);
  const headlessRunnerReady = true;
  const adapterFrameworkReady = true;
  const localFeaHandoffReady = Boolean(result && analysisRun);
  const externalProverReady = true;
  const reviewGeometryReady = true;
  const pcfExportReady = true;
  const caepipeMbfExportReady = true;
  const caepipeExternalRunReady = true;
  const exportAdapterSdkReady = true;
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
      pending_operation_count: operationRecordCount,
      editor_intent_count: editorIntents.length,
      persisted_editor_intent_count: projectSummary?.editor_intent_count ?? 0,
      proposal_operation_count: proposal ? 1 : 0,
      persisted_proposal_count: projectSummary?.proposal_count ?? 0,
      persisted_selected_review_target_count: projectSummary?.selected_review_target_count ?? 0,
      persisted_selected_review_target_ref: projectSummary?.selected_review_target_ref ?? "not_selected",
      persisted_mechanics_result_count: projectSummary?.persisted_mechanics_result_count ?? 0,
      persisted_analysis_run_count: projectSummary?.persisted_analysis_run_count ?? 0,
      persisted_analysis_run_ref: projectSummary?.persisted_analysis_run_ref ?? "not_persisted",
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
      pending_operation_count: operationRecordCount,
      editor_intent_count: editorIntents.length,
      persisted_editor_intent_count: projectSummary?.editor_intent_count ?? 0,
      proposal_operation_count: proposal ? 1 : 0,
      persisted_proposal_count: projectSummary?.proposal_count ?? 0,
      persisted_selected_review_target_count: projectSummary?.selected_review_target_count ?? 0,
      persisted_selected_review_target_ref: projectSummary?.selected_review_target_ref ?? "not_selected",
      persisted_mechanics_result_count: projectSummary?.persisted_mechanics_result_count ?? 0,
      persisted_analysis_run_count: projectSummary?.persisted_analysis_run_count ?? 0,
      persisted_analysis_run_ref: projectSummary?.persisted_analysis_run_ref ?? "not_persisted",
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
      export_id: "telemetry_boundary_review",
      label: "Telemetry boundary review",
      document_kind: "openpipestress.technical_preview.telemetry_boundary_review",
      readiness: telemetryBoundaryReady ? "available" : "pending_telemetry_boundary_surface",
      deliverable_refs: ["DEL-12-03", "DEL-12-01", "DEL-12-02", "DEL-02-04", "DEL-10-02"],
      source_refs: [
        model.project.id,
        "docs/security/telemetry_policy.md",
        "core/security/telemetry_policy/",
        "tests/security/test_telemetry_policy.py",
        "apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx"
      ],
      config_resolution: "absent_or_preview_config_resolves_disabled",
      telemetry_enabled: Boolean(storageCapability?.telemetry_enabled),
      requested_enabled: false,
      explicit_opt_in: false,
      allowlist_approved: false,
      attempted_event_count: 4,
      allowed_event_count: 0,
      blocked_event_count: 4,
      payload_constructed: false,
      network_transport_initialized: false,
      endpoint_initialized: false,
      vendor_initialized: false,
      upload_queue_initialized: false,
      telemetry_persistence_initialized: false,
      no_bypass_surfaces: ["plugins", "adapters", "import_export", "reports", "private_libraries", "cli_runner"],
      open_decision_count: 6,
      security_certification_claim: false,
      redaction_action: "telemetry_metadata_only_no_payload_no_network",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "DEL-12-03 desktop telemetry boundary review; telemetry remains disabled/default-off, no payload or network behavior is initialized, and opt-in, consent surface, endpoint, vendor, retention, and allowlist decisions stay TBD."
    },
    {
      export_id: "secret_private_library_boundary_review",
      label: "Secret/private-library boundary review",
      document_kind: "openpipestress.technical_preview.secret_private_library_boundary_review",
      readiness: secretPrivateLibraryReady ? "available" : "pending_secret_private_library_surface",
      deliverable_refs: ["DEL-12-04", "DEL-12-01", "DEL-12-02", "DEL-12-03", "DEL-02-04", "DEL-10-02"],
      source_refs: [
        model.project.id,
        "docs/security/secret_private_library_handling.md",
        "core/security/secret_private_library/",
        "tests/security/test_secret_private_library_handling.py",
        "apps/desktop/src/features/secret-private-library/SecretPrivateLibraryPanel.tsx"
      ],
      reference_count: 4,
      private_library_count: 2,
      private_path_count: 1,
      credential_reference_count: 1,
      public_fixture_block_count: 4,
      public_report_metadata_only_count: 4,
      local_private_metadata_only_count: 4,
      exact_secret_provider: "TBD",
      encrypted_storage_default: "TBD",
      storage_roots: "TBD",
      permission_grant_persistence: "TBD",
      external_secret_manager_behavior: "TBD",
      direct_sql_access: false,
      raw_sqlite_handle_allowed: false,
      storage_bypass_requested: false,
      cloud_or_network_reference: false,
      secret_material_present: false,
      concrete_path_present: false,
      metadata_only: true,
      security_certification_claim: false,
      redaction_action: "secret_private_library_metadata_only_no_payload_no_secret_values",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "DEL-12-04 desktop secret/private-library boundary review; private libraries, private paths, and credential references are metadata-only, public fixtures are blocked, public report context is reference-only, and provider, encryption, storage-root, permission, cloud/network, and external secret-manager decisions stay TBD."
    },
    {
      export_id: "security_threat_model_review",
      label: "Security threat model review",
      document_kind: "openpipestress.technical_preview.security_threat_model_review",
      readiness: securityThreatModelReady ? "available" : "pending_security_threat_model_surface",
      deliverable_refs: ["DEL-12-05", "DEL-12-01", "DEL-12-02", "DEL-12-03", "DEL-12-04", "DEL-10-02", "DEL-10-03", "DEL-17-09"],
      source_refs: [
        model.project.id,
        "docs/security/threat_model.md",
        "tests/test_security_threat_model.py",
        "apps/desktop/src/features/security-threat-model/SecurityThreatModelPanel.tsx"
      ],
      threat_count: 19,
      high_risk_count: 14,
      medium_high_risk_count: 5,
      asset_class_count: 10,
      trust_boundary_count: 10,
      export_workflow_count: 6,
      open_decision_count: 14,
      tbd_decision_count: 13,
      local_first: true,
      telemetry_default_off: true,
      direct_sql_access: false,
      raw_sqlite_handle_allowed: false,
      storage_bypass_requested: false,
      plugin_manifest_grants_runtime_access: false,
      no_bypass_controls_present: true,
      unit_evidence_required: true,
      unit_policy_ref: "unit-policy-evidence:security-threat-model-no-bypass",
      unit_policy:
        "security_threat_model_requires_unit_checks_no_bypass_for_unit_bearing_workflows",
      conversion_performed: false,
      security_certification_claim: false,
      redaction_action: "threat_model_metadata_only_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "DEL-12-05 desktop threat-model review; summarizes current private-data, report/export, plugin/import/FEA handoff, telemetry, secret, and supply-chain threat coverage without claiming security sufficiency or release readiness."
    },
    {
      export_id: "editor_contract_review",
      label: "Editor contract review",
      document_kind: "openpipestress.technical_preview.editor_contract_review",
      readiness: editorContractReady ? "available" : "pending_editor_contract_surface",
      deliverable_refs: ["DEL-07-03", "DEL-07-02", "DEL-07-01", "DEL-06-03", "DEL-02-02", "DEL-12-02", "DEL-16-01"],
      source_refs: [
        model.project.id,
        "apps/desktop/src/features/editor-contract/EditorContractPanel.tsx",
        "apps/desktop/src/features/viewport/PipeViewport.tsx",
        ...editorIntents.map((intent) => intent.queue_id ?? intent.operation_id)
      ],
      editor_count: 4,
      editor_surface_count: 7,
      queued_intent_count: editorIntents.length,
      ready_editor_count: 2,
      blocked_editor_count: 2,
      diagnostic_count: 4,
      rule_pack_reference_status: "private_reference_only_missing_required_inputs",
      private_library_payload_status: "reference_slots_only_no_private_payload",
      direct_model_mutation_allowed: false,
      accepted_model_state_mutated: false,
      unit_evidence_required: true,
      unit_policy_ref: "DEL-02-02:unit_bearing_values_require_explicit_unit_metadata",
      unit_policy: "unit_bearing_values_require_explicit_unit_metadata",
      missing_unit_behavior: "diagnostic_blocking",
      conversion_performed: false,
      redaction_action: "editor_contract_metadata_only_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "DEL-07-03 desktop editor contract preview; materials/components/rule-pack references remain review-only command intent context, with private rule packs and private libraries represented by references only."
    },
    {
      export_id: "missing_data_warning_blocking_review",
      label: "Missing-data warning/blocking review",
      document_kind: "openpipestress.technical_preview.missing_data_warning_blocking_review",
      readiness: missingDataReviewReady ? "available" : "pending_missing_data_warning_surface",
      deliverable_refs: ["DEL-07-04", "DEL-07-07", "DEL-06-03", "DEL-05-04", "DEL-04-06", "DEL-02-03", "DEL-12-02"],
      source_refs: [
        model.project.id,
        "apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx",
        ...diagnostics.map((item) => item.id ?? item.code)
      ],
      warning_class_count: 6,
      active_warning_count: 5,
      solve_blocking_count: 0,
      rule_check_blocking_count: 2,
      provenance_warning_count: 2,
      assumption_warning_count: 1,
      nonlinear_warning_count: 0,
      ip_boundary_warning_count: 0,
      mechanics_solve_blocked: false,
      rule_check_blocked: true,
      mechanics_results_reviewable: Boolean(result && run),
      mechanics_results_qualified_by_rule_inputs: Boolean(result && run),
      silent_defaults_used: false,
      auto_fill_missing_data: false,
      unit_evidence_required: true,
      unit_policy_ref: "unit-input-policy-evidence:missing-data-warning-blocking-review",
      unit_policy:
        "missing_unit_bearing_inputs_require_explicit_units_without_default_unit_inference",
      conversion_performed: false,
      assistive_text_fields_available: true,
      color_only_signaling_allowed: false,
      redaction_action: "missing_data_warning_metadata_only_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "DEL-07-04 desktop missing-data warning review; solve-required and rule-check-required data stay distinct, missing values are not auto-filled, and warning meaning is exported as text metadata."
    },
    {
      export_id: "rule_completeness_review",
      label: "Rule-check completeness review",
      document_kind: "openpipestress.technical_preview.rule_completeness_review",
      readiness: "available",
      deliverable_refs: ["DEL-06-03", "DEL-07-04", "DEL-05-04", "DEL-08-03", "DEL-02-02"],
      source_refs: [model.project.id, result?.run_id ?? "not generated"],
      finding_count: diagnostics.length,
      rule_check_status: result?.status.rule_check ?? model.analysis_status.rule_check,
      mechanics_results_reviewable: Boolean(result),
      unit_evidence_required: true,
      redaction_action: "rule_completeness_metadata_only_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Rule completeness review records explicit rule-input unit policy and missing private/user rule data without conversion."
    },
    {
      export_id: "agent_proposal_review",
      label: "Agent proposal review",
      document_kind: "openpipestress.technical_preview.agent_proposal_review",
      readiness: agentProposalReviewReady ? "available" : "pending_agent_proposal",
      deliverable_refs: ["DEL-16-01", "DEL-16-02", "DEL-16-03", "DEL-16-04", "DEL-07-08", "DEL-02-02"],
      source_refs: [
        model.project.id,
        proposal?.proposal_id ?? "not generated",
        selectedReviewTarget ? `${selectedReviewTarget.target_type}:${selectedReviewTarget.id}` : "no selected review target"
      ],
      proposal_ref: proposal?.proposal_id ?? "not generated",
      proposal_operation_ref: proposal?.operation.operation_id ?? "not generated",
      selected_review_target_ref: selectedReviewTarget
        ? `${selectedReviewTarget.target_type}:${selectedReviewTarget.id}`
        : "not selected",
      validation_status: proposal?.validation.application_status ?? "not generated",
      diff_preview_status: proposal?.validation.diff_preview_status ?? "not generated",
      unit_validation_status: proposal?.validation.unit_validation ?? "not generated",
      review_only: true,
      user_acceptance_required: Boolean(proposal?.audit_boundary.requires_user_acceptance),
      accepted_model_state_mutated: Boolean(proposal?.audit_boundary.mutates_accepted_model_state),
      unit_evidence_required: true,
      redaction_action: "agent_proposal_metadata_only_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Agent proposal review records metadata-only operation proposal unit-validation status without applying operations or converting units."
    },
    {
      export_id: "accessibility_usability_baseline_review",
      label: "Accessibility/usability baseline review",
      document_kind: "openpipestress.technical_preview.accessibility_usability_baseline_review",
      readiness: accessibilityBaselineReady ? "available" : "pending_accessibility_baseline_surface",
      deliverable_refs: ["DEL-07-06", "DEL-07-01", "DEL-07-02", "DEL-07-03", "DEL-07-04", "DEL-07-05", "DEL-07-07"],
      source_refs: [
        model.project.id,
        "core/gui/accessibility/engine.py",
        "tests/test_accessibility_usability_baseline.py",
        "apps/desktop/src/features/accessibility-baseline/AccessibilityBaselinePanel.tsx"
      ],
      source_surface_count: 6,
      total_finding_count: 69,
      pass_count: 57,
      warning_count: 10,
      fail_count: 1,
      blocking_count: 1,
      keyboard_path_finding_count: 4,
      focus_order_finding_count: 3,
      readable_label_finding_count: 4,
      warning_visibility_finding_count: 5,
      result_review_visibility_finding_count: 4,
      solve_state_feedback_finding_count: 3,
      accessibility_target_status: "TBD_by_human_project_authority",
      desktop_runtime_evaluation: "not_performed",
      software_makes_accessibility_conformance_claim: false,
      color_only_status_signaling_allowed: false,
      redaction_action: "accessibility_baseline_metadata_only_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "DEL-07-06 desktop accessibility/usability baseline review; reports deterministic core GUI-contract evidence for keyboard, focus, labels, warning visibility, result review, and solve feedback without selecting a final conformance target."
    },
    {
      export_id: "design_authoring_comparison_workspace",
      label: "Design-authoring comparison workspace",
      document_kind: "openpipestress.technical_preview.design_authoring_comparison_workspace",
      readiness: designWorkspaceReady ? "available" : "pending_design_workspace_surface",
      deliverable_refs: [
        "DEL-07-08",
        "DEL-07-01",
        "DEL-07-04",
        "DEL-07-05",
        "DEL-07-07",
        "DEL-13-01",
        "DEL-13-03",
        "DEL-14-01",
        "DEL-14-02",
        "DEL-14-04",
        "DEL-16-02",
        "DEL-16-03",
        "DEL-16-04"
      ],
      source_refs: [
        model.project.id,
        "core/gui/design_workspace/engine.py",
        "tests/test_design_authoring_comparison_workspace.py",
        "apps/desktop/src/features/design-workspace/DesignWorkspacePanel.tsx"
      ],
      current_design_knowledge_record_count: knowledge?.records.length ?? 0,
      current_model_state_count: run ? 1 : 0,
      current_analysis_run_count: run ? 1 : 0,
      current_result_row_count: result?.results.length ?? 0,
      current_comparison_pair_count: comparison?.summary.comparable_result_pairs ?? 0,
      operation_review_record_count: operationRecordCount,
      core_design_knowledge_record_count: 2,
      core_constraint_warning_count: 1,
      core_model_state_count: 2,
      core_analysis_run_count: 2,
      core_graphical_overlay_count: 5,
      core_operation_diff_review_row_count: 1,
      workspace_mutates_accepted_model_state: false,
      accepted_operation_requires_explicit_user_acceptance_record: true,
      redaction_action: "design_workspace_metadata_only_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "DEL-07-08 desktop design-authoring workspace review; composes design knowledge, constraint warnings, state/run context, comparison tables, overlay descriptors, and operation diff review without applying operations or mutating accepted model state."
    },
    {
      export_id: "build_package_readiness",
      label: "Build/package readiness",
      document_kind: "openpipestress.technical_preview.build_package_readiness",
      readiness: buildReadinessReady ? "available" : "pending_build_readiness_surface",
      deliverable_refs: ["DEL-10-04", "DEL-10-05", "DEL-12-01", "DEL-12-02"],
      source_refs: [
        "package.json",
        "apps/desktop/package.json",
        "apps/desktop/src-tauri/tauri.conf.json",
        "docs/BUILD_AND_RELEASE.md",
        "tools/release/check_release_readiness.py"
      ],
      root_script_count: 4,
      desktop_script_count: 4,
      readiness_profile_count: 5,
      tauri_shell_status: "present",
      tauri_bundle_active: false,
      provider_neutral_readiness_tool: true,
      ci_provider: "TBD",
      release_matrix: "TBD",
      signing_status: "TBD",
      publishing_status: "TBD",
      release_publication_authorized: false,
      installer_or_binary_generated: false,
      redaction_action: "local_build_metadata_only_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Provider-neutral local build/package readiness metadata; no CI provider, installer, signing, publishing, release, or professional claim."
    },
    {
      export_id: "validation_release_evidence_review",
      label: "Validation and release evidence review",
      document_kind: "openpipestress.technical_preview.validation_release_evidence_review",
      readiness: validationEvidenceReady ? "available" : "pending_validation_evidence_surface",
      deliverable_refs: [
        "DEL-09-04",
        "DEL-09-05",
        "DEL-10-04",
        "DEL-09-01",
        "DEL-09-02",
        "DEL-09-03",
        "DEL-08-05"
      ],
      source_refs: [
        "docs/VALIDATION_STRATEGY.md",
        "docs/validation_manual/index.md",
        "docs/RELEASE_QUALITY_GATES.md",
        "docs/BUILD_AND_RELEASE.md",
        "tools/release/check_release_readiness.py",
        "tests/test_release_readiness_script.py",
        "apps/desktop/src/features/validation-evidence/ValidationEvidencePanel.tsx"
      ],
      manual_section_count: 10,
      evidence_area_count: 8,
      release_profile_count: 5,
      required_release_path_count: 7,
      skeleton_check_count: 2,
      validation_manual_open_decision_count: 5,
      release_gate_family_count: 5,
      release_authorization_status: "not_authorized",
      final_threshold_policy: "TBD",
      gui_validation_evidence_policy: "TBD",
      browser_panel_runs_tool: false,
      dry_run_default: true,
      redaction_action: "validation_release_evidence_metadata_only_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "DEL-09-04/DEL-09-05 validation and release-evidence review; records manual sections, evidence inventory, local readiness profiles, release-gate families, and unresolved TBDs without authorizing release or professional reliance."
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
      export_id: "stress_neutral_csv_json_package",
      label: "Stress-neutral CSV/JSON package",
      document_kind: "openpipestress.technical_preview.stress_neutral_csv_json_package",
      readiness: stressNeutralReady ? "available" : "pending_mechanics_run",
      deliverable_refs: ["DEL-17-06", "DEL-08-04", "DEL-14-02", "DEL-14-05", "DEL-17-02"],
      source_refs: result && run
        ? [result.run_id, run.model_state_ref.ref, `result-envelope:${result.run_id}`]
        : [model.project.id],
      result_ref_count: result?.results.length ?? 0,
      csv_column_count: 11,
      stable_id_count: result?.results.length ?? 0,
      member_roles: ["manifest", "csv_text", "result_rows", "stable_id_map", "loss_report", "validation_report", "diagnostics"],
      comparison_semantics: "diagnostic_export_only_no_pass_fail",
      schema_validation_status: "desktop_preview_shape_aligned_not_runtime_json_schema_validated",
      canonical_package_hash_status: "computed_local_preview_sha256_by_target_panel",
      vendor_format_claim: false,
      solver_input_deck_claim: false,
      target_compatibility_claim: false,
      solver_validation_claim: false,
      code_compliance_claim: false,
      professional_reliance_claim: false,
      redaction_action: "classify_and_warn_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Stress-neutral CSV/JSON browser preview; fixed result columns, stable IDs, loss report, and diagnostic-only comparison semantics."
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
      export_id: "local_fea_handoff_package",
      label: "Local FEA handoff package",
      document_kind: "openpipestress.technical_preview.local_fea_handoff_package",
      readiness: localFeaHandoffReady ? "available" : "pending_mechanics_run",
      deliverable_refs: ["DEL-10-03", "DEL-10-02", "DEL-08-04", "DEL-14-02", "DEL-15-01"],
      source_refs: result && run ? [result.run_id, run.model_state_ref.ref, `result-envelope:${result.run_id}`] : [model.project.id],
      contract_kind: "schema_first_local_fea_handoff_contract",
      global_analysis_role: "primary_global_centerline_frame_model",
      local_analysis_role: "optional_specialized_shell_solid_handoff",
      package_kind: "local_shell_solid_fea_handoff",
      selected_region_basis: "diagnostic_suggested",
      guidance_label_count: result && run ? 4 : 0,
      unsupported_behavior_flag_count: result && run ? 5 : 0,
      diagnostic_count: result && run ? 4 : 0,
      selected_result_ref_count: result && run ? 5 : 0,
      concrete_export_format: "TBD",
      target_solver_adapter: "TBD",
      mesh_generation: "TBD",
      external_solver_invocation: "TBD",
      transfer_method_label: result && run ? "result_reference_only" : "TBD",
      redaction_required_before_public_export: true,
      human_review_required: true,
      software_makes_approval_claim: false,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_authentication_claim: false,
      redaction_action: "local_fea_handoff_metadata_only_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Target-neutral local shell/solid FEA handoff preview; selected region and transfer references are review context only, with mesh, target format, adapter, and external solver invocation remaining TBD."
    },
    {
      export_id: "external_prover_boundary_metadata",
      label: "External prover boundary metadata",
      document_kind: "openpipestress.technical_preview.external_prover_boundary_metadata",
      readiness: externalProverReady ? "available" : "pending_external_prover_metadata",
      deliverable_refs: ["DEL-15-04", "DEL-15-01", "DEL-15-02", "DEL-15-03", "DEL-17-05"],
      source_refs: result && run
        ? [result.run_id, run.model_state_ref.ref, "external:desktop-preview-metadata-only"]
        : [model.project.id, "external:desktop-preview-metadata-only"],
      metadata_contract_status: "non_authoritative_workflow_metadata",
      external_reference_count: 1,
      attachment_count: 1,
      unsupported_target_flag_count: 3,
      diagnostic_ref_count: diagnostics.length,
      external_tool_invoked: false,
      commercial_result_payload_ingested: false,
      software_creates_external_validation_record: false,
      redaction_action: "metadata_refs_and_hashes_only_no_payload_ingestion",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Metadata-only external-prover workflow context; no external tool invocation, commercial result ingestion, or software-generated acceptance record."
    },
    {
      export_id: "review_geometry_export",
      label: "Review geometry export",
      document_kind: "openpipestress.technical_preview.review_geometry_export",
      readiness: reviewGeometryReady ? "available" : "pending_review_geometry_profile",
      deliverable_refs: ["DEL-17-08", "DEL-17-01", "DEL-17-02", "DEL-15-01", "DEL-15-02", "DEL-15-03"],
      source_refs: result && run ? [result.run_id, run.model_state_ref.ref, model.project.id] : [model.project.id],
      review_geometry_status: "visual_review_geometry_only",
      geometry_format: "glTF_2_0_json_preview",
      gltf_version_basis: "2.0",
      node_count: model.nodes.length,
      pipe_segment_count: model.pipe_segments.length,
      line_primitive_count: model.pipe_segments.length,
      stable_id_count: stableEntityCount,
      transform_policy: "preview_z_up_to_gltf_y_up_rotation_x_minus_90",
      sidecar_id_map_required: true,
      glb_binary_writer_status: "TBD",
      viewer_compatibility: "TBD",
      solver_geometry_equivalence_claim: false,
      professional_validation_claim: false,
      target_compatibility_claim: false,
      redaction_action: "classify_and_warn_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Local glTF JSON visual-review geometry with stable-ID sidecar metadata; not solver geometry, target compatibility evidence, or professional validation."
    },
    {
      export_id: "conservative_pcf_export",
      label: "Conservative PCF export",
      document_kind: "openpipestress.technical_preview.conservative_pcf_export_package",
      readiness: pcfExportReady ? "available" : "pending_pcf_profile_review",
      deliverable_refs: ["DEL-17-07", "DEL-17-01", "DEL-17-02", "DEL-03-02", "DEL-13-04", "DEL-15-02"],
      source_refs: result && run ? [result.run_id, run.model_state_ref.ref, model.project.id] : [model.project.id],
      target_family: "pcf",
      profile_id: "ops.pcf.conservative_subset",
      target_profile_version_basis: "TBD",
      package_status: "conservative_pcf_export_foundation",
      payload_kind: "conservative_pcf_pipe_subset",
      node_count: model.nodes.length,
      pipe_segment_count: model.pipe_segments.length,
      stable_id_count: model.pipe_segments.length,
      loss_category_count: 6,
      validation_status: "blocked_missing_explicit_pcf_target_fields",
      schema_validation_status: "desktop_preview_shape_aligned_not_runtime_json_schema_validated",
      canonical_package_hash_status: "computed_local_preview_sha256_by_target_panel",
      support_restraint_policy: "unsupported_or_tbd_until_source_confirmed",
      translator_default_policy: "hidden_defaults_blocked_or_loss_reported",
      target_compatibility_claim: false,
      solver_validation_claim: false,
      code_compliance_claim: false,
      professional_reliance_claim: false,
      redaction_action: "classify_and_warn_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Local conservative PCF review package with sidecar stable IDs, mandatory loss report, and blocking TBDs for target profile and nominal size."
    },
    {
      export_id: "caepipe_mbf_export",
      label: "CAEPIPE MBF export",
      document_kind: "openpipestress.technical_preview.caepipe_mbf_export_package",
      readiness: caepipeMbfExportReady ? "available" : "pending_caepipe_mbf_profile_review",
      deliverable_refs: ["DEL-17-04", "DEL-17-01", "DEL-17-02"],
      source_refs: result && run ? [result.run_id, run.model_state_ref.ref, model.project.id] : [model.project.id],
      target_family: "caepipe_mbf",
      profile_id: "ops.caepipe_mbf.smoke_tbd",
      target_version_basis: "TBD-17-01-001",
      record_subset_basis: "TBD-17-01-002",
      package_status: "caepipe_mbf_export_foundation",
      payload_kind: "caepipe_mbf_smoke_subset_payload",
      node_count: model.nodes.length,
      pipe_element_count: model.pipe_segments.length,
      support_count: model.supports.length,
      load_case_count: model.load_cases.length,
      stable_id_count: model.nodes.length + model.pipe_segments.length + model.supports.length + model.load_cases.length,
      loss_category_count: 6,
      validation_status: "boundary_checked",
      schema_validation_status: "desktop_preview_shape_aligned_not_runtime_json_schema_validated",
      canonical_package_hash_status: "computed_local_preview_sha256_by_target_panel",
      stable_id_policy: "sidecar_mapping_until_direct_mbf_carrier_confirmed",
      external_execution_policy: "not_invoked_by_this_package",
      caepipe_compatibility_claim: false,
      solver_validation_claim: false,
      code_compliance_claim: false,
      professional_reliance_claim: false,
      redaction_action: "classify_and_warn_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Local CAEPIPE MBF smoke-subset review package with sidecar stable IDs, mandatory loss report, and carried TBDs for target version, record subset, and direct stable-ID carrier."
    },
    {
      export_id: "caepipe_external_run_evidence",
      label: "CAEPIPE external-run evidence",
      document_kind: "openpipestress.technical_preview.caepipe_external_run_package",
      readiness: caepipeExternalRunReady ? "available" : "pending_caepipe_external_run_boundary",
      deliverable_refs: ["DEL-17-05", "DEL-17-04", "DEL-17-02", "DEL-17-01", "DEL-15-04"],
      source_refs: result && run
        ? [result.run_id, run.model_state_ref.ref, "caepipe-mbf:desktop-preview-del-17-04", model.project.id]
        : ["caepipe-mbf:desktop-preview-del-17-04", model.project.id],
      package_status: "parser_only_evidence",
      mbf_package_ref: "caepipe-mbf:desktop-preview-del-17-04",
      executable_path_state: "absent",
      invocation_profile: "TBD-17-05-invocation-profile",
      external_tool_invoked: false,
      parser_status: "parsed_parser_only_fixture",
      parser_row_count: 3,
      parser_section_count: 2,
      parser_correlation_status: "canonical_id_map",
      validation_status: "boundary_checked",
      schema_validation_status: "desktop_preview_shape_aligned_not_runtime_json_schema_validated",
      caepipe_compatibility_claim: false,
      solver_validation_claim: false,
      code_compliance_claim: false,
      professional_reliance_claim: false,
      redaction_action: "parser_only_public_fixture_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Local CAEPIPE external-run evidence preview; executable path is absent, external execution is not attempted, parser-only CSV rows are invented, and live invocation remains user-owned/TBD."
    },
    {
      export_id: "export_adapter_sdk_registry",
      label: "Export adapter SDK registry",
      document_kind: "openpipestress.technical_preview.export_adapter_sdk_registry",
      readiness: exportAdapterSdkReady ? "available" : "pending_adapter_sdk_registry",
      deliverable_refs: ["DEL-17-09", "DEL-17-02", "DEL-17-01", "DEL-02-04", "DEL-10-01", "DEL-10-02"],
      source_refs: result && run ? [result.run_id, run.model_state_ref.ref, model.project.id] : [model.project.id],
      registry_id: "ops.export_adapter_sdk.registry_preview",
      sdk_contract_status: "contract_level_preview",
      target_registry_count: 5,
      preview_panel_available_count: 5,
      capability_count: 4,
      validation_status: "boundary_checked",
      schema_validation_status: "desktop_preview_shape_aligned_not_runtime_json_schema_validated",
      plugin_runtime: "TBD",
      permission_taxonomy: "TBD",
      public_transport_protocol: "TBD",
      source_basis_admitted_target_count: 0,
      target_support_claim_count: 0,
      target_compatibility_claim: false,
      solver_validation_claim: false,
      code_compliance_claim: false,
      professional_reliance_claim: false,
      redaction_action: "local_sdk_registry_metadata_only_no_private_payload",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false,
      review_note:
        "Local export-adapter SDK target registry preview; additional target source-basis admission, plugin runtime, permissions, public transport, and target support remain TBD."
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
        "apps/desktop/src/features/export-review/ExportReviewPanel.tsx",
        "apps/desktop/src/features/build-readiness/BuildReadinessPanel.tsx",
        "apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx",
        "apps/desktop/src/features/secret-private-library/SecretPrivateLibraryPanel.tsx",
        "apps/desktop/src/features/security-threat-model/SecurityThreatModelPanel.tsx",
        "apps/desktop/src/features/editor-contract/EditorContractPanel.tsx",
        "apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx",
        "apps/desktop/src/features/accessibility-baseline/AccessibilityBaselinePanel.tsx",
        "apps/desktop/src/features/design-workspace/DesignWorkspacePanel.tsx",
        "apps/desktop/src/features/validation-evidence/ValidationEvidencePanel.tsx",
        "apps/desktop/src/features/local-fea-handoff/LocalFeaHandoffPanel.tsx",
        "apps/desktop/src/features/external-prover/ExternalProverBoundaryPanel.tsx",
        "apps/desktop/src/features/review-geometry/ReviewGeometryPanel.tsx",
        "apps/desktop/src/features/pcf-export/PcfExportPanel.tsx",
        "apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx",
        "apps/desktop/src/features/caepipe-external/CaepipeExternalHarnessPanel.tsx",
        "apps/desktop/src/features/export-adapter-sdk/ExportAdapterSdkPanel.tsx",
        "apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx",
        "apps/desktop/src/features/viewport/PipeViewport.tsx"
      ],
      target_count: result && run ? 22 : 21,
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
  const unitPolicySummary = buildExportReviewUnitPolicySummary({ model, result, exports });

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.export_review_manifest",
    export_scope: "local_browser_download_preview",
    deliverable_refs: ["DEL-12-02", "DEL-02-05", "DEL-12-01", "DEL-12-03", "DEL-12-04", "DEL-12-05", "DEL-08-04", "DEL-08-05", "DEL-08-06", "DEL-02-04", "DEL-07-01", "DEL-07-02", "DEL-07-03", "DEL-07-04", "DEL-07-06", "DEL-07-08", "DEL-09-01", "DEL-09-02", "DEL-09-03", "DEL-09-04", "DEL-09-05", "DEL-10-01", "DEL-10-02", "DEL-10-03", "DEL-10-04", "DEL-10-05", "DEL-13-01", "DEL-13-03", "DEL-14-01", "DEL-14-02", "DEL-14-04", "DEL-15-01", "DEL-15-04", "DEL-17-02", "DEL-17-03", "DEL-17-04", "DEL-17-05", "DEL-17-06", "DEL-17-07", "DEL-17-08", "DEL-17-09", "DEL-16-01", "DEL-16-02", "DEL-16-03", "DEL-16-04"],
    scope_items: ["SOW-040", "SOW-050", "SOW-041", "SOW-029", "SOW-037", "SOW-046", "SOW-043", "SOW-054", "SOW-032", "SOW-026", "SOW-027", "SOW-020", "SOW-021", "SOW-022", "SOW-024", "SOW-030", "SOW-031", "SOW-049", "SOW-074", "SOW-075", "SOW-076"],
    objectives: ["OBJ-001", "OBJ-002", "OBJ-006", "OBJ-008", "OBJ-009", "OBJ-010", "OBJ-011", "OBJ-012", "OBJ-015", "OBJ-016", "OBJ-017", "OBJ-018"],
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
    unit_policy_summary: unitPolicySummary,
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

function buildExportReviewUnitPolicySummary({
  model,
  result,
  exports
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  exports: Array<{ export_id: string; document_kind: string; readiness: string }>;
}) {
  const disclosure = buildExportUnitSystemDisclosure({
    model,
    result,
    targetExportUnits: {},
    conversionPolicy: "export_review_manifest_inventory_only_no_target_conversion",
    conversionPerformed: false,
    conversionScope: [],
    sourceLocation: "apps/desktop/src/features/export-review/ExportReviewPanel.tsx"
  });
  const unitEvidenceMatrix = exports.map((item) => {
    const unitEvidenceRequired = UNIT_EVIDENCE_REQUIRED_EXPORT_IDS.has(item.export_id);
    return {
      export_id: item.export_id,
      document_kind: item.document_kind,
      readiness: item.readiness,
      unit_evidence_required: unitEvidenceRequired,
      unit_evidence_status: unitEvidenceRequired
        ? item.readiness === "available"
          ? "covered_by_target_panel_or_export_packet"
          : "pending_source_export_packet"
        : "not_unit_bearing_metadata_or_boundary_review",
      conversion_policy: unitEvidenceRequired
        ? "source_units_preserved_or_target_panel_discloses_conversion_policy"
        : "not_applicable",
      conversion_performed_by_export_review_manifest: false
    };
  });
  const requiredRows = unitEvidenceMatrix.filter((item) => item.unit_evidence_required);
  const presentRows = requiredRows.filter((item) => item.unit_evidence_status === "covered_by_target_panel_or_export_packet");

  return {
    ...disclosure,
    evidence_id: "unit-policy-evidence:export-review-manifest",
    review_scope: "export_review_manifest_unit_evidence_inventory",
    unit_review_policy:
      "inventory_unit_bearing_export_records_and_require_target_panels_or_export_packets_to_carry_DEC_018_unit_evidence",
    unit_evidence_matrix: unitEvidenceMatrix,
    covered_export_ids: presentRows.map((item) => item.export_id),
    not_unit_bearing_export_ids: unitEvidenceMatrix
      .filter((item) => !item.unit_evidence_required)
      .map((item) => item.export_id),
    summary: {
      reviewed_export_count: exports.length,
      unit_evidence_required_count: requiredRows.length,
      unit_evidence_present_count: presentRows.length,
      conversion_performed_count: unitEvidenceMatrix.filter((item) => item.conversion_performed_by_export_review_manifest)
        .length
    }
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
