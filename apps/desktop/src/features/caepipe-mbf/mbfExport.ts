// File > Export MBF… (DEL-17-04 Design `desktop-mbf-export-import.md` §3.1–§3.3, §3.5, §3.6, §3.9.5; slices 4a and
// 4b.1): the export review's presentation and the native commands' argument and outcome types. The dialog
// (`MbfExportDialog.tsx`) never runs the wasm writer: it asks the injected `reviewMbfExport` (the native
// `review_mbf_export`, which runs the capped worker's task 3 in `review` mode) for the bounded review of the session's
// canonical model text, then calls the injected `saveMbfExport` (the native `save_mbf_export`), which repeats every
// check in its own `save`-mode run, refuses a truncated review or a result above the output ceiling, writes the native
// writer's bytes and refuses a save whose digests differ from the reviewed ones. The engine classifies the writer's
// result (`export_review` in `core/handoff/caepipe_mbf_engine`); this module adds the product descriptions. Slice 4b.2
// connects the commands, the menu item and the Exports panel.
//
// Product text names only the format ("Model batch file (.mbf)") and states nothing about any other program
// (§3.3, §3.5): the review shows a neutral profile label, each entry by its code, its entity and a product-side
// description of what SWBPIPE did or did not write (`MBF_LOSS_DESCRIPTIONS`), never the engine's message text (the
// review does not carry it), and the unit conversions by their exact factors; the writer's assumptions are not
// displayed. Nothing here transmits anything (F-PIP-1).
import type { ModelDocumentSaveArgs, ModelFileDiagnostic } from "../model-file/modelDocumentSave";

/** The export dialog's writer settings (§3.1, §3.6). */
export type MbfExportSettings = {
  title: string;
  referenceTemperature: { value: number; unit: "degC" | "degF" };
  /** No row limit in the product (Q-4). */
  maxLayoutRows: null;
};

/** The most bytes of UTF-8 a title may hold (the engine refuses a longer one, `MBF-EXPORT-SETTINGS-INVALID`). */
export const MBF_MAX_TITLE_BYTES = 4096;

/** The settings' JSON text: what the review is given and the save is given, so both read the same text with the
 * engine's one mapping (`export_settings_from_json`). */
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

/** The governed local-private, lossless route of the save (A-4). */
export const MBF_EXPORT_ROUTE_ID = "DOTH-MBF-012";
export const MBF_EXPORT_INTENT_LABEL =
  "Save my model, including private project data, as a model batch file and its identity file in a folder I choose";
export const MBF_EXPORT_ROUTE = { routeId: MBF_EXPORT_ROUTE_ID, intentLabel: MBF_EXPORT_INTENT_LABEL } as const;
/** The native refusal of a save whose model or settings changed after the review. */
export const MBF_EXPORT_REVIEW_STALE = "MBF-EXPORT-REVIEW-STALE";
/** Any truncated review blocks the save (§3.3), whatever the entries not shown. */
export const MBF_REVIEW_TRUNCATED = "MBF-REVIEW-TRUNCATED";
/** The save result (MBF, identity file, review and framing) would exceed the 64 MiB output ceiling (§3.7). */
export const MBF_EXPORT_OUTPUT_TOO_LARGE = "MBF-EXPORT-OUTPUT-TOO-LARGE";
/** The browser-only route has no native review and no save (§3.9.5). */
export const MBF_REVIEW_NATIVE_ONLY = "MBF-REVIEW-NATIVE-ONLY";
/** The native save's refusal of a model the writer refuses; its reply carries the bounded review. */
export const MBF_EXPORT_WRITER_REFUSED = "MBF-EXPORT-WRITER-REFUSED";
/** What each global blocking condition says (§3.3, §3.9.5). */
export const MBF_REVIEW_TRUNCATED_SENTENCE = "The loss report is too large to show in full, so the export is not offered.";
export const MBF_OUTPUT_TOO_LARGE_SENTENCE =
  "The model batch file, its identity file and the review together are larger than the 64 MiB an export may write, so the export is not offered.";
export const MBF_REVIEW_NATIVE_ONLY_SENTENCE = "The model batch file review runs in the desktop app.";
/** What native code appends to the chosen file's stem to name the identity file (A-5). */
export const MBF_IDENTITY_FILE_SUFFIX = ".swbpipe-ids.json";
/** The open items the writer's profile carries (§3.3). */
export const MBF_PROFILE_OPEN_ITEMS = ["TBD-17-01-001", "TBD-17-01-002", "TBD-17-01-003"] as const;
/** The profile as the review names it (§3.3): not the writer's internal profile id. */
export const MBF_PROFILE_LABEL = "SI model batch file, pilot subset";
/** In place of the writer's assumptions, which the review does not display (§3.3). */
export const MBF_ASSUMPTIONS_SENTENCE = "The writer's assumptions are not displayed here.";

