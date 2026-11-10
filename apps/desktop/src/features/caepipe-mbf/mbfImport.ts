// File > Import MBF… (DEL-17-04 Design `desktop-mbf-export-import.md` §4.1, §4.3, §4.6–§4.8; slice 5): the import
// review's presentation and the native commands' argument and outcome types. The dialog (`MbfImportDialog.tsx`) never
// runs an importer: the native `open_mbf_import` / `review_mbf_import` run worker task 5 and return a bounded review
// (`import_review` in `core/handoff/caepipe_mbf_engine`); no file bytes, model, import report or id map reach the
// webview during review. Adopt goes through the maintained Open transaction (`handleOpenModelDocument` with the
// workspace session's `mbfImportSource`), whose `read()` is the native `adopt_mbf_import` followed by the one
// `checkNativeOutcome`.
//
// Product text names only the format and states SWBPIPE's own mapping behaviour, never facts about another program
// (owner, 2026-10-26): each code is shown with its product description (`MBF_IMPORT_DESCRIPTIONS`), and each applied
// mapping assumption by its structured identifier and parameters with a product description
// (`MBF_IMPORT_ASSUMPTIONS`). The engine's message texts and assumption sentences are not carried. Nothing here
// transmits anything (F-PIP-1).
import type { ModelFileDiagnostic } from "../model-file/modelDocumentShape";

// -- Codes ------------------------------------------------------------------------------------------------------------

export const MBF_IMPORT_NATIVE_ONLY = "MBF-IMPORT-NATIVE-ONLY";
export const MBF_IMPORT_CANCELLED = "MBF-IMPORT-CANCELLED";
export const MBF_IMPORT_RETIREMENT_UNCONFIRMED = "MBF-IMPORT-RETIREMENT-UNCONFIRMED";
export const MBF_IMPORT_REVIEW_STALE = "MBF-IMPORT-REVIEW-STALE";
export const MBF_IMPORT_REVIEW_TRUNCATED = "MBF-IMPORT-REVIEW-TRUNCATED";
export const MBF_IMPORT_ASSUMPTION_UNAVAILABLE = "MBF-IMPORT-ASSUMPTION-UNAVAILABLE";
export const MBF_IMPORT_MODEL_TOO_LARGE = "MBF-IMPORT-MODEL-TOO-LARGE";
export const MBF_IMPORT_NOT_ADOPTABLE = "MBF-IMPORT-NOT-ADOPTABLE";
export const MBF_IMPORT_FAILED = "MBF-IMPORT-FAILED";
export const MBF_IMPORT_BUSY = "MBF-IMPORT-BUSY";
export const INGRESS_RAW_BODY_REQUIRED = "INGRESS-RAW-BODY-REQUIRED";

/** The boundary statement every review shows (§4.3). */
export const MBF_IMPORT_BOUNDARY_SENTENCE =
  "A successful import means the listed subset of this file was read. It is not evidence that two programs analyse the model alike.";
export const MBF_IMPORT_BUSY_MESSAGE = "Import not adopted: another project operation is in progress.";

// -- Types ------------------------------------------------------------------------------------------------------------

/** A review's long string: as itself, or as `{prefix, bytes, sha256}` beyond the per-field bound. */
export type ReviewField = string | { prefix: string; bytes: number; sha256: string };

export type MbfImportListCounts = { total: number; listed: number; not_shown: number; not_shown_blocking: number };
export type MbfImportListName = "refusals" | "document_check" | "assumptions" | "declaration_needed" | "loss_entries" | "target_options";

/** A designated source as the review states it (§5.4): the file, its digest and size, and the outcome with its reasons
 * (`MBF_IMPORT_DESIGNATION_REASONS`) and the identity classes that differ. */
export type MbfImportDesignationPayload = {
  file_name: ReviewField;
  sha256: ReviewField;
  bytes: number;
  outcome: "not_established" | "established_for_identity_set";
  reasons: string[];
  differing_classes: string[];
};

