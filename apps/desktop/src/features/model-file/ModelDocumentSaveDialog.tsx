import { useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { MODEL_DOCUMENT_SAVE_INTENT_LABEL } from "./modelDocumentSave";

/** The open Save Model Document… dialog: its opener (focus returns there) and
 * the two answers. `save` carries the dialog's own explicit intent. */
export type ModelDocumentSaveDialogRequest = {
  projectName: string;
  opener: HTMLElement | null;
  cancel: () => void;
  save: (explicitLocalPrivateIntent: boolean) => void;
};

/** File > Save Model Document… (DEL-15-01 W-6a). The save is the local-private
 * context: the file chooser alone is not the intent, so the dialog asks for it
 * with its own control, as the report package save does. It is modal like the
 * replace prompt: the workspace is inert while it is open (App.tsx), Tab stays
 * on its controls, window shortcuts are held back, Cancel has focus and Escape
 * cancels. Choosing the file and writing it happen only after the explicit
 * choice. */
export function ModelDocumentSaveDialog({ request }: { request: ModelDocumentSaveDialogRequest | null }) {
  const dialog = useRef<HTMLElement>(null);
  const cancel = useRef<HTMLButtonElement>(null);
  const [intent, setIntent] = useState(false);
  useEffect(() => {
    setIntent(false);
    if (!request) return;
    const opener = request.opener;
    cancel.current?.focus();
    const hold = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        event.preventDefault();
        event.stopImmediatePropagation();
        request.cancel();
        return;
      }
      if (event.key === "Tab") {
        const controls = [...(dialog.current?.querySelectorAll<HTMLElement>("input, button:not(:disabled)") ?? [])];
        if (controls.length === 0) return;
        const at = controls.indexOf(document.activeElement as HTMLElement);
        const next = event.shiftKey ? (at <= 0 ? controls.length - 1 : at - 1) : (at + 1) % controls.length;
        event.preventDefault();
        event.stopImmediatePropagation();
        controls[next].focus();
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
  }, [request]);
  if (!request) return null;
  return createPortal(
    <div className="replace-project-backdrop" data-testid="model-document-save-backdrop">
      <section
        ref={dialog}
        role="dialog"
        aria-modal="true"
        aria-labelledby="model-document-save-title"
        aria-describedby="model-document-save-detail"
        className="replace-project-prompt panel"
        data-testid="model-document-save-dialog"
      >
        <h2 id="model-document-save-title">Save Model Document</h2>
        <p id="model-document-save-detail" data-testid="model-document-save-detail">
          Saves the model document of "{request.projectName}" as its canonical JSON (RFC 8785) to a file you
          choose, starting in your Documents folder. The file's SHA-256 is the session's model hash, and the file
          opens again with Open Model Document…. It holds your private project data. Nothing is sent anywhere, and
          the session's model and results are unchanged.
        </p>
        <label className="model-document-save-intent">
          <input
            type="checkbox"
            checked={intent}
            data-testid="model-document-save-intent"
            onChange={(event) => setIntent(event.currentTarget.checked)}
          />
          {MODEL_DOCUMENT_SAVE_INTENT_LABEL}
        </label>
        <div className="report-actions">
          <button type="button" ref={cancel} data-testid="model-document-save-cancel" onClick={() => request.cancel()}>
            Cancel
          </button>
          <button
            type="button"
            data-testid="model-document-save-confirm"
            disabled={!intent}
            onClick={() => { if (intent) request.save(true); }}
          >
            Choose file and save…
          </button>
        </div>
      </section>
    </div>,
    document.body
  );
}
