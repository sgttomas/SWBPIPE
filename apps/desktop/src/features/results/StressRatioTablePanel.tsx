/** The sorted stress and ratio table (survey item A3; DEL-07-05 Design `stress-ratio-table.md`
 * §5, §7, §8). One category at a time: the user's rule-check outcomes of one of the pack's stress
 * categories, sorted by ratio (each row stating its direction, §3.1) or by computed value, or one
 * emitted stress kind sorted by magnitude.
 * A threshold shows only the rows at or above it; it never hides an incomplete, withheld or failed
 * check (FR-RULE-003). Each row's location button highlights it through the session's selection.
 *
 * Self-contained: it takes the model, the result, the overlays' standing and the session's
 * rule-check run and derives everything through `buildStressRatioTable` and `stressRatioView`, so
 * another surface (W-17's key-results panel, A6) can embed it with the same props. Values, and the
 * titles that carry them, follow the report's display rule (`reportQuantityDisplay.ts`: six
 * significant digits, half away from zero, Pa shown as MPa) in the stored units. A view only: it
 * changes no model, result, run or export, and it persists and sends nothing. */

import { useEffect, useMemo, useState } from "react";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { rulePackReference, type SessionRuleCheckRun } from "../report/renderableReportInput";
import { displayQuantityParts } from "../report/reportQuantityDisplay";
import { CsvExportControl } from "../result-export/CsvExportControl";
import { CSV_SOFTWARE_VERSION, CSV_UNIT_BASIS_DISPLAY_RULE, csvResultProvenance, csvUnitBasis } from "../result-export/csvProvenance";
import {
  RATIO_DIRECTION_TEXT,
  STRESS_RATIO_CSV_KIND,
  STRESS_RATIO_NONE,
  STRESS_RATIO_NOT_RANKED,
  STRESS_RATIO_NO_LOCATION,
  STRESS_RATIO_NO_RATIO,
  STRESS_RATIO_STATUS_TEXT as STATUS_TEXT,
  STRESS_RATIO_WITHHELD,
  csvRulePackRef,
  stressRatioCsv
} from "./stressRatioTableCsv";
import type { ResultOverlayStanding } from "./numericalHealth";
import {
  THRESHOLD_INVALID_NOTE,
  buildStressRatioTable,
  ratioDirection,
  stressRatioLocationText,
  stressRatioPage,
  stressRatioView,
  type RatioDirection,
  type StressRatioGroup,
  type StressRatioMode,
  type StressRatioPage,
  type StressRatioRow,
  type StressRatioTable,
  type StressRatioTarget,
  type StressRatioValue,
  type StressRatioView
} from "./stressRatioTable";

export const STRESS_RATIO_TITLE = "Stress and Ratio Table";
export const STRESS_RATIO_PAGE = 50;
export const STRESS_RATIO_PINNED_TEXT =
  "Incomplete and withheld checks are listed first. They are never sorted, and the threshold never hides them. A failed check is listed at its rank on every page, and the threshold never hides it.";
export const STRESS_RATIO_BOUNDARY_TEXT =
  "Ratios, limits and statuses are your rule pack's own outcomes, shown in this session only. The product supplies no allowable; this table sorts and filters what is stated and qualifies nothing.";
export const STRESS_RATIO_KEY_TEXT: Readonly<Record<"ratio" | "rule_stress" | "emitted_stress", string>> = Object.freeze({
  ratio: "Ratio = your rule's computed value ÷ its limit under a less-than relation, or its limit ÷ its computed value under a greater-than relation; the Direction column says which. It is formed only with both in one unit, a positive limit and, under greater-than, a positive computed value. In either direction a ratio above 1 means the computed value is past your limit. Largest first.",
  rule_stress: "Sorted by the magnitude of your rule's computed value, largest first.",
  emitted_stress: "Sorted by the magnitude of the emitted stress, largest first; values are shown signed."
});
/** The key's text when the keyed rows' units differ and the rows keep source order (§5, Units). */
export const STRESS_RATIO_KEY_TEXT_UNSORTED: Readonly<Record<"rule_stress" | "emitted_stress", string>> = Object.freeze({
  rule_stress: "Your rule's computed values, in run order: their units differ, so they are not sorted.",
  emitted_stress: "Emitted stresses, in result-row order: their units differ, so they are not sorted; values are shown signed."
});

