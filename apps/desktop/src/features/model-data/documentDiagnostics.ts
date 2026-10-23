// The one document validator's findings for the session model, as entries of
// the existing diagnostics list (W-13 §3, §5.3, §6). The validator is the
// applier's Rust code through the wasm engine; the webview never
// re-implements it. Before the engine has loaded there is nothing to list
// here (the engine's absence is reported where operations run).
import type { Diagnostic, PreviewModel } from "../../types";
import { loadedWasmEngine } from "../../services/wasmEngine/loadWasmEngine";

export const DOCUMENT_VALIDATOR_SOURCE = "core/model_operations/operation_applier/src/document_validator.rs";

type Finding = { code: string; severity: string; path: string; message: string; affected_refs?: string[] };

export function modelDocumentDiagnostics(model: PreviewModel): Diagnostic[] {
  const engine = loadedWasmEngine();
  if (!engine) return [];
  let report: { diagnostics?: Finding[] };
  try {
    report = JSON.parse(engine.validateModelDocumentJson(JSON.stringify(model))) as { diagnostics?: Finding[] };
  } catch {
    return [];
  }
  return (report.diagnostics ?? []).map((finding, index) => ({
    id: `model-document:${finding.code}:${finding.path || "document"}:${index}`,
    code: finding.code,
    severity: finding.severity === "blocking" ? "blocking" : "warning",
    message: finding.path ? `${finding.message} (at ${finding.path})` : finding.message,
    affected_refs: finding.affected_refs ?? [],
    source: DOCUMENT_VALIDATOR_SOURCE
  }));
}
