// Run-sheet entry, slice S2 (DEL-07-02 Design `run-sheet-entry.md` §3.6): the
// sheet's state, the session events it follows, the history ledger and the
// reconciliation. Every function here is pure: the workspace session computes a
// staged transition from an event in its Prepare phase (a throw changes
// nothing) and publishes it with one setter (`runSheetStore.ts`).
//
// A row's status is never read from entity existence. It combines the ledger,
// which mirrors the session's undo and redo stacks and says which of the row's
// actions are in effect, with the reconciliation of their expected records
// against the current model, on every model revision.
import type { OperationBatch, OperationBatchOutcome } from "../../../services/operationBatchService";
import type { OperationOutcomeDiagnostic, PreviewModel } from "../../../types";
import type { SessionEvent } from "../../workspace/sessionEvents";
import { mapOutcomeToRows, type RunSheetRowBuild } from "./runSheetBatch";
import { DEFAULT_NUMBERING, initialIdState, type QuantityColumn, type RunSheetIdState, type RunSheetNumbering, type RunSheetRowInput } from "./runSheetModel";

/** A row's entered cell texts (the row input without its key). */
export type RowValues = Omit<RunSheetRowInput, "key">;
export type OwnedType = "node" | "pipe" | "support" | "component";
export type OwnedRef = Readonly<{ type: OwnedType; id: string }>;
/** One expected record: the JSON the apply's `applied_model` held, or `null` for "absent". */
export type ExpectedRecord = Readonly<{ ref: OwnedRef; record: unknown | null }>;
export type RowAction = "create" | "edit" | "insert" | "delete" | "refresh";
export type RowEntry = Readonly<{
  batchId: string;
  action: RowAction;
  ownedIds: readonly OwnedRef[];
  /** Outstanding tombstones: ids the row once owned that a sheet checkpoint removed. */
  removedIds: readonly OwnedRef[];
  valuesAfter: RowValues;
  expected: readonly ExpectedRecord[];
}>;
export type CheckpointRecord = Readonly<{ operationId: string; foreign: boolean; rowEntries: Readonly<Record<string, RowEntry>> }>;
export type Ledger = Readonly<{
  /** Folded entries: in effect, and no longer undoable from the sheet. */
  base: Readonly<Record<string, RowEntry>>;
  records: readonly CheckpointRecord[];
  /** Records `[0, cursor)` are in effect, in apply order; `[cursor, end)` are undone. */
  cursor: number;
}>;

export type PendingMarker = Readonly<{ batchId: string; registrationKey: string | null; action: RowAction }>;
export type SheetRow = Readonly<{
  key: string;
  /** Values the user typed; the only layer that counts as "edited, not applied". */
  edits: RowValues;
  /** Values an Undo put back into a draft row; synthetic, cleared by Redo. */
  restored: RowValues;
  pending: PendingMarker | null;
  requeueNeeded: boolean;
  /** The engine's refusal of this row's last queued batch, mapped through its rowKey. */
  engineRefusal: Readonly<{ batchId: string; diagnostics: readonly OperationOutcomeDiagnostic[] }> | null;
}>;

/** What the sheet handed to the review queue: the rows acting, their operations and owned ids. */
export type SubmittedBatch = Readonly<{
  batchId: string;
  batch: OperationBatch;
  rows: readonly RunSheetRowBuild[];
  ownedRefs: Readonly<Record<string, readonly OwnedRef[]>>;
  values: Readonly<Record<string, RowValues>>;
}>;

export type RunSheetHeader = Readonly<{ lengthUnit: string; source: string; numbering: RunSheetNumbering }>;
export type RunSheetState = Readonly<{
  /** The project the sheet belongs to; a different project starts an empty sheet. */
  projectId: string | null;
  sheetKey: string;
  /** The next batch sequence; each Queue and Requeue takes a new batch id. */
  batchSeq: number;
  header: RunSheetHeader;
  columnUnits: Readonly<Partial<Record<QuantityColumn, string>>>;
  rows: readonly SheetRow[];
  idState: RunSheetIdState;
  ledger: Ledger;
  submitted: Readonly<Record<string, SubmittedBatch>>;
  /** "changed by row ⟨n⟩" markers (§2.5), by row key: the edited row's key. */
  changedBy: Readonly<Record<string, string>>;
  /** Row keys counter (internal keys, never ids). */
  nextRowKey: number;
}>;

