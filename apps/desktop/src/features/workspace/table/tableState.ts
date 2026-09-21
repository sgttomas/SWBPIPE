import type { ReactNode } from "react";
import type { EntityKey } from "../selectionState";

export type TableCell = Readonly<{ value: string; unit: string; readonly?: boolean; readout?: ReactNode }>;
export type TableRow = Readonly<{ key: EntityKey; label: string; searchText?: string; cells: Readonly<Record<string, TableCell>> }>;
export type TableColumn = Readonly<{ key: string; label: string; unit: string; kind?: "text" | "quantity"; validate?: (text: string) => string | undefined; equivalent?: (before: string, after: string) => boolean; compare?: (a: string, b: string) => number }>;
export type CellAddress = Readonly<{ rowKey: EntityKey; columnKey: string }>;
export type CapturedCell = CellAddress & Readonly<{
  token: number;
  before: string;
  unit: string;
  generation: string;
  row: TableRow;
}>;
export type TableEdit = Readonly<{ captured: CapturedCell; initialSelection: "all" | "end"; text: string; pending: boolean; error?: string }>;
export type TableSort = Readonly<{ columnKey: string; direction: "ascending" | "descending" }> | null;

export function sameCell(a: CellAddress | null, b: CellAddress | null): boolean {
  return Boolean(a && b && a.rowKey === b.rowKey && a.columnKey === b.columnKey);
}

export function coordinateError(text: string): string | undefined {
  return /^[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?$/.test(text.trim()) && Number.isFinite(Number(text))
    ? undefined : "Enter a finite number in the entered unit.";
}

export function capturedCellIsCurrent(captured: CapturedCell, rows: readonly TableRow[], generation: string): boolean {
  const cell = rows.find((row) => row.key === captured.rowKey)?.cells[captured.columnKey];
  return captured.generation === generation && cell?.value === captured.before && cell.unit === captured.unit;
}

/** Navigation is a projection; it never mutates the supplied file or view order. */
export function adjacentCell(
  address: CellAddress, rows: readonly TableRow[], columns: readonly TableColumn[],
  key: string, backwards = false
): CellAddress {
  let row = rows.findIndex((item) => item.key === address.rowKey);
  let column = columns.findIndex((item) => item.key === address.columnKey);
  if (row < 0 || column < 0 || !rows.length || !columns.length) return address;
  if (key === "Tab") {
    const next = Math.max(0, Math.min(rows.length * columns.length - 1, row * columns.length + column + (backwards ? -1 : 1)));
    row = Math.floor(next / columns.length); column = next % columns.length;
  } else if (key === "Enter") row = Math.min(rows.length - 1, Math.max(0, row + (backwards ? -1 : 1)));
  else if (key === "ArrowDown") row = Math.min(rows.length - 1, row + 1);
  else if (key === "ArrowUp") row = Math.max(0, row - 1);
  else if (key === "ArrowLeft") column = Math.max(0, column - 1);
  else if (key === "ArrowRight") column = Math.min(columns.length - 1, column + 1);
  return { rowKey: rows[row].key, columnKey: columns[column].key };
}
