// Run sheet S2 (DEL-07-02 Design `run-sheet-entry.md` §1–§2, §3.4, §4): the
// sheet's view, pure. It resolves every shown row with S1's resolver and lists
// what each cell shows and why. Applied rows are views of the model: their
// values are the entered values their checkpoint put in effect, resolved
// against the model without the entities the sheet's applied rows own, so the
// chain resolves exactly as it did when it was drafted and a draft row after
// an applied one continues from its stored end. Only draft rows are queued.
import type { PreviewModel } from "../../../types";
import { buildRunSheetBatch, type RunSheetBuild } from "./runSheetBatch";
import {
  INHERITING_CELLS, MESSAGES, QUANTITY_COLUMNS, assignRowNumber, idsForNumber, modelIds, resolveRunSheet,
  type CellOrigin, type QuantityColumn, type ResolvedRunSheet, type ResolvedRunSheetRow, type RunSheetConversions, type RunSheetFinding, type RunSheetRowInput,
} from "./runSheetModel";
import {
  blankRow, currentEntry, editedNotApplied, shownValues,
  type OwnedRef, type RowReconciliation, type RowStatus, type RowValues, type RunSheetState, type SheetRow, type SubmittedBatch,
} from "./runSheetLedger";

export type SheetColumnKey =
  | "dx" | "dy" | "dz" | "kind" | "support" | "restraints" | "node" | "section" | "od" | "wall" | "material" | "orientation"
  | "insulationThickness" | "insulationDensity" | "contentsDensity" | "materialDensity" | "millTolerance" | "corrosionAllowance"
  | "bendRadius" | "k" | "kSource" | "label" | "nodeId" | "pipeId" | "position" | "length";
export type SheetColumn = Readonly<{
  key: SheetColumnKey;
  label: string;
  editable: boolean;
  code?: "kind" | "support";
  quantity?: QuantityColumn;
  extra?: boolean;
}>;
export const SHEET_COLUMNS: readonly SheetColumn[] = [
  { key: "dx", label: "dX", editable: true },
  { key: "dy", label: "dY", editable: true },
  { key: "dz", label: "dZ", editable: true },
  { key: "kind", label: "Kind", editable: true, code: "kind" },
  { key: "support", label: "Support", editable: true, code: "support" },
  { key: "restraints", label: "Restraints", editable: true },
  { key: "node", label: "Node", editable: true },
  { key: "section", label: "Section", editable: true },
  { key: "od", label: "OD", editable: true, quantity: "od" },
  { key: "wall", label: "Wall", editable: true, quantity: "wall" },
  { key: "material", label: "Material", editable: true },
  { key: "orientation", label: "Orientation", editable: true },
  { key: "insulationThickness", label: "Ins. thk", editable: true, quantity: "insulationThickness" },
  { key: "insulationDensity", label: "Ins. dens.", editable: true, quantity: "insulationDensity" },
  { key: "contentsDensity", label: "Contents dens.", editable: true, quantity: "contentsDensity" },
  { key: "materialDensity", label: "Material density", editable: true, quantity: "materialDensity", extra: true },
  { key: "millTolerance", label: "Mill tolerance", editable: true, quantity: "millTolerance", extra: true },
  { key: "corrosionAllowance", label: "Corrosion allowance", editable: true, quantity: "corrosionAllowance", extra: true },
  { key: "bendRadius", label: "Bend R", editable: true, quantity: "bendRadius" },
  { key: "k", label: "k", editable: true },
  { key: "kSource", label: "k source", editable: true },
  { key: "label", label: "Label", editable: true },
  { key: "nodeId", label: "Node id", editable: true },
  { key: "pipeId", label: "Pipe id", editable: true },
  { key: "position", label: "Position", editable: false },
  { key: "length", label: "Length", editable: false },
];
/** §5.1 The Enter fast path: dX → dY → dZ → Kind → Support → the next row's dX. */
export const FAST_PATH: readonly SheetColumnKey[] = ["dx", "dy", "dz", "kind", "support"];
export const KIND_NAMES: Readonly<Record<string, string>> = { run: "run", bend: "bend run", support: "support only", from: "start from node", point: "start at point" };
export const CODE_LISTS = {
  kind: "R run, B bend run, S support only, F start from node, P start at point; blank is a run",
  support: "A anchor, G guide, L line stop, V vertical support; blank is no support",
} as const;

