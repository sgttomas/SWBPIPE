import { useLayoutEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { USER_RELEASE_STATEMENT, userReleasedAtOf } from "./reportPackageRequest";
import type { UserReleaseConfirmation } from "./userReleasedReportPackageSave";

/** Why a shareable package cannot be saved for this report, shown in place of the statement
 * (§2.2): the model's library or rule-pack reference, or a result the user-release cell
 * refuses (with the cell's reason). */
export type UserReleaseDialogRefusal =
  | { kind: "library_or_rule_pack"; reason: string }
  | { kind: "result"; reason: string };

/** The open Save Shareable Report Package… dialog: the report it covers, its opener (focus
 * returns there), any refusal, and the two answers. */
export type UserReleaseDialogRequest = {
  projectName: string;
  /** SHA-256 of the RFC 8785 bytes of the session model document (64 lowercase hex). */
  modelDocumentSha256: string;
  runId: string;
  refusal: UserReleaseDialogRefusal | null;
  opener: HTMLElement | null;
  cancel: () => void;
  save: (confirmation: UserReleaseConfirmation) => void;
};

export const USER_RELEASE_DIALOG_TITLE = "Release your project data in one shareable report";
export const USER_RELEASE_LIBRARY_REFUSAL =
  "This model uses a library record or a rule pack. Shareable packages do not yet carry library record values or rule-pack checks, so a shareable package cannot be saved for this model. Its local private package is unaffected.";
export const USER_RELEASE_RESULT_REFUSAL =
  "This result is not one a shareable package carries. Shareable packages carry a pressure-free (preview-physics-2) result, or an exact pressure route (pressure-2 or pressure-source-1) result that is eligible for output, so a shareable package cannot be saved for this result. Its local private package is unaffected.";

/** File > Save Shareable Report Package… (W-18b; DEL-08-01 Design `report-user-release.md`
 * §2.2). The user's explicit selection (opening this dialog from its own control) and
 * statement of their redistribution right (the checkbox, verbatim the record's `statement`)
 * start one save. The checkbox starts unticked at every opening; Escape and Cancel close the
 * dialog and discard the selection. Nothing is remembered beyond the one save (§2.3
 * "Session"). Modal as Save Handoff… is: the workspace is inert while it is open (App.tsx),
 * Tab stays on its controls and window shortcuts are held back. The dialog records no
 * acceptance, issuance or software release. */
export function UserReleaseDialog({ request }: { request: UserReleaseDialogRequest | null }) {
  // A request is always closed (null) before the next opens, so each dialog starts unticked.
  return request ? <UserReleaseDialogBody request={request} /> : null;
}

function UserReleaseDialogBody({ request }: { request: UserReleaseDialogRequest }) {
  const dialog = useRef<HTMLElement>(null);
  const statement = useRef<HTMLInputElement>(null);
  const cancel = useRef<HTMLButtonElement>(null);
  const [stated, setStated] = useState(false);
  useLayoutEffect(() => {
    const opener = request.opener;
    (statement.current ?? cancel.current)?.focus();
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
  const refused = request.refusal !== null;
  const ready = stated && !refused;
  return createPortal(
    <div className="replace-project-backdrop" data-testid="user-release-backdrop">
      <section
        ref={dialog}
        role="dialog"
        aria-modal="true"
        aria-labelledby="user-release-title"
        aria-describedby="user-release-detail"
        className="replace-project-prompt panel"
        data-testid="user-release-dialog"
      >
        <h2 id="user-release-title">{USER_RELEASE_DIALOG_TITLE}</h2>
        <p id="user-release-detail" data-testid="user-release-detail">
          This saves a report package you may share. It includes this project's values and results, unredacted.
        </p>
        <ul data-testid="user-release-terms">
          <li data-testid="user-release-scope">
            It covers only this report: model "{request.projectName}" (SHA-256 {request.modelDocumentSha256.slice(0, 12)}…), run{" "}
            {request.runId}.
          </li>
          <li>
            It carries the model summary, the load cases, this solve's results and any model-input tables in the report
            sections you chose; every result row is still listed with its checksum. An exact pressure route result also
            carries its exact-route standing, disclosures, headlines and limitations. It carries no rule-pack reference, user-rule
            outcome or run comparison.
          </li>
          <li>
            It needs a result solved in this session: a pressure-free (preview-physics-2) result, or an exact pressure
            route (pressure-2 or pressure-source-1) result that is eligible for output. A model with a library or
            rule-pack reference cannot be shared this way.
          </li>
          <li>
            The application checks this model only for library and rule-pack marks. It cannot tell where any other value
            came from, so your statement below covers every value in this report.
          </li>
          <li>Content marked protected, quarantined or rejected still stops the save.</li>
          <li>Nothing is remembered: your next report is local private unless you choose this again.</li>
        </ul>
        {refused ? (
          <div role="status" data-testid="user-release-refusal">
            <p>{request.refusal!.kind === "library_or_rule_pack" ? USER_RELEASE_LIBRARY_REFUSAL : USER_RELEASE_RESULT_REFUSAL}</p>
            <p data-testid="user-release-refusal-reason">{request.refusal!.reason}</p>
          </div>
        ) : (
          <label className="model-document-save-intent">
            <input
              ref={statement}
              type="checkbox"
              checked={stated}
              data-testid="user-release-statement"
              onChange={(event) => setStated(event.currentTarget.checked)}
            />
            {USER_RELEASE_STATEMENT}
          </label>
        )}
        <div className="report-actions">
          <button
            type="button"
            data-testid="user-release-confirm"
            disabled={!ready}
            onClick={() => {
              if (!ready) return;
              request.save({ userReleasedAt: userReleasedAtOf(new Date()) });
            }}
          >
            Save Shareable Report Package…
          </button>
          <button type="button" ref={cancel} data-testid="user-release-cancel" onClick={() => request.cancel()}>
            Cancel
          </button>
        </div>
      </section>
    </div>,
    document.body
  );
}
