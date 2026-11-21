/** One group of the results table ("All rows", DEL-07-05 REQ-010): each row on one line with its
 * entity's readable name, its load case or combination, its component, its value in the display
 * unit at the report's display precision, its location, its frame and its stored value. The row's
 * raw id stays in the row's detail. Columns sort on a click of their header (the state is shown in
 * the header and is operable by keyboard); sorting orders rows and forms no value. */
import type { ReactNode } from "react";
import { ArrowDown, ArrowUp, ArrowUpDown } from "lucide-react";
import type { MechanicsResult, PreviewModel, ResultBasisRef } from "../../types";
import { entityLabel } from "../model-workspace/modelView";
import { resolveEntitySelection } from "./resultInterpretation";
import { EngineeringQuantity } from "./EngineeringQuantity";

type Row = MechanicsResult["results"][number];

export type RowsSortColumn = "entity" | "case" | "component" | "value" | "location" | "frame";
export type RowsSort = Readonly<{ column: RowsSortColumn; direction: "ascending" | "descending" }>;

const COLUMN_LABELS: Readonly<Record<RowsSortColumn, string>> = Object.freeze({
  entity: "Entity", case: "Case", component: "Component", value: "Value", location: "Location", frame: "Frame"
});

/** A model entity's readable name, else its reference as stated. */
export function entityNameOf(model: PreviewModel | null | undefined): (ref: string) => string {
  if (!model) return (ref) => ref;
  const cache = new Map<string, string>();
  return (ref) => {
    let name = cache.get(ref);
    if (name === undefined) {
      const entity = resolveEntitySelection(model, ref);
      name = entity ? entityLabel(model, entity) : ref;
      cache.set(ref, name);
    }
    return name;
  };
}

/** A row's load case or combination by its name in the model, else as the navigator states it. */
export function caseNameOf(model: PreviewModel | null | undefined): (basis: ResultBasisRef | null | undefined) => string {
  return (basis) => {
    if (!basis) return "No case stated";
    const kind = basis.ref_type === "load_case" ? "load" : basis.ref_type === "combination" ? "combination" : null;
    const name = model && kind ? entityLabel(model, { type: kind, id: basis.ref_id } as never) : basis.ref_id;
    return name === basis.ref_id ? basis.ref_id : name;
  };
}

/** The next sort after a click on `column`: magnitude descending first for values, ascending for
 * text; a second click reverses it. */
export function nextRowsSort(current: RowsSort | null, column: RowsSortColumn): RowsSort {
  if (current?.column === column) return { column, direction: current.direction === "ascending" ? "descending" : "ascending" };
  return { column, direction: column === "value" ? "descending" : "ascending" };
}

/** `rows` ordered by `sort` (values by magnitude), ties in their published order. */
export function sortRows(rows: readonly Row[], sort: RowsSort | null, keyOf: (row: Row, column: RowsSortColumn) => string | number): Row[] {
  if (!sort) return [...rows];
  const sign = sort.direction === "ascending" ? 1 : -1;
  return rows.map((row, index) => ({ row, index, key: keyOf(row, sort.column) }))
    .sort((a, b) => {
      const order = typeof a.key === "number" && typeof b.key === "number" ? a.key - b.key : String(a.key).localeCompare(String(b.key), undefined, { numeric: true });
      return order * sign || a.index - b.index;
    })
    .map((item) => item.row);
}

export function SortHeader({ column, sort, onSort, disabledReason, className }: { column: RowsSortColumn; sort: RowsSort | null; onSort: (column: RowsSortColumn) => void; disabledReason: string | null; className?: string }) {
  const active = sort?.column === column ? sort.direction : null;
  const Icon = active === "ascending" ? ArrowUp : active === "descending" ? ArrowDown : ArrowUpDown;
  const label = COLUMN_LABELS[column];
  return (
    <th scope="col" aria-sort={active ?? "none"} className={className}>
      <button type="button" className="results-sort-button" data-testid={`results-sort-${column}`} aria-disabled={disabledReason ? true : undefined}
        title={disabledReason ?? `Sort by ${label.toLowerCase()}${column === "value" ? " (magnitude)" : ""}`}
        onClick={() => { if (!disabledReason) onSort(column); }}>
        {label}<Icon size={12} aria-hidden="true" />
      </button>
    </th>
  );
}

export function ResultRowsTable({
  csvKind, items, selectedResultId, onSelectResult, entityName, caseName, dimension, frameText, notes, sort, onSort, sortDisabledReason
}: {
  csvKind: string;
  items: readonly Row[];
  selectedResultId: string | null;
  onSelectResult: (id: string) => void;
  entityName: (ref: string) => string;
  caseName: (basis: ResultBasisRef | null | undefined) => string;
  dimension: (row: Row) => string;
  frameText: (row: Row) => string;
  /** What else the row carries (its category, row label, numerical health, withheld reason). */
  notes: (row: Row) => ReactNode;
  sort: RowsSort | null;
  onSort: (column: RowsSortColumn) => void;
  sortDisabledReason: string | null;
}) {
  const header = (column: RowsSortColumn, className?: string) => <SortHeader column={column} sort={sort} onSort={onSort} disabledReason={sortDisabledReason} className={className} />;
  return (
    <div className="results-table-scroll">
      <table data-csv-export={csvKind} className="results-rows-table">
        <thead>
          <tr>
            {header("entity", "results-sticky-column")}
            {header("case")}
            {header("component")}
            {header("value")}
            {header("location")}
            {header("frame")}
            <th scope="col" title="The stored value, as the solve published it, at full precision">Stored</th>
            <th scope="col">Notes</th>
          </tr>
        </thead>
        <tbody>
          {items.map((item) => (
            <tr
              key={item.id}
              aria-selected={item.id === selectedResultId}
              className={item.id === selectedResultId ? "selected-result-row" : ""}
              data-testid={`result-row-${item.id}`}
              data-row-id={item.id}
              onClick={() => onSelectResult(item.id)}
              onKeyDown={(event) => {
                if (event.key === "Enter" || event.key === " ") {
                  event.preventDefault();
                  onSelectResult(item.id);
                }
              }}
              tabIndex={0}
            >
              <th scope="row" className="results-sticky-column" title={item.entity_ref}>{entityName(item.entity_ref)}</th>
              <td>{caseName(item.basis_ref)}</td>
              <td>{item.metadata?.component ?? item.kind}</td>
              <td className="result-value-cell"><EngineeringQuantity quantity={{ value: item.value, unit: item.unit, dimension_id: dimension(item) }} /></td>
              <td>{item.metadata?.location ?? "summary"}</td>
              <td data-testid={`result-row-frame-${item.id}`}>{frameText(item)}</td>
              <td className="dual-unit-cell" data-testid={`result-row-dual-${item.id}`}><span>Entered: {item.value} {item.unit}</span></td>
              <td className="results-row-notes">{notes(item)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
