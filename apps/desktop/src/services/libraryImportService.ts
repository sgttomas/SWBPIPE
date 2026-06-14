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

export async function validateLibraryImport(
  payload: Record<string, unknown>,
  libraryKind: LibraryKind,
  intendedVisibility: IntendedVisibility
): Promise<LibraryImportValidationRoute> {
  if (!isTauriRuntime()) {
    return { route: "unavailable_browser_preview", diagnostic: LIBRARY_IMPORT_BACKEND_DIAGNOSTIC };
  }
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
