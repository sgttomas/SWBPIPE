// Run sheet S2 (DEL-07-02 Design `run-sheet-entry.md` §1, §2.5, §3.5, §3.6, §4,
// §5): the "Run sheet" tab of the Model tree's Grid mode. A draft surface: its
// rows emit one ordinary operation batch through the existing review queue. It
// suggests no section, material, orientation, radius or k; every value is
// typed, inherited from a typed value, read from a stored record, or derived by
// a stated rule.
import { useEffect, useId, useLayoutEffect, useMemo, useRef, useState, type KeyboardEvent as ReactKeyboardEvent, type ReactElement } from "react";
import { computeModelHash } from "../../../services/hashService";
import { validateOperationBatch, type OperationBatch } from "../../../services/operationBatchService";
import { convertDisplayQuantities } from "../../../services/displayQuantityService";
import { acceptedUnits, loadUnitCatalog, unitEntryMatchesDimension, type UnitCatalogRoute } from "../../../services/unitCatalogService";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../../types";
import type { QueuedBatch } from "../../toolkit/BatchReviewPanel";
import { ContextMenu, isContextMenuKey, keyboardMenuRequest, type ContextMenuEntry, type ContextMenuRequest } from "../../workspace/contextMenu/ContextMenu";
import type { ViewportViewCommand } from "../../viewport/viewportSelection";
import { mapOutcomeToRows } from "./runSheetBatch";
import { adoptCurrentValues, applySessionEvent, currentEntry, reconcileRunSheet, removeRowFromSheet, type RunSheetState } from "./runSheetLedger";
import { KIND_CODES, MESSAGES, QUANTITY_COLUMNS, SUPPORT_CODES, reservedOperationIds, type PipeNumberFrom, type QuantityColumn, type RunSheetFinding, type RunSheetNumbering } from "./runSheetModel";
import { CENTER_OF_GRAVITY_CONVENTION } from "../../component-creation/componentIntent";
import { componentField } from "./runSheetComponents";
import type { RunSheetStore } from "./runSheetStore";
import {
  CODE_LISTS, COLUMN_GROUP_LABELS, FAST_PATH, ID_COLUMNS, SHEET_COLUMNS, assignIds, buildDraftBatch, cellName, cellView, deleteDraftRow, discardEdit, effectiveText, insertRow,
  lengthUnitLock, markSubmitted, recordShownPipeIds, releaseUnregistered, retryNumbering, rowHeaderText, sentenceFragment, setCell, sheetView,
  type SheetColumn, type SheetColumnGroup, type SheetColumnKey, type SheetView, type ViewRow,
} from "./runSheetView";

/** What the shell supplies: the store, the queue's reservations and the queue itself. */
export type RunSheetHost = Readonly<{
  store: RunSheetStore;
  queuedBatches: readonly QueuedBatch[];
  editorIntents: readonly EditorOperationIntent[];
  registerBatch: (batch: OperationBatch, options: { reveal: false }) => Promise<string | null>;
}>;
type Props = Readonly<{
  host: RunSheetHost;
  model: PreviewModel;
  operationBusy?: boolean;
  viewCommand?: ((command: ViewportViewCommand) => void) | null;
  onSelect?: (ref: EntityRef) => void;
}>;

/** The engine check of the built batch runs this long after a committed change (§4.2, debounced). */
export const RUN_SHEET_TIMING = { engineCheckDelayMs: 250 };
const ORIGIN_GLYPH: Readonly<Record<string, string>> = { inherited: "↑", stored: "⤷" };
const isPrintable = (event: ReactKeyboardEvent) => event.key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey;
type Address = Readonly<{ rowKey: string; column: SheetColumnKey }>;
type Editing = Readonly<{ rowKey: string; column: SheetColumnKey; text: string }>;

