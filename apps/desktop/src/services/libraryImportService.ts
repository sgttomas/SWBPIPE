import { invoke } from "@tauri-apps/api/core";
import { invokeWithIngress } from "./nativeIngress";

// Library-import provenance seam (Phase C3, TP-C3-IMPORTCMD-001). Validation of
// an already-parsed material/section/component import payload routes through the
// desktop (Tauri) command `validate_library_import`, backed by the runtime
// library-import provenance crate (the DEL-03-07 port). Browser preview mode
// reports an explicit unavailable route instead of synthesizing a TypeScript
// fallback validator. Imported private libraries are never transmitted or
// committed (OPS-K-PRIV-1, PRD §13.5, §17.3). Every status is a software
// finding only (docs/claims_registry.md, DEC-081).

export const LIBRARY_IMPORT_BACKEND_DIAGNOSTIC =
  "LIBRARY-IMPORT-BACKEND-DESKTOP-ONLY: library-import provenance validation runs through the " +
  "desktop (Tauri) backend command validate_library_import backed by core/library_import; " +
  "browser preview mode does not synthesize a fallback validator.";

export type LibraryKind = "material" | "section" | "component" | "hanger";

export type IntendedVisibility = "public" | "private";

// Mirrors core/library_import/library_import_document::ImportFinding. `severity`
// is the PRD §13.5 blocking-vs-advisory axis: `blocking`/`quarantine` block the
// import; `review_required` is advisory (human review before acceptance).
export type LibraryImportFinding = {
  code: string;
  severity: "blocking" | "quarantine" | "review_required";
  path: string;
  message: string;
  remediation: string;
};

export type LibraryImportOutcome =
  | "QUARANTINE"
  | "REJECTED"
  | "REVIEW_REQUIRED"
  | "PRIVATE_LOCAL_ONLY"
  | "ACCEPTED_PUBLIC";

// Mirrors the `validate_library_import` command envelope. `diagnostics` is the
// PKG-02 import-boundary projection of the same findings.
export type LibraryImportValidation = {
  document_kind: string;
  outcome: LibraryImportOutcome;
  library_kind: LibraryKind;
  intended_visibility: IntendedVisibility;
  accepted: boolean;
  has_blocking_findings: boolean;
  findings: LibraryImportFinding[];
  diagnostics: Record<string, unknown>[];
  professional_boundary_notice: string;
};

export type LibraryImportValidationRoute =
  | { route: "tauri_backend"; validation: LibraryImportValidation }
  | { route: "unavailable_browser_preview"; diagnostic: string };

function isTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

function libraryImportUnavailable(): {
  route: "unavailable_browser_preview";
  diagnostic: string;
} {
  return { route: "unavailable_browser_preview", diagnostic: LIBRARY_IMPORT_BACKEND_DIAGNOSTIC };
}

export async function validateLibraryImport(
  payload: Record<string, unknown>,
  libraryKind: LibraryKind,
  intendedVisibility: IntendedVisibility
): Promise<LibraryImportValidationRoute> {
  if (!isTauriRuntime()) return libraryImportUnavailable();
  const validation = await invokeWithIngress<LibraryImportValidation>("validate_library_import", {
    payload,
    libraryKind,
    intendedVisibility
  });
  return { route: "tauri_backend", validation };
}

// Split findings along the PRD §13.5 blocking-vs-advisory axis once, at the
// seam, so downstream surfaces do not re-derive severity semantics. `blocking`
// holds `blocking`/`quarantine` (the import cannot be accepted as-is);
// `advisory` holds `review_required` (human review before acceptance).
export function partitionLibraryImportFindings(validation: LibraryImportValidation): {
  blocking: LibraryImportFinding[];
  advisory: LibraryImportFinding[];
} {
  const blocking: LibraryImportFinding[] = [];
  const advisory: LibraryImportFinding[] = [];
  for (const finding of validation.findings) {
    if (finding.severity === "blocking" || finding.severity === "quarantine") {
      blocking.push(finding);
    } else {
      advisory.push(finding);
    }
  }
  return { blocking, advisory };
}