/** The sentence that ends the description of every blocking code (§3.3). */
export const MBF_BLOCKING_SENTENCE = "so SWBPIPE does not write the file.";

/** The codes the writer raises as errors (`refuse`, or a diagnostic of severity `error`): each one alone keeps the file
 * from being written. The others are omissions, warnings and notes, after which the file can still be written. */
export const MBF_BLOCKING_CODES: ReadonlySet<string> = new Set([
  "MBF-SI-BEND-FLEXIBILITY-MISSING", "MBF-SI-BEND-GEOMETRY-INCONSISTENT", "MBF-SI-BEND-SIF-INCOMPLETE", "MBF-SI-BEND-UNSUPPORTED", "MBF-SI-COMBINATION-BASIS-UNSUPPORTED", "MBF-SI-COMBINATION-NESTED-UNSUPPORTED", "MBF-SI-COMPONENT-UNSUPPORTED", "MBF-SI-CONDITION-USE-UNSUPPORTED", "MBF-SI-CORROSION-ALLOWANCE-MISSING", "MBF-SI-DIFFERENCE-BASIS-UNSUPPORTED", "MBF-SI-GEOMETRY-UNSUPPORTED", "MBF-SI-IMPOSED-DISPLACEMENT-UNSUPPORTED", "MBF-SI-INPUT-INVALID", "MBF-SI-LOAD-CASE-FEATURE-UNSUPPORTED", "MBF-SI-LOAD-CASE-MAPPING-UNSUPPORTED", "MBF-SI-LOAD-UNSUPPORTED", "MBF-SI-MATERIAL-EXPANSION-MISSING", "MBF-SI-MATERIAL-POISSON-MISSING", "MBF-SI-NODE-NUMBER-LIMIT", "MBF-SI-PIPING-CODE-UNSUPPORTED", "MBF-SI-PREFLIGHT-REFUSED", "MBF-SI-PRESSURE-CASE-UNSUPPORTED", "MBF-SI-PRESSURE-REGION-FORMAT", "MBF-SI-PRESSURE-UNSUPPORTED", "MBF-SI-PROJECT-UNIT-UNSUPPORTED", "MBF-SI-REFERENCE-CONFIGURATION-UNSUPPORTED", "MBF-SI-REFERENCE-UNRESOLVED", "MBF-SI-SCHEMA-VERSION", "MBF-SI-SECTION-BINDING-UNSUPPORTED", "MBF-SI-SECTION-REFERENCE-STALE", "MBF-SI-SECTION-UNSUPPORTED", "MBF-SI-SHEAR-MODEL-UNSUPPORTED", "MBF-SI-SNUBBER-UNSUPPORTED", "MBF-SI-SPECTRUM-UNSUPPORTED", "MBF-SI-SUBTRAHEND-SUPPORT-STATE-UNSUPPORTED", "MBF-SI-SUPPORT-DIRECTION-UNSUPPORTED", "MBF-SI-SUPPORT-UNSUPPORTED", "MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED", "MBF-SI-TEST-CONDITION-UNSUPPORTED", "MBF-SI-TOPOLOGY-UNSUPPORTED", "MBF-SI-UNIT-UNSUPPORTED", "MBF-SI-VALUE-OUT-OF-RANGE", "MODEL_SCHEMA_VERSION_RETIRED"
]);

/** What SWBPIPE did or did not write, for each code the writer can emit (§3.3). The engine's own messages are not
 * shown: they are the engine's and its Python twin's, kept byte for byte, and some describe another program. Each
 * description states only what this export writes; a blocking code's says that SWBPIPE does not write the file. */