export type { SessionEvent };

/** The session keeps 25 checkpoints; records deeper than this behind the cursor can never move again. */
export const SESSION_HISTORY_DEPTH = 25;
export const emptyLedger = (): Ledger => ({ base: {}, records: [], cursor: 0 });
export const blankRow = (key: string): SheetRow => ({ key, edits: {}, restored: {}, pending: null, requeueNeeded: false, engineRefusal: null });

export function initialRunSheetState(projectId: string | null, sheetKey = "sheet-1", rowCount = 1): RunSheetState {
  return {
    projectId, sheetKey, batchSeq: 1,
    header: { lengthUnit: "", source: "", numbering: { ...DEFAULT_NUMBERING } },
    columnUnits: {},
    rows: Array.from({ length: rowCount }, (_, index) => blankRow(`r${index + 1}`)),
    idState: initialIdState(DEFAULT_NUMBERING),
    ledger: emptyLedger(),
    submitted: {},
    changedBy: {},
    nextRowKey: rowCount + 1,
  };
}

const sameValues = (a: unknown, b: unknown) => (a ?? "") === (b ?? "");
/** JSON value equality: deep, independent of key order, numbers by value (so +0 and −0 agree). */
export function sameJsonValue(a: unknown, b: unknown): boolean {
  if (a === b) return true;
  if (typeof a === "number" && typeof b === "number") return a === b;
  if (Array.isArray(a) || Array.isArray(b)) return Array.isArray(a) && Array.isArray(b) && a.length === b.length && a.every((value, index) => sameJsonValue(value, b[index]));
  if (!a || !b || typeof a !== "object" || typeof b !== "object") return false;
  const ka = Object.keys(a).filter((key) => (a as Record<string, unknown>)[key] !== undefined).sort();
  const kb = Object.keys(b).filter((key) => (b as Record<string, unknown>)[key] !== undefined).sort();
  return ka.length === kb.length && ka.every((key, index) => key === kb[index] && sameJsonValue((a as Record<string, unknown>)[key], (b as Record<string, unknown>)[key]));
}
const clone = <T,>(value: T): T => (value === undefined ? value : JSON.parse(JSON.stringify(value)) as T);
export const refKey = (ref: OwnedRef) => JSON.stringify([ref.type, ref.id]);
const COLLECTION: Readonly<Record<OwnedType, "nodes" | "pipe_segments" | "supports" | "components">> = {
  node: "nodes", pipe: "pipe_segments", support: "supports", component: "components",
};
export function recordIn(model: PreviewModel, ref: OwnedRef): unknown | null {
  const list = (model as unknown as Record<string, Array<{ id?: string }> | undefined>)[COLLECTION[ref.type]] ?? [];
  return list.find((item) => item?.id === ref.id) ?? null;
}

/** The entries in effect, in apply order: the folded base, then records `[0, cursor)`. */
export function entriesInEffect(ledger: Ledger, rowKey: string): RowEntry[] {
  const out: RowEntry[] = [];
  if (ledger.base[rowKey]) out.push(ledger.base[rowKey]);
  for (let index = 0; index < ledger.cursor; index += 1) {
    const entry = ledger.records[index].rowEntries[rowKey];
    if (entry) out.push(entry);
  }
  return out;
}
export const currentEntry = (ledger: Ledger, rowKey: string): RowEntry | null => entriesInEffect(ledger, rowKey).at(-1) ?? null;

/** The row's values as shown: `restored` overlaid by `edits`, over the current entry's `valuesAfter`. */
export function shownValues(state: RunSheetState, row: SheetRow): RowValues {
  return { ...(currentEntry(state.ledger, row.key)?.valuesAfter ?? {}), ...row.restored, ...row.edits };
}
/** Values typed that no in-effect entry holds ("edited, not applied"), for a row with a current entry. */
export function editedNotApplied(state: RunSheetState, row: SheetRow): string[] {
  const entry = currentEntry(state.ledger, row.key);
  if (!entry) return [];
  return Object.keys(row.edits).filter((key) => !sameValues((row.edits as Record<string, unknown>)[key], (entry.valuesAfter as Record<string, unknown>)[key]));
}

