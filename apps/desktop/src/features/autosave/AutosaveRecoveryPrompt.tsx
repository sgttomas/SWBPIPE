import { useLayoutEffect, useRef } from "react";
import { createPortal } from "react-dom";
import type { AutosaveRecoveryPromptState } from "./useAutosave";

/** The recovery offer (DEL-02-05 Design `autosave-and-backup.md` §3). It is
 * modal as the unsaved-changes prompt is: it takes the shared modal hold, the
 * workspace is inert while it is open (App.tsx), Tab stays on its buttons,
 * window shortcuts are held back, and focus returns to where it was when it
 * closes. It says what the record is, which is newer (item (b)), what Open
 * would replace and that the stored project stays unchanged. Keep for later
 * is the default (focused, and Escape); Discard asks once more, where Cancel
 * is the default. An unreadable record shows its code and offers Discard
 * only. Nothing is applied unless Open autosave is chosen. */
export function AutosaveRecoveryPrompt({ prompt }: { prompt: AutosaveRecoveryPromptState | null }) {
  const dialog = useRef<HTMLElement>(null);
  const safe = useRef<HTMLButtonElement>(null);
  const opener = prompt?.opener ?? null;
  const open = prompt !== null;
  // Focus moves to the safe choice whenever the view changes (the offer, or
  // the Discard confirmation).
  useLayoutEffect(() => {
    if (prompt) safe.current?.focus();
  }, [prompt?.fileName, prompt?.confirmingDiscard]);
  useLayoutEffect(() => {
    if (!prompt) return;
    const hold = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        event.preventDefault();
        event.stopImmediatePropagation();
        prompt.choose(prompt.confirmingDiscard ? "discard-cancelled" : "keep");
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
    return () => window.removeEventListener("keydown", hold, true);
  }, [prompt]);
  // Focus returns to the opener once the dialog closes and the workspace is
  // no longer inert.
  useLayoutEffect(() => {
    if (!open) return;
    return () => {
      if (opener?.isConnected) requestAnimationFrame(() => opener.focus());
    };
  }, [open, opener]);
  if (!prompt) return null;
  const { record } = prompt;
  return createPortal(
    <div className="replace-project-backdrop" data-testid="autosave-recovery-backdrop">
      <section
        ref={dialog}
        role="alertdialog"
        aria-modal="true"
        aria-labelledby="autosave-recovery-title"
        aria-describedby="autosave-recovery-detail"
        className="replace-project-prompt panel"
        data-testid="autosave-recovery-prompt"
      >
        {prompt.confirmingDiscard ? (
          <>
            <h2 id="autosave-recovery-title">Discard this autosave?</h2>
            <p id="autosave-recovery-detail" data-testid="autosave-discard-detail">
              The autosave{record ? ` of ${record.projectName}, written ${record.writtenAt},` : ""} is removed from this computer and cannot be recovered afterwards. No stored project and no saved file is changed.
            </p>
            <div className="report-actions">
              <button type="button" ref={safe} data-testid="autosave-discard-cancel" onClick={() => prompt.choose("discard-cancelled")}>
                Cancel
              </button>
              <button type="button" data-testid="autosave-discard-confirm" onClick={() => prompt.choose("discard-confirmed")}>
                Discard autosave
              </button>
            </div>
          </>
        ) : (
          <>
            <h2 id="autosave-recovery-title">{record ? "An autosave can be recovered" : "An autosave cannot be read"}</h2>
            <div id="autosave-recovery-detail">
              {record ? (
                <p data-testid="autosave-recovery-what">
                  Autosave of {record.projectName} ({record.projectId}), written {record.writtenAt}, {record.byteCount} bytes, model hash {record.shortHash}. {prompt.notIncluded}
                </p>
              ) : (
                <p data-testid="autosave-recovery-unreadable">
                  The autosave {prompt.fileName} could not be read: {prompt.unreadable}. It cannot be opened; it can be discarded.
                </p>
              )}
              <p data-testid="autosave-recovery-newer">{prompt.newer}</p>
              {record ? <p data-testid="autosave-recovery-replaces">{prompt.replaces}</p> : null}
              <p data-testid="autosave-recovery-stored">{prompt.stored}</p>
            </div>
            <div className="report-actions">
              <button type="button" ref={safe} data-testid="autosave-recovery-keep" onClick={() => prompt.choose("keep")}>
                {record ? "Keep for later" : "Close"}
              </button>
              {record ? (
                <button type="button" data-testid="autosave-recovery-open" onClick={() => prompt.choose("open")}>
                  Open autosave
                </button>
              ) : null}
              <button type="button" data-testid="autosave-recovery-discard" onClick={() => prompt.choose("discard")}>
                Discard…
              </button>
            </div>
          </>
        )}
      </section>
    </div>,
    document.body
  );
}