export const MBF_LOSS_DESCRIPTIONS: Readonly<Record<string, string>> = {
  "MBF-SI-BEND-FLEXIBILITY-MISSING": "This bend has no flexibility factor, so SWBPIPE does not write the file.",
  "MBF-SI-BEND-GEOMETRY-INCONSISTENT": "This bend's radius, angle and adjoining pipes do not agree, so SWBPIPE does not write the file.",
  "MBF-SI-BEND-SIF-INCOMPLETE": "This bend gives only some of its stress intensification factors, so SWBPIPE does not write the file.",
  "MBF-SI-BEND-UNSUPPORTED": "This bend's arrangement is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-COMBINATION-BASIS-UNSUPPORTED": "This combination's basis is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-COMBINATION-NESTED-UNSUPPORTED": "This combination uses another combination, which is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-COMBINATION-NOT-EXPORTED": "SWBPIPE does not write load combinations to the file.",
  "MBF-SI-COMPONENT-UNSUPPORTED": "This component's kind is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-CONDITION-USE-UNSUPPORTED": "This load case's condition use is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-CORROSION-ALLOWANCE-MISSING": "This pipe has no corrosion allowance, which the written pressure data needs, so SWBPIPE does not write the file.",
  "MBF-SI-DIFFERENCE-BASIS-UNSUPPORTED": "This combination's difference basis is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-FIELD-NOT-EXPORTED": "SWBPIPE does not write these fields to the file.",
  "MBF-SI-GEOMETRY-UNSUPPORTED": "This geometry is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-IMPOSED-DISPLACEMENT-UNSUPPORTED": "This imposed displacement is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-INPUT-INVALID": "This entry is not a valid model value, so SWBPIPE does not write the file.",
  "MBF-SI-LAYOUT-ROW-LIMIT": "The layout has more rows than the row limit allows.",
  "MBF-SI-LOAD-CASE-EMPTY": "This load case has no loads, so it is not written.",
  "MBF-SI-LOAD-CASE-FEATURE-UNSUPPORTED": "This load case feature is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-LOAD-CASE-MAPPING-UNSUPPORTED": "SWBPIPE cannot place this load case in the file's load set, so SWBPIPE does not write the file.",
  "MBF-SI-LOAD-UNSUPPORTED": "This load is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-MATERIAL-EXPANSION-MISSING": "This material has no thermal expansion coefficient, which the file needs, so SWBPIPE does not write the file.",
  "MBF-SI-MATERIAL-POISSON-MISSING": "This material has neither a Poisson ratio nor a shear modulus, which the file needs, so SWBPIPE does not write the file.",
  "MBF-SI-MEMBER-NOT-EXPORTED": "SWBPIPE does not write this part of the model to the file.",
  "MBF-SI-MODEL-SETTINGS-NOT-EXPORTED": "SWBPIPE does not write the model's input validation settings to the file.",
  "MBF-SI-NODE-NOT-EXPORTED": "This node is not connected to any pipe segment, so it is not written.",
  "MBF-SI-NODE-NUMBER-LIMIT": "The model has more nodes than the file's node numbering allows, so SWBPIPE does not write the file.",
  "MBF-SI-ORIENTATION-NOT-EXPORTED": "SWBPIPE does not write the pipe segments' local axis references to the file.",
  "MBF-SI-PIPING-CODE-B31J-LOCKED": "This piping code choice is outside the subset SWBPIPE writes.",
  "MBF-SI-PIPING-CODE-NOT-WRITTEN": "No piping code was chosen, so the file names none.",
  "MBF-SI-PIPING-CODE-UNSUPPORTED": "This piping code is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-PREFLIGHT-REFUSED": "SWBPIPE read the written file back and it did not give the model back as written, so SWBPIPE does not write the file.",
  "MBF-SI-PRESSURE-CASE-UNSUPPORTED": "This pressure load case is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-PRESSURE-REGION-FORMAT": "This pressure region cannot be written in the file's format, so SWBPIPE does not write the file.",
  "MBF-SI-PRESSURE-REGION-ZERO": "This pressure region has zero pressure, so it is not written.",
  "MBF-SI-PRESSURE-UNSUPPORTED": "This pressure data is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-PROJECT-UNIT-UNSUPPORTED": "This project unit is not one SWBPIPE converts for the file, so SWBPIPE does not write the file.",
  "MBF-SI-REFERENCE-CONFIGURATION-UNSUPPORTED": "The model has reference configurations, which are outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-REFERENCE-UNRESOLVED": "This entry refers to something the model does not hold or SWBPIPE does not write, so SWBPIPE does not write the file.",
  "MBF-SI-SCHEMA-VERSION": "The model's schema version is not the current one, so SWBPIPE does not write the file.",
  "MBF-SI-SECTION-BINDING-UNSUPPORTED": "This pipe's section binding is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-SECTION-FIELD-NOT-EXPORTED": "SWBPIPE does not write this section field to the file.",
  "MBF-SI-SECTION-REFERENCE-STALE": "This pipe's section no longer matches the section it refers to, so SWBPIPE does not write the file.",
  "MBF-SI-SECTION-UNSUPPORTED": "This section is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-SHEAR-MODEL-NOT-EXPORTED": "SWBPIPE does not write the pressure data's shear model to the file.",
  "MBF-SI-SHEAR-MODEL-UNSUPPORTED": "The model's shear model is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-SHEAR-MODULUS-NOT-EXPORTED": "SWBPIPE does not write this material's shear modulus to the file.",
  "MBF-SI-SNUBBER-UNSUPPORTED": "This support is a snubber, which is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-SPECTRUM-UNSUPPORTED": "The model has response spectra, which are outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-SUBTRAHEND-SUPPORT-STATE-UNSUPPORTED": "This combination's support state is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-SUPPORT-DIRECTION-UNSUPPORTED": "This support's direction is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-SUPPORT-EMPTY": "This support restrains nothing, so it is not written.",
  "MBF-SI-SUPPORT-UNSUPPORTED": "This support is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED": "This material's properties depend on temperature, which is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-TEST-CONDITION-UNSUPPORTED": "This test condition is outside the subset SWBPIPE writes, so SWBPIPE does not write the file.",
  "MBF-SI-TOPOLOGY-UNSUPPORTED": "This node carries a support or load but no pipe segment, so SWBPIPE does not write the file.",
  "MBF-SI-UNIT-UNSUPPORTED": "This value's unit is not one SWBPIPE converts for the file, so SWBPIPE does not write the file.",
  "MBF-SI-UNITS-NORMALIZED": "SWBPIPE converted these units to SI by their exact definitions.",
  "MBF-SI-VALUE-OUT-OF-RANGE": "This value cannot be written as a finite number in the file's units, so SWBPIPE does not write the file.",
  "MODEL_SCHEMA_VERSION_RETIRED": "The model's schema version is retired, so SWBPIPE does not write the file."
};

