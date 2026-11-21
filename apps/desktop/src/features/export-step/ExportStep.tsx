import { useLayoutEffect, useRef, useState, type ReactNode } from "react";
import { createPortal } from "react-dom";
import { rowSaveStatusText, type RowSaveState } from "./exportSave";
import { EXPORT_FORM_WORDS, STANDING_LINE_LIMIT, chooseExportForm, shortTerm, type ControlDisposition, type ExportForm, type PackageRecord, type RecordEntry } from "./exportForm";

/** One export of the Exports page (DEL-07-12): a row, and the record behind it.
 *
 * The row holds the export's name, one line of what it writes, the panel's own line stating the units it writes
 * (where the panel shows one), the label of a record that is not an actual current generated record with its line
 * of what is not written, and the format's standing limits (`chooseExportForm`): their terms where they fit on one
 * line, otherwise their count as a control that opens the record.
 *
 * Its form is chosen in one place, `chooseExportForm`. Where the export writes on the press, the row holds the
 * panel's save controls (`ControlledExportLink`) and a Record control that opens the whole record. Where the record
 * opens first, the row's button says what stands in the way, and the record opens with those entries at its head
 * and the save controls after it. With no record the button is disabled, and the panel's notices and note stay in
 * the row.
 *
 * The record is a dialog, modal as the model batch file's export dialog is: it holds what the panel shows of its
 * record, complete and in the generator's own terms. Focus moves into it, Tab stays on its controls, window
 * shortcuts are held back, Escape and Close leave it and return focus to the control that opened it, and the page
 * behind it is covered. Nothing is written by opening or closing it. The row and the record add no record, summary
 * or decision of their own. */
