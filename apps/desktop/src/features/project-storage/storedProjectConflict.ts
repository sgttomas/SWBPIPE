// HELP_HUMAN ruling (2026-10-10): the app never silently replaces a stored
// project. A save onto a stored `project.id` asks: Replace (names the stored
// project and when it was last saved), Save as a new project (a new project id;
// the document is otherwise unchanged), or Cancel, the default.
import type { StoredProjectEntry } from "../../services/projectService";
import type { LocalProjectEnvelope, LocalProjectIndexEntry, PreviewModel } from "../../types";

export type ReplaceProjectChoice = "replace" | "save-as-new" | "cancel";

export type ReplaceProjectPromptState = {
  kind: "create" | "save";
  existing: StoredProjectEntry;
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

/** W-12 §2, §3: a Replace keeps the project id's saved model states and recorded
 * runs; the prompt says how many of each. */
export function savedStatesSentence(existing: StoredProjectEntry): string {
  const count = existing.saved_state_count;
  if (count === undefined) return "";
  const states = `${count} saved model state${count === 1 ? "" : "s"}`;
  const runs = existing.recorded_run_count;
  const history = runs === undefined ? states : `${states} and ${runs} recorded run${runs === 1 ? "" : "s"}`;
  return ` Replacing it keeps the ${history} attached to project id ${existing.project_id}.`;
}

export function keptStoredProjectMessage(existing: LocalProjectIndexEntry): string {
  return `Not saved: the stored project ${storedProjectDescription(existing)} was kept.`;
}
