// Result colour on the viewport's tubes (FR-GUI-001; design system 2.6 and 6.7; owner ruling
// OD-5; survey item A4, DEL-07-05 Design `result-contour.md`). For one load case or combination,
// either one scalar per member (the largest absolute value of one emitted row kind over that
// member's rows: its stations, or its end nodes for a nodal quantity), or, along members, the
// largest absolute value at each result station the rows state. A user-rule ratio is read from
// the session's rule-check outcomes as the stress and ratio table forms it. A magnitude display of
// emitted rows and rule outcomes only: it computes no engineering quantity, sets no limit and makes
// no professional claim. Colour is view state; nothing here writes to the model, the result or any
// export.

import type { MechanicsResult, PreviewModel, ResultBasisRef } from "../../types";
import { overlayMayShowRow, type ResultOverlayStanding } from "../results/numericalHealth";
import type { StressRatioGroup, StressRatioTable } from "../results/stressRatioTable";
import { entityKey } from "../workspace/selectionState";
import { resultNoValueHex, resultScaleHex } from "./viewportPalette";
import type { ResultColourTable } from "./viewportResource";

type ResultRow = MechanicsResult["results"][number];

/** "ratio" is a user-rule ratio: a pure number, shown without a unit and never converted. */
export type ResultOverlayDimension = "length" | "stress" | "force" | "moment" | "ratio";

/** How a quantity's rows reach a member: rows of the member itself, rows of its two end nodes,
 * or rows of another entity whose emitted source rows are the member's. */
export type ResultColourAttachment = "member" | "end_nodes" | "source_rows";

export type ResultColourQuantity = Readonly<{
  id: string;
  label: string;
  /** The emitted row kind, or the rule pack's stress category id for a rule ratio. */
  kind: string;
  dimension: ResultOverlayDimension;
  attachment: ResultColourAttachment;
  /** Emitted rows (absent: rows), or the session rule-check run's outcomes of one stress category. */
  source?: "rows" | "rule_ratio";
  /** Along members: true when each row states one signed value. Adjacent stations whose section
   * signs are opposite, or cannot be established, are not blended (`signBreak`). Absent: an
   * unsigned magnitude, or an extremum whose governing component may switch, whose sign is never
   * inferred. */
  signed?: boolean;
}>;

/** How values reach the tube: one value per member, or along each member from its stated stations. */
export type ResultColourResolution = "member" | "station";

/**
 * The product's fixed result-station grid: the `metadata.location` tokens the solve routes state
 * for a member's stations, as fractions of the member from its From node (end_i) to its To node
 * (end_j). Any other location (a governing station, a node, a summary) states no position.
 */
export const RESULT_STATION_FRACTIONS: Readonly<Record<string, number>> = Object.freeze({
  end_i: 0,
  quarter_1: 0.25,
  midspan: 0.5,
  quarter_3: 0.75,
  end_j: 1
});

export function stationFraction(location: string | null | undefined): number | null {
  return typeof location === "string" && Object.hasOwn(RESULT_STATION_FRACTIONS, location) ? RESULT_STATION_FRACTIONS[location] : null;
}

/** The producer's statement that an end row is the node-on-element action at its end. */
const I_END_FORCE_SIGN = "positive value follows the element-local DOF at the i-end force vector";
const J_END_FORCE_SIGN = "positive value follows the element-local DOF at the j-end force vector";
/** The curved-bend section convention, stated without the words "j-side"; the producer forms its
 * section resultants as the j-side cut action (`core/product_physics/src/lib.rs`, curved station
 * resultants: "Negate all components together to express the j-side cut action"). */
const CURVED_BEND_SECTION_SIGN =
  "local x is endpoint arc tangent toward j; local z is bend-plane normal; local y is z cross x toward arc center; resultants come from section equilibrium over assembled end actions";

/**
 * The factor that expresses a signed row's value in the section convention (the j-side section
 * action), from the sign convention and location the row states, or null when it cannot be
 * converted exactly. Force and moment end rows are node-on-element end actions: the i-end value is
 * negated and the j-end value kept. A row that already states the j-side section action (the
 * station rows, and the endpoint torsional shear stress) is kept. Anything else is not converted.
 */
