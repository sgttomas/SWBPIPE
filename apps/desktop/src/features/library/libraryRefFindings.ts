// The store's findings on the model's library references (W-13 §3.5; W-17
// §2.7): MODEL-LIBRARY-REF-UNRESOLVED and MODEL-LIBRARY-REF-VERSION-CHANGED,
// both warnings. They are published by the project open and model-document
// open paths and by the Library Manager's list refresh, and read by the
// diagnostics list and the inspector's Provenance record section.
//
// A finding is shown only while the entity still carries the reference it was
// raised on (same library, record and recorded version), so a re-apply, an
// undo or a different project hides it without a fresh check. The recorded
// version is never updated by any of this.
import { useSyncExternalStore } from "react";
import { modelLibraryRefs, type LibraryRefFinding } from "../../services/libraryImportService";
import type { Diagnostic, PreviewModel } from "../../types";

export const LIBRARY_REF_CHECK_SOURCE = "apps/desktop/src-tauri/src/lib.rs check_model_library_refs";

let published: { projectId: string; findings: LibraryRefFinding[] } = { projectId: "", findings: [] };
const listeners = new Set<() => void>();

export function publishLibraryRefFindings(projectId: string, findings: LibraryRefFinding[]): void {
  published = { projectId, findings: [...findings] };
  for (const listener of listeners) listener();
}

function subscribe(listener: () => void): () => void {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

export function usePublishedLibraryRefFindings() {
  return useSyncExternalStore(subscribe, () => published, () => published);
}

/** The published findings that still apply to `model`: the entity still
 * carries, at any depth, the reference the finding was raised on (the open
 * check's own walk, `modelLibraryRefs`). */
export function currentLibraryRefFindings(model: PreviewModel, state: typeof published = published): LibraryRefFinding[] {
  if (state.findings.length === 0 || state.projectId !== model.project?.id) return [];
  const references = modelLibraryRefs(model);
  return state.findings.filter((finding) =>
    references.some((reference) => reference.entity_ref === finding.entity_ref
      && reference.library_kind === finding.library_kind && reference.library_id === finding.library_id
      && (reference.record_id ?? null) === (finding.record_id ?? null)
      && (reference.library_version ?? null) === (finding.recorded_version ?? null)));
}

/** Entries of the diagnostics list, keyed to the entity. */
export function libraryRefDiagnostics(findings: LibraryRefFinding[]): Diagnostic[] {
  return findings.map((finding) => ({
    id: `library-ref:${finding.code}:${finding.entity_ref}`,
    code: finding.code,
    severity: "warning",
    message: finding.message,
    affected_refs: [finding.entity_ref],
    source: LIBRARY_REF_CHECK_SOURCE
  }));
}
