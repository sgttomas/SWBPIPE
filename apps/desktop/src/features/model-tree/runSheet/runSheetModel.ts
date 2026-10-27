// Run-sheet entry, slice S1 (DEL-07-02 Design `run-sheet-entry.md`): the row
// types, the inheritance resolution (§2), id assignment (§3.2) and the sheet's
// own rules (§4.2). Pure and synchronous: unit conversions arrive as a table the
// caller fills through the engine's display-conversion facade (§6.2), and the
// resolver lists any conversion it still needs. The batch builder
// (`runSheetBatch.ts`) reads only this module's output.
//
// The sheet suggests no section, material, orientation, radius or k (§6.3):
// every value below is entered, inherited from an entered value, read from a
// stored record, or derived by a stated rule (ids, labels, readouts).
import { convertDisplayQuantities, type DisplayQuantityRequest } from "../../../services/displayQuantityService";
import { trimLikeRust } from "../../../services/rustTrim";
import type { PreviewModel } from "../../../types";

export type Vec3 = { x: number; y: number; z: number };
export type Quantity = { value: number; unit: string };

/** §1.1 Kind codes; blank is a run. */
export type RunSheetKind = "run" | "bend" | "support" | "from" | "point";
export const KIND_CODES: Readonly<Record<string, RunSheetKind>> = { R: "run", B: "bend", S: "support", F: "from", P: "point" };
/** §5.2 Support codes and the `family` each writes. */
export const SUPPORT_CODES: Readonly<Record<string, string>> = { A: "anchor", G: "guide", L: "line_stop", V: "vertical_support" };
/** The restraint vocabulary (`operation_applier` `RESTRAINT_TOKENS`); `*` is all six. */
export const RESTRAINT_TOKENS = ["UX", "UY", "UZ", "RX", "RY", "RZ"] as const;

/** Quantity columns (§1.2): each has one column unit; values are stored as entered. */
export type QuantityColumn =
  | "od" | "wall" | "insulationThickness" | "insulationDensity" | "contentsDensity"
  | "materialDensity" | "millTolerance" | "corrosionAllowance" | "bendRadius";
export const QUANTITY_COLUMNS: Readonly<Record<QuantityColumn, { label: string; dimension: "length" | "density"; member?: string }>> = {
  od: { label: "OD", dimension: "length", member: "outside_diameter" },
  wall: { label: "Wall", dimension: "length", member: "wall_thickness" },
  insulationThickness: { label: "Ins. thk", dimension: "length", member: "insulation_thickness" },
  insulationDensity: { label: "Ins. dens.", dimension: "density", member: "insulation_density" },
  contentsDensity: { label: "Contents dens.", dimension: "density", member: "contents_density" },
  materialDensity: { label: "Material density", dimension: "density", member: "material_density" },
  millTolerance: { label: "Mill tolerance", dimension: "length", member: "mill_tolerance" },
  corrosionAllowance: { label: "Corrosion allowance", dimension: "length", member: "corrosion_allowance" },
  bendRadius: { label: "Bend R", dimension: "length" },
};
/** Optional section members, ended by `-` (§2.4). */
export const OPTIONAL_SECTION_COLUMNS = [
  "insulationThickness", "insulationDensity", "contentsDensity", "materialDensity", "millTolerance", "corrosionAllowance",
] as const satisfies readonly QuantityColumn[];
/** §2.1 The cells that inherit. */
export const INHERITING_CELLS = ["section", "od", "wall", "material", "orientation", ...OPTIONAL_SECTION_COLUMNS, "bendRadius"] as const;
export type InheritingCell = typeof INHERITING_CELLS[number];

/** One sheet row as typed. Every cell is raw text; blank is `""` or absent. */
export type RunSheetRowInput = {
  key: string;
  kind?: string;
  dx?: string; dy?: string; dz?: string;
  support?: string; restraints?: string;
  node?: string;
  section?: string;
  od?: string; wall?: string;
  material?: string;
  orientation?: string;
  insulationThickness?: string; insulationDensity?: string; contentsDensity?: string;
  materialDensity?: string; millTolerance?: string; corrosionAllowance?: string;
  bendRadius?: string; k?: string; kSource?: string;
  label?: string;
  /** Ids the user overwrote (§3.2); otherwise derived from the row's number. */
  ids?: { node?: string; pipe?: string; support?: string };
};

export type RunSheetNumbering = { nodePrefix: string; pipePrefix: string; start: number; step: number };
export const DEFAULT_NUMBERING: RunSheetNumbering = { nodePrefix: "node:N", pipePrefix: "pipe:P", start: 10, step: 10 };

export type RunSheetInput = {
  sheetKey: string;
  batchSeq: number;
  /** The increments' unit; default `project.units.length` (§1.2 sheet header). */
  lengthUnit?: string;
  /** Header Source: the provenance of every created entity (§1.2). */
  source: string;
  numbering?: Partial<RunSheetNumbering>;
  /** Column units chosen in the headers; default the project unit of the dimension. */
  columnUnits?: Partial<Record<QuantityColumn, string>>;
  rows: RunSheetRowInput[];
};

/** §3.2 The counter: numbers only move forward and are never reused. */
export type RunSheetIdState = { next: number; assigned: Readonly<Record<string, number>> };

export type CellOrigin =
  | { kind: "entered" }
  | { kind: "inherited"; row: number }
  | { kind: "stored"; pipe: string }
  | { kind: "derived"; rule: string }
  | { kind: "blank" };
