import { useEffect, useId, useLayoutEffect, useMemo, useRef, useState, type KeyboardEvent } from "react";
import { VirtualList } from "../VirtualList";
import type { EntityKey } from "../selectionState";
import {
  adjacentCell, capturedCellIsCurrent, sameCell,
  type CapturedCell, type CellAddress, type TableColumn, type TableEdit, type TableRow, type TableSort
} from "./tableState";

export type TableApplyResult = Readonly<{ applied: boolean; rejected?: boolean; messages: readonly string[] }>;
export type TableDraftResult = Readonly<{ retained: boolean; messages: readonly string[] }>;
type Props = Readonly<{
  label: string;
  rowHeader?: string;
  testIdPrefix?: string;
  persistentEditor?: boolean;
  bounded?: boolean;
  active?: boolean;
  onDraftStateChange?: (retained: boolean) => void;
  rows: readonly TableRow[];
  columns: readonly TableColumn[];
  generation: string;
  filter: string;
  density: "comfortable" | "compact";
  selectedKey: EntityKey;
  busy?: boolean;
  onSelect: (key: EntityKey) => void;
}> & (Readonly<{ policy?: "direct"; onApply: (captured: CapturedCell, text: string) => Promise<TableApplyResult> }> | Readonly<{
  policy: "review";
  resetEditsKey: number;
  onDraftChange: (captured: CapturedCell, text: string) => void;
  onKeepDraft: (captured: CapturedCell, text: string) => TableDraftResult;
}>);