/** The worker's review (`swbpipe.mbf_import_review`, format 3), as the native reply embeds it. */
export type MbfImportReviewPayload = {
  document_kind: "swbpipe.mbf_import_review";
  format: 3;
  status: "imported" | "refused";
  adoptable: boolean;
  model_sha256: string | null;
  model_bytes: number;
  completeness: "complete" | "truncated";
  transfer_limit: "within" | "beyond";
  blocks: { import_refused: boolean; document_check: boolean; truncated: boolean; model_too_large: boolean; assumption_unavailable: boolean };
  field_max_bytes: number;
  source: {
    file_name: ReviewField;
    mbf_sha256: ReviewField;
    mbf_bytes: number;
    identity_file: { state: "absent" } | { state: "present"; sha256: ReviewField; bytes: number };
  };
  binding: { state: "bound" | "unbound" | "rebound_unconfirmed"; reason: string };
  declaration: { sha256: ReviewField | null; recorded_origins: number };
  units: { unit_mode: ReviewField; angles: ReviewField; vertical_axis: ReviewField; project_units: Record<string, ReviewField> | "not_available" };
  model: { document_check: "passed" | "blocked" | "not_run"; schema_status: ReviewField | null };
  identity: {
    basis: "restored_from_identity_file" | "formed_from_mbf_names";
    classes: Array<{ class: string; count: number; restored: number; formed_from_mbf_names: number }>;
    /** Known original ids not preserved, counted once; zero does not prove original completeness. */
    unrecoverable: number;
    unrecovered_fields: number;
    /** Established only for a designated source whose every check passed, in a complete review (§5.4): the same ids in
     * each identity class, never whole-model equivalence. */
    source_completeness: "not_established" | "established_for_identity_set";
    designation: MbfImportDesignationPayload | null;
  };
  counts: { not_shown: number; not_shown_blocking: number; lists: Record<MbfImportListName, MbfImportListCounts> };
  refusals: Array<{ code: ReviewField; line: number; entity: ReviewField }>;
  document_check: Array<{ code: ReviewField; path: ReviewField }>;
  assumptions: Array<{ id: ReviewField; parameters: Record<string, ReviewField> }>;
  declaration_needed: ReviewField[];
  loss_entries: Array<{ code: ReviewField; entity: ReviewField; category: ReviewField }>;
  target_options: Array<{ name: ReviewField; state: "recorded_not_applied" }>;
};

export type MbfImportDiagnostic = ModelFileDiagnostic;

/** `open_mbf_import`'s and `review_mbf_import`'s reply (§4.7). */
export type MbfImportReviewOutcome =
  | { outcome: "reviewed"; sourceId: number; fileName: string; reviewSha256: string; review: MbfImportReviewPayload }
  | { outcome: "refused"; fileName: string; diagnostics: MbfImportDiagnostic[] };
export type MbfImportOpenOutcome = { outcome: "cancelled" } | MbfImportReviewOutcome;

/** `designate_mbf_import_source`'s reply (§5.4): the review stating the designation (X) and the digest of the review it
 * was formed over (Y, the review shown when it was sent). */
export type MbfImportDesignateOutcome =
  | { outcome: "cancelled" }
  | { outcome: "designated"; sourceId: number; fileName: string; baseReviewSha256: string; reviewSha256: string; review: MbfImportReviewPayload }
  | { outcome: "refused"; fileName: string; diagnostics: MbfImportDiagnostic[] };

/** `adopt_mbf_import`'s reply: the `NativeModelDocumentOutcome` shape `checkNativeOutcome` takes, or a refusal. */
export type MbfImportAdoptOutcome =
  | { outcome: "opened"; file_name: string; byte_count: number; schema_status: string; document: unknown; warnings: ModelFileDiagnostic[]; reviewSha256: string; modelSha256: string }
  | { outcome: "refused"; file_name: string; diagnostics: MbfImportDiagnostic[] };

/** One material's provenance declaration (§4.1 step 4): vendor-library and unknown origins cannot be declared. */
export type MbfDeclarationEntry = { origin: "project_invented" | "user_supplied"; source_name: string };
export type MbfDeclaration = Record<string, MbfDeclarationEntry> | null;

export type MbfReviewImportArgs = { sourceId: number; declaration: MbfDeclaration };
export type MbfAdoptImportArgs = { sourceId: number; declaration: MbfDeclaration; reviewSha256: string };
/** Designate's arguments: the exact current declaration and the review shown (`reviewSha256`, the base review Y). */
export type MbfDesignateImportArgs = { sourceId: number; declaration: MbfDeclaration; reviewSha256: string };

/** What Adopt captures from the current review (§4.8.5, §4.8.7): bounded identity and status metadata only. The flow
 * keeps no review body, model or file bytes. */
export type MbfAdoptCapture = {
  sourceId: number;
  declaration: MbfDeclaration;
  reviewSha256: string;
  modelSha256: string;
  fileName: string;
  binding: "bound" | "unbound" | "rebound_unconfirmed";
  restored: number;
  formedFromNames: number;
};

// -- Fields -----------------------------------------------------------------------------------------------------------

/** A review field as text: a long one by its prefix, marked as cut. */
export function fieldText(field: ReviewField | null | undefined): string {
  if (field === null || field === undefined) return "";
  if (typeof field === "string") return field;
  return `${field.prefix}… (${field.bytes} bytes)`;
}

