import { invoke } from "@tauri-apps/api/core";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  EditorOperationIntent,
  LocalProjectEnvelope,
  ModelDocumentMigrationStatus,
  LocalProjectIndexEntry,
  LocalStorageCapability,
  MechanicsResult,
  ModelHashEvidence,
  PreviewModel,
  ProjectEnvelopeHashEvidence,
  SelectedReviewTarget
} from "../types";

let browserPreviewSnapshot: LocalProjectEnvelope | null = null;
let browserPreviewSnapshotCreatedAtUnix = 0;
let browserPreviewSnapshotUpdatedAtUnix = 0;

function recordBrowserSnapshotTimestamps(resetCreated: boolean): void {
  const nowUnix = Math.floor(Date.now() / 1000);
  if (resetCreated || browserPreviewSnapshotCreatedAtUnix === 0) {
    browserPreviewSnapshotCreatedAtUnix = nowUnix;
  }
  browserPreviewSnapshotUpdatedAtUnix = nowUnix;
}

function hasTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

// DEC-033 (2026-06-12): 0.2.0 adds the optional combination shape fields from
// TP-APP-R2-COMBEXPR-001; minor bumps for any change to what a document can
// contain. Mirrors apps/desktop/src-tauri/src/model_document_migration.rs.
export const SUPPORTED_MODEL_SCHEMA_VERSION = "0.2.0";
const MODEL_MIGRATION_FRAMEWORK = "application_service_separate_db_and_product_schema";

type LocalModelDocumentMigration = {
  source_version: string;
  target_version: string;
  migration_id: string;
  apply: (model: PreviewModel) => PreviewModel;
};

// Browser-preview mirror of the backend published transform chain.
// 0.1.0 -> 0.2.0 (DEC-033): the 0.2.0 shape only adds optional combination
// members, so every valid 0.1.0 document is a valid 0.2.0 document unchanged;
// the transform is a documented no-op and the chain walker stamps the version.
const MODEL_DOCUMENT_MIGRATIONS_LOCAL: LocalModelDocumentMigration[] = [
  {
    source_version: "0.1.0",
    target_version: "0.2.0",
    migration_id: "model-doc-0.1.0-to-0.2.0-additive-combination-shape-noop",
    apply: (model) => model
  }
];

function parseSemver(raw: string): [number, number, number] | null {
  const match = /^(\d+)\.(\d+)\.(\d+)$/.exec(raw.trim());
  if (!match) return null;
  return [Number(match[1]), Number(match[2]), Number(match[3])];
}

// Browser-preview mirror of the backend DEC-019 evaluation: documents are
// current, migrated in memory along the mirrored transform chain, refused as
// newer than supported, or refused as unsupported.
export function evaluateModelDocumentLocal(model: PreviewModel): ModelDocumentMigrationStatus {
  return migrateModelDocumentLocal(model).status;
}

function migrateModelDocumentLocal(model: PreviewModel): {
  model: PreviewModel;
  status: ModelDocumentMigrationStatus;
} {
  const raw = typeof model.schema_version === "string" ? model.schema_version : "";
  const documentVersion = parseSemver(raw);
  const supported = parseSemver(SUPPORTED_MODEL_SCHEMA_VERSION);
  const base = {
    source_schema_version: raw || "missing_schema_version",
    target_schema_version: SUPPORTED_MODEL_SCHEMA_VERSION,
    migration_framework: MODEL_MIGRATION_FRAMEWORK,
    db_migration_status: "browser_memory_preview_no_store_ledger",
    applied_migration_ids: [] as string[]
  };
  if (!documentVersion || !supported) {
    return {
      model,
      status: {
        ...base,
        status: "unsupported_schema",
        product_schema_migration_status: "unsupported_schema",
        persistence_state: "not_applicable_document_refused",
        detail: "Model document has no valid semver schema_version; refusing without coercion."
      }
    };
  }
  const compare =
    documentVersion[0] - supported[0] || documentVersion[1] - supported[1] || documentVersion[2] - supported[2];
  if (compare === 0) {
    return {
      model,
      status: {
        ...base,
        status: "current",
        product_schema_migration_status: "current",
        persistence_state: "stored_document_current",
        detail: "Model document schema_version matches the supported version; no migration applied."
      }
    };
  }
  if (compare > 0) {
    return {
      model,
      status: {
        ...base,
        status: "newer_than_supported",
        product_schema_migration_status: "newer_than_supported",
        persistence_state: "not_applicable_document_refused",
        detail: `Model document schema_version ${raw} is newer than the supported ${SUPPORTED_MODEL_SCHEMA_VERSION}; refusing to open for editing (no down-migration).`
      }
    };
  }
  // Older than supported: walk the mirrored transform chain in memory.
  let currentModel = model;
  let currentVersionRaw = raw;
  const appliedMigrationIds: string[] = [];
  for (;;) {
    if (currentVersionRaw === SUPPORTED_MODEL_SCHEMA_VERSION) {
      return {
        model: currentModel,
        status: {
          ...base,
          applied_migration_ids: appliedMigrationIds,
          status: "migrated",
          product_schema_migration_status: "migrated",
          persistence_state: "in_memory_only_not_yet_saved",
          detail: `Model document migrated in memory from ${raw} to ${SUPPORTED_MODEL_SCHEMA_VERSION}; stored bytes are unchanged until save.`
        }
      };
    }
    const step = MODEL_DOCUMENT_MIGRATIONS_LOCAL.find((item) => item.source_version === currentVersionRaw);
    if (!step) {
      return {
        model,
        status: {
          ...base,
          applied_migration_ids: appliedMigrationIds,
          status: "unsupported_schema",
          product_schema_migration_status: "unsupported_schema",
          persistence_state: "not_applicable_document_refused",
          detail: `No migration path exists in the browser preview from ${currentVersionRaw} to ${SUPPORTED_MODEL_SCHEMA_VERSION}; refusing without coercion.`
        }
      };
    }
    currentModel = { ...step.apply(cloneModel(currentModel)), schema_version: step.target_version };
    appliedMigrationIds.push(step.migration_id);
    currentVersionRaw = step.target_version;
  }
}

