import { Download, HardDrive } from "lucide-react";
import type {
  AgentProposal,
  EditorOperationIntent,
  LocalProjectIndexEntry,
  LocalProjectSummary,
  LocalStorageCapability,
  ModelHashIntegrityEvidence,
  PreviewModel
} from "../../types";

export function ProjectStorageAuditPanel({
  model,
  storageCapability,
  projectSummary,
  projectIndex,
  projectMessage,
  projectOperation,
  editorIntents,
  proposal,
  modelHashIntegrity = null
}: {
  model: PreviewModel;
  storageCapability: LocalStorageCapability | null;
  projectSummary: LocalProjectSummary | null;
  projectIndex: LocalProjectIndexEntry[] | null;
  projectMessage: string;
  projectOperation: string;
  editorIntents: EditorOperationIntent[];
  proposal: AgentProposal | null;
  modelHashIntegrity?: ModelHashIntegrityEvidence | null;
}) {
  const packet = buildProjectStorageAuditPacket({
    model,
    storageCapability,
    projectSummary,
    projectIndex,
    projectMessage,
    projectOperation,
    editorIntents,
    proposal
  });

  return (
    <section className="panel project-storage-panel" aria-label="Project storage audit" data-testid="project-storage-panel">
      <div className="panel-title">
        <HardDrive size={16} aria-hidden="true" />
        Project Storage Audit
      </div>
      <div className="report-actions">
        <ControlledExportLink
          className="report-export-link"
          data-testid="project-storage-export-link"
          download={`openpipestress-preview-storage-audit-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Local storage JSON
        </ControlledExportLink>
        <span data-testid="project-storage-summary">
          operation={packet.summary.last_operation}; pending operations={packet.summary.pending_operation_count};
          proposals={packet.summary.proposal_operation_count}; accepted_state_mutated=
          {String(packet.summary.accepted_model_state_mutated)}
        </span>
      </div>
      <div className="report-list" data-testid="project-storage-body">
        <StorageLine
          label="Storage capability"
          value={`${packet.summary.storage_engine}; mode=${packet.summary.storage_mode}; FTS5=${String(
            packet.summary.fts5_available
          )}`}
          testId="project-storage-capability"
        />
        <StorageLine
          label="Local-only boundary"
          value={`network=${String(packet.summary.network_required)}; daemon=${String(
            packet.summary.daemon_required
          )}; telemetry=${String(packet.summary.telemetry_enabled)}; repository_default_private_write=false`}
          testId="project-storage-local-boundary"
        />
        <StorageLine
          label="Project snapshot"
          value={`${packet.project_summary?.project_id ?? model.project.id}; ${
            packet.project_summary?.storage_mode ?? "not_persisted_this_session"
          }; persisted_editor_intents=${packet.summary.persisted_editor_intent_count}; persisted_proposals=${
            packet.summary.persisted_proposal_count
          }; persisted_review_targets=${
            packet.summary.persisted_selected_review_target_count
          }; persisted_review_target_ref=${
            packet.summary.persisted_selected_review_target_ref
          }; persisted_mechanics_results=${
            packet.summary.persisted_mechanics_result_count
          }; persisted_analysis_runs=${
            packet.summary.persisted_analysis_run_count
          }; persisted_analysis_run_ref=${
            packet.summary.persisted_analysis_run_ref
          }; copied_external_files=${String(
            packet.summary.copied_external_files
          )}`}
          testId="project-storage-snapshot"
        />
        <StorageLine
          label="Project index"
          value={`state=${packet.summary.project_index_state}; listed_projects=${
            packet.summary.listed_project_count
          }; refs=${packet.project_index_refs.length > 0 ? packet.project_index_refs.join(", ") : "none"}`}
          testId="project-storage-project-index"
        />
        <StorageLine
          label="Model hash persistence"
          value={`persisted_model_hashes=${projectSummary?.persisted_model_hash_count ?? 0}; persisted_model_hash_ref=${
            projectSummary?.persisted_model_hash_ref ?? "not_persisted"
          }`}
          testId="model-hash-persistence"
        />
        <StorageLine
          label="Model hash integrity"
          value={
            modelHashIntegrity
              ? `integrity_status=${modelHashIntegrity.integrity_status}; persisted_value=${modelHashIntegrity.persisted_value}; recomputed_value=${modelHashIntegrity.recomputed_value}; verification_basis=${modelHashIntegrity.verification_basis}; review-only integrity signal for human review`
              : "no open-verification has run this session; review-only integrity signal for human review"
          }
          testId="model-hash-integrity"
        />
        <StorageLine
          label="Unit round-trip"
          value={`status=${packet.summary.unit_round_trip_status}; checked_refs=${
            packet.summary.unit_round_trip_checked_ref_count
          }; signature=${packet.summary.unit_round_trip_signature}; model=${formatUnitRecord(
            packet.unit_policy_evidence.model_units
          )}; conversion=${String(packet.unit_policy_evidence.conversion_performed)}`}
          testId="project-storage-unit-round-trip"
        />
        <StorageLine
          label="Payload boundary"
          value={`private payload=false; protected content=false; release/professional claim=false`}
          testId="project-storage-payload-boundary"
        />
      </div>
      <small className="report-note">
        Storage audit packets are local evidence only; create/open/save operations do not accept model changes or copy
        external project files. Acceptance and professional judgment remain with the responsible engineer.
      </small>
    </section>
  );
}

function StorageLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildProjectStorageAuditPacket({
  model,
  storageCapability,
  projectSummary,
  projectIndex,
  projectMessage,
  projectOperation,
  editorIntents,
  proposal
}: {
  model: PreviewModel;
  storageCapability: LocalStorageCapability | null;
  projectSummary: LocalProjectSummary | null;
  projectIndex: LocalProjectIndexEntry[] | null;
  projectMessage: string;
  projectOperation: string;
  editorIntents: EditorOperationIntent[];
  proposal: AgentProposal | null;
}) {
  const storageEngine = storageCapability?.engine ?? "storage_check_pending";
  const storageMode = projectSummary?.storage_mode ?? "not_persisted_this_session";
  const proposalCount = proposal ? 1 : 0;
  const pendingOperationCount = editorIntents.length + proposalCount;
  const reviewOperationStatuses = unique([
    ...editorIntents.map((intent) => intent.validation.application_status),
    ...(proposal ? [proposal.validation.application_status] : [])
  ]);
  const unitPolicyEvidence = buildProjectStorageUnitPolicyEvidence({ model, projectSummary });

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.local_project_persistence_audit",
    export_scope: "local_browser_download_preview",
    deliverable_refs: ["DEL-02-02", "DEL-02-05", "DEL-12-01", "DEL-12-02"],
    scope_items: ["SOW-029", "SOW-040", "SOW-050"],
    objectives: ["OBJ-010", "OBJ-012"],
    project_ref: model.project.id,
    project_name: model.project.name,
    summary: {
      last_operation: projectOperation,
      storage_engine: storageEngine,
      storage_mode: storageMode,
      migration_status: projectSummary?.migration_status ?? "not_applicable",
      fts5_available: Boolean(storageCapability?.fts5_available),
      fts_indexed: Boolean(projectSummary?.fts_indexed),
      pending_operation_count: pendingOperationCount,
      editor_intent_count: editorIntents.length,
      proposal_operation_count: proposalCount,
      persisted_editor_intent_count: projectSummary?.editor_intent_count ?? 0,
      persisted_proposal_count: projectSummary?.proposal_count ?? 0,
      persisted_selected_review_target_count: projectSummary?.selected_review_target_count ?? 0,
      persisted_selected_review_target_ref: projectSummary?.selected_review_target_ref ?? "not_selected",
      persisted_mechanics_result_count: projectSummary?.persisted_mechanics_result_count ?? 0,
      persisted_analysis_run_count: projectSummary?.persisted_analysis_run_count ?? 0,
      persisted_analysis_run_ref: projectSummary?.persisted_analysis_run_ref ?? "not_persisted",
      unit_round_trip_status: projectSummary?.unit_round_trip_status ?? "not_persisted_this_session",
      unit_round_trip_checked_ref_count: projectSummary?.unit_round_trip_checked_ref_count ?? 0,
      unit_round_trip_signature: projectSummary?.unit_round_trip_signature ?? "not_persisted",
      project_index_state: projectIndex ? "listed" : "not_requested",
      listed_project_count: projectIndex?.length ?? 0,
      applied_operation_count: 0,
      accepted_model_state_mutated: false,
      network_required: Boolean(storageCapability?.network_required),
      daemon_required: Boolean(storageCapability?.daemon_required),
      telemetry_enabled: Boolean(storageCapability?.telemetry_enabled),
      copied_external_files: Boolean(projectSummary?.copied_external_files)
    },
    storage_capability: storageCapability,
    project_summary: projectSummary,
    project_index: projectIndex ?? [],
    project_index_refs: (projectIndex ?? []).map((entry) => entry.project_id),
    unit_policy_evidence: unitPolicyEvidence,
    project_message: projectMessage,
    editor_intent_refs: editorIntents.map((intent) => intent.operation_id),
    proposal_refs: proposal ? [proposal.proposal_id] : [],
    editor_operation_statuses: unique(editorIntents.map((intent) => intent.validation.application_status)),
    review_operation_statuses: reviewOperationStatuses,
    boundary: {
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
    },
    data_boundary: model.data_boundary,
    diagnostics: storageDiagnostics(storageCapability, projectSummary),
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    professional_boundary: professionalBoundary()
  };
}

function buildProjectStorageUnitPolicyEvidence({
  model,
  projectSummary
}: {
  model: PreviewModel;
  projectSummary: LocalProjectSummary | null;
}) {
  return {
    evidence_id: "unit-policy-evidence:project-storage-audit",
    unit_system_ref: reference("UnitSystem", "unit-system:dec-018-si-dual-display"),
    storage_convention: "entered_units_preserved",
    storage_unit_policy: "local_project_storage_audit_records_unit_round_trip_status_without_conversion",
    model_units: sortedStringRecord(model.project.units),
    unit_round_trip_status: projectSummary?.unit_round_trip_status ?? "not_persisted_this_session",
    unit_round_trip_checked_ref_count: projectSummary?.unit_round_trip_checked_ref_count ?? 0,
    unit_round_trip_signature: projectSummary?.unit_round_trip_signature ?? "not_persisted",
    conversion_policy: "project_storage_audit_reports_persistence_unit_metadata_no_conversion",
    conversion_performed: false,
    source: "apps/desktop/src/features/project-storage/ProjectStorageAuditPanel.tsx",
    decision_basis_refs: [
      reference("Decision", "DEC-018"),
      reference("Deliverable", "DEL-02-02"),
      reference("Deliverable", "DEL-02-05")
    ]
  };
}

function storageDiagnostics(storageCapability: LocalStorageCapability | null, projectSummary: LocalProjectSummary | null) {
  const diagnostics = [
    diagnostic(
      "STORAGE-AUDIT-LOCAL-ONLY",
      "info",
      "Project storage is local evidence; acceptance and professional judgment remain with the responsible engineer."
    )
  ];
  if (!storageCapability) {
    diagnostics.push(diagnostic("STORAGE-CAPABILITY-PENDING", "warning", "Local storage capability has not loaded yet."));
  }
  if (storageCapability && !storageCapability.fts5_available) {
    diagnostics.push(diagnostic("STORAGE-FTS5-NOT-AVAILABLE", "info", "Browser fallback is not FTS5 indexed."));
  }
  if (!projectSummary) {
    diagnostics.push(diagnostic("STORAGE-SNAPSHOT-NOT-WRITTEN", "warning", "No local create/open/save summary is available."));
  }
  return diagnostics;
}

function diagnostic(code: string, severity: "info" | "warning", message: string) {
  return {
    code,
    severity,
    message,
    source: "apps/desktop/src/features/project-storage/ProjectStorageAuditPanel.tsx"
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

function reference(objectType: string, ref: string) {
  return { object_type: objectType, ref };
}

function sortedStringRecord(values: Record<string, string>): Record<string, string> {
  return Object.fromEntries(Object.entries(values).sort(([left], [right]) => left.localeCompare(right)));
}

function formatUnitRecord(units: Record<string, string>): string {
  return Object.entries(units)
    .map(([key, value]) => `${key}=${value}`)
    .join(",");
}

function unique(values: string[]): string[] {
  return Array.from(new Set(values)).sort();
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}
import { ControlledExportLink } from "../redaction-controls/ControlledExportLink";
