// File > Export MBF… (slices 4a and
// 4b.1): the export review's presentation and the native commands' argument and outcome types. The dialog
// (`MbfExportDialog.tsx`) never runs the wasm writer: it asks the injected `reviewMbfExport` (the native
// `review_mbf_export`, which runs the capped worker's task 3 in `review` mode) for the bounded review of the session's
// canonical model text, then calls the injected `saveMbfExport` (the native `save_mbf_export`), which repeats every
// check in its own `save`-mode run, refuses a truncated review or a result above the output ceiling, writes the native
// writer's bytes and refuses a save whose digests differ from the reviewed ones. The engine classifies the writer's
// result (`export_review` in `core/handoff/caepipe_mbf_engine`); this module adds the product descriptions. Slice 4b.2a
// connected the commands; slice 4b.2b mounts the dialog from File > Export MBF… and the Exports panel, on the barrier's
// services (`mbfExportCancel.ts`).
//
// Product text names only the format ("Model batch file (.mbf)") and states nothing about any other program
// (§3.3, §3.5): the review shows a neutral profile label, each entry by its code, its entity and a product-side
// description of what SWBPIPE did or did not write (`MBF_LOSS_DESCRIPTIONS`), never the engine's message text (the
// review does not carry it), and the unit conversions by their exact factors; the writer's assumptions are not
// displayed. Nothing here transmits anything (F-PIP-1).
import type { ModelDocumentSaveArgs, ModelFileDiagnostic } from "../model-file/modelDocumentSave";

/** The export dialog's writer settings. */
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
/** Any truncated review blocks the save, whatever the entries not shown. */
export const MBF_REVIEW_TRUNCATED = "MBF-REVIEW-TRUNCATED";
/** The save result (MBF, identity file, review and framing) would exceed the 64 MiB output ceiling. */
export const MBF_EXPORT_OUTPUT_TOO_LARGE = "MBF-EXPORT-OUTPUT-TOO-LARGE";
/** The browser-only route has no native review and no save. */
export const MBF_REVIEW_NATIVE_ONLY = "MBF-REVIEW-NATIVE-ONLY";
/** The native save's refusal of a model the writer refuses; its reply carries the bounded review. */
export const MBF_EXPORT_WRITER_REFUSED = "MBF-EXPORT-WRITER-REFUSED";
/** What each global blocking condition says. */
export const MBF_REVIEW_TRUNCATED_SENTENCE = "The loss report is too large to show in full, so the export is not offered.";
export const MBF_OUTPUT_TOO_LARGE_SENTENCE =
  "The model batch file, its identity file and the review together are larger than the 64 MiB an export may write, so the export is not offered.";
export const MBF_REVIEW_NATIVE_ONLY_SENTENCE = "The model batch file review runs in the desktop app.";
/** What native code appends to the chosen file's stem to name the identity file (A-5). */
export const MBF_IDENTITY_FILE_SUFFIX = ".swbpipe-ids.json";
/** The open items the writer's profile carries. */
export const MBF_PROFILE_OPEN_ITEMS = ["TBD-17-01-001", "TBD-17-01-002", "TBD-17-01-003"] as const;
/** The profile as the review names it: not the writer's internal profile id. */
export const MBF_PROFILE_LABEL = "SI model batch file, pilot subset";
/** In place of the writer's assumptions, which the review does not display. */
export const MBF_ASSUMPTIONS_SENTENCE = "The writer's assumptions are not displayed here.";

/** The sentence that ends the description of every blocking code. */
export const MBF_BLOCKING_SENTENCE = "so SWBPIPE does not write the file.";

/** The codes the writer raises as errors (`refuse`, or a diagnostic of severity `error`): each one alone keeps the file
 * from being written. The others are omissions, warnings and notes, after which the file can still be written. */
