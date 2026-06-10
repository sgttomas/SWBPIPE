import { invoke } from "@tauri-apps/api/core";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  EditorOperationIntent,
  LocalProjectEnvelope,
  LocalProjectIndexEntry,
  LocalStorageCapability,
  MechanicsResult,
  PreviewModel,
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
    compile_options: []
  };
}

function envelope(
  model: PreviewModel,
  editorIntents: EditorOperationIntent[],
  proposal: AgentProposal | null,
  selectedReviewTarget: SelectedReviewTarget | null,
  mechanicsResult: MechanicsResult | null,
  analysisRun: AnalysisRunEnvelope | null,
  message: string
): LocalProjectEnvelope {
  const snapshot = cloneModel(model);
  return {
    summary: {
      project_id: snapshot.project.id,
      project_name: snapshot.project.name,
      database_path: "browser-session memory fallback",
      storage_mode: "browser_memory_preview",
      migration_status: "current",
      fts_indexed: false,
      copied_external_files: false,
      editor_intent_count: editorIntents.length,
      proposal_count: proposal ? 1 : 0,
      selected_review_target_count: selectedReviewTarget ? 1 : 0,
      selected_review_target_ref: selectedReviewTargetRef(selectedReviewTarget),
      persisted_mechanics_result_count: mechanicsResult ? 1 : 0,
      persisted_analysis_run_count: analysisRun ? 1 : 0,
      persisted_analysis_run_ref: persistedAnalysisRunRef(analysisRun, mechanicsResult),
      message
    },
    model: snapshot,
    editor_intents: cloneEditorIntents(editorIntents),
    proposal: cloneProposal(proposal),
    selected_review_target: cloneSelectedReviewTarget(selectedReviewTarget),
    mechanics_result: cloneJson(mechanicsResult),
    analysis_run: cloneJson(analysisRun)
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
    analysis_run: cloneJson(project.analysis_run)
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

export async function createLocalProject(
  model: PreviewModel,
  editorIntents: EditorOperationIntent[] = [],
  proposal: AgentProposal | null = null,
  selectedReviewTarget: SelectedReviewTarget | null = null,
  mechanicsResult: MechanicsResult | null = null,
  analysisRun: AnalysisRunEnvelope | null = null
): Promise<LocalProjectEnvelope> {
  if (!hasTauriRuntime()) {
    browserPreviewSnapshot = envelope(
      model,
      editorIntents,
      proposal,
      selectedReviewTarget,
      mechanicsResult,
      analysisRun,
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
    analysisRun
  });
}

export async function openLocalProject(): Promise<LocalProjectEnvelope | null> {
  if (!hasTauriRuntime()) {
    if (!browserPreviewSnapshot) return null;
    const opened = cloneEnvelope(browserPreviewSnapshot);
    opened.summary.message = "Opened local browser-preview project snapshot.";
    return opened;
  }
  return invoke<LocalProjectEnvelope | null>("open_local_project", { projectId: null });
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
  analysisRun: AnalysisRunEnvelope | null = null
): Promise<LocalProjectEnvelope> {
  if (!hasTauriRuntime()) {
    browserPreviewSnapshot = envelope(
      model,
      editorIntents,
      proposal,
      selectedReviewTarget,
      mechanicsResult,
      analysisRun,
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
      analysis_run: analysisRun
    }
  });
}