function keyText(view: StressRatioView, group: StressRatioGroup): string {
  if (view.mode === "ratio") return STRESS_RATIO_KEY_TEXT.ratio;
  const source = group.source === "rule_category" ? "rule_stress" : "emitted_stress";
  return view.sorted ? STRESS_RATIO_KEY_TEXT[source] : STRESS_RATIO_KEY_TEXT_UNSORTED[source];
}

/** The polite live line: rows listed on this page, rows hidden by the threshold, rows a later page lists. */
export function stressRatioSummaryText(view: StressRatioView, page: StressRatioPage): string {
  const hidden = view.hiddenCount > 0 ? `; ${view.hiddenCount} below the threshold hidden` : "";
  const later = page.notListedCount > 0 ? `; ${page.notListedCount} more not yet listed` : "";
  return `Showing ${page.listedCount} of ${view.totalCount} rows${hidden}${later}.`;
}
/** Each ratio row's direction (§3.1), in the Direction column (defined beside the CSV adapter). */
export { RATIO_DIRECTION_TEXT };
/** The formula in a ratio's title. */
const RATIO_FORMULA: Readonly<Record<RatioDirection, string>> = Object.freeze({ upper: "computed ÷ limit", lower: "limit ÷ computed" });
export const RATIO_DISABLED_NO_RUN = "No rule-check run in this session";
export const RATIO_DISABLED_EMITTED = "Emitted stress: no rule outcome";


export type StressRatioTablePanelProps = Readonly<{
  model: PreviewModel;
  result: MechanicsResult | null;
  /** The result's analysis-run record, for the CSV preamble's result hash (RT-R6). */
  analysisRun?: AnalysisRunEnvelope | null;
  /** The reader's numerical standing and the session's numerical health view (the overlays' input). */
  standing: ResultOverlayStanding | null | undefined;
  /** The session's rule-check run, only while it is a run of `result`. */
  ruleCheckRun?: SessionRuleCheckRun | null;
  /** The session's selected result row, marked in the table. */
  selectedResultId?: string | null;
  /** Highlights a row's location: an emitted row, or else an entity. */
  onSelectLocation?: (target: StressRatioTarget) => void;
  /** A prefix for the panel's test ids, so two embeddings stay distinct. */
  testIdPrefix?: string;
}>;

/** A value under the display rule; its title carries the same number and unit and the row's id. */
function ValueText({ value, id, shareUnit }: { value: StressRatioValue; id: string; shareUnit: boolean }) {
  const parts = displayQuantityParts(value.value, value.unit);
  return <span title={`${parts.number} ${parts.unit} (${id})`}>{shareUnit ? parts.number : `${parts.number} ${parts.unit}`}</span>;
}

/** An empty cell: a dash on screen, a word for a screen reader. */
function Dash({ label }: { label: string }) {
  return <span title={label}><span aria-hidden="true">—</span><span className="visually-hidden">{label}</span></span>;
}

function sharedUnit(values: readonly (StressRatioValue | null)[]): string | null {
  const units = new Set(values.flatMap((value) => (value ? [value.unit] : [])));
  return units.size === 1 ? [...units][0] : null;
}

function headerWithUnit(label: string, unit: string | null): string {
  return unit ? `${label} (${displayQuantityParts(0, unit).unit})` : label;
}

