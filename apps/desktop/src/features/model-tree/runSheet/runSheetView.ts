// Run sheet S2 (DEL-07-02 Design `run-sheet-entry.md` §1–§2, §3.4, §4): the
// sheet's view, pure. It resolves every shown row with S1's resolver and lists
// what each cell shows and why. Applied rows are views of the model: their
// values are the entered values their checkpoint put in effect, resolved
// against the model without the entities the sheet's applied rows own, so the
// chain resolves exactly as it did when it was drafted and a draft row after
// an applied one continues from its stored end. Only draft rows are queued.
import type { PreviewComponent, PreviewModel } from "../../../types";
import { buildRunSheetBatch, type RunSheetBuild } from "./runSheetBatch";
import {
  BEND_ID_CELLS, INHERITING_CELLS, MESSAGES, QUANTITY_COLUMNS, assignRowNumber, idsForNumber, modelIds, resolveRunSheet,
  conversionKey, isPipeRowKind,
  type CellOrigin, type QuantityColumn, type ResolvedRunSheet, type ResolvedRunSheetRow, type RunSheetConversions, type RunSheetFinding, type RunSheetRowInput, type Vec3,
} from "./runSheetModel";
import type { DisplayQuantityRequest } from "../../../services/displayQuantityService";
import {
  blankRow, currentEntry, editedNotApplied, shownValues,
  type OwnedRef, type RecordLookup, type RowReconciliation, type RowStatus, type RowValues, type RunSheetState, type SheetRow, type SubmittedBatch,
} from "./runSheetLedger";

import { COMPONENT_FIELDS, componentColumn, componentField, componentCellsFromRecord, type ComponentColumnKey } from "./runSheetComponents";

export type SheetColumnKey = ComponentColumnKey
  | "dx" | "dy" | "dz" | "kind" | "support" | "restraints" | "node" | "section" | "od" | "wall" | "material" | "orientation"
  | "insulationThickness" | "insulationDensity" | "contentsDensity" | "materialDensity" | "millTolerance" | "corrosionAllowance"
  | "bendRadius" | "k" | "kSource" | "label" | "nodeId" | "pipeId"
  | "supportId" | "supportLabel" | "bendStartId" | "bendEndId" | "bendArcId" | "bendComponentId" | "bendSource"
  | "position" | "length";
/** The collapsible column groups (§1.2), each shown and hidden by its own header toggle. */
export type SheetColumnGroup = "sectionExtras" | "idsAndSources" | "componentDetails";
export const COLUMN_GROUP_LABELS: Readonly<Record<SheetColumnGroup, string>> = { sectionExtras: "Section extras", idsAndSources: "Ids and sources", componentDetails: "Component details" };
export type SheetColumn = Readonly<{
  key: SheetColumnKey;
  label: string;
  editable: boolean;
  code?: "kind" | "support";
  quantity?: QuantityColumn;
  group?: SheetColumnGroup;
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
  { key: "materialDensity", label: "Material density", editable: true, quantity: "materialDensity", group: "sectionExtras" },
  { key: "millTolerance", label: "Mill tolerance", editable: true, quantity: "millTolerance", group: "sectionExtras" },
  { key: "corrosionAllowance", label: "Corrosion allowance", editable: true, quantity: "corrosionAllowance", group: "sectionExtras" },
  { key: "bendRadius", label: "Bend R", editable: true, quantity: "bendRadius" },
  { key: "k", label: "k", editable: true },
  { key: "kSource", label: "k source", editable: true },
  { key: "label", label: "Label", editable: true },
  { key: "nodeId", label: "Node id", editable: true },
  { key: "pipeId", label: "Pipe id", editable: true },
  { key: "supportId", label: "Support id", editable: true, group: "idsAndSources" },
  { key: "supportLabel", label: "Support label", editable: true, group: "idsAndSources" },
  { key: "bendStartId", label: "Bend start id", editable: true, group: "idsAndSources" },
  { key: "bendEndId", label: "Bend end id", editable: true, group: "idsAndSources" },
  { key: "bendArcId", label: "Bend arc id", editable: true, group: "idsAndSources" },
  { key: "bendComponentId", label: "Bend component id", editable: true, group: "idsAndSources" },
  { key: "bendSource", label: "Bend geometry source", editable: true, group: "idsAndSources" },
  ...COMPONENT_FIELDS.map((field) => ({ key: componentColumn(field.key), label: field.label, editable: true, group: "componentDetails" as const })),
  { key: "position", label: "Position", editable: false },
  { key: "length", label: "Length", editable: false },
];
/** §5.1 The Enter fast path: dX → dY → dZ → Kind → Support → the next row's dX. */
export const FAST_PATH: readonly SheetColumnKey[] = ["dx", "dy", "dz", "kind", "support"];
export const KIND_NAMES: Readonly<Record<string, string>> = { run: "run", bend: "bend run", support: "support only", from: "start from node", point: "start at point", valve: "valve run", flange: "flange run" };
/** The id cells and the row input's `ids` member each writes (§3.2). */
export const ID_COLUMNS: Readonly<Partial<Record<SheetColumnKey, keyof NonNullable<RowValues["ids"]>>>> = {
  nodeId: "node", pipeId: "pipe", supportId: "support",
  bendStartId: "bendStart", bendEndId: "bendEnd", bendArcId: "bendArc", bendComponentId: "bendComponent",
};
const BEND_COLUMNS: ReadonlyArray<readonly [SheetColumnKey, typeof BEND_ID_CELLS[number]]> = [
  ["bendStartId", BEND_ID_CELLS[0]], ["bendEndId", BEND_ID_CELLS[1]], ["bendArcId", BEND_ID_CELLS[2]], ["bendComponentId", BEND_ID_CELLS[3]],
];
export const CODE_LISTS = {
  kind: "R run, B bend run, S support only, F start from node, P start at point, V valve run, L flange run; blank is a run",
  support: "A anchor, G guide, L line stop, V vertical support; blank is no support",
} as const;

