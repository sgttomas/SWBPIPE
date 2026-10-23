// Result colour on the viewport's tubes (FR-GUI-001; design system 2.6 and 6.7; owner ruling
// OD-5). One scalar per member, for one load case or combination: the largest absolute value of
// one emitted row kind over that member's rows (its stations, or its end nodes for a nodal
// quantity). A magnitude display of emitted rows only: it sets no threshold, computes no
// engineering quantity and makes no professional claim. Colour is view state; nothing here
// writes to the model, the result or any export.

import type { MechanicsResult, PreviewModel, ResultBasisRef } from "../../types";
import { overlayMayShowRow, type ResultOverlayStanding } from "../results/numericalHealth";
import { entityKey } from "../workspace/selectionState";
import { resultNoValueHex, resultScaleHex } from "./viewportPalette";
import type { ResultColourTable } from "./viewportResource";

type ResultRow = MechanicsResult["results"][number];

export type ResultOverlayDimension = "length" | "stress" | "force" | "moment";

/** How a quantity's rows reach a member: rows of the member itself, rows of its two end nodes,
 * or rows of another entity whose emitted source rows are the member's. */
export type ResultColourAttachment = "member" | "end_nodes" | "source_rows";

export type ResultColourQuantity = Readonly<{
  id: string;
  label: string;
  kind: string;
  dimension: ResultOverlayDimension;
  attachment: ResultColourAttachment;
}>;

/**
 * The quantities result colour can show: the per-member row kinds the solve routes emit. A kind
 * absent from a result is not offered for it. No quantity is formed from other rows.
 */
export const RESULT_COLOUR_QUANTITIES: readonly ResultColourQuantity[] = Object.freeze([
  { id: "displacement", label: "Displacement magnitude", kind: "displacement_magnitude", dimension: "length", attachment: "end_nodes" },
  { id: "normal-stress-maximum", label: "Normal-stress maximum", kind: "pipe_elastic_normal_stress_maximum_v2", dimension: "stress", attachment: "member" },
  { id: "normal-stress-extreme", label: "Section normal-stress extreme |σ±|", kind: "element_local_normal_stress_extreme", dimension: "stress", attachment: "member" },
  { id: "intensified-bending", label: "Intensified bending stress", kind: "component_equal_factor_intensified_bending_stress_v1", dimension: "stress", attachment: "source_rows" },
  { id: "transverse-shear", label: "Transverse shear stress maximum", kind: "element_local_transverse_shear_stress_maximum", dimension: "stress", attachment: "member" },
  { id: "torsional-shear", label: "Torsional shear stress |τ|", kind: "element_local_torsional_shear_stress", dimension: "stress", attachment: "member" },
  { id: "axial-force", label: "Axial force |N|", kind: "element_local_axial_force", dimension: "force", attachment: "member" },
  { id: "shear-force-y", label: "Shear force |Vy|", kind: "element_local_shear_force_y", dimension: "force", attachment: "member" },
  { id: "shear-force-z", label: "Shear force |Vz|", kind: "element_local_shear_force_z", dimension: "force", attachment: "member" },
  { id: "torsional-moment", label: "Torsional moment |T|", kind: "element_local_torsional_moment", dimension: "moment", attachment: "member" },
  { id: "bending-moment-y", label: "Bending moment |My|", kind: "element_local_bending_moment_y", dimension: "moment", attachment: "member" },
  { id: "bending-moment-z", label: "Bending moment |Mz|", kind: "element_local_bending_moment_z", dimension: "moment", attachment: "member" }
].map((quantity) => Object.freeze(quantity as ResultColourQuantity)));

/** Range-envelope rows pick each value separately; they are never one solved state, so neither
 * overlay draws them (as the rotation glyphs do not). */
export const RANGE_ENVELOPE_BASIS = "explicit_user_range_envelope";

export type ResultOverlayCase = Readonly<{ key: string; basis: ResultBasisRef; label: string }>;

export function basisKey(basis: ResultBasisRef | undefined): string | null {
  return basis ? `${basis.ref_type}:${basis.ref_id}` : null;
}

function isDrawableRow(row: ResultRow): boolean {
  return Number.isFinite(row.value) && row.metadata?.basis !== RANGE_ENVELOPE_BASIS && basisKey(row.basis_ref) !== null;
}

/** The load cases and combinations the result's rows are stated for, in the order they first
 * appear: load cases first, then combinations. Range-envelope rows add no case. */