/** The row input as entered, for any row (draft rows read their buffer; others their in-effect values). */
export function rowInput(state: RunSheetState, row: SheetRow, status: RowStatus): RunSheetRowInput {
  const values = status === "draft" || status === "queued" ? shownValues(state, row) : currentEntry(state.ledger, row.key)?.valuesAfter ?? {};
  return { key: row.key, ...values };
}

/** The model without the entities the sheet's applied rows own (the chain resolves as drafted). */
export function sheetBasisModel(model: PreviewModel, state: RunSheetState, statuses: ReadonlyMap<string, RowReconciliation>): PreviewModel {
  const owned = new Map<string, Set<string>>();
  for (const row of state.rows) {
    const status = statuses.get(row.key)?.status;
    if (status !== "applied" && status !== "detached") continue;
    for (const ref of currentEntry(state.ledger, row.key)?.ownedIds ?? []) {
      if (!owned.has(ref.type)) owned.set(ref.type, new Set());
      owned.get(ref.type)!.add(ref.id);
    }
  }
  if (owned.size === 0) return model;
  const keep = (type: OwnedRef["type"]) => (item: { id: string }) => !owned.get(type)?.has(item.id);
  return {
    ...model,
    nodes: model.nodes.filter(keep("node")),
    pipe_segments: model.pipe_segments.filter(keep("pipe")),
    supports: (model.supports ?? []).filter(keep("support")),
    components: model.components.filter(keep("component")),
  };
}

export type ViewRow = Readonly<{
  sheetRow: SheetRow;
  status: RowStatus;
  resolved: ResolvedRunSheetRow;
  /** Every refusal shown on the row: the sheet rules, the engine's, and the sheet's lifecycle rules. */
  refusals: readonly RunSheetFinding[];
  editedNotApplied: readonly string[];
  /** The entered values shown: the in-effect values, overlaid by `restored` and `edits`. */
  values: RowValues;
}>;
export type SheetView = Readonly<{
  resolved: ResolvedRunSheet;
  rows: readonly ViewRow[];
  /** The draft rows a Queue would emit. */
  draftRows: readonly ViewRow[];
  /** Why Queue is disabled, or null. */
  queueBlocked: string | null;
  /** True when any draft row was released from a retired or abandoned batch. */
  requeue: boolean;
}>;

export const LIFECYCLE_MESSAGES = {
  waits: (batch: string) => `Waits for batch ${batch} to be applied.`,
  detachedPredecessor: (row: number) => `Row ${row} changed outside the sheet; start this chain again with F.`,
  existsNow: (id: string) => `Id ${id} now exists in the model.`,
  engine: (code: string, message: string) => `${code}: ${message}`,
} as const;

