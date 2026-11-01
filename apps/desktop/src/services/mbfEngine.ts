// The webview's MBF writer and importer (DEL-17-04 Design `desktop-mbf-export-import.md` §2, §3.1–§3.3, slice
// 4a): the wasm32 build of `core/handoff/caepipe_mbf_wasm`, a JSON-text wrapper of the Rust MBF engine. The glue
// and wasm artifact are produced by `npm run build:wasm` into `public/mbf-engine/` and are not committed; an
// absent artifact is a named failure (MBF-ENGINE-ASSET-ABSENT) with the build command, never a fallback, because
// no other MBF engine exists in the webview. Loading follows `wasmEngine/loadSelfWeightEngine.ts`.
//
// The export review runs the writer here; the text is shown, then discarded. The bytes written are always the
// native writer's output for the hash-checked text (`src-tauri/src/mbf_export.rs`), which refuses a save whose
// digests differ from the reviewed ones (`MBF-EXPORT-REVIEW-STALE`).

export const MBF_ENGINE_ABSENT_DIAGNOSTIC = "MBF-ENGINE-ASSET-ABSENT";
export const MBF_ENGINE_BUILD_COMMAND = "npm run build:wasm --workspace apps/desktop";

/** The export dialog's writer settings (§3.1, §3.6). */
export type MbfExportSettings = {
  title: string;
  referenceTemperature: { value: number; unit: "degC" | "degF" };
  /** No row limit in the product (Q-4). */
  maxLayoutRows: null;
};

/** The most bytes of UTF-8 a title may hold (the engine refuses a longer one, `MBF-EXPORT-SETTINGS-INVALID`). */
export const MBF_MAX_TITLE_BYTES = 4096;

/** The settings' JSON text: what the review gives the wasm writer and the native save is given, so both read the same
 * text with the engine's one mapping (`export_settings_from_json`). */
export function mbfSettingsJson(settings: MbfExportSettings): string {
  return JSON.stringify({
    title: settings.title,
    referenceTemperature: { value: settings.referenceTemperature.value, unit: settings.referenceTemperature.unit },
    maxLayoutRows: null
  });
}

/** A unit the writer normalized by its exact definition, and its exact factor (`numerator/denominator`) where it has
 * one (an absolute temperature has an offset as well). */
export type MbfUnitConversion = { unit: string; definition: string; factor: string | null };

export type MbfDiagnostic = { code: string; severity: string; entity?: string; message: string };
export type MbfLossEntry = { entity: string; category: string; code: string; reason: string };

/** The writer's `to_dict()` (`MbfSiResult`). */
export type MbfWriterReport = {
  profile_id: string;
  status: "written" | "refused";
  text: string | null;
  diagnostics: MbfDiagnostic[];
  loss_report: MbfLossEntry[];
  assumptions: string[];
  material_map: Record<string, { swbpipe_material?: string; origin?: string | null; [key: string]: unknown }>;
  layout_row_count: number;
  [key: string]: unknown;
};

export type MbfFailure = { code: string; message: string };

/** The writer's review of one model text and settings. Digests are lowercase hex SHA-256. */
export type MbfRenderOutcome =
  | {
      status: "written" | "refused";
      profileId: string;
      mbfVersion: string;
      mbfText: string | null;
      mbfSha256: string | null;
      /** The identity file's text (`<stem>.swbpipe-ids.json`). */
      sidecarText: string | null;
      sidecarSha256: string | null;
      report: MbfWriterReport;
      unitConversions: MbfUnitConversion[];
    }
  | { status: "failed"; failure: MbfFailure };

export type MbfImportOutcome =
  | { status: string; modelJson: string | null; reportJson: string; idMapJson: string }
  | { status: "failed"; failure: MbfFailure };

export type WasmMbfEngine = {
  renderMbfJson: (modelJson: string, settingsJson: string) => string;
  importMbfJson: (mbfBytes: Uint8Array, sidecar: Uint8Array | undefined, optionsJson: string) => string;
};

type GeneratedGlueModule = {
  default: (options?: { module_or_path: BufferSource | string | URL }) => Promise<unknown>;
  render_mbf_json: WasmMbfEngine["renderMbfJson"];
  import_mbf_json: (mbfBytes: Uint8Array, sidecar: Uint8Array | null | undefined, optionsJson: string) => string;
};

// Held in constants so neither tsc nor Vite resolves the absent module at build time.
const GENERATED_GLUE_FILENAME = "open_pipe_stress_caepipe_mbf_wasm.js";
const GENERATED_WASM_FILENAME = "open_pipe_stress_caepipe_mbf_wasm_bg.wasm";
const BROWSER_GLUE_PATH = `/mbf-engine/${GENERATED_GLUE_FILENAME}`;

function browserGlueUrl(): string {
  return new URL(BROWSER_GLUE_PATH, globalThis.location.href).href;
}

let enginePromise: Promise<WasmMbfEngine> | null = null;

function absenceError(stage: string, detail: string): Error {
  return new Error(
    `${MBF_ENGINE_ABSENT_DIAGNOSTIC}: ${stage}: ${detail} — the MBF engine is required for the model batch file ` +
      `review and no fallback engine exists. Build it with \`${MBF_ENGINE_BUILD_COMMAND}\`.`
  );
}