export function resultOverlayCases(model: PreviewModel, result: MechanicsResult | null): ResultOverlayCase[] {
  if (!result) return [];
  const labels = new Map(model.load_cases.map((loadCase) => [loadCase.id, loadCase.label] as const));
  const seen = new Map<string, ResultOverlayCase>();
  for (const row of result.results) {
    if (!isDrawableRow(row)) continue;
    const basis = row.basis_ref!;
    if (basis.ref_type !== "load_case" && basis.ref_type !== "combination") continue;
    const key = basisKey(basis)!;
    if (seen.has(key)) continue;
    const label = basis.ref_type === "load_case" && labels.get(basis.ref_id)
      ? `${labels.get(basis.ref_id)} (${basis.ref_id})`
      : basis.ref_id;
    seen.set(key, Object.freeze({ key, basis: Object.freeze({ ref_type: basis.ref_type, ref_id: basis.ref_id }), label }));
  }
  const cases = [...seen.values()];
  return [...cases.filter((item) => item.basis.ref_type === "load_case"), ...cases.filter((item) => item.basis.ref_type === "combination")];
}

type MemberIndex = Readonly<{
  pipeIds: ReadonlySet<string>;
  pipesAtNode: ReadonlyMap<string, readonly string[]>;
  rowsById: ReadonlyMap<string, ResultRow>;
}>;

function memberIndex(model: PreviewModel, result: MechanicsResult): MemberIndex {
  const pipesAtNode = new Map<string, string[]>();
  for (const pipe of model.pipe_segments) {
    for (const node of new Set([pipe.from, pipe.to])) pipesAtNode.set(node, [...(pipesAtNode.get(node) ?? []), pipe.id]);
  }
  return {
    pipeIds: new Set(model.pipe_segments.map((pipe) => pipe.id)),
    pipesAtNode,
    rowsById: new Map(result.results.map((row) => [row.id, row] as const))
  };
}

/** The members a row's value belongs to under the quantity's attachment. */
function membersOf(row: ResultRow, attachment: ResultColourAttachment, index: MemberIndex): string[] {
  if (attachment === "member") return index.pipeIds.has(row.entity_ref) ? [row.entity_ref] : [];
  if (attachment === "end_nodes") return [...(index.pipesAtNode.get(row.entity_ref) ?? [])];
  const members = new Set<string>();
  for (const ref of row.source_result_refs ?? []) {
    const source = index.rowsById.get(ref);
    if (source && index.pipeIds.has(source.entity_ref)) members.add(source.entity_ref);
  }
  return [...members];
}

/** The quantities with at least one finite, drawable row attached to a member of the model. */
export function availableResultColourQuantities(model: PreviewModel, result: MechanicsResult | null): ResultColourQuantity[] {
  if (!result) return [];
  const index = memberIndex(model, result);
  return RESULT_COLOUR_QUANTITIES.filter((quantity) =>
    result.results.some((row) => row.kind === quantity.kind && isDrawableRow(row) && membersOf(row, quantity.attachment, index).length > 0));
}

export type ResultColourMember = Readonly<{
  /** The largest absolute value over the member's rows, in the overlay's unit. */
  value: number;
  /** The row that gives it. */
  rowId: string;
  /** Position on the scale, 0 to 1: value / maximum (design system 2.6: the range is 0 to the maximum). */
  position: number;
}>;

export type ResultColourOverlay = Readonly<{
  state: "available" | "held" | "unavailable";
  /** Why there is no colour: for "unavailable", or "held" (the standing withholds every row). */
  reason: string | null;
  quantity: ResultColourQuantity | null;
  case: ResultOverlayCase | null;
  unit: string | null;
  members: ReadonlyMap<string, ResultColourMember>;
  /** Members of the model. */
  memberCount: number;
  /** Members with no colourable value, drawn as such and counted in the legend. */
  noValueCount: number;
  /** Rows of the quantity and case that the numerical standing withholds. */
  heldRowCount: number;
  minimum: number | null;
  maximum: number | null;
}>;

function unavailable(model: PreviewModel, reason: string, quantity: ResultColourQuantity | null, overlayCase: ResultOverlayCase | null): ResultColourOverlay {
  return Object.freeze({
    state: "unavailable", reason, quantity, case: overlayCase, unit: null, members: new Map(),
    memberCount: model.pipe_segments.length, noValueCount: model.pipe_segments.length, heldRowCount: 0, minimum: null, maximum: null
  });
}

