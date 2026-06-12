import { invoke } from "@tauri-apps/api/core";
import {
  loadWasmEngine,
  WASM_ENGINE_BUILD_COMMAND,
  type WasmOperationEngine
} from "./wasmEngine/loadWasmEngine";
import type { EditorOperationIntent, ModelHashEvidence, OperationOutcome, PreviewModel } from "../types";

// Thin routing adapter for the structured editor-operation seam (DEC-020 /
// ADR-0001): one engine, `core/model_operations/operation_applier`, in every
// environment. With a Tauri backend the native build answers through the
// `validate_model_operation` / `apply_model_operation` commands (the
// authoritative route, unchanged); without one (vite dev, jsdom tests,
// Playwright) the SAME crate answers as a wasm32 module. No TypeScript
// engine exists; this file contains routing and honest engine-status
// reporting only — never validation, diff, or apply semantics.

const WASM_INPUT_ERROR_DOCUMENT_KIND = "openpipestress.desktop.wasm_engine_input_error";

function tauriAvailable(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

type WasmEngineInputErrorEnvelope = {
  document_kind: string;
  error?: { code?: string; severity?: string; input?: string; message?: string };
};

// Parses a wasm-engine JSON return. The engine never traps on malformed
// input; it returns a structured input-error envelope instead, which this
// adapter surfaces as an explicit thrown error — the operation did not run.
export function outcomeFromWasmEngineJson(raw: string): OperationOutcome {
  const parsed = JSON.parse(raw) as OperationOutcome | WasmEngineInputErrorEnvelope;
  if (parsed.document_kind === WASM_INPUT_ERROR_DOCUMENT_KIND) {
    const envelope = parsed as WasmEngineInputErrorEnvelope;
    throw new Error(
      `${envelope.error?.code ?? "WASM-ENGINE-INPUT-JSON-INVALID"}: ${
        envelope.error?.message ?? "the wasm operation engine rejected its input as invalid JSON"
      }`
    );
  }
  return parsed as OperationOutcome;
}

async function runWasmOperation(
  model: PreviewModel,
  intent: EditorOperationIntent,
  modelHash: ModelHashEvidence | null,
  mode: "validate_only" | "apply"
): Promise<OperationOutcome> {
  const engine = await loadWasmEngine();
  const run = mode === "apply" ? engine.applyOperationJson : engine.validateOperationJson;
  const raw = run(JSON.stringify(model), JSON.stringify(intent), JSON.stringify(modelHash ?? null));
  return outcomeFromWasmEngineJson(raw);
}

export async function validateModelOperation(
  model: PreviewModel,
  intent: EditorOperationIntent,
  modelHash: ModelHashEvidence | null
): Promise<OperationOutcome> {
  if (tauriAvailable()) {
    return invoke<OperationOutcome>("validate_model_operation", {
      model,
      intent,
      claimedModelHash: modelHash ?? null
    });
  }
  return runWasmOperation(model, intent, modelHash, "validate_only");
}

export async function applyModelOperation(
  model: PreviewModel,
  intent: EditorOperationIntent,
  modelHash: ModelHashEvidence | null
): Promise<OperationOutcome> {
  if (tauriAvailable()) {
    return invoke<OperationOutcome>("apply_model_operation", {
      model,
      intent,
      claimedModelHash: modelHash ?? null
    });
  }
  return runWasmOperation(model, intent, modelHash, "apply");
}

// Honest engine-status reporting for the UI: which route answers operations
// in this environment, and whether it is ready. The wasm engine loads
// lazily; an absent artifact is an explicit named diagnostic with the build
// command (no fallback engine exists).
export type OperationEngineStatus =
  | { route: "tauri_backend_apply"; state: "ready"; detail: null }
  | { route: "local_wasm_engine"; state: "loading" | "ready"; detail: null }
  | { route: "local_wasm_engine"; state: "unavailable"; detail: string };

export function initialOperationEngineStatus(): OperationEngineStatus {
  if (tauriAvailable()) return { route: "tauri_backend_apply", state: "ready", detail: null };
  return { route: "local_wasm_engine", state: "loading", detail: null };
}

export async function warmupOperationEngine(): Promise<OperationEngineStatus> {
  if (tauriAvailable()) return { route: "tauri_backend_apply", state: "ready", detail: null };
  try {
    await loadWasmEngine();
    return { route: "local_wasm_engine", state: "ready", detail: null };
  } catch (error) {
    return { route: "local_wasm_engine", state: "unavailable", detail: String(error) };
  }
}

export { WASM_ENGINE_BUILD_COMMAND };
export type { WasmOperationEngine };
