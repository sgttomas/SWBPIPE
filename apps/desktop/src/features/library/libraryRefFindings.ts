// The store's findings on the model's library references (W-13 §3.5; W-17
// §2.7): MODEL-LIBRARY-REF-UNRESOLVED and MODEL-LIBRARY-REF-VERSION-CHANGED,
// both warnings. They are published by the project open and model-document
// open paths and by the Library Manager's list refresh, and read by the
// diagnostics list and the inspector's Provenance record section.
//
// A finding is shown only while the entity still carries the reference it was
// raised on (same library, record and recorded version), so a re-apply, an
// undo or a different project hides it without a fresh check. Each reference
// the model carries now keeps at most one published finding: where an entity
// carried the same reference twice and one is removed, one of the two findings
// goes with it. The recorded version is never updated by any of this.
//
// DEL-08-01 REQ-022: the findings that apply to a report's model are a report input. The
// workspace session bridges this store into its report source sequence
// (`workspaceSession.ts`: a subscription for its lifetime that advances the sequence when
// a publication changes the findings applicable to the current model, including a clear);
// a report captures the session's snapshot with its other inputs and never rereads this
// store while it is prepared.
import { useSyncExternalStore } from "react";
import { modelLibraryRefs, type LibraryRefFinding } from "../../services/libraryImportService";
import type { Diagnostic, PreviewModel } from "../../types";

export const LIBRARY_REF_CHECK_SOURCE = "apps/desktop/src-tauri/src/lib.rs check_model_library_refs";

/** One publication: immutable once published (a new publication replaces it). */
export type LibraryRefFindingsState = Readonly<{ projectId: string; findings: readonly LibraryRefFinding[] }>;

export const NO_LIBRARY_REF_FINDINGS: LibraryRefFindingsState = Object.freeze({ projectId: "", findings: Object.freeze([]) as readonly LibraryRefFinding[] });

let published: LibraryRefFindingsState = NO_LIBRARY_REF_FINDINGS;
const listeners = new Set<() => void>();

/** Freeze the owned copy, including any nested JSON values added to findings later. */
function freezeFindingValue(value: unknown): void {
  if (value === null || typeof value !== "object" || Object.isFrozen(value)) return;
  Object.freeze(value);
  for (const member of Object.values(value)) freezeFindingValue(member);
}

export function publishLibraryRefFindings(projectId: string, findings: LibraryRefFinding[]): void {
  // Neither a publisher's retained record nor a consumer may change this capture.
  const snapshot = structuredClone(findings);
  freezeFindingValue(snapshot);
  published = Object.freeze({ projectId, findings: snapshot });
  for (const listener of listeners) listener();
}

/** The current publication. */
export function publishedLibraryRefFindings(): LibraryRefFindingsState {
  return published;
}

/** Listens to publications; listeners run synchronously within `publishLibraryRefFindings`. */
export function subscribeLibraryRefFindings(listener: () => void): () => void {
  listeners.add(listener);
  return () => {
    listeners.delete(listener);
  };
}

const subscribe = subscribeLibraryRefFindings;

export function usePublishedLibraryRefFindings() {
  return useSyncExternalStore(subscribe, () => published, () => published);
}

/** The published findings that still apply to `model`: the entity still
 * carries, at any depth, the reference the finding was raised on (the open
 * check's own walk, `modelLibraryRefs`). The findings are matched to the
 * model's references as a multiset, in published order: a reference keeps at
 * most one finding, so a finding whose reference was removed is dropped even
 * while the entity carries another, identical reference. That is the rule the
 * native report admission applies to the Diagnostics a report appends
 * (`report_session_diagnostics.rs`: at most one finding per reference site). */
export function currentLibraryRefFindings(model: PreviewModel, state: LibraryRefFindingsState = published): LibraryRefFinding[] {
  if (state.findings.length === 0 || state.projectId !== model.project?.id) return [];
  const references = modelLibraryRefs(model);
  const taken = references.map(() => false);
  return state.findings.filter((finding) => {
    const at = references.findIndex((reference, index) => !taken[index]
      && reference.entity_ref === finding.entity_ref
      && reference.library_kind === finding.library_kind && reference.library_id === finding.library_id
      && (reference.record_id ?? null) === (finding.record_id ?? null)
      && (reference.library_version ?? null) === (finding.recorded_version ?? null));
    if (at < 0) return false;
    taken[at] = true;
    return true;
  });
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
