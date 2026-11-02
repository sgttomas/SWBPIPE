// File > Export MBF… (DEL-17-04 Design `desktop-mbf-export-import.md` §3.1–§3.3, §3.5, §3.6; slice 4a): the export
// review's pure parts and the native save's argument and outcome types. The dialog (`MbfExportDialog.tsx`) runs the
// wasm writer (`services/mbfEngine.ts`) over the session's canonical model text for the review, then calls an
// injected `saveMbfExport` with the arguments of the native command `save_mbf_export`
// (`src-tauri/src/mbf_export.rs`), which repeats every check, writes the native writer's bytes and refuses a save
// whose digests differ from the reviewed ones. Slice 4b connects the command, the menu item and the Exports panel.
//
// Product text names only the format ("Model batch file (.mbf)") and states nothing about any other program
// (§3.3, §3.5): the review shows a neutral profile label, each loss entry by its code, its entity and a product-side
// description of what SWBPIPE did or did not write (`MBF_LOSS_DESCRIPTIONS`), never the engine's message text, and the
// unit conversions by their exact factors; the writer's assumptions are not displayed. Nothing here transmits anything
// (F-PIP-1).
import type { MbfExportSettings, MbfLossEntry, MbfRenderOutcome, MbfUnitConversion, MbfWriterReport } from "../../services/mbfEngine";
import type { ModelDocumentSaveArgs, ModelFileDiagnostic } from "../model-file/modelDocumentSave";

/** The governed local-private, lossless route of the save (A-4). */
export const MBF_EXPORT_ROUTE_ID = "DOTH-MBF-012";
export const MBF_EXPORT_INTENT_LABEL =
  "Save my model, including private project data, as a model batch file and its identity file in a folder I choose";
export const MBF_EXPORT_ROUTE = { routeId: MBF_EXPORT_ROUTE_ID, intentLabel: MBF_EXPORT_INTENT_LABEL } as const;
/** The native refusal of a save whose model or settings changed after the review. */
export const MBF_EXPORT_REVIEW_STALE = "MBF-EXPORT-REVIEW-STALE";
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
  | { outcome: "refused"; diagnostics: ModelFileDiagnostic[] };

/** The function slice 4b supplies: the native `save_mbf_export` through ingress. */
export type SaveMbfExport = (args: MbfExportArgs) => Promise<MbfExportSaveOutcome>;

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

/** A loss entry or warning as the review shows it: its code, its entity and the product-side description. */
export type MbfReviewEntry = { code: string; entity: string; description: string };

export type MbfReview = {
  status: "written" | "refused";
  /** What keeps the file from being written, by family in first-seen order: each `unsupported` loss entry, and each
   * error diagnostic that has no loss entry of its own (an invalid or out-of-range value, an unresolved reference, a
   * schema version). Each blocks the save. */
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
};

/** The review of §3.3 from the writer's outcome. */
function reviewEntry(item: { code: string; entity?: string }, blocking?: boolean): MbfReviewEntry {
  return { code: item.code, entity: item.entity ?? "", description: lossDescription(item.code, blocking) };
}

export function mbfReview(outcome: Extract<MbfRenderOutcome, { status: "written" | "refused" }>): MbfReview {
  const report: MbfWriterReport = outcome.report;
  const notWritten: MbfReview["notWritten"] = [];
  const block = (entry: MbfReviewEntry) => {
    const family = lossFamily(entry.entity);
    const group = notWritten.find((item) => item.family === family);
    if (group) group.entries.push(entry);
    else notWritten.push({ family, entries: [entry] });
  };
  for (const entry of report.loss_report.filter((item: MbfLossEntry) => item.category === "unsupported")) block(reviewEntry(entry, true));
  // An error the writer raised without a loss entry (`diag` and `invalid` in the engine) is never silent.
  for (const item of report.diagnostics.filter((diagnostic) => diagnostic.severity === "error")) {
    const entity = item.entity ?? "";
    if (!report.loss_report.some((entry) => entry.code === item.code && entry.entity === entity)) block(reviewEntry(item, true));
  }
  const undeclaredMaterials = Object.values(report.material_map ?? {})
    .filter((entry) => entry.origin === "undeclared")
    .map((entry) => String(entry.swbpipe_material ?? ""));
  return {
    status: outcome.status,
    notWritten,
    refusedUnexplained: outcome.status === "refused" && notWritten.length === 0,
    omitted: report.loss_report.filter((item) => item.category === "omitted").map((item) => reviewEntry(item)),
    unitConversions: outcome.unitConversions,
    // Each omitted entry is also a warning diagnostic: the warnings listed are the others.
    warnings: report.diagnostics
      .filter((item) => item.severity === "warning")
      .filter((item) => !report.loss_report.some((entry) => entry.category === "omitted" && entry.code === item.code && entry.entity === item.entity))
      .map((item) => reviewEntry(item)),
    undeclaredMaterials,
    mbfSha256: outcome.mbfSha256,
    sidecarSha256: outcome.sidecarSha256,
    layoutRowCount: report.layout_row_count
  };
}

/** Whether the review admits Save (§3.3): written, with nothing not written and both digests. */
export function reviewAdmitsSave(review: MbfReview | null): boolean {
  return review !== null && review.status === "written" && review.notWritten.length === 0
    && review.mbfSha256 !== null && review.sidecarSha256 !== null;
}

/** The status line after a save (§3.1 step 5): both files and the count of omitted entries. */
export function savedStatusLine(outcome: Extract<MbfExportSaveOutcome, { outcome: "saved" }>, omittedCount: number): string {
  const durability = outcome.durability === "durable" ? "" : " The folder's update may not be durable yet.";
  return `Saved ${outcome.mbf.file_name} and its identity file ${outcome.identity_file.file_name}; ${omittedCount} omitted ${omittedCount === 1 ? "entry" : "entries"}.${durability}`;
}
