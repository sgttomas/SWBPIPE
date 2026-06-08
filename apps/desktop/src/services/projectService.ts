import { invoke } from "@tauri-apps/api/core";
import type {
  AgentProposal,
  EditorOperationIntent,
  LocalProjectEnvelope,
  LocalStorageCapability,
  PreviewModel
} from "../types";

let browserPreviewSnapshot: LocalProjectEnvelope | null = null;

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
      proposal_count: proposal ? 1 : 0,
      message
    },
    model: snapshot,
    editor_intents: cloneEditorIntents(editorIntents),
    proposal: cloneProposal(proposal)
  };
}

function cloneEnvelope(project: LocalProjectEnvelope): LocalProjectEnvelope {
  return {
    summary: { ...project.summary },
    model: cloneModel(project.model),
    editor_intents: cloneEditorIntents(project.editor_intents ?? []),
    proposal: cloneProposal(project.proposal)
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

export async function getLocalStorageCapability(): Promise<LocalStorageCapability> {
  if (!hasTauriRuntime()) return localOnlyCapability();
  return invoke<LocalStorageCapability>("get_local_storage_capability");
}

export async function createLocalProject(
  model: PreviewModel,
  editorIntents: EditorOperationIntent[] = [],
  proposal: AgentProposal | null = null
): Promise<LocalProjectEnvelope> {
  if (!hasTauriRuntime()) {
    browserPreviewSnapshot = envelope(
      model,
      editorIntents,
      proposal,
      "Created local browser-preview project snapshot without external file copies."
    );
    return cloneEnvelope(browserPreviewSnapshot);
  }
  return invoke<LocalProjectEnvelope>("create_local_project", { model, editorIntents, proposal });
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

export async function saveLocalProject(
  model: PreviewModel,
  editorIntents: EditorOperationIntent[] = [],
  proposal: AgentProposal | null = null
): Promise<LocalProjectEnvelope> {
  if (!hasTauriRuntime()) {
    browserPreviewSnapshot = envelope(
      model,
      editorIntents,
      proposal,
      "Saved local browser-preview project snapshot without external file copies."
    );
    return cloneEnvelope(browserPreviewSnapshot);
  }
  return invoke<LocalProjectEnvelope>("save_local_project", {
    request: {
      project_id: model.project.id,
      project_name: model.project.name,
      model,
      editor_intents: editorIntents,
      proposal
    }
  });
}