export function RunSheet({ host, model, operationBusy = false, viewCommand, onSelect }: Props) {
  const { store } = host;
  const state = store.state;
  const sheetBatchPrefix = `batch:run-sheet-${state.sheetKey}-`;
  const reservedIds = useMemo(() => reservedOperationIds([
    ...host.queuedBatches.filter((entry) => !entry.retired && !entry.batch.batch_id.startsWith(sheetBatchPrefix)).flatMap((entry) => entry.batch.operations),
    ...host.editorIntents,
  ]), [host.queuedBatches, host.editorIntents, sheetBatchPrefix]);
  const [conversions, setConversions] = useState<Map<string, number | null>>(() => new Map());
  const [engine, setEngine] = useState<{ version: number; refusals: Map<string, RunSheetFinding[]> }>({ version: -1, refusals: new Map() });
  const [version, setVersion] = useState(0);
  const baseView = useMemo(() => sheetView({ state, statuses: store.statuses, model, conversions, reservedIds, lookup: store.lookup }), [state, store.statuses, model, conversions, reservedIds, store.lookup]);
  const view = useMemo(() => engine.version === version && engine.refusals.size
    ? sheetView({ state, statuses: store.statuses, model, conversions, reservedIds, engineRefusals: engine.refusals, lookup: store.lookup })
    : baseView, [baseView, engine, version, state, store.statuses, model, conversions, reservedIds, store.lookup]);
  const [shownGroups, setShownGroups] = useState<Readonly<Record<SheetColumnGroup, boolean>>>({ sectionExtras: false, idsAndSources: false, componentDetails: false });
  const columns = useMemo(() => SHEET_COLUMNS.filter((column) => !column.group || shownGroups[column.group]), [shownGroups]);
  const [focus, setFocus] = useState<Address | null>(null);
  const [editing, setEditingState] = useState<Editing | null>(null);
  // The edit being finished is taken once: Enter or Tab finish it, and the
  // input's blur that follows finds nothing left to commit.
  const editingRef = useRef<Editing | null>(null);
  const setEditing = (next: Editing | null) => { editingRef.current = next; setEditingState(next); };
  const [extent, setExtent] = useState<string | null>(null);
  const [live, setLive] = useState("");
  const [changed, setChanged] = useState<ReadonlyMap<string, ReadonlyMap<SheetColumnKey, number>>>(() => new Map());
  const [menu, setMenu] = useState<{ rowKey: string; request: ContextMenuRequest; serial: number } | null>(null);
  const menuSerial = useRef(0);
  const [queueing, setQueueing] = useState(false);
  const [unitRoute, setUnitRoute] = useState<UnitCatalogRoute | null>(null);
  const tableRef = useRef<HTMLTableElement>(null);
  const footerRef = useRef<HTMLDivElement>(null);
  const pendingFocus = useRef<Address | "footer" | null>(null);
  const descPrefix = useId();
  const announce = (message: string) => setLive(message);

  useEffect(() => {
    let active = true;
    loadUnitCatalog().then((route) => { if (active) setUnitRoute(route); }).catch(() => { if (active) setUnitRoute(null); });
    return () => { active = false; };
  }, []);
  // Conversions through the engine's display-conversion facade (§6.2); never a reimplemented factor.
  useEffect(() => {
    const missing = [...baseView.resolved.missingConversions, ...baseView.displayConversions];
    if (missing.length === 0) return;
    let active = true;
    convertDisplayQuantities(missing).then((results) => {
      if (!active) return;
      setConversions((current) => {
        const next = new Map(current);
        missing.forEach((request, index) => next.set(request.id, results[index]?.status === "converted" ? results[index].value : null));
        return next;
      });
    }).catch(() => { /* The rows keep "converting"; Queue stays disabled. */ });
    return () => { active = false; };
  }, [baseView.resolved.missingConversions, baseView.displayConversions]);
  // The engine's rules (§4.2): the built batch is validated after each committed change, debounced.
  useEffect(() => {
    if (baseView.queueBlocked || queueing) return;
    const at = version;
    let active = true;
    const timer = setTimeout(() => {
      void (async () => {
        try {
          const build = await buildDraftBatch(baseView, model);
          const hash = await computeModelHash(model);
          if (!hash || !active) return;
          const outcome = await validateOperationBatch(model, build.batch, hash);
          if (!active) return;
          setEngine({ version: at, refusals: engineFindings(mapOutcomeToRows(outcome, build), outcome.diagnostics, build.rows[0]?.key) });
        } catch {
          // The engine check is advisory here; Queue validates again.
        }
      })();
    }, RUN_SHEET_TIMING.engineCheckDelayMs);
    return () => { active = false; clearTimeout(timer); };
  }, [baseView, model, version, queueing]);

  // §3.2 A pipe id numbered by its start node is fixed the first time it is shown.
  useLayoutEffect(() => {
    if (store.current() !== state) return;
    const next = recordShownPipeIds(state, baseView);
    if (next !== state) store.update(() => next);
  }, [baseView, state, store]);

  /** Show or hide a column group. Collapsing the group that holds the focused cell moves the
   * sheet's tab stop to the nearest shown cell before it in the same row (§5.1). */
  function toggleGroup(group: SheetColumnGroup) {
    const hiding = shownGroups[group];
    setShownGroups((current) => ({ ...current, [group]: !current[group] }));
    if (!hiding || !focus) return;
    const at = SHEET_COLUMNS.findIndex((column) => column.key === focus.column);
    if (SHEET_COLUMNS[at]?.group !== group) return;
    const visible = (column: SheetColumn) => !column.group || (column.group !== group && shownGroups[column.group]);
    const target = [...SHEET_COLUMNS.slice(0, at)].reverse().find(visible) ?? SHEET_COLUMNS.slice(at + 1).find(visible);
    if (target) setFocus({ rowKey: focus.rowKey, column: target.key });
  }

  // ---- Focus.
  const shownKeys = view.rows.map((row) => row.sheetRow.key);
  const rowIndex = (key: string) => shownKeys.indexOf(key);
  const viewRow = (key: string) => view.rows[rowIndex(key)];
  const cellElement = (address: Address) => tableRef.current?.querySelector<HTMLElement>(`[data-cell="${cssEscape(address.rowKey)}|${address.column}"]`) ?? null;
  const editableAt = (row: ViewRow | undefined, column: SheetColumn) => Boolean(row && row.status === "draft" && column.editable);
  function moveFocus(target: Address | "footer") {
    pendingFocus.current = target;
    if (target === "footer") setFocus(null);
    else setFocus(target);
  }
  useLayoutEffect(() => {
    const target = pendingFocus.current;
    if (!target) return;
    pendingFocus.current = null;
    // A hidden or inert sheet (a modal holds the workspace) takes no focus.
    if (tableRef.current?.closest("[hidden], [inert]")) return;
    if (target === "footer") {
      const queueButton = footerRef.current?.querySelector<HTMLButtonElement>('[data-testid="run-sheet-queue"]');
      (queueButton && !queueButton.disabled ? queueButton : footerRef.current)?.focus();
      return;
    }
    if (editing && editing.rowKey === target.rowKey && editing.column === target.column) return;
    cellElement(target)?.focus();
  });
  // A focused row that is no longer shown (Undo, Redo, deletion) follows the Delete-row rule (§5.1).
  const lastShown = useRef<string[]>(shownKeys);
  useLayoutEffect(() => {
    const previous = lastShown.current;
    lastShown.current = shownKeys;
    if (!focus || shownKeys.includes(focus.rowKey)) return;
    if (tableRef.current?.closest("[hidden], [inert]")) { setFocus(null); return; }
    const at = previous.indexOf(focus.rowKey);
    const following = previous.slice(at + 1).find((key) => shownKeys.includes(key));
    const preceding = previous.slice(0, Math.max(0, at)).reverse().find((key) => shownKeys.includes(key));
    const inSheet = tableRef.current?.contains(document.activeElement) || document.activeElement === document.body;
    const next = following ?? preceding;
    if (!inSheet) { setFocus(next ? { rowKey: next, column: focus.column } : null); return; }
    moveFocus(next ? { rowKey: next, column: focus.column } : "footer");
  });

  // ---- Edits.
  function update(change: (current: RunSheetState) => RunSheetState) {
    store.update(change);
    setVersion((value) => value + 1);
  }
  function commit(rowKey: string, column: SheetColumnKey, text: string) {
    const before = sheetView({ state: store.current(), statuses: store.statuses, model, conversions, reservedIds, lookup: store.lookup });
    let next = setCell(store.current(), rowKey, column, text);
    if (text.trim() || column === "kind") next = assignIds(next, rowKey, model, reservedIds);
    update(() => next);
    const after = sheetView({ state: next, statuses: store.statuses, model, conversions, reservedIds, lookup: store.lookup });
    markChanges(before, after, rowKey);
    const row = after.rows.find((item) => item.sheetRow.key === rowKey);
    if (row?.refusals.length) announce(`Row ${row.resolved.number} refused: ${sentenceFragment(row.refusals[0].message)}.`);
  }
  /** §2.5 Later cells whose effective value the edit changed read "changed by row ⟨n⟩". */
  function markChanges(before: SheetView, after: SheetView, editedKey: string) {
    const edited = after.rows.find((row) => row.sheetRow.key === editedKey);
    if (!edited) return;
    const at = after.rows.indexOf(edited);
    const unit = (quantity: QuantityColumn) => columnUnitFor(quantity);
    const marks = new Map(changed);
    for (const row of after.rows.slice(at + 1)) {
      const old = before.rows.find((item) => item.sheetRow.key === row.sheetRow.key);
      if (!old) continue;
      const cells = new Map(marks.get(row.sheetRow.key) ?? []);
      for (const column of SHEET_COLUMNS) {
        if (cellView(before, old, column, unit).text !== cellView(after, row, column, unit).text) cells.set(column.key, edited.resolved.number);
      }
      if (cells.size) marks.set(row.sheetRow.key, cells);
    }
    setChanged(marks);
  }
  function clearMarks(rowKey?: string) {
    if (rowKey === undefined) { if (changed.size) setChanged(new Map()); return; }
    if (!changed.has(rowKey)) return;
    const next = new Map(changed);
    next.delete(rowKey);
    setChanged(next);
  }
  function addRow(index: number, focusColumn: SheetColumnKey = "dx") {
    const { state: next, key } = insertRow(store.current(), index);
    update(() => next);
    announce(`Row ${index + 1} added.`);
    moveFocus({ rowKey: key, column: focusColumn });
    return key;
  }
  function removeRow(rowKey: string) {
    const index = rowIndex(rowKey);
    const column = focus?.column ?? "dx";
    const nextKey = shownKeys[index + 1] ?? shownKeys[index - 1];
    update((current) => deleteDraftRow(current, rowKey));
    announce(`Row ${index + 1} deleted.`);
    moveFocus(nextKey ? { rowKey: nextKey, column } : "footer");
  }
  function fillDown(from: Address) {
    const row = viewRow(from.rowKey);
    const column = SHEET_COLUMNS.find((item) => item.key === from.column)!;
    if (!row || !column.editable) return;
    const value = effectiveText(row.resolved, from.column, typedText(row, from.column));
    const start = rowIndex(from.rowKey);
    const end = extent && rowIndex(extent) > start ? rowIndex(extent) : start + 1;
    const targets = view.rows.slice(start + 1, end + 1).filter((item) => item.status === "draft").map((item) => item.sheetRow.key);
    if (!targets.length) { announce("Fill down needs following draft rows."); return; }
    update((current) => targets.reduce((next, key) => {
      const written = setCell(next, key, from.column, value);
      return value.trim() || from.column === "kind" ? assignIds(written, key, model, reservedIds) : written;
    }, current));
    announce(`Filled ${column.label} into ${targets.length} row${targets.length === 1 ? "" : "s"}.`);
  }

  // ---- Queue (§3.5): one batch from all draft rows, validated, then handed to the review queue.
  // Queue never reads a render: an edit committed in the same keystroke (Ctrl/Cmd+Enter in an
  // editor) is already in the store's latest state. The batch and the ledger's queued values come
  // from one snapshot of that state, resolved against the latest model; if the sheet changes while
  // the batch is built, nothing is queued.
  const queueingRef = useRef(false);
  const latest = useRef({ model, reservedIds, conversions });
  latest.current = { model, reservedIds, conversions };
  async function queue() {
    if (queueingRef.current || operationBusy) return;
    const { model: basis, reservedIds: reserved, conversions: known } = latest.current;
    const viewOf = (state: RunSheetState) => sheetView({ state, statuses: reconcileRunSheet(state, store.lookup), model: basis, conversions: known, reservedIds: reserved, lookup: store.lookup });
    // Pipe ids an edit in the same keystroke shows for the first time are fixed before the
    // snapshot (§3.2), so the display's own recording cannot look like a change to the sheet.
    const shown = store.current();
    const recorded = recordShownPipeIds(shown, viewOf(shown));
    if (recorded !== shown) store.update(() => recorded);
    const snapshot = store.current();
    const fresh = viewOf(snapshot);
    if (fresh.queueBlocked) {
      announce(`Queue disabled: ${fresh.queueBlocked}`);
      return;
    }
    queueingRef.current = true;
    setQueueing(true);
    clearMarks();
    try {
      const build = await buildDraftBatch(fresh, basis);
      const batchId = build.batch.batch_id;
      const acting = fresh.draftRows;
      if (store.current() !== snapshot) {
        announce("The sheet changed while the batch was built; nothing was queued. Queue again.");
        return;
      }
      // The rows are queued, read-only, while the batch is checked and registered.
      store.update(() => markSubmitted(snapshot, fresh, build));
      const model = basis;
      const hash = await computeModelHash(model);
      if (!hash) throw new Error("The current model hash is unavailable.");
      const outcome = await validateOperationBatch(model, build.batch, hash);
      if (outcome.validation.batch_validation_status !== "passed" || outcome.simulation_disposition === "rolled_back_no_model_published") {
        store.update((current) => applySessionEvent(current, { type: "refused", batchId, key: "", outcome }));
        const refusedRow = build.rows.find((row) => mapOutcomeToRows(outcome, build).get(row.key)?.status === "refused");
        const number = refusedRow ? rowIndex(refusedRow.key) + 1 : null;
        announce(`Queue refused by the engine${number ? `: row ${number}` : ""}. The model was not changed.`);
        return;
      }
      const key = await host.registerBatch(build.batch, { reveal: false });
      if (!key) {
        store.update((current) => releaseUnregistered(current, batchId));
        announce("The batch was not queued; the model or the queue changed. Queue again.");
        return;
      }
      announce(`Queued rows ${acting.map((row) => row.resolved.number).join(", ")} as ${batchId}. Review and apply it in Pending changes.`);
    } catch (error) {
      store.update((current) => ({ ...current, rows: current.rows.map((row) => row.pending?.registrationKey === null ? { ...row, pending: null } : row) }));
      announce(`Queue failed: ${error instanceof Error ? error.message : String(error)}`);
    } finally {
      queueingRef.current = false;
      setQueueing(false);
      setVersion((value) => value + 1);
    }
  }

  // ---- Keys (§5.1, §5.2).
  /** The next or previous editable cell in column order, wrapping rows, from any cell (§5.1 Tab). */
  function tabTarget(address: Address, backwards: boolean): Address | "footer" | null {
    const order = view.rows.flatMap((row) => columns.map((column) => ({ rowKey: row.sheetRow.key, column: column.key, editable: editableAt(row, column) })));
    const at = order.findIndex((item) => item.rowKey === address.rowKey && item.column === address.column);
    const step = backwards ? -1 : 1;
    for (let index = at + step; index >= 0 && index < order.length; index += step) {
      if (order[index].editable) return { rowKey: order[index].rowKey, column: order[index].column };
    }
    return backwards ? null : "footer";
  }
  function enterTarget(address: Address): Address | "append" | null {
    const index = rowIndex(address.rowKey);
    const fast = FAST_PATH.indexOf(address.column);
    if (fast >= 0) {
      if (fast < FAST_PATH.length - 1) return { rowKey: address.rowKey, column: FAST_PATH[fast + 1] };
      const next = view.rows[index + 1];
      return next ? { rowKey: next.sheetRow.key, column: "dx" } : "append";
    }
    const next = view.rows[index + 1];
    return next ? { rowKey: next.sheetRow.key, column: address.column } : null;
  }
  function advance(address: Address) {
    const target = enterTarget(address);
    if (target === "append") addRow(view.rows.length);
    else if (target) moveFocus(target);
    else moveFocus(address);
  }
  function arrow(address: Address, key: string, shift: boolean) {
    const index = rowIndex(address.rowKey);
    const at = columns.findIndex((column) => column.key === address.column);
    if (key === "ArrowLeft" || key === "ArrowRight") {
      const next = columns[Math.max(0, Math.min(columns.length - 1, at + (key === "ArrowLeft" ? -1 : 1)))];
      moveFocus({ rowKey: address.rowKey, column: next.key });
      setExtent(null);
      return;
    }
    // Shift extends the row selection from its current extent (Fill down's "selected following rows").
    const from = shift && extent && rowIndex(extent) >= 0 ? rowIndex(extent) : index;
    const nextIndex = Math.max(0, Math.min(view.rows.length - 1, from + (key === "ArrowUp" ? -1 : 1)));
    const target = view.rows[nextIndex].sheetRow.key;
    if (shift) setExtent(target);
    else {
      setExtent(null);
      moveFocus({ rowKey: target, column: address.column });
    }
  }
  const modified = (event: ReactKeyboardEvent) => event.ctrlKey || event.metaKey || event.altKey;
  const chord = (event: ReactKeyboardEvent) => (event.ctrlKey || event.metaKey) && !event.altKey;
  function cellKeyDown(event: ReactKeyboardEvent<HTMLElement>, address: Address) {
    if (event.nativeEvent.isComposing || event.defaultPrevented) return;
    const row = viewRow(address.rowKey);
    const column = SHEET_COLUMNS.find((item) => item.key === address.column)!;
    const consume = () => { event.preventDefault(); event.stopPropagation(); };
    if (chord(event) && event.key === "Enter") { consume(); void queue(); return; }
    if (chord(event) && event.key.toLowerCase() === "d" && !event.shiftKey) { consume(); fillDown(address); return; }
    if (isContextMenuKey(event)) { consume(); openMenu(address.rowKey, event.currentTarget); return; }
    if (modified(event)) return;
    if (event.key.startsWith("Arrow")) { consume(); arrow(address, event.key, event.shiftKey); return; }
    if (event.key === "Tab") {
      const target = tabTarget(address, event.shiftKey);
      if (!target) return;
      consume();
      moveFocus(target);
      return;
    }
    if (event.key === "Enter") {
      consume();
      if (FAST_PATH.includes(address.column) || !editableAt(row, column)) advance(address);
      else startEdit(address, typedText(row, address.column));
      return;
    }
    if (event.key === "F2" && editableAt(row, column) && !column.code) { consume(); startEdit(address, typedText(row, address.column)); return; }
    if ((event.key === "Delete" || event.key === "Backspace") && editableAt(row, column)) { consume(); commit(address.rowKey, address.column, ""); return; }
    if (!isPrintable(event)) return;
    if (column.code) {
      consume();
      if (!editableAt(row, column)) { announce(readonlyReason(row)); return; }
      const code = event.key.toUpperCase();
      const valid = column.code === "kind" ? code in KIND_CODES : code in SUPPORT_CODES;
      if (valid) commit(address.rowKey, address.column, code);
      else announce(`${event.key} is not a ${column.code} code. Codes: ${CODE_LISTS[column.code]}.`);
      return;
    }
    if (editableAt(row, column)) { consume(); startEdit(address, event.key); }
    else if (row && column.editable) { consume(); announce(readonlyReason(row)); }
  }
  function startEdit(address: Address, text: string) {
    setEditing({ ...address, text });
    setFocus(address);
  }
  function finishEdit(next: Address | "footer" | "stay" | "fast" | null, queueAfter = false) {
    const current = editingRef.current;
    if (!current) return;
    setEditing(null);
    const original = typedText(viewRow(current.rowKey), current.column);
    if (current.text !== original) commit(current.rowKey, current.column, current.text);
    const address = { rowKey: current.rowKey, column: current.column };
    if (next === "fast") advance(address);
    else if (next === "stay") moveFocus(address);
    else if (next) moveFocus(next);
    if (queueAfter) void queue();
  }
  function editorKeyDown(event: ReactKeyboardEvent<HTMLInputElement>) {
    const editing = editingRef.current;
    if (!editing || event.nativeEvent.isComposing) return;
    const address = { rowKey: editing.rowKey, column: editing.column };
    const consume = () => { event.preventDefault(); event.stopPropagation(); };
    if (event.key === "Escape") { consume(); setEditing(null); moveFocus(address); return; }
    if (chord(event) && event.key === "Enter") { consume(); finishEdit("stay", true); return; }
    if (modified(event)) return;
    if (event.key === "Enter") {
      consume();
      if (FAST_PATH.includes(editing.column)) finishEdit("fast");
      else {
        const target = enterTarget(address);
        finishEdit(target && target !== "append" ? target : "stay");
      }
      return;
    }
    if (event.key === "Tab") {
      const target = tabTarget(address, event.shiftKey);
      if (!target) return;
      consume();
      finishEdit(target);
    }
  }

  // ---- Menus (§5.3).
  function openMenu(rowKey: string, invoker: HTMLElement) {
    setMenu({ rowKey, serial: ++menuSerial.current, request: keyboardMenuRequest(invoker, invoker, () => [
      cellElement({ rowKey, column: focus?.column ?? "dx" }),
      ...[...(tableRef.current?.querySelectorAll<HTMLElement>("[data-cell]") ?? [])],
      footerRef.current,
    ]) });
  }
  function menuEntries(rowKey: string): ContextMenuEntry[] {
    const row = viewRow(rowKey);
    if (!row) return [];
    const index = rowIndex(rowKey);
    const insertReason = (at: number) => {
      const after = view.rows[at];
      return after && after.status !== "draft" ? "The rows after this point are queued or applied; inserting between them comes with applied-row edits." : null;
    };
    const command = (id: string, label: string, run: () => void, reason: string | null = null, shortcut?: string): ContextMenuEntry =>
      ({ kind: "command", id, label, run, disabled: Boolean(reason), reason, shortcut });
    const entries: ContextMenuEntry[] = [
      command("insert-above", "Insert row above", () => addRow(index), insertReason(index)),
      command("insert-below", "Insert row below", () => addRow(index + 1), insertReason(index + 1)),
      command("delete-row", "Delete row", () => removeRow(rowKey), row.status === "draft" ? null : readonlyReason(row)),
      command("fill-down", "Fill down", () => fillDown({ rowKey, column: focus?.rowKey === rowKey ? focus.column : "section" }),
        row.status === "draft" && view.rows.slice(index + 1).some((item) => item.status === "draft") ? null : "No following draft rows to fill.", "Ctrl+D"),
      command("start-branch", "Start branch here", () => {
        const key = addRow(view.rows.length, "dx");
        const end = row.resolved.end?.id ?? "";
        store.update((current) => assignIds(setCell(setCell(current, key, "kind", "F"), key, "node", end), key, model, reservedIds));
        setVersion((value) => value + 1);
      }, startBranchReason(row, store.lookup)),
      command("fit-row", "Fit row in viewport", () => {
        const pipe = row.resolved.ids.pipe;
        const node = row.resolved.end?.id;
        const target = pipe ? { type: "pipe" as const, id: pipe } : node ? { type: "node" as const, id: node } : null;
        if (target) onSelect?.(target);
        viewCommand?.({ type: "fit-selection" });
      }, row.status === "applied" && viewCommand ? null : row.status === "applied" ? "The viewport is not available." : "A row that is not applied has no geometry in the model yet."),
    ];
    if (row.status === "detached") {
      const entry = currentEntry(store.current().ledger, rowKey);
      const adoptable = entry && entry.ownedIds.every((ref) => store.lookup(ref) !== undefined);
      entries.push({ kind: "separator", id: "detached" },
        command("adopt", "Adopt current values", () => {
          const next = adoptCurrentValues(store.current(), rowKey, store.lookup);
          if (next) update(() => next);
        }, adoptable ? null : "An owned entity is missing from the model."),
        command("remove", "Remove from sheet", () => update((current) => removeRowFromSheet(current, rowKey))));
    }
    if (row.editedNotApplied.length) entries.push(command("discard-edit", "Discard edit", () => update((current) => discardEdit(current, rowKey))));
    return entries;
  }

  // ---- Units.
  const projectUnit = (dimension: string) => (model.project.units as Record<string, string | undefined>)[dimension] ?? "";
  function columnUnitFor(quantity: QuantityColumn) {
    return state.columnUnits[quantity] ?? projectUnit(QUANTITY_COLUMNS[quantity].dimension);
  }
  /** The catalog's units of a dimension, with the current and project units; none without the catalog. */
  function unitOptions(dimension: string, current: string): string[] {
    if (unitRoute?.route !== "tauri_unit_catalog") return [];
    const options = new Set<string>();
    for (const entry of acceptedUnits(unitRoute.catalog)) if (unitEntryMatchesDimension(entry, dimension)) options.add(entry.symbol);
    if (options.size === 0) return [];
    if (projectUnit(dimension)) options.add(projectUnit(dimension));
    if (current) options.add(current);
    return [...options];
  }
  const unitLock = lengthUnitLock(view);
  const numberingLocked = Object.keys(state.idState.assigned).length > 0;
  const lengthUnit = state.header.lengthUnit || projectUnit("length");
  /** A numbering change; a new start moves the counter, and every refused row is numbered again (§3.2). */
  const setNumbering = (patch: Partial<RunSheetNumbering>) => update((current) => retryNumbering({
    ...current,
    idState: patch.start !== undefined ? { ...current.idState, next: patch.start } : current.idState,
    header: { ...current.header, numbering: { ...current.header.numbering, ...patch } },
  }, model, reservedIds));

  function unitControl(label: string, dimension: string, value: string, onChange: (unit: string) => void, disabledReason: string | null = null) {
    const options = unitOptions(dimension, value);
    // Without the catalog (the browser preview) the unit is typed; the engine's conversion checks it (§6.2).
    if (options.length === 0) {
      return <input aria-label={`${label} unit`} placeholder="Choose unit" value={value} disabled={Boolean(disabledReason)} title={disabledReason ?? undefined}
        onChange={(event) => onChange(event.target.value.trim())} />;
    }
    return <select aria-label={`${label} unit`} value={value} disabled={Boolean(disabledReason)} title={disabledReason ?? undefined} onChange={(event) => onChange(event.target.value)}>
      {value ? null : <option value="">Choose unit</option>}
      {options.map((unit) => <option key={unit} value={unit}>{unit}</option>)}
    </select>;
  }

  const descriptions: ReactElement[] = [];
  const statusLine = view.queueBlocked ?? (view.requeue ? "Rows were released from a withdrawn batch; Requeue prepares them on the current model." : "Ready to queue.");
  const queueReason = operationBusy ? "Wait for the current request to finish." : queueing ? "Queueing…" : view.queueBlocked;
  return (
    <section className="run-sheet" aria-label="Run sheet editor" data-testid="run-sheet"
      data-ledger-cursor={state.ledger.cursor} data-ledger-records={state.ledger.records.length}>
      <div className="run-sheet-header" role="group" aria-label="Run sheet header">
        <label>Source<input aria-label="Run sheet source" value={state.header.source} placeholder="Required"
          onChange={(event) => { const source = event.target.value; update((current) => ({ ...current, header: { ...current.header, source } })); }} /></label>
        <label>Length unit{unitControl("Length", "length", lengthUnit, (unit) => update((current) => ({ ...current, header: { ...current.header, lengthUnit: unit } })), unitLock)}</label>
        {unitLock ? <span data-testid="run-sheet-unit-lock" className="run-sheet-note">{unitLock}</span> : null}
        <fieldset disabled={numberingLocked} title={numberingLocked ? "Ids are assigned; the numbering is fixed for this sheet." : undefined}>
          <legend>Numbering</legend>
          <label>Node prefix<input aria-label="Node id prefix" value={state.header.numbering.nodePrefix} onChange={(event) => setNumbering({ nodePrefix: event.target.value })} /></label>
          <label>Pipe prefix<input aria-label="Pipe id prefix" value={state.header.numbering.pipePrefix} onChange={(event) => setNumbering({ pipePrefix: event.target.value })} /></label>
          <label>Start<input aria-label="Numbering start" inputMode="numeric" value={String(state.header.numbering.start)} onChange={(event) => setNumbering({ start: Number(event.target.value) })} /></label>
          <label>Step<input aria-label="Numbering step" inputMode="numeric" value={String(state.header.numbering.step)} onChange={(event) => setNumbering({ step: Number(event.target.value) })} /></label>
          <label>Pad numbers to N digits<input aria-label="Pad numbers to N digits" inputMode="numeric" placeholder="No padding"
            value={state.header.numbering.pad === undefined ? "" : String(state.header.numbering.pad)}
            onChange={(event) => { const text = event.target.value.trim(); setNumbering({ pad: text === "" ? undefined : Number(text) }); }} /></label>
          <label>Pipe number from<select aria-label="Pipe number from" value={state.header.numbering.pipeNumber ?? "end"}
            onChange={(event) => setNumbering({ pipeNumber: event.target.value as PipeNumberFrom })}>
            <option value="end">its end node</option>
            <option value="start">its start node</option>
          </select></label>
        </fieldset>
        {(Object.keys(COLUMN_GROUP_LABELS) as SheetColumnGroup[]).map((group) => (
          <button key={group} type="button" aria-expanded={shownGroups[group]} onClick={() => toggleGroup(group)}>{COLUMN_GROUP_LABELS[group]}</button>
        ))}
      </div>
      {shownGroups.componentDetails ? <p className="run-sheet-note" role="note">
        V creates a valve and L a flange on the row's pipe, at its end node. Component inputs never inherit; blank unit cells use the project unit, shown as derived.
        {" "}{CENTER_OF_GRAVITY_CONVENTION} Solver consumption is mechanics_geometry_only or not_solver_consumed; blank means undeclared.
        A stiffness scale is optional and requires its source reference.
      </p> : null}
      <div className="run-sheet-scroll" role="region" aria-label="Run sheet rows">
        <table ref={tableRef} aria-label="Run sheet" data-testid="run-sheet-table">
          <thead>
            <tr>
              <th scope="col">#</th>
              {columns.map((column) => (
                <th scope="col" key={column.key}>
                  {column.label}
                  {column.quantity ? <span className="run-sheet-unit">{unitControl(QUANTITY_COLUMNS[column.quantity].label, QUANTITY_COLUMNS[column.quantity].dimension, columnUnitFor(column.quantity),
                    (unit) => update((current) => ({ ...current, columnUnits: { ...current.columnUnits, [column.quantity!]: unit } })))}</span> : null}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {view.rows.map((row) => {
              const rowKey = row.sheetRow.key;
              const header = rowHeaderText(row);
              const inRange = extent && focus && rowIndex(rowKey) > rowIndex(focus.rowKey) && rowIndex(rowKey) <= rowIndex(extent);
              return (
                <tr key={rowKey} data-testid={`run-sheet-row-${rowKey}`} data-status={row.status} aria-selected={inRange ? true : undefined}
                  onFocus={() => clearMarks(rowKey)}
                  onContextMenu={(event) => { event.preventDefault(); setMenu({ rowKey, serial: ++menuSerial.current, request: { anchor: { x: event.clientX, y: event.clientY }, returnFocus: event.target instanceof HTMLElement ? event.target : null } }); }}>
                  <th scope="row" aria-label={header} data-testid={`run-sheet-row-header-${rowKey}`} className={row.refusals.length ? "refused" : undefined}
                    onKeyDown={(event) => { if (isContextMenuKey(event)) { event.preventDefault(); event.stopPropagation(); openMenu(rowKey, event.currentTarget); } }} tabIndex={-1}>
                    {row.resolved.number}{row.refusals.length ? <span aria-hidden="true" className="run-sheet-refused"> !</span> : null}
                    <span className="run-sheet-status">{statusLabel(row)}</span>
                  </th>
                  {columns.map((column) => {
                    const address = { rowKey, column: column.key };
                    const cell = cellView(view, row, column, columnUnitFor);
                    const name = cellName(row, column, cell);
                    const mark = changed.get(rowKey)?.get(column.key);
                    const descId = `${descPrefix}-${rowKey}-${column.key}`;
                    const description = [cell.origin, editableAt(row, column) || !column.editable ? null : readonlyReason(row),
                      column.code ? `Codes: ${CODE_LISTS[column.code]}` : null, cell.refusal ? `refused: ${cell.refusal}` : null,
                      mark ? `changed by row ${mark}` : null, row.editedNotApplied.includes(column.key) ? "edited, not applied" : null].filter(Boolean).join("; ");
                    const active = focus ? focus.rowKey === rowKey && focus.column === column.key : rowKey === view.rows[0]?.sheetRow.key && column.key === "dx";
                    const isEditing = editing?.rowKey === rowKey && editing.column === column.key;
                    descriptions.push(<span key={descId} id={descId}>{description}</span>);
                    const glyph = ORIGIN_GLYPH[cell.origin.startsWith("inherited from pipe") ? "stored" : cell.origin.startsWith("inherited") ? "inherited" : ""];
                    return (
                      <td key={column.key} data-cell={`${rowKey}|${column.key}`} data-testid={`run-sheet-cell-${rowKey}-${column.key}`}
                        tabIndex={isEditing ? -1 : active ? 0 : -1} aria-label={name} aria-describedby={descId} aria-readonly={!editableAt(row, column) || undefined}
                        className={[cell.muted ? "muted" : "", cell.italic ? "derived" : "", cell.refusal ? "refused" : "", mark ? "changed" : ""].filter(Boolean).join(" ") || undefined}
                        onFocus={() => { if (!isEditing && (!focus || focus.rowKey !== rowKey || focus.column !== column.key)) setFocus(address); }}
                        onClick={() => setFocus(address)}
                        onKeyDown={(event) => { if (!isEditing) cellKeyDown(event, address); }}>
                        {isEditing ? (
                          <input autoFocus aria-label={name} aria-describedby={descId} value={editing.text}
                            onChange={(event) => setEditing({ ...editing, text: event.target.value })}
                            onKeyDown={editorKeyDown}
                            onBlur={() => finishEdit(null)} />
                        ) : (
                          <>{glyph ? <span aria-hidden="true" className="run-sheet-origin">{glyph} </span> : null}{cell.text}{cell.unit && cell.text ? <span className="run-sheet-cell-unit"> {cell.unit}</span> : null}</>
                        )}
                      </td>
                    );
                  })}
                </tr>
              );
            })}
          </tbody>
        </table>
        <div hidden data-testid="run-sheet-descriptions">{descriptions}</div>
      </div>
      <div className="run-sheet-footer" ref={footerRef} tabIndex={-1} data-testid="run-sheet-footer">
        <button type="button" data-testid="run-sheet-queue" disabled={Boolean(queueReason)} title={queueReason ?? undefined} onClick={() => void queue()}>
          {view.requeue ? "Requeue rows" : "Queue rows"}
        </button>
        <button type="button" data-testid="run-sheet-add-row" onClick={() => addRow(view.rows.length)}>Add row</button>
        <span data-testid="run-sheet-queue-reason" className="run-sheet-note">{queueReason ?? statusLine}</span>
        {view.rows.filter((row) => row.status === "detached").map((row) => (
          <p key={row.sheetRow.key} role="note" data-testid={`run-sheet-detached-${row.sheetRow.key}`}>
            Row {row.resolved.number}: {(store.statuses.get(row.sheetRow.key)?.restoredOutside.length ? "Restored outside the sheet: " : "Changed outside the sheet: ")}
            {[...(store.statuses.get(row.sheetRow.key)?.changed ?? []), ...(store.statuses.get(row.sheetRow.key)?.restoredOutside ?? [])].join(", ")}
          </p>
        ))}
      </div>
      <p role="status" aria-live="polite" className="run-sheet-live" data-testid="run-sheet-live">{live}</p>
      {menu ? <ContextMenu key={menu.serial} label={`Row ${rowIndex(menu.rowKey) + 1} commands`} entries={menuEntries(menu.rowKey)} request={menu.request} onClose={() => setMenu(null)} testId="run-sheet-menu" /> : null}
    </section>
  );
}

function engineFindings(checks: ReturnType<typeof mapOutcomeToRows>, batchDiagnostics: readonly { severity: string; code: string; message: string }[], firstRow: string | undefined): Map<string, RunSheetFinding[]> {
  const out = new Map<string, RunSheetFinding[]>();
  for (const [key, check] of checks) {
    if (check.status === "refused") out.set(key, check.diagnostics.filter((item) => item.severity === "blocking").map((item) => ({ rule: "incomplete", message: `${item.code}: ${item.message}` })));
    else if (check.status === "not_checked") out.set(key, [{ rule: "incomplete", message: check.message }]);
  }
  const blocking = batchDiagnostics.filter((item) => item.severity === "blocking");
  if (blocking.length && firstRow) out.set(firstRow, [...(out.get(firstRow) ?? []), ...blocking.map((item) => ({ rule: "incomplete" as const, message: `${item.code}: ${item.message}` }))]);
  return out;
}

/** Why Start branch here is unavailable on a row, or null: never a node that does not exist. */
function startBranchReason(row: ViewRow, lookup: RunSheetStore["lookup"]): string | null {
  const applied = row.status === "applied" || row.status === "detached";
  if (row.resolved.kind === "bend" && !applied) return "A bend's corner is replaced by its tangent points when it is applied; start a branch from the applied bend.";
  if (row.resolved.endMissing) return MESSAGES.bendOutgoingMissing(row.resolved.endMissing);
  const end = row.resolved.end?.id;
  if (!end) return "This row has no end node yet.";
  if (applied && lookup({ type: "node", id: end }) === undefined) return `Node ${end} no longer exists in the model.`;
  return null;
}

function typedText(row: ViewRow | undefined, column: SheetColumnKey): string {
  if (!row) return "";
  const field = componentField(column);
  if (field) return row.values.component?.[field] ?? "";
  const member = ID_COLUMNS[column];
  if (member) return row.values.ids?.[member] ?? "";
  const value = (row.values as Record<string, unknown>)[column];
  return typeof value === "string" ? value : "";
}
function readonlyReason(row: ViewRow | undefined): string {
  switch (row?.status) {
    case "queued": return `Queued in ${row.sheetRow.pending?.batchId}; read-only until it is applied or cleared from Pending changes.`;
    case "applied": return "Applied; this row shows the stored model. Undo returns it to draft.";
    case "detached": return "Changed outside the sheet; adopt the current values or remove the row from the sheet.";
    case "deleted": return "Deleted; Undo restores it.";
    default: return "Read-only.";
  }
}
function statusLabel(row: ViewRow): string {
  const label = row.status === "draft" && row.sheetRow.requeueNeeded ? "draft, Requeue needed" : row.status;
  return row.editedNotApplied.length ? `${label}, edited, not applied` : label;
}
function cssEscape(value: string): string {
  return typeof CSS !== "undefined" && CSS.escape ? CSS.escape(value) : value.replace(/["\\]/g, "\\$&");
}
