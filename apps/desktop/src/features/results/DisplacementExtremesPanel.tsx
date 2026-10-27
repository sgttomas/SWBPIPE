/** The displacement extremes panel (survey item A8; DEL-07-05 Design `displacement-extremes.md`
 * §5 to §7). Per node, the largest and smallest emitted translation and rotation of each component
 * over the chosen cases, with the governing case. A filter lists the nodes of one support kind, a
 * sort orders nodes by one component's largest magnitude, and the table is paged so a large model
 * renders one page. Each node's row header selects the node through the session's selection.
 *
 * Self-contained: it takes the model, the result and the overlays' standing and derives everything
 * through `buildDisplacementExtremes` and `displacementExtremesView`, so another surface (W-17's
 * key-results panel, A6) can embed it with the same props. Values, and the titles that carry them,
 * follow the report's display rule (`reportQuantityDisplay.ts`: six significant digits, half away
 * from zero, Pa shown as MPa) in the stored units. A view only: it changes no model, result or
 * export, and it persists and sends nothing. */

import { ChevronLeft, ChevronRight } from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import type { MechanicsResult, PreviewModel } from "../../types";
import { displayQuantityParts } from "../report/reportQuantityDisplay";
import type { ResultOverlayStanding } from "./numericalHealth";
import { SUPPORT_LOAD_ENVELOPE_TEXT } from "./SupportLoadSummaryPanel";
import {
  DISPLACEMENT_COMPONENTS,
  DISPLACEMENT_ENVELOPE_REASON,
  FILTER_ALL,
  buildDisplacementExtremes,
  displacementExtremesView,
  type DisplacementExtremeValue,
  type DisplacementExtremes,
  type DisplacementExtremesView,
  type DisplacementNodeExtremes,
  type DisplacementSort
} from "./displacementExtremes";

export const DISPLACEMENT_EXTREMES_TITLE = "Displacement Extremes";
export const DISPLACEMENT_EXTREMES_PAGE = 50;
export const DISPLACEMENT_EXTREMES_NO_CASE = "No admitted case is chosen; no extreme is formed.";
export const DISPLACEMENT_EXTREMES_BOUNDARY_TEXT =
  "Emitted global translations and rotations of the chosen solved cases. Maximum and minimum are arithmetic over those values; they check no clearance or travel, and the product supplies no limit.";

export type DisplacementExtremesPanelProps = Readonly<{
  model: PreviewModel;
  result: MechanicsResult | null;
  /** The reader's numerical standing and the session's numerical health view (the overlays' input). */
  standing: ResultOverlayStanding | null | undefined;
  /** The session's selected node: its row is marked, and its page is shown when it changes. */
  selectedNodeId?: string | null;
  /** Selects a node through the session's selection. */
  onSelectNode?: (nodeId: string) => void;
  /** A prefix for the panel's test ids, so two embeddings stay distinct. */
  testIdPrefix?: string;
}>;

/** A value under the display rule; its title carries the same number and unit, the case and the row id. */
function ValueText({ value, shareUnit }: { value: DisplacementExtremeValue; shareUnit: boolean }) {
  const parts = displayQuantityParts(value.value, value.unit);
  return <span title={`${parts.number} ${parts.unit}, ${value.caseLabel} (${value.resultId})`}>{shareUnit ? parts.number : `${parts.number} ${parts.unit}`}</span>;
}

/** An empty cell: a dash on screen, a word for a screen reader. */
function Dash({ label }: { label: string }) {
  return <span title={label}><span aria-hidden="true">—</span><span className="visually-hidden">{label}</span></span>;
}

function exclusionText(view: DisplacementExtremesView, total: number): string {
  const { withheld, envelope, notChosen } = view.exclusions;
  const parts = [
    withheld > 0 ? `${withheld} withheld by numerical standing (${view.exclusions.withheldRows} rows)` : null,
    envelope > 0 ? `${envelope} range envelope${envelope === 1 ? "" : "s"}` : null,
    notChosen > 0 ? `${notChosen} not chosen` : null
  ].filter((part): part is string => part !== null);
  const base = `Maximum and minimum over ${view.chosenCases.length} of ${total} case${total === 1 ? "" : "s"}`;
  return parts.length > 0 ? `${base}; not included: ${parts.join(", ")}.` : `${base}.`;
}

function unusedText(data: DisplacementExtremes): string | null {
  const { otherFrame, notModelNode } = data.unusedRows;
  const parts = [
    otherFrame > 0 ? `${otherFrame} in another frame` : null,
    notModelNode > 0 ? `${notModelNode} of nodes not in the model` : null
  ].filter((part): part is string => part !== null);
  return parts.length > 0 ? `Movement rows not used: ${parts.join(", ")}.` : null;
}

