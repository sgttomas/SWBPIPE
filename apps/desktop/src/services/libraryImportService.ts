import { invoke } from "@tauri-apps/api/core";

// Library-import provenance seam (Phase C3, TP-C3-IMPORTCMD-001). Validation of
// an already-parsed material/section/component import payload routes through the
// desktop (Tauri) command `validate_library_import`, backed by the runtime
// library-import provenance crate (the DEL-03-07 port). Browser preview mode
// reports an explicit unavailable route instead of synthesizing a TypeScript
// fallback validator. Imported private libraries are never transmitted or
// committed (OPS-K-PRIV-1, PRD §13.5, §17.3). Every status is a software
// finding only — never a legal, certification, sealing, authentication, or
// code-compliance determination.

export const LIBRARY_IMPORT_BACKEND_DIAGNOSTIC =
  "LIBRARY-IMPORT-BACKEND-DESKTOP-ONLY: library-import provenance validation runs through the " +
  "desktop (Tauri) backend command validate_library_import backed by core/library_import; " +
  "browser preview mode does not synthesize a fallback validator.";

export type LibraryKind = "material" | "section" | "component";

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
  const validation = await invoke<LibraryImportValidation>("validate_library_import", {
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
  const result = await invoke<LocalLibrarySaveResult>("save_local_library", {
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
