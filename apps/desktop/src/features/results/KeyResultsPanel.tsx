/** The key-results panel (survey item A6; DEL-07-05 Design `key-results-panel.md`): after a solve,
 * one place to put the result on screen. Each action is one button and one key, and it sets view
 * state that already exists: the viewport's result colour quantity and case, the deformation case,
 * the session's selection of a support (which the support load summary follows), a results-table
 * filter, or the Results section; or it returns to editing. The panel shows no value of its own:
 * categories and ratios appear only from the user's rule-check run of this result, and an absent
 * or incomplete pack is shown as absent or incomplete. A view only: it changes no model, result,
 * export or undo history. */

import { X } from "lucide-react";
import { Fragment, useId, useMemo, useState, type KeyboardEvent, type RefObject } from "react";
import type { MechanicsResult, PreviewModel } from "../../types";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";
import type { ResultOverlayStanding } from "./numericalHealth";
import { isNavigatorExcludedTarget } from "./resultsNavigator";
import {
  KEY_DEFLECTION_NO_CASES,
  KEY_RATIO_NONE,
  KEY_CONTACT_NONE,
  KEY_RESULTS_KEYS,
  buildKeyResults,
  keyResultsKeyAction,
  type KeyResultsActionId
} from "./keyResults";

export const KEY_RESULTS_TITLE = "Key results";
export const KEY_RESULTS_BOUNDARY_TEXT =
  "Each view shows the result's own rows or your rule-check outcomes in an existing view. Nothing here is a limit, an acceptance or a compliance finding.";
export const KEY_RESULTS_CANVAS_HIDDEN = "The canvas is hidden in Table view; choose Model or Both view to see it.";

export type KeyResultsTable = "results" | "stress-ratio" | "displacement-extremes";

export type KeyResultsPanelProps = Readonly<{
  model: PreviewModel;
  result: MechanicsResult | null;
  /** App's `viewportResultStanding`: the reader's standing and the session's health view. */
  standing: ResultOverlayStanding | null | undefined;
  /** The session's rule-check run, only while `result` is the current solved result it was run on. */
  ruleCheckRun?: SessionRuleCheckRun | null;
  /** A reopened run is shown in Results. */
  historical?: boolean;
  /** The current stage shows no canvas (Table view). */
  canvasHidden?: boolean;
  everySolve: boolean;
  onEverySolveChange: (value: boolean) => void;
  onShowRatioContour: (quantityId: string, caseKey: string) => void;
  onShowDeflectedShape: (caseKey: string) => void;
  onShowSupport: (supportId: string) => void;
  onShowContactStatus: () => void;
  onOpenTable: (table: KeyResultsTable) => void;
  onBackToEditing: () => void;
  onOpenRuleChecks: () => void;
  onClose: () => void;
  panelRef?: RefObject<HTMLElement | null>;
}>;

type Offer = Readonly<{ label: string; reason: string | null; run: () => string }>;

