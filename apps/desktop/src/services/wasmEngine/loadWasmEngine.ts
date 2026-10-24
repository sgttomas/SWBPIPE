// Loads the wasm32 build of `core/model_operations/operation_applier` — the
// sole browser-mode operation engine per DEC-020 / ADR-0001. The generated
// glue and wasm artifact are produced by `npm run build:wasm` into
// `public/wasm-engine/` (TP-APP-R2-WASMPKG-001: Vite serves `public/` at the
// site root in dev and copies it verbatim into `dist`, so the one root
// `/wasm-engine/` URL resolves identically under the dev server, `vite
// preview` of the production dist, and the packaged tauri:// asset protocol)
// and are not committed; an absent artifact is an explicit named failure
// (WASM-ENGINE-ASSET-ABSENT) with the build command in the message — never a
// silent fallback to another engine, because no other engine exists.

export const WASM_ENGINE_ABSENT_DIAGNOSTIC = "WASM-ENGINE-ASSET-ABSENT";
export const WASM_ENGINE_BUILD_COMMAND = "npm run build:wasm --workspace apps/desktop";

export type WasmOperationEngine = {
  validateOperationJson: (modelJson: string, intentJson: string, claimedModelHashJson: string) => string;
  applyOperationJson: (modelJson: string, intentJson: string, claimedModelHashJson: string) => string;
  validateOperationBatchJson: (modelJson: string, batchJson: string, claimedModelHashJson: string) => string;
  applyOperationBatchJson: (modelJson: string, batchJson: string, claimedModelHashJson: string) => string;
  convertDisplayQuantitiesJson: (inputJson: string) => string;
  // W-13 §5.3: the one model-document validator (model-data slots). JSON text
  // in; `{document_kind, blocking, diagnostics}` out, or the input-error
  // envelope when the text is not JSON.
  validateModelDocumentJson: (modelJson: string) => string;
  // H1 / F-5a hash seam: the engine's canonicalization and hashing, exported
  // so the frontend never re-implements either. Both throw on invalid JSON
  // input (WASM-ENGINE-INPUT-JSON-INVALID).
  canonicalJsonString: (valueJson: string) => string;
  canonicalSha256Hex: (valueJson: string) => string;
  canonicalJsonCheckedV1: (valueJson: string) => string;
  canonicalSha256HexCheckedV1: (valueJson: string) => string;
  // R1 (b7-plan Unit R): the Rust retained-precision reader, the desktop's
  // only one. JSON text in; the reader's outcome JSON out
  // (`retained_precision::outcome_json`). Throws only on input that is not
  // JSON (WASM-ENGINE-INPUT-JSON-INVALID).
  validateRetainedPrecisionJson: (sourceJson: string, invocationJson?: string) => string;
  validateRetainedPrecisionTransportJson: (sourceJson: string) => string;
  retainedPrecisionSemanticsJson: (sourceJson: string, validationJson: string, requestedJson: string) => string;
  // R2 (b7-plan Unit R): the Rust base-route readers (`reader_verdict`). JSON
  // text in, the verdict's JSON out; an absent model, invocation or source reads
  // as absent. Throw only on input that is not JSON.
  readerDispatchJson: (sourceJson: string, modelJson?: string, invocationJson?: string) => string;
  readerValidateJson: (reader: string, sourceJson: string, modelJson?: string, invocationJson?: string) => string;
  readerTransportJson: (reader: string, sourceJson: string) => string;
  readerRowSemanticsJson: (rowJson: string, sourceJson?: string) => string;
  // DEL-07-05 numerical health indicator (Design `numerical-health-indicator.md`
  // §2.6): the Rust parse of a result's ordinary-solve diagnostics, with the base
  // reader's findings for it (a JSON array of codes, or the dispatch outcome).
  // Presentation only; never read by admission or standing. Throws only on input
  // that is not JSON; the desktop then shows no indicator (fail closed).
  numericalHealthJson: (sourceJson: string, readerFindingsJson?: string) => string;
  // W-17 §2.3a: the shear modulus a library insert derives from E and ν, bit
  // for bit the exact route's (`derived_shear_modulus_pa`): `{value, unit:"Pa"}`
  // or `{error}`.
  derivedShearModulusJson: (elasticModulus: number, unit: string, poissonRatio: number) => string;
};

