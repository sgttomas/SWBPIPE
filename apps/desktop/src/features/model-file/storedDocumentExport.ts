// Recovery export of a stored model document (W-17, DEL-02-02 Design
// `project-unit-selection.md` §3). A stored project or saved state whose
// document is refused at open for lacking `project.units` (`MODEL-FILE-SHAPE`)
// can be saved to a file the user chooses, exactly as the store holds it: a
// project's stored model JSON text, or a saved state's canonical (RFC 8785)
// text. The user adds the map and opens the file with Open Model Document….
//
// Nothing is adopted into the session and nothing is computed from the
// document here: the webview sends only the stored row's identity and the
// explicit intent. The native command (`src-tauri/src/stored_model_document_export.rs`)
// opens the store read-only (a store needing migration is refused, never
// migrated), reads the row within the memory pool's admission and the model
// document file cap, refuses a text with a duplicate object member, screens its
// dispositions as Save Model Document does, shows the Save Model Document
// chooser and writes the bytes unchanged.
import { invoke } from "@tauri-apps/api/core";
import { nativeFailureText } from "../../services/nativeIngress";
import { isTauriRuntime } from "../../services/nativeMenu";
import {
  MODEL_DOC_SAVE_DESKTOP_ONLY,
  MODEL_DOC_SAVE_INTENT_REQUIRED,
  readOutcome,
  type ModelDocumentSaveOutcome
} from "./modelDocumentSave";

export const STORED_DOCUMENT_EXPORT_INTENT_LABEL =
  "Save the stored model document, including private project data, to a file I choose";

/** The stored row to export: a project's document (`stateId` null) or one of its saved states. */
export type StoredDocumentExportTarget = {
  projectId: string;
  stateId: string | null;
  /** How the row is named in the dialog and the status line. */
  description: string;
  /** The open's refusal, shown in the dialog. */
  refusal: string;
};

export function storedProjectDescription(projectId: string): string {
  return `the stored project \`${projectId}\``;
}

/** Writes the stored row's bytes to a file the user chooses; never opens it. */
export async function saveStoredModelDocumentFile(
  target: StoredDocumentExportTarget,
  explicitLocalPrivateIntent: boolean
): Promise<ModelDocumentSaveOutcome> {
  if (explicitLocalPrivateIntent !== true) {
    return { outcome: "refused", diagnostics: [{ code: MODEL_DOC_SAVE_INTENT_REQUIRED, path: "", message: `Saving the stored document needs the explicit choice "${STORED_DOCUMENT_EXPORT_INTENT_LABEL}".` }] };
  }
  if (!isTauriRuntime()) {
    return { outcome: "refused", diagnostics: [{ code: MODEL_DOC_SAVE_DESKTOP_ONLY, path: "", message: "Saving a stored document to a file needs the desktop app." }] };
  }
  return readOutcome(await invoke<unknown>("save_stored_model_document_file", {
    projectId: target.projectId,
    stateId: target.stateId,
    explicitLocalPrivateIntent: true
  }));
}

/** The status line for the export's outcome. */
export function storedDocumentExportMessage(target: StoredDocumentExportTarget, outcome: ModelDocumentSaveOutcome): string {
  if (outcome.outcome === "cancelled") return `Save of ${target.description} cancelled; nothing was written.`;
  if (outcome.outcome === "refused") {
    const shown = outcome.diagnostics.slice(0, 8).map((item) => `${item.code}${item.path ? ` at ${item.path}` : ""}: ${item.message}`);
    const more = outcome.diagnostics.length > shown.length ? ` (+${outcome.diagnostics.length - shown.length} more)` : "";
    return `Save of ${target.description} refused: ${shown.join(" | ")}${more}`;
  }
  const durability = outcome.durability === "durable" ? "" : " The file was saved, but its directory sync did not complete; its durability is uncertain.";
  const warnings = outcome.warnings.length > 0 ? ` Warnings: ${outcome.warnings.slice(0, 4).map((item) => `${item.code}${item.path ? ` at ${item.path}` : ""}`).join(" | ")}.` : "";
  return `Saved ${target.description} to ${outcome.file_name} exactly as stored (${outcome.byte_count} bytes; SHA-256 ${outcome.model_hash}). It was not opened. Add project.units to the file, then open it with Open Model Document….${durability}${warnings}`;
}

export function storedDocumentExportFailureMessage(target: StoredDocumentExportTarget, error: unknown): string {
  return `Save of ${target.description} failed: ${nativeFailureText(error)}`;
}