export function StressRatioTablePanel({
  model, result, analysisRun = null, standing, ruleCheckRun = null, selectedResultId = null, onSelectLocation, testIdPrefix = "stress-ratio"
}: StressRatioTablePanelProps) {
  const table = useMemo(() => buildStressRatioTable({ model, result, standing, ruleCheckRun }), [model, result, standing, ruleCheckRun]);
  const [chosenId, setChosenId] = useState<string | null>(null);
  const [chosenMode, setChosenMode] = useState<StressRatioMode>("ratio");
  const [thresholds, setThresholds] = useState<Readonly<Record<StressRatioMode, string>>>({ ratio: "", stress: "" });
  const [limit, setLimit] = useState(STRESS_RATIO_PAGE);
  const group = table.groups.find((item) => item.id === chosenId) ?? table.groups[0] ?? null;
  const view = useMemo(
    () => (group ? stressRatioView(group, { mode: chosenMode, threshold: thresholds[group.modes.includes(chosenMode) ? chosenMode : group.modes[0]] }) : null),
    [group, chosenMode, thresholds]
  );
  // A new category, mode or threshold lists from the top again.
  useEffect(() => setLimit(STRESS_RATIO_PAGE), [group?.id, view?.mode, thresholds]);
  const page = useMemo(() => (view ? stressRatioPage(view, limit) : null), [view, limit]);
  const tid = (suffix: string) => `${testIdPrefix}-${suffix}`;
  const health = standing?.health ?? null;

  return (
    <section className="result-detail-panel stress-ratio-table" aria-label="Stress and ratio table" data-testid={tid("panel")}>
      <h3>{STRESS_RATIO_TITLE}</h3>
      <p data-testid={tid("run")}>{table.run.note}</p>
      {table.state === "unavailable" || !group || !view || !page ? (
        <p className="muted" data-testid={tid("unavailable")}>{table.reason ?? "Nothing to list"}</p>
      ) : (
        <>
          <Controls
            table={table}
            group={group}
            view={view}
            threshold={thresholds[view.mode]}
            onGroup={setChosenId}
            onMode={setChosenMode}
            onThreshold={(text) => setThresholds((current) => ({ ...current, [view.mode]: text }))}
            tid={tid}
          />
          {health ? <p data-testid={tid("standing")}>{health.headline}{health.outputSentence ? `. ${health.outputSentence}` : ""}</p> : null}
          <small data-testid={tid("key")}>{keyText(view, group)}</small>
          {view.unitNote ? <p data-testid={tid("unit-note")}>{view.unitNote}</p> : null}
          {group.source === "rule_category" ? <small data-testid={tid("pinned-note")}>{STRESS_RATIO_PINNED_TEXT}</small> : null}
          <CsvExportControl
            tableLabel={group.label}
            testId={tid("export-csv")}
            build={async () => stressRatioCsv({
              group,
              view,
              provenance: await csvResultProvenance({ model, result, analysisRun }),
              rulePacks: table.run.state === "used" && ruleCheckRun ? [csvRulePackRef(await rulePackReference(ruleCheckRun))] : [],
              softwareVersion: CSV_SOFTWARE_VERSION,
              unitBasis: csvUnitBasis(model, CSV_UNIT_BASIS_DISPLAY_RULE),
              threshold: thresholds[view.mode]
            })}
          />
          <RowsTable group={group} view={view} page={page} selectedResultId={selectedResultId} onSelectLocation={onSelectLocation} tid={tid} />
          <p aria-live="polite" data-testid={tid("summary")}>{stressRatioSummaryText(view, page)}</p>
          {page.notListedCount > 0 ? (
            <button type="button" data-testid={tid("more")} onClick={() => setLimit((current) => current + STRESS_RATIO_PAGE)}>
              Show {STRESS_RATIO_PAGE} more
            </button>
          ) : null}
          <small>{STRESS_RATIO_BOUNDARY_TEXT}</small>
        </>
      )}
    </section>
  );
}