function requireEditableModelDocument(model: PreviewModel): {
  model: PreviewModel;
  status: ModelDocumentMigrationStatus;
} {
  const evaluation = migrateModelDocumentLocal(model);
  if (evaluation.status.status !== "current" && evaluation.status.status !== "migrated") {
    const status = evaluation.status;
    throw new Error(
      `Model document refused for editing: status=${status.status}; source_schema_version=${status.source_schema_version}; target_schema_version=${status.target_schema_version}; ${status.detail}`
    );
  }
  return evaluation;
}

function localOnlyCapability(): LocalStorageCapability {
  return {
    engine: "Browser memory preview",
    bundled: true,
    fts5_available: false,
    network_required: false,
    daemon_required: false,
    telemetry_enabled: false,
    path_policy: "browser-session memory fallback; no repository-default writes",
    large_file_policy: "reference external files by path/hash metadata; do not silently copy large files",
    database_path: "browser-session memory fallback",
    compile_options: [],
    migration_framework: BROWSER_MIGRATION_FRAMEWORK,
    migration_status: BROWSER_MIGRATION_STATUS,
    store_schema_version: 0,
    store_schema_target_version: 0,
    migrations_applied_on_open: []
  };
}

const BROWSER_MIGRATION_FRAMEWORK = "browser_memory_preview_no_sqlite_migration_ledger";
const BROWSER_MIGRATION_STATUS = "browser_memory_snapshot_no_sql_store_migrations_applicable";

function envelope(
  model: PreviewModel,
  editorIntents: EditorOperationIntent[],
  proposal: AgentProposal | null,
  selectedReviewTarget: SelectedReviewTarget | null,
  mechanicsResult: MechanicsResult | null,
  analysisRun: AnalysisRunEnvelope | null,
  modelHash: ModelHashEvidence | null,
  projectEnvelopeHash: ProjectEnvelopeHashEvidence | null,
  message: string
): LocalProjectEnvelope {
  // DEC-019/DEC-033 mirror: older documents migrate in memory along the
  // mirrored chain and newer/unsupported documents are refused. The browser
  // preview has no migration ledger, so it never persists rewritten bytes:
  // the snapshot stores the document bytes as given and the migration remains
  // in-memory evidence (no silent destructive rewrite without a ledger).
  const documentStatus = { ...requireEditableModelDocument(model).status };
  if (documentStatus.status === "migrated") {
    documentStatus.detail = `Model document migrates in memory from ${documentStatus.source_schema_version} to ${documentStatus.target_schema_version}; the browser-memory snapshot keeps the stored bytes unchanged because no migration ledger exists in the browser preview.`;
  }
  const snapshot = cloneModel(model);
  return {
    summary: {
      project_id: snapshot.project.id,
      project_name: snapshot.project.name,
      database_path: "browser-session memory fallback",
      storage_mode: "browser_memory_preview",
      migration_status: BROWSER_MIGRATION_STATUS,
      migration_framework: BROWSER_MIGRATION_FRAMEWORK,
      store_schema_version: 0,
      store_schema_target_version: 0,
      migrations_applied_on_open: [],
      fts_indexed: false,
      copied_external_files: false,
      editor_intent_count: editorIntents.length,
      proposal_count: proposal ? 1 : 0,
      selected_review_target_count: selectedReviewTarget ? 1 : 0,
      selected_review_target_ref: selectedReviewTargetRef(selectedReviewTarget),
      persisted_mechanics_result_count: mechanicsResult ? 1 : 0,
      persisted_analysis_run_count: analysisRun ? 1 : 0,
      persisted_analysis_run_ref: persistedAnalysisRunRef(analysisRun, mechanicsResult),
      persisted_model_hash_count: modelHash ? 1 : 0,
      persisted_model_hash_ref: modelHash?.value ?? "not_persisted",
      persisted_project_envelope_hash_count: projectEnvelopeHash ? 1 : 0,
      persisted_project_envelope_hash_ref: projectEnvelopeHash?.value ?? "not_persisted",
      message
    },
    model: snapshot,
    editor_intents: cloneEditorIntents(editorIntents),
    proposal: cloneProposal(proposal),
    selected_review_target: cloneSelectedReviewTarget(selectedReviewTarget),
    mechanics_result: cloneJson(mechanicsResult),
    analysis_run: cloneJson(analysisRun),
    model_hash: cloneJson(modelHash),
    project_envelope_hash: cloneJson(projectEnvelopeHash),
    model_document_migration: documentStatus,
    model_migration_ledger: []
  };
}

