import { useLayoutEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { modelStateDescription } from "../../services/modelStateService";
import { HANDOFF_NEW_STATE_LABEL, HANDOFF_SAVE_INTENT_LABEL, type HandoffBasis } from "./handoffPackageSave";

/** The dialog's answer: the explicit intent, the package label, and the name of
 * the new basis state when one is to be saved first (null: the existing basis). */
export type HandoffSaveChoice = {
  explicitLocalPrivateIntent: boolean;
  packageLabel: string;
  newStateName: string | null;
};

/** The open Save Handoff… dialog: its opener (focus returns there), the basis
 * and the two answers. */
export type HandoffSaveDialogRequest = {
  projectName: string;
  basis: HandoffBasis;
  opener: HTMLElement | null;
  cancel: () => void;
  save: (choice: HandoffSaveChoice) => void;
};

/** File > Save Handoff… (DEL-15-01 W-6b). The save is the local-private
 * context: the folder chooser alone is not the intent, so the dialog asks for it
 * with its own control. When the model is not yet a saved state, the new
 * `handoff_basis` state is confirmed here, by its own control, before anything is
 * written. It is modal as the model-document dialog is: the workspace is inert
 * while it is open (App.tsx), Tab stays on its controls, window shortcuts are
 * held back, Cancel has focus and Escape cancels. */
export function HandoffSaveDialog({ request }: { request: HandoffSaveDialogRequest | null }) {
  // A request is always closed (null) before the next opens, so each dialog
  // starts from its own request's values.
  return request ? <HandoffSaveDialogBody request={request} /> : null;
}

function HandoffSaveDialogBody({ request }: { request: HandoffSaveDialogRequest }) {
  const dialog = useRef<HTMLElement>(null);
  const cancel = useRef<HTMLButtonElement>(null);
  const [intent, setIntent] = useState(false);
  const [label, setLabel] = useState(request.projectName);
  const [stateName, setStateName] = useState(request.basis.kind === "new" ? request.basis.defaultName : "");
  const [stateConfirmed, setStateConfirmed] = useState(false);
  useLayoutEffect(() => {
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
  const needsState = request.basis.kind === "new";
  const ready = intent && label.trim().length > 0 && (!needsState || (stateConfirmed && stateName.trim().length > 0));
  return createPortal(
    <div className="replace-project-backdrop" data-testid="handoff-save-backdrop">
      <section
        ref={dialog}
        role="dialog"
        aria-modal="true"
        aria-labelledby="handoff-save-title"
        aria-describedby="handoff-save-detail"
        className="replace-project-prompt panel"
        data-testid="handoff-save-dialog"
      >
        <h2 id="handoff-save-title">Save Handoff</h2>
        <p id="handoff-save-detail" data-testid="handoff-save-detail">
          Saves a handoff package folder (.swbhandoff) for "{request.projectName}" where you choose, starting in your
          Documents folder: the model document as its canonical JSON (RFC 8785) and a handoff manifest that binds it
          by SHA-256 and lists its units, entity ids, library and rule-pack references, warnings and unresolved
          assumptions. It holds your private project data. Nothing is sent anywhere, and the session's model and
          results are unchanged. The manifest supports downstream modeling and review; it records no acceptance.
        </p>
        <p data-testid="handoff-save-basis">
          {request.basis.kind === "existing"
            ? `Basis state: ${modelStateDescription(request.basis.state)}, the saved state of this model.`
            : "This model is not a saved model state yet. A handoff is based on a saved state, so one is saved first; it stays saved even if the package is not."}
        </p>
        {needsState ? (
          <>
            <label className="model-document-save-intent">
              New state name
              <input
                type="text"
                value={stateName}
                data-testid="handoff-save-state-name"
                onChange={(event) => setStateName(event.currentTarget.value)}
              />
            </label>
            <label className="model-document-save-intent">
              <input
                type="checkbox"
                checked={stateConfirmed}
                data-testid="handoff-save-state-confirm"
                onChange={(event) => setStateConfirmed(event.currentTarget.checked)}
              />
              {HANDOFF_NEW_STATE_LABEL}
            </label>
          </>
        ) : null}
        <label className="model-document-save-intent">
          Package label
          <input type="text" value={label} data-testid="handoff-save-label" onChange={(event) => setLabel(event.currentTarget.value)} />
        </label>
        <label className="model-document-save-intent">
          <input
            type="checkbox"
            checked={intent}
            data-testid="handoff-save-intent"
            onChange={(event) => setIntent(event.currentTarget.checked)}
          />
          {HANDOFF_SAVE_INTENT_LABEL}
        </label>
        <div className="report-actions">
          <button type="button" ref={cancel} data-testid="handoff-save-cancel" onClick={() => request.cancel()}>
            Cancel
          </button>
          <button
            type="button"
            data-testid="handoff-save-confirm"
            disabled={!ready}
            onClick={() => {
              if (!ready) return;
              request.save({ explicitLocalPrivateIntent: true, packageLabel: label.trim(), newStateName: needsState ? stateName.trim() : null });
            }}
          >
            Choose folder and save…
          </button>
        </div>
      </section>
    </div>,
    document.body
  );
}