export function sectionSignFactor(signConvention: string | null | undefined, location: string | null | undefined): 1 | -1 | null {
  if (typeof signConvention !== "string") return null;
  if (signConvention.startsWith(I_END_FORCE_SIGN)) return location === "end_i" ? -1 : null;
  if (signConvention.startsWith(J_END_FORCE_SIGN)) return location === "end_j" ? 1 : null;
  if (signConvention === CURVED_BEND_SECTION_SIGN) return 1;
  if (signConvention.includes("j-side section action") && !signConvention.includes("minus")) return 1;
  return null;
}

/**
 * Whether the contour must not blend between two adjacent stated stations: for a signed quantity,
 * when their section signs are opposite (both non-zero), or when either station's section sign
 * cannot be established. An unsigned quantity always blends.
 */
export function signBreak(
  a: Readonly<{ signed?: boolean; sign?: number | null }>,
  b: Readonly<{ signed?: boolean; sign?: number | null }>
): boolean {
  if (!a.signed && !b.signed) return false;
  if (typeof a.sign !== "number" || typeof b.sign !== "number") return true;
  return a.sign * b.sign < 0;
}

/** The ratio quantity id of a rule-check stress category group. */
export const RULE_RATIO_QUANTITY_PREFIX = "rule-ratio:";

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
  { id: "torsional-shear", label: "Torsional shear stress |τ|", kind: "element_local_torsional_shear_stress", dimension: "stress", attachment: "member", signed: true },
  { id: "axial-force", label: "Axial force |N|", kind: "element_local_axial_force", dimension: "force", attachment: "member", signed: true },
  { id: "shear-force-y", label: "Shear force |Vy|", kind: "element_local_shear_force_y", dimension: "force", attachment: "member", signed: true },
  { id: "shear-force-z", label: "Shear force |Vz|", kind: "element_local_shear_force_z", dimension: "force", attachment: "member", signed: true },
  { id: "torsional-moment", label: "Torsional moment |T|", kind: "element_local_torsional_moment", dimension: "moment", attachment: "member", signed: true },
  { id: "bending-moment-y", label: "Bending moment |My|", kind: "element_local_bending_moment_y", dimension: "moment", attachment: "member", signed: true },
  { id: "bending-moment-z", label: "Bending moment |Mz|", kind: "element_local_bending_moment_z", dimension: "moment", attachment: "member", signed: true }
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

/** The pipe a rule outcome's locations name, and its station, or null when they name no single pipe. */
function ruleOutcomePlace(row: StressRatioGroup["rows"][number], pipeIds: ReadonlySet<string>): { member: string; fraction: number | null } | null {
  const entities = new Set(row.locations.map((location) => location.entityRef));
  if (entities.size !== 1) return null;
  const member = [...entities][0];
  if (!pipeIds.has(member)) return null;
  const fractions = new Set(row.locations.map((location) => stationFraction(location.station)));
  return { member, fraction: fractions.size === 1 ? [...fractions][0] : null };
}

/** The ratio quantities of the session's rule-check run: one per stress category of the user's
 * rule pack with at least one outcome located on a single pipe of the model. None without a run. */
export function ruleRatioColourQuantities(model: PreviewModel, table: StressRatioTable | null | undefined): ResultColourQuantity[] {
  if (!table || table.run.state !== "used") return [];
  const pipeIds = new Set(model.pipe_segments.map((pipe) => pipe.id));
  return table.groups
    .filter((group) => group.source === "rule_category" && group.rows.some((row) => ruleOutcomePlace(row, pipeIds) !== null))
    .map((group) => Object.freeze({
      id: `${RULE_RATIO_QUANTITY_PREFIX}${group.id}`,
      label: `Rule ratio · ${group.label}`,
      kind: group.ref,
      dimension: "ratio" as const,
      attachment: "member" as const,
      source: "rule_ratio" as const
    }));
}

/** The quantities with at least one finite, drawable row attached to a member of the model, then
 * the rule-check run's ratio quantities. */
export function availableResultColourQuantities(
  model: PreviewModel,
  result: MechanicsResult | null,
  ratioTable?: StressRatioTable | null
): ResultColourQuantity[] {
  if (!result) return [];
  const index = memberIndex(model, result);
  return [
    ...RESULT_COLOUR_QUANTITIES.filter((quantity) =>
      result.results.some((row) => row.kind === quantity.kind && isDrawableRow(row) && membersOf(row, quantity.attachment, index).length > 0)),
    ...ruleRatioColourQuantities(model, ratioTable)
  ];
}