function cloneEnvelope(project: LocalProjectEnvelope): LocalProjectEnvelope {
  return {
    summary: { ...project.summary },
    model: cloneModel(project.model),
    editor_intents: cloneEditorIntents(project.editor_intents ?? []),
    proposal: cloneProposal(project.proposal),
    selected_review_target: cloneSelectedReviewTarget(project.selected_review_target),
    mechanics_result: cloneJson(project.mechanics_result),
    analysis_run: cloneJson(project.analysis_run),
    model_hash: cloneJson(project.model_hash),
    project_envelope_hash: cloneJson(project.project_envelope_hash),
    model_document_migration: cloneJson(project.model_document_migration),
    model_migration_ledger: JSON.parse(JSON.stringify(project.model_migration_ledger ?? []))
  };
}

function cloneModel(model: PreviewModel): PreviewModel {
  return JSON.parse(JSON.stringify(model)) as PreviewModel;
}

function cloneEditorIntents(editorIntents: EditorOperationIntent[]): EditorOperationIntent[] {
  return JSON.parse(JSON.stringify(editorIntents)) as EditorOperationIntent[];
}

function cloneProposal(proposal: AgentProposal | null | undefined): AgentProposal | null {
  if (!proposal) return null;
  return JSON.parse(JSON.stringify(proposal)) as AgentProposal;
}

function cloneSelectedReviewTarget(
  selectedReviewTarget: SelectedReviewTarget | null | undefined
): SelectedReviewTarget | null {
  if (!selectedReviewTarget) return null;
  return JSON.parse(JSON.stringify(selectedReviewTarget)) as SelectedReviewTarget;
}

function cloneJson<T>(value: T | null | undefined): T | null {
  if (!value) return null;
  return JSON.parse(JSON.stringify(value)) as T;
}

function selectedReviewTargetRef(selectedReviewTarget: SelectedReviewTarget | null): string {
  if (!selectedReviewTarget) return "not_selected";
  return `${selectedReviewTarget.target_type}: ${selectedReviewTarget.id}`;
}

function persistedAnalysisRunRef(
  analysisRun: AnalysisRunEnvelope | null,
  mechanicsResult: MechanicsResult | null
): string {
  return analysisRun?.analysis_run?.run_id ?? mechanicsResult?.run_id ?? "not_persisted";
}

export async function getLocalStorageCapability(): Promise<LocalStorageCapability> {
  if (!hasTauriRuntime()) return localOnlyCapability();
  return invoke<LocalStorageCapability>("get_local_storage_capability");
}

export function buildBlankLocalModelDocument(createdAt: Date = new Date()): PreviewModel {
  const stamp = createdAt.toISOString().replace(/\.\d{3}Z$/, "Z");
  const token = stamp.replace(/[^0-9TZ]/g, "").toLowerCase();
  const projectId = `project:blank-local-${token}`;
  return {
    schema_version: SUPPORTED_MODEL_SCHEMA_VERSION,
    document_kind: "openpipestress.product_preview.model",
    data_boundary: {
      public_examples_policy: "blank_user_created_local_document_no_bundled_engineering_values",
      protected_source_policy: "no_protected_standards_content_inserted",
      private_data_policy: "local_user_document_not_committed_to_repository",
      professional_boundary: "human_review_required_no_software_approval_claim"
    },
    project: {
      id: projectId,
      name: "Blank Local Model",
      description:
        "User-created local blank model document. Add explicit nodes, pipe runs, supports, materials, loads, and combinations through structured editors before solving.",
      units: {
        length: "m",
        force: "N",
        moment: "N*m",
        pressure: "Pa",
        stress: "Pa",
        temperature: "C"
      }
    },
    analysis_status: {
      mechanics: "MODEL_INCOMPLETE",
      rule_check: "RULE_INPUTS_INCOMPLETE",
      professional_acceptance: "NOT_PROVIDED"
    },
    materials: [],
    sections: [],
    nodes: [],
    pipe_segments: [],
    supports: [],
    components: [],
    load_cases: [],
    combinations: [],
    diagnostics: [
      {
        id: "diagnostic:blank-project:authoring-target",
        code: "BLANK_PROJECT_AUTHORING_TARGET",
        severity: "blocking",
        source: "apps/desktop/src/services/projectService.ts",
        affected_refs: [projectId],
        message:
          "Blank local model document is intentionally incomplete; no fixture entities, hidden loads, or engineering defaults were inserted."
      }
    ]
  };
}

