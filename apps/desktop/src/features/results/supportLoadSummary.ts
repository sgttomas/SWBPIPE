/** Per-support load summary across every solved case and combination (survey item A2;
 * DEL-07-05 Design `support-load-summary.md`).
 *
 * For each support of the model: one row per load case and combination the result states rows
 * for, with the support's emitted reaction components (Fx, Fy, Fz, Mx, My, Mz), its emitted
 * force and moment magnitude rows (|F|, |M|) and the emitted translations and rotations of its
 * node (ux, uy, uz, rx, ry, rz), all in the global frame; then the largest and smallest value of
 * each column over the rows that are one solved state, each naming the first row (in row order)
 * that holds it.
 *
 * Pure post-processing of emitted rows: nothing here forms a new engineering quantity. A
 * resultant is the emitted magnitude row or is absent; it is never computed from components.
 * Extremes are arithmetic over the listed values, not a qualification or a limit. A row the
 * numerical standing withholds (the viewport overlays' rule, `overlayMayShowRow`) carries no
 * values and says why; it and every row that is not one solved state stay out of the extremes,
 * and the summary counts them. Stored values and units are kept unchanged for an export (D2). */

import type { MechanicsResult, PreviewModel, ResultBasisRef } from "../../types";
import {
  REACTION_COMPONENT_KIND,
  REACTION_DRAWN_FRAME,
  REACTION_FORCE_MAGNITUDE_KIND,
  REACTION_MOMENT_MAGNITUDE_KIND,
  withheldSupportAttributions
} from "../viewport/reactionOverlay";
import { RANGE_ENVELOPE_BASIS, basisKey } from "../viewport/resultColourOverlay";
import { overlayMayShowRow, rowHealthState, type ResultOverlayStanding } from "./numericalHealth";
import { NUMERICAL_HEALTH_STATE_TEXT } from "./numericalHealthText";

type ResultRow = MechanicsResult["results"][number];
type Support = PreviewModel["supports"][number];

export type SupportLoadColumnId =
  | "Fx" | "Fy" | "Fz" | "F" | "Mx" | "My" | "Mz" | "M"
  | "ux" | "uy" | "uz" | "rx" | "ry" | "rz";
export type SupportLoadColumnGroup = "force" | "moment" | "translation" | "rotation";
export type SupportLoadColumn = Readonly<{
  id: SupportLoadColumnId;
  label: string;
  group: SupportLoadColumnGroup;
  /** The display-units dimension of the column. */
  dimension: "force" | "moment" | "displacement" | "rotation";
  /** The emitted row kind, and for reaction components the row's `metadata.component`. */
  kind: string;
  component?: string;
  /** Rows of the support itself, or of the support's node. */
  of: "support" | "node";
}>;

const column = (id: SupportLoadColumnId, label: string, group: SupportLoadColumnGroup, kind: string, component?: string): SupportLoadColumn =>
  Object.freeze({
    id, label, group, kind, ...(component ? { component } : {}),
    dimension: group === "translation" ? "displacement" : group,
    of: group === "translation" || group === "rotation" ? "node" : "support"
  } as SupportLoadColumn);

/** The table's columns, in display order. */
export const SUPPORT_LOAD_COLUMNS: readonly SupportLoadColumn[] = Object.freeze([
  column("Fx", "Fx", "force", REACTION_COMPONENT_KIND, "Fx"),
  column("Fy", "Fy", "force", REACTION_COMPONENT_KIND, "Fy"),
  column("Fz", "Fz", "force", REACTION_COMPONENT_KIND, "Fz"),
  column("F", "|F|", "force", REACTION_FORCE_MAGNITUDE_KIND),
  column("Mx", "Mx", "moment", REACTION_COMPONENT_KIND, "Mx"),
  column("My", "My", "moment", REACTION_COMPONENT_KIND, "My"),
  column("Mz", "Mz", "moment", REACTION_COMPONENT_KIND, "Mz"),
  column("M", "|M|", "moment", REACTION_MOMENT_MAGNITUDE_KIND),
  column("ux", "ux", "translation", "global_nodal_displacement_x"),
  column("uy", "uy", "translation", "global_nodal_displacement_y"),
  column("uz", "uz", "translation", "global_nodal_displacement_z"),
  column("rx", "rx", "rotation", "global_nodal_rotation_x"),
  column("ry", "ry", "rotation", "global_nodal_rotation_y"),
  column("rz", "rz", "rotation", "global_nodal_rotation_z")
]);