/** One stated station of a member: its fraction from the From node, and its value in the overlay's
 * unit, or null when the station has no colourable value (a row the numerical standing withholds,
 * a non-finite row, or a rule outcome that is withheld, incomplete or forms no ratio). */
export type ResultColourStation = Readonly<{
  fraction: number;
  value: number | null;
  /** The row that gives the station's value (its largest magnitude), or null without a value. */
  rowId: string | null;
  rowIds: readonly string[];
  /** Whether the quantity is signed (`ResultColourQuantity.signed`). */
  signed: boolean;
  /** For a signed quantity: the section sign (`sectionSignFactor`) its non-zero rows share, 0 when
   * every row is zero, and null when the non-zero rows disagree or any row's convention cannot be
   * converted. Null for an unsigned quantity. */
  sign: -1 | 0 | 1 | null;
}>;

export type ResultColourMember = Readonly<{
  /** The largest absolute value over the member's rows, in the overlay's unit; along members, the
   * largest of its stations' values when it has stations. */
  value: number;
  /** The row that gives it. */
  rowId: string;
  /** Position on the scale, 0 to 1: value / maximum (design system 2.6: the range is 0 to the maximum). */
  position: number;
  /** Along members: the member's stated stations in order, two or more. Empty for one value per
   * member, and for a member with fewer than two stated stations (it is shown as one value). */
  stations: readonly ResultColourStation[];
}>;

export type ResultColourOverlay = Readonly<{
  state: "available" | "held" | "unavailable";
  /** Why there is no colour: for "unavailable", or "held" (the standing withholds every row). */
  reason: string | null;
  quantity: ResultColourQuantity | null;
  case: ResultOverlayCase | null;
  resolution: ResultColourResolution;
  unit: string | null;
  members: ReadonlyMap<string, ResultColourMember>;
  /** Members of the model. */
  memberCount: number;
  /** Members with no colourable value, drawn as such and counted in the legend. */
  noValueCount: number;
  /** Rows (or rule outcomes) of the quantity and case that the numerical standing withholds. */
  heldRowCount: number;
  /** Along members: members drawn from their stations. */
  stationMemberCount: number;
  /** Along members: stated stations with no colourable value; the tube is not coloured beside them. */
  gapStationCount: number;
  /** Along members: intervals of a signed quantity not blended (`signBreak`): opposite section
   * signs, or a sign that cannot be established. */
  signGapIntervalCount: number;
  /** Along members: shown rows of a member drawn from its stations that state no station; not drawn. */
  unplacedRowCount: number;
  /** Rule ratio: outcomes that are incomplete or form no ratio, so colour nothing. */
  noRatioCount: number;
  minimum: number | null;
  maximum: number | null;
}>;

function emptyOverlay(
  model: PreviewModel,
  state: "held" | "unavailable",
  reason: string,
  quantity: ResultColourQuantity | null,
  overlayCase: ResultOverlayCase | null,
  resolution: ResultColourResolution,
  counts: { heldRowCount?: number; noRatioCount?: number } = {}
): ResultColourOverlay {
  return Object.freeze({
    state, reason, quantity, case: overlayCase, resolution, unit: null, members: new Map(),
    memberCount: model.pipe_segments.length, noValueCount: model.pipe_segments.length,
    heldRowCount: counts.heldRowCount ?? 0, stationMemberCount: 0, gapStationCount: 0, signGapIntervalCount: 0, unplacedRowCount: 0,
    noRatioCount: counts.noRatioCount ?? 0, minimum: null, maximum: null
  });
}

export const RESULT_COLOUR_HELD_REASON = "No result colour on the current model";

/** One value reaching one member: from a row, or from a rule outcome. "gap" is a value that cannot
 * colour (non-finite, or an outcome with no ratio); "held" is withheld by the numerical standing. */
type Contribution = Readonly<{
  id: string;
  member: string;
  fraction: number | null;
  value: number;
  unit: string;
  state: "shown" | "held" | "gap";
  signConvention: string | null;
  location: string | null;
}>;