/** Whether a field is carried in full (a material whose name the review cuts cannot be declared). */
export function fieldWhole(field: ReviewField): field is string {
  return typeof field === "string";
}

// -- Product descriptions ---------------------------------------------------------------------------------------------

/** The product description of every code the import can show: the importer's own codes, the native commands' and the
 * desktop's. Each states what SWBPIPE did with the file; none states a fact about another program. */
export const MBF_IMPORT_DESCRIPTIONS: Readonly<Record<string, string>> = {
  // The importer's codes (core/handoff/caepipe_mbf_engine/src/importer and sidecar.rs).
  "MBF-IMPORT-BEND-FLEXIBILITY-MISSING": "This bend states no flexibility factor, so SWBPIPE does not import the file.",
  "MBF-IMPORT-BEND-GEOMETRY-INVALID": "This bend's radius, angle and adjoining runs do not agree, so SWBPIPE does not import the file.",
  "MBF-IMPORT-BEND-RADIUS-MISSING": "This bend states no radius, so SWBPIPE does not import the file.",
  "MBF-IMPORT-BEND-UNSUPPORTED": "This bend's arrangement is outside the subset SWBPIPE imports.",
  "MBF-IMPORT-CHARACTER-UNSUPPORTED": "The file contains a character SWBPIPE does not read.",
  "MBF-IMPORT-COMMENT-INVALID": "This comment line is not in the form SWBPIPE reads.",
  "MBF-IMPORT-COMMENT-UNKNOWN": "This comment is not one SWBPIPE reads.",
  "MBF-IMPORT-COMMENT-UNSUPPORTED": "This comment is outside the subset SWBPIPE imports.",
  "MBF-IMPORT-CONTENTS-UNSUPPORTED": "This element's contents are outside the subset SWBPIPE imports.",
  "MBF-IMPORT-CORROSION-ALLOWANCE-MISSING": "This pipe section states no corrosion allowance, which the imported pressure data needs.",
  "MBF-IMPORT-DIRECTION-UNRECOVERABLE": "The identity file does not record this load's direction. SWBPIPE uses the reported fallback direction.",
  "MBF-IMPORT-DISPLACEMENT-UNSUPPORTED": "This imposed displacement is outside the subset SWBPIPE imports.",
  "MBF-IMPORT-ENCODING-UNSUPPORTED": "The file's text encoding is not one SWBPIPE reads.",
  "MBF-IMPORT-FIELD-MISSING": "A field SWBPIPE needs is missing from this record.",
  "MBF-IMPORT-FIELD-UNDOCUMENTED": "This record has a field outside the documented form SWBPIPE reads.",
  "MBF-IMPORT-FORCE-CASE-UNSUPPORTED": "This force's load case is outside the subset SWBPIPE imports.",
  "MBF-IMPORT-FROM-COORDINATES-CONFLICT": "This row's start coordinates conflict with the position already read for the node.",
  "MBF-IMPORT-GEOMETRY-ZERO-LENGTH": "This element has zero length, so SWBPIPE does not import the file.",
  "MBF-IMPORT-GRAVITY-UNSUPPORTED": "The file's gravity setting is outside the subset SWBPIPE imports.",
  "MBF-IMPORT-ID-UNRECOVERABLE": "This original load id is not preserved because the file sums it with other loads.",
  "MBF-IMPORT-IDENTITIES-MERGED": "The file sums these original loads into one primitive with a formed id.",
  "MBF-IMPORT-REFERENCE-UNRECOVERABLE": "The identity file preserves this bend's id but not its original tangent-node choice. SWBPIPE uses the near tangent node.",
  "MBF-IMPORT-FIELD-UNRECOVERABLE": "The identity file preserves this support's id but does not record its original family.",
  "MBF-IMPORT-SOURCE-COMPLETENESS-UNESTABLISHED": "The file cannot recover members the writer does not export or establish whether every original source member was represented.",
  "MBF-IMPORT-LENGTH-NOTATION-INVALID": "This length is not in a notation SWBPIPE reads exactly.",
  "MBF-IMPORT-LINE-END-UNSUPPORTED": "The file's line endings are not ones SWBPIPE reads.",
  "MBF-IMPORT-LOAD-FIELD-UNSUPPORTED": "This load field is outside the subset SWBPIPE imports.",
  "MBF-IMPORT-LOAD-SLOT-INACTIVE": "This load slot is inactive in the file and is not imported.",
  "MBF-IMPORT-LOAD-UNSUPPORTED": "This load is outside the subset SWBPIPE imports.",
  "MBF-IMPORT-LOADS-ABSENT": "The file has no load records; the imported model has the loads SWBPIPE forms by default only.",
  "MBF-IMPORT-MATERIAL-FRP-UNSUPPORTED": "This material type is outside the subset SWBPIPE imports.",
  "MBF-IMPORT-MATERIAL-PROTECTED-SUSPECTED": "This material looks like protected library data, which SWBPIPE does not import.",
  "MBF-IMPORT-MATERIAL-PROVENANCE-REQUIRED": "This material needs a provenance declaration: project-invented example data, or your own named source.",
  "MBF-IMPORT-MATERIAL-TEMPERATURE-DEPENDENT": "This material's properties vary with temperature, which this import does not read.",
  "MBF-IMPORT-NODE-NUMBER-INVALID": "This node number is not one SWBPIPE reads.",
  "MBF-IMPORT-NODE-POSITION-MISSING": "This node's position cannot be formed from the file.",
  "MBF-IMPORT-NODE-UNKNOWN": "This record names a node the file does not define.",
  "MBF-IMPORT-NUMBER-INVALID": "This value is not a number SWBPIPE reads exactly.",
  "MBF-IMPORT-OPTION-UNSUPPORTED": "This option is outside the subset SWBPIPE imports.",
  "MBF-IMPORT-ORIENTATION-DEFAULTED": "This element's local orientation is not stated; SWBPIPE uses its default orientation rule.",
  "MBF-IMPORT-ORIENTATION-UNUSABLE": "This element's stated orientation cannot be used, so SWBPIPE does not import the file.",
  "MBF-IMPORT-PRESSURE-BORE-MISMATCH": "Pressure regions meet at a bore change SWBPIPE cannot join exactly.",
  "MBF-IMPORT-PRESSURE-CLOSURE-INVALID": "The pressure closure declaration is not valid for this file.",
  "MBF-IMPORT-PRESSURE-CLOSURE-REQUIRED": "This file's pressure needs a closure declaration at its ends, which the desktop import does not offer yet.",
  "MBF-IMPORT-PRESSURE-DISPLACEMENT-UNSUPPORTED": "A displacement in a pressure case is outside the subset SWBPIPE imports.",
  "MBF-IMPORT-PRESSURE-EFFECTS-OMITTED": "The file states no pressure-effect options; SWBPIPE's exact pressure route models neither of those effects.",
  "MBF-IMPORT-PRESSURE-GEOMETRY-INVALID": "A pressure region's geometry is outside what the exact pressure route accepts.",
  "MBF-IMPORT-PRESSURE-ID-COLLISION": "A pressure region's identity collides with another, so SWBPIPE does not import the file.",
  "MBF-IMPORT-PRESSURE-INTERNAL-BRANCH": "A pressure region branches internally, which the exact pressure route does not accept.",
  "MBF-IMPORT-PRESSURE-MITRE": "A pressure region includes a mitre, which the exact pressure route does not accept.",
  "MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR": "A pressure region's run turns where the exact pressure route needs a straight run.",
  "MBF-IMPORT-PRESSURE-UNSUPPORTED": "This pressure record is outside the subset SWBPIPE imports.",
  "MBF-IMPORT-PROPERTY-MISSING": "A property SWBPIPE needs is missing.",
  "MBF-IMPORT-RECORD-REPEATED": "This record is repeated, so SWBPIPE cannot tell which one the file means.",
  "MBF-IMPORT-RECORD-UNUSED": "This record is not used by any element and is not imported.",
  "MBF-IMPORT-REFERENCE-TEMPERATURE-MISSING": "The file states no reference temperature, which thermal changes need.",
  "MBF-IMPORT-REFERENCE-UNRESOLVED": "This row refers to a record the file does not define.",
  "MBF-IMPORT-ROW-KEY-REPEATED": "This row repeats a key, so SWBPIPE cannot tell which value the file means.",
  "MBF-IMPORT-ROW-KEY-UNSUPPORTED": "This row key is outside the subset SWBPIPE imports.",
  "MBF-IMPORT-ROW-SYNTAX-INVALID": "This row is not in the form SWBPIPE reads.",
  "MBF-IMPORT-SECTION-AMBIGUOUS": "This section heading is ambiguous.",
  "MBF-IMPORT-SECTION-DELEGATED": "This section is recorded and left for later work; it is not applied.",
  "MBF-IMPORT-SECTION-FIELD-UNSUPPORTED": "This section field is outside the subset SWBPIPE imports.",
  "MBF-IMPORT-SECTION-MISSING": "A section SWBPIPE needs is missing from the file.",
  "MBF-IMPORT-SECTION-REPEATED": "This section appears more than once.",
  "MBF-IMPORT-SECTION-UNKNOWN": "This section is not one SWBPIPE reads.",
  "MBF-IMPORT-SECTION-UNSUPPORTED": "This section is outside the subset SWBPIPE imports.",
  "MBF-IMPORT-SECTION-ZERO-OMITTED": "This section holds only zero values and is left out.",
  "MBF-IMPORT-SHEAR-MODEL-INVALID": "The shear model given is not one SWBPIPE accepts.",
  "MBF-IMPORT-SIDECAR-AMBIGUOUS": "The identity file names one entity twice, so it is not applied.",
  "MBF-IMPORT-SIDECAR-CONFLICT": "The identity file disagrees with the file or with a declaration, so SWBPIPE does not import it.",
  "MBF-IMPORT-SIDECAR-DIGEST-MISMATCH": "The identity file's own digest does not verify, so SWBPIPE does not import it.",
  "MBF-IMPORT-SIDECAR-ENTRY-ABSENT": "The identity file has no entry for an entity in the file.",
  "MBF-IMPORT-SIDECAR-INVALID": "The identity file is not one SWBPIPE reads (its form or format), so SWBPIPE does not import it.",
  "MBF-IMPORT-SIDECAR-NO-SELF-DIGEST": "The identity file has no digest of itself.",
  "MBF-IMPORT-SIDECAR-NOT-APPLIED": "The identity file beside the MBF is for other bytes, so ids are formed from node numbers.",
  "MBF-IMPORT-SIDECAR-NOT-REPRODUCED": "The identity file does not match what SWBPIPE regenerates from the imported model, so it is refused.",
  "MBF-IMPORT-SUPPORT-UNSUPPORTED": "This support is outside the subset SWBPIPE imports.",
  "MBF-IMPORT-TAG-WITHOUT-SUPPORT": "This tag names no support and is not imported.",
  "MBF-IMPORT-TARGET-OPTION-DELEGATED": "This option is recorded, not applied.",
  "MBF-IMPORT-TBD-17-04-005": "This record's meaning is still open in SWBPIPE's import profile, so it is refused.",
  "MBF-IMPORT-TBD-17-04-007": "This record's meaning rests on an open profile question and is refused without an identity file.",
  "MBF-IMPORT-TBD-17-04-009": "This record's meaning is still open in SWBPIPE's import profile, so it is refused.",
  "MBF-IMPORT-TBD-17-04-010": "This record's meaning is still open in SWBPIPE's import profile, so it is refused.",
  "MBF-IMPORT-THERMAL-LOADS-UNSUPPORTED": "These thermal loads are outside the subset SWBPIPE imports.",
  "MBF-IMPORT-TOPOLOGY-EMPTY": "The file defines no elements to import.",
  "MBF-IMPORT-TOPOLOGY-LOOP": "The file's layout closes a loop, which SWBPIPE does not import.",
  "MBF-IMPORT-VALUE-OUT-OF-RANGE": "This value is outside the range SWBPIPE accepts.",
  "MBF-IMPORT-VALUE-UNREPRESENTABLE": "This value cannot be represented exactly in SWBPIPE's model.",
  "MBF-IMPORT-VERSION-MISSING": "The file states no version.",
  "MBF-IMPORT-VERSION-UNSUPPORTED": "The file's version is not one SWBPIPE reads.",
  // The native commands' codes (apps/desktop/src-tauri/src/mbf_import.rs).
  "MBF-IMPORT-EPOCH-INVALID": "The import did not start: its request was not current. Nothing was imported.",
  "MBF-IMPORT-EPOCH-EXHAUSTED": "Import isn't available until SWBPIPE restarts. Nothing was imported.",
  [MBF_IMPORT_CANCELLED]: "The import was cancelled. Nothing was imported and your model is unchanged.",
  "MBF-IMPORT-ARGS-INVALID": "The import request was not in the expected form. Nothing was imported.",
  "MBF-IMPORT-RETIREMENT-ID-INVALID": "The import could not be retired. Nothing was imported.",
  "MBF-OPEN-NOT-A-FILE": "The chosen name, or its identity file, is not a regular file (a link, folder or other kind). Nothing was imported.",
  // A designated source (§5.4) is refused with the same codes as a chosen file, at the path `source`.
  "MBF-OPEN-TOO-LARGE": "The chosen file, or its identity file, is too large to import (16 MiB for the MBF, 64 MiB for the identity file).",
  "MBF-OPEN-UNREADABLE": "The chosen file, or its identity file, could not be read. Nothing was imported.",
  "MBF-OPEN-NAME-INVALID": "The chosen file's name cannot be used (not UTF-8, or longer than 1 KiB).",
  "MBF-IMPORT-SOURCE-RETIRED": "The chosen file is no longer current. Choose it again.",
  "MBF-IMPORT-SOURCE-CHANGED": "The chosen file or its identity file changed since it was chosen. Choose it again.",
  "MBF-IMPORT-DECLARATION-INVALID": "The material declaration is not one SWBPIPE accepts.",
  "MBF-IMPORT-MEMORY-UNAVAILABLE": "The import's memory was not available. Nothing was imported; you can try again.",
  "MBF-IMPORT-WORKER-PROTOCOL": "The import did not complete. Nothing was imported.",
  "MBF-IMPORT-WORKER-REQUEST-INVALID": "The import request was not in the expected form. Nothing was imported.",
  "MBF-IMPORT-ENGINE-FAILED": "SWBPIPE's importer returned no result for this file. Nothing was imported.",
  "MBF-IMPORT-SCHEMA-NOT-CURRENT": "The imported model is not of the current model document version.",
  [MBF_IMPORT_REVIEW_STALE]: "The import changed since its review was shown. Review it again before adopting.",
  [MBF_IMPORT_REVIEW_TRUNCATED]: "The review could not list every entry, so the import cannot be adopted from the desktop.",
  [MBF_IMPORT_ASSUMPTION_UNAVAILABLE]: "An applied mapping assumption cannot be shown, so the import cannot be adopted.",
  [MBF_IMPORT_MODEL_TOO_LARGE]: "The imported model is larger than the desktop can adopt (64 MiB).",
  [MBF_IMPORT_NOT_ADOPTABLE]: "The review blocks adoption: see the refusals listed.",
  // The desktop's own.
  [MBF_IMPORT_NATIVE_ONLY]: "Importing a model batch file needs the desktop app.",
  [MBF_IMPORT_RETIREMENT_UNCONFIRMED]: "The import did not start: an earlier import could not be confirmed stopped. Nothing was imported.",
  [MBF_IMPORT_FAILED]: "The import did not complete. Nothing was imported.",
  [MBF_IMPORT_BUSY]: MBF_IMPORT_BUSY_MESSAGE,
  [INGRESS_RAW_BODY_REQUIRED]: "Import isn't available in this window. Nothing was imported and your model is unchanged. Reload the window or restart SWBPIPE to import again."
};