/** Owns interaction drafts only. Canonical values and operation acceptance belong to the caller. */
export function EngineeringTable(props: Props) {
  const { label, rows, columns, generation, filter, density, selectedKey, busy = false, bounded = false, active: surfaceActive = true, onDraftStateChange, onSelect } = props;
  const review = props.policy === "review";
  const resetEditsKey = review ? props.resetEditsKey : 0;
  const body = useTableBodyHeight(bounded, surfaceActive);
  const [focused, setFocused] = useState<CellAddress | null>(null);
  const [edit, setEditState] = useState<TableEdit | null>(null);
  useEffect(() => { onDraftStateChange?.(Boolean(edit)); }, [Boolean(edit), onDraftStateChange]);
  const [sort, setSort] = useState<TableSort>(null);
  const [feedback, setFeedback] = useState("");
  const [reveal, setReveal] = useState(0);
  const errorId = useId();
  const editorId = useId();
  const root = useRef<HTMLDivElement>(null);
  const input = useRef<HTMLInputElement>(null);
  const footer = useRef<HTMLDivElement>(null);
  const editRef = useRef<TableEdit | null>(null);
  const sequence = useRef(0);
  const initializedInputToken = useRef<number | null>(null);
  const pointerFocus = useRef<CellAddress | null>(null);
  const latest = useRef(props);
  latest.current = props;
  const previousGeneration = useRef(generation);
  const selectionOwnership = useRef({ key: selectedKey, revision: 0 });
  if (selectionOwnership.current.key !== selectedKey) {
    selectionOwnership.current = { key: selectedKey, revision: selectionOwnership.current.revision + 1 };
  }
  const focusRequest = useRef<CellAddress | null>(null);
  const editorFocusRequest = useRef<{ token: number; generation: string; owner: Element | null; selectionRevision: number } | null>(null);
  const ownedCellElement = useRef<{ element: HTMLElement; selectionRevision: number } | null>(null);
  // A completed label edit can disappear on a later canonical publication.
  // Recover only focus lost through removal, never an intentional external move.
  useLayoutEffect(() => {
    const owned = ownedCellElement.current; const element = owned?.element;
    if (!editorFocusRequest.current && element && owned?.selectionRevision === selectionOwnership.current.revision && !element.isConnected && document.activeElement === document.body && surfaceActive && !root.current?.closest("[hidden], [inert]")) footer.current?.focus();
  }, [rows, filter, surfaceActive]);
  useEffect(() => {
    const track = (event: FocusEvent) => {
      const element = event.target as HTMLElement;
      ownedCellElement.current = root.current?.contains(element) && element.matches("[data-table-cell], .engineering-table-cell input, .engineering-table-editor-layer input") ? { element, selectionRevision: selectionOwnership.current.revision } : null;
    };
    document.addEventListener("focusin", track);
    return () => document.removeEventListener("focusin", track);
  }, []);
  function setEdit(value: TableEdit | null) { editRef.current = value; setEditState(value); }

  const matchingRows = useMemo(() => rows.filter((row) => !filter.trim() || (row.searchText ?? `${row.label} ${row.key}`).toLowerCase().includes(filter.trim().toLowerCase())), [rows, filter]);
  const comparisonRows = useMemo(() => {
    const result = [...matchingRows];
    // Keep the captured row available until Apply or Cancel, including invalid text.
    if (edit && !result.some((row) => row.key === edit.captured.rowKey)) result.push(rows.find((row) => row.key === edit.captured.rowKey) ?? edit.captured.row);
    return result;
  }, [matchingRows, rows, edit]);
  const sortUnavailable = Boolean(sort && columns.find((column) => column.key === sort.columnKey)?.projectedSort &&
    comparisonRows.some((row) => !rows.some((live) => live.key === row.key) || !Number.isFinite(row.cells[sort.columnKey]?.sortValue)));
  const viewRows = useMemo(() => {
    const result = [...comparisonRows];
    if (sort && !sortUnavailable) result.sort((a, b) => {
      const av = a.cells[sort.columnKey]?.value ?? ""; const bv = b.cells[sort.columnKey]?.value ?? "";
      const column = columns.find((column) => column.key === sort.columnKey);
      const difference = column?.projectedSort ? a.cells[sort.columnKey].sortValue! - b.cells[sort.columnKey].sortValue! : column?.compare?.(av, bv) ?? av.localeCompare(bv);
      return difference * (sort.direction === "ascending" ? 1 : -1);
    });
    return result;
  }, [comparisonRows, sort, columns, sortUnavailable]);
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

  useLayoutEffect(() => { setEdit(null); setFeedback(""); }, [resetEditsKey]);

  useLayoutEffect(() => {
    if (!focusRequest.current) return;
    const request = focusRequest.current;
    if (!viewRows.some((row) => row.key === request.rowKey && row.cells[request.columnKey])) {
      focusRequest.current = null;
      footer.current?.focus();
      return;
    }
    const element = Array.from(root.current?.querySelectorAll<HTMLElement>("[data-table-cell]") ?? []).find((candidate) => candidate.dataset.rowKey === request.rowKey && candidate.dataset.columnKey === request.columnKey);
    if (element) { element.focus(); focusRequest.current = null; }
  }, [focused, viewRows, reveal]);

  function focusCell(address: CellAddress, moveDomFocus = true) {
    setFocused(address);
    // Review editing preserves the existing ordered selection; its rowheader
    // remains the explicit model-selection control.
    if (!review) onSelect(address.rowKey);
    if (moveDomFocus) { focusRequest.current = address; setReveal((value) => value + 1); }
  }
  function startEdit(address: CellAddress, replacement?: string) {
    if (busy || editRef.current) return;
    const row = rows.find((item) => item.key === address.rowKey); const cell = row?.cells[address.columnKey];
    if (!row || !cell || cell.readonly) return;
    focusRequest.current = null;
    focusCell(address, false); setFeedback("");
    const captured = { ...address, token: ++sequence.current, before: cell.value, unit: cell.unit, generation, row };
    if (props.persistentEditor) editorFocusRequest.current = { token: captured.token, generation, owner: document.activeElement, selectionRevision: selectionOwnership.current.revision };
    setEdit({ captured, initialSelection: replacement === undefined ? "all" : "end", text: replacement ?? cell.value, pending: false });
    if (props.policy === "review" && replacement !== undefined) props.onDraftChange(captured, replacement);
  }
  function cancel() {
    const current = editRef.current;
    if (!current || current.pending) return;
    const liveCell = latest.current.rows.find((row) => row.key === current.captured.rowKey)?.cells[current.captured.columnKey];
    const staleReview = props.policy === "review" && (current.captured.generation !== latest.current.generation || !liveCell || liveCell.readonly || liveCell.unit !== current.captured.unit);
    if (props.policy === "review" && !staleReview) props.onDraftChange(current.captured, current.captured.before);
    sequence.current += 1; setEdit(null); setFeedback(staleReview ? "The editor basis changed; editor closed without rewriting retained drafts. Queue uses the current model value and unit." : "");
    const destination = latest.current.rows.some((row) => row.key === current.captured.rowKey) ? current.captured
      : viewRows.find((row) => latest.current.rows.some((canonical) => canonical.key === row.key));
    if (destination) focusCell("rowKey" in destination ? destination : { rowKey: destination.key, columnKey: columns[0].key });
    else { setFocused(null); root.current?.focus(); }
  }
  async function apply(next?: CellAddress, restoreFocus = false, boundaryExit?: HTMLElement): Promise<boolean> {
    const current = editRef.current;
    if (!current || current.pending) return false;
    const column = columns.find((candidate) => candidate.key === current.captured.columnKey);
    const error = review ? undefined : column?.validate?.(current.text);
    if (error) { setEdit({ ...current, error }); return false; }
    const liveCell = latest.current.rows.find((row) => row.key === current.captured.rowKey)?.cells[current.captured.columnKey];
    if (review && (!liveCell || current.captured.generation !== latest.current.generation)) { setEdit({ ...current, error: "This cell or project changed. Cancel to return to the current model." }); return false; }
    if (review && liveCell?.unit !== current.captured.unit) { setEdit({ ...current, error: "The entered unit changed. Cancel this editor; retained drafts queue using the current model value and unit." }); return false; }
    if (liveCell?.readonly) { setEdit({ ...current, error: "This cell is now read-only. Cancel to return to the current value." }); return false; }
    if (!review && !capturedCellIsCurrent(current.captured, latest.current.rows, latest.current.generation)) {
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
    if (props.policy === "review") {
      const result = props.onKeepDraft(captured, current.text);
      if (!result.retained) { setEdit({ ...current, error: result.messages.join(" · ") }); return false; }
      setEdit(null); setFeedback(result.messages.join(" · ")); finishFocus(); return true;
    }
    if (column?.equivalent?.(captured.before, current.text) ?? current.text === captured.before) {
      setEdit(null); finishFocus(); return true;
    }
    setEdit({ ...current, pending: true, error: undefined });
    let result: TableApplyResult;
    try { result = await props.onApply(captured, current.text); }
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
  const column = edit ? columns.find((candidate) => candidate.key === edit.captured.columnKey) : undefined;
  const editor = edit && column ? <input ref={input} id={editorId} data-kind={column.kind} autoFocus aria-label={`${edit.captured.row.label} ${column.label}${column.kind === "text" ? "" : ` [${edit.captured.unit}]`}`} aria-invalid={Boolean(edit.error)} aria-describedby={edit.error ? errorId : undefined} value={edit.text} readOnly={edit.pending} onFocus={(event) => {
              if (initializedInputToken.current === edit.captured.token) return;
              initializedInputToken.current = edit.captured.token;
              if (edit.initialSelection === "all") event.currentTarget.select();
              else event.currentTarget.setSelectionRange(event.currentTarget.value.length, event.currentTarget.value.length);
            }} onChange={(event) => {
              const current = editRef.current;
              if (current && !current.pending && current.captured.token === edit.captured.token) {
                setEdit({ ...current, text: event.target.value, error: undefined });
                if (props.policy === "review") {
                  const live = latest.current.rows.find((row) => row.key === current.captured.rowKey)?.cells[current.captured.columnKey];
                  if (live && !live.readonly && live.unit === current.captured.unit && latest.current.generation === current.captured.generation) props.onDraftChange(current.captured, event.target.value);
                  else setEdit({ ...current, error: "The editor basis changed. Cancel to return to retained drafts; Queue uses the current model value and unit." });
                }
              }
            }} onKeyDown={(event) => cellKey(event, edit.captured, true)} onBlur={(event) => {
              const destination = event.relatedTarget as HTMLElement | null;
              // Footer controls explicitly own Apply/Cancel. Cell clicks own their next target.
              if (destination && root.current?.contains(destination) && destination.closest("[data-table-action], [data-table-cell]")) return;
              void apply();
            }} /> : null;
  const editorPosition = usePersistentEditorPosition(root, Boolean(props.persistentEditor && edit), edit?.captured.token, viewRows, surfaceActive);
  useLayoutEffect(() => {
    const request = editorFocusRequest.current;
    if (!request) return;
    if (request.token !== edit?.captured.token || request.generation !== generation || !surfaceActive || selectionOwnership.current.revision !== request.selectionRevision || root.current?.closest("[hidden], [inert]")) { editorFocusRequest.current = null; return; }
    if (initializedInputToken.current === request.token) { editorFocusRequest.current = null; return; }
    if (editorPosition.clip.visibility === "hidden") return;
    // Only recover focus removed with the initiating cell. A different live
    // focus owner, selection, project, or hidden surface cancels this request.
    const owner = document.activeElement;
    if (owner === request.owner || (owner === document.body && !request.owner?.isConnected)) input.current?.focus();
    editorFocusRequest.current = null;
  }, [editorPosition, edit, generation, surfaceActive]);
  const template = `minmax(130px, 1.4fr) ${columns.map((column) => column.minWidth ? `minmax(${column.minWidth}px, 1fr)` : column.kind === "text" ? "minmax(150px, 1.5fr)" : review ? "minmax(160px, 1fr)" : "minmax(90px, 1fr)").join(" ")}`;
  const tableWidth = 130 + columns.reduce((sum, column) => sum + (column.minWidth ?? (column.kind === "text" ? 150 : review ? 160 : 90)), 0);
  return <div className={`engineering-table${bounded ? " bounded" : ""}`} ref={root} style={{ "--table-min-width": `${tableWidth}px` } as React.CSSProperties} tabIndex={-1} data-testid={`${props.testIdPrefix ?? ""}${review ? "engineering-table-review" : "engineering-table"}`}>
    <div role="grid" aria-label={label} aria-rowcount={viewRows.length + 1} aria-colcount={columns.length + 1}>
      <div role="row" className="engineering-table-row engineering-table-header" style={{ gridTemplateColumns: template }}>
        <div role="columnheader">{props.rowHeader ?? "Node"}</div>
        {columns.map((column) => <div key={column.key} role="columnheader" aria-sort={sort?.columnKey === column.key && !sortUnavailable ? sort.direction : "none"}>
          <button type="button" aria-label={`Sort ${column.label}`} onClick={() => setSort((current) => current?.columnKey !== column.key ? { columnKey: column.key, direction: "ascending" } : current.direction === "ascending" ? { ...current, direction: "descending" } : null)}>{column.label}{column.kind === "text" ? "" : ` [${column.unit}]`} <span aria-hidden="true">{sort?.columnKey === column.key ? sort.direction === "ascending" ? "↑" : "↓" : "↕"}</span></button>
        </div>)}
      </div>
      <div className="engineering-table-body-slot" ref={body.ref}>
      <VirtualList items={viewRows} itemKey={(row) => row.key} activeIndex={activeIndex} pinIndex={activeIndex} revealActiveRequest={reveal} height={bounded ? Math.min(body.height, viewRows.length * (density === "compact" ? 30 : 36)) : 360} rowHeight={density === "compact" ? 30 : 36} role="rowgroup" testId={`${props.testIdPrefix ?? ""}${review ? "engineering-table-review-rows" : "engineering-table-rows"}`} renderItem={(row, index) => <div role="row" aria-rowindex={index + 2} aria-selected={selectedKey === row.key} className={`engineering-table-row${index % 2 ? " stripe" : ""}${selectedKey === row.key ? " selected" : ""}`} style={{ gridTemplateColumns: template }}>
        <div role="rowheader"><button type="button" disabled={!rows.some((canonical) => canonical.key === row.key)} onClick={() => onSelect(row.key)} title={row.label}>{row.label}{!rows.some((canonical) => canonical.key === row.key) ? " (removed edit)" : ""}</button></div>
        {columns.map((column) => {
          const address = { rowKey: row.key, columnKey: column.key }; const editing = sameCell(edit?.captured ?? null, address);
          return <div key={column.key} role="gridcell" aria-owns={editing && props.persistentEditor ? editorId : undefined} aria-selected={sameCell(focused, address)} title={row.cells[column.key].unavailable} aria-readonly={Boolean(row.cells[column.key].readonly)} data-kind={column.kind} className={`engineering-table-cell${row.cells[column.key].readout ? " has-readout" : ""}${editing ? " editing" : ""}${editing && edit?.error ? " invalid" : ""}`}>
            {editing && edit ? props.persistentEditor ? <span className="engineering-table-editor-anchor" data-editor-anchor={edit.captured.token} /> : editor : <button type="button" disabled={!rows.some((canonical) => canonical.key === row.key)} data-table-cell="true" data-row-key={row.key} data-column-key={column.key} data-testid={`${review ? "review" : "table"}-cell-${row.label}-${column.key}`} aria-label={`${row.label} ${column.label}: ${row.cells[column.key].value}${column.kind === "text" ? "" : ` ${row.cells[column.key].unit}`}`} tabIndex={sameCell(rovingFocus, address) ? 0 : -1} onPointerDown={() => { pointerFocus.current = address; }} onFocus={() => {
              if (!sameCell(pointerFocus.current, address) && !editRef.current && !sameCell(focused, address)) focusCell(address, false);
            }} onBlur={() => { if (sameCell(pointerFocus.current, address)) pointerFocus.current = null; }} onClick={() => {
              pointerFocus.current = null;
              if (editRef.current) { void apply(address, true); return; }
              if (sameCell(focused, address)) startEdit(address); else focusCell(address);
            }} onDoubleClick={() => { if (!editRef.current) startEdit(address); }} onKeyDown={(event) => cellKey(event, address, false)}>{row.cells[column.key].value}</button>}
            {row.cells[column.key].showUnit ? <span className="engineering-table-unit" title={editing && edit ? edit.captured.unit : row.cells[column.key].unit}>{(editing && edit ? edit.captured.unit : row.cells[column.key].unit) || "unit missing"}</span> : null}
            {row.cells[column.key].readout ? <small aria-label="Quantity readout">{row.cells[column.key].readout}</small> : null}
          </div>;
        })}
      </div>} />
      </div>
    </div>
    {props.persistentEditor && edit ? <div className="engineering-table-editor-layer" style={editorPosition.clip}>
      <div className="engineering-table-editor-position" style={editorPosition.box}>{editor}</div>
    </div> : null}
    {/* Keep the editor focused until a footer click; no action occurs on pointer-down. */}
    <div className="engineering-table-footer" ref={footer} role="group" aria-label={`${label} footer`} tabIndex={-1}>
      {edit ? <><span>Editing {columns.find((column) => column.key === edit.captured.columnKey)?.label} · {edit.captured.row.label}</span><button type="button" data-table-action="apply" onPointerDown={(event) => event.preventDefault()} disabled={edit.pending || busy} onClick={() => void apply(undefined, true)} title={review ? "Keep draft (Enter)" : "Apply (Enter)"}>{review ? "Keep draft" : "Apply"}</button><button type="button" data-table-action="cancel" onPointerDown={(event) => event.preventDefault()} disabled={edit.pending} onClick={cancel} title="Cancel (Escape)">Cancel</button></> : <span>{matchingRows.length} of {rows.length} rows</span>}
      {sortUnavailable ? <span role="status">Quantity sort unavailable; showing input order while values or units cannot be converted.</span> : null}
      {sort ? <button type="button" onClick={() => setSort(null)}>{sortUnavailable ? "Requested sort" : "Sorted"} by {columns.find((column) => column.key === sort.columnKey)?.label} · Clear</button> : null}
      {edit && !rows.some((row) => row.key === edit.captured.rowKey) ? <span role="alert">The edited row was removed. This retained draft cannot be applied; Cancel to return to the current model.</span>
        : edit && !matchingRows.some((row) => row.key === edit.captured.rowKey) ? <span>Editing row retained outside the filter.</span> : null}
    </div>
    {body.allocationConflict ? <p role="alert" className="engineering-table-message">No space is available for table rows. Expand the table view to continue.</p> : null}
    {edit?.error ? <p id={errorId} role="alert" className="engineering-table-message">{edit.error}</p> : null}
    {feedback ? <p role="status" className="engineering-table-message">{feedback}</p> : null}
  </div>;
}

/** Observe a finite allocated slot, never the intrinsic list content. Hidden
 * retained surfaces keep their last positive viewport until they are revealed. */
export function useTableBodyHeight(bounded: boolean, active: boolean) {
  const [element, setElement] = useState<HTMLDivElement | null>(null);
  const [height, setHeight] = useState(bounded ? 0 : 360);
  const [measured, setMeasured] = useState(false);
  useLayoutEffect(() => {
    if (!bounded || !active || !element) return;
    const measure = () => {
      if (!element.isConnected || element.closest("[hidden]") || element.clientWidth === 0) return;
      const available = element.clientHeight;
      setMeasured(true);
      setHeight((previous) => previous === available ? previous : available);
    };
    measure();
    if (typeof ResizeObserver === "undefined") return;
    const observer = new ResizeObserver(measure);
    observer.observe(element);
    return () => observer.disconnect();
  }, [bounded, active, element]);
  return { ref: setElement, height, allocationConflict: bounded && active && measured && height === 0 };
}

/** Chromium drops native text-Undo when an input's ancestor is moved, even if
 * React restores focus to the same node. Projected families keep the editor in
 * one DOM position and only move its visual box as the keyed rows reorder. */
function usePersistentEditorPosition(root: React.RefObject<HTMLDivElement | null>, enabled: boolean, token: number | undefined, rows: readonly TableRow[], active: boolean) {
  const [position, setPosition] = useState<{ clip: React.CSSProperties; box: React.CSSProperties }>({ clip: { visibility: "hidden" }, box: {} });
  useLayoutEffect(() => {
    if (!enabled) return;
    const host = root.current;
    if (!host) return;
    const measure = () => {
      const anchor = host.querySelector<HTMLElement>(`[data-editor-anchor="${token}"]`);
      const grid = host.querySelector<HTMLElement>('[role="grid"]');
      if (!anchor || !grid || !active) {
        setPosition((previous) => previous.clip.visibility === "hidden" ? previous : { clip: { visibility: "hidden" }, box: previous.box }); return;
      }
      // Inert ancestry already excludes interaction, but still has a live
      // layout (e.g. a page over the stage). Keep measuring that layout. A
      // display-hidden surface keeps its last usable placement until revealed.
      if (host.closest("[hidden]")) return;
      const a = anchor.getBoundingClientRect(), h = host.getBoundingClientRect(), g = grid.getBoundingClientRect();
      if (a.width <= 0 || a.height <= 0 || h.width <= 0 || h.height <= 0) return;
      const b = host.querySelector<HTMLElement>(".engineering-table-body-slot")!.getBoundingClientRect();
      const top = Math.max(a.top, b.top, g.top), right = Math.min(a.right, b.right, g.left + grid.clientWidth);
      const bottom = Math.min(a.bottom, b.bottom, g.top + grid.clientHeight), left = Math.max(a.left, b.left, g.left);
      const next = { clip: { clipPath: `inset(${Math.max(0, top - h.top)}px ${Math.max(0, h.right - right)}px ${Math.max(0, h.bottom - bottom)}px ${Math.max(0, left - h.left)}px)` },
        box: { left: a.left - h.left, top: a.top - h.top, width: a.width, height: a.height } };
      setPosition((previous) => JSON.stringify(previous) === JSON.stringify(next) ? previous : next);
    };
    measure();
    host.addEventListener("scroll", measure, true);
    const observer = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(measure);
    observer?.observe(host);
    const anchor = host.querySelector<HTMLElement>(`[data-editor-anchor="${token}"]`); if (anchor) observer?.observe(anchor);
    return () => { host.removeEventListener("scroll", measure, true); observer?.disconnect(); };
  }, [root, enabled, token, rows, active]);
  return position;
}