/** The row input as entered, for any row (draft rows read their buffer; others their in-effect
 * values). An applied or detached row that created its end node carries that node's stored
 * position, so the chain after it continues from stored geometry (§3.4, §6.2 x_{k−1}). */
export function rowInput(state: RunSheetState, row: SheetRow, status: RowStatus, lookup?: RecordLookup): RunSheetRowInput {
  if (status === "draft" || status === "queued") return { key: row.key, ...shownValues(state, row) };
  const entry = currentEntry(state.ledger, row.key);
  const values = entry?.valuesAfter ?? {};
  const input: RunSheetRowInput = { key: row.key, ...values };
  // An applied run or bend row keeps the pipe its batch created (its first owned pipe).
  const kind = (values.kind ?? "").trim().toUpperCase();
  const ownPipe = (kind === "" || kind === "R" || kind === "B" || kind === "V" || kind === "L") ? entry?.ownedIds.find((ref) => ref.type === "pipe") : undefined;
  if (ownPipe) input.storedPipeId = ownPipe.id;
  // An applied bend row reads its geometry source from its stored component.
  const component = kind === "B" ? entry?.ownedIds.find((ref) => ref.type === "component") : undefined;
  if (component) {
    const source = (lookup?.(component) as { geometry?: { bend_geometry_source_reference?: unknown } } | undefined)?.geometry?.bend_geometry_source_reference;
    input.storedBendSource = typeof source === "string" ? source : "";
  }
  if (kind === "V" || kind === "L") {
    const own = entry?.ownedIds.find((ref) => ref.type === "component");
    const record = own ? lookup?.(own) as PreviewComponent | undefined : undefined;
    if (record) {
      input.component = componentCellsFromRecord(record);
      const expected = entry?.expected.find((item) => item.ref.type === "component" && item.ref.id === own!.id)?.record as PreviewComponent | undefined;
      if (record.kind !== (kind === "V" ? "valve" : "flange") || record.node !== expected?.node || record.geometry?.rigid_pipe_ref !== ownPipe?.id) {
        input.storedComponentMismatch = `Component ${record.id} is now ${record.kind} at ${record.node}, mapped to ${record.geometry?.rigid_pipe_ref ?? "no pipe"}; this row retains its original mapping.`;
      }
    }
    else input.storedComponentMissing = own?.id ?? values.component?.id ?? "(unknown component)";
  }
  const stored = storedEndOf(entry, values, lookup);
  if (stored) {
    if (stored.end) input.storedEnd = stored.end;
    if (stored.endId) input.storedEndId = stored.endId;
    if (stored.pipeEnd) input.storedPipeEnd = stored.pipeEnd;
    if (stored.endMissing) input.storedEndMissing = stored.endMissing;
    if (stored.pipeEndMissing) input.storedPipeEndMissing = stored.pipeEndMissing;
  }
  return input;
}
const isBendRow = (values: RowValues) => (values.kind ?? "").trim().toUpperCase() === "B";
const finitePosition = (record: unknown): Vec3 | null => {
  const position = (record as { position?: Vec3 } | undefined)?.position;
  return position && [position.x, position.y, position.z].every((value) => typeof value === "number" && Number.isFinite(value)) ? { x: position.x, y: position.y, z: position.z } : null;
};
type StoredEnd = { end?: Vec3; endId?: string; pipeEnd?: Vec3; endMissing?: string; pipeEndMissing?: string };
/** The stored geometry an applied row continues from (§3.4, §6.2), when the model still holds it:
 * - a run or P row that created its end node: that node;
 * - a bend row: the bend removed its corner, so its own pipe ends at the stored incoming tangent
 *   point T1 and the chain continues from the stored outgoing tangent point T2, by its id. Each
 *   tangent is read on its own, by the ids the bend's insertion named (`new_nodes`: T1, then T2);
 *   a tangent that no longer exists is reported missing. The corner is never resolved. */