/** The view of the whole sheet (§2, §4.2), resolved synchronously with the conversions known so far. */
export function sheetView(input: {
  state: RunSheetState;
  statuses: ReadonlyMap<string, RowReconciliation>;
  model: PreviewModel;
  conversions: RunSheetConversions;
  reservedIds: Iterable<string>;
  engineRefusals?: ReadonlyMap<string, readonly RunSheetFinding[]>;
}): SheetView {
  const { state, statuses, model } = input;
  const shown = state.rows.filter((row) => statuses.get(row.key)?.status !== "deleted");
  const statusOf = (row: SheetRow): RowStatus => statuses.get(row.key)?.status ?? "draft";
  const resolved = resolveRunSheet({
    sheetKey: state.sheetKey, batchSeq: state.batchSeq, lengthUnit: state.header.lengthUnit, source: state.header.source,
    numbering: state.header.numbering, columnUnits: state.columnUnits,
    rows: shown.map((row) => rowInput(state, row, statusOf(row))),
  }, { model: sheetBasisModel(model, state, statuses), idState: state.idState, reservedIds: input.reservedIds, conversions: input.conversions });
  const inModel = modelIds(model);
  const rows: ViewRow[] = shown.map((sheetRow, index) => {
    const status = statusOf(sheetRow);
    const row = resolved.rows[index];
    let refusals: RunSheetFinding[] = [];
    if (status === "draft") {
      refusals = row.refusals.map((finding) => {
        const id = /^Id (.+) is already used\.$/.exec(finding.message)?.[1];
        return id && inModel.has(id) ? { ...finding, message: LIFECYCLE_MESSAGES.existsNow(id) } : finding;
      });
      const predecessorIndex = index - 1;
      const continues = row.kind === "run" || row.kind === "bend" || row.kind === "support";
      const predecessor = continues && predecessorIndex >= 0 ? shown[predecessorIndex] : null;
      if (predecessor) {
        const before = statusOf(predecessor);
        if (before === "queued") refusals.push({ rule: "incomplete", message: LIFECYCLE_MESSAGES.waits(predecessor.pending?.batchId ?? "") });
        if (before === "detached") refusals.push({ rule: "incomplete", message: LIFECYCLE_MESSAGES.detachedPredecessor(predecessorIndex + 1) });
      }
      for (const diagnostic of sheetRow.engineRefusal?.diagnostics ?? []) refusals.push({ rule: "incomplete", message: LIFECYCLE_MESSAGES.engine(diagnostic.code, diagnostic.message) });
      refusals.push(...(input.engineRefusals?.get(sheetRow.key) ?? []));
    }
    return { sheetRow, status, resolved: row, refusals, editedNotApplied: editedNotApplied(state, sheetRow), values: shownValues(state, sheetRow) };
  });
  const draftRows = rows.filter((row) => row.status === "draft" && hasValues(row));
  const refused = draftRows.filter((row) => row.refusals.length > 0);
  const queueBlocked = resolved.blockers.length ? resolved.blockers.map((finding) => finding.message).join(" ")
    : resolved.missingConversions.length ? "Converting the sheet's units; wait a moment."
    : draftRows.length === 0 ? "No draft rows to queue."
    : refused.length ? `Refused rows: ${refused.map((row) => row.resolved.number).join(", ")}. Correct them to queue.`
    : null;
  return { resolved, rows, draftRows, queueBlocked, requeue: draftRows.some((row) => row.sheetRow.requeueNeeded) };
}

const hasValues = (row: ViewRow) => Object.values(shownLike(row)).some((value) => value !== undefined && value !== "");
const shownLike = (row: ViewRow) => ({ ...row.sheetRow.restored, ...row.sheetRow.edits }) as Record<string, unknown>;

/** The ids a row's operations leave in the model, typed (§3.4 ownership). */
export function ownedRefsOf(row: ResolvedRunSheetRow, ownedIds: readonly string[]): OwnedRef[] {
  const types = new Map<string, OwnedRef["type"]>();
  if (row.ids.node) types.set(row.ids.node, "node");
  if (row.ids.pipe) types.set(row.ids.pipe, "pipe");
  if (row.ids.support) types.set(row.ids.support, "support");
  if (row.ids.bend) {
    types.set(row.ids.bend.start, "node");
    types.set(row.ids.bend.end, "node");
    types.set(row.ids.bend.arc, "pipe");
    types.set(row.ids.bend.component, "component");
  }
  return ownedIds.map((id) => ({ type: types.get(id) ?? "node", id }));
}

/** Build the one batch from the draft rows (§3.5): the builder over the real model. */
export async function buildDraftBatch(view: SheetView, model: PreviewModel): Promise<RunSheetBuild> {
  if (view.queueBlocked) throw new Error(view.queueBlocked);
  return buildRunSheetBatch({ ...view.resolved, basis: model, rows: view.draftRows.map((row) => row.resolved), queueable: true });
}

/** The values a row is queued with, as the checkpoint's `valuesAfter`. */
export function queuedValues(state: RunSheetState, row: SheetRow): RowValues {
  return shownValues(state, row);
}

/** Queue (§3.6 Transitions): each acting row gets a pending marker, not in the
 * ledger; its values stay in the buffer. The batch's rows, operations, owned ids
 * and queued values are kept for its checkpoint. The batch sequence moves on. */