/** The fraction of a row's value along a member under the quantity's attachment, or null. */
function rowFraction(row: ResultRow, member: string, attachment: ResultColourAttachment, model: PreviewModel, members: readonly string[]): number | null {
  if (attachment === "end_nodes") {
    const pipe = model.pipe_segments.find((item) => item.id === member);
    if (!pipe || pipe.from === pipe.to) return null;
    return row.entity_ref === pipe.from ? 0 : row.entity_ref === pipe.to ? 1 : null;
  }
  if (attachment === "source_rows" && members.length !== 1) return null;
  return stationFraction(row.metadata?.location);
}

function rowContributions(
  model: PreviewModel,
  result: MechanicsResult,
  standing: ResultOverlayStanding | null | undefined,
  quantity: ResultColourQuantity,
  overlayCase: ResultOverlayCase,
  resolution: ResultColourResolution
): { contributions: Contribution[]; heldRowCount: number } {
  const index = memberIndex(model, result);
  const contributions: Contribution[] = [];
  let heldRowCount = 0;
  for (const row of result.results) {
    if (row.kind !== quantity.kind || row.metadata?.basis === RANGE_ENVELOPE_BASIS || basisKey(row.basis_ref) !== overlayCase.key) continue;
    const finite = Number.isFinite(row.value);
    // One value per member reads finite rows only, as it always has; along members a non-finite
    // row at a stated station leaves that station without a value.
    if (!finite && resolution === "member") continue;
    const members = membersOf(row, quantity.attachment, index);
    if (members.length === 0) continue;
    const held = finite && !overlayMayShowRow(standing, row);
    if (held) heldRowCount += 1;
    const state = held ? "held" : finite ? "shown" : "gap";
    for (const member of members) {
      contributions.push({
        id: row.id, member, fraction: rowFraction(row, member, quantity.attachment, model, members), value: row.value, unit: row.unit, state,
        signConvention: row.metadata?.sign_convention ?? null, location: row.metadata?.location ?? null
      });
    }
  }
  return { contributions, heldRowCount };
}

function ruleContributions(
  model: PreviewModel,
  group: StressRatioGroup,
  overlayCase: ResultOverlayCase
): { contributions: Contribution[]; heldRowCount: number; noRatioCount: number } {
  const pipeIds = new Set(model.pipe_segments.map((pipe) => pipe.id));
  const contributions: Contribution[] = [];
  let heldRowCount = 0;
  let noRatioCount = 0;
  for (const row of group.rows) {
    if (!row.case || row.case.envelope || basisKey(row.case.basis) !== overlayCase.key) continue;
    const place = ruleOutcomePlace(row, pipeIds);
    if (!place) continue;
    const ratio = row.outcome?.ratio ?? null;
    const held = row.standing !== "admitted";
    const hasRatio = !held && ratio !== null && Number.isFinite(ratio);
    if (held) heldRowCount += 1;
    else if (!hasRatio) noRatioCount += 1;
    contributions.push({
      id: row.key, member: place.member, fraction: place.fraction, value: hasRatio ? ratio! : Number.NaN, unit: "",
      state: held ? "held" : hasRatio ? "shown" : "gap", signConvention: null, location: null
    });
  }
  return { contributions, heldRowCount, noRatioCount };
}

type Largest = { value: number; rowId: string };

function largestOf(items: readonly Contribution[]): Largest | null {
  let best: Largest | null = null;
  for (const item of items) {
    const magnitude = Math.abs(item.value);
    if (!best || magnitude > best.value) best = { value: magnitude, rowId: item.id };
  }
  return best;
}

/** A member's stated stations: the largest magnitude at each fraction, or null where any value
 * stated there cannot colour. */
/** A signed station's section sign: the sign its non-zero rows share once each is expressed in the
 * section convention, 0 when every row is zero, null when they disagree or any row cannot be
 * converted. */
function stationSign(at: readonly Contribution[]): -1 | 0 | 1 | null {
  const signs = new Set<number>();
  for (const item of at) {
    const factor = sectionSignFactor(item.signConvention, item.location);
    if (factor === null) return null;
    const sign = Math.sign(item.value * factor);
    if (sign !== 0) signs.add(sign);
  }
  if (signs.size > 1) return null;
  return signs.size === 0 ? 0 : ([...signs][0] as -1 | 1);
}

