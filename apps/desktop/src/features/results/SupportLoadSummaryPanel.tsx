/** The per-support load summary panel (survey item A2; DEL-07-05 Design
 * `support-load-summary.md` §5, §6). One support at a time: a table of its emitted reactions,
 * resultants and node movements for every case and combination, then the maximum and minimum of
 * each column with the case that holds it. Previous and next step through the model's supports
 * (also Alt+ArrowLeft and Alt+ArrowRight inside the panel).
 *
 * Self-contained: it takes the model, the result and the overlays' standing and derives
 * everything through `buildSupportLoadSummaries`, so another surface (W-17's key-results panel,
 * A6) can embed it with the same props. Values, and the titles that carry them with their
 * result-row ids, follow the report's display rule (`reportQuantityDisplay.ts`: six significant
 * digits, half away from zero, Pa shown as MPa) in the stored units. A view only: it changes no model, result or export. */

import { ChevronLeft, ChevronRight } from "lucide-react";
import { useEffect, useMemo, useRef, useState, type KeyboardEvent } from "react";
import type { MechanicsResult, PreviewModel } from "../../types";
import { displayQuantityParts } from "../report/reportQuantityDisplay";
import type { ResultOverlayStanding } from "./numericalHealth";
import {
  SUPPORT_LOAD_COLUMNS,
  buildSupportLoadSummaries,
  type SupportLoadColumn,
  type SupportLoadRow,
  type SupportLoadSummary,
  type SupportLoadValue
} from "./supportLoadSummary";

export const SUPPORT_LOAD_SUMMARY_TITLE = "Support Load Summary";
export const SUPPORT_LOAD_BOUNDARY_TEXT =
  "Emitted rows in global axes, as the solver states them. Maximum and minimum are taken over the listed solved rows; they mark no limit and qualify nothing.";
export const SUPPORT_LOAD_ENVELOPE_TEXT =
  "Range envelope: each value is selected separately from the envelope's operands, so a row is not one solved state and stays out of the maximum and minimum.";
export const SUPPORT_LOAD_ALLOWABLES_TEXT = "No allowable loads are shown: the support record carries none.";

export type SupportLoadSummaryPanelProps = Readonly<{
  model: PreviewModel;
  result: MechanicsResult | null;
  /** The reader's numerical standing and the session's numerical health view (the overlays' input). */
  standing: ResultOverlayStanding | null | undefined;
  /** When this names a support of the model (for example the session's selection), the panel shows it. */
  focusSupportId?: string | null;
  /** A request count: each new value shows `focusSupportId` again, even when it is unchanged and
   * the user has stepped to another support (A6's "Show support load summary"). */
  focusRequest?: number;
  /** A prefix for the panel's test ids, so two embeddings stay distinct. */
  testIdPrefix?: string;
}>;

const REACTION_COLUMN_COUNT = SUPPORT_LOAD_COLUMNS.filter((item) => item.of === "support").length;

const ABSENCE_TEXT: Readonly<Record<NonNullable<SupportLoadRow["reactionAbsence"]>, string>> = Object.freeze({
  attribution_withheld: "Reaction attribution withheld by the solver",
  no_rows: "No reaction rows",
  frame_not_global: "Reaction rows in another frame; not tabulated"
});

function columnUnit(rows: readonly SupportLoadRow[], item: SupportLoadColumn): string | null {
  const units = new Set(rows.flatMap((row) => (row.values[item.id] ? [row.values[item.id]!.unit] : [])));
  return units.size === 1 ? [...units][0] : null;
}

/** A value's text under the display rule; its title gives the number with its display unit and
 * the result-row id, so no surface of the panel shows a stored value unrounded. */
function ValueText({ value, shareUnit }: { value: SupportLoadValue; shareUnit: boolean }) {
  const parts = displayQuantityParts(value.value, value.unit);
  return <span title={`${parts.number} ${parts.unit} (${value.resultId})`}>{shareUnit ? parts.number : `${parts.number} ${parts.unit}`}</span>;
}

function equipmentText(summary: SupportLoadSummary): string | null {
  const parts: string[] = [];
  if (summary.equipment) {
    const kind = summary.equipment.kind === "equipment_nozzle" ? "equipment nozzle" : "equipment";
    parts.push(`Equipment connection (${kind}): ${summary.equipment.equipmentReference}${summary.equipment.nozzleReference ? `, nozzle ${summary.equipment.nozzleReference}` : ""}`);
  }
  if (summary.equipmentInterfaces.length > 0) parts.push(`Equipment interface at this node: ${summary.equipmentInterfaces.join(", ")}`);
  return parts.length > 0 ? parts.join(". ") : null;
}

