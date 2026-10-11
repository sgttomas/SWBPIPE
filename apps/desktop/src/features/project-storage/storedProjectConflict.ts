// HELP_HUMAN ruling (2026-10-10): the app never silently replaces a stored
// project. A save onto a stored `project.id` asks: Replace (names the stored
// project and when it was last saved), Save as a new project (a new project id;
// the document is otherwise unchanged), or Cancel, the default.
import type { LocalProjectEnvelope, LocalProjectIndexEntry, PreviewModel } from "../../types";

export type ReplaceProjectChoice = "replace" | "save-as-new" | "cancel";

export type ReplaceProjectPromptState = {
  kind: "create" | "save";
  existing: LocalProjectIndexEntry;
  /** The session's own project, saved elsewhere since this session opened or saved it. */
  stale: boolean;
  /** Where focus was when the prompt opened; it returns there on close. */
  opener: HTMLElement | null;
  choose: (choice: ReplaceProjectChoice) => void;
};

export type StoredProjectWrite =
  | { outcome: "written"; envelope: LocalProjectEnvelope }
  | { outcome: "save-as-new"; model: PreviewModel }
  | { outcome: "kept"; existing: LocalProjectIndexEntry };

export function lastSavedText(updatedAtUnix: number): string {
  return new Date(updatedAtUnix * 1000).toLocaleString();
}

export function storedProjectDescription(existing: LocalProjectIndexEntry): string {
  return `"${existing.project_name}" (${existing.project_id}), last saved ${lastSavedText(existing.updated_at_unix)}`;
}

export function keptStoredProjectMessage(existing: LocalProjectIndexEntry): string {
  return `Not saved: the stored project ${storedProjectDescription(existing)} was kept.`;
}
