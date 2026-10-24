import { useLayoutEffect, useRef } from "react";
import { createPortal } from "react-dom";
import { storedProjectDescription, type ReplaceProjectPromptState } from "./storedProjectConflict";

/** Asks before a save would replace a stored project (HELP_HUMAN rulings
 * 2026-10-10). It is modal: the workspace is inert while it is open (App.tsx),
 * Tab stays on its buttons, window shortcuts are held back, Cancel has focus and
 * Escape cancels. Nothing is replaced unless Replace is chosen.
 *
 * Focus and the key hold are set in a layout effect, inside the commit that
 * shows the prompt. A passive effect can run a task later, so a key pressed
 * once the prompt is visible could reach the workspace, or Cancel could lack
 * focus. */
export function ReplaceProjectPrompt({ prompt }: { prompt: ReplaceProjectPromptState | null }) {
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
    <div className="replace-project-backdrop" data-testid="replace-project-backdrop">
      <section
        ref={dialog}
        role="alertdialog"
        aria-modal="true"
        aria-labelledby="replace-project-title"
        aria-describedby="replace-project-detail"
        className="replace-project-prompt panel"
        data-testid="replace-project-prompt"
      >
        <h2 id="replace-project-title">
          {prompt.stale ? "The stored copy of this project has changed" : "A project with this id is already stored"}
        </h2>
        <p id="replace-project-detail" data-testid="replace-project-detail">
          {prompt.stale
            ? <>The stored project {storedProjectDescription(prompt.existing)} has been saved since this session last saw it. {prompt.kind === "create" ? "Creating" : "Saving"} this project would replace that stored copy.</>
            : <>{prompt.kind === "create" ? "Creating" : "Saving"} this project would replace the stored project {storedProjectDescription(prompt.existing)}.</>}
        </p>
        <div className="report-actions">
          <button type="button" ref={cancel} data-testid="replace-project-cancel" onClick={() => prompt.choose("cancel")}>
            Cancel
          </button>
          <button type="button" data-testid="replace-project-save-as-new" onClick={() => prompt.choose("save-as-new")}>
            Save as a new project
          </button>
          <button type="button" data-testid="replace-project-replace" onClick={() => prompt.choose("replace")}>
            Replace "{prompt.existing.project_name}"
          </button>
        </div>
      </section>
    </div>,
    document.body
  );
}