/** The only frame tabulated, as the reaction arrows draw: the model's global axes. */
export const SUPPORT_LOAD_FRAME = REACTION_DRAWN_FRAME;

export type SupportLoadCase = Readonly<{
  key: string;
  basis: ResultBasisRef;
  label: string;
  /** A range-envelope combination: each value is selected separately from its operands. */
  envelope: boolean;
}>;

/** One emitted value: the stored value and unit, unchanged, and the row it comes from. */
export type SupportLoadValue = Readonly<{ value: number; unit: string; resultId: string }>;

/** Why a row's reactions are not listed (its node's movements may still be). */
export type SupportReactionAbsence =
  /** The solver withheld this support's attribution for the load case. */
  | "attribution_withheld"
  /** No reaction rows of this support for the case. */
  | "no_rows"
  /** The rows state a frame other than global; they are not tabulated. */
  | "frame_not_global";

export type SupportLoadRow = Readonly<{
  case: SupportLoadCase;
  /** `withheld`: the numerical standing withholds the case; no values are given. */
  standing: "admitted" | "withheld";
  /** The reason text, for a withheld row. */
  withheldReason: string | null;
  /** Null when reaction values are listed. */
  reactionAbsence: SupportReactionAbsence | null;
  /** The producer's reason code (`attribution_withheld`) or the stated frame (`frame_not_global`). */
  reactionAbsenceDetail: string | null;
  values: Readonly<Partial<Record<SupportLoadColumnId, SupportLoadValue>>>;
  /** Whether the row is one solved state that the extremes are taken over. */
  inExtremes: boolean;
}>;

export type SupportLoadExtremeValue = Readonly<SupportLoadValue & { caseKey: string; caseLabel: string }>;
export type SupportLoadExtreme = Readonly<{
  column: SupportLoadColumnId;
  max: SupportLoadExtremeValue | null;
  min: SupportLoadExtremeValue | null;
  /** Why no extreme is formed although values exist (the units differ). */
  note: string | null;
}>;

/** Rows listed but not in the extremes, by reason. */
export type SupportLoadExclusions = Readonly<{ withheld: number; envelope: number; empty: number }>;

export type SupportLoadSummary = Readonly<{
  supportId: string;
  label: string;
  nodeId: string;
  /** The support's equipment boundary association, when it has one. */
  equipment: Readonly<{ kind: string; equipmentReference: string; nozzleReference: string | null }> | null;
  /** Equipment interface records at the support's node (model annotations; no evaluation). */
  equipmentInterfaces: readonly string[];
  /** The sign convention the listed reaction rows state (first listed row), or null. */
  signConvention: string | null;
  rows: readonly SupportLoadRow[];
  extremes: readonly SupportLoadExtreme[];
  /** Rows in the extremes. */
  extremeRowCount: number;
  exclusions: SupportLoadExclusions;
}>;

export type SupportLoadSummaries = Readonly<{
  state: "available" | "unavailable";
  /** Why nothing is tabulated, when unavailable. */
  reason: string | null;
  frame: typeof SUPPORT_LOAD_FRAME;
  cases: readonly SupportLoadCase[];
  supports: readonly SupportLoadSummary[];
}>;