/** Why a designated source does not establish the identity set (§5.4; the engine's `DESIGNATION_REASONS`). Each states
 * what SWBPIPE checked; none states a fact about another program. */
export const MBF_IMPORT_DESIGNATION_REASONS: Readonly<Record<string, string>> = {
  identity_file_absent: "The MBF has no identity file beside it, so the file it was written from cannot be checked.",
  identity_file_not_format_6: "The identity file does not record the document it was written from (it is not format 6), so that document cannot be checked.",
  source_not_a_model_document: "The designated file is not a model document SWBPIPE reads.",
  source_digest_differs: "The designated document is not the one this MBF was written from.",
  rerender_refused: "SWBPIPE could not write the designated document again with the recorded export settings.",
  rerender_mbf_differs: "Writing the designated document again with the recorded export settings does not give this MBF's bytes.",
  rerender_identity_file_differs: "Writing the designated document again with the recorded export settings does not give this identity file's bytes.",
  import_not_bound_format_6: "This import is not bound to an identity file that records the designated document.",
  identity_inventory_malformed: "An entity in the designated document or in the import has no usable id.",
  identity_set_differs: "The ids differ in the classes listed.",
  identities_not_preserved: "The import does not preserve some original ids (see the loss report).",
  fields_not_recovered: "The import does not recover some fields or references (see the loss report).",
  review_truncated: "The review could not list every entry, so the outcome is not established."
};

