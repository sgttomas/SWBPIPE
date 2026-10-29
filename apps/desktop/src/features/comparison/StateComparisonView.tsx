import { useMemo, useState } from "react";
import { GitCompare } from "lucide-react";
import { modelStateDescription } from "../../services/modelStateService";
import type { StateComparisonResult } from "../history/useModelStateHistory";
import { CsvExportControl } from "../result-export/CsvExportControl";
import { CSV_SOFTWARE_VERSION, CSV_UNIT_BASIS_STORED } from "../result-export/csvProvenance";
import { CHANGE_LABELS, CLASSIFICATION_LABELS, STATE_COMPARISON_CSV_KIND, stateComparisonCsv, storedValueFullText } from "./stateComparisonCsv";
import {
  ENTITY_FAMILIES,
  FAMILY_LABELS,
  pathText,
  ROW_CLASSIFICATIONS,
  type EntityFamily,
  type FieldChange,
  type RowClassification,
  type StateComparisonRow
} from "./stateComparison";

// The comparison of two saved model states (W-12 S3a; DEL-14-01 Design
// `desktop-states-runs-comparison.md` §4.1, §4.3): the counts per
// classification, a table of entities filterable by classification, family and
// id, and each changed entity's field paths with both values and their units.
// Values are shown as stored: a quantity whose unit differs is marked
// `unit changed` and is never converted or subtracted. The table pages at
// `COMPARISON_PAGE_ROWS` rows, so every matching row is reachable. The table exports to a
// local-private CSV (RT-R6, `stateComparisonCsv.ts`); the comparison envelope export is v0.2
// (FR-CMP-010). It reports differences and establishes nothing
// about either state (F-PIP-2, F-PIP-3).

/** The table's page size: at most this many rows are rendered at once, and
 * every matching row is reachable by paging. */
export const COMPARISON_PAGE_ROWS = 200;
/** The most characters of one stored value shown in a cell. */
const MAX_VALUE_CHARS = 160;

type ClassificationFilter = "differences" | "all" | RowClassification;

function cut(text: string): string {
  const characters = [...text];
  return characters.length > MAX_VALUE_CHARS ? `${characters.slice(0, MAX_VALUE_CHARS).join("")}…` : text;
}

/** A stored value as shown: a quantity as its number(s) and its own unit, anything else as JSON. */
export function storedValueText(value: unknown): string {
  return cut(storedValueFullText(value));
}

function testIdOf(row: Pick<StateComparisonRow, "family" | "key">): string {
  return `state-comparison-row-${row.family}-${row.key.join("/")}`.replace(/[^a-zA-Z0-9:_/-]+/g, "-");
}

function FieldChangeItem({ change }: { change: FieldChange }) {
  return (
    <li className="state-comparison-field" data-kind={change.kind}>
      <code>{pathText(change.path)}</code> <span className="muted">{CHANGE_LABELS[change.kind]}</span>
      <span>: left {storedValueText(change.left)}</span>
      <span>; right {storedValueText(change.right)}</span>
    </li>
  );
}