/** The product-side description of a writer code (§3.3); a code without one is named as such, never by the engine's
 * message, and a blocking one says the file is not written. */
export function lossDescription(code: string, blocking = MBF_BLOCKING_CODES.has(code)): string {
  return MBF_LOSS_DESCRIPTIONS[code] ?? (blocking
    ? `SWBPIPE does not write the file because of this entry.`
    : "SWBPIPE does not write this entry to the file.");
}

export const MBF_IDENTITY_SENTENCE =
  "Identities travel in the identity file beside the MBF. Without it, an import forms ids from node numbers and refuses records whose meaning is still open.";
export const MBF_BOUNDARY_SENTENCE =
  "SWBPIPE writes the subset listed here. It does not check how another program reads or analyses the file.";
export const MBF_REPLACE_SENTENCE =
  "If you replace an existing .mbf file, the identity file of the same name beside it is replaced with it.";
export const MBF_UNDECLARED_MATERIAL_SENTENCE = "importing this file asks where these values came from";

/** The native command's arguments (§3.6); the route evidence keeps the sibling saves' member name. */
export type MbfExportArgs = ModelDocumentSaveArgs & {
  /** The settings text the review ran with (`mbfSettingsJson`): the native save reads the same text. */
  settingsJson: string;
  suggestedName: string;
  /** The digests the review showed: lowercase hex SHA-256 of the MBF text and of the identity file. */
  reviewed: { mbfSha256: string; sidecarSha256: string };
};

export type MbfSavedFile = { file_name: string; byte_count: number; sha256: string; replaced_existing: boolean; durability: string };

/** The native command's outcome. */
export type MbfExportSaveOutcome =
  | { outcome: "cancelled" }
  | {
      outcome: "saved";
      mbf: MbfSavedFile;
      identity_file: MbfSavedFile;
      durability: string;
      path_containment: string;
      limitation: string;
      warnings: ModelFileDiagnostic[];
    }
  /** A refusal before the chooser from the save's own worker run carries the bounded review. */
  | { outcome: "refused"; diagnostics: ModelFileDiagnostic[]; review?: MbfReviewPayload };

/** The function slice 4b.2 supplies: the native `save_mbf_export` through ingress. */
export type SaveMbfExport = (args: MbfExportArgs) => Promise<MbfExportSaveOutcome>;

/** A string of the bounded review: itself, or beyond the per-field bound its prefix (cut on a character boundary), its
 * full length in bytes and its SHA-256. */
export type MbfReviewField = string | { prefix: string; bytes: number; sha256: string };
export type MbfReviewListName = "not_written" | "omitted" | "warnings" | "undeclared_materials";
export type MbfReviewListCounts = { total: number; listed: number; not_shown: number; not_shown_blocking: number };

