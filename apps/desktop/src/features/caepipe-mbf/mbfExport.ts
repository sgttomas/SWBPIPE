// File > Export MBF… (DEL-17-04 Design `desktop-mbf-export-import.md` §3.1–§3.3, §3.5, §3.6; slice 4a): the export
// review's pure parts and the native save's argument and outcome types. The dialog (`MbfExportDialog.tsx`) runs the
// wasm writer (`services/mbfEngine.ts`) over the session's canonical model text for the review, then calls an
// injected `saveMbfExport` with the arguments of the native command `save_mbf_export`
// (`src-tauri/src/mbf_export.rs`), which repeats every check, writes the native writer's bytes and refuses a save
// whose digests differ from the reviewed ones. Slice 4b connects the command, the menu item and the Exports panel.
//
// Product text names only the format ("Model batch file (.mbf)") and states nothing about any other program
// (§3.5). Nothing here transmits anything (F-PIP-1).
import type { MbfExportSettings, MbfLossEntry, MbfRenderOutcome, MbfWriterReport } from "../../services/mbfEngine";
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

export const MBF_IDENTITY_SENTENCE =
  "Identities travel in the identity file beside the MBF. Without it, an import forms ids from node numbers and refuses records whose meaning is still open.";
export const MBF_BOUNDARY_SENTENCE =
  "SWBPIPE writes the subset listed here. It does not check how another program reads or analyses the file.";
export const MBF_REPLACE_SENTENCE =
  "If you replace an existing .mbf file, the identity file of the same name beside it is replaced with it.";
export const MBF_UNDECLARED_MATERIAL_SENTENCE = "importing this file asks where these values came from";

/** The native command's arguments (§3.6); the route evidence keeps the sibling saves' member name. */
export type MbfExportArgs = ModelDocumentSaveArgs & {
  settings: MbfExportSettings;
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

export type MbfReview = {
  status: "written" | "refused";
  profileId: string;
  mbfVersion: string;
  /** `unsupported` entries by family, in first-seen order: each blocks the save. */
  notWritten: Array<{ family: string; entries: MbfLossEntry[] }>;
  omitted: MbfLossEntry[];
  assumptions: string[];
  warnings: Array<{ code: string; message: string; entity?: string }>;
  undeclaredMaterials: string[];
  mbfSha256: string | null;
  sidecarSha256: string | null;
  layoutRowCount: number;
};

/** The review of §3.3 from the writer's outcome. */
export function mbfReview(outcome: Extract<MbfRenderOutcome, { status: "written" | "refused" }>): MbfReview {
  const report: MbfWriterReport = outcome.report;
  const notWritten: MbfReview["notWritten"] = [];
  for (const entry of report.loss_report.filter((item) => item.category === "unsupported")) {
    const family = lossFamily(entry.entity);
    const group = notWritten.find((item) => item.family === family);
    if (group) group.entries.push(entry);
    else notWritten.push({ family, entries: [entry] });
  }
  const undeclaredMaterials = Object.values(report.material_map ?? {})
    .filter((entry) => entry.origin === "undeclared")
    .map((entry) => String(entry.swbpipe_material ?? ""));
  return {
    status: outcome.status,
    profileId: outcome.profileId,
    mbfVersion: outcome.mbfVersion,
    notWritten,
    omitted: report.loss_report.filter((item) => item.category === "omitted"),
    assumptions: report.assumptions,
    warnings: report.diagnostics
      .filter((item) => item.severity === "warning")
      .map((item) => ({ code: item.code, message: item.message, entity: item.entity })),
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