function exclusionText(summary: SupportLoadSummary): string {
  const { withheld, envelope, empty } = summary.exclusions;
  const parts = [
    withheld > 0 ? `${withheld} withheld by numerical standing` : null,
    envelope > 0 ? `${envelope} range envelope${envelope === 1 ? "" : "s"}` : null,
    empty > 0 ? `${empty} with no emitted values` : null
  ].filter((part): part is string => part !== null);
  const base = `Maximum and minimum over ${summary.extremeRowCount} of ${summary.rows.length} rows`;
  return parts.length > 0 ? `${base}; not included: ${parts.join(", ")}.` : `${base}.`;
}

export function SupportLoadSummaryPanel({ model, result, standing, focusSupportId = null, focusRequest = 0, testIdPrefix = "support-load" }: SupportLoadSummaryPanelProps) {
  const summaries = useMemo(() => buildSupportLoadSummaries({ model, result, standing }), [model, result, standing]);
  const supports = summaries.supports;
  const [chosenId, setChosenId] = useState<string | null>(null);
  // A focused support (the session's selection) moves the panel to it once each time the focus
  // or the focus request changes; a new result or standing does not undo the user's stepping.
  const focusKey = focusSupportId ? JSON.stringify([focusSupportId, focusRequest]) : null;
  const appliedFocus = useRef<string | null>(null);
  useEffect(() => {
    if (focusKey === appliedFocus.current) return;
    if (!focusKey) appliedFocus.current = null;
    else if (supports.some((item) => item.supportId === focusSupportId)) {
      appliedFocus.current = focusKey;
      setChosenId(focusSupportId);
    }
  }, [focusKey, focusSupportId, supports]);
  const index = Math.max(0, supports.findIndex((item) => item.supportId === chosenId));
  const summary = supports[index] ?? null;
  const step = (delta: number) => {
    const next = supports[index + delta];
    if (next) setChosenId(next.supportId);
  };
  const onKeyDown = (event: KeyboardEvent<HTMLElement>) => {
    if (!event.altKey || (event.key !== "ArrowLeft" && event.key !== "ArrowRight")) return;
    event.preventDefault();
    step(event.key === "ArrowRight" ? 1 : -1);
  };
  const health = standing?.health ?? null;
  const tid = (suffix: string) => `${testIdPrefix}-${suffix}`;

  return (
    <section className="result-detail-panel support-load-summary" aria-label="Support load summary" data-testid={tid("summary")} onKeyDown={onKeyDown}>
      <h3>{SUPPORT_LOAD_SUMMARY_TITLE}</h3>
      {summaries.state === "unavailable" || !summary ? (
        <p className="muted" data-testid={tid("unavailable")}>{summaries.reason ?? "No supports"}</p>
      ) : (
        <>
          <div className="support-load-stepper" role="group" aria-label="Support">
            <button type="button" aria-label="Previous support" aria-keyshortcuts="Alt+ArrowLeft" data-testid={tid("previous")} disabled={index === 0} onClick={() => step(-1)}>
              <ChevronLeft size={14} aria-hidden="true" />
            </button>
            <select aria-label="Support" data-testid={tid("select")} value={summary.supportId} onChange={(event) => setChosenId(event.target.value)}>
              {supports.map((item) => <option key={item.supportId} value={item.supportId}>{item.label === item.supportId ? item.supportId : `${item.label} (${item.supportId})`}</option>)}
            </select>
            <button type="button" aria-label="Next support" aria-keyshortcuts="Alt+ArrowRight" data-testid={tid("next")} disabled={index >= supports.length - 1} onClick={() => step(1)}>
              <ChevronRight size={14} aria-hidden="true" />
            </button>
            <span aria-live="polite" data-testid={tid("position")}>Support {index + 1} of {supports.length}</span>
          </div>
          <p data-testid={tid("context")}>
            Node {summary.nodeId}. Frame: global. Sign convention: {summary.signConvention ?? "no reaction rows listed"}.
            {equipmentText(summary) ? <> <span data-testid={tid("equipment")}>{equipmentText(summary)}.</span></> : null}
          </p>
          {health ? <p data-testid={tid("standing")}>{health.headline}{health.outputSentence ? `. ${health.outputSentence}` : ""}</p> : null}
          <SupportLoadTable summary={summary} tid={tid} />
          <p data-testid={tid("extremes-note")}>{exclusionText(summary)}</p>
          {summary.rows.some((row) => row.case.envelope && row.standing === "admitted") ? <small data-testid={tid("envelope-note")}>{SUPPORT_LOAD_ENVELOPE_TEXT}</small> : null}
          <small data-testid={tid("allowables")}>{SUPPORT_LOAD_ALLOWABLES_TEXT}</small>
          <small>{SUPPORT_LOAD_BOUNDARY_TEXT}</small>
        </>
      )}
    </section>
  );
}