/** The engine's bounded review (`export_review`, at most 1 MiB): status first, the entries that fit, and the counts of
 * those that do not. */
export type MbfReviewPayload = {
  document_kind: "swbpipe.mbf_export_review";
  review_format: number;
  status: MbfReviewField;
  /** The writer refused, or an entry is not written: computed over the full result. */
  blocking: boolean;
  /** The writer refused and named nothing that explains it. */
  refused_unexplained: boolean;
  completeness: "complete" | "truncated";
  /** Whether the save result would be within the 64 MiB output ceiling. */
  output_ceiling: "within" | "beyond";
  mbf_sha256: MbfReviewField | null;
  sidecar_sha256: MbfReviewField | null;
  mbf_bytes: number;
  sidecar_bytes: number;
  layout_row_count: number;
  field_max_bytes: number;
  not_shown: number;
  not_shown_blocking: number;
  lists: Record<MbfReviewListName, MbfReviewListCounts>;
  unit_conversions: Array<{ unit: MbfReviewField; definition: MbfReviewField; factor: MbfReviewField | null }>;
  entries: Array<{
    list: Exclude<MbfReviewListName, "undeclared_materials">;
    code: MbfReviewField;
    entity: MbfReviewField;
    category: "unsupported" | "error" | "omitted" | "warning";
  }>;
  undeclared_materials: MbfReviewField[];
};

/** The native `review_mbf_export`'s arguments (§3.9.5): the same model, settings and binding a save would write. */
export type MbfReviewArgs = { canonicalText: string; claimedModelHash: string; settingsJson: string };

export type MbfReviewOutcome =
  | { outcome: "reviewed"; review: MbfReviewPayload }
  | { outcome: "refused"; diagnostics: ModelFileDiagnostic[] };

/** The function slice 4b.2 supplies: the native `review_mbf_export` through ingress. */
export type ReviewMbfExport = (args: MbfReviewArgs) => Promise<MbfReviewOutcome>;

/** The default reference temperature (§3.1): 20 °C, or 68 °F in a degF project (passed as
 * `reference_temperature_f`, exact). */
export function defaultReferenceTemperature(projectTemperatureUnit: unknown): MbfExportSettings["referenceTemperature"] {
  return projectTemperatureUnit === "degF" ? { value: 68, unit: "degF" } : { value: 20, unit: "degC" };
}

/** The writer settings (§3.1): no row limit (Q-4). */
export function mbfExportSettings(title: string, referenceTemperature: MbfExportSettings["referenceTemperature"]): MbfExportSettings {
  return { title, referenceTemperature, maxLayoutRows: null };
}

/** A label's slug: lowercase ASCII letters and digits in single-hyphen tokens (DEL-07-01 `export-file-names.md`). */
export function fileNameSlug(label: string): string {
  return label
    .normalize("NFKD")
    .replace(/[\u0300-\u036f]/g, "")
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
}

/** The suggested name: `swbpipe-model.mbf`, or `swbpipe-model-<label slug>.mbf` (#1711, kind `model`). */
export function suggestedMbfName(label: string): string {
  const slug = fileNameSlug(label);
  return slug ? `swbpipe-model-${slug}.mbf` : "swbpipe-model.mbf";
}

/** The family of a loss entry's entity (`component:…`, `support:…`, `model.spectra`, …). */
export function lossFamily(entity: string): string {
  const head = entity.split(/[:.]/)[0] ?? entity;
  return head === "model" ? entity.split(".").slice(0, 2).join(".") : head;
}

const FAMILY_LABELS: Record<string, string> = {
  component: "Components",
  support: "Supports",
  material: "Materials",
  section: "Sections",
  pipe: "Pipe segments",
  node: "Nodes",
  load: "Loads",
  load_case: "Load cases",
  combination: "Combinations"
};

export function familyLabel(family: string): string {
  return FAMILY_LABELS[family] ?? family;
}

/** A review string as text: a string carried in full as itself, a longer one as its prefix and its full length. */
export function reviewFieldText(field: MbfReviewField | null): string {
  if (field === null) return "";
  return typeof field === "string" ? field : `${field.prefix}… (${field.bytes} bytes in full, SHA-256 ${field.sha256})`;
}

/** A review string carried in full, or null. */
function wholeField(field: MbfReviewField | null): string | null {
  return typeof field === "string" ? field : null;
}