/** A designation reason's product text; an unknown reason is named as such. */
export function designationReasonText(reason: string): string {
  return Object.hasOwn(MBF_IMPORT_DESIGNATION_REASONS, reason) ? MBF_IMPORT_DESIGNATION_REASONS[reason] : `SWBPIPE has no description for ${reason}.`;
}

/** A code's product text; an unknown code is named as such, never by the engine's message. */
export function importCodeText(code: string): string {
  return MBF_IMPORT_DESCRIPTIONS[code] ?? `SWBPIPE has no description for ${code}.`;
}

/** The structured assumption identifiers the engine records (`ASSUMPTION_IDS`) with their product descriptions, each
 * formed from the assumption's own parameters. An identifier without one makes the review unadoptable here. */
type Parameters = Record<string, string>;
export const MBF_IMPORT_ASSUMPTIONS: Readonly<Record<string, (p: Parameters) => string>> = {
  units_and_axes: (p) =>
    `Units: the file's ${p.unit_mode} unit mode (MBF version ${p.mbf_version}), angles in ${p.angles}; quantities keep the file's units. ${
      p.rotation === "z_to_y_exact"
        ? "Z is vertical in the file: every coordinate, offset, force, moment, displacement, rotation and restraint axis is turned exactly into SWBPIPE's Y-vertical axes, (X, Y, Z) to (X, Z, -Y)."
        : "Y is vertical, as in SWBPIPE."
    }`,
  offset_summation_exact: () => "Node positions are summed from the printed offsets as exact rationals and rounded to binary64 once per node.",
  english_units_reexport_normalized: (p) =>
    `US-customary units are kept; a later SI export converts them by exact definitions, with the reference temperature in ${p.reference_temperature_unit}.`,
  bare_metal_weight: (p) =>
    p.standard_gravity !== "none"
      ? `Weight density in ${p.density_unit} becomes weight per length in ${p.weight_unit} along global -Y using standard gravity ${p.standard_gravity} m/s²; the area is the bare-metal area, so weight acts on bare metal only.`
      : `Weight density in ${p.density_unit} becomes weight per length in ${p.weight_unit} along global -Y; the area is the bare-metal area, so weight acts on bare metal only.`,
  shear_modulus_derived_temperature_independent: (p) =>
    `The shear modulus is derived as ${p.relation} with E in Pa; materials are temperature-independent (the same E and expansion coefficient at every temperature).`,
  thermal_difference_zero_fields: (p) =>
    `Each element's thermal change is ${p.thermal_change}; a zero change adds no load. Section fields given as zero are left absent${
      p.corrosion_allowance_under_pressure === "true" ? ", except the corrosion allowance under pressure" : ""
    }.`,
  pressure_shear_model: (p) =>
    `Shear model ${p.shear_model} (${p.source === "caller" ? "as given" : "the exact pressure contract's default, as the file states none"}); curved bends stay ${p.curved_bends.replace(/_/g, "-")} under either model.`,
  pressure_regions: (p) =>
    `Pressure is kept in ${p.unit} as ${p.region_count} exact pressure region(s) under contract ${p.contract} with shear model ${p.shear_model}; regions are ${
      p.regions === "restored"
        ? "restored from the identity file, each checked whole"
        : "formed from maximal runs of equal pressure and bore, ending at tees, pressure or bore changes and non-collinear straight corners. Closure at each real end is declared by the caller; a corner transfers to the continuous wall unless an explicit node closure declaration overrides it"
    }. Pressure acts in the ${p.pressure_case} case; each section's corrosion allowance is the zero the file states.`,
  support_guide: (p) => `A guide uses the preceding element, falling back to the following element when none precedes it; it restrains the two global translations lateral to that axis-aligned element, rigidly or as springs in ${p.stiffness_unit}.`,
  support_limit_stop: (p) => `A two-way limit stop along an axis-aligned direction is a rigid line stop on that direction, or a spring in ${p.stiffness_unit}.`,
  support_skewed: (p) => `A skewed restraint along an axis-aligned direction is a two-way spring in ${p.stiffness_unit} on that direction.`,
  support_flexible_anchor: () => "A flexible anchor is a spring on each translation with a non-zero stiffness; a zero stiffness leaves that direction free.",
  spring_stiffness_unit: (p) => `Spring stiffnesses keep the file's unit, ${p.unit}.`,
  earth_gravity_weight: (p) => `Weights are imported as earth weights along ${p.direction}.`,
  binding_convention: (p) =>
    `${
      p.convention === "load_reference"
        ? "A load reference is carried until it changes"
        : p.convention === "force_without_flc"
          ? "A force without a load-case flag belongs to the sustained case"
          : "A moment without a load-case flag belongs to the sustained case"
    }; basis: ${p.basis === "bound" ? "the identity file's convention (bound import)" : `SWBPIPE's profile decision for MBF version ${p.mbf_version} (still open)`}.`,
  no_combinations_created: () => "No load combinations are created by the import."
};