export function KeyResultsPanel(props: KeyResultsPanelProps) {
  const { model, result, standing, ruleCheckRun = null, historical = false, canvasHidden = false } = props;
  const titleId = useId();
  const data = useMemo(
    () => buildKeyResults({ model, result, standing, ruleCheckRun, historical }),
    [historical, model, result, ruleCheckRun, standing]
  );
  const [categoryChoice, setCategoryChoice] = useState<string | null>(null);
  const [caseChoice, setCaseChoice] = useState<string | null>(null);
  const [supportChoice, setSupportChoice] = useState<string | null>(null);
  const [status, setStatus] = useState<string>("");
  const category = data.ratio.categories.find((item) => item.quantityId === categoryChoice) ?? data.ratio.categories[0] ?? null;
  const deflectionCase = data.cases.find((item) => item.key === caseChoice) ?? data.cases[0] ?? null;
  const support = data.supports.items.find((item) => item.supportId === supportChoice) ?? data.supports.items[0] ?? null;
  const viewReason = data.state === "available" ? null : data.reason;
  const canvasNote = canvasHidden ? ` ${KEY_RESULTS_CANVAS_HIDDEN}` : "";

  const offers: Readonly<Record<KeyResultsActionId, Offer>> = {
    ratio: {
      label: "Show ratio contour",
      reason: viewReason ?? (category ? category.unavailableReason : data.ratio.state === "used" ? KEY_RATIO_NONE : data.ratio.note),
      run: () => {
        props.onShowRatioContour(category!.quantityId, category!.caseKey!);
        return `Result colour: ${category!.label}, ${category!.caseLabel}.${canvasNote}`;
      }
    },
    deflection: {
      label: "Show deflected shape",
      reason: viewReason ?? (deflectionCase ? null : KEY_DEFLECTION_NO_CASES),
      run: () => {
        props.onShowDeflectedShape(deflectionCase!.key);
        return `Deflected shape: ${deflectionCase!.label}.${canvasNote}`;
      }
    },
    support: {
      label: "Show support load summary",
      reason: viewReason ?? (support ? null : data.supports.reason),
      run: () => {
        props.onShowSupport(support!.supportId);
        return `Support load summary: ${support!.label}, in Results.`;
      }
    },
    contact: {
      label: "Show contact and lift-off state",
      reason: viewReason ?? (data.contact.rowCount > 0 ? null : KEY_CONTACT_NONE),
      run: () => {
        props.onShowContactStatus();
        return "Results table: support contact and lift-off state rows, all cases.";
      }
    },
    "results-table": { label: "Results table", reason: null, run: () => { props.onOpenTable("results"); return "Results table, in Results."; } },
    "stress-ratio-table": { label: "Stress and ratio table", reason: null, run: () => { props.onOpenTable("stress-ratio"); return "Stress and ratio table, in Results."; } },
    "displacement-extremes": { label: "Displacement extremes", reason: null, run: () => { props.onOpenTable("displacement-extremes"); return "Displacement extremes, in Results."; } },
    edit: { label: "Back to editing", reason: null, run: () => { props.onBackToEditing(); return ""; } }
  };

  /** The one path for a button and its key. An unavailable action does nothing but say why. */
  function act(action: KeyResultsActionId) {
    const offer = offers[action];
    setStatus(offer.reason ?? offer.run());
  }

  function onKeyDown(event: KeyboardEvent<HTMLElement>) {
    if (event.defaultPrevented) return;
    if (event.key === "Escape") {
      event.preventDefault();
      props.onClose();
      return;
    }
    if (isNavigatorExcludedTarget(event.target)) return;
    const action = keyResultsKeyAction(event);
    if (!action) return;
    event.preventDefault();
    act(action);
  }

  // A plain function, not a component, so a re-render keeps the same button and its focus.
  function actionButton(action: KeyResultsActionId) {
    const offer = offers[action];
    const key = KEY_RESULTS_KEYS[action];
    const reasonId = `${titleId}-${action}-reason`;
    return (
      <Fragment key={action}>
        <button
          type="button"
          className="key-results-action"
          data-testid={`key-results-${action}`}
          aria-keyshortcuts={key}
          aria-disabled={offer.reason ? true : undefined}
          aria-describedby={offer.reason ? reasonId : undefined}
          title={`${offer.label} (${key})`}
          onClick={() => act(action)}
        >
          <span>{offer.label}</span> <kbd aria-hidden="true">{key}</kbd>
        </button>
        {offer.reason ? <small className="key-results-reason" id={reasonId} data-testid={`key-results-${action}-reason`}>{offer.reason}</small> : null}
      </Fragment>
    );
  }

  const ratio = data.ratio;
  return (
    <section
      className="key-results-panel"
      role="region"
      aria-labelledby={titleId}
      data-testid="key-results-panel"
      data-state={data.state}
      ref={props.panelRef}
      onKeyDown={onKeyDown}
    >
      <header className="key-results-header">
        <h2 id={titleId}>{KEY_RESULTS_TITLE}</h2>
        {result && data.state !== "historical" ? <code data-testid="key-results-run">{result.run_id}</code> : null}
        <button type="button" className="shell-icon-button" data-testid="key-results-close" aria-label="Close key results" aria-keyshortcuts="Escape" title="Close (⎋)" onClick={() => props.onClose()}>
          <X size={14} aria-hidden="true" />
        </button>
      </header>
      {data.reason ? <p className="key-results-line" data-testid="key-results-reason">{data.reason}</p> : null}
      {data.standingText ? <p className="key-results-line" data-testid="key-results-standing">{data.standingText}</p> : null}

      <div className="key-results-group" role="group" aria-label="Ratio contours">
        <h3>Ratio contours</h3>
        <p className="key-results-line" data-testid="key-results-ratio-note" data-ratio-state={ratio.state}>{ratio.note}</p>
        {ratio.aggregate ? <p className="key-results-line" data-testid="key-results-ratio-aggregate" data-status-token={ratio.aggregate.token}>{ratio.aggregate.text}</p> : null}
        {ratio.incompleteNote ? <p className="key-results-line key-results-incomplete" data-testid="key-results-ratio-incomplete">{ratio.incompleteNote}</p> : null}
        {ratio.declaredWithoutOutcomes.length > 0 ? (
          <p className="key-results-line key-results-incomplete" data-testid="key-results-ratio-declared-without-outcomes">
            Declared by the pack with no outcome in this run: {ratio.declaredWithoutOutcomes.join(", ")}.
          </p>
        ) : null}
        {ratio.categories.length > 0 ? (
          <label className="key-results-choice">
            <span>Stress category</span>
            <select data-testid="key-results-ratio-category" value={category?.quantityId ?? ""} onChange={(event) => setCategoryChoice(event.target.value)}>
              {ratio.categories.map((item) => (
                <option key={item.quantityId} value={item.quantityId}>
                  {item.label}: {item.outcomeCount} outcome{item.outcomeCount === 1 ? "" : "s"}
                  {item.incompleteCount > 0 ? `, ${item.incompleteCount} incomplete` : ""}
                  {item.withheldCount > 0 ? `, ${item.withheldCount} withheld` : ""}
                </option>
              ))}
            </select>
          </label>
        ) : null}
        {actionButton("ratio")}
        {ratio.state === "not_run" ? (
          <button type="button" className="key-results-link" data-testid="key-results-rule-checks" onClick={() => props.onOpenRuleChecks()}>Rule checks…</button>
        ) : null}
      </div>

      <div className="key-results-group" role="group" aria-label="Deflected shape">
        <h3>Deflected shape</h3>
        {data.cases.length > 0 ? (
          <label className="key-results-choice">
            <span>Case</span>
            <select data-testid="key-results-deflection-case" value={deflectionCase?.key ?? ""} onChange={(event) => setCaseChoice(event.target.value)}>
              {data.cases.map((item) => <option key={item.key} value={item.key}>{item.label}</option>)}
            </select>
          </label>
        ) : null}
        {actionButton("deflection")}
      </div>

      <div className="key-results-group" role="group" aria-label="Supports">
        <h3>Anchors and equipment connections</h3>
        {data.supports.items.length > 0 ? (
          <label className="key-results-choice">
            <span>Support</span>
            <select data-testid="key-results-support-choice" value={support?.supportId ?? ""} onChange={(event) => setSupportChoice(event.target.value)}>
              {data.supports.items.map((item) => <option key={item.supportId} value={item.supportId}>{item.text}</option>)}
            </select>
          </label>
        ) : null}
        {actionButton("support")}
        {data.contact.rowCount > 0 ? (
          <p className="key-results-line" data-testid="key-results-contact-count">
            {data.contact.rowCount} support state row{data.contact.rowCount === 1 ? "" : "s"} over {data.contact.caseCount} case{data.contact.caseCount === 1 ? "" : "s"}.
          </p>
        ) : null}
        {actionButton("contact")}
      </div>

      <div className="key-results-group" role="group" aria-label="Tables and editing">
        <h3>Tables</h3>
        {actionButton("results-table")}
        {actionButton("stress-ratio-table")}
        {actionButton("displacement-extremes")}
        {actionButton("edit")}
      </div>

      <label className="key-results-every-solve">
        <input type="checkbox" data-testid="key-results-every-solve" checked={props.everySolve} onChange={(event) => props.onEverySolveChange(event.target.checked)} />
        Open after every solve (this session)
      </label>
      <p className="key-results-line key-results-boundary" data-testid="key-results-boundary">{KEY_RESULTS_BOUNDARY_TEXT}</p>
      <p className="key-results-line" role="status" aria-live="polite" data-testid="key-results-status">{status}</p>
    </section>
  );
}