function memberStations(items: readonly Contribution[], signed: boolean): { stations: ResultColourStation[]; largest: Largest | null } {
  const byFraction = new Map<number, Contribution[]>();
  for (const item of items) {
    if (item.fraction === null) continue;
    byFraction.set(item.fraction, [...(byFraction.get(item.fraction) ?? []), item]);
  }
  let largest: Largest | null = null;
  const stations = [...byFraction].sort(([a], [b]) => a - b).map(([fraction, at]) => {
    const colourable = at.every((item) => item.state === "shown");
    const best = colourable ? largestOf(at) : null;
    if (best && (!largest || best.value > largest.value)) largest = best;
    return Object.freeze({
      fraction, value: best ? best.value : null, rowId: best ? best.rowId : null, rowIds: Object.freeze(at.map((item) => item.id)),
      signed, sign: signed && best ? stationSign(at) : null
    });
  });
  return { stations, largest };
}

/**
 * The colour of each member for one quantity and one case. A row the numerical standing withholds
 * (`overlayMayShowRow`) never colours; a member without a shown row has no value. Rows of one
 * quantity and case in more than one unit are not compared: the overlay is then unavailable.
 *
 * Along members (`resolution: "station"`), a member whose rows state two or more stations of the
 * station grid is drawn from those stations only: each station's value is the largest magnitude
 * stated there, and a station where any stated value is withheld or not finite has none. A member
 * with fewer than two stated stations keeps one value, as with one value per member.
 */
export function buildResultColourOverlay(input: {
  model: PreviewModel;
  result: MechanicsResult | null;
  standing: ResultOverlayStanding | null | undefined;
  quantity: ResultColourQuantity | null;
  overlayCase: ResultOverlayCase | null;
  resolution?: ResultColourResolution;
  /** The stress and ratio table of the result and the session's rule-check run, for a rule ratio. */
  ratioTable?: StressRatioTable | null;
}): ResultColourOverlay {
  const { model, result, standing, quantity, overlayCase, ratioTable = null } = input;
  const resolution = input.resolution ?? "member";
  const empty = (reason: string, state: "held" | "unavailable" = "unavailable", counts = {}) =>
    emptyOverlay(model, state, reason, quantity, overlayCase, resolution, counts);
  if (!result) return empty("No result");
  if (result.status.mechanics !== "MECHANICS_SOLVED") return empty("Mechanics not solved");
  if (!quantity) return empty("No member result rows for a colour scale");
  if (!overlayCase) return empty("No load case or combination with result rows");
  let gathered: { contributions: Contribution[]; heldRowCount: number; noRatioCount?: number };
  if (quantity.source === "rule_ratio") {
    const group = ratioTable?.run.state === "used"
      ? ratioTable.groups.find((item) => item.source === "rule_category" && `${RULE_RATIO_QUANTITY_PREFIX}${item.id}` === quantity.id)
      : undefined;
    if (!group) return empty("No rule-check run of this result for the ratio");
    gathered = ruleContributions(model, group, overlayCase);
  } else {
    gathered = rowContributions(model, result, standing, quantity, overlayCase, resolution);
  }
  const { contributions, heldRowCount } = gathered;
  const noRatioCount = gathered.noRatioCount ?? 0;
  const shown = contributions.filter((item) => item.state === "shown");
  if (shown.length === 0) {
    return heldRowCount > 0
      ? empty(RESULT_COLOUR_HELD_REASON, "held", { heldRowCount, noRatioCount })
      : empty(`No ${quantity.label.toLowerCase()} ${quantity.source === "rule_ratio" ? "ratios" : "rows"} for ${overlayCase.label}`, "unavailable", { noRatioCount });
  }
  const units = [...new Set(shown.map((item) => item.unit))].sort();
  if (units.length !== 1) return empty(`Units mixed (${units.join(", ")})`);
  const byMember = new Map<string, Contribution[]>();
  for (const item of contributions) byMember.set(item.member, [...(byMember.get(item.member) ?? []), item]);
  const drawn = new Map<string, { largest: Largest; stations: ResultColourStation[] }>();
  let gapStationCount = 0;
  let signGapIntervalCount = 0;
  let unplacedRowCount = 0;
  for (const [member, items] of byMember) {
    if (resolution === "station") {
      const { stations, largest } = memberStations(items, quantity.signed === true);
      if (stations.length >= 2) {
        gapStationCount += stations.filter((station) => station.value === null).length;
        signGapIntervalCount += stations.filter((station, index) => index > 0 && station.value !== null && stations[index - 1].value !== null &&
          signBreak(stations[index - 1], station)).length;
        unplacedRowCount += items.filter((item) => item.fraction === null && item.state === "shown").length;
        // A member none of whose stations can colour has no value, as any other member without one.
        if (largest) drawn.set(member, { largest, stations });
        continue;
      }
    }
    const largest = largestOf(items.filter((item) => item.state === "shown"));
    if (largest) drawn.set(member, { largest, stations: [] });
  }
  if (drawn.size === 0) {
    return heldRowCount > 0
      ? empty(RESULT_COLOUR_HELD_REASON, "held", { heldRowCount, noRatioCount })
      : empty(`No colourable ${quantity.label.toLowerCase()} values for ${overlayCase.label}`, "unavailable", { noRatioCount });
  }
  const values = [...drawn.values()].map((item) => item.largest.value);
  const maximum = Math.max(...values);
  const minimum = Math.min(...values);
  const members = new Map([...drawn].map(([member, item]) => [member, Object.freeze({
    value: item.largest.value,
    rowId: item.largest.rowId,
    position: maximum > 0 ? item.largest.value / maximum : 0,
    stations: Object.freeze(item.stations)
  })] as const));
  return Object.freeze({
    state: "available", reason: null, quantity, case: overlayCase, resolution, unit: units[0], members,
    memberCount: model.pipe_segments.length,
    noValueCount: model.pipe_segments.filter((pipe) => !members.has(pipe.id)).length,
    heldRowCount,
    stationMemberCount: [...members.values()].filter((member) => member.stations.length >= 2).length,
    gapStationCount, signGapIntervalCount, unplacedRowCount, noRatioCount, minimum, maximum
  });
}