function updateRows(state: RunSheetState, update: (row: SheetRow) => SheetRow): RunSheetState {
  let changed = false;
  const rows = state.rows.map((row) => {
    const next = update(row);
    if (next !== row) changed = true;
    return next;
  });
  return changed ? { ...state, rows } : state;
}

/** Undone records go when a new checkpoint is pushed: the session cleared its redo stack. */
function disposeUndone(ledger: Ledger): Ledger {
  return ledger.records.length === ledger.cursor ? ledger : { ...ledger, records: ledger.records.slice(0, ledger.cursor) };
}
/** Fold the in-effect records into the base and discard the undone ones (§3.6). */
export function foldLedger(ledger: Ledger, keep = 0): Ledger {
  const foldCount = Math.max(0, ledger.cursor - keep);
  const base: Record<string, RowEntry> = { ...ledger.base };
  for (const record of ledger.records.slice(0, foldCount)) Object.assign(base, record.rowEntries);
  const records = keep === 0 ? [] : ledger.records.slice(foldCount);
  return { base, records, cursor: ledger.cursor - foldCount };
}
/** Append one record and move the cursor; records the session can no longer undo are folded. */
function appendRecord(ledger: Ledger, record: CheckpointRecord): Ledger {
  const disposed = disposeUndone(ledger);
  const next = { ...disposed, records: [...disposed.records, record], cursor: disposed.cursor + 1 };
  return next.cursor > SESSION_HISTORY_DEPTH ? foldLedger(next, SESSION_HISTORY_DEPTH) : next;
}

function expectedFor(model: PreviewModel, owned: readonly OwnedRef[], removed: readonly OwnedRef[]): ExpectedRecord[] {
  return [
    ...owned.map((ref) => ({ ref, record: clone(recordIn(model, ref)) })),
    ...removed.map((ref) => ({ ref, record: null })),
  ];
}
const withoutRefs = (refs: readonly OwnedRef[], drop: readonly OwnedRef[]) => {
  const dropped = new Set(drop.map(refKey));
  return refs.filter((ref) => !dropped.has(refKey(ref)));
};
const unionRefs = (a: readonly OwnedRef[], b: readonly OwnedRef[]) => {
  const seen = new Set(a.map(refKey));
  return [...a, ...b.filter((ref) => !seen.has(refKey(ref)))];
};

/** The checkpoint of a sheet batch: an entry for every acting row and every row whose owned entities it changed. */
function sheetCheckpoint(state: RunSheetState, submitted: SubmittedBatch, appliedModel: PreviewModel, outcomes: OperationBatchOutcome["operation_outcomes"]): CheckpointRecord {
  const rowEntries: Record<string, RowEntry> = {};
  for (const built of submitted.rows) {
    const previous = currentEntry(state.ledger, built.key);
    const owned = submitted.ownedRefs[built.key] ?? [];
    const previouslyOwned = previous?.ownedIds ?? [];
    const removedNow = withoutRefs(previouslyOwned, owned);
    const removedIds = withoutRefs(unionRefs(previous?.removedIds ?? [], removedNow), owned);
    rowEntries[built.key] = {
      batchId: submitted.batchId, action: previous ? "edit" : "create", ownedIds: owned, removedIds,
      valuesAfter: clone(submitted.values[built.key] ?? {}), expected: expectedFor(appliedModel, owned, removedIds),
    };
  }
  // Rows a batch affects (§3.6): every diff row's entity in some other row's owned ids.
  const touched = new Set(outcomes.flatMap((step) => step.diff_preview.map((row) => row.entity_ref)));
  for (const row of state.rows) {
    if (rowEntries[row.key]) continue;
    const previous = currentEntry(state.ledger, row.key);
    if (!previous || !previous.ownedIds.some((ref) => touched.has(ref.id))) continue;
    rowEntries[row.key] = {
      batchId: submitted.batchId, action: "refresh", ownedIds: previous.ownedIds, removedIds: previous.removedIds,
      valuesAfter: previous.valuesAfter, expected: expectedFor(appliedModel, previous.ownedIds, previous.removedIds),
    };
  }
  return { operationId: submitted.batchId, foreign: false, rowEntries };
}