/** One applied assumption's product description, or null when it has no representation here (blocks Adopt). */
export function assumptionText(entry: { id: ReviewField; parameters: Record<string, ReviewField> }): string | null {
  if (!fieldWhole(entry.id)) return null;
  const describe = Object.hasOwn(MBF_IMPORT_ASSUMPTIONS, entry.id) ? MBF_IMPORT_ASSUMPTIONS[entry.id] : undefined;
  if (!describe) return null;
  const parameters: Parameters = {};
  for (const [name, value] of Object.entries(entry.parameters)) {
    if (!fieldWhole(value)) return null;
    parameters[name] = value;
  }
  return describe(parameters);
}

// -- Adoption ---------------------------------------------------------------------------------------------------------

/** Version boundary before any format-specific interpretation. An older review (format 1's metrics, or format 2
 * without the designation members) cannot be shown or adopted as format 3. This is not a general payload validator. */
export function importReviewFormatCurrent(review: { document_kind: unknown; format: unknown }): boolean {
  return review.document_kind === "swbpipe.mbf_import_review" && review.format === 3;
}

/** The review's blocking codes as the dialog shows them, in a fixed order (empty when it may be adopted). */
export function importBlockingCodes(review: MbfImportReviewPayload): string[] {
  if (!importReviewFormatCurrent(review)) return ["MBF-IMPORT-WORKER-PROTOCOL"];
  const codes: string[] = [];
  if (review.status !== "imported" || review.blocks.document_check) codes.push(MBF_IMPORT_NOT_ADOPTABLE);
  if (review.completeness !== "complete") codes.push(MBF_IMPORT_REVIEW_TRUNCATED);
  const unshown = review.assumptions.some((entry) => assumptionText(entry) === null);
  if (review.blocks.assumption_unavailable || unshown) codes.push(MBF_IMPORT_ASSUMPTION_UNAVAILABLE);
  if (review.transfer_limit !== "within") codes.push(MBF_IMPORT_MODEL_TOO_LARGE);
  if (codes.length === 0 && !review.adoptable) codes.push(MBF_IMPORT_NOT_ADOPTABLE);
  return codes;
}

