// Model document 0.9.0 S3b (DEL-02-01 Work `model-document-0.9.0.md` §4.2, §10): the
// load manager's per-pipe condition-resolution preview. A thin wrapper, beside
// operationService, over the engine's read-only export
// `preview_operating_conditions_json` (`operating_conditions::preview_operating_conditions`,
// the solve's own parse, document checks, refusals and resolution plan). The wasm
// engine is the one the desktop already loads in every environment; no native
// command is added. The desktop computes no ΔT and judges nothing here: it
// reports the engine's records and diagnostics as given. Nothing is written to
// the model.
import { loadWasmEngine } from "./wasmEngine/loadWasmEngine";
import type { PreviewModel } from "../types";

export const OPERATING_CONDITION_PREVIEW_DOCUMENT_KIND = "openpipestress.operating_condition_preview";

export type OperatingConditionPreviewRecord = {
  id: string;
  category: string;
  target: { type: string; pipe: string } | string;
  direction: string;
  magnitude: { value: number; unit: string };
  dimension: string;
  provenance: string;
};

export type OperatingConditionPreviewDiagnostic = {
  id?: string;
  code: string;
  severity: string;
  message: string;
  affected_refs?: string[];
};

export type OperatingConditionPreview = {
  document_kind: string;
  /** Condition resolution only; never a statement that the model solves. */
  resolved: boolean;
  /** The request parse's refusal text, when the document could not be read. */
  refusal?: string;
  cases: Array<{
    case_id: string;
    generated_primitive_loads: OperatingConditionPreviewRecord[];
    /** null when the case does not take its wind exposure from the sets. */
    wind_exposed_pipe_refs: string[] | null;
  }>;
  diagnostics: OperatingConditionPreviewDiagnostic[];
};

type InputErrorEnvelope = { document_kind: string; error?: { code?: string; message?: string } };

/** The engine's resolution preview of `model`, as the engine reports it. Throws when the
 * engine is unavailable or rejects its input; the caller shows that, never a substitute. */
export async function previewOperatingConditions(model: PreviewModel): Promise<OperatingConditionPreview> {
  const engine = await loadWasmEngine();
  const parsed = JSON.parse(engine.previewOperatingConditionsJson(JSON.stringify(model))) as OperatingConditionPreview | InputErrorEnvelope;
  if (parsed.document_kind !== OPERATING_CONDITION_PREVIEW_DOCUMENT_KIND) {
    const envelope = parsed as InputErrorEnvelope;
    throw new Error(`${envelope.error?.code ?? "OPERATING-CONDITION-PREVIEW-UNAVAILABLE"}: ${envelope.error?.message ?? "the engine returned no condition preview"}`);
  }
  return parsed as OperatingConditionPreview;
}
