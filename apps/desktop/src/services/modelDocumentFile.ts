// File > Open Model Document…: the native chooser, file-level checks
// (`src-tauri/src/model_document_file.rs`) and the desktop shape check
// (`features/model-file/modelDocumentShape.ts`). The session adopts only an
// `opened` outcome; its document is the file's document, unchanged.
import { invoke } from "@tauri-apps/api/core";
import { modelDocumentShapeDiagnostics, type ModelFileDiagnostic } from "../features/model-file/modelDocumentShape";
import type { PreviewModel } from "../types";
import { isTauriRuntime } from "./nativeMenu";

export type { ModelFileDiagnostic };

export type ModelDocumentOpenOutcome =
  | { outcome: "cancelled" }
  | { outcome: "opened"; file_name: string; byte_count: number; schema_status: string; document: PreviewModel }
  | { outcome: "refused"; file_name: string; diagnostics: ModelFileDiagnostic[] };

type NativeOutcome =
  | { outcome: "cancelled" }
  | { outcome: "opened"; file_name: string; byte_count: number; schema_status: string; document: unknown }
  | { outcome: "refused"; file_name: string; diagnostics: ModelFileDiagnostic[] };

export const MODEL_FILE_NATIVE_ONLY = "MODEL-FILE-NATIVE-ONLY";

export async function openModelDocumentFile(): Promise<ModelDocumentOpenOutcome> {
  if (!isTauriRuntime()) {
    return {
      outcome: "refused",
      file_name: "",
      diagnostics: [{ code: MODEL_FILE_NATIVE_ONLY, path: "", message: "Opening a model document file needs the desktop app." }]
    };
  }
  return checkNativeOutcome(await invoke<NativeOutcome>("open_model_document_file"));
}

/** Apply the desktop shape check to the native outcome. */
export function checkNativeOutcome(native: NativeOutcome): ModelDocumentOpenOutcome {
  if (native.outcome !== "opened") return native;
  const diagnostics = modelDocumentShapeDiagnostics(native.document);
  if (diagnostics.length > 0) return { outcome: "refused", file_name: native.file_name, diagnostics };
  return { ...native, document: native.document as PreviewModel };
}

/** One status line naming every refusal diagnostic. */
export function modelFileRefusalMessage(file_name: string, diagnostics: ModelFileDiagnostic[]): string {
  const shown = diagnostics.slice(0, 8).map((item) => `${item.code}${item.path ? ` at ${item.path}` : ""}: ${item.message}`);
  const more = diagnostics.length > shown.length ? ` (+${diagnostics.length - shown.length} more)` : "";
  return `Open model document refused${file_name ? ` (${file_name})` : ""}: ${shown.join(" | ")}${more}`;
}