export const SUPPORT_LOAD_NO_RESULT = "No result";
export const SUPPORT_LOAD_NOT_SOLVED = "Mechanics not solved";
export const SUPPORT_LOAD_NO_SUPPORTS = "The model has no supports";
export const SUPPORT_LOAD_NO_CASES = "No load case or combination with result rows";
export const SUPPORT_LOAD_WITHHELD_NO_HEALTH = "Withheld: numerical health is not available for this result.";
export const SUPPORT_LOAD_WITHHELD_NOT_ELIGIBLE = "Withheld: the result's numerical standing does not admit it.";
export const SUPPORT_LOAD_WITHHELD_NO_STATE = "Withheld: this case has no numerical health state.";
export function supportLoadWithheldStateText(state: keyof typeof NUMERICAL_HEALTH_STATE_TEXT): string {
  return `Withheld: numerical health ${NUMERICAL_HEALTH_STATE_TEXT[state]}.`;
}

const COLUMN_BY_KIND = new Map<string, SupportLoadColumn>(
  SUPPORT_LOAD_COLUMNS.filter((item) => item.kind !== REACTION_COMPONENT_KIND).map((item) => [item.kind, item] as const)
);
const COLUMN_BY_COMPONENT = new Map<string, SupportLoadColumn>(
  SUPPORT_LOAD_COLUMNS.filter((item) => item.kind === REACTION_COMPONENT_KIND).map((item) => [item.component!, item] as const)
);

function columnOf(row: ResultRow): SupportLoadColumn | null {
  if (row.kind === REACTION_COMPONENT_KIND) return COLUMN_BY_COMPONENT.get(row.metadata?.component ?? "") ?? null;
  return COLUMN_BY_KIND.get(row.kind) ?? null;
}

/** The cases the result states rows for, in the order they first appear, load cases first, then
 * combinations (the viewport's case order), range-envelope combinations included and marked. */
export function supportLoadCases(model: PreviewModel, result: MechanicsResult): SupportLoadCase[] {
  const caseLabels = new Map(model.load_cases.map((item) => [item.id, item.label] as const));
  const combinationLabels = new Map((model.combinations ?? []).map((item) => [item.id, item.label] as const));
  const seen = new Map<string, { basis: ResultBasisRef; envelope: boolean }>();
  for (const row of result.results) {
    const basis = row.basis_ref;
    if (!basis || (basis.ref_type !== "load_case" && basis.ref_type !== "combination") || !Number.isFinite(row.value)) continue;
    const key = basisKey(basis)!;
    const entry = seen.get(key) ?? { basis: Object.freeze({ ref_type: basis.ref_type, ref_id: basis.ref_id }), envelope: false };
    if (row.metadata?.basis === RANGE_ENVELOPE_BASIS) entry.envelope = true;
    seen.set(key, entry);
  }
  const cases = [...seen].map(([key, entry]) => {
    const own = entry.basis.ref_type === "load_case" ? caseLabels.get(entry.basis.ref_id) : combinationLabels.get(entry.basis.ref_id);
    return Object.freeze({ key, basis: entry.basis, label: own ? `${own} (${entry.basis.ref_id})` : entry.basis.ref_id, envelope: entry.envelope });
  });
  return [...cases.filter((item) => item.basis.ref_type === "load_case"), ...cases.filter((item) => item.basis.ref_type === "combination")];
}

function withheldReason(standing: ResultOverlayStanding | null | undefined, basis: ResultBasisRef): string {
  if (!standing?.health) return SUPPORT_LOAD_WITHHELD_NO_HEALTH;
  if (!standing.eligible) return SUPPORT_LOAD_WITHHELD_NOT_ELIGIBLE;
  const state = rowHealthState(standing.health, { basis_ref: basis });
  return state ? supportLoadWithheldStateText(state) : SUPPORT_LOAD_WITHHELD_NO_STATE;
}