// --- Local-only private-library persistence (Phase C3, TP-C3-LIBSTORE-001) ---
// Imported private libraries persist in the local SQLite store only, via the
// desktop backend; browser preview reports the unavailable route. The store is
// private-by-default and never transmitted or committed (OPS-K-PRIV-1, PRD
// §13.5/§17.3). Save only persists an accepted import — a blocked/quarantined
// import returns stored:false with its findings so the caller can surface them.

export type LocalLibraryIndexEntry = {
  project_id: string;
  library_kind: LibraryKind;
  library_id: string;
  library_name: string;
  privacy_class: string;
  storage_mode: string;
  created_at_unix: number;
  updated_at_unix: number;
};

export type LocalLibraryEnvelope = {
  project_id: string;
  library_kind: LibraryKind;
  library_id: string;
  storage_mode: string;
  created_at_unix: number;
  updated_at_unix: number;
  document: Record<string, unknown>;
  validation: LibraryImportValidation;
  message: string;
};

export type LocalLibrarySaveResult = {
  project_id: string;
  library_kind: LibraryKind;
  library_id: string;
  stored: boolean;
  storage_mode: string;
  created_at_unix: number | null;
  updated_at_unix: number | null;
  document: Record<string, unknown>;
  validation: LibraryImportValidation;
  message: string;
};

export type LocalLibraryDeleteReceipt = {
  project_id: string;
  library_kind: LibraryKind;
  library_id: string;
  deleted: boolean;
  message: string;
};

type LibraryUnavailable = { route: "unavailable_browser_preview"; diagnostic: string };

export type LibrarySaveRoute =
  | { route: "tauri_backend"; result: LocalLibrarySaveResult }
  | LibraryUnavailable;

export type LibraryOpenRoute =
  | { route: "tauri_backend"; envelope: LocalLibraryEnvelope | null }
  | LibraryUnavailable;

export type LibraryListRoute =
  | { route: "tauri_backend"; entries: LocalLibraryIndexEntry[] }
  | LibraryUnavailable;

export type LibraryDeleteRoute =
  | { route: "tauri_backend"; receipt: LocalLibraryDeleteReceipt }
  | LibraryUnavailable;

export async function saveLocalLibrary(
  projectId: string,
  libraryKind: LibraryKind,
  document: Record<string, unknown>
): Promise<LibrarySaveRoute> {
  if (!isTauriRuntime()) return libraryImportUnavailable();
  const result = await invokeWithIngress<LocalLibrarySaveResult>("save_local_library", {
    projectId,
    libraryKind,
    document
  });
  return { route: "tauri_backend", result };
}

export async function openLocalLibrary(
  projectId: string,
  libraryKind: LibraryKind,
  libraryId: string
): Promise<LibraryOpenRoute> {
  if (!isTauriRuntime()) return libraryImportUnavailable();
  const envelope = await invoke<LocalLibraryEnvelope | null>("open_local_library", {
    projectId,
    libraryKind,
    libraryId
  });
  return { route: "tauri_backend", envelope };
}

export async function listLocalLibraries(projectId: string | null): Promise<LibraryListRoute> {
  if (!isTauriRuntime()) return libraryImportUnavailable();
  const entries = await invoke<LocalLibraryIndexEntry[]>("list_local_libraries", {
    projectId
  });
  return { route: "tauri_backend", entries };
}

export async function deleteLocalLibrary(
  projectId: string,
  libraryKind: LibraryKind,
  libraryId: string
): Promise<LibraryDeleteRoute> {
  if (!isTauriRuntime()) return libraryImportUnavailable();
  const receipt = await invoke<LocalLibraryDeleteReceipt>("delete_local_library", {
    projectId,
    libraryKind,
    libraryId
  });
  return { route: "tauri_backend", receipt };
}