/** Whether a review may be adopted: adoptable, complete, within the transfer limit, every applied assumption shown,
 * and a candidate model digest. */
export function importAdoptable(review: MbfImportReviewPayload): boolean {
  return review.adoptable && importBlockingCodes(review).length === 0 && typeof review.model_sha256 === "string";
}

/** The declaration the form sends (§4.1 step 4): one entry per listed material it declares, never the engine's `*`;
 * `null` when it declares none. */
export function declarationFor(entries: ReadonlyArray<readonly [string, MbfDeclarationEntry | null]>): MbfDeclaration {
  const declaration: Record<string, MbfDeclarationEntry> = Object.create(null);
  for (const [name, entry] of entries) {
    if (name === "*" || entry === null) continue;
    if (entry.origin === "user_supplied" && entry.source_name.trim() === "") continue;
    declaration[name] = { origin: entry.origin, source_name: entry.source_name };
  }
  return Object.keys(declaration).length > 0 ? declaration : null;
}

/** A declaration's identity, to tell which declaration a review was formed for. */
export function declarationKey(declaration: MbfDeclaration): string {
  if (declaration === null) return "null";
  return JSON.stringify(Object.keys(declaration).sort().map((name) => [name, declaration[name].origin, declaration[name].source_name]));
}

/** The status line after a landed import (§4.1 step 6): the file, the binding and the identity counts of the review
 * the user adopted. */
