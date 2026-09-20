import { useId, useLayoutEffect, useMemo, useRef, useState, type KeyboardEvent } from "react";
import { VirtualList } from "../VirtualList";
import type { EntityKey } from "../selectionState";
import {
  adjacentCell, capturedCellIsCurrent, sameCell,
  type CapturedCell, type CellAddress, type TableColumn, type TableEdit, type TableRow, type TableSort
} from "./tableState";

export type TableApplyResult = Readonly<{ applied: boolean; rejected?: boolean; messages: readonly string[] }>;
type Props = Readonly<{
  label: string;
  rows: readonly TableRow[];
  columns: readonly TableColumn[];
  generation: string;
  filter: string;
  density: "comfortable" | "compact";
  selectedKey: EntityKey;
  busy?: boolean;
  onSelect: (key: EntityKey) => void;
  onApply: (captured: CapturedCell, text: string) => Promise<TableApplyResult>;
}>;

/** Owns interaction drafts only. Canonical values and operation acceptance belong to the caller. */
export function EngineeringTable(props: Props) {
  const { label, rows, columns, generation, filter, density, selectedKey, busy = false, onSelect, onApply } = props;
  const [focused, setFocused] = useState<CellAddress | null>(null);
  const [edit, setEditState] = useState<TableEdit | null>(null);
  const [sort, setSort] = useState<TableSort>(null);
  const [feedback, setFeedback] = useState("");
  const [reveal, setReveal] = useState(0);
  const errorId = useId();
  const root = useRef<HTMLDivElement>(null);
  const input = useRef<HTMLInputElement>(null);
  const footer = useRef<HTMLDivElement>(null);
  const editRef = useRef<TableEdit | null>(null);
  const sequence = useRef(0);
  const pointerFocus = useRef<CellAddress | null>(null);
  const latest = useRef(props);
  latest.current = props;
  const previousGeneration = useRef(generation);
  const selectionOwnership = useRef({ key: selectedKey, revision: 0 });
  if (selectionOwnership.current.key !== selectedKey) {
    selectionOwnership.current = { key: selectedKey, revision: selectionOwnership.current.revision + 1 };
  }
  const focusRequest = useRef<CellAddress | null>(null);
  function setEdit(value: TableEdit | null) { editRef.current = value; setEditState(value); }

  const matchingRows = useMemo(() => rows.filter((row) => !filter.trim() || (row.searchText ?? `${row.label} ${row.key}`).toLowerCase().includes(filter.trim().toLowerCase())), [rows, filter]);
  const viewRows = useMemo(() => {
    const result = [...matchingRows];
    // Keep the captured row available until Apply or Cancel, including invalid text.
    if (edit && !result.some((row) => row.key === edit.captured.rowKey)) result.push(rows.find((row) => row.key === edit.captured.rowKey) ?? edit.captured.row);
    if (sort) result.sort((a, b) => {
      const av = a.cells[sort.columnKey]?.value ?? ""; const bv = b.cells[sort.columnKey]?.value ?? "";
      const difference = columns.find((column) => column.key === sort.columnKey)?.compare?.(av, bv) ?? av.localeCompare(bv);
      return difference * (sort.direction === "ascending" ? 1 : -1);
    });
    return result;
  }, [matchingRows, rows, edit, sort, columns]);
  // A remembered cell may be filtered out or removed. Keep a live keyboard
  // entry without moving DOM focus or publishing a new model selection.
  const rovingFocus = focused && viewRows.some((row) => row.key === focused.rowKey && rows.some((live) => live.key === row.key) && row.cells[focused.columnKey])
    ? focused
    : !edit && viewRows[0] && columns[0] ? { rowKey: viewRows[0].key, columnKey: columns[0].key } : null;
  const active = edit?.captured ?? rovingFocus;
  const activeIndex = active ? viewRows.findIndex((row) => row.key === active.rowKey) : null;

  useLayoutEffect(() => {
    if (previousGeneration.current === generation) return;
    previousGeneration.current = generation;
    sequence.current += 1;
    setFeedback(editRef.current ? "The project changed; the previous cell edit was cancelled." : "");
    setEdit(null); setFocused(null); focusRequest.current = null;
  }, [generation]);

  useLayoutEffect(() => {
    if (!focusRequest.current) return;
    const request = focusRequest.current;
    if (!viewRows.some((row) => row.key === request.rowKey && row.cells[request.columnKey])) {
      focusRequest.current = null;
      return;
    }
    const element = Array.from(root.current?.querySelectorAll<HTMLElement>("[data-table-cell]") ?? []).find((candidate) => candidate.dataset.rowKey === request.rowKey && candidate.dataset.columnKey === request.columnKey);
    if (element) { element.focus(); focusRequest.current = null; }
  }, [focused, viewRows, reveal]);

  function focusCell(address: CellAddress, moveDomFocus = true) {
    setFocused(address); onSelect(address.rowKey);
    if (moveDomFocus) { focusRequest.current = address; setReveal((value) => value + 1); }
  }
  function startEdit(address: CellAddress, replacement?: string) {
    if (busy || editRef.current) return;
    const row = rows.find((item) => item.key === address.rowKey); const cell = row?.cells[address.columnKey];
    if (!row || !cell) return;
    focusRequest.current = null;
    focusCell(address, false); setFeedback("");
    setEdit({ captured: { ...address, token: ++sequence.current, before: cell.value, unit: cell.unit, generation, row }, text: replacement ?? cell.value, pending: false });
  }
  function cancel() {
    const current = editRef.current;
    if (!current || current.pending) return;
    sequence.current += 1; setEdit(null); setFeedback("");
    const destination = latest.current.rows.some((row) => row.key === current.captured.rowKey) ? current.captured
      : viewRows.find((row) => latest.current.rows.some((canonical) => canonical.key === row.key));
    if (destination) focusCell("rowKey" in destination ? destination : { rowKey: destination.key, columnKey: columns[0].key });
    else { setFocused(null); root.current?.focus(); }
  }
  async function apply(next?: CellAddress, restoreFocus = false, boundaryExit?: HTMLElement): Promise<boolean> {
    const current = editRef.current;
    if (!current || current.pending) return false;
    const column = columns.find((candidate) => candidate.key === current.captured.columnKey);
    const error = column?.validate?.(current.text);
    if (error) { setEdit({ ...current, error }); return false; }
    if (!capturedCellIsCurrent(current.captured, latest.current.rows, latest.current.generation)) {
      setEdit({ ...current, error: "This cell or project changed after editing began. Cancel and edit the current value." }); return false;
    }
    if (busy) { setEdit({ ...current, error: "Another operation is running. Apply again when it finishes." }); return false; }
    const owner = document.activeElement;
    const selectionRevision = selectionOwnership.current.revision;
    const captured = current.captured;
    const stillOwns = () => editRef.current?.captured.token === captured.token && latest.current.generation === captured.generation;
    const canFocus = () => restoreFocus && document.activeElement === owner && selectionOwnership.current.revision === selectionRevision;
    const finishFocus = () => {
      if (!canFocus()) return;
      if (boundaryExit) {
        if (boundaryExit.isConnected && root.current?.contains(boundaryExit) && !boundaryExit.closest("[hidden], [inert]")) boundaryExit.focus();
      } else focusCell(next ?? captured);
    };
    if (column?.equivalent?.(captured.before, current.text) ?? current.text === captured.before) {
      setEdit(null); finishFocus(); return true;
    }
    setEdit({ ...current, pending: true, error: undefined });
    let result: TableApplyResult;
    try { result = await onApply(captured, current.text); }
    catch (error) { result = { applied: false, messages: [`Apply failed: ${String(error)}`] }; }
    if (!stillOwns()) return false;
    if (!result.applied && !result.rejected) {
      setEdit({ ...current, pending: false, error: result.messages.join(" · ") || "The operation was not applied. Check the current model and try again." });
      return false;
    }
    if (result.rejected && !result.applied) {
      setEdit({ ...current, text: captured.before, pending: false, error: `Engine rejected: ${result.messages.join(" · ") || "The value was not accepted."}` });
      return false;
    }
    setEdit(null);
    setFeedback(result.messages.join(" · ") || (result.applied ? "Cell applied." : "The engine rejected this value."));
    finishFocus();
    return result.applied;
  }
  function cellKey(event: KeyboardEvent<HTMLElement>, address: CellAddress, editing: boolean) {
    if (event.isDefaultPrevented() || event.nativeEvent.isComposing || event.ctrlKey || event.metaKey || event.altKey) return;
    if (!editing && editRef.current) {
      if (event.key === "Escape") { event.preventDefault(); event.stopPropagation(); cancel(); }
      else if (event.key.length === 1 || ["Enter", "ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown"].includes(event.key)) {
        event.preventDefault(); event.stopPropagation();
        if (!editRef.current.pending) input.current?.focus();
      }
      return;
    }
    if (editing && event.key === "Escape") { event.preventDefault(); event.stopPropagation(); cancel(); return; }
    if (editing && ["ArrowLeft", "ArrowRight"].includes(event.key)) return;
    if (["Tab", "ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"].includes(event.key) || (editing && event.key === "Enter")) {
      // Enter follows file order, even in a sorted view. Other movement follows the view.
      const order = event.key === "Enter" ? rows.filter((row) => viewRows.some((visible) => visible.key === row.key)) : viewRows;
      const next = adjacentCell(address, order, columns, event.key, event.shiftKey);
      if (event.key === "Tab" && sameCell(next, address)) {
        if (editing) {
          // The input is replaced on Apply. Native Tab would traverse from the
          // replacement cell, so own this transition and use a persistent target.
          event.preventDefault(); event.stopPropagation();
          const destination = event.shiftKey
            ? root.current?.querySelector<HTMLElement>('[role="rowheader"] button')
            : footer.current;
          if (destination) void apply(undefined, true, destination);
        }
        return;
      }
      event.preventDefault(); event.stopPropagation();
      if (editing) void apply(next, true); else focusCell(next);
    } else if (!editing && (event.key === "Enter" || event.key.length === 1)) {
      event.preventDefault(); event.stopPropagation(); startEdit(address, event.key === "Enter" ? undefined : event.key);
    }
  }
  const template = `minmax(130px, 1.4fr) repeat(${columns.length}, minmax(90px, 1fr))`;
  return <div className="engineering-table" ref={root} tabIndex={-1} data-testid="engineering-table">
    <div role="grid" aria-label={label} aria-rowcount={viewRows.length + 1} aria-colcount={columns.length + 1}>
      <div role="row" className="engineering-table-row engineering-table-header" style={{ gridTemplateColumns: template }}>
        <div role="columnheader">Node</div>
        {columns.map((column) => <div key={column.key} role="columnheader" aria-sort={sort?.columnKey === column.key ? sort.direction : "none"}>
          <button type="button" aria-label={`Sort ${column.label}`} onClick={() => setSort((current) => current?.columnKey !== column.key ? { columnKey: column.key, direction: "ascending" } : current.direction === "ascending" ? { ...current, direction: "descending" } : null)}>{column.label} [{column.unit}] <span aria-hidden="true">{sort?.columnKey === column.key ? sort.direction === "ascending" ? "↑" : "↓" : "↕"}</span></button>
        </div>)}
      </div>
      <VirtualList items={viewRows} itemKey={(row) => row.key} activeIndex={activeIndex} pinIndex={activeIndex} revealActiveRequest={reveal} height={360} rowHeight={density === "compact" ? 30 : 36} role="rowgroup" testId="engineering-table-rows" renderItem={(row, index) => <div role="row" aria-rowindex={index + 2} aria-selected={selectedKey === row.key} className={`engineering-table-row${index % 2 ? " stripe" : ""}${selectedKey === row.key ? " selected" : ""}`} style={{ gridTemplateColumns: template }}>
        <div role="rowheader"><button type="button" disabled={!rows.some((canonical) => canonical.key === row.key)} onClick={() => onSelect(row.key)} title={row.label}>{row.label}{!rows.some((canonical) => canonical.key === row.key) ? " (removed edit)" : ""}</button></div>
        {columns.map((column) => {
          const address = { rowKey: row.key, columnKey: column.key }; const editing = sameCell(edit?.captured ?? null, address);
          return <div key={column.key} role="gridcell" aria-selected={sameCell(focused, address)} className={`engineering-table-cell${editing ? " editing" : ""}${editing && edit?.error ? " invalid" : ""}`}>
            {editing && edit ? <input ref={input} autoFocus aria-label={`${row.label} ${column.label} [${edit.captured.unit}]`} aria-invalid={Boolean(edit.error)} aria-describedby={edit.error ? errorId : undefined} value={edit.text} readOnly={edit.pending} onFocus={(event) => event.currentTarget.select()} onChange={(event) => {
              const current = editRef.current;
              if (current && !current.pending && current.captured.token === edit.captured.token) setEdit({ ...current, text: event.target.value, error: undefined });
            }} onKeyDown={(event) => cellKey(event, address, true)} onBlur={(event) => {
              const destination = event.relatedTarget as HTMLElement | null;
              // Footer controls explicitly own Apply/Cancel. Cell clicks own their next target.
              if (destination && root.current?.contains(destination) && destination.closest("[data-table-action], [data-table-cell]")) return;
              void apply();
            }} /> : <button type="button" disabled={!rows.some((canonical) => canonical.key === row.key)} data-table-cell="true" data-row-key={row.key} data-column-key={column.key} data-testid={`table-cell-${row.label}-${column.key}`} aria-label={`${row.label} ${column.label}: ${row.cells[column.key].value} ${row.cells[column.key].unit}`} tabIndex={sameCell(rovingFocus, address) ? 0 : -1} onPointerDown={() => { pointerFocus.current = address; }} onFocus={() => {
              if (!sameCell(pointerFocus.current, address) && !editRef.current && !sameCell(focused, address)) focusCell(address, false);
            }} onBlur={() => { if (sameCell(pointerFocus.current, address)) pointerFocus.current = null; }} onClick={() => {
              pointerFocus.current = null;
              if (editRef.current) { void apply(address, true); return; }
              if (sameCell(focused, address)) startEdit(address); else focusCell(address);
            }} onDoubleClick={() => { if (!editRef.current) startEdit(address); }} onKeyDown={(event) => cellKey(event, address, false)}>{row.cells[column.key].value}</button>}
          </div>;
        })}
      </div>} />
    </div>
    <div className="engineering-table-footer" ref={footer} role="group" aria-label={`${label} footer`} tabIndex={-1}>
      {edit ? <><span>Editing {columns.find((column) => column.key === edit.captured.columnKey)?.label} · {edit.captured.row.label}</span><button type="button" data-table-action="apply" disabled={edit.pending || busy} onClick={() => void apply(undefined, true)} title="Apply (Enter)">Apply</button><button type="button" data-table-action="cancel" disabled={edit.pending} onClick={cancel} title="Cancel (Escape)">Cancel</button></> : <span>{matchingRows.length} of {rows.length} rows</span>}
      {sort ? <button type="button" onClick={() => setSort(null)}>Sorted by {columns.find((column) => column.key === sort.columnKey)?.label} · Clear</button> : null}
      {edit && !rows.some((row) => row.key === edit.captured.rowKey) ? <span role="alert">The edited row was removed. This retained draft cannot be applied; Cancel to return to the current model.</span>
        : edit && !matchingRows.some((row) => row.key === edit.captured.rowKey) ? <span>Editing row retained outside the filter.</span> : null}
    </div>
    {edit?.error ? <p id={errorId} role="alert" className="engineering-table-message">{edit.error}</p> : null}
    {feedback ? <p role="status" className="engineering-table-message">{feedback}</p> : null}
  </div>;
}