function storedEndOf(entry: ReturnType<typeof currentEntry>, values: RowValues, lookup: RecordLookup | undefined): StoredEnd | null {
  if (!entry || !lookup) return null;
  const kind = (values.kind ?? "").trim().toUpperCase();
  if (kind === "B") {
    const [incoming, outgoing] = entry.ownedIds.filter((ref) => ref.type === "node");
    if (!incoming || !outgoing) return null;
    const pipeEnd = finitePosition(lookup(incoming));
    const end = finitePosition(lookup(outgoing));
    return {
      ...(end ? { end, endId: outgoing.id } : { endMissing: outgoing.id }),
      ...(pipeEnd ? { pipeEnd } : { pipeEndMissing: incoming.id }),
    };
  }
  const createsEnd = ((kind === "" || kind === "R" || kind === "V" || kind === "L") && !(values.node ?? "").trim()) || kind === "P";
  if (!createsEnd) return null;
  const node = entry.ownedIds.find((ref) => ref.type === "node");
  const end = node ? finitePosition(lookup(node)) : null;
  return end ? { end } : null;
}

/** The model without the entities the sheet's applied rows own (the chain resolves as drafted). */
export function sheetBasisModel(model: PreviewModel, state: RunSheetState, statuses: ReadonlyMap<string, RowReconciliation>): PreviewModel {
  const owned = new Map<string, Set<string>>();
  for (const row of state.rows) {
    const status = statuses.get(row.key)?.status;
    if (status !== "applied" && status !== "detached") continue;
    const entry = currentEntry(state.ledger, row.key);
    // A bend row's tangent nodes, arc and component stay: they are stored records the sheet
    // resolves (an F branch may name a tangent node), and no row recreates them.
    const bend = Boolean(entry && isBendRow(entry.valuesAfter));
    const ownPipe = bend ? entry!.ownedIds.find((ref) => ref.type === "pipe")?.id : undefined;
    for (const ref of entry?.ownedIds ?? []) {
      if (bend && !(ref.type === "support" || (ref.type === "pipe" && ref.id === ownPipe))) continue;
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
  /** An applied or detached row's increments, `stored(end) − stored(predecessor)`, for display only
   * (§3.4): in the sheet unit once converted, else in the stored unit. Absolute for a P row. */
  storedIncrements: Readonly<{ value: Vec3; unit: string }> | null;
  /** An applied bend row's own pipe end, its stored incoming tangent point (Position readout). */
  storedPipeEnd: Vec3 | null;
  /** Why an applied or detached row's stored increments and position cannot be read: a bend's
   * tangent point, or the stored point its pipe starts from, no longer exists. Shown, never
   * replaced by the historical corner. */
  storedMissing: string | null;
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
  /** Display-only conversions of applied rows' stored increments still to fetch. */
  displayConversions: readonly DisplayQuantityRequest[];
}>;

export const LIFECYCLE_MESSAGES = {
  waits: (batch: string) => `Waits for batch ${batch} to be applied.`,
  detachedPredecessor: (row: number) => `Row ${row} changed outside the sheet; start this chain again with F.`,
  existsNow: (id: string, cell = "Id") => `${cell} ${id} now exists in the model.`,
  engine: (code: string, message: string) => `${code}: ${message}`,
} as const;

/** "⟨cell⟩ ⟨id⟩ is already used." for each id cell (`MESSAGES.idUsed`). */
const USED_ID = new RegExp(`^(${["Id", "Support id", ...BEND_ID_CELLS.map((cell) => cell.label)].join("|")}) (.+) is already used\\.$`);

/** The view of the whole sheet (§2, §4.2), resolved synchronously with the conversions known so far. */
export function sheetView(input: {
  state: RunSheetState;
  statuses: ReadonlyMap<string, RowReconciliation>;
  model: PreviewModel;
  conversions: RunSheetConversions;
  reservedIds: Iterable<string>;
  engineRefusals?: ReadonlyMap<string, readonly RunSheetFinding[]>;
  /** The current records by entity (the session's index); applied rows read stored positions here. */
  lookup?: RecordLookup;
}): SheetView {
  const { state, statuses, model } = input;
  const shown = state.rows.filter((row) => statuses.get(row.key)?.status !== "deleted");
  const statusOf = (row: SheetRow): RowStatus => statuses.get(row.key)?.status ?? "draft";
  const lookup = input.lookup ?? modelLookup(model);
  const inputs = shown.map((row) => rowInput(state, row, statusOf(row), lookup));
  const resolved = resolveRunSheet({
    sheetKey: state.sheetKey, batchSeq: state.batchSeq, lengthUnit: state.header.lengthUnit, source: state.header.source,
    numbering: state.header.numbering, columnUnits: state.columnUnits,
    rows: inputs,
  }, { model: sheetBasisModel(model, state, statuses), idState: state.idState, reservedIds: input.reservedIds, conversions: input.conversions });
  const inModel = modelIds(model);
  const displayConversions: DisplayQuantityRequest[] = [];
  const rows: ViewRow[] = shown.map((sheetRow, index) => {
    const status = statusOf(sheetRow);
    const row = resolved.rows[index];
    let refusals: RunSheetFinding[] = [];
    if (status === "draft") {
      refusals = row.refusals.map((finding) => {
        const used = USED_ID.exec(finding.message);
        return used && inModel.has(used[2]) ? { ...finding, message: LIFECYCLE_MESSAGES.existsNow(used[2], used[1]) } : finding;
      });
      const predecessorIndex = index - 1;
      const continues = isPipeRowKind(row.kind) || row.kind === "support";
      const predecessor = continues && predecessorIndex >= 0 ? shown[predecessorIndex] : null;
      if (predecessor) {
        const before = statusOf(predecessor);
        if (before === "queued") refusals.push({ rule: "incomplete", message: LIFECYCLE_MESSAGES.waits(predecessor.pending?.batchId ?? "") });
        if (before === "detached") refusals.push({ rule: "incomplete", message: LIFECYCLE_MESSAGES.detachedPredecessor(predecessorIndex + 1) });
      }
      for (const diagnostic of sheetRow.engineRefusal?.diagnostics ?? []) refusals.push({ rule: "incomplete", message: LIFECYCLE_MESSAGES.engine(diagnostic.code, diagnostic.message) });
      refusals.push(...(input.engineRefusals?.get(sheetRow.key) ?? []));
    }
    let storedIncrements: ViewRow["storedIncrements"] = null;
    const storedPipeEnd = inputs[index].storedPipeEnd ?? null;
    let storedMissing: string | null = null;
    if (status === "applied" || status === "detached") {
      const before = index > 0 ? resolved.rows[index - 1] : null;
      if (inputs[index].storedPipeEndMissing) storedMissing = MESSAGES.bendIncomingMissing(inputs[index].storedPipeEndMissing!);
      else if ((isPipeRowKind(row.kind)) && before?.endMissing && !row.startPosition) storedMissing = MESSAGES.bendOutgoingMissing(before.endMissing);
    }
    if ((status === "applied" || status === "detached") && row.end && !row.end.existing && inputs[index].storedEnd) {
      const from = row.kind === "point" ? { x: 0, y: 0, z: 0 } : row.startPosition;
      const to = storedPipeEnd ?? row.end.position;
      if (from && (isPipeRowKind(row.kind) || row.kind === "point")) {
        const stored = { x: to.x - from.x, y: to.y - from.y, z: to.z - from.z };
        storedIncrements = displayIncrements(stored, resolved.storedLengthUnit, resolved.sheetLengthUnit, input.conversions, displayConversions);
      }
    }
    return { sheetRow, status, resolved: row, refusals, editedNotApplied: editedNotApplied(state, sheetRow), values: shownValues(state, sheetRow), storedIncrements, storedPipeEnd, storedMissing };
  });
  const draftRows = rows.filter((row) => row.status === "draft" && hasValues(row));
  const refused = draftRows.filter((row) => row.refusals.length > 0);
  const queueBlocked = resolved.blockers.length ? resolved.blockers.map((finding) => finding.message).join(" ")
    : resolved.missingConversions.length ? "Converting the sheet's units; wait a moment."
    : draftRows.length === 0 ? "No draft rows to queue."
    : refused.length ? `Refused rows: ${refused.map((row) => row.resolved.number).join(", ")}. Correct them to queue.`
    : null;
  return { resolved, rows, draftRows, queueBlocked, requeue: draftRows.some((row) => row.sheetRow.requeueNeeded), displayConversions };
}

/** A lookup over the model's own collections, where no index is supplied. */
function modelLookup(model: PreviewModel): RecordLookup {
  const nodes = new Map(model.nodes.map((node) => [node.id, node]));
  const components = new Map(model.components.map((component) => [component.id, component]));
  return (ref) => ref.type === "node" ? nodes.get(ref.id) : ref.type === "component" ? components.get(ref.id) : undefined;
}

/** §3.2 A pipe id numbered by its start node is fixed the first time it is shown: each draft or
 * queued row that shows one, with a start node and no entered or stored id, records it in the
 * id state. Nothing is recorded while a conversion is pending, since positions decide starts. */
export function recordShownPipeIds(state: RunSheetState, view: SheetView): RunSheetState {
  if (view.resolved.missingConversions.length) return state;
  let pipes: Record<string, { id: string; rule: string }> | null = null;
  for (const row of view.rows) {
    const resolved = row.resolved;
    if ((row.status !== "draft" && row.status !== "queued") || !resolved.start || !resolved.ids.pipe || !resolved.pipeIdRule) continue;
    if (state.idState.pipes?.[row.sheetRow.key] || row.values.ids?.pipe?.trim()) continue;
    pipes ??= { ...(state.idState.pipes ?? {}) };
    pipes[row.sheetRow.key] = { id: resolved.ids.pipe, rule: resolved.pipeIdRule };
  }
  return pipes ? { ...state, idState: { ...state.idState, pipes } } : state;
}

/** A numbering change retries every row whose number was refused (§3.2), in sheet order. */
export function retryNumbering(state: RunSheetState, model: PreviewModel, reserved: Iterable<string>): RunSheetState {
  const held = [...reserved];
  return state.rows.reduce((next, row) => {
    if (next.idState.refused?.[row.key] === undefined) return next;
    const { [row.key]: _retried, ...refused } = next.idState.refused;
    return assignIds({ ...next, idState: { ...next.idState, refused } }, row.key, model, held);
  }, state);
}
/** Stored increments in the sheet unit through the engine's conversions (display only), else in the stored unit. */
function displayIncrements(stored: Vec3, storedUnit: string, sheetUnit: string, conversions: RunSheetConversions, requests: DisplayQuantityRequest[]): { value: Vec3; unit: string } {
  if (!sheetUnit || sheetUnit === storedUnit) return { value: stored, unit: storedUnit };
  const out: number[] = [];
  for (const value of [stored.x, stored.y, stored.z]) {
    const key = conversionKey(value, storedUnit, sheetUnit, "length");
    const converted = conversions.get(key);
    if (typeof converted === "number") out.push(converted);
    else if (!conversions.has(key)) requests.push({ id: key, value: value === 0 ? 0 : value, from_unit: storedUnit, to_unit: sheetUnit, dimension_id: "length" });
  }
  return out.length === 3 ? { value: { x: out[0], y: out[1], z: out[2] }, unit: sheetUnit } : { value: stored, unit: storedUnit };
}

const hasValues = (row: ViewRow) => Object.values(shownLike(row)).some((value) => value !== undefined && value !== "");
const shownLike = (row: ViewRow) => ({ ...row.sheetRow.restored, ...row.sheetRow.edits }) as Record<string, unknown>;

/** The ids a row's operations leave in the model, typed (§3.4 ownership). */
export function ownedRefsOf(row: ResolvedRunSheetRow, ownedIds: readonly string[]): OwnedRef[] {
  const types = new Map<string, OwnedRef["type"]>();
  if (row.ids.node) types.set(row.ids.node, "node");
  if (row.ids.pipe) types.set(row.ids.pipe, "pipe");
  if (row.ids.support) types.set(row.ids.support, "support");
  if (row.ids.component) types.set(row.ids.component, "component");
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
export function markSubmitted(shown: RunSheetState, view: SheetView, build: RunSheetBuild): RunSheetState {
  const state = recordShownPipeIds(shown, view);
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
  ...Object.fromEntries(COMPONENT_FIELDS.map((field) => [field.label, [componentColumn(field.key)]])),
  "Support id": ["supportId"], "Support label": ["supportLabel"], "Bend geometry source": ["bendSource"],
  ...Object.fromEntries(BEND_COLUMNS.map(([key, cell]) => [cell.label, [key]])),
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
  const field = componentField(column.key);
  if (field) {
    if (resolvedRow.componentMissing) return { ...base, text: field === "id" ? resolvedRow.componentMissing : "", origin: `missing: component ${resolvedRow.componentMissing}`, refusal: "This component no longer exists." };
    const draft = resolvedRow.component;
    const raw = row.values.component?.[field] ?? "";
    const value = draft?.[field] ?? raw;
    const stored = row.status === "applied" || row.status === "detached";
    return { ...base, ...(field === "id" && resolvedRow.componentMismatch ? { refusal: resolvedRow.componentMismatch } : {}), text: value, italic: Boolean(draft && !raw && value),
      origin: stored ? "stored component value" : raw ? "entered" : value ? "derived: the project unit" : "blank" };
  }
  switch (column.key) {
    case "dx": case "dy": case "dz": {
      if (row.storedMissing) return { ...base, text: "", origin: `missing: ${row.storedMissing}` };
      if (row.storedIncrements) {
        const axis = column.key === "dx" ? "x" : column.key === "dy" ? "y" : "z";
        return { ...base, text: fmt(row.storedIncrements.value[axis]), unit: row.storedIncrements.unit, italic: true,
          origin: row.resolved.kind === "point" ? "derived: the stored position" : "derived: stored end − stored predecessor" };
      }
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
      const rule = column.key === "pipeId" && resolvedRow.pipeIdRule ? resolvedRow.pipeIdRule : "the sheet's numbering";
      return { ...base, text: id ?? "", italic: !overridden && Boolean(id), origin: overridden ? "entered" : id ? `derived: ${rule}` : "blank" };
    }
    case "supportId": case "supportLabel": {
      const typed = column.key === "supportId" ? (row.values.ids?.support ?? "") : entered("supportLabel");
      // Shown only where the row has a support; an entry elsewhere stays visible with its refusal.
      const shown = resolvedRow.ids.support ? (column.key === "supportId" ? resolvedRow.ids.support : resolvedRow.labels.support) : undefined;
      if (!shown) return { ...base, text: typed, origin: typed.trim() ? "entered" : "blank" };
      const rule = column.key === "supportId" ? "support: and the support node's id token" : "the support id's token";
      return { ...base, text: shown, italic: !typed.trim(), muted: !typed.trim(), origin: typed.trim() ? "entered" : `derived: ${rule}` };
    }
    case "bendStartId": case "bendEndId": case "bendArcId": case "bendComponentId": {
      const cell = BEND_COLUMNS.find(([key]) => key === column.key)![1];
      const typed = row.values.ids?.[cell.input] ?? "";
      const shown = resolvedRow.kind === "bend" ? resolvedRow.ids.bend?.[cell.member] : undefined;
      if (!shown) return { ...base, text: typed, origin: typed.trim() ? "entered" : "blank" };
      return { ...base, text: shown, italic: !typed.trim(), muted: !typed.trim(), origin: typed.trim() ? "entered" : "derived: the corner node's id token" };
    }
    case "bendSource": {
      const source = resolvedRow.bendSource;
      if (!source) return { ...base, text: entered("bendSource"), origin: entered("bendSource").trim() ? "entered" : "blank" };
      return { ...base, text: source.value, italic: source.origin.kind === "derived", muted: source.origin.kind !== "entered", origin: originText(source.origin) };
    }
    case "position": {
      if (row.storedMissing) return { ...base, text: "", origin: `missing: ${row.storedMissing}` };
      const end = row.storedPipeEnd ?? resolvedRow.end?.position;
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
      const field = componentField(column);
      if (field) return { ...row, edits: { ...row.edits, component: { ...row.restored.component, ...row.edits.component, [field]: text } }, engineRefusal: null };
      const member = ID_COLUMNS[column];
      if (member) {
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
  const field = componentField(column);
  if (field && row.component) return row.component[field];
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