// --- Built-in invented starting template (Phase C3, TP-C3-LIBGUI-001) ---
// The import manager GUI needs a clean starting point the user edits, mirroring
// the C2 rule-pack manager's buildDraftRulePackDocument helper. This template
// mirrors the canonical accepted private payload the desktop backend test pins
// (`storable_material_payload`): a private-classified library object carrying
// every required provenance field and an empty record array. Under a *private*
// import it validates to PRIVATE_LOCAL_ONLY with no findings and is storable —
// the happy-path the user then edits. It is invented and non-engineering: it
// carries no real material/section/component values and is never a real
// dataset, redistribution clearance, or compliance claim.

const TEMPLATE_PROVENANCE = {
  source_name: "Invented local draft",
  source_location: "user-authored private draft",
  source_license: "private user basis",
  contributor: "SWBPIPE user",
  contributor_certification: "invented non-engineering draft; not for project reliance",
  redistribution_status: "private_only",
  review_status: "accepted"
} as const;

const LIBRARY_TEMPLATE_KEYS: Record<
  LibraryKind,
  { libraryKey: string; recordsKey: string; libraryId: string }
> = {
  material: {
    libraryKey: "material_library",
    recordsKey: "material_records",
    libraryId: "matlib.invented.local_draft"
  },
  section: {
    libraryKey: "section_library",
    recordsKey: "section_records",
    libraryId: "seclib.invented.local_draft"
  },
  hanger: {
    libraryKey: "hanger_library",
    recordsKey: "hanger_records",
    libraryId: "hangerlib.invented.local_draft"
  },
  component: {
    libraryKey: "component_library",
    recordsKey: "component_records",
    libraryId: "complib.invented.local_draft"
  }
};

export function buildInventedLibraryImportTemplate(
  libraryKind: LibraryKind
): Record<string, unknown> {
  const keys = LIBRARY_TEMPLATE_KEYS[libraryKind];
  return {
    schema_version: libraryKind === "hanger" ? "1.0.0" : "0.1.0",
    [keys.libraryKey]: {
      library_id: keys.libraryId,
      name: `Invented private ${libraryKind} library (local draft)`,
      privacy_class: libraryKind === "hanger" ? "private_user_supplied" : "private_user_data",
      provenance: { ...TEMPLATE_PROVENANCE }
    },
    [keys.recordsKey]: []
  };
}

// --- Library record/slot resolution preview (Phase C3, TP-C3-LIBREFPICKER-001) ---
// A rule-pack `private_library_value` input authors a `library_value_ref`
// (library_kind/library_id/record_id/slot_id; DeclarationsEditor, the run-time
// resolution in the desktop `run_rule_checks` wrapper). The run panel previously
// surfaced that reference read-only, so a user could not tell from the panel
// whether it would resolve in their local store, nor discover the valid
// record/slot ids to author. These pure helpers index a stored library document
// so the panel can preview resolution and browse records/slots WITHOUT mutating
// the pack, overriding the authored reference, or rendering the private value
// (IP boundary: the value is read only at run time, never embedded or shown).
//
// The per-kind dispatch MUST mirror the desktop resolver
// (`src-tauri` extract_library_slot_value / find_library_slot_value): which
// member holds the records, which field is the record id, and which
// `(slot_array, slot_id_field)` pairs hold the value slots (scanned in order).
const LIBRARY_RECORD_SLOT_SHAPE: Record<
  LibraryKind,
  { recordsMember: string; recordIdKey: string; slotArrays: { array: string; slotIdKey: string }[] }
> = {
  material: {
    recordsMember: "material_records",
    recordIdKey: "material_id",
    slotArrays: [{ array: "allowables", slotIdKey: "allowable_id" }]
  },
  section: {
    recordsMember: "section_records",
    recordIdKey: "section_id",
    slotArrays: [
      { array: "dimensions", slotIdKey: "dimension_id" },
      { array: "properties", slotIdKey: "property_id" }
    ]
  },
  // Hangers have typed physical inputs, not rule-pack value slots.
  hanger: { recordsMember: "hanger_records", recordIdKey: "hanger_id", slotArrays: [] },
  component: {
    recordsMember: "component_records",
    recordIdKey: "component_id",
    slotArrays: [{ array: "fields", slotIdKey: "field_id" }]
  }
};

export type LibraryRecordIndex = {
  record_id: string;
  slot_ids: string[];
};

