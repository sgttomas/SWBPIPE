import { useLayoutEffect, useRef } from "react";
import { createPortal } from "react-dom";
import type { UnsavedChangesPromptState } from "./unsavedChangesGuard";

/** Asks before a command replaces a working model that has unsaved edits
 * (`unsavedChangesGuard.ts`). It is modal as the replace prompt is: the
 * workspace is inert while it is open (App.tsx), Tab stays on its buttons,
 * window shortcuts are held back, Cancel has focus and Escape cancels. Nothing
 * is replaced unless Save first (and the save lands) or Discard is chosen.
 *
 * Focus and the key hold are set in a layout effect, inside the commit that
 * shows the prompt (as #1587 did for the replace prompt): a passive effect can
 * run a task later, so a key pressed once the prompt is visible could reach the
 * workspace, or Cancel could lack focus. */
export function UnsavedChangesPrompt({ prompt }: { prompt: UnsavedChangesPromptState | null }) {
  const dialog = useRef<HTMLElement>(null);
  const cancel = useRef<HTMLButtonElement>(null);
  useLayoutEffect(() => {
    if (!prompt) return;
    const opener = prompt.opener;
    cancel.current?.focus();
    const hold = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        event.preventDefault();
        event.stopImmediatePropagation();
        prompt.choose("cancel");
        return;
      }
      if (event.key === "Tab") {
        const buttons = [...(dialog.current?.querySelectorAll<HTMLButtonElement>("button") ?? [])];
        if (buttons.length === 0) return;
        const at = buttons.indexOf(document.activeElement as HTMLButtonElement);
        const next = event.shiftKey ? (at <= 0 ? buttons.length - 1 : at - 1) : (at + 1) % buttons.length;
        event.preventDefault();
        event.stopImmediatePropagation();
        buttons[next].focus();
        return;
      }
      if (event.metaKey || event.ctrlKey || event.altKey) {
        event.preventDefault();
        event.stopImmediatePropagation();
      }
    };
    window.addEventListener("keydown", hold, true);
    return () => {
      window.removeEventListener("keydown", hold, true);
      // Focus returns to where it was when the prompt opened, once the
      // workspace is no longer inert.
      if (opener?.isConnected) requestAnimationFrame(() => opener.focus());
    };
  }, [prompt]);
  if (!prompt) return null;
  return createPortal(
    <div className="replace-project-backdrop" data-testid="unsaved-changes-backdrop">
      <section
        ref={dialog}
        role="alertdialog"
        aria-modal="true"
        aria-labelledby="unsaved-changes-title"
        aria-describedby="unsaved-changes-detail"
        className="replace-project-prompt panel"
        data-testid="unsaved-changes-prompt"
      >
        <h2 id="unsaved-changes-title">The working model has unsaved edits</h2>
        <p id="unsaved-changes-detail" data-testid="unsaved-changes-detail">
          {prompt.action} replaces the working model. Its edits since the last save are not saved and would be lost.
        </p>
        <div className="report-actions">
          <button type="button" ref={cancel} data-testid="unsaved-changes-cancel" onClick={() => prompt.choose("cancel")}>
            Cancel
          </button>
          <button type="button" data-testid="unsaved-changes-save" onClick={() => prompt.choose("save")}>
            Save Local Project, then {prompt.verb}
          </button>
          <button type="button" data-testid="unsaved-changes-discard" onClick={() => prompt.choose("discard")}>
            Discard edits and {prompt.verb}
          </button>
        </div>
      </section>
    </div>,
    document.body
  );
}