function Controls({ table, group, view, threshold, onGroup, onMode, onThreshold, tid }: {
  table: StressRatioTable;
  group: StressRatioGroup;
  view: StressRatioView;
  threshold: string;
  onGroup: (id: string) => void;
  onMode: (mode: StressRatioMode) => void;
  onThreshold: (text: string) => void;
  tid: (suffix: string) => string;
}) {
  const rules = table.groups.filter((item) => item.source === "rule_category");
  const stresses = table.groups.filter((item) => item.source === "emitted_stress");
  const ratioDisabledReason = group.modes.includes("ratio") ? null : table.run.state === "used" ? RATIO_DISABLED_EMITTED : RATIO_DISABLED_NO_RUN;
  const unit = view.keyUnit ? displayQuantityParts(0, view.keyUnit).unit : null;
  const disabled = view.threshold.state === "disabled";
  return (
    <div className="stress-ratio-controls">
      <label>
        Category{" "}
        <select data-testid={tid("group")} value={group.id} onChange={(event) => onGroup(event.target.value)}>
          {rules.length > 0 ? (
            <optgroup label="Your rule pack's stress categories">
              {rules.map((item) => <option key={item.id} value={item.id}>{item.label}</option>)}
            </optgroup>
          ) : null}
          {stresses.length > 0 ? (
            <optgroup label="Emitted stresses">
              {stresses.map((item) => <option key={item.id} value={item.id}>{item.label}</option>)}
            </optgroup>
          ) : null}
        </select>
      </label>
      <fieldset className="stress-ratio-mode" data-testid={tid("mode")} data-mode={view.mode}>
        <legend>Sort and show by</legend>
        {(["ratio", "stress"] as const).map((mode) => (
          <label key={mode}>
            <input
              type="radio"
              name={tid("mode")}
              value={mode}
              data-testid={tid(`mode-${mode}`)}
              checked={view.mode === mode}
              disabled={!group.modes.includes(mode)}
              aria-describedby={mode === "ratio" && ratioDisabledReason ? tid("mode-reason") : undefined}
              onChange={() => onMode(mode)}
            />
            {mode === "ratio" ? "Ratio" : "Stress"}
          </label>
        ))}
        {ratioDisabledReason ? <small id={tid("mode-reason")} data-testid={tid("mode-reason")}>{ratioDisabledReason}</small> : null}
      </fieldset>
      <label>
        Show at or above{" "}
        <input
          type="text"
          inputMode="decimal"
          size={8}
          data-testid={tid("threshold")}
          value={threshold}
          disabled={disabled}
          aria-describedby={tid("threshold-note")}
          onChange={(event) => onThreshold(event.target.value)}
        />
        {unit ? ` ${unit}` : null}
      </label>
      <small id={tid("threshold-note")} data-testid={tid("threshold-note")}>
        {view.threshold.state === "invalid" ? THRESHOLD_INVALID_NOTE
          : view.threshold.state === "disabled" ? view.threshold.reason
          : view.threshold.state === "applied" ? "Rows below the threshold are hidden."
          : "No threshold."}
      </small>
    </div>
  );
}

