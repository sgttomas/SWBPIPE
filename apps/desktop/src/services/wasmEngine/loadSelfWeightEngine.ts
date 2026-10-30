export const WASM_ENGINE_ABSENT_DIAGNOSTIC = "SELF-WEIGHT-ENGINE-ASSET-ABSENT";
export const WASM_ENGINE_BUILD_COMMAND = "npm run build:wasm --workspace apps/desktop";

export type WasmSelfWeightEngine = {
  generateSelfWeightPlanJson: (modelJson: string, requestJson: string) => string;
};
type GeneratedGlueModule = {
  default: (options?: { module_or_path: BufferSource | string | URL }) => Promise<unknown>;
  generate_self_weight_plan_json: WasmSelfWeightEngine["generateSelfWeightPlanJson"];
};

// The browser specifier is intentionally held in a constant (not an inline
// literal) so neither tsc nor Vite resolves it at build time: the generated
// module is absent until `npm run build:wasm` runs, and its absence must be
// a runtime diagnostic, not a compile failure. `public/` contents are served
// at the site root in every browser lane.
const GENERATED_GLUE_FILENAME = "open_pipe_stress_self_weight_wasm.js";
const GENERATED_WASM_FILENAME = "open_pipe_stress_self_weight_wasm_bg.wasm";
const BROWSER_GLUE_PATH = `/self-weight-engine/${GENERATED_GLUE_FILENAME}`;

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

let enginePromise: Promise<WasmSelfWeightEngine> | null = null;

function absenceError(stage: string, detail: string): Error {
  return new Error(
    `${WASM_ENGINE_ABSENT_DIAGNOSTIC}: ${stage}: ${detail} — the self-weight planning engine is required in ` +
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
    // Real ESM: `public/self-weight-engine/` relative to this module's directory
    // (`src/services/wasmEngine/`).
    candidates.push(
      path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "..", "..", "public", "self-weight-engine")
    );
  } catch {
    // import.meta.url is not a parseable file URL under this transform.
  }
  // Vitest with cwd at the desktop workspace, and runs from the repo root.
  candidates.push(path.resolve(process.cwd(), "public", "self-weight-engine"));
  candidates.push(path.resolve(process.cwd(), "apps", "desktop", "public", "self-weight-engine"));

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

async function instantiate(): Promise<WasmSelfWeightEngine> {
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
      // which is `/self-weight-engine/` in every browser lane.
      await glue.default();
    } catch (error) {
      throw absenceError("wasm instantiation failed", String(error));
    }
  }

  if (typeof glue.generate_self_weight_plan_json !== "function") throw absenceError("required export missing", "generate_self_weight_plan_json");
  return { generateSelfWeightPlanJson: glue.generate_self_weight_plan_json };
}

/// Lazy singleton: first caller triggers instantiation; failures are not
/// cached so a later call after `npm run build:wasm` succeeds.
export function loadSelfWeightEngine(): Promise<WasmSelfWeightEngine> {
  if (!enginePromise) {
    enginePromise = instantiate().catch((error: unknown) => {
      enginePromise = null;
      throw error;
    });
  }
  return enginePromise;
}