export function ExportRow({
  testId,
  className,
  ariaLabel,
  stepTestId,
  icon,
  name,
  what,
  press,
  units,
  content,
  label,
  notWritten,
  packageRecord,
  disposition,
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
  /** One line: what the press does, where the row writes on the press with no further choice. */
  press?: string;
  /** The panel's line stating the units the export writes, or null where the panel shows none. */
  units?: ReactNode;
  /** One line of what the file holds, in the generator's terms with counts. */
  content?: ReactNode;
  /** The label of a record that is not an actual current generated record, shown in the row and in the record. */
  label?: string | null;
  /** For a record so labelled: one line of what is not written, in the generator's own words, shown with the
   * pointer to the export used instead for exchange, in the row and in the record. */
  notWritten?: string | null;
  /** The package's record entries, read from the package the save controls write, or null while there is none. */
  packageRecord: PackageRecord | null;
  /** What the export control will do with the file (`AS_GIVEN` until the control reports it). */
  disposition: ControlDisposition;
  /** The row's action, built by the panel so that a gate it wraps around its controls stays where it was: where
   * the row writes on the press it places `write` (the save controls and the Record control); otherwise its button
   * (`ExportStepButton`) with `label` and, where it is refused or unavailable, its own line saying so. */
  action: (control: { open: (opener: HTMLElement) => void; label: string; write: ReactNode | null }) => ReactNode;
  /** Notices the panel shows of its result: in the record while it can open, otherwise in the row. */
  notices?: ReactNode;
  /** The rest of what the panel shows of its record, or null while there is none (the record cannot open). */
  record: ReactNode | null;
  /** The panel's own note on the export: in the record after it while it can open, otherwise in the row. */
  note?: ReactNode;
  /** The panel's save controls: in the row where it writes on the press, otherwise only inside the record. */
  saves?: ReactNode;
}) {
  const [opener, setOpener] = useState<HTMLElement | null>(null);
  const open = (element: HTMLElement) => setOpener(element);
  // The one rule (`chooseExportForm`) chooses the row's form from the package's record and the control's
  // disposition.
  const shown = record !== null && packageRecord !== null ? chooseExportForm(packageRecord, disposition) : null;
  // A record that is no longer there (the model, result or run moved on) retires the record showing it: what it
  // showed is not offered for a save.
  if (opener !== null && shown === null) setOpener(null);
  // So does a control that is gone: a gate the panel wraps around its controls (the output policy's refusal for a
  // gated result) stands in its place, and the save is not offered.
  useLayoutEffect(() => {
    if (opener !== null && !opener.isConnected) setOpener(null);
  });
  const writes = shown?.form === "write-on-press";
  const exchange = (prefix: string) => notWritten ? (
    <>
      <p className="export-row-not-written" data-testid={`${prefix}-not-written`}>{notWritten}</p>
      <p className="export-row-exchange" data-testid={`${prefix}-exchange`}>{EXPORT_FORM_WORDS.exchangeInstead}</p>
    </>
  ) : null;
  const write = writes ? (
    <>
      {saves}
      <button
        type="button"
        className="export-row-record"
        data-testid={`${stepTestId}-record-open`}
        aria-haspopup="dialog"
        onClick={(event) => open(event.currentTarget)}
      >
        {EXPORT_FORM_WORDS.record}
      </button>
    </>
  ) : null;
  return (
    <section className={`${className} export-row`} aria-label={ariaLabel} data-testid={testId}>
      <div className="panel-title">
        {icon}
        {name}
      </div>
      <div className="export-row-body">
        <p className="export-row-what" data-testid={`${stepTestId}-what`}>{what}</p>
        {press ? <p className="export-row-press" data-testid={`${stepTestId}-press`}>{press}</p> : null}
        {label ? <p className="export-row-label" data-testid={`${stepTestId}-record-label`}>{label}</p> : null}
        {exchange(stepTestId)}
        {content ? <div className="export-row-content">{content}</div> : null}
        {units ? <div className="export-row-units">{units}</div> : null}
        {shown ? <StandingLimits standing={shown.standing} stepTestId={stepTestId} open={open} /> : null}
        {record === null ? notices : null}
      </div>
      <div className="export-row-action">{action({ open, label: buttonLabel(shown), write })}</div>
      {record === null && note ? <div className="export-row-note">{note}</div> : null}
      {opener !== null && shown !== null ? (
        <ExportStepDialog
          name={name}
          label={label ?? null}
          stepTestId={stepTestId}
          opener={opener}
          close={() => setOpener(null)}
        >
          {shown.form === "record-first" ? <Causes form={shown} stepTestId={stepTestId} /> : null}
          {exchange(`${stepTestId}-step`)}
          {notices}
          {record}
          {note}
          {packageRecord ? <Entries entries={packageRecord.entries} validation={packageRecord.validation ?? null} stepTestId={stepTestId} /> : null}
          {!writes && saves ? <div className="export-step-saves" data-testid={`${stepTestId}-step-saves`}>{saves}</div> : null}
        </ExportStepDialog>
      ) : null}
    </section>
  );
}

function buttonLabel(form: ExportForm | null): string {
  if (form === null || form.form === "write-on-press") return "Export…";
  if (form.why === "withheld") return EXPORT_FORM_WORDS.reviewWithheld;
  if (form.why === "replaced") return EXPORT_FORM_WORDS.reviewReplaced(form.replaced ?? 0);
  return EXPORT_FORM_WORDS.reviewThisModel(form.causes.length);
}

/** The format's standing limits at the row: the terms where they fit on one line, otherwise their count as a
 * control that opens the record. They do not stop the press. */
function StandingLimits({ standing, stepTestId, open }: { standing: ReadonlyArray<RecordEntry>; stepTestId: string; open: (opener: HTMLElement) => void }) {
  if (standing.length === 0) return null;
  const terms = standing.map((entry) => shortTerm(entry.term));
  const line = EXPORT_FORM_WORDS.standingLine(terms);
  if (line.length <= STANDING_LINE_LIMIT) return <p className="export-row-standing" data-testid={`${stepTestId}-standing`}>{line}</p>;
  return (
    <button
      type="button"
      className="export-row-standing link-button"
      data-testid={`${stepTestId}-standing`}
      aria-haspopup="dialog"
      title={terms.join("; ")}
      onClick={(event) => open(event.currentTarget)}
    >
      {EXPORT_FORM_WORDS.standingCount(standing.length)}
    </button>
  );
}