// How an authored library_value_ref relates to what is actually in the local
// store: `resolves` when both the record and slot exist; `record_missing` when
// no record matches; `slot_missing` when the record exists but the slot does not;
// `library_missing` when the referenced library is not in the store at all
// (decided by the panel from the store list / a null open, not here).
export type LibraryReferenceResolution =
  | "resolves"
  | "record_missing"
  | "slot_missing"
  | "library_missing";

function asObjectArray(value: unknown): Record<string, unknown>[] {
  return Array.isArray(value)
    ? value.filter((item): item is Record<string, unknown> => typeof item === "object" && item !== null)
    : [];
}

function readId(item: Record<string, unknown>, key: string): string | null {
  const value = item[key];
  return typeof value === "string" && value.length > 0 ? value : null;
}

// Pure: extract the record ids and, per record, the value-slot ids from a stored
// library `document` for `libraryKind`, mirroring the desktop resolver's per-kind
// dispatch. Slot ids are gathered across every slot array for the kind (the
// resolver scans them in order) into one de-duplicated, first-seen-order list,
// which is exactly the membership a slot-existence check needs. Records or slots
// without a string id, and non-object shapes, are skipped (never invented).
export function indexLibraryRecordsSlots(
  document: Record<string, unknown> | null | undefined,
  libraryKind: LibraryKind
): LibraryRecordIndex[] {
  if (!document) return [];
  const shape = LIBRARY_RECORD_SLOT_SHAPE[libraryKind];
  const records: LibraryRecordIndex[] = [];
  for (const record of asObjectArray(document[shape.recordsMember])) {
    const recordId = readId(record, shape.recordIdKey);
    if (!recordId) continue;
    const slotIds: string[] = [];
    for (const { array, slotIdKey } of shape.slotArrays) {
      for (const slot of asObjectArray(record[array])) {
        const slotId = readId(slot, slotIdKey);
        if (slotId && !slotIds.includes(slotId)) slotIds.push(slotId);
      }
    }
    records.push({ record_id: recordId, slot_ids: slotIds });
  }
  return records;
}

// Pure: classify an authored `(recordId, slotId)` against an indexed library's
// records. Library-presence is decided upstream (store list / null open), so
// this only distinguishes record_missing / slot_missing / resolves.
export function classifyLibraryReference(
  records: LibraryRecordIndex[],
  recordId: string,
  slotId: string
): "resolves" | "record_missing" | "slot_missing" {
  const record = records.find((entry) => entry.record_id === recordId);
  if (!record) return "record_missing";
  return record.slot_ids.includes(slotId) ? "resolves" : "slot_missing";
}

// --- Model library references (W-13 §3.5; W-17 §2.7) ---
// Each `library_ref` of the model is compared with the project's local store
// at project open and load and on library-list refresh:
// MODEL-LIBRARY-REF-UNRESOLVED when the store holds no such library or record,
// MODEL-LIBRARY-REF-VERSION-CHANGED when a stored record's digest differs
// from the recorded `library_version`. Both are warnings; the recorded version
// is never updated by the check. Only the references travel to the backend.
//
// The check applies the save screen's rules (W-6a, `modelDocumentSave.ts`,
// `src-tauri/src/model_document_save.rs`): the references are found by the one
// walk below, which the save screen also uses (every `library_ref` object at
// any depth); the backend trims the identities and reads every matching
// stored record (`library_identity`, `matching_library_records`), never the
// first. The native save's walk is pinned to this one by
// `fixtures/model_document/library_ref_sites.json`.

export const MODEL_LIBRARY_REF_UNRESOLVED = "MODEL-LIBRARY-REF-UNRESOLVED";
export const MODEL_LIBRARY_REF_VERSION_CHANGED = "MODEL-LIBRARY-REF-VERSION-CHANGED";

export type ModelLibraryRef = {
  entity_ref: string;
  library_kind: LibraryKind;
  library_id: string;
  library_version?: string;
  record_id?: string;
};

