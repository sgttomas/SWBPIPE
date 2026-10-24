// The unsaved-changes guard (W-12 S1; Agent 0, 2026-10-24): every command that
// replaces the working model (Open Local Project, a project in the index,
// Open Model Document…, New Blank and the blank load/reference model, and Open
// State) asks first while the model has edits no save holds. One prompt, one
// notion of "edited" (the session's `modelEdited`), one choice set:
// - Cancel, the default: nothing is replaced and the edits stay;
// - Save Local Project first: Save Local Project's own flow (with its replace
//   prompt) runs, and the command goes ahead only if that save landed;
// - Discard: the command goes ahead and the edits are dropped.
// Called only by the session (`useWorkspaceSession`).
import { useRef, useState } from "react";
import type { MutableRefObject } from "react";

export type UnsavedChangesChoice = "save" | "discard" | "cancel";

/** What the prompt asks about: what the command does, and its verb for the buttons. */
export type UnsavedChangesRequest = {
  /** "Opening local project `project:x`" — the sentence's subject. */
  action: string;
  /** "open it" / "create it". */
  verb: string;
};

export type UnsavedChangesPromptState = UnsavedChangesRequest & {
  /** Where focus was when the prompt opened; it returns there on close. */
  opener: HTMLElement | null;
  choose: (choice: UnsavedChangesChoice) => void;
};

/** `proceed`: no edits, Discard, or Save first and the save landed.
 * `cancelled`: Cancel. `not-saved`: Save first did not land. `busy`: another
 * project operation or prompt owns the session, so no prompt was shown. */
export type UnsavedChangesOutcome = "proceed" | "cancelled" | "not-saved" | "busy";

export const PROJECT_SESSION_BUSY = "another project operation or prompt is in progress; nothing was replaced.";

export function unsavedChangesRefusal(outcome: Exclude<UnsavedChangesOutcome, "proceed">, refused: string): string {
  if (outcome === "busy") return `${refused}: ${PROJECT_SESSION_BUSY}`;
  return outcome === "cancelled"
    ? `${refused}: the working model and its unsaved edits were kept.`
    : `${refused}: the local project was not saved, so the working model and its unsaved edits were kept.`;
}

export function useUnsavedChangesGuard(deps: {
  /** The session's `modelEdited`, read when a command starts. */
  modelEditedRef: MutableRefObject<boolean>;
  /** Takes the shared modal hold while the prompt is open (`modalHolds.ts`). */
  holdModal: () => () => void;
  /** True while another project operation or prompt owns the session: the
   * guard then shows no prompt (one prompt at a time). */
  projectSessionBusy: () => boolean;
  /** Save Local Project's flow; true when the save landed. */
  saveLocalProject: () => Promise<boolean>;
}) {
  const [unsavedChangesPrompt, setUnsavedChangesPrompt] = useState<UnsavedChangesPromptState | null>(null);
  const depsRef = useRef(deps);
  depsRef.current = deps;

  /** True when a model-replacing command must ask first. Read synchronously,
   * so a command without edits to protect starts exactly as it did before. */
  function replacingNeedsConfirmation(): boolean {
    return depsRef.current.modelEditedRef.current;
  }

  async function confirmReplacingEdits(request: UnsavedChangesRequest): Promise<UnsavedChangesOutcome> {
    if (!replacingNeedsConfirmation()) return "proceed";
    if (depsRef.current.projectSessionBusy()) return "busy";
    const choice = await new Promise<UnsavedChangesChoice>((resolve) => {
      const release = depsRef.current.holdModal();
      // Captured before the workspace turns inert, which drops its focus.
      const opener = document.activeElement instanceof HTMLElement ? document.activeElement : null;
      setUnsavedChangesPrompt({
        ...request,
        opener,
        choose: (picked) => {
          release();
          setUnsavedChangesPrompt(null);
          resolve(picked);
        }
      });
    });
    if (choice === "cancel") return "cancelled";
    if (choice === "discard") return "proceed";
    return (await depsRef.current.saveLocalProject()) ? "proceed" : "not-saved";
  }

  return { unsavedChangesPrompt, replacingNeedsConfirmation, confirmReplacingEdits };
}