function RowsTable({ group, view, page, selectedResultId, onSelectLocation, tid }: {
  group: StressRatioGroup;
  view: StressRatioView;
  page: StressRatioPage;
  selectedResultId: string | null;
  onSelectLocation?: (target: StressRatioTarget) => void;
  tid: (suffix: string) => string;
}) {
  const rule = group.source === "rule_category";
  // Failed outcomes are listed at their rank on every page (`stressRatioPage`, §7).
  const shown = page.ranked;
  const listed = [...view.pinned, ...shown.map((item) => item.row), ...view.unranked];
  const stressUnit = sharedUnit(listed.map((row) => row.stress));
  const computedUnit = sharedUnit(listed.map((row) => row.outcome?.computed ?? null));
  const limitUnit = sharedUnit(listed.map((row) => row.outcome?.limit ?? null));
  const valueColumns = !rule ? 1 : view.mode === "ratio" ? 3 : 2;
  const keyLabel = !rule ? "stress magnitude" : view.mode === "ratio" ? "ratio" : "computed value magnitude";
  const sort = view.sorted ? "descending" : "none";
  const tableLabel = view.sorted ? `${group.label}: sorted by ${keyLabel}` : `${group.label}: in source order, ${keyLabel} not sorted (units differ)`;
  return (
    <div className="stress-ratio-table-scroll">
      <table aria-label={tableLabel} data-testid={tid("table")} data-group={group.id} data-csv-export={STRESS_RATIO_CSV_KIND}>
        <thead>
          <tr>
            <th scope="col">{view.sorted ? "Rank" : "Position"}</th>
            <th scope="col">Location</th>
            <th scope="col">Case</th>
            {rule ? <><th scope="col">Check</th><th scope="col">Status</th></> : null}
            {!rule ? <th scope="col" aria-sort={sort} data-testid={tid("column-stress")}>{headerWithUnit("Stress", stressUnit)}</th> : null}
            {rule && view.mode === "ratio" ? (
              <>
                <th scope="col" aria-sort={sort} data-testid={tid("column-ratio")}>Ratio</th>
                <th scope="col" data-testid={tid("column-direction")}>Direction</th>
                <th scope="col">Relation</th>
              </>
            ) : null}
            {rule && view.mode === "stress" ? (
              <>
                <th scope="col" aria-sort={sort} data-testid={tid("column-computed")}>{headerWithUnit("Computed", computedUnit)}</th>
                <th scope="col" data-testid={tid("column-limit")}>{headerWithUnit("Limit", limitUnit)}</th>
              </>
            ) : null}
          </tr>
        </thead>
        <tbody>
          {group.withheldCases.map((item) => (
            <tr key={`withheld:${item.case.key}`} data-testid={tid(`withheld-${item.case.key}`)} data-part="pinned" data-standing="withheld">
              <th scope="row"><Dash label={STRESS_RATIO_NOT_RANKED} /></th>
              <td colSpan={3}>{item.case.label}: {item.rowCount} row{item.rowCount === 1 ? "" : "s"} withheld. {item.reason}</td>
            </tr>
          ))}
          {view.pinned.map((row) => (
            <PinnedRow key={row.key} row={row} valueColumns={valueColumns} selectedResultId={selectedResultId} onSelectLocation={onSelectLocation} tid={tid} />
          ))}
          {shown.map((item) => (
            <Row key={item.row.key} row={item.row} rank={item.rank} part="ranked" mode={view.mode} rule={rule}
              units={{ stress: stressUnit, computed: computedUnit, limit: limitUnit }}
              selectedResultId={selectedResultId} onSelectLocation={onSelectLocation} tid={tid} />
          ))}
          {view.unranked.map((row) => (
            <Row key={row.key} row={row} rank={null} part="unranked" mode={view.mode} rule={rule}
              units={{ stress: stressUnit, computed: computedUnit, limit: limitUnit }}
              selectedResultId={selectedResultId} onSelectLocation={onSelectLocation} tid={tid} />
          ))}
        </tbody>
      </table>
    </div>
  );
}

function LocationCell({ row, onSelectLocation }: { row: StressRatioRow; onSelectLocation?: (target: StressRatioTarget) => void }) {
  const text = row.locations.length > 0 ? row.locations.map(stressRatioLocationText).join("; ") : STRESS_RATIO_NO_LOCATION;
  const selectable = onSelectLocation && (row.target.resultId !== null || row.target.entityRef !== null);
  return (
    <td>
      {selectable ? (
        <button type="button" className="stress-ratio-location" aria-label={`Highlight ${text}, ${row.caseLabel}`} onClick={() => onSelectLocation!(row.target)}>{text}</button>
      ) : text}
    </td>
  );
}

/** A withheld row carries no pass or fail status in any attribute (§4); an incomplete status is
 * always stated. */
function rowAttributes(row: StressRatioRow, part: string, selectedResultId: string | null, tid: (suffix: string) => string) {
  const selected = selectedResultId !== null && row.target.resultId === selectedResultId;
  const status = row.outcome?.status;
  return {
    "data-testid": tid(`row-${row.key}`),
    "data-part": part,
    "data-standing": row.standing,
    "data-status": row.standing === "admitted" || status === "RULE_INPUTS_INCOMPLETE" ? status : undefined,
    "aria-current": selected ? ("true" as const) : undefined,
    className: selected ? "selected-result-row" : undefined
  };
}