export type LibraryRefFinding = ModelLibraryRef & {
  code: typeof MODEL_LIBRARY_REF_UNRESOLVED | typeof MODEL_LIBRARY_REF_VERSION_CHANGED;
  severity: "warning";
  recorded_version: string | null;
  stored_version: string | null;
  message: string;
};

/** One `library_ref` object of a model: its path, the id of its nearest
 * enclosing object that has a non-empty string `id` (else the reference's own
 * path), and the reference as recorded. */
export type LibraryRefSite = { path: string; entity_ref: string; reference: Record<string, unknown> };

function isRecordObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/** Every `library_ref` object of a model at any depth, in document order. The
 * one walk of the open-time store check and the save screen. */
export function libraryRefSites(model: unknown): LibraryRefSite[] {
  const found: LibraryRefSite[] = [];
  const walk = (value: unknown, path: string, owner: string | null) => {
    if (Array.isArray(value)) { value.forEach((entry, index) => walk(entry, `${path}[${index}]`, owner)); return; }
    if (!isRecordObject(value)) return;
    const own = typeof value.id === "string" && value.id.length > 0 ? value.id : null;
    const named = own ?? owner;
    for (const [key, child] of Object.entries(value)) {
      const at = path ? `${path}.${key}` : key;
      if (key === "library_ref" && isRecordObject(child)) {
        found.push({ path: at, entity_ref: named ?? at, reference: child });
      }
      walk(child, at, named);
    }
  };
  walk(model, "", null);
  return found;
}

/** One `library_ref` member of a model, whatever its value: its JSON pointer and the value. */
export type LibraryRefMemberSite = { pointer: string; value: unknown };

function pointerToken(key: string): string {
  return key.replaceAll("~", "~0").replaceAll("/", "~1");
}

/** Every member named `library_ref` at any depth, in document order, whatever its value:
 * an object, `null`, a string, a number, a boolean or an array (a positional record among
 * them). W-18b's library rule 1 (DEL-08-01 Design `report-user-release.md` §2.7), the
 * webview's early check; the native detector is `export_disposition_screen::library_ref_member_sites`,
 * both pinned to `fixtures/model_document/library_ref_member_sites.json`. `libraryRefSites`
 * above keeps only the object members and is unchanged. */
export function libraryRefMemberSites(model: unknown): LibraryRefMemberSite[] {
  const found: LibraryRefMemberSite[] = [];
  const walk = (value: unknown, pointer: string) => {
    if (Array.isArray(value)) { value.forEach((entry, index) => walk(entry, `${pointer}/${index}`)); return; }
    if (!isRecordObject(value)) return;
    for (const [key, child] of Object.entries(value)) {
      const at = `${pointer}/${pointerToken(key)}`;
      if (key === "library_ref") found.push({ pointer: at, value: child });
      walk(child, at);
    }
  };
  walk(model, "");
  return found;
}

/** The references the open-time store check sends, as the model records them
 * (the backend normalizes). A non-string kind or library id is sent empty, so
 * it reads as unresolved rather than failing the check. */
export function modelLibraryRefs(model: unknown): ModelLibraryRef[] {
  const textOf = (value: unknown) => (typeof value === "string" ? value : "");
  return libraryRefSites(model).map(({ entity_ref, reference }) => ({
    entity_ref,
    library_kind: textOf(reference.library_kind) as LibraryKind,
    library_id: textOf(reference.library_id),
    ...(typeof reference.library_version === "string" ? { library_version: reference.library_version } : {}),
    ...(typeof reference.record_id === "string" ? { record_id: reference.record_id } : {})
  }));
}

export type LibraryRefCheckRoute =
  | { route: "tauri_backend"; findings: LibraryRefFinding[] }
  | { route: "unavailable_browser_preview"; diagnostic: string };

export async function checkModelLibraryRefs(projectId: string, references: ModelLibraryRef[]): Promise<LibraryRefCheckRoute> {
  if (!isTauriRuntime()) return libraryImportUnavailable();
  if (references.length === 0) return { route: "tauri_backend", findings: [] };
  const findings = await invoke<LibraryRefFinding[]>("check_model_library_refs", { projectId, references });
  return { route: "tauri_backend", findings };
}