export const MBF_BLOCKING_CODES: ReadonlySet<string> = new Set([
  "MBF-SI-BEND-FLEXIBILITY-MISSING", "MBF-SI-BEND-GEOMETRY-INCONSISTENT", "MBF-SI-BEND-SIF-INCOMPLETE", "MBF-SI-BEND-UNSUPPORTED", "MBF-SI-COMBINATION-BASIS-UNSUPPORTED", "MBF-SI-COMBINATION-NESTED-UNSUPPORTED", "MBF-SI-COMPONENT-UNSUPPORTED", "MBF-SI-CONDITION-USE-UNSUPPORTED", "MBF-SI-CORROSION-ALLOWANCE-MISSING", "MBF-SI-DIFFERENCE-BASIS-UNSUPPORTED", "MBF-SI-GEOMETRY-UNSUPPORTED", "MBF-SI-IMPOSED-DISPLACEMENT-UNSUPPORTED", "MBF-SI-INPUT-INVALID", "MBF-SI-LOAD-CASE-FEATURE-UNSUPPORTED", "MBF-SI-LOAD-CASE-MAPPING-UNSUPPORTED", "MBF-SI-LOAD-UNSUPPORTED", "MBF-SI-MATERIAL-EXPANSION-MISSING", "MBF-SI-MATERIAL-POISSON-MISSING", "MBF-SI-NODE-NUMBER-LIMIT", "MBF-SI-PIPING-CODE-UNSUPPORTED", "MBF-SI-PREFLIGHT-REFUSED", "MBF-SI-PRESSURE-CASE-UNSUPPORTED", "MBF-SI-PRESSURE-REGION-FORMAT", "MBF-SI-PRESSURE-UNSUPPORTED", "MBF-SI-PROJECT-UNIT-UNSUPPORTED", "MBF-SI-REFERENCE-CONFIGURATION-UNSUPPORTED", "MBF-SI-REFERENCE-UNRESOLVED", "MBF-SI-SCHEMA-VERSION", "MBF-SI-SECTION-BINDING-UNSUPPORTED", "MBF-SI-SECTION-REFERENCE-STALE", "MBF-SI-SECTION-UNSUPPORTED", "MBF-SI-SHEAR-MODEL-UNSUPPORTED", "MBF-SI-SNUBBER-UNSUPPORTED", "MBF-SI-SPECTRUM-UNSUPPORTED", "MBF-SI-SUBTRAHEND-SUPPORT-STATE-UNSUPPORTED", "MBF-SI-SUPPORT-DIRECTION-UNSUPPORTED", "MBF-SI-SUPPORT-UNSUPPORTED", "MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED", "MBF-SI-TEST-CONDITION-UNSUPPORTED", "MBF-SI-TOPOLOGY-UNSUPPORTED", "MBF-SI-UNIT-UNSUPPORTED", "MBF-SI-VALUE-OUT-OF-RANGE", "MODEL_SCHEMA_VERSION_RETIRED"
]);

/** What SWBPIPE did or did not write, for each code the writer can emit. The engine's own messages are not
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

/** The product-side description of a writer code; a code without one is named as such, never by the engine's
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

/** The native command's arguments; the route evidence keeps the sibling saves' member name. */
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

/** A start's outcome as the barrier tags it (`mbfExportCancel.ts` `MbfStarted`): whether it is still current (no
 * retirement since, and the caller's own check still holds). The dialog discards an outcome that is not current. */
export type MbfStartedOutcome<T> = { outcome: T; current: () => boolean };

/** Whether the caller still wants the start (checked by the barrier immediately before every send). */
export type MbfStillCurrent = () => boolean;

/** The save the dialog is given: the barrier's `saveMbfExport` (the native `save_mbf_export` through ingress). */
export type SaveMbfExport = (args: MbfExportArgs, stillCurrent: MbfStillCurrent) => Promise<MbfStartedOutcome<MbfExportSaveOutcome>>;

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

/** The native `review_mbf_export`'s arguments: the same model, settings and binding a save would write. */
export type MbfReviewArgs = { canonicalText: string; claimedModelHash: string; settingsJson: string };