export function markSubmitted(state: RunSheetState, view: SheetView, build: RunSheetBuild): RunSheetState {
  const batchId = build.batch.batch_id;
  const acting = new Map(view.draftRows.map((row) => [row.sheetRow.key, row]));
  const submitted: SubmittedBatch = {
    batchId, batch: build.batch, rows: build.rows,
    ownedRefs: Object.fromEntries(build.rows.map((built) => [built.key, ownedRefsOf(acting.get(built.key)!.resolved, built.ownedIds)])),
    values: Object.fromEntries(state.rows.filter((row) => acting.has(row.key)).map((row) => [row.key, queuedValues(state, row)])),
  };
  return {
    ...state, batchSeq: state.batchSeq + 1, submitted: { ...state.submitted, [batchId]: submitted },
    rows: state.rows.map((row) => acting.has(row.key) ? { ...row, pending: { batchId, registrationKey: null, action: "create" }, requeueNeeded: false, engineRefusal: null } : row),
  };
}
/** A batch the queue did not register: its rows are draft again, unchanged. */
export function releaseUnregistered(state: RunSheetState, batchId: string): RunSheetState {
  return { ...state, rows: state.rows.map((row) => row.pending?.batchId === batchId && row.pending.registrationKey === null ? { ...row, pending: null } : row) };
}

// ---- Cells.

