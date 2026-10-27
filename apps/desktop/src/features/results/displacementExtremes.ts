/** Displacement extremes per node across chosen cases (survey item A8; DEL-07-05 Design
 * `displacement-extremes.md`).
 *
 * For each model node: the largest and smallest emitted global translation (ux, uy, uz) and
 * rotation (rx, ry, rz) over the load cases and combinations the user chooses, each naming the
 * case that holds it and its result-row id. A filter lists only the nodes of supports of one kind
 * (the model's stated support family, or any hanger); a sort orders nodes by the largest magnitude
 * of one component.
 *
 * Pure post-processing of emitted rows: the extremes are arithmetic over the listed values, not a
 * clearance or travel check, and no limit is supplied. A case the numerical standing withholds
 * (the viewport overlays' rule, `overlayMayShowRow`) keeps no value and is counted; a range
 * envelope is not one solved state and stays out (A2's rule). Stored values and units are kept
 * unchanged for an export (D2). */

import type { MechanicsResult, PreviewModel } from "../../types";
import { basisKey } from "../viewport/resultColourOverlay";
import { overlayMayShowRow, type ResultOverlayStanding } from "./numericalHealth";
import {
  SUPPORT_LOAD_COLUMNS,
  SUPPORT_LOAD_FRAME,
  caseWithheldReason,
  supportLoadCases,
  type SupportLoadCase,
  type SupportLoadColumn
} from "./supportLoadSummary";

type ResultRow = MechanicsResult["results"][number];
type Support = PreviewModel["supports"][number];

export type DisplacementComponentId = "ux" | "uy" | "uz" | "rx" | "ry" | "rz";

/** The six node-movement columns of A2's table (kind, label, dimension), in display order. */
export const DISPLACEMENT_COMPONENTS: readonly (SupportLoadColumn & { id: DisplacementComponentId })[] = Object.freeze(
  SUPPORT_LOAD_COLUMNS.filter((item): item is SupportLoadColumn & { id: DisplacementComponentId } => item.of === "node")
);

/** The model document's `SupportFamily` values with the support form's labels (a test keeps the
 * values equal to `schemas/model.schema.yaml`). */
export const SUPPORT_FAMILIES: readonly Readonly<{ value: string; label: string }>[] = Object.freeze([
  { value: "anchor", label: "Anchor" },
  { value: "guide", label: "Guide" },
  { value: "line_stop", label: "Line stop" },
  { value: "vertical_support", label: "Vertical support" },
  { value: "spring", label: "Spring" },
  { value: "variable_spring_hanger", label: "Variable spring hanger" },
  { value: "spring_hanger", label: "Spring hanger" },
  { value: "constant_effort_support", label: "Constant-effort support" },
  { value: "nonlinear", label: "Nonlinear support" }
].map((item) => Object.freeze(item)));

/** The hanger families (also the `SpringHanger.hanger_type` values). */
export const HANGER_FAMILIES: readonly string[] = Object.freeze(["variable_spring_hanger", "spring_hanger", "constant_effort_support"]);

const FAMILY_LABEL = new Map(SUPPORT_FAMILIES.map((item) => [item.value, item.label] as const));

export const FAMILY_NOT_STATED = "Family not stated";
export function familyLabel(family: string | null): string {
  if (family === null) return FAMILY_NOT_STATED;
  return FAMILY_LABEL.get(family) ?? `${JSON.stringify(family)} (as stated)`;
}

/** One emitted value: the stored value and unit, unchanged, and the row it comes from. */
export type DisplacementValue = Readonly<{ value: number; unit: string; resultId: string }>;
export type DisplacementExtremeValue = Readonly<DisplacementValue & { caseKey: string; caseLabel: string }>;

export type DisplacementCase = Readonly<SupportLoadCase & {
  standing: "admitted" | "withheld";
  withheldReason: string | null;
  /** Finite movement rows of the case (of any node). */
  rowCount: number;
  /** Admitted and not a range envelope: the case may enter the extremes. */
  choosable: boolean;
}>;

export type DisplacementSupport = Readonly<{
  supportId: string;
  label: string;
  /** The stated family, or null when absent or null. */
  family: string | null;
  /** The family's label, with a nonlinear support's stated behavior and DOF. */
  kindText: string;
  hanger: boolean;
}>;

export type DisplacementNode = Readonly<{
  nodeId: string;
  label: string;
  /** Model order. */
  order: number;
  supports: readonly DisplacementSupport[];
  /** Per component, the kept value of each choosable case, indexed as `DisplacementExtremes.cases`. */
  values: Readonly<Record<DisplacementComponentId, readonly (DisplacementValue | undefined)[]>>;
}>;

export type DisplacementFilter = Readonly<{ id: string; label: string; nodeCount: number }>;