/** An entry as the review shows it: its code, its entity and the product-side description. */
export type MbfReviewEntry = { code: string; entity: string; description: string };

export type MbfReview = {
  status: string;
  /** The writer refused, or an entry is not written (computed over the full result, before any bound). */
  blocking: boolean;
  /** What keeps the file from being written, by family in first-seen order, as far as the review lists it: each
   * `unsupported` loss entry, and each error diagnostic that has no loss entry of its own. Each blocks the save. */
  notWritten: Array<{ family: string; entries: MbfReviewEntry[] }>;
  /** The writer refused and named nothing that explains it: the dialog still says that the file is not written. */
  refusedUnexplained: boolean;
  omitted: MbfReviewEntry[];
  unitConversions: MbfUnitConversion[];
  warnings: MbfReviewEntry[];
  undeclaredMaterials: string[];
  mbfSha256: string | null;
  sidecarSha256: string | null;
  layoutRowCount: number;
  /** Every entry is listed; a truncated review blocks the save. */
  complete: boolean;
  /** The save result would be within the output ceiling. */
  outputWithinCeiling: boolean;
  /** The entries not listed, and how many of them block. */
  notShown: number;
  notShownBlocking: number;
  lists: Record<MbfReviewListName, MbfReviewListCounts>;
};

/** The dialog's review from the engine's bounded review (§3.3): the engine has classified the entries; this adds the
 * product descriptions and the families. */
export function mbfReview(payload: MbfReviewPayload): MbfReview {
  const notWritten: MbfReview["notWritten"] = [];
  const omitted: MbfReviewEntry[] = [];
  const warnings: MbfReviewEntry[] = [];
  for (const item of payload.entries) {
    const code = reviewFieldText(item.code);
    const entity = reviewFieldText(item.entity);
    const blocking = item.list === "not_written";
    const entry = { code, entity, description: lossDescription(code, blocking || MBF_BLOCKING_CODES.has(code)) };
    if (item.list === "not_written") {
      const family = lossFamily(typeof item.entity === "string" ? item.entity : item.entity.prefix);
      const group = notWritten.find((candidate) => candidate.family === family);
      if (group) group.entries.push(entry);
      else notWritten.push({ family, entries: [entry] });
    } else if (item.list === "omitted") omitted.push(entry);
    else warnings.push(entry);
  }
  return {
    status: reviewFieldText(payload.status),
    blocking: payload.blocking,
    notWritten,
    refusedUnexplained: payload.refused_unexplained,
    omitted,
    unitConversions: payload.unit_conversions.map((item) => ({
      unit: reviewFieldText(item.unit),
      definition: reviewFieldText(item.definition),
      factor: item.factor === null ? null : reviewFieldText(item.factor)
    })),
    warnings,
    undeclaredMaterials: payload.undeclared_materials.map(reviewFieldText),
    mbfSha256: wholeField(payload.mbf_sha256),
    sidecarSha256: wholeField(payload.sidecar_sha256),
    layoutRowCount: payload.layout_row_count,
    complete: payload.completeness === "complete",
    outputWithinCeiling: payload.output_ceiling === "within",
    notShown: payload.not_shown,
    notShownBlocking: payload.not_shown_blocking,
    lists: payload.lists
  };
}

/** The review's global blocking conditions (§3.3): a truncated review, and a save result above the output ceiling. */
export function reviewBlockingCodes(review: MbfReview): string[] {
  return [...(review.complete ? [] : [MBF_REVIEW_TRUNCATED]), ...(review.outputWithinCeiling ? [] : [MBF_EXPORT_OUTPUT_TOO_LARGE])];
}

/** Whether the review admits Save (§3.3): written, nothing blocking, complete, within the output ceiling and with both
 * digests. The native save checks all of it again in its own run. */
export function reviewAdmitsSave(review: MbfReview | null): boolean {
  return review !== null && review.status === "written" && !review.blocking && review.notWritten.length === 0
    && review.complete && review.outputWithinCeiling && review.mbfSha256 !== null && review.sidecarSha256 !== null;
}

/** The status line after a save (§3.1 step 5): both files and the count of omitted entries. */
export function savedStatusLine(outcome: Extract<MbfExportSaveOutcome, { outcome: "saved" }>, omittedCount: number): string {
  const durability = outcome.durability === "durable" ? "" : " The folder's update may not be durable yet.";
  return `Saved ${outcome.mbf.file_name} and its identity file ${outcome.identity_file.file_name}; ${omittedCount} omitted ${omittedCount === 1 ? "entry" : "entries"}.${durability}`;
}