/** A resolved cell. `absent` marks an optional member ended by `-` (§2.4). */
export type ResolvedCell<T> = { value?: T; absent?: true; origin: CellOrigin; error?: RunSheetFinding };
export type ResolvedCells = {
  section: ResolvedCell<string>;
  od: ResolvedCell<Quantity>;
  wall: ResolvedCell<Quantity>;
  material: ResolvedCell<string>;
  orientation: ResolvedCell<Vec3>;
  insulationThickness: ResolvedCell<Quantity>;
  insulationDensity: ResolvedCell<Quantity>;
  contentsDensity: ResolvedCell<Quantity>;
  materialDensity: ResolvedCell<Quantity>;
  millTolerance: ResolvedCell<Quantity>;
  corrosionAllowance: ResolvedCell<Quantity>;
  bendRadius: ResolvedCell<Quantity>;
};

export type RunSheetRule =
  | "no-current-node" | "zero-length" | "below-resolution" | "both-forms" | "coincident" | "near-coincident"
  | "missing-material" | "missing-section" | "missing-orientation" | "parallel-orientation" | "invalid-code"
  | "increment-not-number" | "quantity-without-unit" | "quantity-not-number" | "support" | "bend" | "ids"
  | "unknown-node" | "source" | "length-unit";
export type RunSheetFinding = { rule: RunSheetRule; message: string; column?: string; detail?: string };

export type BendIds = { start: string; end: string; arc: string; component: string };
export type ResolvedRunSheetRow = {
  key: string;
  /** 1-based row number, as shown in the # column. */
  number: number;
  kind: RunSheetKind | null;
  /** The node this row starts from: the current node (run, bend, S), the named node (F) or none (P). */
  start: string | null;
  startPosition: Vec3 | null;
  /** The row's end node: a run's new or named end, P's node, F's named node; null for S or when unknown. */
  end: { id: string; position: Vec3; existing: boolean } | null;
  /** Typed increments as numbers in the sheet unit (blank is 0), or P's absolute coordinates. */
  increments: Vec3 | null;
  /** Readout: the run's length in the stored unit. */
  length: number | null;
  cells: ResolvedCells;
  label: { value: string; origin: CellOrigin } | null;
  ids: { node?: string; pipe?: string; support?: string; bend?: BendIds };
  labels: { pipe?: string; support?: string };
  support: { family: string; restraints: string[] } | null;
  bend: { radius: Quantity; k: number; kSource: string } | null;
  refusals: RunSheetFinding[];
  warnings: RunSheetFinding[];
};

export type ResolvedRunSheet = {
  basis: PreviewModel;
  sheetKey: string;
  batchSeq: number;
  /** Trimmed header Source. */
  source: string;
  storedLengthUnit: string;
  sheetLengthUnit: string;
  rows: ResolvedRunSheetRow[];
  /** Sheet-level findings (Source, length units): they disable Queue (§4.2). */
  blockers: RunSheetFinding[];
  /** Conversions the table lacked; resolve again once they are added. */
  missingConversions: DisplayQuantityRequest[];
  queueable: boolean;
};

export type RunSheetConversions = ReadonlyMap<string, number | null>;
export type RunSheetContext = {
  model: PreviewModel;
  idState: RunSheetIdState;
  /** Create, connect and insert targets of pending intents and queued batches (§3.2). */
  reservedIds?: Iterable<string>;
  conversions?: RunSheetConversions;
};

export const MESSAGES = {
  noCurrentNode: "Start the sheet with F (from a node) or P (at a point).",
  zeroLength: "This run has no length.",
  belowResolution: "The increment is below the coordinate resolution at this position.",
  bothForms: "Enter increments or an end node, not both.",
  coincident: (id: string) => `Ends on node ${id}. Name it in Node to connect to it.`,
  nearCoincident: (id: string) => `Within the node-proximity tolerance of node ${id}, and not joined to it.`,
  missingMaterial: "Enter a material id of this model.",
  missingSection: "Enter a section, or OD and wall.",
  missingOrientation: "Enter an orientation (y reference).",
  parallelOrientation: "The orientation is parallel to this run; enter another.",
  invalidCode: (text: string, what: "kind" | "support") => `${text} is not a ${what} code.`,
  incrementNotNumber: (unit: string) => `Increments are numbers in ${unit}, the sheet's unit.`,
  quantityWithoutUnit: (column: string) => `Choose the ${column} unit.`,
  quantityNotNumber: (column: string, unit: string) => `${column} is a number in ${unit}.`,
  support: "Enter the restraints of this support.",
  restraintsWithoutSupport: "Enter the support code of these restraints.",
  idUsed: (id: string) => `Id ${id} is already used.`,
  idBlank: (what: string) => `Enter the ${what} id.`,
  unknownNode: (id: string) => `Node ${id} is not in the model or drafted earlier in this sheet.`,
  source: "Enter the sheet's Source.",
  noStoredUnit: "The model has no project length unit.",
  noSheetUnit: "Choose the sheet's length unit.",
  bendNeedsNextRun: "A bend needs the next run.",
  bendMismatch: (field: string, next: number) => `A bend joins one section and material; row ${next} differs in ${field}.`,
  bendSupport: "A bend corner carries no support.",
  bendCornerReferenced: (row: number) => `Row ${row} refers to this bend's corner; a bend corner joins exactly two pipes and nothing else.`,
  bendCornerPipes: (count: number) => `A bend corner joins exactly two pipes; this one would join ${count}.`,
  bendRadius: "Enter a positive bend radius R.",
  bendK: "Enter k and its source for this bend; none is suggested.",
  notChecked: "Not checked: an earlier row is refused.",
} as const;

