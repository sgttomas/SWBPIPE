import { Download, HardDrive } from "lucide-react";
import type {
  AgentProposal,
  EditorOperationIntent,
  LocalProjectSummary,
  LocalStorageCapability,
  PreviewModel
} from "../../types";

export function ProjectStorageAuditPanel({
  model,
  storageCapability,
  projectSummary,
  projectMessage,
  projectOperation,
  editorIntents,
  proposal
}: {
  model: PreviewModel;
  storageCapability: LocalStorageCapability | null;
  projectSummary: LocalProjectSummary | null;
  projectMessage: string;
  projectOperation: string;
  editorIntents: EditorOperationIntent[];
  proposal: AgentProposal | null;
}) {
  const packet = buildProjectStorageAuditPacket({
    model,
    storageCapability,
    projectSummary,
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
        <a
          className="report-export-link"
          data-testid="project-storage-export-link"
          download={`openpipestress-preview-storage-audit-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Local storage JSON
        </a>
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
          }; persisted_proposals=${packet.summary.persisted_proposal_count}; copied_external_files=${String(
            packet.summary.copied_external_files
          )}`}
          testId="project-storage-snapshot"
        />
        <StorageLine
          label="Payload boundary"
          value={`private payload=false; protected content=false; release/professional claim=false`}
          testId="project-storage-payload-boundary"
        />
      </div>
      <small className="report-note">
        Storage audit packets are local technical-preview evidence only; create/open/save operations do not accept model
        changes, copy external project files, or create professional reliance.
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
  projectMessage,
  projectOperation,
  editorIntents,
  proposal
}: {
  model: PreviewModel;
  storageCapability: LocalStorageCapability | null;
  projectSummary: LocalProjectSummary | null;
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

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.local_project_persistence_audit",
    export_scope: "local_browser_download_preview",
    deliverable_refs: ["DEL-02-05", "DEL-12-01", "DEL-12-02"],
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
      persisted_proposal_count: projectSummary?.proposal_count ?? 0,
      applied_operation_count: 0,
      accepted_model_state_mutated: false,
      network_required: Boolean(storageCapability?.network_required),
      daemon_required: Boolean(storageCapability?.daemon_required),
      telemetry_enabled: Boolean(storageCapability?.telemetry_enabled),
      copied_external_files: Boolean(projectSummary?.copied_external_files)
    },
    storage_capability: storageCapability,
    project_summary: projectSummary,
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

function storageDiagnostics(storageCapability: LocalStorageCapability | null, projectSummary: LocalProjectSummary | null) {
  const diagnostics = [
    diagnostic(
      "STORAGE-AUDIT-LOCAL-ONLY",
      "info",
      "Project storage is local technical-preview evidence and does not create professional acceptance."
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

function unique(values: string[]): string[] {
  return Array.from(new Set(values)).sort();
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}