export function StateComparisonView({ result, onClose }: { result: StateComparisonResult; onClose: () => void }) {
  const { comparison, left, right } = result;
  const [classification, setClassification] = useState<ClassificationFilter>("differences");
  const [family, setFamily] = useState<EntityFamily | "all">("all");
  const [idFilter, setIdFilter] = useState("");
  const [page, setPage] = useState(0);
  // A filter change returns to the first page.
  const filtered = <T,>(set: (value: T) => void) => (value: T) => { set(value); setPage(0); };
  const families = useMemo(
    () => ENTITY_FAMILIES.filter((candidate) => comparison.rows.some((row) => row.family === candidate)),
    [comparison]
  );
  const matching = comparison.rows.filter((row) =>
    (classification === "all" || (classification === "differences" ? row.classification !== "unchanged" : row.classification === classification))
    && (family === "all" || row.family === family)
    && (!idFilter.trim() || row.id.includes(idFilter.trim()))
  );
  const pages = Math.max(1, Math.ceil(matching.length / COMPARISON_PAGE_ROWS));
  const shownPage = Math.min(page, pages - 1);
  const first = shownPage * COMPARISON_PAGE_ROWS;
  const visible = matching.slice(first, first + COMPARISON_PAGE_ROWS);
  const { summary } = comparison;
  return (
    <section className="panel state-comparison-panel" aria-label="Saved state comparison" data-testid="state-comparison">
      <div className="panel-title">
        <GitCompare size={16} aria-hidden="true" />
        Saved state comparison
      </div>
      <p className="muted" data-testid="state-comparison-sides">
        Left: {modelStateDescription(left)}, saved {left.created_at}. Right: {modelStateDescription(right)}, saved {right.created_at}.
      </p>
      <p className="muted" data-testid="state-comparison-basis">
        Entities are mapped automatically by stable id within each family; there is no manual mapping. Values are shown as
        stored, with their own units: nothing is converted or subtracted. This lists differences between two saved
        snapshots only and establishes nothing about either model; human review is still required.
      </p>
      <div className="comparison-summary" data-testid="state-comparison-summary">
        {ROW_CLASSIFICATIONS.map((name) => (
          <div className="comparison-fact" key={name} data-testid={`state-comparison-count-${name}`}>
            <span>{CLASSIFICATION_LABELS[name]}</span>
            <strong>{summary[name]}</strong>
          </div>
        ))}
        <div className="comparison-fact" data-testid="state-comparison-count-total">
          <span>Entities</span>
          <strong>{summary.total}</strong>
        </div>
        <div className="comparison-fact" data-testid="state-comparison-count-unit-changed">
          <span>Fields with a different unit</span>
          <strong>{summary.unit_changed_fields}</strong>
        </div>
      </div>
      <div className="report-actions" role="group" aria-label="Filter the comparison">
        <label className="memory-pool-field">
          <span>Classification</span>
          <select data-testid="state-comparison-filter-classification" value={classification} onChange={(event) => filtered(setClassification)(event.target.value as ClassificationFilter)}>
            <option value="differences">Differences only</option>
            <option value="all">All entities</option>
            {ROW_CLASSIFICATIONS.map((name) => <option key={name} value={name}>{CLASSIFICATION_LABELS[name]}</option>)}
          </select>
        </label>
        <label className="memory-pool-field">
          <span>Family</span>
          <select data-testid="state-comparison-filter-family" value={family} onChange={(event) => filtered(setFamily)(event.target.value as EntityFamily | "all")}>
            <option value="all">All families</option>
            {families.map((name) => <option key={name} value={name}>{FAMILY_LABELS[name]}</option>)}
          </select>
        </label>
        <label className="memory-pool-field">
          <span>Id contains</span>
          <input data-testid="state-comparison-filter-id" value={idFilter} onChange={(event) => filtered(setIdFilter)(event.target.value)} />
        </label>
        <button type="button" data-testid="state-comparison-close" onClick={onClose}>Close comparison</button>
      </div>
      <CsvExportControl
        tableLabel="Saved state comparison"
        testId="state-comparison-export-csv"
        build={() => stateComparisonCsv({
          left, right, rows: matching, filters: { classification, family, idFilter },
          softwareVersion: CSV_SOFTWARE_VERSION, unitBasis: CSV_UNIT_BASIS_STORED
        })}
      />
      <table className="state-comparison-table" data-testid="state-comparison-table" data-csv-export={STATE_COMPARISON_CSV_KIND}>
        <thead>
          <tr><th scope="col">Family</th><th scope="col">Id</th><th scope="col">Classification</th><th scope="col">Fields</th></tr>
        </thead>
        <tbody>
          {visible.map((row) => (
            <tr key={JSON.stringify([row.family, ...row.key])} data-testid={testIdOf(row)} data-classification={row.classification}>
              <td>{FAMILY_LABELS[row.family]}</td>
              <td><code>{row.id}</code></td>
              <td>{CLASSIFICATION_LABELS[row.classification]}</td>
              <td>
                {row.classification === "added" ? <span className="muted">Only in the right state.</span> : null}
                {row.classification === "removed" ? <span className="muted">Only in the left state.</span> : null}
                {row.changes.length ? (
                  <ul className="state-comparison-fields">
                    {row.changes.map((change) => <FieldChangeItem key={JSON.stringify(change.path)} change={change} />)}
                  </ul>
                ) : null}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <div className="report-actions" role="group" aria-label="Comparison pages">
        <small className="report-note" data-testid="state-comparison-shown">
          {matching.length === 0
            ? "No entity matches the filters."
            : `Rows ${first + 1}–${first + visible.length} of ${matching.length}.`}
        </small>
        <button type="button" data-testid="state-comparison-page-previous" disabled={shownPage === 0} title={shownPage === 0 ? "This is the first page." : undefined} onClick={() => setPage(shownPage - 1)}>Previous</button>
        <button type="button" data-testid="state-comparison-page-next" disabled={shownPage >= pages - 1} title={shownPage >= pages - 1 ? "This is the last page." : undefined} onClick={() => setPage(shownPage + 1)}>Next</button>
      </div>
    </section>
  );
}