export type DisplacementExtremes = Readonly<{
  state: "available" | "unavailable";
  reason: string | null;
  cases: readonly DisplacementCase[];
  nodes: readonly DisplacementNode[];
  filters: readonly DisplacementFilter[];
  /** Finite movement rows not used: in another frame, or of a node not in the model. */
  unusedRows: Readonly<{ otherFrame: number; notModelNode: number }>;
}>;

export const DISPLACEMENT_NO_RESULT = "No result";
export const DISPLACEMENT_NOT_SOLVED = "Mechanics not solved";
export const DISPLACEMENT_NO_NODES = "The model has no nodes";
export const DISPLACEMENT_NO_CASES = "No load case or combination with displacement or rotation rows";
export const DISPLACEMENT_ENVELOPE_REASON = "Range envelope: not one solved state";

export const FILTER_ALL = "all";
export const FILTER_SUPPORTS = "supports";
export const FILTER_HANGERS = "hangers";
export const FILTER_FAMILY_NOT_STATED = "family-not-stated";
export const familyFilterId = (family: string) => `family:${family}`;

const COMPONENT_BY_KIND = new Map(DISPLACEMENT_COMPONENTS.map((item) => [item.kind, item] as const));

function supportOf(support: Support): DisplacementSupport {
  const family = typeof support.family === "string" && support.family !== "" ? support.family : null;
  const nonlinear = family === "nonlinear" && support.nonlinear ? ` (${[support.nonlinear.behavior, support.nonlinear.dof].filter(Boolean).join(", ")})` : "";
  return Object.freeze({
    supportId: support.id,
    label: support.label || support.id,
    family,
    kindText: `${familyLabel(family)}${nonlinear}`,
    hanger: (family !== null && HANGER_FAMILIES.includes(family)) || HANGER_FAMILIES.includes(support.hanger?.hanger_type ?? "")
  });
}

function unavailable(reason: string): DisplacementExtremes {
  return Object.freeze({
    state: "unavailable", reason, cases: Object.freeze([]), nodes: Object.freeze([]), filters: Object.freeze([]),
    unusedRows: Object.freeze({ otherFrame: 0, notModelNode: 0 })
  });
}

function filtersOf(nodes: readonly DisplacementNode[]): DisplacementFilter[] {
  const count = (test: (node: DisplacementNode) => boolean) => nodes.filter(test).length;
  const stated = new Set(nodes.flatMap((node) => node.supports.map((item) => item.family)));
  const families = [
    ...SUPPORT_FAMILIES.map((item) => item.value).filter((value) => stated.has(value)),
    ...[...stated].filter((value): value is string => value !== null && !FAMILY_LABEL.has(value))
  ];
  const filters: DisplacementFilter[] = [
    { id: FILTER_ALL, label: "All nodes", nodeCount: nodes.length },
    { id: FILTER_SUPPORTS, label: "Support nodes", nodeCount: count((node) => node.supports.length > 0) },
    { id: FILTER_HANGERS, label: "Hangers", nodeCount: count((node) => node.supports.some((item) => item.hanger)) },
    ...families.map((family) => ({ id: familyFilterId(family), label: familyLabel(family), nodeCount: count((node) => node.supports.some((item) => item.family === family)) })),
    { id: FILTER_FAMILY_NOT_STATED, label: FAMILY_NOT_STATED, nodeCount: count((node) => node.supports.some((item) => item.family === null)) }
  ];
  return filters.filter((item) => item.id === FILTER_ALL || item.nodeCount > 0).map((item) => Object.freeze(item));
}

/** Whether a node is listed under a filter id (an unknown id lists every node). */
export function nodeMatchesFilter(node: DisplacementNode, filter: string): boolean {
  if (filter === FILTER_SUPPORTS) return node.supports.length > 0;
  if (filter === FILTER_HANGERS) return node.supports.some((item) => item.hanger);
  if (filter === FILTER_FAMILY_NOT_STATED) return node.supports.some((item) => item.family === null);
  if (filter.startsWith("family:")) return node.supports.some((item) => item.family === filter.slice("family:".length));
  return true;
}

/**
 * Every model node's kept movement values over the cases the result states movement rows for.
 * `standing` is the reader's numerical standing and the session's numerical health view, the
 * same input the viewport overlays take.
 */
