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
  // H1 / F-5a hash seam: the engine's canonicalization and hashing, exported
  // so the frontend never re-implements either. Both throw on invalid JSON
  // input (WASM-ENGINE-INPUT-JSON-INVALID).
  canonicalJsonString: (valueJson: string) => string;
  canonicalSha256Hex: (valueJson: string) => string;
};

type GeneratedGlueModule = {
  default: (options?: { module_or_path: BufferSource | string | URL }) => Promise<unknown>;
  validate_operation_json: (modelJson: string, intentJson: string, claimedModelHashJson: string) => string;
  apply_operation_json: (modelJson: string, intentJson: string, claimedModelHashJson: string) => string;
  canonical_json_string: (valueJson: string) => string;
  canonical_sha256_hex: (valueJson: string) => string;
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

// Node / Vitest: locate the generated wasm bytes on disk. Vite's test
// transform rewrites `import.meta.url` to a root-relative file URL
// (`file:///src/...`), so module-relative resolution alone is unreliable;
// probe the deterministic candidates and fail loudly listing every probed
// path. This probes locations of ONE artifact — it is not an engine fallback.
async function readWasmBytesUnderNode(): Promise<Uint8Array<ArrayBuffer>> {
  const fsSpecifier = "node:fs/promises";
  const pathSpecifier = "node:path";
  const urlSpecifier = "node:url";
  const { readFile } = (await import(/* @vite-ignore */ fsSpecifier)) as typeof import("node:fs/promises");
  const path = (await import(/* @vite-ignore */ pathSpecifier)) as typeof import("node:path");
  const { fileURLToPath } = (await import(/* @vite-ignore */ urlSpecifier)) as typeof import("node:url");

  const candidates: string[] = [];
  try {
    // Real ESM: the module's own directory.
    candidates.push(path.join(path.dirname(fileURLToPath(import.meta.url)), "__generated__", GENERATED_WASM_FILENAME));
  } catch {
    // import.meta.url is not a parseable file URL under this transform.
  }
  // Vitest with cwd at the desktop workspace, and runs from the repo root.
  candidates.push(path.resolve(process.cwd(), "src", "services", "wasmEngine", "__generated__", GENERATED_WASM_FILENAME));
  candidates.push(
    path.resolve(process.cwd(), "apps", "desktop", "src", "services", "wasmEngine", "__generated__", GENERATED_WASM_FILENAME)
  );

  for (const candidate of candidates) {
    try {
      // Copy into a fresh Uint8Array: Node's Buffer is typed over
      // ArrayBufferLike, which BufferSource rejects.
      return new Uint8Array(await readFile(candidate));
    } catch {
      // Probe the next candidate; absence is reported below with all paths.
    }
  }
  throw absenceError("wasm artifact not found on disk", `probed: ${candidates.join(", ")}`);
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
      const bytes = await readWasmBytesUnderNode();
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
    applyOperationJson: glue.apply_operation_json,
    canonicalJsonString: glue.canonical_json_string,
    canonicalSha256Hex: glue.canonical_sha256_hex
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
