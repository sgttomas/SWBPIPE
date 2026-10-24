// The one document validator's findings for the session model, as entries of
// the existing diagnostics list (W-13 §3, §5.3, §6). They come from the same
// helper the stores use (`modelDocumentSlotFindings`), so the validator is the
// applier's Rust code through the wasm engine and the webview never
// re-implements it. When the engine cannot judge the document (not loaded, or
// an input error) that is listed too, as a warning: the list shows the
// document is unchecked rather than staying silent.
import type { Diagnostic, PreviewModel } from "../../types";
import { modelDocumentSlotFindings } from "../model-file/modelDocumentShape";

export const DOCUMENT_VALIDATOR_SOURCE = "core/model_operations/operation_applier/src/document_validator.rs";

export function modelDocumentDiagnostics(model: PreviewModel): Diagnostic[] {
  return modelDocumentSlotFindings(model).map((finding, index) => ({
    id: `model-document:${finding.code}:${finding.path || "document"}:${index}`,
    code: finding.code,
    severity: finding.origin === "engine" ? "warning" : finding.severity,
    message: finding.origin === "engine"
      ? `The model-data slots were not checked: ${finding.message}`
      : finding.path ? `${finding.message} (at ${finding.path})` : finding.message,
    affected_refs: finding.affected_refs,
    source: DOCUMENT_VALIDATOR_SOURCE
  }));
}