export function buildDisplacementExtremes(input: {
  model: PreviewModel;
  result: MechanicsResult | null;
  standing: ResultOverlayStanding | null | undefined;
}): DisplacementExtremes {
  const { model, result, standing } = input;
  if (!result) return unavailable(DISPLACEMENT_NO_RESULT);
  if (result.status.mechanics !== "MECHANICS_SOLVED") return unavailable(DISPLACEMENT_NOT_SOLVED);
  if (model.nodes.length === 0) return unavailable(DISPLACEMENT_NO_NODES);

  // Rows per case key, then the cases with movement rows in A2's case order.
  const rowCounts = new Map<string, number>();
  const movementRows: Array<{ row: ResultRow; key: string; component: DisplacementComponentId }> = [];
  for (const row of result.results) {
    const component = COMPONENT_BY_KIND.get(row.kind);
    const key = basisKey(row.basis_ref);
    if (!component || key === null || !Number.isFinite(row.value)) continue;
    if (row.basis_ref!.ref_type !== "load_case" && row.basis_ref!.ref_type !== "combination") continue;
    rowCounts.set(key, (rowCounts.get(key) ?? 0) + 1);
    movementRows.push({ row, key, component: component.id });
  }
  const cases: DisplacementCase[] = supportLoadCases(model, result)
    .filter((item) => rowCounts.has(item.key))
    .map((item) => {
      const admitted = overlayMayShowRow(standing, { basis_ref: item.basis });
      return Object.freeze({
        ...item,
        standing: admitted ? "admitted" : "withheld",
        withheldReason: admitted ? null : caseWithheldReason(standing, item.basis),
        rowCount: rowCounts.get(item.key)!,
        choosable: admitted && !item.envelope
      } as const);
    });
  if (cases.length === 0) return unavailable(DISPLACEMENT_NO_CASES);

  const caseIndex = new Map(cases.map((item, index) => [item.key, index] as const));
  const nodeIndex = new Map(model.nodes.map((node, index) => [node.id, index] as const));
  const values = model.nodes.map(() => Object.fromEntries(DISPLACEMENT_COMPONENTS.map((item) => [item.id, new Array<DisplacementValue | undefined>(cases.length)])) as Record<DisplacementComponentId, (DisplacementValue | undefined)[]>);
  let otherFrame = 0;
  let notModelNode = 0;
  for (const { row, key, component } of movementRows) {
    const at = caseIndex.get(key);
    if (at === undefined || !cases[at].choosable) continue;
    const node = nodeIndex.get(row.entity_ref);
    if (node === undefined) { notModelNode += 1; continue; }
    // The kinds are global; a row that states another frame is not used.
    if (row.metadata?.coordinate_system !== undefined && row.metadata.coordinate_system !== SUPPORT_LOAD_FRAME) { otherFrame += 1; continue; }
    const slots = values[node][component];
    // The first emitted row of a component for the case is kept; the producer emits one.
    if (slots[at] === undefined) slots[at] = Object.freeze({ value: row.value, unit: row.unit, resultId: row.id });
  }

  const supportsByNode = new Map<string, DisplacementSupport[]>();
  for (const support of model.supports) {
    const list = supportsByNode.get(support.node) ?? [];
    list.push(supportOf(support));
    supportsByNode.set(support.node, list);
  }
  const nodes = model.nodes.map((node, order): DisplacementNode => Object.freeze({
    nodeId: node.id,
    label: node.label || node.id,
    order,
    supports: Object.freeze(supportsByNode.get(node.id) ?? []),
    values: Object.freeze(Object.fromEntries(Object.entries(values[order]).map(([id, slots]) => [id, Object.freeze(slots)])) as DisplacementNode["values"])
  }));
  return Object.freeze({
    state: "available",
    reason: null,
    cases: Object.freeze(cases),
    nodes: Object.freeze(nodes),
    filters: Object.freeze(filtersOf(nodes)),
    unusedRows: Object.freeze({ otherFrame, notModelNode })
  });
}

// --- The view: chosen cases, filter and sort (§5) -------------------------------------------

export type DisplacementSort = "model" | DisplacementComponentId;

export type DisplacementExtreme = Readonly<{
  max: DisplacementExtremeValue | null;
  min: DisplacementExtremeValue | null;
  /** Why no extreme is formed although values exist (the units differ). */
  note: string | null;
}>;

export type DisplacementNodeExtremes = Readonly<{
  nodeId: string;
  label: string;
  order: number;
  supports: readonly DisplacementSupport[];
  extremes: Readonly<Record<DisplacementComponentId, DisplacementExtreme>>;
  /** The sort key (largest |value| of the sort component), or null: model order, or unranked. */
  key: number | null;
}>;

export type DisplacementExtremesView = Readonly<{
  filter: DisplacementFilter;
  chosenCases: readonly DisplacementCase[];
  /** Cases listed but not in the extremes. */
  exclusions: Readonly<{ withheld: number; withheldRows: number; envelope: number; notChosen: number }>;
  rows: readonly DisplacementNodeExtremes[];
  sort: Readonly<{ component: DisplacementComponentId | null; sorted: boolean; note: string | null; rankedCount: number }>;
  /** Per component, the one stored unit of every extreme listed, or null. */
  units: Readonly<Record<DisplacementComponentId, string | null>>;
}>;