function SupportLoadTable({ summary, tid }: { summary: SupportLoadSummary; tid: (suffix: string) => string }) {
  const listed = summary.rows.filter((row) => row.standing === "admitted");
  const units = new Map(SUPPORT_LOAD_COLUMNS.map((item) => [item.id, columnUnit(listed, item)] as const));
  const header = (item: SupportLoadColumn) => {
    const unit = units.get(item.id);
    return unit ? `${item.label} (${displayQuantityParts(0, unit).unit})` : item.label;
  };
  const extremes = new Map(summary.extremes.map((item) => [item.column, item] as const));
  return (
    <div className="support-load-table-scroll">
      <table aria-label={`Loads and movements at support ${summary.label}`} data-testid={tid("table")}>
        <thead>
          <tr>
            <th scope="col">Case</th>
            {SUPPORT_LOAD_COLUMNS.map((item) => <th key={item.id} scope="col" data-testid={tid(`column-${item.id}`)}>{header(item)}</th>)}
          </tr>
        </thead>
        <tbody>
          {summary.rows.map((row) => (
            <tr key={row.case.key} data-testid={tid(`row-${row.case.key}`)} data-standing={row.standing}>
              <th scope="row">{row.case.label}{row.case.envelope ? <small> range envelope</small> : null}</th>
              {row.standing === "withheld" ? (
                <td colSpan={SUPPORT_LOAD_COLUMNS.length} data-testid={tid(`withheld-${row.case.key}`)}>{row.withheldReason}</td>
              ) : (
                <>
                  {row.reactionAbsence ? (
                    <td colSpan={REACTION_COLUMN_COUNT} data-testid={tid(`absence-${row.case.key}`)}>
                      {ABSENCE_TEXT[row.reactionAbsence]}{row.reactionAbsenceDetail ? ` (${row.reactionAbsenceDetail})` : ""}
                    </td>
                  ) : (
                    SUPPORT_LOAD_COLUMNS.filter((item) => item.of === "support").map((item) => <ValueCell key={item.id} row={row} item={item} shareUnit={units.get(item.id) !== null} tid={tid} />)
                  )}
                  {SUPPORT_LOAD_COLUMNS.filter((item) => item.of === "node").map((item) => <ValueCell key={item.id} row={row} item={item} shareUnit={units.get(item.id) !== null} tid={tid} />)}
                </>
              )}
            </tr>
          ))}
        </tbody>
        <tbody className="support-load-extremes">
          {(["max", "min"] as const).map((which) => (
            <tr key={which} data-testid={tid(`extreme-${which}`)}>
              <th scope="row">{which === "max" ? "Maximum" : "Minimum"}</th>
              {SUPPORT_LOAD_COLUMNS.map((item) => {
                const extreme = extremes.get(item.id);
                const value = extreme?.[which] ?? null;
                return (
                  <td key={item.id} data-testid={tid(`extreme-${which}-${item.id}`)}>
                    {value ? <><ValueText value={value} shareUnit={units.get(item.id) !== null} /><br /><small>{value.caseLabel}</small></> : extreme?.note ? <small>{extreme.note}</small> : <Dash label="none" />}
                  </td>
                );
              })}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function ValueCell({ row, item, shareUnit, tid }: { row: SupportLoadRow; item: SupportLoadColumn; shareUnit: boolean; tid: (suffix: string) => string }) {
  const value = row.values[item.id];
  return <td data-testid={tid(`cell-${row.case.key}-${item.id}`)}>{value ? <ValueText value={value} shareUnit={shareUnit} /> : <Dash label="not emitted" />}</td>;
}

/** An empty cell: a dash on screen, a word for a screen reader. */
function Dash({ label }: { label: string }) {
  return <span title={label}><span aria-hidden="true">—</span><span className="visually-hidden">{label}</span></span>;
}