// Node / Vitest: the generated assets on disk (one artifact set, probed at its deterministic locations).
async function locateAssetsUnderNode(): Promise<{ glueUrl: string; wasmBytes: Uint8Array<ArrayBuffer> }> {
  const fsSpecifier = "node:fs/promises";
  const pathSpecifier = "node:path";
  const urlSpecifier = "node:url";
  const { readFile } = (await import(/* @vite-ignore */ fsSpecifier)) as typeof import("node:fs/promises");
  const path = (await import(/* @vite-ignore */ pathSpecifier)) as typeof import("node:path");
  const { fileURLToPath, pathToFileURL } = (await import(/* @vite-ignore */ urlSpecifier)) as typeof import("node:url");
  const candidates: string[] = [];
  try {
    candidates.push(path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "..", "public", "mbf-engine"));
  } catch {
    // import.meta.url is not a parseable file URL under this transform.
  }
  candidates.push(path.resolve(process.cwd(), "public", "mbf-engine"));
  candidates.push(path.resolve(process.cwd(), "apps", "desktop", "public", "mbf-engine"));
  for (const candidate of candidates) {
    try {
      const wasmBytes = new Uint8Array(await readFile(path.join(candidate, GENERATED_WASM_FILENAME)));
      return { glueUrl: pathToFileURL(path.join(candidate, GENERATED_GLUE_FILENAME)).href, wasmBytes };
    } catch {
      // Probe the next candidate; absence is reported below with all paths.
    }
  }
  throw absenceError("wasm artifact not found on disk", `probed: ${candidates.join(", ")}`);
}

async function instantiate(): Promise<WasmMbfEngine> {
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
      await glue.default();
    } catch (error) {
      throw absenceError("wasm instantiation failed", String(error));
    }
  }
  for (const name of ["render_mbf_json", "import_mbf_json"] as const) {
    if (typeof glue[name] !== "function") throw absenceError("required export missing", name);
  }
  return {
    renderMbfJson: glue.render_mbf_json,
    importMbfJson: (mbfBytes, sidecar, optionsJson) => glue.import_mbf_json(mbfBytes, sidecar ?? null, optionsJson)
  };
}

/** Lazy singleton; a failure is not cached, so a later call after `npm run build:wasm` succeeds. */
export function loadMbfEngine(): Promise<WasmMbfEngine> {
  if (!enginePromise) {
    enginePromise = instantiate().catch((error: unknown) => {
      enginePromise = null;
      throw error;
    });
  }
  return enginePromise;
}

type RawRender = {
  status: string;
  profile_id?: string;
  mbf_version?: string;
  mbf_text?: string | null;
  mbf_sha256?: string | null;
  sidecar_text?: string | null;
  sidecar_sha256?: string | null;
  report_json?: string | null;
  unit_conversions?: MbfUnitConversion[];
  failure?: MbfFailure | null;
};

/** Runs the writer over the session's canonical model text with the dialog's settings text (`mbfSettingsJson`). */
export async function renderMbfExport(
  canonicalText: string,
  settingsJson: string,
  engine?: WasmMbfEngine
): Promise<MbfRenderOutcome> {
  const loaded = engine ?? (await loadMbfEngine());
  const raw = JSON.parse(loaded.renderMbfJson(canonicalText, settingsJson)) as RawRender;
  if (raw.status !== "written" && raw.status !== "refused") {
    return { status: "failed", failure: raw.failure ?? { code: "MBF-ENGINE-FAILED", message: "The MBF engine returned no review." } };
  }
  return {
    status: raw.status,
    profileId: raw.profile_id ?? "",
    mbfVersion: raw.mbf_version ?? "",
    mbfText: raw.mbf_text ?? null,
    mbfSha256: raw.mbf_sha256 ?? null,
    sidecarText: raw.sidecar_text ?? null,
    sidecarSha256: raw.sidecar_sha256 ?? null,
    report: JSON.parse(raw.report_json ?? "null") as MbfWriterReport,
    unitConversions: raw.unit_conversions ?? []
  };
}

/** Runs the importer (the later import slice): the MBF bytes, the identity file's bytes when read, its label. */
export async function importMbf(
  mbfBytes: Uint8Array,
  sidecar: Uint8Array | undefined,
  options: { sourceLabel: string; materialProvenance?: Record<string, unknown> | null },
  engine?: WasmMbfEngine
): Promise<MbfImportOutcome> {
  const loaded = engine ?? (await loadMbfEngine());
  const raw = JSON.parse(loaded.importMbfJson(mbfBytes, sidecar, JSON.stringify(options))) as {
    status: string;
    model_json?: string | null;
    report_json?: string;
    id_map_json?: string;
    failure?: MbfFailure | null;
  };
  if (raw.status === "failed" || raw.report_json === undefined || raw.id_map_json === undefined) {
    return { status: "failed", failure: raw.failure ?? { code: "MBF-ENGINE-FAILED", message: "The MBF engine returned no import." } };
  }
  return { status: raw.status, modelJson: raw.model_json ?? null, reportJson: raw.report_json, idMapJson: raw.id_map_json };
}
