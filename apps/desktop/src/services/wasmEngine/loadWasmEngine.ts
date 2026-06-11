// Loads the wasm32 build of `core/model_operations/operation_applier` — the
// sole browser-mode operation engine per DEC-020 / ADR-0001. The generated
// glue and wasm artifact are produced by `npm run build:wasm` and are not
// committed; an absent artifact is an explicit named failure
// (WASM-ENGINE-ASSET-ABSENT) with the build command in the message — never a
// silent fallback to another engine, because no other engine exists.

export const WASM_ENGINE_ABSENT_DIAGNOSTIC = "WASM-ENGINE-ASSET-ABSENT";
export const WASM_ENGINE_BUILD_COMMAND = "npm run build:wasm --workspace apps/desktop";

export type WasmOperationEngine = {
  validateOperationJson: (modelJson: string, intentJson: string, claimedModelHashJson: string) => string;
  applyOperationJson: (modelJson: string, intentJson: string, claimedModelHashJson: string) => string;
};

type GeneratedGlueModule = {
  default: (options?: { module_or_path: BufferSource | string | URL }) => Promise<unknown>;
  validate_operation_json: (modelJson: string, intentJson: string, claimedModelHashJson: string) => string;
  apply_operation_json: (modelJson: string, intentJson: string, claimedModelHashJson: string) => string;
};

// Specifiers are intentionally held in constants (not inline literals) so
// neither tsc nor Vite resolves them at build time: the generated module is
// absent until `npm run build:wasm` runs, and its absence must be a runtime
// diagnostic, not a compile failure.
const GENERATED_GLUE_SPECIFIER = "./__generated__/open_pipe_stress_operation_applier.js";
const GENERATED_WASM_FILENAME = "open_pipe_stress_operation_applier_bg.wasm";

let enginePromise: Promise<WasmOperationEngine> | null = null;

function absenceError(stage: string, detail: string): Error {
  return new Error(
    `${WASM_ENGINE_ABSENT_DIAGNOSTIC}: ${stage}: ${detail} — the wasm operation engine is required in ` +
      `browser mode (DEC-020 / ADR-0001) and no fallback engine exists. Build it with \`${WASM_ENGINE_BUILD_COMMAND}\`.`
  );
}

async function instantiate(): Promise<WasmOperationEngine> {
  let glue: GeneratedGlueModule;
  try {
    glue = (await import(/* @vite-ignore */ GENERATED_GLUE_SPECIFIER)) as GeneratedGlueModule;
  } catch (error) {
    throw absenceError("generated glue module import failed", String(error));
  }

  const runningUnderNode = typeof process !== "undefined" && Boolean(process.versions?.node);
  try {
    if (runningUnderNode) {
      // Node / Vitest (including jsdom environments): load the wasm bytes
      // from disk next to the generated glue; no fetch is available for
      // file URLs there.
      const fsSpecifier = "node:fs/promises";
      const { readFile } = (await import(/* @vite-ignore */ fsSpecifier)) as typeof import("node:fs/promises");
      const wasmUrl = new URL(`./__generated__/${GENERATED_WASM_FILENAME}`, import.meta.url);
      const bytes = await readFile(wasmUrl);
      await glue.default({ module_or_path: bytes });
    } else {
      // Real browser: the wasm-bindgen web-target default path fetches the
      // sibling _bg.wasm relative to the glue module URL.
      await glue.default();
    }
  } catch (error) {
    throw absenceError("wasm instantiation failed", String(error));
  }

  return {
    validateOperationJson: glue.validate_operation_json,
    applyOperationJson: glue.apply_operation_json
  };
}

/// Lazy singleton: first caller triggers instantiation; failures are not
/// cached so a later call after `npm run build:wasm` succeeds.
export function loadWasmEngine(): Promise<WasmOperationEngine> {
  if (!enginePromise) {
    enginePromise = instantiate().catch((error: unknown) => {
      enginePromise = null;
      throw error;
    });
  }
  return enginePromise;
}
