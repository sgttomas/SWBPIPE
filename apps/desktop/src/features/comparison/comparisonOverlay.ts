// The comparison overlay's view model (DEL-07-08 REQ-011, AC-003): from a comparison result to
// the marks to draw, and from the marks and the two compared models to what is drawn. It reads the
// comparison result the table reads and classifies nothing itself: a state comparison's marks are
// its rows' classes (DEL-14-03), a run comparison's marks are its rows' statuses (DEL-14-04),
// named in the words the tables show.
//
// What is drawn: the later of the two compared models (the right side), with each entity that
// exists only in the earlier one (the left side) drawn as a ghost in its own position. A classified
// entity with no drawing (the model, a material, a section, a load case, a setting, or an entity
// whose position cannot be read) is counted by class and reason, never dropped, and nothing is
// defaulted to make the drawing look complete (REQ-008). A difference is shown, not judged
// (AX-003): no text here says more than which comparison class an entity is in.
import type { RunComparison } from "./runComparison";
import { FAMILY_LABELS, type EntityFamily, type StateComparison } from "./stateComparison";
import { CLASSIFICATION_LABELS } from "./stateComparisonCsv";

export type ComparisonOverlayKind = "states" | "runs";

/** The state classes, as the state table names them (`CLASSIFICATION_LABELS`). */
export type StateOverlayClass = "added" | "removed" | "changed" | "unchanged";
/** The run classes, as the run table's status filter and counts name them. */
export type RunOverlayClass = "nonzero_delta" | "zero_delta" | "not_compared" | "left_only" | "right_only";
export type OverlayClass = StateOverlayClass | RunOverlayClass;

/** Where the classified thing exists: the earlier (left) side, the later (right) side, or both. */
export type OverlaySide = "left" | "right" | "both";

/** One mark: an entity reference, its class and the side it exists on. A state mark carries its
 * family; a run mark's entity is named only by its rows' `object_ref`, so its family is null. */
export type OverlayMark = Readonly<{ ref: string; family: EntityFamily | null; markClass: OverlayClass; side: OverlaySide }>;

/** A class's words, its glyph and its line style: the glyph and the line style are the class's
 * signals besides colour (DEL-07-06 REQ-011). `swatch` names the colour slot the stylesheet gives
 * the class (`.comparison-overlay [data-swatch]`), from the token source. */
export type OverlayClassStyle = Readonly<{ markClass: OverlayClass; label: string; glyph: string; line: "solid" | "dashed" | "dash-dot" | "dotted" | "thin"; swatch: string }>;

export const STATE_OVERLAY_CLASSES: readonly OverlayClassStyle[] = Object.freeze([
  { markClass: "added", label: CLASSIFICATION_LABELS.added, glyph: "+", line: "solid", swatch: "added" },
  { markClass: "removed", label: CLASSIFICATION_LABELS.removed, glyph: "−", line: "dashed", swatch: "removed" },
  { markClass: "changed", label: CLASSIFICATION_LABELS.changed, glyph: "Δ", line: "dash-dot", swatch: "changed" },
  { markClass: "unchanged", label: CLASSIFICATION_LABELS.unchanged, glyph: "=", line: "thin", swatch: "unchanged" }
]);

export const RUN_OVERLAY_CLASSES: readonly OverlayClassStyle[] = Object.freeze([
  { markClass: "nonzero_delta", label: "Compared with a non-zero delta", glyph: "Δ", line: "dash-dot", swatch: "changed" },
  { markClass: "not_compared", label: "Not compared", glyph: "n", line: "dotted", swatch: "not-compared" },
  { markClass: "left_only", label: "Only in the left run", glyph: "L", line: "dashed", swatch: "removed" },
  { markClass: "right_only", label: "Only in the right run", glyph: "R", line: "solid", swatch: "added" },
  { markClass: "zero_delta", label: "Compared with a zero delta", glyph: "=", line: "thin", swatch: "unchanged" }
]);

export function overlayClasses(kind: ComparisonOverlayKind): readonly OverlayClassStyle[] {
  return kind === "states" ? STATE_OVERLAY_CLASSES : RUN_OVERLAY_CLASSES;
}

/** The state comparison's marks: one per row, of the row's class, on the side it exists. */
export function stateOverlayMarks(comparison: StateComparison): OverlayMark[] {
  return comparison.rows.map((row) => ({
    ref: row.id,
    family: row.family,
    markClass: row.classification,
    side: row.classification === "added" ? "right" : row.classification === "removed" ? "left" : "both"
  }));
}

/** The run comparison's marks: for each entity its rows name (`object_ref`), one mark per class
 * its rows are in: a compared pair by whether its delta is zero, a pair not compared, and a row
 * only in one run. An entity can be in several classes; each is a mark. */