type GeneratedGlueModule = {
  default: (options?: { module_or_path: BufferSource | string | URL }) => Promise<unknown>;
  validate_operation_json: (modelJson: string, intentJson: string, claimedModelHashJson: string) => string;
  apply_operation_json: (modelJson: string, intentJson: string, claimedModelHashJson: string) => string;
  validate_operation_batch_json: WasmOperationEngine["validateOperationBatchJson"];
  apply_operation_batch_json: WasmOperationEngine["applyOperationBatchJson"];
  convert_display_quantities_json: WasmOperationEngine["convertDisplayQuantitiesJson"];
  validate_model_document_json: WasmOperationEngine["validateModelDocumentJson"];
  derived_shear_modulus_json: WasmOperationEngine["derivedShearModulusJson"];
  canonical_json_string: (valueJson: string) => string;
  canonical_sha256_hex: (valueJson: string) => string;
  canonical_json_checked_v1: (valueJson: string) => string;
  canonical_sha256_hex_checked_v1: (valueJson: string) => string;
  validate_retained_precision_json: (sourceJson: string, invocationJson?: string | null) => string;
  validate_retained_precision_transport_json: (sourceJson: string) => string;
  retained_precision_semantics_json: WasmOperationEngine["retainedPrecisionSemanticsJson"];
  reader_dispatch_json: (sourceJson: string, modelJson?: string | null, invocationJson?: string | null) => string;
  reader_validate_json: (reader: string, sourceJson: string, modelJson?: string | null, invocationJson?: string | null) => string;
  reader_transport_json: (reader: string, sourceJson: string) => string;
  reader_row_semantics_json: (rowJson: string, sourceJson?: string | null) => string;
  numerical_health_json: (sourceJson: string, readerFindingsJson?: string | null) => string;
};

// The browser specifier is intentionally held in a constant (not an inline
// literal) so neither tsc nor Vite resolves it at build time: the generated
// module is absent until `npm run build:wasm` runs, and its absence must be
// a runtime diagnostic, not a compile failure. `public/` contents are served
// at the site root in every browser lane.
const GENERATED_GLUE_FILENAME = "open_pipe_stress_operation_applier.js";
const GENERATED_WASM_FILENAME = "open_pipe_stress_operation_applier_bg.wasm";
const BROWSER_GLUE_PATH = `/wasm-engine/${GENERATED_GLUE_FILENAME}`;

// The browser import must be a FULLY-QUALIFIED URL (origin included), not the
// root-absolute path: Vite's dev transform wraps dynamic imports in its
// `injectQuery(url, "import")` client helper, and the dev server refuses to
// serve publicDir files with the `?import` query (HTTP 500). The helper skips
// URLs that carry a protocol, so resolving against `location.href` makes the
// browser import the raw asset identically under the dev server, `vite
// preview`, and the packaged `tauri://localhost/` asset protocol.
function browserGlueUrl(): string {
  return new URL(BROWSER_GLUE_PATH, globalThis.location.href).href;
}

let enginePromise: Promise<WasmOperationEngine> | null = null;
let loadedEngine: WasmOperationEngine | null = null;

function absenceError(stage: string, detail: string): Error {
  return new Error(
    `${WASM_ENGINE_ABSENT_DIAGNOSTIC}: ${stage}: ${detail} — the wasm operation engine is required in ` +
      `browser mode (DEC-020 / ADR-0001) and no fallback engine exists. Build it with \`${WASM_ENGINE_BUILD_COMMAND}\`.`
  );
}

// Node / Vitest: locate the generated assets on disk. Vite's test transform
// rewrites `import.meta.url` to a root-relative file URL (`file:///src/...`),
// so module-relative resolution alone is unreliable; probe the deterministic
// candidates and fail loudly listing every probed path. This probes locations
// of ONE artifact set — it is not an engine fallback.
async function locateAssetsUnderNode(): Promise<{ glueUrl: string; wasmBytes: Uint8Array<ArrayBuffer> }> {
  const fsSpecifier = "node:fs/promises";
  const pathSpecifier = "node:path";
  const urlSpecifier = "node:url";
  const { readFile } = (await import(/* @vite-ignore */ fsSpecifier)) as typeof import("node:fs/promises");
  const path = (await import(/* @vite-ignore */ pathSpecifier)) as typeof import("node:path");
  const { fileURLToPath, pathToFileURL } = (await import(/* @vite-ignore */ urlSpecifier)) as typeof import("node:url");

  const candidates: string[] = [];
  try {
    // Real ESM: `public/wasm-engine/` relative to this module's directory
    // (`src/services/wasmEngine/`).
    candidates.push(
      path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "..", "..", "public", "wasm-engine")
    );
  } catch {
    // import.meta.url is not a parseable file URL under this transform.
  }
  // Vitest with cwd at the desktop workspace, and runs from the repo root.
  candidates.push(path.resolve(process.cwd(), "public", "wasm-engine"));
  candidates.push(path.resolve(process.cwd(), "apps", "desktop", "public", "wasm-engine"));

  for (const candidate of candidates) {
    try {
      // Copy into a fresh Uint8Array: Node's Buffer is typed over
      // ArrayBufferLike, which BufferSource rejects.
      const wasmBytes = new Uint8Array(await readFile(path.join(candidate, GENERATED_WASM_FILENAME)));
      return { glueUrl: pathToFileURL(path.join(candidate, GENERATED_GLUE_FILENAME)).href, wasmBytes };
    } catch {
      // Probe the next candidate; absence is reported below with all paths.
    }
  }
  throw absenceError("wasm artifact not found on disk", `probed: ${candidates.join(", ")}`);
}