export type CellView = Readonly<{
  text: string;
  unit: string;
  origin: string;
  muted: boolean;
  italic: boolean;
  refusal: string | null;
}>;
const DIMENSION_LABEL = (column: SheetColumn) => (column.quantity ? QUANTITY_COLUMNS[column.quantity].label : column.label);
export function originText(origin: CellOrigin | undefined, absent?: boolean): string {
  if (absent) return "absent";
  switch (origin?.kind) {
    case "entered": return "entered";
    case "inherited": return `inherited from row ${origin.row}`;
    case "stored": return `inherited from pipe ${origin.pipe}`;
    case "derived": return `derived: ${origin.rule}`;
    default: return "blank";
  }
}
const FINDING_COLUMNS: Readonly<Record<string, readonly SheetColumnKey[]>> = {
  "dX, dY, dZ": ["dx", "dy", "dz"], Kind: ["kind"], Support: ["support"], Restraints: ["restraints"], Node: ["node"], Section: ["section"],
  Material: ["material"], Orientation: ["orientation"], Ids: ["nodeId", "pipeId"], k: ["k", "kSource"],
  ...Object.fromEntries(Object.entries(QUANTITY_COLUMNS).map(([key, value]) => [value.label, [key as SheetColumnKey]])),
};
export function cellRefusal(row: ViewRow, column: SheetColumn): string | null {
  const finding = row.refusals.find((item) => item.column && (FINDING_COLUMNS[item.column] ?? []).includes(column.key));
  return finding ? [finding.message, finding.detail].filter(Boolean).join(" ") : null;
}
const fmt = (value: number) => String(value);
export function cellView(view: SheetView, row: ViewRow, column: SheetColumn, columnUnit: (column: QuantityColumn) => string): CellView {
  const resolvedRow = row.resolved;
  const entered = (key: keyof RowValues): string => {
    const typed = row.values[key];
    return typeof typed === "string" ? typed : "";
  };
  const base = { unit: "", muted: false, italic: false, refusal: cellRefusal(row, column) };
  switch (column.key) {
    case "dx": case "dy": case "dz": {
      const text = entered(column.key);
      if (text.trim()) return { ...base, text, unit: view.resolved.sheetLengthUnit, origin: "entered" };
      const none = resolvedRow.kind === "from" || resolvedRow.kind === "support" || Boolean(entered("node").trim());
      return { ...base, text: none ? "" : "0", unit: none ? "" : view.resolved.sheetLengthUnit, muted: true, origin: none ? "blank" : "blank: zero on this axis" };
    }
    case "kind": {
      const text = entered("kind").trim().toUpperCase();
      return { ...base, text, origin: text ? "entered" : "blank: a run" };
    }
    case "support": {
      const text = entered("support").trim().toUpperCase();
      return { ...base, text, origin: text ? "entered" : "blank: no support" };
    }
    case "restraints": case "node": case "k": case "kSource": {
      const text = entered(column.key);
      return { ...base, text, origin: text.trim() ? "entered" : "blank" };
    }
    case "label": {
      const label = resolvedRow.label;
      return { ...base, text: label?.value ?? "", italic: label?.origin.kind === "derived", muted: Boolean(label) && label?.origin.kind !== "entered", origin: originText(label?.origin) };
    }
    case "nodeId": case "pipeId": {
      const id = column.key === "nodeId" ? (resolvedRow.end && !resolvedRow.end.existing ? resolvedRow.ids.node : undefined) : resolvedRow.ids.pipe;
      const overridden = Boolean(column.key === "nodeId" ? row.values.ids?.node : row.values.ids?.pipe);
      return { ...base, text: id ?? "", italic: !overridden && Boolean(id), origin: overridden ? "entered" : id ? "derived: the sheet's numbering" : "blank" };
    }
    case "position": {
      const end = resolvedRow.end?.position;
      return { ...base, text: end ? `${fmt(end.x)}, ${fmt(end.y)}, ${fmt(end.z)}` : "", unit: end ? view.resolved.storedLengthUnit : "", italic: true, origin: "derived: the resulting end point" };
    }
    case "length":
      return { ...base, text: resolvedRow.length === null ? "" : fmt(resolvedRow.length), unit: resolvedRow.length === null ? "" : view.resolved.storedLengthUnit, italic: true, origin: "derived: the pipe's length" };
    case "section": case "material": {
      const cell = resolvedRow.cells[column.key];
      return { ...base, text: cell.value ?? "", muted: cell.value !== undefined && cell.origin.kind !== "entered", italic: cell.origin.kind === "derived", origin: originText(cell.origin) };
    }
    case "orientation": {
      const cell = resolvedRow.cells.orientation;
      if (cell.error) return { ...base, text: entered("orientation"), origin: "entered" };
      return { ...base, text: cell.value ? `${fmt(cell.value.x)} ${fmt(cell.value.y)} ${fmt(cell.value.z)}` : "", muted: cell.value !== undefined && cell.origin.kind !== "entered", origin: originText(cell.origin) };
    }
    default: {
      const quantity = column.quantity!;
      const cell = resolvedRow.cells[quantity];
      if (cell.error) return { ...base, text: entered(quantity), unit: columnUnit(quantity), origin: "entered" };
      if (cell.absent) return { ...base, text: "—", muted: cell.origin.kind !== "entered", origin: originText(cell.origin, true) };
      if (!cell.value) return { ...base, text: "", unit: columnUnit(quantity), origin: originText(cell.origin) };
      // A value inherited from a stored pipe keeps its stored unit and shows it (§1.2).
      return {
        ...base, text: fmt(cell.value.value), unit: cell.value.unit,
        muted: cell.origin.kind !== "entered", italic: cell.origin.kind === "derived", origin: originText(cell.origin),
      };
    }
  }
}

/** The accessible name of a cell (§5.3): "Row ⟨n⟩ ⟨column⟩: ⟨value⟩ ⟨unit⟩". */
export function cellName(row: ViewRow, column: SheetColumn, cell: CellView): string {
  return `Row ${row.resolved.number} ${DIMENSION_LABEL(column)}: ${[cell.text, cell.unit].filter(Boolean).join(" ")}`.trimEnd();
}
/** The row header (§5.3): "Row 4, bend run, draft, refused: a bend needs the next run". */
export function rowHeaderText(row: ViewRow): string {
  const kind = row.resolved.kind ? KIND_NAMES[row.resolved.kind] : "invalid kind";
  const parts = [`Row ${row.resolved.number}`, kind, row.status];
  if (row.sheetRow.requeueNeeded && row.status === "draft") parts.push("requeue needed");
  if (row.editedNotApplied.length) parts.push("edited, not applied");
  const first = row.refusals[0];
  return first ? `${parts.join(", ")}, refused: ${sentenceFragment(first.message)}` : parts.join(", ");
}
/** A message as a clause: an ordinary first word is lowercased; a code or id keeps its case. */
export const sentenceFragment = (message: string) => {
  const trimmed = message.replace(/\.$/, "");
  return /^[A-Z][a-z]/.test(trimmed) ? trimmed.charAt(0).toLowerCase() + trimmed.slice(1) : trimmed;
};