export function runOverlayMarks(comparison: RunComparison): OverlayMark[] {
  const marks = new Map<string, OverlayMark>();
  const add = (ref: string, markClass: RunOverlayClass, side: OverlaySide) => {
    const key = JSON.stringify([ref, markClass]);
    if (!marks.has(key)) marks.set(key, { ref, family: null, markClass, side });
  };
  for (const pair of comparison.pairs) {
    const markClass = pair.status === "not_compared" ? "not_compared" : pair.delta !== 0 ? "nonzero_delta" : "zero_delta";
    add(pair.identity.object_ref, markClass, "both");
  }
  for (const row of comparison.unmatched) add(row.identity.object_ref, row.side === "left" ? "left_only" : "right_only", row.side);
  return [...marks.values()];
}

// ---------------------------------------------------------------------------
// What is drawn

export type OverlayPoint = Readonly<{ x: number; y: number; z: number }>;

/** The families the overlay draws: a node at its position, a pipe segment between its end nodes,
 * a support and a component at their node. */
export const DRAWN_FAMILIES: readonly EntityFamily[] = Object.freeze(["node", "pipe_segment", "support", "component"]);

const COLLECTION_OF: Readonly<Partial<Record<EntityFamily, string>>> = {
  node: "nodes",
  pipe_segment: "pipe_segments",
  support: "supports",
  component: "components"
};

/** Why a classified entity is not drawn. */
export type NotDrawnReason = "no_drawing" | "no_position" | "not_in_either_model" | "length_unit_differs";

export const NOT_DRAWN_LABELS: Readonly<Record<NotDrawnReason, string>> = Object.freeze({
  no_drawing: "an entity the model drawing does not show (the model, a material, a section, a load case, a setting or a record)",
  no_position: "an entity whose position cannot be read from its model",
  not_in_either_model: "a result row's entity that neither compared model holds",
  length_unit_differs: "an entity only in the earlier model, whose length unit differs from the later model's (positions are not converted)"
});

/** One drawn entity: its reference and family, its classes in legend order, whether it is a ghost
 * (it exists only in the earlier model and is drawn in its own position there), and its geometry. */
export type DrawnOverlayEntity = Readonly<{
  ref: string;
  family: EntityFamily;
  /** The entity's name as the model tree shows it: its record's label, else its reference. */
  label: string;
  classes: readonly OverlayClass[];
  ghost: boolean;
  geometry: Readonly<{ kind: "point"; at: OverlayPoint }> | Readonly<{ kind: "segment"; from: OverlayPoint; to: OverlayPoint }>;
}>;

export type OverlayClassCount = Readonly<{ markClass: OverlayClass; label: string; total: number; drawn: number; notDrawn: number }>;

export type ComparisonOverlayScene = Readonly<{
  kind: ComparisonOverlayKind;
  marks: readonly OverlayMark[];
  /** Every entity of the later model the drawing shows (unclassified ones as context, `classes`
   * empty), then the ghosts of the earlier model; marked ones carry their classes. */
  drawn: readonly DrawnOverlayEntity[];
  /** The classified entities not drawn, with why, in mark order. */
  notDrawn: readonly Readonly<{ mark: OverlayMark; reason: NotDrawnReason }>[];
  /** Per class in legend order: how many entities it marks, how many are drawn, how many are not. */
  counts: readonly OverlayClassCount[];
}>;

type PlainObject = Record<string, unknown>;