async function instantiate(): Promise<WasmOperationEngine> {
  const runningUnderNode = typeof process !== "undefined" && Boolean(process.versions?.node);
  let glue: GeneratedGlueModule;

  if (runningUnderNode) {
    const { glueUrl, wasmBytes } = await locateAssetsUnderNode();
    try {
      glue = (await import(/* @vite-ignore */ glueUrl)) as GeneratedGlueModule;
    } catch (error) {
      throw absenceError("generated glue module import failed", String(error));
    }
    try {
      await glue.default({ module_or_path: wasmBytes });
    } catch (error) {
      throw absenceError("wasm instantiation failed", String(error));
    }
  } else {
    try {
      glue = (await import(/* @vite-ignore */ browserGlueUrl())) as GeneratedGlueModule;
    } catch (error) {
      throw absenceError("generated glue module import failed", String(error));
    }
    try {
      // Real browser: the wasm-bindgen web-target default path fetches the
      // sibling _bg.wasm relative to the glue module URL (`import.meta.url`),
      // which is `/wasm-engine/` in every browser lane.
      await glue.default();
    } catch (error) {
      throw absenceError("wasm instantiation failed", String(error));
    }
  }

  for (const name of ["validate_operation_json", "apply_operation_json", "validate_operation_batch_json", "apply_operation_batch_json", "convert_display_quantities_json", "validate_model_document_json", "canonical_json_string", "canonical_sha256_hex", "canonical_json_checked_v1", "canonical_sha256_hex_checked_v1", "validate_retained_precision_json", "validate_retained_precision_transport_json", "retained_precision_semantics_json", "reader_dispatch_json", "reader_validate_json", "reader_transport_json", "reader_row_semantics_json", "numerical_health_json"] as const) {
    if (typeof glue[name] !== "function") throw absenceError("required export missing", name);
  }
  return {
    validateOperationJson: glue.validate_operation_json,
    applyOperationJson: glue.apply_operation_json,
    validateOperationBatchJson: glue.validate_operation_batch_json,
    applyOperationBatchJson: glue.apply_operation_batch_json,
    convertDisplayQuantitiesJson: glue.convert_display_quantities_json,
    validateModelDocumentJson: glue.validate_model_document_json,
    derivedShearModulusJson: glue.derived_shear_modulus_json,
    canonicalJsonString: glue.canonical_json_string,
    canonicalSha256Hex: glue.canonical_sha256_hex,
    canonicalJsonCheckedV1: glue.canonical_json_checked_v1,
    canonicalSha256HexCheckedV1: glue.canonical_sha256_hex_checked_v1,
    validateRetainedPrecisionJson: (sourceJson, invocationJson) => glue.validate_retained_precision_json(sourceJson, invocationJson ?? null),
    validateRetainedPrecisionTransportJson: glue.validate_retained_precision_transport_json,
    retainedPrecisionSemanticsJson: glue.retained_precision_semantics_json,
    readerDispatchJson: (sourceJson, modelJson, invocationJson) => glue.reader_dispatch_json(sourceJson, modelJson ?? null, invocationJson ?? null),
    readerValidateJson: (reader, sourceJson, modelJson, invocationJson) => glue.reader_validate_json(reader, sourceJson, modelJson ?? null, invocationJson ?? null),
    readerTransportJson: glue.reader_transport_json,
    readerRowSemanticsJson: (rowJson, sourceJson) => glue.reader_row_semantics_json(rowJson, sourceJson ?? null),
    numericalHealthJson: (sourceJson, readerFindingsJson) => glue.numerical_health_json(sourceJson, readerFindingsJson ?? null)
  };
}

/// Lazy singleton: first caller triggers instantiation; failures are not
/// cached so a later call after `npm run build:wasm` succeeds.
export function loadWasmEngine(): Promise<WasmOperationEngine> {
  if (!enginePromise) {
    enginePromise = instantiate().then((engine) => {
      loadedEngine = engine;
      return engine;
    }, (error: unknown) => {
      enginePromise = null;
      throw error;
    });
  }
  return enginePromise;
}

/// The engine once `loadWasmEngine` has resolved, else null. R2 (b7-plan Unit
/// R): the base-route readers' verdicts are synchronous presentation inputs,
/// so the desktop loads the engine before it renders (`main.tsx`) and a
/// verdict asked for without it is refused, never computed another way.
export function loadedWasmEngine(): WasmOperationEngine | null {
  return loadedEngine;
}