export type MbfReviewOutcome =
  | { outcome: "reviewed"; review: MbfReviewPayload }
  | { outcome: "refused"; diagnostics: ModelFileDiagnostic[] };

/** The review the dialog is given: the barrier's `reviewMbfExport` (the native `review_mbf_export` through ingress). */
export type ReviewMbfExport = (args: MbfReviewArgs, stillCurrent: MbfStillCurrent) => Promise<MbfStartedOutcome<MbfReviewOutcome>>;

/** The default reference temperature: 20 °C, or 68 °F in a degF project (passed as
 * `reference_temperature_f`, exact). */
export function defaultReferenceTemperature(projectTemperatureUnit: unknown): MbfExportSettings["referenceTemperature"] {
  return projectTemperatureUnit === "degF" ? { value: 68, unit: "degF" } : { value: 20, unit: "degC" };
}

/** The writer settings: no row limit (Q-4). */
export function mbfExportSettings(title: string, referenceTemperature: MbfExportSettings["referenceTemperature"]): MbfExportSettings {
  return { title, referenceTemperature, maxLayoutRows: null };
}

/** A label's slug: lowercase ASCII letters and digits in single-hyphen tokens. */
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

/** The dialog's review from the engine's bounded review: the engine has classified the entries; this adds the
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

/** The review's global blocking conditions: a truncated review, and a save result above the output ceiling. */
export function reviewBlockingCodes(review: MbfReview): string[] {
  return [...(review.complete ? [] : [MBF_REVIEW_TRUNCATED]), ...(review.outputWithinCeiling ? [] : [MBF_EXPORT_OUTPUT_TOO_LARGE])];
}

/** Whether the review admits Save: written, nothing blocking, complete, within the output ceiling and with both
 * digests. The native save checks all of it again in its own run. */
export function reviewAdmitsSave(review: MbfReview | null): boolean {
  return review !== null && review.status === "written" && !review.blocking && review.notWritten.length === 0
    && review.complete && review.outputWithinCeiling && review.mbfSha256 !== null && review.sidecarSha256 !== null;
}

/** The status line after a save: both files and the count of omitted entries. */
export function savedStatusLine(outcome: Extract<MbfExportSaveOutcome, { outcome: "saved" }>, omittedCount: number): string {
  const durability = outcome.durability === "durable" ? "" : " The folder's update may not be durable yet.";
  return `Saved ${outcome.mbf.file_name} and its identity file ${outcome.identity_file.file_name}; ${omittedCount} omitted ${omittedCount === 1 ? "entry" : "entries"}.${durability}`;
}

/** The export's own refusals that are not about the model (product-text rule): what happened and what the
 * user can do (try again, reload the window, restart SWBPIPE), in the product's words. None of them names the
 * mechanism that keeps one export from overlapping another, and none suggests that anything here grants a save. */
export const MBF_EXPORT_CANCELLED = "MBF-EXPORT-CANCELLED";
export const MBF_EXPORT_RETIREMENT_UNCONFIRMED = "MBF-EXPORT-RETIREMENT-UNCONFIRMED";
export const MBF_EXPORT_EPOCH_INVALID = "MBF-EXPORT-EPOCH-INVALID";
export const MBF_EXPORT_EPOCH_EXHAUSTED = "MBF-EXPORT-EPOCH-EXHAUSTED";
export const MBF_EXPORT_RETIREMENT_ID_INVALID = "MBF-EXPORT-RETIREMENT-ID-INVALID";
/** A start refused at the native ingress once this window's connection to the app has switched to its fallback, which
 * cannot carry the export's request. For a review this means nothing was written; for
 * a save the outcome is unknown (`MBF_SAVE_REFUSAL_TEXT`). */