function isPlainObject(value: unknown): value is PlainObject {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function finitePoint(value: unknown): OverlayPoint | null {
  if (!isPlainObject(value)) return null;
  const { x, y, z } = value;
  return typeof x === "number" && typeof y === "number" && typeof z === "number" && Number.isFinite(x) && Number.isFinite(y) && Number.isFinite(z)
    ? { x, y, z }
    : null;
}

/** A model document's drawn entities by family and id, read defensively: a document the overlay
 * cannot read gives none. */
type DrawnIndex = {
  lengthUnit: string | null;
  byRef: Map<string, { family: EntityFamily; record: PlainObject }>;
  nodes: Map<string, OverlayPoint | null>;
  order: Array<{ family: EntityFamily; id: string }>;
};

function drawnIndex(document: unknown): DrawnIndex {
  const index: DrawnIndex = { lengthUnit: null, byRef: new Map(), nodes: new Map(), order: [] };
  if (!isPlainObject(document)) return index;
  const project = document.project;
  const units = isPlainObject(project) ? project.units : null;
  index.lengthUnit = isPlainObject(units) && typeof units.length === "string" ? units.length : null;
  for (const family of DRAWN_FAMILIES) {
    const list = document[COLLECTION_OF[family]!];
    if (!Array.isArray(list)) continue;
    for (const record of list) {
      if (!isPlainObject(record) || typeof record.id !== "string") continue;
      if (index.byRef.has(record.id)) continue;
      index.byRef.set(record.id, { family, record });
      index.order.push({ family, id: record.id });
      if (family === "node") index.nodes.set(record.id, finitePoint(record.position));
    }
  }
  return index;
}

function labelOf(record: PlainObject, ref: string): string {
  return typeof record.label === "string" && record.label ? record.label : ref;
}

function geometryOf(index: DrawnIndex, family: EntityFamily, record: PlainObject): DrawnOverlayEntity["geometry"] | null {
  const nodeAt = (id: unknown) => (typeof id === "string" ? index.nodes.get(id) ?? null : null);
  if (family === "node") {
    const at = finitePoint(record.position);
    return at ? { kind: "point", at } : null;
  }
  if (family === "pipe_segment") {
    const from = nodeAt(record.from);
    const to = nodeAt(record.to);
    return from && to ? { kind: "segment", from, to } : null;
  }
  const at = nodeAt(record.node);
  return at ? { kind: "point", at } : null;
}

/**
 * What the overlay draws, from the marks and the two compared model documents (`left` the earlier,
 * `right` the later). Every drawable entity of the later model is drawn, with the classes its marks
 * give it; a marked entity found only in the earlier model is drawn as a ghost in its own position
 * there, provided both models state the same length unit; every other mark is counted as not
 * drawn, with its reason.
 */
export function comparisonOverlayScene(kind: ComparisonOverlayKind, marks: readonly OverlayMark[], left: unknown, right: unknown): ComparisonOverlayScene {
  const styles = overlayClasses(kind);
  const order = new Map(styles.map((style, position) => [style.markClass, position] as const));
  const later = drawnIndex(right);
  const earlier = drawnIndex(left);
  const classesByRef = new Map<string, Set<OverlayClass>>();
  for (const mark of marks) {
    const set = classesByRef.get(mark.ref) ?? new Set<OverlayClass>();
    set.add(mark.markClass);
    classesByRef.set(mark.ref, set);
  }
  const sorted = (ref: string) => [...(classesByRef.get(ref) ?? [])].sort((a, b) => order.get(a)! - order.get(b)!);
  const familyMatches = (mark: OverlayMark, family: EntityFamily) => mark.family === null || mark.family === family;
  const drawn: DrawnOverlayEntity[] = [];
  const drawnRefs = new Set<string>();
  for (const { family, id } of later.order) {
    const geometry = geometryOf(later, family, later.byRef.get(id)!.record);
    if (!geometry) continue;
    drawn.push({ ref: id, family, label: labelOf(later.byRef.get(id)!.record, id), classes: sorted(id), ghost: false, geometry });
    drawnRefs.add(id);
  }
  const ghostRefs = new Set<string>();
  const notDrawn: Array<{ mark: OverlayMark; reason: NotDrawnReason }> = [];
  for (const mark of marks) {
    const inLater = later.byRef.get(mark.ref);
    if (inLater && familyMatches(mark, inLater.family)) {
      if (!drawnRefs.has(mark.ref)) notDrawn.push({ mark, reason: "no_position" });
      continue;
    }
    if (mark.family !== null && !DRAWN_FAMILIES.includes(mark.family)) {
      notDrawn.push({ mark, reason: "no_drawing" });
      continue;
    }
    const inEarlier = earlier.byRef.get(mark.ref);
    if (!inEarlier || !familyMatches(mark, inEarlier.family)) {
      notDrawn.push({ mark, reason: mark.family === null ? "not_in_either_model" : "no_position" });
      continue;
    }
    if (earlier.lengthUnit !== later.lengthUnit) {
      notDrawn.push({ mark, reason: "length_unit_differs" });
      continue;
    }
    if (ghostRefs.has(mark.ref)) continue;
    const geometry = geometryOf(earlier, inEarlier.family, inEarlier.record);
    if (!geometry) {
      notDrawn.push({ mark, reason: "no_position" });
      continue;
    }
    drawn.push({ ref: mark.ref, family: inEarlier.family, label: labelOf(inEarlier.record, mark.ref), classes: sorted(mark.ref), ghost: true, geometry });
    ghostRefs.add(mark.ref);
  }
  const counts = styles.map((style): OverlayClassCount => {
    const total = marks.filter((mark) => mark.markClass === style.markClass).length;
    const notDrawnCount = notDrawn.filter((entry) => entry.mark.markClass === style.markClass).length;
    return { markClass: style.markClass, label: style.label, total, drawn: total - notDrawnCount, notDrawn: notDrawnCount };
  });
  return { kind, marks, drawn, notDrawn, counts };
}

/** An entity's name in the overlay's words: its family and reference. */
export function overlayEntityName(entity: Pick<DrawnOverlayEntity, "ref" | "family">): string {
  return `${FAMILY_LABELS[entity.family]} ${entity.ref}`;
}

/** The words of an entity's classes, for its accessible name. */
export function overlayClassWords(kind: ComparisonOverlayKind, classes: readonly OverlayClass[]): string {
  const styles = overlayClasses(kind);
  return classes.length
    ? classes.map((markClass) => styles.find((style) => style.markClass === markClass)!.label).join("; ")
    : "not classified by this comparison";
}