export function DisplacementExtremesPanel({
  model, result, standing, selectedNodeId = null, onSelectNode, testIdPrefix = "displacement-extremes"
}: DisplacementExtremesPanelProps) {
  const data = useMemo(() => buildDisplacementExtremes({ model, result, standing }), [model, result, standing]);
  const [unchosen, setUnchosen] = useState<ReadonlySet<string>>(() => new Set());
  const [filter, setFilter] = useState(FILTER_ALL);
  const [sort, setSort] = useState<DisplacementSort>("model");
  const [page, setPage] = useState(0);
  const caseKeys = useMemo(() => data.cases.filter((item) => item.choosable && !unchosen.has(item.key)).map((item) => item.key), [data, unchosen]);
  const view = useMemo(() => displacementExtremesView(data, { caseKeys, filter, sort }), [data, caseKeys, filter, sort]);
  const pageCount = Math.max(1, Math.ceil(view.rows.length / DISPLACEMENT_EXTREMES_PAGE));
  const shownPage = Math.min(page, pageCount - 1);
  // A new filter, sort or case choice lists from the first page again.
  useEffect(() => setPage(0), [filter, sort, caseKeys]);
  // The session's selected node moves the panel to its page once each time the selection changes.
  const appliedSelection = useRef<string | null>(null);
  useEffect(() => {
    if (selectedNodeId === appliedSelection.current) return;
    appliedSelection.current = selectedNodeId;
    const at = selectedNodeId ? view.rows.findIndex((row) => row.nodeId === selectedNodeId) : -1;
    if (at >= 0) setPage(Math.floor(at / DISPLACEMENT_EXTREMES_PAGE));
  }, [selectedNodeId, view]);
  const tid = (suffix: string) => `${testIdPrefix}-${suffix}`;
  const health = standing?.health ?? null;
  const first = shownPage * DISPLACEMENT_EXTREMES_PAGE;
  const rows = view.rows.slice(first, first + DISPLACEMENT_EXTREMES_PAGE);
  const unused = unusedText(data);

  return (
    <section className="result-detail-panel displacement-extremes" aria-label="Displacement extremes" data-testid={tid("panel")}>
      <h3>{DISPLACEMENT_EXTREMES_TITLE}</h3>
      {data.state === "unavailable" ? (
        <p className="muted" data-testid={tid("unavailable")}>{data.reason}</p>
      ) : (
        <>
          {health ? <p data-testid={tid("standing")}>{health.headline}{health.outputSentence ? `. ${health.outputSentence}` : ""}</p> : null}
          <div className="displacement-extremes-controls">
            <label>
              Nodes{" "}
              <select data-testid={tid("filter")} value={view.filter.id} onChange={(event) => setFilter(event.target.value)}>
                {data.filters.map((item) => <option key={item.id} value={item.id}>{item.label} ({item.nodeCount})</option>)}
              </select>
            </label>
            <label>
              Sort{" "}
              <select data-testid={tid("sort")} value={sort} onChange={(event) => setSort(event.target.value as DisplacementSort)}>
                <option value="model">Node order</option>
                {DISPLACEMENT_COMPONENTS.map((item) => <option key={item.id} value={item.id}>Largest |{item.label}|</option>)}
              </select>
            </label>
          </div>
          <fieldset className="displacement-extremes-cases" data-testid={tid("cases")}>
            <legend>Cases</legend>
            {data.cases.map((item) => {
              const reason = item.standing === "withheld" ? item.withheldReason : item.envelope ? DISPLACEMENT_ENVELOPE_REASON : null;
              const reasonId = tid(`case-reason-${item.key}`);
              return (
                <label key={item.key} data-standing={item.standing}>
                  <input
                    type="checkbox"
                    data-testid={tid(`case-${item.key}`)}
                    checked={item.choosable && !unchosen.has(item.key)}
                    disabled={!item.choosable}
                    aria-describedby={reason ? reasonId : undefined}
                    onChange={(event) => setUnchosen((current) => {
                      const next = new Set(current);
                      if (event.target.checked) next.delete(item.key);
                      else next.add(item.key);
                      return next;
                    })}
                  />
                  {item.label}
                  {reason ? <small id={reasonId} data-testid={reasonId}> {reason}{item.standing === "withheld" ? ` (${item.rowCount} rows)` : ""}</small> : null}
                </label>
              );
            })}
            <button type="button" data-testid={tid("cases-all")} disabled={unchosen.size === 0} onClick={() => setUnchosen(new Set())}>All cases</button>
          </fieldset>
          <p data-testid={tid("exclusions")}>{exclusionText(view, data.cases.length)}</p>
          {data.cases.some((item) => item.envelope && item.standing === "admitted") ? <small data-testid={tid("envelope-note")}>{SUPPORT_LOAD_ENVELOPE_TEXT}</small> : null}
          {unused ? <small data-testid={tid("unused")}>{unused}</small> : null}
          {view.sort.note ? <p data-testid={tid("sort-note")}>{view.sort.note}</p> : null}
          {view.chosenCases.length === 0 ? (
            <p data-testid={tid("no-case")}>{DISPLACEMENT_EXTREMES_NO_CASE}</p>
          ) : (
            <>
              <ExtremesTable view={view} rows={rows} selectedNodeId={selectedNodeId} onSelectNode={onSelectNode} tid={tid} />
              <div className="displacement-extremes-pager" role="group" aria-label="Pages">
                <button type="button" aria-label="Previous page" data-testid={tid("page-previous")} disabled={shownPage === 0} onClick={() => setPage(shownPage - 1)}>
                  <ChevronLeft size={14} aria-hidden="true" />
                </button>
                <span aria-live="polite" data-testid={tid("page")}>
                  {view.rows.length === 0 ? "No nodes" : `Nodes ${first + 1}–${first + rows.length} of ${view.rows.length}`}
                </span>
                <button type="button" aria-label="Next page" data-testid={tid("page-next")} disabled={shownPage >= pageCount - 1} onClick={() => setPage(shownPage + 1)}>
                  <ChevronRight size={14} aria-hidden="true" />
                </button>
              </div>
            </>
          )}
          <small>{DISPLACEMENT_EXTREMES_BOUNDARY_TEXT}</small>
        </>
      )}
    </section>
  );
}

