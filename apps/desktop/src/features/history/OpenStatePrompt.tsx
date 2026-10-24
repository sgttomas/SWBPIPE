import { useEffect, useRef } from "react";
import { createPortal } from "react-dom";
import { modelStateDescription } from "../../services/modelStateService";
import type { OpenStatePromptState } from "./useModelStateHistory";

/** The unsaved-changes guard of Open State (W-12 S1): opening a saved state
 * replaces a working model whose edits no save holds. It is modal as the
 * replace prompt is: the workspace is inert while it is open (App.tsx), Tab
 * stays on its buttons, window shortcuts are held back, Cancel has focus and
 * Escape cancels. Nothing changes unless Open is chosen. */
export function OpenStatePrompt({ prompt }: { prompt: OpenStatePromptState | null }) {
  const dialog = useRef<HTMLElement>(null);
  const cancel = useRef<HTMLButtonElement>(null);
  useEffect(() => {
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
      if (opener?.isConnected) requestAnimationFrame(() => opener.focus());
    };
  }, [prompt]);
  if (!prompt) return null;
  return createPortal(
    <div className="replace-project-backdrop" data-testid="open-state-backdrop">
      <section
        ref={dialog}
        role="alertdialog"
        aria-modal="true"
        aria-labelledby="open-state-title"
        aria-describedby="open-state-detail"
        className="replace-project-prompt panel"
        data-testid="open-state-prompt"
      >
        <h2 id="open-state-title">The working model has unsaved edits</h2>
        <p id="open-state-detail" data-testid="open-state-detail">
          Opening the saved state {modelStateDescription(prompt.state)} replaces the working model. Its unsaved edits are
          not in any saved project or state and will be lost.
        </p>
        <div className="report-actions">
          <button type="button" ref={cancel} data-testid="open-state-cancel" onClick={() => prompt.choose("cancel")}>
            Cancel
          </button>
          <button type="button" data-testid="open-state-confirm" onClick={() => prompt.choose("open")}>
            Open "{prompt.state.state_name}"
          </button>
        </div>
      </section>
    </div>,
    document.body
  );
}