const NUMBER = /^[+-]?(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?$/;
const text = (value: string | undefined) => trimLikeRust(value ?? "");
/** A typed number, or null for anything else (no hex, no Infinity, no unit text). */
export function parseNumber(raw: string): number | null {
  const value = text(raw);
  if (!NUMBER.test(value)) return null;
  const parsed = Number(value);
  return Number.isFinite(parsed) ? parsed : null;
}
/** §6.2 −0 cannot cross JSON, so every computed coordinate is normalized before use. */
export const normalizeZero = (x: number) => (x === 0 ? 0 : x);
/** The id's token: the id without its `type:` prefix (§3.2, as `InsertBendAtCornerForm`). */
export const idToken = (id: string) => id.replace(/^[^:]*:/, "");

export function numberingOf(input: RunSheetInput): RunSheetNumbering {
  return { ...DEFAULT_NUMBERING, ...(input.numbering ?? {}) };
}

/** Every id a row number yields (§3.2): end node, pipe, support and the bend's four. */
export function idsForNumber(numbering: RunSheetNumbering, n: number): { node: string; pipe: string; support: string; bend: BendIds } {
  const node = `${numbering.nodePrefix}${n}`;
  return { node, pipe: `${numbering.pipePrefix}${n}`, support: supportIdFor(node), bend: bendIdsFor(node) };
}
export const supportIdFor = (node: string) => `support:${idToken(node)}`;
export function bendIdsFor(corner: string): BendIds {
  const token = idToken(corner);
  return { start: `node:${token}-T1`, end: `node:${token}-T2`, arc: `pipe:${token}-bend`, component: `component:${token}-bend` };
}

export function initialIdState(numbering: RunSheetNumbering = DEFAULT_NUMBERING): RunSheetIdState {
  return { next: numbering.start, assigned: {} };
}

/** §3.2 Assign a row its number once, skipping any number that yields a reserved id.
 * The counter only moves forward, so a released number is never reused. */
export function assignRowNumber(state: RunSheetIdState, numbering: RunSheetNumbering, rowKey: string, reserved: ReadonlySet<string>): RunSheetIdState {
  if (state.assigned[rowKey] !== undefined) return state;
  let n = state.next;
  const yields = (value: number) => {
    const ids = idsForNumber(numbering, value);
    return [ids.node, ids.pipe, ids.support, ids.bend.start, ids.bend.end, ids.bend.arc, ids.bend.component];
  };
  while (yields(n).some((id) => reserved.has(id))) n += numbering.step;
  return { next: n + numbering.step, assigned: { ...state.assigned, [rowKey]: n } };
}

/** Every id the model holds, in any collection of records with an `id`. */
export function modelIds(model: PreviewModel): Set<string> {
  const ids = new Set<string>();
  for (const value of Object.values(model as unknown as Record<string, unknown>)) {
    if (!Array.isArray(value)) continue;
    for (const item of value) {
      if (item && typeof item === "object" && typeof (item as { id?: unknown }).id === "string") ids.add((item as { id: string }).id);
    }
  }
  return ids;
}

/** §3.2 Reservations from pending intents and queued batches: create, connect
 * and insert targets (the rule of `routeDraft.ts` `reservedIds`, widened to
 * queued batches), and the new ids a queued bend insertion names. */
export function reservedOperationIds(operations: ReadonlyArray<unknown>): Set<string> {
  const ids = new Set<string>();
  for (const candidate of operations) {
    if (!candidate || typeof candidate !== "object") continue;
    const op = candidate as { operation_kind?: unknown; target?: { ref?: unknown }; change?: { change_kind?: unknown; after?: unknown } };
    if ((op.operation_kind === "create" || op.operation_kind === "connect" || op.operation_kind === "insert") && typeof op.target?.ref === "string" && op.target.ref.trim()) {
      ids.add(op.target.ref);
    }
    if (op.change?.change_kind === "insert_bend_at_corner" && typeof op.change.after === "string") {
      try {
        const after = JSON.parse(op.change.after) as { new_nodes?: Array<{ id?: unknown }>; new_pipe?: { id?: unknown }; component?: { id?: unknown } };
        for (const item of [...(after.new_nodes ?? []), after.new_pipe, after.component]) {
          if (item && typeof item.id === "string") ids.add(item.id);
        }
      } catch {
        // A malformed queued payload reserves nothing more; the engine refuses it.
      }
    }
  }
  return ids;
}

export function conversionKey(value: number, from: string, to: string, dimension: string): string {
  return `${JSON.stringify(normalizeZero(value))}|${from}|${to}|${dimension}`;
}

type Converter = { convert(value: number, from: string, to: string, dimension: string): number | null | undefined; missing: DisplayQuantityRequest[] };
function converter(table: RunSheetConversions | undefined): Converter {
  const missing: DisplayQuantityRequest[] = [];
  const seen = new Set<string>();
  return {
    missing,
    convert(value, from, to, dimension) {
      // §6.2 c(v) = v when the sheet and stored units are the same unit: the
      // identity branch of `convert_for_dimension`, taken here without a call.
      if (from === to) return value;
      const key = conversionKey(value, from, to, dimension);
      if (table?.has(key)) return table.get(key);
      if (!seen.has(key)) {
        seen.add(key);
        missing.push({ id: key, value: normalizeZero(value), from_unit: from, to_unit: to, dimension_id: dimension });
      }
      return undefined;
    },
  };
}

type PipeRecord = PreviewModel["pipe_segments"][number];
type SectionRecord = NonNullable<PreviewModel["sections"]>[number];

/** The binding rules' shared pipe section (`section_bindings.rs` `source`), or why not. */
export function sharedPipeSection(model: PreviewModel, id: string): { section: SectionRecord } | { error: string } {
  const matches = (model.sections ?? []).filter((section) => section.id === id);
  if (!id || matches.length !== 1) return { error: `Section ${id} is not a section of this model.` };
  const section = matches[0];
  const properties = section.properties ?? {};
  const keys = Object.keys(properties);
  if (typeof section.name !== "string" || section.provenance === undefined) return { error: `Section ${id} has no name or provenance.` };
  if (section.section_type !== "pipe" || keys.length !== 2 || !("outside_diameter" in properties) || !("wall_thickness" in properties)) {
    return { error: `Section ${id} is not a shared pipe section (type pipe, outside diameter and wall only).` };
  }
  return { section };
}

const sameJson = (a: unknown, b: unknown): boolean => {
  if (a === b) return true;
  if (Array.isArray(a) || Array.isArray(b)) return Array.isArray(a) && Array.isArray(b) && a.length === b.length && a.every((v, i) => sameJson(v, b[i]));
  if (!a || !b || typeof a !== "object" || typeof b !== "object") return false;
  const ka = Object.keys(a).sort();
  const kb = Object.keys(b).sort();
  return ka.length === kb.length && ka.every((k, i) => k === kb[i] && sameJson((a as Record<string, unknown>)[k], (b as Record<string, unknown>)[k]));
};

const blankCells = (): ResolvedCells => ({
  section: { origin: { kind: "blank" } }, od: { origin: { kind: "blank" } }, wall: { origin: { kind: "blank" } },
  material: { origin: { kind: "blank" } }, orientation: { origin: { kind: "blank" } },
  insulationThickness: { origin: { kind: "blank" } }, insulationDensity: { origin: { kind: "blank" } },
  contentsDensity: { origin: { kind: "blank" } }, materialDensity: { origin: { kind: "blank" } },
  millTolerance: { origin: { kind: "blank" } }, corrosionAllowance: { origin: { kind: "blank" } },
  bendRadius: { origin: { kind: "blank" } },
});

/** §2.2 A blank cell takes the predecessor's effective value; the origin names
 * the row the value was entered on, or the stored pipe it came from. */
function inherit<T>(cell: ResolvedCell<T>, predecessorNumber: number): ResolvedCell<T> {
  if (cell.value === undefined && !cell.absent) return { origin: { kind: "blank" } };
  const origin: CellOrigin = cell.origin.kind === "entered" || cell.origin.kind === "derived"
    ? { kind: "inherited", row: predecessorNumber }
    : cell.origin;
  return { ...(cell.value !== undefined ? { value: cell.value } : {}), ...(cell.absent ? { absent: true as const } : {}), origin };
}

/** §2.3 rule 2: a field inherits from the stored pipes meeting a node only when all agree. */
function fromStoredPipes(pipes: PipeRecord[]): ResolvedCells {
  const cells = blankCells();
  if (pipes.length === 0) return cells;
  const origin: CellOrigin = { kind: "stored", pipe: pipes[0].id };
  const agreed = <T>(read: (pipe: PipeRecord) => T): { agree: boolean; value: T } => {
    const first = read(pipes[0]);
    return { agree: pipes.every((pipe) => sameJson(read(pipe), first)), value: first };
  };
  const ref = agreed((pipe) => pipe.section_ref);
  if (ref.agree && typeof ref.value === "string") cells.section = { value: ref.value, origin };
  for (const column of ["od", "wall", ...OPTIONAL_SECTION_COLUMNS] as const) {
    const member = QUANTITY_COLUMNS[column].member!;
    const value = agreed((pipe) => pipe.section?.[member]);
    if (!value.agree) continue;
    cells[column] = value.value === undefined
      ? (column === "od" || column === "wall" ? { origin: { kind: "blank" } } : { absent: true, origin })
      : { value: { value: value.value.value, unit: value.value.unit }, origin };
  }
  const material = agreed((pipe) => pipe.material);
  if (material.agree && material.value) cells.material = { value: material.value, origin };
  const orientation = agreed((pipe) => pipe.y_reference);
  if (orientation.agree && orientation.value) cells.orientation = { value: { x: orientation.value.x, y: orientation.value.y, z: orientation.value.z }, origin };
  return cells;
}

/** The resolver (§2, §3.2, §4.2). Pure: the same input gives the same output. */
export function resolveRunSheet(input: RunSheetInput, context: RunSheetContext): ResolvedRunSheet {
  const model = context.model;
  const numbering = numberingOf(input);
  const convert = converter(context.conversions);
  const storedLengthUnit = model.project?.units?.length ?? "";
  const sheetLengthUnit = text(input.lengthUnit) || storedLengthUnit;
  const blockers: RunSheetFinding[] = [];
  const source = text(input.source);
  if (!source) blockers.push({ rule: "source", message: MESSAGES.source });
  if (!storedLengthUnit) blockers.push({ rule: "length-unit", message: MESSAGES.noStoredUnit });
  if (!sheetLengthUnit) blockers.push({ rule: "length-unit", message: MESSAGES.noSheetUnit });

  const columnUnit = (column: QuantityColumn): string =>
    text(input.columnUnits?.[column]) || text(model.project?.units?.[QUANTITY_COLUMNS[column].dimension]);
  const storedNodes = new Map(model.nodes.map((node) => [node.id, node]));
  const rows: ResolvedRunSheetRow[] = [];
  /** Nodes drafted by earlier rows: id -> the drafting row. */
  const drafted = new Map<string, ResolvedRunSheetRow>();

  input.rows.forEach((raw, index) => {
    const number = index + 1;
    const refusals: RunSheetFinding[] = [];
    const warnings: RunSheetFinding[] = [];
    const kindText = text(raw.kind).toUpperCase();
    const kind: RunSheetKind | null = kindText === "" ? "run" : KIND_CODES[kindText] ?? null;
    if (kind === null) refusals.push({ rule: "invalid-code", column: "Kind", message: MESSAGES.invalidCode(text(raw.kind), "kind") });
    const continues = kind === "run" || kind === "bend" || kind === "support";
    const predecessor = continues && index > 0 ? rows[index - 1] : null;
    const nodeName = text(raw.node);

    // ---- The current node (§1.1).
    let start: string | null = null;
    let startPosition: Vec3 | null = null;
    if (continues) {
      const end = predecessor?.end ?? null;
      if (end) {
        start = end.id;
        startPosition = end.position;
      } else if (predecessor?.kind === "support" && predecessor.start) {
        start = predecessor.start;
        startPosition = predecessor.startPosition;
      } else {
        refusals.push({ rule: "no-current-node", message: MESSAGES.noCurrentNode });
      }
    }

    // ---- Inheritance (§2).
    let cells: ResolvedCells;
    if (kind === "from") {
      const draftRow = drafted.get(nodeName);
      if (draftRow) {
        cells = blankCells();
        for (const cell of INHERITING_CELLS) (cells as Record<string, ResolvedCell<unknown>>)[cell] = inherit(draftRow.cells[cell] as ResolvedCell<unknown>, draftRow.number);
      } else {
        cells = fromStoredPipes(model.pipe_segments.filter((pipe) => pipe.from === nodeName || pipe.to === nodeName));
      }
    } else if (kind === "point" || predecessor === null) {
      cells = blankCells();
    } else {
      cells = blankCells();
      for (const cell of INHERITING_CELLS) (cells as Record<string, ResolvedCell<unknown>>)[cell] = inherit(predecessor.cells[cell] as ResolvedCell<unknown>, predecessor.number);
    }
    // An S row passes its predecessor's values through unchanged (§2.2): it enters none.
    const accepts = kind !== null && kind !== "support";
    const entered = <K extends keyof RunSheetRowInput>(key: K) => text(raw[key] as string | undefined);
    const enteredValue = <K extends keyof RunSheetRowInput>(key: K) => (accepts ? entered(key) : "");
    const quantity = (column: QuantityColumn, optional: boolean): ResolvedCell<Quantity> | null => {
      const typed = enteredValue(column);
      if (!typed) return null;
      if (optional && typed === "-") return { absent: true, origin: { kind: "entered" } };
      const unit = columnUnit(column);
      const label = QUANTITY_COLUMNS[column].label;
      if (!unit) return { origin: { kind: "entered" }, error: { rule: "quantity-without-unit", column: label, message: MESSAGES.quantityWithoutUnit(label) } };
      const value = parseNumber(typed);
      if (value === null) return { origin: { kind: "entered" }, error: { rule: "quantity-not-number", column: label, message: MESSAGES.quantityNotNumber(label, unit) } };
      return { value: { value: normalizeZero(value), unit }, origin: { kind: "entered" } };
    };
    // Section, OD and wall (§2.4): an entered Section wins and shows the
    // section's values; an entered OD or Wall under an inherited Section clears it.
    const sectionTyped = enteredValue("section");
    const odTyped = quantity("od", false);
    const wallTyped = quantity("wall", false);
    if (sectionTyped) {
      cells.section = { value: sectionTyped, origin: { kind: "entered" } };
    } else if ((odTyped || wallTyped) && cells.section.value !== undefined) {
      cells.section = { origin: { kind: "derived", rule: "cleared by the OD or Wall entered on this row" } };
    }
    let sectionError: string | undefined;
    if (cells.section.value !== undefined) {
      const shared = sharedPipeSection(model, cells.section.value);
      if ("error" in shared) {
        sectionError = shared.error;
        cells.od = { origin: { kind: "blank" } };
        cells.wall = { origin: { kind: "blank" } };
      } else {
        const rule = `from section ${shared.section.id}`;
        const od = shared.section.properties.outside_diameter!;
        const wall = shared.section.properties.wall_thickness!;
        cells.od = { value: { value: od.value, unit: od.unit }, origin: { kind: "derived", rule } };
        cells.wall = { value: { value: wall.value, unit: wall.unit }, origin: { kind: "derived", rule } };
      }
    } else {
      if (odTyped) cells.od = odTyped;
      if (wallTyped) cells.wall = wallTyped;
    }
    for (const column of OPTIONAL_SECTION_COLUMNS) {
      const typed = quantity(column, true);
      if (typed) cells[column] = typed;
    }
    const materialTyped = enteredValue("material");
    if (materialTyped) cells.material = { value: materialTyped, origin: { kind: "entered" } };
    const orientationTyped = enteredValue("orientation");
    if (orientationTyped) {
      const parts = orientationTyped.split(/[\s,]+/).filter(Boolean).map(parseNumber);
      cells.orientation = parts.length === 3 && parts.every((part) => part !== null)
        ? { value: { x: normalizeZero(parts[0]!), y: normalizeZero(parts[1]!), z: normalizeZero(parts[2]!) }, origin: { kind: "entered" } }
        : { origin: { kind: "entered" }, error: { rule: "missing-orientation", column: "Orientation", message: MESSAGES.missingOrientation, detail: "Enter the orientation as three numbers." } };
    }
    // Bend R inherits only from earlier bend rows of its chain; it is entered on bend rows only.
    if (kind === "bend") {
      const typed = quantity("bendRadius", false);
      if (typed) cells.bendRadius = typed;
    } else if (kind === "from" || kind === "point") {
      cells.bendRadius = { origin: { kind: "blank" } };
    }

    // ---- Ids (§3.2).
    const assigned = context.idState.assigned[raw.key];
    const derivedIds = assigned === undefined ? null : idsForNumber(numbering, assigned);
    const overrides = raw.ids ?? {};
    const ids: ResolvedRunSheetRow["ids"] = {};
    const labels: ResolvedRunSheetRow["labels"] = {};

    // ---- Geometry.
    let end: ResolvedRunSheetRow["end"] = null;
    let increments: Vec3 | null = null;
    let length: number | null = null;
    const incrementCells = (["dx", "dy", "dz"] as const).map((axis) => entered(axis));
    const anyIncrement = incrementCells.some(Boolean);
    const parseIncrements = (): Vec3 | null => {
      const values = incrementCells.map((cell) => (cell === "" ? 0 : parseNumber(cell)));
      if (values.some((value) => value === null)) {
        refusals.push({ rule: "increment-not-number", column: "dX, dY, dZ", message: MESSAGES.incrementNotNumber(sheetLengthUnit || "the sheet unit") });
        return null;
      }
      return { x: values[0]!, y: values[1]!, z: values[2]! };
    };
    const c = (value: number) => (sheetLengthUnit && storedLengthUnit ? convert.convert(value, sheetLengthUnit, storedLengthUnit, "length") : undefined);
    /** c over three axes; null when the facade cannot convert, undefined while a conversion is pending. */
    const convertAll = (values: Vec3): Vec3 | null | undefined => {
      const converted = [values.x, values.y, values.z].map(c);
      if (converted.some((value) => value === null)) {
        refusals.push({ rule: "length-unit", column: "dX, dY, dZ", message: `The sheet unit ${sheetLengthUnit} does not convert to the model's length unit ${storedLengthUnit}.` });
        return null;
      }
      if (converted.some((value) => value === undefined)) return undefined;
      return { x: converted[0]!, y: converted[1]!, z: converted[2]! };
    };
    const nodePosition = (id: string): Vec3 | null => drafted.get(id)?.end?.position ?? (storedNodes.get(id)?.position ?? null);

    if (kind === "from") {
      const position = nodeName ? nodePosition(nodeName) : null;
      if (!nodeName || !position) refusals.push({ rule: "no-current-node", column: "Node", message: MESSAGES.noCurrentNode, ...(nodeName ? { detail: MESSAGES.unknownNode(nodeName) } : {}) });
      else {
        start = nodeName;
        startPosition = position;
        end = { id: nodeName, position, existing: true };
      }
    } else if (kind === "point") {
      increments = parseIncrements();
      const converted = increments ? convertAll(increments) : null;
      if (converted) {
        // §6.2 x_P = c(X), normalized.
        const id = text(overrides.node) || derivedIds?.node || "";
        end = { id, position: { x: normalizeZero(converted.x), y: normalizeZero(converted.y), z: normalizeZero(converted.z) }, existing: false };
      }
    } else if ((kind === "run" || kind === "bend") && start && startPosition) {
      if (nodeName) {
        if (anyIncrement) refusals.push({ rule: "both-forms", column: "Node", message: MESSAGES.bothForms });
        const position = nodePosition(nodeName);
        if (!position) refusals.push({ rule: "unknown-node", column: "Node", message: MESSAGES.unknownNode(nodeName) });
        else {
          end = { id: nodeName, position, existing: true };
          if (nodeName === start || sameJson(position, startPosition)) refusals.push({ rule: "zero-length", message: MESSAGES.zeroLength });
        }
      } else {
        increments = parseIncrements();
        if (increments) {
          if (increments.x === 0 && increments.y === 0 && increments.z === 0) refusals.push({ rule: "zero-length", message: MESSAGES.zeroLength });
          const converted = convertAll(increments);
          if (converted) {
            // §6.2 x_k = x_{k−1} + c(dx_k), one rounding in the conversion and one in the addition.
            const x = normalizeZero(startPosition.x + converted.x);
            const y = normalizeZero(startPosition.y + converted.y);
            const z = normalizeZero(startPosition.z + converted.z);
            const position = { x, y, z };
            const id = text(overrides.node) || derivedIds?.node || "";
            end = { id, position, existing: false };
            const moved = increments.x !== 0 || increments.y !== 0 || increments.z !== 0;
            if (moved && x === startPosition.x && y === startPosition.y && z === startPosition.z) {
              refusals.push({ rule: "below-resolution", column: "dX, dY, dZ", message: MESSAGES.belowResolution });
            }
          }
        }
      }
      if (end && startPosition) length = Math.hypot(end.position.x - startPosition.x, end.position.y - startPosition.y, end.position.z - startPosition.z);
    }

    // ---- Ids, labels and support (§3.2, §5.2).
    const creates = (kind === "run" || kind === "bend") ? !nodeName : kind === "point";
    if (creates) {
      ids.node = text(overrides.node) || derivedIds?.node || "";
      if (!ids.node) refusals.push({ rule: "ids", column: "Ids", message: MESSAGES.idBlank("node") });
    }
    if (kind === "run" || kind === "bend") {
      ids.pipe = text(overrides.pipe) || derivedIds?.pipe || "";
      if (!ids.pipe) refusals.push({ rule: "ids", column: "Ids", message: MESSAGES.idBlank("pipe") });
      else labels.pipe = idToken(ids.pipe);
    }
    let label: ResolvedRunSheetRow["label"] = null;
    if (creates && ids.node) {
      const typed = enteredValue("label");
      label = typed ? { value: typed, origin: { kind: "entered" } } : { value: idToken(ids.node), origin: { kind: "derived", rule: "the node id's token" } };
    }
    const supportText = text(raw.support).toUpperCase();
    const restraintsText = text(raw.restraints);
    let support: ResolvedRunSheetRow["support"] = null;
    if (supportText) {
      const family = SUPPORT_CODES[supportText];
      if (!family) refusals.push({ rule: "invalid-code", column: "Support", message: MESSAGES.invalidCode(text(raw.support), "support") });
      const tokens = restraintsText === "*" ? [...RESTRAINT_TOKENS] : restraintsText.split(/[\s,]+/).filter(Boolean).map((token) => token.toUpperCase());
      const valid = tokens.length > 0 && tokens.every((token) => (RESTRAINT_TOKENS as readonly string[]).includes(token)) && new Set(tokens).size === tokens.length;
      if (!valid) refusals.push({ rule: "support", column: "Restraints", message: MESSAGES.support });
      if (family && valid) support = { family, restraints: tokens };
      const at = kind === "support" ? start : end?.id ?? null;
      if (at && kind !== null) {
        ids.support = text(overrides.support) || supportIdFor(at);
        labels.support = idToken(ids.support);
      }
    } else if (restraintsText) {
      refusals.push({ rule: "support", column: "Restraints", message: MESSAGES.restraintsWithoutSupport });
    }

    // ---- Element rules (§4.2) for rows that create a pipe.
    if (kind === "run" || kind === "bend") {
      const material = cells.material.value;
      if (!material || !(model.materials ?? []).some((item) => item.id === material)) {
        refusals.push({ rule: "missing-material", column: "Material", message: MESSAGES.missingMaterial });
      }
      const unitless = [cells.od.error, cells.wall.error].find((error) => error?.rule === "quantity-without-unit");
      const sectionDetail = sectionError
        ?? (cells.od.error ?? cells.wall.error)?.message
        ?? (cells.od.value === undefined ? "OD is missing." : cells.wall.value === undefined ? "Wall is missing." : undefined)
        ?? (cells.od.value!.value <= 0 || cells.wall.value!.value <= 0 ? "OD and wall must be positive." : undefined);
      if (unitless) refusals.push(unitless);
      else if (sectionDetail) refusals.push({ rule: "missing-section", column: "Section", message: MESSAGES.missingSection, detail: sectionDetail });
      else {
        const od = convert.convert(cells.od.value!.value, cells.od.value!.unit, storedLengthUnit, "length");
        const wall = convert.convert(cells.wall.value!.value, cells.wall.value!.unit, storedLengthUnit, "length");
        if (typeof od === "number" && typeof wall === "number" && wall >= od / 2) {
          refusals.push({ rule: "missing-section", column: "Wall", message: MESSAGES.missingSection, detail: "The wall must be less than half the OD." });
        }
      }
      for (const column of OPTIONAL_SECTION_COLUMNS) {
        const error = cells[column].error;
        if (error) refusals.push(error);
      }
      const y = cells.orientation.value;
      if (cells.orientation.error) refusals.push(cells.orientation.error);
      else if (!y || (y.x === 0 && y.y === 0 && y.z === 0)) {
        refusals.push({ rule: "missing-orientation", column: "Orientation", message: MESSAGES.missingOrientation });
      } else if (end && startPosition && length) {
        const d = { x: (end.position.x - startPosition.x) / length, y: (end.position.y - startPosition.y) / length, z: (end.position.z - startPosition.z) / length };
        const dot = y.x * d.x + y.y * d.y + y.z * d.z;
        if (y.x - dot * d.x === 0 && y.y - dot * d.y === 0 && y.z - dot * d.z === 0) {
          refusals.push({ rule: "parallel-orientation", column: "Orientation", message: MESSAGES.parallelOrientation });
        }
      }
    }
    let bend: ResolvedRunSheetRow["bend"] = null;
    if (kind === "bend") {
      const radius = cells.bendRadius;
      if (radius.error?.rule === "quantity-without-unit") refusals.push(radius.error);
      else if (!radius.value || radius.value.value <= 0) refusals.push({ rule: "bend", column: "Bend R", message: MESSAGES.bendRadius });
      const k = parseNumber(enteredValue("k"));
      const kSource = enteredValue("kSource");
      if (k === null || k <= 0 || !kSource) refusals.push({ rule: "bend", column: "k", message: MESSAGES.bendK });
      if (supportText) refusals.push({ rule: "bend", column: "Support", message: MESSAGES.bendSupport });
      if (end?.id) ids.bend = bendIdsFor(end.id);
      if (radius.value && radius.value.value > 0 && k !== null && k > 0 && kSource) bend = { radius: radius.value, k, kSource };
    }

    const row: ResolvedRunSheetRow = {
      key: raw.key, number, kind, start, startPosition, end, increments, length, cells, label, ids, labels, support, bend, refusals, warnings,
    };
    rows.push(row);
    if (end && !end.existing && (kind === "run" || kind === "bend" || kind === "point") && end.id) drafted.set(end.id, row);
  });

  // ---- Rules that need the whole sheet.
  const reservedOutside = new Set<string>([...modelIds(model), ...(context.reservedIds ?? [])]);
  const holders = new Map<string, number>();
  const createdIds = (row: ResolvedRunSheetRow): string[] => {
    const out: string[] = [];
    if (row.ids.node) out.push(row.ids.node);
    if (row.ids.pipe) out.push(row.ids.pipe);
    if (row.ids.support && row.support) out.push(row.ids.support);
    if (row.ids.bend) out.push(row.ids.bend.start, row.ids.bend.end, row.ids.bend.arc, row.ids.bend.component);
    return out;
  };
  for (const row of rows) for (const id of createdIds(row)) holders.set(id, (holders.get(id) ?? 0) + 1);
  for (const row of rows) {
    for (const id of createdIds(row)) {
      if (reservedOutside.has(id) || (holders.get(id) ?? 0) > 1) {
        row.refusals.push({ rule: "ids", column: "Ids", message: MESSAGES.idUsed(id) });
      }
    }
  }

  // Coincidence (exact: refused) and near-coincidence (warning) of each new node.
  const tolerance = model.model_settings?.node_proximity_tolerance;
  const toleranceStored = convert.convert(tolerance ? tolerance.value : 0.005, tolerance ? tolerance.unit : "m", storedLengthUnit, "length");
  const drafts: Array<{ id: string; position: Vec3; row: number }> = [];
  const joined = new Set<string>();
  for (const row of rows) {
    if ((row.kind === "run" || row.kind === "bend") && row.start && row.end) joined.add([row.start, row.end.id].sort().join("\u0000"));
  }
  for (const row of rows) {
    if (!row.end || row.end.existing || !(row.kind === "run" || row.kind === "bend" || row.kind === "point")) continue;
    const here = row.end.position;
    const candidates = [
      ...model.nodes.map((node) => ({ id: node.id, position: node.position })),
      ...drafts,
    ];
    const same = candidates.find((node) => node.position.x === here.x && node.position.y === here.y && node.position.z === here.z);
    if (same) row.refusals.push({ rule: "coincident", message: MESSAGES.coincident(same.id) });
    else if (typeof toleranceStored === "number") {
      const near = candidates.find((node) =>
        !joined.has([node.id, row.end!.id].sort().join("\u0000")) &&
        Math.hypot(node.position.x - here.x, node.position.y - here.y, node.position.z - here.z) <= toleranceStored);
      if (near) row.warnings.push({ rule: "near-coincident", message: MESSAGES.nearCoincident(near.id) });
    }
    drafts.push({ id: row.end.id, position: here, row: row.number });
  }

  // Bend requirements (§3.3): the next run, one section and material, nothing else at the corner.
  rows.forEach((row, index) => {
    if (row.kind !== "bend") return;
    const next = rows[index + 1];
    if (!next || !(next.kind === "run" || next.kind === "bend")) {
      row.refusals.push({ rule: "bend", message: MESSAGES.bendNeedsNextRun });
      return;
    }
    const corner = row.end?.id;
    if (!corner) return;
    const compared: Array<[string, (cells: ResolvedCells) => unknown]> = [
      ["Section", (cells) => cells.section.value],
      ["Material", (cells) => cells.material.value],
      ...(["od", "wall", ...OPTIONAL_SECTION_COLUMNS] as const).map((column) => [QUANTITY_COLUMNS[column].label, (cells: ResolvedCells) => cells[column].value] as [string, (cells: ResolvedCells) => unknown]),
    ];
    const differs = compared.find(([, read]) => !sameJson(read(row.cells), read(next.cells)));
    if (differs) row.refusals.push({ rule: "bend", message: MESSAGES.bendMismatch(differs[0], next.number) });
    const referencing = rows.find((other) => other !== row && other !== next && (
      (other.kind === "from" && other.start === corner) ||
      ((other.kind === "run" || other.kind === "bend") && other.end?.id === corner) ||
      (other.kind === "support" && other.start === corner)));
    if (referencing) row.refusals.push({ rule: "bend", message: MESSAGES.bendCornerReferenced(referencing.number) });
    const storedPipes = model.pipe_segments.filter((pipe) => pipe.from === corner || pipe.to === corner).length;
    const draftedPipes = rows.filter((other) => (other.kind === "run" || other.kind === "bend") && other.start && other.end && (other.start === corner || other.end.id === corner)).length;
    if (!referencing && storedPipes + draftedPipes !== 2) row.refusals.push({ rule: "bend", message: MESSAGES.bendCornerPipes(storedPipes + draftedPipes) });
  });

  const missingConversions = convert.missing;
  return {
    basis: model,
    sheetKey: input.sheetKey,
    batchSeq: input.batchSeq,
    source,
    storedLengthUnit,
    sheetLengthUnit,
    rows,
    blockers,
    missingConversions,
    queueable: blockers.length === 0 && missingConversions.length === 0 && rows.length > 0 && rows.every((row) => row.refusals.length === 0),
  };
}

/** Resolve with every unit conversion taken from the engine's display-conversion
 * facade (§6.2: `convert_for_dimension`, never a reimplemented factor). */
export async function resolveRunSheetWithEngine(input: RunSheetInput, context: Omit<RunSheetContext, "conversions"> & { conversions?: Map<string, number | null> }): Promise<{ resolved: ResolvedRunSheet; conversions: Map<string, number | null> }> {
  const conversions = new Map(context.conversions ?? []);
  for (let pass = 0; pass < 3; pass += 1) {
    const resolved = resolveRunSheet(input, { ...context, conversions });
    if (resolved.missingConversions.length === 0) return { resolved, conversions };
    const results = await convertDisplayQuantities(resolved.missingConversions);
    resolved.missingConversions.forEach((request, index) => {
      const result = results[index];
      conversions.set(request.id, result && result.status === "converted" ? result.value : null);
    });
  }
  return { resolved: resolveRunSheet(input, { ...context, conversions }), conversions };
}