/** The scale's five ticks (design system 2.6): 0, a quarter, a half, three quarters and the maximum. */
export function resultColourTicks(overlay: ResultColourOverlay): number[] {
  return overlay.maximum === null ? [] : [0, 0.25, 0.5, 0.75, 1].map((fraction) => fraction * overlay.maximum!);
}

/** How the overlay is shown: colours on the tubes, or numbers at the stated values in place of colours. */
export type ResultColourDisplay = "colour" | "numbers";

/** Whether a value is coloured under a threshold: strictly above it, or always when there is none. */
export function aboveThreshold(value: number, threshold: number | null | undefined): boolean {
  return threshold === null || threshold === undefined || value > threshold;
}

/**
 * The per-member colours the viewport draws, for both themes: a member with a value takes its
 * position on the result scale, every other member the no-value colour. Under a threshold a member
 * whose value is not above it is left out of the table, so it draws its plain pipe colour. Along
 * members, a member drawn from its stations has the no-value colour on its own tube, beneath and
 * beyond its sleeve. Null
 * unless the overlay is available and shown as colours, so a held or unavailable overlay, or the
 * numbers display, leaves the figure neutral.
 */
export function resultColourTable(
  model: PreviewModel,
  overlay: ResultColourOverlay,
  options: { threshold?: number | null; display?: ResultColourDisplay } = {}
): ResultColourTable | null {
  if (overlay.state !== "available" || options.display === "numbers") return null;
  const noValue = Object.freeze({ light: resultNoValueHex("light"), dark: resultNoValueHex("dark") });
  return new Map(model.pipe_segments.flatMap((pipe) => {
    const member = overlay.members.get(pipe.id);
    // Along members, a member drawn from its stations takes the no-value colour on its own tube:
    // the sleeve shows the stated span, and nothing outside it is coloured.
    if (member && member.stations.length >= 2) return [[entityKey({ type: "pipe", id: pipe.id }), noValue] as const];
    if (member && !aboveThreshold(member.value, options.threshold)) return [];
    const colour = member
      ? Object.freeze({ light: resultScaleHex("light", member.position), dark: resultScaleHex("dark", member.position) })
      : noValue;
    return [[entityKey({ type: "pipe", id: pipe.id }), colour] as const];
  }));
}