function releasePending(state: RunSheetState, batchId: string, requeueNeeded: boolean, refusal?: SheetRow["engineRefusal"]): RunSheetState {
  return updateRows(state, (row) => row.pending?.batchId !== batchId ? row : {
    ...row, pending: null, requeueNeeded: requeueNeeded || row.requeueNeeded, engineRefusal: refusal === undefined ? row.engineRefusal : refusal,
  });
}

/** The pure ledger transition for one session event (§3.6 Transitions). It writes nothing. */
export function applySessionEvent(state: RunSheetState, event: SessionEvent): RunSheetState {
  switch (event.type) {
    case "queued":
      return updateRows(state, (row) => row.pending?.batchId === event.batchId && row.pending.registrationKey !== event.key
        ? { ...row, pending: { ...row.pending, registrationKey: event.key } } : row);
    case "validated":
      return state;
    case "refused": {
      const submitted = state.submitted[event.batchId];
      if (!submitted || !state.rows.some((row) => row.pending?.batchId === event.batchId)) return state;
      const checks = mapOutcomeToRows(event.outcome, { batch: submitted.batch, rows: submitted.rows });
      return updateRows(state, (row) => {
        if (row.pending?.batchId !== event.batchId) return row;
        const check = checks.get(row.key);
        const diagnostics = check && check.status !== "not_checked" ? check.diagnostics.filter((item) => item.severity === "blocking") : [];
        const batchLevel = row.key === submitted.rows[0]?.key ? event.outcome.diagnostics.filter((item) => item.severity === "blocking") : [];
        return { ...row, pending: null, engineRefusal: { batchId: event.batchId, diagnostics: [...diagnostics, ...batchLevel] } };
      });
    }
    case "invalidated":
    case "abandoned":
      return releasePending(state, event.batchId, true);
    case "withdrawn":
      return releasePending(state, event.batchId, false);
    case "checkpoint": {
      const submitted = state.submitted[event.operationId];
      const pending = submitted && state.rows.some((row) => row.pending?.batchId === event.operationId);
      if (!pending || !event.appliedModel) {
        return { ...state, ledger: appendRecord(state.ledger, { operationId: event.operationId, foreign: true, rowEntries: {} }) };
      }
      const record = sheetCheckpoint(state, submitted, event.appliedModel, event.operationOutcomes ?? []);
      const next = { ...state, ledger: appendRecord(state.ledger, record) };
      return updateRows(next, (row) => row.pending?.batchId !== event.operationId ? row
        : { ...row, pending: null, edits: {}, restored: {}, requeueNeeded: false, engineRefusal: null });
    }
    case "undone": {
      const ledger = state.ledger;
      const record = ledger.records[ledger.cursor - 1];
      if (!record || record.operationId !== event.operationId) return { ...state, ledger: foldLedger(ledger) };
      const next = { ...state, ledger: { ...ledger, cursor: ledger.cursor - 1 } };
      return updateRows(next, (row) => {
        const entry = record.rowEntries[row.key];
        if (!entry || entry.action === "refresh" || currentEntry(next.ledger, row.key)) return row;
        // An acting row with no entry left in effect is draft again; its create values return as `restored`.
        return { ...row, restored: clone(entry.valuesAfter) };
      });
    }
    case "redone": {
      const ledger = state.ledger;
      const record = ledger.records[ledger.cursor];
      if (!record || record.operationId !== event.operationId) return { ...state, ledger: foldLedger(ledger) };
      const next = { ...state, ledger: { ...ledger, cursor: ledger.cursor + 1 } };
      return updateRows(next, (row) => {
        const entry = record.rowEntries[row.key];
        if (!entry) return row;
        const edits: Record<string, string> = {};
        for (const [key, value] of Object.entries(row.edits)) {
          if (!sameValues(value, (entry.valuesAfter as Record<string, unknown>)[key])) edits[key] = value as string;
        }
        return { ...row, restored: {}, edits };
      });
    }
    case "history-reset":
      if (event.projectId !== state.projectId) return initialRunSheetState(event.projectId, state.sheetKey);
      return { ...state, ledger: foldLedger(state.ledger) };
  }
}