function PinnedRow({ row, valueColumns, selectedResultId, onSelectLocation, tid }: {
  row: StressRatioRow;
  valueColumns: number;
  selectedResultId: string | null;
  onSelectLocation?: (target: StressRatioTarget) => void;
  tid: (suffix: string) => string;
}) {
  const outcome = row.outcome!;
  const incomplete = outcome.status === "RULE_INPUTS_INCOMPLETE";
  return (
    <tr {...rowAttributes(row, "pinned", selectedResultId, tid)}>
      <th scope="row"><Dash label={STRESS_RATIO_NOT_RANKED} /></th>
      <LocationCell row={row} onSelectLocation={onSelectLocation} />
      <td>{row.caseLabel}</td>
      <td>{outcome.checkId}</td>
      <td data-testid={tid(`status-${row.key}`)}>{incomplete ? STATUS_TEXT.RULE_INPUTS_INCOMPLETE : STRESS_RATIO_WITHHELD}</td>
      <td colSpan={valueColumns} data-testid={tid(`reason-${row.key}`)}>
        {incomplete ? outcome.incompleteReason : row.withheldReason}
        {incomplete && row.standing === "withheld" ? ` ${row.withheldReason}` : ""}
      </td>
    </tr>
  );
}

function Row({ row, rank, part, mode, rule, units, selectedResultId, onSelectLocation, tid }: {
  row: StressRatioRow;
  rank: number | null;
  part: "ranked" | "unranked";
  mode: StressRatioMode;
  rule: boolean;
  units: Readonly<{ stress: string | null; computed: string | null; limit: string | null }>;
  selectedResultId: string | null;
  onSelectLocation?: (target: StressRatioTarget) => void;
  tid: (suffix: string) => string;
}) {
  const outcome = row.outcome;
  const direction = outcome && outcome.ratio !== null ? ratioDirection(outcome.relation) : null;
  return (
    <tr {...rowAttributes(row, part, selectedResultId, tid)}>
      <th scope="row">{rank === null ? <Dash label={STRESS_RATIO_NOT_RANKED} /> : rank}</th>
      <LocationCell row={row} onSelectLocation={onSelectLocation} />
      <td>{row.caseLabel}</td>
      {rule && outcome ? <><td>{outcome.checkId}</td><td data-testid={tid(`status-${row.key}`)}>{STATUS_TEXT[outcome.status] ?? outcome.status}</td></> : null}
      {!rule && row.stress ? (
        <td data-testid={tid(`value-${row.key}`)}><ValueText value={row.stress} id={row.stress.resultId} shareUnit={units.stress !== null} /></td>
      ) : null}
      {rule && outcome && mode === "ratio" ? (
        <>
          <td data-testid={tid(`value-${row.key}`)}>
            {outcome.ratio !== null && direction
              ? <span title={`${displayQuantityParts(outcome.ratio, "").number} (${RATIO_FORMULA[direction]}; ${outcome.identity})`}>{displayQuantityParts(outcome.ratio, "").number}</span>
              : <small>{outcome.ratioNote ?? STRESS_RATIO_NO_RATIO}</small>}
          </td>
          <td data-testid={tid(`direction-${row.key}`)}>{direction ? RATIO_DIRECTION_TEXT[direction] : <Dash label={STRESS_RATIO_NONE} />}</td>
          <td>{outcome.relation}</td>
        </>
      ) : null}
      {rule && outcome && mode === "stress" ? (
        <>
          <td data-testid={tid(`value-${row.key}`)}>{outcome.computed ? <ValueText value={outcome.computed} id={outcome.identity} shareUnit={units.computed !== null} /> : <Dash label={STRESS_RATIO_NONE} />}</td>
          <td data-testid={tid(`limit-${row.key}`)}>{outcome.limit ? <ValueText value={outcome.limit} id={outcome.identity} shareUnit={units.limit !== null} /> : <Dash label={STRESS_RATIO_NONE} />}</td>
        </>
      ) : null}
    </tr>
  );
}
