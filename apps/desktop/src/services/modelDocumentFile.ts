// File > Open Model Document…: the native chooser, file-level checks
// (`src-tauri/src/model_document_file.rs`), the desktop shape check and the
// one document validator through the wasm engine
// (`features/model-file/modelDocumentShape.ts`). The session adopts only an
// `opened` outcome; its document is the file's document, unchanged, and the
// validator's warnings travel with it.
import { invoke } from "@tauri-apps/api/core";
import { modelDocumentShapeDiagnostics, modelDocumentSlotValidation, type ModelFileDiagnostic } from "../features/model-file/modelDocumentShape";
import type { PreviewModel } from "../types";
import { isTauriRuntime } from "./nativeMenu";

export type { ModelFileDiagnostic };

export type ModelDocumentOpenOutcome =
  | { outcome: "cancelled" }
  | { outcome: "opened"; file_name: string; byte_count: number; schema_status: string; document: PreviewModel; warnings: ModelFileDiagnostic[] }
  | { outcome: "refused"; file_name: string; diagnostics: ModelFileDiagnostic[] };

type NativeOutcome =
  | { outcome: "cancelled" }
  | { outcome: "opened"; file_name: string; byte_count: number; schema_status: string; document: unknown; warnings?: ModelFileDiagnostic[] }
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

/** Apply the desktop shape check and the document validator to the native outcome. */
export function checkNativeOutcome(native: NativeOutcome): ModelDocumentOpenOutcome {
  if (native.outcome !== "opened") return native;
  const diagnostics = modelDocumentShapeDiagnostics(native.document);
  if (diagnostics.length > 0) return { outcome: "refused", file_name: native.file_name, diagnostics };
  const slots = modelDocumentSlotValidation(native.document);
  if (slots.blocking.length > 0) return { outcome: "refused", file_name: native.file_name, diagnostics: slots.blocking };
  return { ...native, document: native.document as PreviewModel, warnings: slots.warnings };
}

/** The open status line's warning suffix ("" when there is none). */
export function modelFileWarningSuffix(warnings: ModelFileDiagnostic[]): string {
  if (warnings.length === 0) return "";
  const shown = warnings.slice(0, 4).map((item) => `${item.code}${item.path ? ` at ${item.path}` : ""}`);
  const more = warnings.length > shown.length ? ` (+${warnings.length - shown.length} more)` : "";
  return ` Warnings: ${shown.join(" | ")}${more}.`;
}

/** One status line naming every refusal diagnostic. */
export function modelFileRefusalMessage(file_name: string, diagnostics: ModelFileDiagnostic[]): string {
  const shown = diagnostics.slice(0, 8).map((item) => `${item.code}${item.path ? ` at ${item.path}` : ""}: ${item.message}`);
  const more = diagnostics.length > shown.length ? ` (+${diagnostics.length - shown.length} more)` : "";
  return `Open model document refused${file_name ? ` (${file_name})` : ""}: ${shown.join(" | ")}${more}`;
}