// ---- Edits (pure).

/** Write one cell of a draft row; blank text returns it to inheritance (§2.4). */
export function setCell(state: RunSheetState, rowKey: string, column: SheetColumnKey, text: string): RunSheetState {
  return {
    ...state,
    rows: state.rows.map((row) => {
      if (row.key !== rowKey) return row;
      if (column === "nodeId" || column === "pipeId") {
        const member = column === "nodeId" ? "node" : "pipe";
        const ids = { ...(row.restored.ids ?? {}), ...(row.edits.ids ?? {}) };
        if (text.trim()) ids[member] = text.trim(); else delete ids[member];
        return { ...row, edits: { ...row.edits, ids }, engineRefusal: null };
      }
      const edits = { ...row.edits } as Record<string, unknown>;
      if (text === "" && (row.restored as Record<string, unknown>)[column] === undefined) delete edits[column];
      else edits[column] = text;
      return { ...row, edits: edits as RowValues, engineRefusal: null };
    }),
  };
}

/** §3.2 A row's number is assigned once, when it first gets a value. */
export function assignIds(state: RunSheetState, rowKey: string, model: PreviewModel, reserved: Iterable<string>): RunSheetState {
  if (state.idState.assigned[rowKey] !== undefined) return state;
  const held = new Set<string>([...modelIds(model), ...reserved]);
  for (const [key, n] of Object.entries(state.idState.assigned)) {
    if (key === rowKey) continue;
    const ids = idsForNumber(state.header.numbering, n);
    for (const id of [ids.node, ids.pipe, ids.support, ids.bend.start, ids.bend.end, ids.bend.arc, ids.bend.component]) held.add(id);
  }
  return { ...state, idState: assignRowNumber(state.idState, state.header.numbering, rowKey, held) };
}

export function insertRow(state: RunSheetState, index: number): { state: RunSheetState; key: string } {
  const key = `r${state.nextRowKey}`;
  const rows = [...state.rows];
  rows.splice(Math.max(0, Math.min(index, rows.length)), 0, blankRow(key));
  return { state: { ...state, rows, nextRowKey: state.nextRowKey + 1 }, key };
}
/** Delete a draft row: its ids leave the sheet, and its numbers are never reused (§3.4). */
export function deleteDraftRow(state: RunSheetState, rowKey: string): RunSheetState {
  return { ...state, rows: state.rows.filter((row) => row.key !== rowKey) };
}
/** Discard edit (§3.6): the row returns to its current entry's values. */
export function discardEdit(state: RunSheetState, rowKey: string): RunSheetState {
  return { ...state, rows: state.rows.map((row) => row.key === rowKey ? { ...row, edits: {}, restored: {} } : row) };
}

/** Values the inheriting cells take (§2.1), for Fill down's "effective value". */
export function effectiveText(row: ResolvedRunSheetRow, column: SheetColumnKey, typed: string): string {
  if (!(INHERITING_CELLS as readonly string[]).includes(column)) return typed;
  if (column === "section" || column === "material") return row.cells[column].value ?? "";
  if (column === "orientation") {
    const value = row.cells.orientation.value;
    return value ? `${value.x} ${value.y} ${value.z}` : "";
  }
  const cell = row.cells[column as QuantityColumn];
  return cell.absent ? "-" : cell.value ? String(cell.value.value) : "";
}

/** Draft rows that have entered increments lock the sheet's length unit (§4.2 Mixed units). */
export function lengthUnitLock(view: SheetView): string | null {
  const locked = view.draftRows.some((row) => {
    const values = { ...row.sheetRow.restored, ...row.sheetRow.edits };
    return Boolean((values.dx ?? "").trim() || (values.dy ?? "").trim() || (values.dz ?? "").trim());
  });
  return locked ? `Draft rows hold increments in ${view.resolved.sheetLengthUnit || "the sheet unit"}. Queue or clear them to change the unit.` : null;
}

export { MESSAGES };