export async function createLocalProject(
  model: PreviewModel,
  editorIntents: EditorOperationIntent[] = [],
  proposal: AgentProposal | null = null,
  selectedReviewTarget: SelectedReviewTarget | null = null,
  mechanicsResult: MechanicsResult | null = null,
  analysisRun: AnalysisRunEnvelope | null = null,
  modelHash: ModelHashEvidence | null = null,
  projectEnvelopeHash: ProjectEnvelopeHashEvidence | null = null
): Promise<LocalProjectEnvelope> {
  if (!hasTauriRuntime()) {
    browserPreviewSnapshot = envelope(
      model,
      editorIntents,
      proposal,
      selectedReviewTarget,
      mechanicsResult,
      analysisRun,
      modelHash,
      projectEnvelopeHash,
      "Created local browser-preview project snapshot without external file copies."
    );
    recordBrowserSnapshotTimestamps(true);
    return cloneEnvelope(browserPreviewSnapshot);
  }
  return invoke<LocalProjectEnvelope>("create_local_project", {
    model,
    editorIntents,
    proposal,
    selectedReviewTarget,
    mechanicsResult,
    analysisRun,
    modelHash,
    projectEnvelopeHash
  });
}

export async function openLocalProject(projectId: string | null = null): Promise<LocalProjectEnvelope | null> {
  if (!hasTauriRuntime()) {
    if (!browserPreviewSnapshot) return null;
    if (projectId && browserPreviewSnapshot.summary.project_id !== projectId) return null;
    requireEditableModelDocument(browserPreviewSnapshot.model);
    const opened = cloneEnvelope(browserPreviewSnapshot);
    opened.summary.message = projectId
      ? `Opened local browser-preview project snapshot by id ${projectId}.`
      : "Opened local browser-preview project snapshot.";
    return opened;
  }
  return invoke<LocalProjectEnvelope | null>("open_local_project", { projectId });
}

export async function listLocalProjects(): Promise<LocalProjectIndexEntry[]> {
  if (!hasTauriRuntime()) {
    if (!browserPreviewSnapshot) return [];
    return [
      {
        project_id: browserPreviewSnapshot.summary.project_id,
        project_name: browserPreviewSnapshot.summary.project_name,
        storage_mode: "browser_memory_preview",
        created_at_unix: browserPreviewSnapshotCreatedAtUnix,
        updated_at_unix: browserPreviewSnapshotUpdatedAtUnix
      }
    ];
  }
  return invoke<LocalProjectIndexEntry[]>("list_local_projects");
}

export async function saveLocalProject(
  model: PreviewModel,
  editorIntents: EditorOperationIntent[] = [],
  proposal: AgentProposal | null = null,
  selectedReviewTarget: SelectedReviewTarget | null = null,
  mechanicsResult: MechanicsResult | null = null,
  analysisRun: AnalysisRunEnvelope | null = null,
  modelHash: ModelHashEvidence | null = null,
  projectEnvelopeHash: ProjectEnvelopeHashEvidence | null = null,
  modelDocumentMigration: ModelDocumentMigrationStatus | null = null
): Promise<LocalProjectEnvelope> {
  if (!hasTauriRuntime()) {
    browserPreviewSnapshot = envelope(
      model,
      editorIntents,
      proposal,
      selectedReviewTarget,
      mechanicsResult,
      analysisRun,
      modelHash,
      projectEnvelopeHash,
      "Saved local browser-preview project snapshot without external file copies."
    );
    recordBrowserSnapshotTimestamps(false);
    return cloneEnvelope(browserPreviewSnapshot);
  }
  return invoke<LocalProjectEnvelope>("save_local_project", {
    request: {
      project_id: model.project.id,
      project_name: model.project.name,
      model,
      editor_intents: editorIntents,
      proposal,
      selected_review_target: selectedReviewTarget,
      mechanics_result: mechanicsResult,
      analysis_run: analysisRun,
      model_hash: modelHash,
      project_envelope_hash: projectEnvelopeHash,
      model_document_migration: modelDocumentMigration
    }
  });
}
