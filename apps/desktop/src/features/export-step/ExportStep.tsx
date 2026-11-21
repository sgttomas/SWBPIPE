import { useLayoutEffect, useRef, useState, type ReactNode } from "react";
import { createPortal } from "react-dom";

/** One export of the Exports page (DEL-07-12): a row at rest, and the step between its button and the save.
 *
 * The row holds the export's name, one line of what it writes, the panel's own line stating the units it writes
 * (where the panel shows one), the label of a record that is not an actual current generated record, and the
 * export's action: its button and, where the export is refused or unavailable, the panel's own line saying so beside
 * it. With no record the button is disabled, and the panel's notices and note stay in the row. Refused with a record
 * (a refused package the panel shows today), the button opens the step to show it, with the refusal in place of the
 * save.
 *
 * The step is a dialog, modal as the model batch file's Export MBF… dialog is: it holds the rest of what the panel
 * shows of its record, complete and in the generator's own terms, and the panel's own save controls
 * (`ControlledExportLink`), which exist only while it is open. Focus moves into it, Tab stays on its controls,
 * window shortcuts are held back, Escape and Close leave it and return focus to the row's button, and the page
 * behind it is covered. Nothing is written by opening or closing it: a file is written only by a save control
 * inside it. The step adds no record, summary or decision of its own. */
export function ExportRow({
  testId,
  className,
  ariaLabel,
  stepTestId,
  icon,
  name,
  what,
  units,
  label,
  action,
  notices,
  record,
  note,
  saves
}: {
  /** The panel's own test id, class and accessible name, kept on the row. */
  testId: string;
  className: string;
  ariaLabel: string;
  /** The prefix of the row's own controls: `<prefix>-open`, `<prefix>-step`, `<prefix>-step-close`. */
  stepTestId: string;
  icon: ReactNode;
  name: string;
  /** One line: what the export writes. */
  what: string;
  /** The panel's line stating the units the export writes, or null where the panel shows none. */
  units?: ReactNode;
  /** The label of a record that is not an actual current generated record, shown in the row and in the step. */
  label?: string | null;
  /** The row's action: built by the panel with `ExportStepButton` and, where it is refused or unavailable, the
   * panel's own line saying so (`export-row-status`), so that a gate the panel wraps around its controls stays
   * where it was. */
  action: (open: (opener: HTMLElement) => void) => ReactNode;
  /** Notices the panel shows of its result: in the step while it can open, otherwise in the row. */
  notices?: ReactNode;
  /** The rest of what the panel shows of its record, or null while there is none (the step cannot open). */
  record: ReactNode | null;
  /** The panel's own note on the export: in the step after the record while it can open, otherwise in the row. */
  note?: ReactNode;
  /** The panel's save controls, shown only inside the step. */
  saves?: ReactNode;
}) {
  const [opener, setOpener] = useState<HTMLElement | null>(null);
  const open = (element: HTMLElement) => setOpener(element);
  // A record that is no longer there (the model, result or run moved on) retires the step showing it: what it
  // showed is not offered for a save.
  if (opener !== null && record === null) setOpener(null);
  // So does a button that is gone: a gate the panel wraps around its controls (the output policy's refusal for a
  // gated result) stands in its place, and the save is not offered.
  useLayoutEffect(() => {
    if (opener !== null && !opener.isConnected) setOpener(null);
  });
  return (
    <section className={`${className} export-row`} aria-label={ariaLabel} data-testid={testId}>
      <div className="panel-title">
        {icon}
        {name}
      </div>
      <div className="export-row-body">
        <p className="export-row-what" data-testid={`${stepTestId}-what`}>{what}</p>
        {label ? <p className="export-row-label" data-testid={`${stepTestId}-record-label`}>{label}</p> : null}
        {units ? <div className="export-row-units">{units}</div> : null}
        {record === null ? notices : null}
      </div>
      <div className="export-row-action">{action(open)}</div>
      {record === null && note ? <div className="export-row-note">{note}</div> : null}
      {opener !== null && record !== null ? (
        <ExportStepDialog
          name={name}
          label={label ?? null}
          stepTestId={stepTestId}
          opener={opener}
          close={() => setOpener(null)}
        >
          {notices}
          {record}
          {note}
          {saves ? <div className="export-step-saves" data-testid={`${stepTestId}-step-saves`}>{saves}</div> : null}
        </ExportStepDialog>
      ) : null}
    </section>
  );
}

/** The row's button: it opens the step. Disabled while the panel has no record to show. Its label is "Export…"
 * where the step offers a save, and "Review…" where the export is refused and the step shows the record without one. */
export function ExportStepButton({
  stepTestId,
  open,
  disabled = false,
  describedBy,
  children = "Export…"
}: {
  stepTestId: string;
  open: (opener: HTMLElement) => void;
  disabled?: boolean;
  /** The id of the line that says why the button is disabled. */
  describedBy?: string;
  children?: ReactNode;
}) {
  return (
    <button
      type="button"
      className="export-row-open"
      data-testid={`${stepTestId}-open`}
      disabled={disabled}
      aria-describedby={describedBy}
      aria-haspopup="dialog"
      onClick={(event) => open(event.currentTarget)}
    >
      {children}
    </button>
  );
}

/** Reports whether it is rendered. A panel places it inside the output gate it wraps around its controls, so that its
 * row reads the gate's own decision (rendered: admitted; not rendered: the gate's refusal stands in its place)
 * without deciding it again. */
export function Rendered({ onChange }: { onChange: (rendered: boolean) => void }) {
  useLayoutEffect(() => {
    onChange(true);
    return () => onChange(false);
  }, [onChange]);
  return null;
}

const FOCUSABLE = "a[href], button:not(:disabled), input:not(:disabled), select:not(:disabled), textarea:not(:disabled), summary, [tabindex]:not([tabindex='-1'])";

function ExportStepDialog({
  name,
  label,
  stepTestId,
  opener,
  close,
  children
}: {
  name: string;
  label: string | null;
  stepTestId: string;
  opener: HTMLElement;
  close: () => void;
  children: ReactNode;
}) {
  const dialog = useRef<HTMLElement>(null);
  const closeRef = useRef(close);
  closeRef.current = close;
  const titleId = `${stepTestId}-step-title`;
  useLayoutEffect(() => {
    // The step opens at the top of its record, with focus inside it.
    dialog.current?.focus();
    const hold = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        event.preventDefault();
        event.stopImmediatePropagation();
        closeRef.current();
        return;
      }
      if (event.key === "Tab") {
        const controls = [...(dialog.current?.querySelectorAll<HTMLElement>(FOCUSABLE) ?? [])].filter((item) => item.getClientRects().length > 0 || item.tagName === "SUMMARY");
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
      if (opener.isConnected) requestAnimationFrame(() => opener.focus());
    };
  }, [opener]);

  return createPortal(
    <div className="replace-project-backdrop export-step-backdrop" data-testid={`${stepTestId}-step-backdrop`}>
      <section
        ref={dialog}
        role="dialog"
        aria-modal="true"
        aria-labelledby={titleId}
        tabIndex={-1}
        className="replace-project-prompt panel export-step-dialog"
        data-testid={`${stepTestId}-step`}
      >
        <h2 id={titleId}>{name}</h2>
        {label ? <p className="export-row-label" data-testid={`${stepTestId}-step-record-label`}>{label}</p> : null}
        <div className="export-step-record">{children}</div>
        <div className="export-step-footer">
          <button type="button" data-testid={`${stepTestId}-step-close`} onClick={() => close()}>Close</button>
        </div>
      </section>
    </div>,
    document.body
  );
}