function extremesOf(rows: readonly SupportLoadRow[]): SupportLoadExtreme[] {
  return SUPPORT_LOAD_COLUMNS.map((item) => {
    const values = rows.flatMap((row) => {
      const value = row.inExtremes ? row.values[item.id] : undefined;
      return value ? [{ ...value, caseKey: row.case.key, caseLabel: row.case.label }] : [];
    });
    const units = [...new Set(values.map((value) => value.unit))].sort();
    if (units.length > 1) return Object.freeze({ column: item.id, max: null, min: null, note: `Units differ (${units.join(", ")}); no extreme is formed` });
    let max: SupportLoadExtremeValue | null = null;
    let min: SupportLoadExtremeValue | null = null;
    // Strict comparisons: a tie keeps the first row in row order (and 0 ties -0).
    for (const value of values) {
      if (max === null || value.value > max.value) max = Object.freeze(value);
      if (min === null || value.value < min.value) min = Object.freeze(value);
    }
    return Object.freeze({ column: item.id, max, min, note: null });
  });
}

type RowIndex = Readonly<{
  /** Finite reaction rows by support id, and finite node-movement rows by node id, each by case key. */
  reactions: ReadonlyMap<string, ReadonlyMap<string, readonly ResultRow[]>>;
  movements: ReadonlyMap<string, ReadonlyMap<string, readonly ResultRow[]>>;
  /** The producer's withheld attributions, by case key then support id. */
  attributions: ReadonlyMap<string, ReadonlyMap<string, string>>;
}>;

function rowIndex(result: MechanicsResult, cases: readonly SupportLoadCase[]): RowIndex {
  const reactions = new Map<string, Map<string, ResultRow[]>>();
  const movements = new Map<string, Map<string, ResultRow[]>>();
  const add = (index: Map<string, Map<string, ResultRow[]>>, entity: string, key: string, row: ResultRow) => {
    let byCase = index.get(entity);
    if (!byCase) index.set(entity, byCase = new Map<string, ResultRow[]>());
    const bucket = byCase.get(key);
    if (bucket) bucket.push(row);
    else byCase.set(key, [row]);
  };
  for (const row of result.results) {
    const key = basisKey(row.basis_ref);
    if (key === null || !Number.isFinite(row.value)) continue;
    const target = columnOf(row);
    if (target?.of === "support" || row.kind === REACTION_COMPONENT_KIND) add(reactions, row.entity_ref, key, row);
    else if (target?.of === "node") add(movements, row.entity_ref, key, row);
  }
  const attributions = new Map(cases.map((item) => [
    item.key,
    new Map(withheldSupportAttributions(result, item).map((entry) => [entry.supportId, entry.reason] as const))
  ] as const));
  return { reactions, movements, attributions };
}