/** What stands in the way, first in a record that opens before the file is written. */
function Causes({ form, stepTestId }: { form: Extract<ExportForm, { form: "record-first" }>; stepTestId: string }) {
  if (form.why === "withheld") return form.reason ? <p className="export-step-reason" data-testid={`${stepTestId}-step-reason`}>{form.reason}</p> : null;
  if (form.causes.length === 0) return null;
  return (
    <div className="export-step-causes" data-testid={`${stepTestId}-step-causes`}>
      <h3>{EXPORT_FORM_WORDS.causesHeading}</h3>
      <ul>
        {form.causes.map((entry, index) => (
          <li key={`${entry.term}:${index}`} data-testid={`${stepTestId}-step-cause`}>
            <strong>{entry.term}</strong>
            {[entry.category, entry.severity].filter(Boolean).length > 0 ? <span> ({[entry.category, entry.severity].filter(Boolean).join("; ")})</span> : null}
            {entry.text ? <span>: {entry.text}</span> : null}
          </li>
        ))}
      </ul>
    </div>
  );
}

/** Every entry of the package's record, in the generator's own terms. */
function Entries({ entries, validation, stepTestId }: { entries: PackageRecord["entries"]; validation: PackageRecord["validation"] | null; stepTestId: string }) {
  if (entries.length === 0 && !validation) return null;
  return (
    <details className="export-step-entries" data-testid={`${stepTestId}-step-entries`} open>
      <summary>Record entries ({entries.length}){validation ? `; validation=${validation.status}` : ""}</summary>
      <ul>
        {entries.map((entry, index) => (
          <li key={`${entry.term}:${index}`} data-testid={`${stepTestId}-step-entry`} data-entry-kind={entry.kind}>
            <strong>{entry.term}</strong>
            {[entry.kind, entry.category, entry.severity].filter(Boolean).length > 0 ? <span> ({[entry.kind, entry.category, entry.severity].filter(Boolean).join("; ")})</span> : null}
            {entry.text ? <span>: {entry.text}</span> : null}
          </li>
        ))}
      </ul>
    </details>
  );
}

/** The row's button: it opens the record. Disabled while the panel has no record to show. Its label is the one
 * `ExportRow` passes the panel's action. */
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

/** One save control of a row, with the row's status line under it.
 *
 * Interim, until the export control reports its disposition to the row (the hook): a save control that writes
 * nothing, rendered by the control as an element with no address (`aria-disabled`, no `href`), is not shown as a
 * control; one line says the file is not available in its place. This reads only the rendered element, and stays
 * outside `chooseExportForm`, because the control renders only where the row's form puts it. Remove it when the
 * hook's disposition is the rule's input and the native saves replace these links.
 *
 * The status line says a save's outcome (`rowSaveStatusText`) and nothing else: empty until a save reports one,
 * and empty after a press of a link that reports none. */
export function ExportSave({ control, file, state = null, children }: { control: string; file: string; state?: RowSaveState; children: ReactNode }) {
  const slot = useRef<HTMLDivElement>(null);
  const [dead, setDead] = useState(false);
  useLayoutEffect(() => {
    const element = slot.current;
    if (!element) return;
    const read = () => {
      const item = element.querySelector(`[data-testid="${control}"]`);
      setDead(item !== null && item.getAttribute("aria-disabled") === "true" && !item.hasAttribute("href") && item.tagName !== "BUTTON");
    };
    read();
    const observer = new MutationObserver(read);
    observer.observe(element, { subtree: true, childList: true, attributes: true, attributeFilter: ["href", "aria-disabled"] });
    return () => observer.disconnect();
  }, [control]);
  return (
    <div ref={slot} className="export-save" data-unavailable={dead ? "true" : undefined}>
      {children}
      {dead ? <p className="export-save-unavailable" data-testid={`${control}-unavailable`}>{EXPORT_FORM_WORDS.unavailable(file)}</p> : null}
      <p className="export-save-status" role="status" data-testid={`${control}-save-status`}>{rowSaveStatusText(state)}</p>
    </div>
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
    // The record opens at its top, with focus inside it.
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
      // Focus returns to the control that opened the record once it is closed: not where the record is mounted
      // again at once (React's development double mount).
      requestAnimationFrame(() => {
        if (opener.isConnected && document.querySelector(`[data-testid="${stepTestId}-step"]`) === null) opener.focus();
      });
    };
  }, [opener, stepTestId]);

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