function ExtremesTable({ view, rows, selectedNodeId, onSelectNode, tid }: {
  view: DisplacementExtremesView;
  rows: readonly DisplacementNodeExtremes[];
  selectedNodeId: string | null;
  onSelectNode?: (nodeId: string) => void;
  tid: (suffix: string) => string;
}) {
  const header = (id: (typeof DISPLACEMENT_COMPONENTS)[number]["id"], label: string) => {
    const unit = view.units[id];
    return unit ? `${label} (${displayQuantityParts(0, unit).unit})` : label;
  };
  return (
    <div className="displacement-extremes-table-scroll">
      <table aria-label={`Displacement and rotation extremes: ${view.filter.label}`} data-testid={tid("table")} data-filter={view.filter.id}>
        <thead>
          <tr>
            <th scope="col" rowSpan={2}>Node</th>
            <th scope="col" rowSpan={2}>Supports</th>
            {DISPLACEMENT_COMPONENTS.map((item) => (
              <th key={item.id} scope="colgroup" colSpan={2} data-testid={tid(`column-${item.id}`)}
                aria-sort={view.sort.component === item.id ? (view.sort.sorted ? "descending" : "none") : undefined}>
                {header(item.id, item.label)}
              </th>
            ))}
          </tr>
          <tr>
            {DISPLACEMENT_COMPONENTS.flatMap((item) => [
              <th key={`${item.id}-max`} scope="col">Max</th>,
              <th key={`${item.id}-min`} scope="col">Min</th>
            ])}
          </tr>
        </thead>
        <tbody>
          {rows.map((row) => {
            const selected = selectedNodeId !== null && row.nodeId === selectedNodeId;
            const name = row.label === row.nodeId ? row.nodeId : `${row.label} (${row.nodeId})`;
            return (
              <tr key={row.nodeId} data-testid={tid(`row-${row.nodeId}`)} data-ranked={row.key === null ? "false" : "true"}
                aria-current={selected ? "true" : undefined} className={selected ? "selected-result-row" : undefined}>
                <th scope="row">
                  {onSelectNode ? (
                    <button type="button" className="displacement-extremes-node" aria-label={`Select node ${name}`} onClick={() => onSelectNode(row.nodeId)}>{name}</button>
                  ) : name}
                </th>
                <td className="displacement-extremes-supports">
                  {row.supports.length > 0 ? row.supports.map((item) => `${item.label}: ${item.kindText}`).join("; ") : <Dash label="no support" />}
                </td>
                {DISPLACEMENT_COMPONENTS.flatMap((item) => {
                  const extreme = row.extremes[item.id];
                  const shareUnit = view.units[item.id] !== null;
                  return (["max", "min"] as const).map((which) => {
                    const value = extreme[which];
                    return (
                      <td key={`${item.id}-${which}`} data-testid={tid(`cell-${row.nodeId}-${item.id}-${which}`)}>
                        {value ? <><ValueText value={value} shareUnit={shareUnit} /><br /><small>{value.caseLabel}</small></>
                          : extreme.note ? <small>{extreme.note}</small> : <Dash label="none" />}
                      </td>
                    );
                  });
                })}
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
