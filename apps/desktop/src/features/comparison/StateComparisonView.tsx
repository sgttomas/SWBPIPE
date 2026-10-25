import { useMemo, useState } from "react";
import { GitCompare } from "lucide-react";
import { modelStateDescription } from "../../services/modelStateService";
import type { StateComparisonResult } from "../history/useModelStateHistory";
import {
  ENTITY_FAMILIES,
  FAMILY_LABELS,
  isQuantity,
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
// `unit changed` and is never converted or subtracted. View-only: comparison
// export is v0.2 (FR-CMP-010). It reports differences and establishes nothing
// about either state (F-PIP-2, F-PIP-3).

/** The most table rows rendered at once; the filters narrow the rest. */
export const MAX_VISIBLE_COMPARISON_ROWS = 200;
/** The most characters of one stored value shown in a cell. */
const MAX_VALUE_CHARS = 160;

type ClassificationFilter = "differences" | "all" | RowClassification;

const CLASSIFICATION_LABELS: Record<RowClassification, string> = {
  added: "Added",
  removed: "Removed",
  changed: "Changed",
  unchanged: "Unchanged"
};

const CHANGE_LABELS: Record<FieldChange["kind"], string> = {
  added: "added",
  removed: "removed",
  changed: "changed",
  unit_changed: "unit changed (not converted)"
};

function cut(text: string): string {
  const characters = [...text];
  return characters.length > MAX_VALUE_CHARS ? `${characters.slice(0, MAX_VALUE_CHARS).join("")}…` : text;
}

/** A stored value as shown: a quantity as its number(s) and its own unit, anything else as JSON. */
export function storedValueText(value: unknown): string {
  if (value === undefined) return "(absent)";
  if (isQuantity(value)) {
    const numbers = Object.entries(value).filter(([key]) => key !== "unit");
    const shownNumbers = numbers.length === 1 && numbers[0][0] === "value"
      ? String(numbers[0][1])
      : `{${numbers.map(([key, number]) => `${key}: ${String(number)}`).join(", ")}}`;
    return cut(`${shownNumbers} ${value.unit}`);
  }
  return cut(JSON.stringify(value));
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
  const families = useMemo(
    () => ENTITY_FAMILIES.filter((candidate) => comparison.rows.some((row) => row.family === candidate)),
    [comparison]
  );
  const matching = comparison.rows.filter((row) =>
    (classification === "all" || (classification === "differences" ? row.classification !== "unchanged" : row.classification === classification))
    && (family === "all" || row.family === family)
    && (!idFilter.trim() || row.id.includes(idFilter.trim()))
  );
  const visible = matching.slice(0, MAX_VISIBLE_COMPARISON_ROWS);
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
          <select data-testid="state-comparison-filter-classification" value={classification} onChange={(event) => setClassification(event.target.value as ClassificationFilter)}>
            <option value="differences">Differences only</option>
            <option value="all">All entities</option>
            {ROW_CLASSIFICATIONS.map((name) => <option key={name} value={name}>{CLASSIFICATION_LABELS[name]}</option>)}
          </select>
        </label>
        <label className="memory-pool-field">
          <span>Family</span>
          <select data-testid="state-comparison-filter-family" value={family} onChange={(event) => setFamily(event.target.value as EntityFamily | "all")}>
            <option value="all">All families</option>
            {families.map((name) => <option key={name} value={name}>{FAMILY_LABELS[name]}</option>)}
          </select>
        </label>
        <label className="memory-pool-field">
          <span>Id contains</span>
          <input data-testid="state-comparison-filter-id" value={idFilter} onChange={(event) => setIdFilter(event.target.value)} />
        </label>
        <button type="button" data-testid="state-comparison-close" onClick={onClose}>Close comparison</button>
      </div>
      <table className="state-comparison-table" data-testid="state-comparison-table">
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
      <small className="report-note" data-testid="state-comparison-shown">
        {matching.length === 0
          ? "No entity matches the filters."
          : matching.length > visible.length
            ? `Showing the first ${visible.length} of ${matching.length} matching entities; narrow the filters to see the others.`
            : `Showing ${matching.length} matching ${matching.length === 1 ? "entity" : "entities"}.`}
      </small>
    </section>
  );
}
