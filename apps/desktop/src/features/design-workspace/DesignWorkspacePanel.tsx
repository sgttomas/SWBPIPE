import { Download, GitCompare } from "lucide-react";
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

export function DesignWorkspacePanel({
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
  const packet = buildDesignWorkspacePacket({
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
    <section className="panel design-workspace-panel" aria-label="Design-authoring workspace" data-testid="design-workspace-panel">
      <div className="panel-title">
        <GitCompare size={16} aria-hidden="true" />
        Design Workspace
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="design-workspace-export-link"
          download={`openpipestress-preview-design-workspace-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Workspace JSON
        </a>
        <span data-testid="design-workspace-summary">
          knowledge={packet.summary.current_design_knowledge_record_count}; states=
          {packet.summary.current_model_state_count}; runs={packet.summary.current_analysis_run_count}; comparisons=
          {packet.summary.current_comparison_pair_count}; operations={packet.summary.operation_review_record_count}
        </span>
      </div>
      <div className="report-list" data-testid="design-workspace-body">
        <WorkspaceLine
          label="Core contract"
          value={`records=${packet.core_contract_evidence.design_knowledge_record_count}; warnings=${packet.core_contract_evidence.constraint_warning_count}; states=${packet.core_contract_evidence.model_state_count}; runs=${packet.core_contract_evidence.analysis_run_count}; overlays=${packet.core_contract_evidence.graphical_overlay_count}`}
          testId="design-workspace-core"
        />
        <WorkspaceLine
          label="Current browser"
          value={`analysis_run=${packet.current_workspace_state.analysis_run_ref}; comparison=${packet.current_workspace_state.comparison_ref}; result_rows=${packet.summary.current_result_row_count}`}
          testId="design-workspace-current"
        />
        <WorkspaceLine
          label="Operation review"
          value={`records=${packet.summary.operation_review_record_count}; accepted_mutation=${String(packet.mutation_boundary.accepted_model_state_mutated)}; application=${packet.mutation_boundary.workspace_application_status}`}
          testId="design-workspace-operation"
        />
        <WorkspaceLine
          label="Routing"
          value={`selected=${packet.review_state_routing.selected_review_target_ref}; route=${packet.review_state_routing.route_state}`}
          testId="design-workspace-routing"
        />
        <WorkspaceLine
          label="Boundary"
          value={`private_payload=${String(packet.private_payload_included)}; protected=${String(packet.protected_content_included)}; professional_claim=${String(packet.release_or_professional_claim)}`}
          testId="design-workspace-boundary"
        />
      </div>
      <small className="report-note">
        Design workspace evidence is a local technical-preview composition of GUI state, comparison context, and
        operation-review metadata. It does not apply operations, mutate accepted model state, or create professional
        approval.
      </small>
    </section>
  );
}

function WorkspaceLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildDesignWorkspacePacket({
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
  const operationRecords = [
    ...editorIntents.map((intent) => ({
      record_source: "editor_intent",
      operation_id: intent.operation_id,
      operation_status: intent.operation_status,
      application_status: intent.validation.application_status,
      requires_user_acceptance: intent.audit_boundary.requires_user_acceptance,
      accepted_model_state_mutated: intent.audit_boundary.mutates_accepted_model_state,
      target_ref: intent.target.ref
    })),
    ...(proposal
      ? [
          {
            record_source: "agent_proposal",
            operation_id: proposal.operation.operation_id,
            operation_status: proposal.operation.operation_status,
            application_status: proposal.validation.application_status ?? "not_applied",
            requires_user_acceptance: proposal.audit_boundary.requires_user_acceptance,
            accepted_model_state_mutated: proposal.audit_boundary.mutates_accepted_model_state,
            target_ref: proposal.operation.affected_entity_ids[0] ?? "TBD"
          }
        ]
      : [])
  ];
  const currentDiagnostics = [...model.diagnostics, ...(knowledge?.diagnostics ?? []), ...(result?.diagnostics ?? [])];
  const comparisonPairCount = comparison?.summary.comparable_result_pairs ?? 0;
  const unresolvedComparisonCount =
    (comparison?.summary.unmatched_left_results ?? 0) + (comparison?.summary.unmatched_right_results ?? 0);
  const selectedRef = selectedReviewTarget ? `${selectedReviewTarget.target_type}:${selectedReviewTarget.id}` : "none";

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.design_authoring_comparison_workspace",
    export_scope: "local_browser_download_preview",
    deliverable_id: "DEL-07-08",
    package_id: "PKG-07",
    scope_item: "SOW-076",
    objectives: ["OBJ-015", "OBJ-016"],
    project_ref: model.project.id,
    source_basis: {
      core_module: "core/gui/design_workspace/engine.py",
      guard_tests: "tests/test_design_authoring_comparison_workspace.py",
      deliverable_context:
        "execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-08_Design-authoring state and comparison workspace/_CONTEXT.md",
      app_surfaces: [
        "apps/desktop/src/features/design-workspace/DesignWorkspacePanel.tsx",
        "apps/desktop/src/features/knowledge/KnowledgePanel.tsx",
        "apps/desktop/src/features/comparison/ComparisonPanel.tsx",
        "apps/desktop/src/features/diff-preview/DiffPreviewPanel.tsx",
        "apps/desktop/src/features/operations/OperationLedgerPanel.tsx",
        "apps/desktop/src/features/export-review/ExportReviewPanel.tsx",
        "apps/desktop/src/features/report/ReportPanel.tsx",
        "apps/desktop/src/features/report-lint/ReportLintPanel.tsx"
      ]
    },
    summary: {
      current_design_knowledge_record_count: knowledge?.records.length ?? 0,
      current_design_knowledge_diagnostic_count: knowledge?.diagnostics.length ?? 0,
      current_model_diagnostic_count: model.diagnostics.length,
      current_result_diagnostic_count: result?.diagnostics.length ?? 0,
      current_total_diagnostic_count: currentDiagnostics.length,
      current_model_state_count: analysisRun ? 1 : 0,
      current_analysis_run_count: analysisRun ? 1 : 0,
      current_result_row_count: result?.results.length ?? 0,
      current_comparison_pair_count: comparisonPairCount,
      current_unmatched_comparison_count: unresolvedComparisonCount,
      operation_review_record_count: operationRecords.length,
      selected_review_target_present: Boolean(selectedReviewTarget)
    },
    core_contract_evidence: {
      design_knowledge_record_count: 2,
      records_with_unresolved_tbd: 1,
      constraint_warning_count: 1,
      constraint_validation_has_blocking_findings: true,
      model_state_count: 2,
      analysis_run_count: 2,
      model_comparison_row_count: 3,
      unmatched_model_comparison_row_count: 2,
      analysis_result_row_count: 1,
      graphical_overlay_count: 5,
      operation_diff_review_row_count: 1,
      route_state: "review_navigation_available",
      deterministic_workspace_hash_present: true,
      workspace_hash_scope: "ui_composition_review_state_only"
    },
    current_workspace_state: {
      design_knowledge_panel: knowledge ? "available" : "unavailable_missing_input",
      constraint_warning_panel: "represented_by_diagnostics_and_missing_data_panel",
      state_run_browser: analysisRun ? "available" : "pending_mechanics_run",
      comparison_tables: comparison ? "available" : "pending_mechanics_run",
      graphical_overlays: comparison ? "descriptor_only_available" : "pending_comparison_context",
      operation_diff_review: operationRecords.length > 0 ? "available" : "empty_operation_queue",
      analysis_run_ref: analysisRun?.analysis_run.run_id ?? "not generated",
      model_state_ref: analysisRun?.analysis_run.model_state_ref.ref ?? "not generated",
      comparison_ref: comparison?.comparison_id ?? "not generated"
    },
    review_state_routing: {
      selected_review_target_ref: selectedRef,
      route_state: selectedReviewTarget ? "review_navigation_available" : "no_selected_review_target",
      result_selection_routes_to_model_context: true,
      diagnostic_selection_routes_to_model_context: true,
      unavailable_panel_ids: analysisRun && comparison ? [] : ["state_run_browser", "comparison_tables"]
    },
    operation_review_records: operationRecords,
    mutation_boundary: {
      workspace_mutates_accepted_model_state: false,
      hidden_accepted_model_mutation: false,
      accepted_model_state_mutated: operationRecords.some((item) => item.accepted_model_state_mutated),
      workspace_application_status: operationRecords.length > 0 ? "not_applied" : "empty_operation_queue",
      gui_originated_changes_route: "structured_operation_or_application_service_command_intent_only",
      accepted_operation_requires_explicit_user_acceptance_record: true
    },
    data_boundary: model.data_boundary,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    professional_boundary: professionalBoundary()
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

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}
