import { invoke } from "@tauri-apps/api/core";
import type { LocalProjectEnvelope, LocalStorageCapability, PreviewModel } from "../types";

function hasTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

function localOnlyCapability(): LocalStorageCapability {
  return {
    engine: "SQLite",
    bundled: true,
    fts5_available: true,
    network_required: false,
    daemon_required: false,
    telemetry_enabled: false,
    path_policy: "app-local user data path; no repository-default writes",
    large_file_policy: "reference external files by path/hash metadata; do not silently copy large files",
    database_path: "Tauri app-local data path",
    compile_options: ["ENABLE_FTS5"]
  };
}

function envelope(model: PreviewModel, message: string): LocalProjectEnvelope {
  return {
    summary: {
      project_id: model.project.id,
      project_name: model.project.name,
      database_path: "Tauri app-local data path",
      storage_mode: "local_sqlite",
      migration_status: "current",
      fts_indexed: true,
      copied_external_files: false,
      message
    },
    model
  };
}

export async function getLocalStorageCapability(): Promise<LocalStorageCapability> {
  if (!hasTauriRuntime()) return localOnlyCapability();
  return invoke<LocalStorageCapability>("get_local_storage_capability");
}

export async function createLocalProject(model: PreviewModel): Promise<LocalProjectEnvelope> {
  if (!hasTauriRuntime()) {
    return envelope(model, "Created local test project snapshot without external file copies.");
  }
  return invoke<LocalProjectEnvelope>("create_local_project", { model });
}

export async function openLocalProject(): Promise<LocalProjectEnvelope | null> {
  if (!hasTauriRuntime()) return null;
  return invoke<LocalProjectEnvelope | null>("open_local_project", { projectId: null });
}

export async function saveLocalProject(model: PreviewModel): Promise<LocalProjectEnvelope> {
  if (!hasTauriRuntime()) {
    return envelope(model, "Saved local test project snapshot without external file copies.");
  }
  return invoke<LocalProjectEnvelope>("save_local_project", {
    request: {
      project_id: model.project.id,
      project_name: model.project.name,
      model
    }
  });
}