export function extremeUnitsDifferNote(units: readonly string[]): string {
  return `Units differ (${units.join(", ")}); no extreme is formed`;
}
export function sortUnitsDifferNote(units: readonly string[]): string {
  return `Units differ (${units.join(", ")}); not sorted`;
}

function extremeOf(node: DisplacementNode, component: DisplacementComponentId, chosen: readonly { index: number; item: DisplacementCase }[]): DisplacementExtreme {
  const slots = node.values[component];
  const units = new Set<string>();
  let max: DisplacementExtremeValue | null = null;
  let min: DisplacementExtremeValue | null = null;
  // Case order; strict comparisons, so a tie keeps the first case (and 0 ties -0).
  for (const { index, item } of chosen) {
    const value = slots[index];
    if (!value) continue;
    units.add(value.unit);
    if (max === null || value.value > max.value) max = Object.freeze({ ...value, caseKey: item.key, caseLabel: item.label });
    if (min === null || value.value < min.value) min = Object.freeze({ ...value, caseKey: item.key, caseLabel: item.label });
  }
  if (units.size > 1) return Object.freeze({ max: null, min: null, note: extremeUnitsDifferNote([...units].sort()) });
  return Object.freeze({ max, min, note: null });
}

/** The view of the derivation under the user's case choice, filter and sort (§5). Pure; frozen. */
export function displacementExtremesView(
  data: DisplacementExtremes,
  options: { caseKeys: readonly string[] | null; filter: string; sort: DisplacementSort }
): DisplacementExtremesView {
  const filter = data.filters.find((item) => item.id === options.filter) ?? data.filters.find((item) => item.id === FILTER_ALL)
    ?? Object.freeze({ id: FILTER_ALL, label: "All nodes", nodeCount: 0 });
  const wanted = options.caseKeys === null ? null : new Set(options.caseKeys);
  const chosen = data.cases.flatMap((item, index) => (item.choosable && (wanted === null || wanted.has(item.key)) ? [{ index, item }] : []));
  const withheld = data.cases.filter((item) => item.standing === "withheld");
  const exclusions = Object.freeze({
    withheld: withheld.length,
    withheldRows: withheld.reduce((sum, item) => sum + item.rowCount, 0),
    envelope: data.cases.filter((item) => item.standing === "admitted" && item.envelope).length,
    notChosen: data.cases.filter((item) => item.choosable).length - chosen.length
  });
  const component = options.sort === "model" ? null : DISPLACEMENT_COMPONENTS.find((item) => item.id === options.sort)?.id ?? null;
  const listed = data.nodes.filter((node) => nodeMatchesFilter(node, filter.id)).map((node) => {
    const extremes = Object.fromEntries(DISPLACEMENT_COMPONENTS.map((item) => [item.id, extremeOf(node, item.id, chosen)])) as Record<DisplacementComponentId, DisplacementExtreme>;
    const sortExtreme = component ? extremes[component] : null;
    const key = sortExtreme?.max && sortExtreme.min ? Math.max(Math.abs(sortExtreme.max.value), Math.abs(sortExtreme.min.value)) : null;
    return { node, extremes: Object.freeze(extremes), key, unit: sortExtreme?.max?.unit ?? null };
  });
  const keyed = listed.filter((item) => item.key !== null);
  const sortUnits = [...new Set(keyed.map((item) => item.unit!))].sort();
  const sorted = component !== null && sortUnits.length <= 1;
  const ordered = sorted
    ? [...keyed.sort((a, b) => (b.key! - a.key!) || (a.node.order - b.node.order)), ...listed.filter((item) => item.key === null)]
    : listed;
  const rows = ordered.map((item) => Object.freeze({
    nodeId: item.node.nodeId,
    label: item.node.label,
    order: item.node.order,
    supports: item.node.supports,
    extremes: item.extremes,
    key: sorted ? item.key : null
  }));
  const units = Object.fromEntries(DISPLACEMENT_COMPONENTS.map((item) => {
    const seen = new Set(rows.flatMap((row) => [row.extremes[item.id].max?.unit, row.extremes[item.id].min?.unit].filter((unit): unit is string => unit !== undefined)));
    return [item.id, seen.size === 1 ? [...seen][0] : null];
  })) as Record<DisplacementComponentId, string | null>;
  return Object.freeze({
    filter,
    chosenCases: Object.freeze(chosen.map((entry) => entry.item)),
    exclusions,
    rows: Object.freeze(rows),
    sort: Object.freeze({
      component,
      sorted,
      note: component !== null && !sorted ? sortUnitsDifferNote(sortUnits) : null,
      rankedCount: sorted ? keyed.length : 0
    }),
    units: Object.freeze(units)
  });
}