function summaryOf(
  support: Support,
  model: PreviewModel,
  index: RowIndex,
  cases: readonly SupportLoadCase[],
  standing: ResultOverlayStanding | null | undefined
): SupportLoadSummary {
  let signConvention: string | null = null;
  const rows = cases.map((item): SupportLoadRow => {
    const reactionRows = index.reactions.get(support.id)?.get(item.key) ?? [];
    const caseRows = [...reactionRows, ...(index.movements.get(support.node)?.get(item.key) ?? [])];
    const attributionReason = index.attributions.get(item.key)?.get(support.id);
    const withheldAttribution = attributionReason === undefined ? null : { reason: attributionReason };
    if (!overlayMayShowRow(standing, { basis_ref: item.basis })) {
      return Object.freeze({
        case: item, standing: "withheld", withheldReason: withheldReason(standing, item.basis),
        reactionAbsence: null, reactionAbsenceDetail: null, values: Object.freeze({}), inExtremes: false
      });
    }
    const values: Partial<Record<SupportLoadColumnId, SupportLoadValue>> = {};
    const otherFrame = reactionRows.find((row) => row.metadata?.coordinate_system !== SUPPORT_LOAD_FRAME);
    for (const row of caseRows) {
      const target = columnOf(row);
      if (!target) continue;
      if (target.of === "support" && otherFrame) continue;
      // A node row that states a frame states the global one (the kinds are global).
      if (target.of === "node" && row.metadata?.coordinate_system !== undefined && row.metadata.coordinate_system !== SUPPORT_LOAD_FRAME) continue;
      // The first emitted row of a column for the case is kept; the producer emits one.
      if (values[target.id]) continue;
      values[target.id] = Object.freeze({ value: row.value, unit: row.unit, resultId: row.id });
      if (target.of === "support" && signConvention === null && row.kind === REACTION_COMPONENT_KIND && row.metadata?.sign_convention) {
        signConvention = row.metadata.sign_convention;
      }
    }
    const reactionAbsence: SupportReactionAbsence | null = withheldAttribution && reactionRows.length === 0
      ? "attribution_withheld"
      : otherFrame ? "frame_not_global" : reactionRows.length === 0 ? "no_rows" : null;
    const reactionAbsenceDetail = reactionAbsence === "attribution_withheld"
      ? withheldAttribution!.reason
      : reactionAbsence === "frame_not_global" ? otherFrame!.metadata?.coordinate_system ?? "none stated" : null;
    const frozen = Object.freeze(values);
    return Object.freeze({
      case: item, standing: "admitted", withheldReason: null, reactionAbsence, reactionAbsenceDetail,
      values: frozen, inExtremes: !item.envelope && Object.keys(frozen).length > 0
    });
  });
  const boundary = support.boundary_association;
  const exclusions = Object.freeze({
    withheld: rows.filter((row) => row.standing === "withheld").length,
    envelope: rows.filter((row) => row.standing === "admitted" && row.case.envelope).length,
    empty: rows.filter((row) => row.standing === "admitted" && !row.case.envelope && !row.inExtremes).length
  });
  return Object.freeze({
    supportId: support.id,
    label: support.label || support.id,
    nodeId: support.node,
    equipment: boundary
      ? Object.freeze({ kind: boundary.kind, equipmentReference: boundary.equipment_reference, nozzleReference: boundary.nozzle_reference ?? null })
      : null,
    equipmentInterfaces: Object.freeze((model.equipment_interfaces ?? []).filter((item) => item.node_ref === support.node).map((item) => item.label || item.id)),
    signConvention,
    rows: Object.freeze(rows),
    extremes: Object.freeze(extremesOf(rows)),
    extremeRowCount: rows.filter((row) => row.inExtremes).length,
    exclusions
  });
}

function unavailable(reason: string, cases: readonly SupportLoadCase[] = []): SupportLoadSummaries {
  return Object.freeze({ state: "unavailable", reason, frame: SUPPORT_LOAD_FRAME, cases, supports: Object.freeze([]) });
}

/**
 * The summary of every support of the model, in model order. `standing` is the reader's numerical
 * standing and the session's numerical health view, the same input the viewport overlays take.
 */
export function buildSupportLoadSummaries(input: {
  model: PreviewModel;
  result: MechanicsResult | null;
  standing: ResultOverlayStanding | null | undefined;
}): SupportLoadSummaries {
  const { model, result, standing } = input;
  if (!result) return unavailable(SUPPORT_LOAD_NO_RESULT);
  if (result.status.mechanics !== "MECHANICS_SOLVED") return unavailable(SUPPORT_LOAD_NOT_SOLVED);
  if (model.supports.length === 0) return unavailable(SUPPORT_LOAD_NO_SUPPORTS);
  const cases = supportLoadCases(model, result);
  if (cases.length === 0) return unavailable(SUPPORT_LOAD_NO_CASES);
  const index = rowIndex(result, cases);
  return Object.freeze({
    state: "available",
    reason: null,
    frame: SUPPORT_LOAD_FRAME,
    cases: Object.freeze(cases),
    supports: Object.freeze(model.supports.map((support) => summaryOf(support, model, index, cases, standing)))
  });
}