export const INGRESS_RAW_BODY_REQUIRED = "INGRESS-RAW-BODY-REQUIRED";
/** A start this page did not send because an earlier save's outcome was unknown (`mbfSaveOutcomeUnknown`): the page's
 * connection to the app can no longer carry an export, until the window is reloaded or SWBPIPE restarts
 * (`mbfExportCancel.ts`, "Export unavailable in this page"). Nothing was sent for this start; what the earlier save
 * wrote stays unknown, so its text repeats where to look. */
export const MBF_EXPORT_UNAVAILABLE = "MBF-EXPORT-UNAVAILABLE";
/** A failure of the review or the save that has no code of its own. */
export const MBF_EXPORT_FAILED = "MBF-EXPORT-FAILED";

export const MBF_EXPORT_REFUSAL_TEXT: Readonly<Record<string, string>> = {
  [MBF_EXPORT_CANCELLED]: "The export was cancelled before anything was written. You can start it again.",
  [MBF_EXPORT_RETIREMENT_UNCONFIRMED]:
    "The export did not start because SWBPIPE could not confirm that an earlier export had stopped. Nothing was written. Try again; if it keeps happening, reload the window or restart SWBPIPE.",
  [MBF_EXPORT_EPOCH_INVALID]:
    "The export did not start because SWBPIPE could not accept its request. Nothing was written. Try again; if it keeps happening, restart SWBPIPE.",
  [MBF_EXPORT_EPOCH_EXHAUSTED]:
    "Export is no longer available until SWBPIPE restarts. Nothing was written. Restart SWBPIPE to export again.",
  [MBF_EXPORT_RETIREMENT_ID_INVALID]:
    "SWBPIPE could not stop an earlier export because it could not accept the request to stop it. Try again; if it keeps happening, restart SWBPIPE.",
  [INGRESS_RAW_BODY_REQUIRED]:
    "Export isn't available in this window. Nothing was written. Reload the window or restart SWBPIPE to export again.",
  [MBF_EXPORT_UNAVAILABLE]:
    "Export isn't available in this window: SWBPIPE could not confirm whether an earlier export wrote its files. This export did not start. If you chose a folder for the earlier export, check that folder for the model batch file and its identity file. Reload the window or restart SWBPIPE to export again."
};

/** The save's own text for the codes whose meaning differs for a save. A review
 * writes nothing, so `MBF_EXPORT_REFUSAL_TEXT` holds for it. A save refused `INGRESS-RAW-BODY-REQUIRED` has an unknown
 * outcome: Tauri 2.11.1's transport resends an invoke through its fallback when its `fetch` fails or when reading the
 * reply fails, so the original save may have committed, written or undone itself before its reply was lost, and the
 * refusal proves only that the resend wrote nothing. The page cannot see the fallback, so it cannot tell such a save
 * from one first sent after the fallback: every save refused so is reported as unknown. */
export const MBF_SAVE_OUTCOME_UNKNOWN_TEXT =
  "SWBPIPE could not confirm whether this export wrote its files, because this window can no longer send exports. If you chose a folder for it, check that folder for the model batch file and its identity file. Reload the window or restart SWBPIPE to export again.";
export const MBF_SAVE_REFUSAL_TEXT: Readonly<Record<string, string>> = {
  [INGRESS_RAW_BODY_REQUIRED]: MBF_SAVE_OUTCOME_UNKNOWN_TEXT
};

/** Whether a save's diagnostics leave its outcome unknown (`MBF_SAVE_REFUSAL_TEXT`): it may have written its files. */
export function mbfSaveOutcomeUnknown(diagnostics: readonly ModelFileDiagnostic[]): boolean {
  return diagnostics.some((item) => item.code === INGRESS_RAW_BODY_REQUIRED);
}

/** Whose closed export a status line is about: the export of the named project, from this project session or an
 * earlier one, so that an old save's outcome is never read as the current project's export. */
export type ClosedExportAttribution = { projectName: string; earlierSession: boolean };

function closedExportSubject({ projectName, earlierSession }: ClosedExportAttribution): string {
  return `The export of "${projectName}"${earlierSession ? " from an earlier project session" : ""}`;
}