export type RowStatus = "draft" | "queued" | "applied" | "deleted" | "detached";
export type RowReconciliation = Readonly<{
  status: RowStatus;
  /** Ids whose records differ from the expectation ("Changed outside the sheet"). */
  changed: readonly string[];
  /** Tombstoned ids that exist again ("Restored outside the sheet"). */
  restoredOutside: readonly string[];
  entry: RowEntry | null;
}>;
/** Look up an entity's current record; `undefined` when absent. */
export type RecordLookup = (ref: OwnedRef) => unknown | undefined;

/** Reconciliation (§3.6): every row with a current entry, by one lookup and one
 * JSON-value comparison per expected id; no collection is scanned. */
export function reconcileRunSheet(state: RunSheetState, lookup: RecordLookup): Map<string, RowReconciliation> {
  const out = new Map<string, RowReconciliation>();
  for (const row of state.rows) {
    const entry = currentEntry(state.ledger, row.key);
    if (row.pending) {
      out.set(row.key, { status: "queued", changed: [], restoredOutside: [], entry });
      continue;
    }
    if (!entry) {
      out.set(row.key, { status: "draft", changed: [], restoredOutside: [], entry: null });
      continue;
    }
    const changed: string[] = [];
    const restoredOutside: string[] = [];
    for (const expected of entry.expected) {
      const current = lookup(expected.ref);
      if (expected.record === null) {
        if (current !== undefined) restoredOutside.push(expected.ref.id);
      } else if (current === undefined || !sameJsonValue(current, expected.record)) changed.push(expected.ref.id);
    }
    const status: RowStatus = changed.length || restoredOutside.length ? "detached" : entry.action === "delete" ? "deleted" : "applied";
    out.set(row.key, { status, changed, restoredOutside, entry });
  }
  return out;
}

/** Adopt current values (§3.6): only when every owned id exists; replaces `expected` with no operation. */
export function adoptCurrentValues(state: RunSheetState, rowKey: string, lookup: RecordLookup): RunSheetState | null {
  const entry = currentEntry(state.ledger, rowKey);
  if (!entry || entry.ownedIds.some((ref) => lookup(ref) === undefined)) return null;
  const back = entry.removedIds.filter((ref) => lookup(ref) !== undefined);
  const ownedIds = [...entry.ownedIds, ...back];
  const removedIds = withoutRefs(entry.removedIds, back);
  const adopted: RowEntry = {
    ...entry, ownedIds, removedIds,
    expected: [...ownedIds.map((ref) => ({ ref, record: clone(lookup(ref)) })), ...removedIds.map((ref) => ({ ref, record: null }))],
  };
  // The adopted entry replaces the row's current entry where it sits in the ledger.
  const ledger = state.ledger;
  for (let index = ledger.cursor - 1; index >= 0; index -= 1) {
    if (ledger.records[index].rowEntries[rowKey]) {
      const records = ledger.records.map((record, at) => at !== index ? record : { ...record, rowEntries: { ...record.rowEntries, [rowKey]: adopted } });
      return { ...state, ledger: { ...ledger, records } };
    }
  }
  return { ...state, ledger: { ...ledger, base: { ...ledger.base, [rowKey]: adopted } } };
}

/** Remove from sheet (§3.6): the row and its tombstones go, with no operation. */
export function removeRowFromSheet(state: RunSheetState, rowKey: string): RunSheetState {
  const strip = (entries: Readonly<Record<string, RowEntry>>) => {
    if (!entries[rowKey]) return entries;
    const { [rowKey]: _removed, ...rest } = entries;
    return rest;
  };
  return {
    ...state,
    rows: state.rows.filter((row) => row.key !== rowKey),
    ledger: { ...state.ledger, base: strip(state.ledger.base), records: state.ledger.records.map((record) => ({ ...record, rowEntries: strip(record.rowEntries) })) },
  };
}