export const RESULT_COLOUR_HELD_REASON = "No result colour on the current model";

/**
 * The colour of each member for one quantity and one case. A row the numerical standing withholds
 * (`overlayMayShowRow`) never colours; a member without a shown row has no value. Rows of one
 * quantity and case in more than one unit are not compared: the overlay is then unavailable.
 */
export function buildResultColourOverlay(input: {
  model: PreviewModel;
  result: MechanicsResult | null;
  standing: ResultOverlayStanding | null | undefined;
  quantity: ResultColourQuantity | null;
  overlayCase: ResultOverlayCase | null;
}): ResultColourOverlay {
  const { model, result, standing, quantity, overlayCase } = input;
  if (!result) return unavailable(model, "No result", quantity, overlayCase);
  if (result.status.mechanics !== "MECHANICS_SOLVED") return unavailable(model, "Mechanics not solved", quantity, overlayCase);
  if (!quantity) return unavailable(model, "No member result rows for a colour scale", quantity, overlayCase);
  if (!overlayCase) return unavailable(model, "No load case or combination with result rows", quantity, overlayCase);
  const index = memberIndex(model, result);
  const shown: Array<{ row: ResultRow; members: string[] }> = [];
  let heldRowCount = 0;
  for (const row of result.results) {
    if (row.kind !== quantity.kind || !isDrawableRow(row) || basisKey(row.basis_ref) !== overlayCase.key) continue;
    const members = membersOf(row, quantity.attachment, index);
    if (members.length === 0) continue;
    if (!overlayMayShowRow(standing, row)) {
      heldRowCount += 1;
      continue;
    }
    shown.push({ row, members });
  }
  const memberCount = model.pipe_segments.length;
  if (shown.length === 0) {
    return Object.freeze({
      state: heldRowCount > 0 ? "held" : "unavailable",
      reason: heldRowCount > 0 ? RESULT_COLOUR_HELD_REASON : `No ${quantity.label.toLowerCase()} rows for ${overlayCase.label}`,
      quantity, case: overlayCase, unit: null, members: new Map(), memberCount, noValueCount: memberCount, heldRowCount, minimum: null, maximum: null
    });
  }
  const units = [...new Set(shown.map(({ row }) => row.unit))].sort();
  if (units.length !== 1) return unavailable(model, `Units mixed (${units.join(", ")})`, quantity, overlayCase);
  const largest = new Map<string, { value: number; rowId: string }>();
  for (const { row, members } of shown) {
    const magnitude = Math.abs(row.value);
    for (const member of members) {
      const current = largest.get(member);
      if (!current || magnitude > current.value) largest.set(member, { value: magnitude, rowId: row.id });
    }
  }
  const values = [...largest.values()].map((item) => item.value);
  const maximum = Math.max(...values);
  const minimum = Math.min(...values);
  const members = new Map([...largest].map(([member, item]) => [member, Object.freeze({
    value: item.value,
    rowId: item.rowId,
    position: maximum > 0 ? item.value / maximum : 0
  })] as const));
  return Object.freeze({
    state: "available", reason: null, quantity, case: overlayCase, unit: units[0], members, memberCount,
    noValueCount: model.pipe_segments.filter((pipe) => !members.has(pipe.id)).length,
    heldRowCount, minimum, maximum
  });
}

/** The scale's five ticks (design system 2.6): 0, a quarter, a half, three quarters and the maximum. */
export function resultColourTicks(overlay: ResultColourOverlay): number[] {
  return overlay.maximum === null ? [] : [0, 0.25, 0.5, 0.75, 1].map((fraction) => fraction * overlay.maximum!);
}

/**
 * The per-member colours the viewport draws, for both themes: a member with a value takes its
 * position on the result scale, every other member the no-value colour. Null unless the overlay is
 * available, so a held or unavailable overlay leaves the figure neutral.
 */
export function resultColourTable(model: PreviewModel, overlay: ResultColourOverlay): ResultColourTable | null {
  if (overlay.state !== "available") return null;
  const noValue = Object.freeze({ light: resultNoValueHex("light"), dark: resultNoValueHex("dark") });
  return new Map(model.pipe_segments.map((pipe) => {
    const member = overlay.members.get(pipe.id);
    const colour = member
      ? Object.freeze({ light: resultScaleHex("light", member.position), dark: resultScaleHex("dark", member.position) })
      : noValue;
    return [entityKey({ type: "pipe", id: pipe.id }), colour] as const;
  }));
}