/** The status line for a closed dialog's save (`mbfExportSession.ts`). A close after the save's commit does not
 * stop it: it may still finish, or fail and undo itself, so the line first says only that it is finishing. The save's
 * own outcome then follows, whatever the close's receipt said: saved, naming both files, or refused, with its codes and
 * their product text (`mbfSaveRefusalText`), a write failure or an undone save included, or unknown, when its result
 * cannot show whether it wrote its files (`mbfSaveOutcomeUnknown`). The outcome's line says only that the export was
 * closed before its result came back, since a save may have completed before the close reached it. */
export function closedSaveStatusLine(
  report:
    | { kind: "finishing" }
    | { kind: "saved"; outcome: Extract<MbfExportSaveOutcome, { outcome: "saved" }> }
    | { kind: "failed"; diagnostics: ModelFileDiagnostic[] }
    | { kind: "unknown"; diagnostics: ModelFileDiagnostic[] },
  attribution: ClosedExportAttribution
): string {
  const subject = closedExportSubject(attribution);
  if (report.kind === "finishing") return `${subject} was already writing its files when it was closed; it is finishing.`;
  if (report.kind === "saved") {
    const durability = report.outcome.durability === "durable" ? "" : " The folder's update may not be durable yet.";
    return `${subject}, closed before its result came back, finished: both files were written, ${report.outcome.mbf.file_name} and its identity file ${report.outcome.identity_file.file_name}.${durability}`;
  }
  const items = report.diagnostics.map((item) => `${item.code}${item.path ? ` (${item.path})` : ""}: ${mbfSaveRefusalText(item)}`);
  if (report.kind === "unknown") return `${subject}, closed before its result came back, has no confirmed result. ${items.join(" ")}`;
  return `${subject}, closed before its result came back, did not finish. ${items.join(" ")}`;
}

/** A code the writer emits (named by the product-side description), as opposed to the save's own codes. */
function isWriterCode(code: string): boolean {
  return code.startsWith("MBF-SI-") || Object.hasOwn(MBF_LOSS_DESCRIPTIONS, code);
}

/** A refusal's text: a writer code by its product-side description, a review condition by its sentence, the export's
 * own refusals by their product text (what happened and what the user can do), else the native message (the save's
 * and the review's own text, never the engine's): a write failure or an undone save by the save's own words. */
export function mbfRefusalText(item: ModelFileDiagnostic): string {
  if (isWriterCode(item.code)) return lossDescription(item.code, true);
  if (item.code === MBF_REVIEW_TRUNCATED) return MBF_REVIEW_TRUNCATED_SENTENCE;
  if (item.code === MBF_EXPORT_OUTPUT_TOO_LARGE) return MBF_OUTPUT_TOO_LARGE_SENTENCE;
  if (Object.hasOwn(MBF_EXPORT_REFUSAL_TEXT, item.code)) return MBF_EXPORT_REFUSAL_TEXT[item.code];
  return item.message;
}

/** A save's refusal text: `mbfRefusalText`, except for the codes whose meaning differs for a save
 * (`MBF_SAVE_REFUSAL_TEXT`). */
export function mbfSaveRefusalText(item: ModelFileDiagnostic): string {
  if (Object.hasOwn(MBF_SAVE_REFUSAL_TEXT, item.code)) return MBF_SAVE_REFUSAL_TEXT[item.code];
  return mbfRefusalText(item);
}

/** A failed review or save as a diagnostic: a start refused at the native ingress after the window's fallback by its
 * code (its product text says what to do), anything else as the export's own failure with the native text. */
export function mbfFailureDiagnostic(error: unknown): ModelFileDiagnostic {
  const message = String(error instanceof Error ? error.message : error);
  if (message.includes(INGRESS_RAW_BODY_REQUIRED)) return { code: INGRESS_RAW_BODY_REQUIRED, path: "", message };
  return { code: MBF_EXPORT_FAILED, path: "", message };
}