export function importedStatusLine(capture: MbfAdoptCapture): string {
  const ids = capture.binding === "bound"
    ? `bound by its identity file; ${capture.restored} ids restored${capture.formedFromNames ? `, ${capture.formedFromNames} formed` : ""}`
    : `unbound; ids were formed from node numbers (${capture.formedFromNames})`;
  return `Imported model batch file ${capture.fileName} (${ids}).`;
}

/** The capture Adopt takes from a reviewed outcome (bounded fields only). */
export function adoptCapture(reviewed: Extract<MbfImportReviewOutcome, { outcome: "reviewed" }>, declaration: MbfDeclaration): MbfAdoptCapture | null {
  const { review } = reviewed;
  if (!importAdoptable(review) || review.model_sha256 === null) return null;
  const restored = review.identity.classes.reduce((sum, item) => sum + item.restored, 0);
  const formed = review.identity.classes.reduce((sum, item) => sum + item.formed_from_mbf_names, 0);
  return {
    sourceId: reviewed.sourceId,
    declaration,
    reviewSha256: reviewed.reviewSha256,
    modelSha256: review.model_sha256,
    fileName: reviewed.fileName,
    binding: review.binding.state,
    restored,
    formedFromNames: formed
  };
}

/** A failed invoke as a diagnostic: an ingress code it names, or the import's own failure. */
export function importFailureDiagnostic(error: unknown): MbfImportDiagnostic {
  const text = error instanceof Error ? error.message : String(error);
  const code = /\b(INGRESS-[A-Z-]+|MBF-[A-Z0-9-]+)\b/.exec(text)?.[1] ?? MBF_IMPORT_FAILED;
  return { code, path: "", message: importCodeText(code) };
}

/** Diagnostics as shown: each with its product text in place of the native message. */
export function productDiagnostics(diagnostics: readonly MbfImportDiagnostic[]): MbfImportDiagnostic[] {
  return diagnostics.map((item) => ({ code: item.code, path: item.path, message: importCodeText(item.code) }));
}
