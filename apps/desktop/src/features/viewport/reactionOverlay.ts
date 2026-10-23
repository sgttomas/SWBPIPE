// Support reaction arrows in the viewport (FR-GUI-001; owner ruling OD-5). For one load case or
// combination, each support's emitted `support_reaction_component_v2` rows are drawn at its node:
// the force (Fx, Fy, Fz) as a single-headed arrow and the moment (Mx, My, Mz) as the rotation
// glyph's double-headed arrow (right-hand rule). Rows are drawn only in the frame they state:
// `coordinate_system: "global"`, the frame the viewport draws in, with the rows' own sign
// convention ("support-on-pipe": the action of the support on the pipe). A display of emitted rows
// only: it computes no engineering quantity and makes no professional claim.

import type { MechanicsResult, PreviewModel, Vec3 } from "../../types";
import { overlayMayShowRow, type ResultOverlayStanding } from "../results/numericalHealth";
import { RANGE_ENVELOPE_BASIS, basisKey, type ResultOverlayCase } from "./resultColourOverlay";

type ResultRow = MechanicsResult["results"][number];

export const REACTION_COMPONENT_KIND = "support_reaction_component_v2";
export const REACTION_FORCE_MAGNITUDE_KIND = "support_reaction_force_magnitude_v2";
export const REACTION_MOMENT_MAGNITUDE_KIND = "support_reaction_moment_magnitude_v2";
/** The only frame drawn: the viewport's own axes are the model's global axes. */
export const REACTION_DRAWN_FRAME = "global";

/** Longest force arrow and longest moment glyph, in model display units (cf. the rotation glyph's 0.5). */
export const REACTION_FORCE_MAX_DISPLAY_LENGTH = 0.9;
export const REACTION_MOMENT_MAX_DISPLAY_LENGTH = 0.5;

const FORCE_AXES: Readonly<Record<string, "x" | "y" | "z">> = Object.freeze({ Fx: "x", Fy: "y", Fz: "z" });
const MOMENT_AXES: Readonly<Record<string, "x" | "y" | "z">> = Object.freeze({ Mx: "x", My: "y", Mz: "z" });

export type ReactionVector = Readonly<{
  supportId: string;
  nodeId: string;
  /** Emitted global components, in `unit`. */
  components: Readonly<{ x: number; y: number; z: number }>;
  /** Length of the component vector. */
  length: number;
  unit: string;
  /** Unit direction in global axes; zero for a zero vector. */
  direction: Vec3;
  /** Display length, proportional to `length` over the longest drawn vector of its kind. */
  displayLength: number;
  /** The emitted magnitude row's value, when the result carries one. */
  emittedMagnitude: number | null;
}>;

export type ReactionWithheld = Readonly<{ supportId: string; reason: string }>;

export type ReactionOverlay = Readonly<{
  state: "available" | "held" | "unavailable";
  reason: string | null;
  case: ResultOverlayCase | null;
  /** Forces and moments by support id; a zero vector is listed but has no arrow. */
  forces: ReadonlyMap<string, ReactionVector>;
  moments: ReadonlyMap<string, ReactionVector>;
  maxForce: Readonly<{ value: number; unit: string }> | null;
  maxMoment: Readonly<{ value: number; unit: string }> | null;
  /** Supports the producer withholds for this case ("Ambiguous attribution is withheld"). */
  withheld: readonly ReactionWithheld[];
  /** Supports of the model with no reaction rows for this case, other than the withheld ones. */
  withoutRows: readonly string[];
  /** Supports whose reaction rows state a frame other than global, or none: not drawn. */
  frameNotDrawn: readonly string[];
  /** Reaction rows of this case that the numerical standing withholds. */
  heldRowCount: number;
  /** Arrows and glyphs drawn (nonzero vectors). */
  drawnCount: number;
}>;

/** The producer's withheld support attributions for a load case (preview route evidence). */
export function withheldSupportAttributions(result: MechanicsResult, overlayCase: ResultOverlayCase): ReactionWithheld[] {
  if (overlayCase.basis.ref_type !== "load_case") return [];
  const cases = (result.contract_evidence as { preview_cases?: unknown } | undefined)?.preview_cases;
  if (!Array.isArray(cases)) return [];
  const entry = cases.find((item) => !!item && typeof item === "object" && (item as { load_case_id?: unknown }).load_case_id === overlayCase.basis.ref_id);
  const withheld = (entry as { support_attribution?: { withheld?: unknown } } | undefined)?.support_attribution?.withheld;
  if (!Array.isArray(withheld)) return [];
  return withheld.flatMap((item) => {
    const supportId = (item as { support_id?: unknown } | null)?.support_id;
    const reason = (item as { reason?: unknown } | null)?.reason;
    return typeof supportId === "string" ? [Object.freeze({ supportId, reason: typeof reason === "string" ? reason : "withheld" })] : [];
  });
}

type Triple = { x?: ResultRow; y?: ResultRow; z?: ResultRow };

function completeTriple(triple: Triple | undefined): { components: { x: number; y: number; z: number }; unit: string } | null {
  if (!triple?.x || !triple.y || !triple.z) return null;
  if (triple.x.unit !== triple.y.unit || triple.y.unit !== triple.z.unit) return null;
  return { components: { x: triple.x.value, y: triple.y.value, z: triple.z.value }, unit: triple.x.unit };
}

function unavailable(reason: string, overlayCase: ResultOverlayCase | null): ReactionOverlay {
  return Object.freeze({
    state: "unavailable", reason, case: overlayCase, forces: new Map(), moments: new Map(), maxForce: null, maxMoment: null,
    withheld: [], withoutRows: [], frameNotDrawn: [], heldRowCount: 0, drawnCount: 0
  });
}

export const REACTION_HELD_REASON = "No reaction arrows on the current model";

/**
 * Builds the reaction arrows of one case. A support draws a force arrow from a complete Fx, Fy, Fz
 * triple in one unit, and a moment glyph from a complete Mx, My, Mz triple, all in the global frame.
 * Lengths are normalised to the longest drawn vector of their kind, so all forces must share one
 * unit and all moments one unit; mixed units leave that kind undrawn rather than compare them.
 */
export function buildReactionOverlay(input: {
  model: PreviewModel;
  result: MechanicsResult | null;
  standing: ResultOverlayStanding | null | undefined;
  overlayCase: ResultOverlayCase | null;
}): ReactionOverlay {
  const { model, result, standing, overlayCase } = input;
  if (!result) return unavailable("No result", overlayCase);
  if (result.status.mechanics !== "MECHANICS_SOLVED") return unavailable("Mechanics not solved", overlayCase);
  if (!overlayCase) return unavailable("No load case or combination with result rows", overlayCase);
  const supportNodes = new Map(model.supports.map((support) => [support.id, support.node] as const));
  const forces = new Map<string, Triple>();
  const moments = new Map<string, Triple>();
  const magnitudes = new Map<string, { force?: number; moment?: number }>();
  const withRows = new Set<string>();
  const frameNotDrawn = new Set<string>();
  let heldRowCount = 0;
  for (const row of result.results) {
    if (row.kind !== REACTION_COMPONENT_KIND && row.kind !== REACTION_FORCE_MAGNITUDE_KIND && row.kind !== REACTION_MOMENT_MAGNITUDE_KIND) continue;
    if (!supportNodes.has(row.entity_ref) || basisKey(row.basis_ref) !== overlayCase.key || row.metadata?.basis === RANGE_ENVELOPE_BASIS) continue;
    if (!Number.isFinite(row.value)) continue;
    withRows.add(row.entity_ref);
    if (!overlayMayShowRow(standing, row)) {
      heldRowCount += 1;
      continue;
    }
    if (row.metadata?.coordinate_system !== REACTION_DRAWN_FRAME) {
      frameNotDrawn.add(row.entity_ref);
      continue;
    }
    if (row.kind === REACTION_FORCE_MAGNITUDE_KIND || row.kind === REACTION_MOMENT_MAGNITUDE_KIND) {
      const entry = magnitudes.get(row.entity_ref) ?? {};
      if (row.kind === REACTION_FORCE_MAGNITUDE_KIND) entry.force = row.value;
      else entry.moment = row.value;
      magnitudes.set(row.entity_ref, entry);
      continue;
    }
    const component = row.metadata?.component ?? "";
    const forceAxis = FORCE_AXES[component];
    const momentAxis = MOMENT_AXES[component];
    if (forceAxis) forces.set(row.entity_ref, { ...forces.get(row.entity_ref), [forceAxis]: row });
    else if (momentAxis) moments.set(row.entity_ref, { ...moments.get(row.entity_ref), [momentAxis]: row });
  }
  const withheld = withheldSupportAttributions(result, overlayCase).filter((item) => supportNodes.has(item.supportId));
  const withheldIds = new Set(withheld.map((item) => item.supportId));
  const withoutRows = model.supports.map((support) => support.id).filter((id) => !withRows.has(id) && !withheldIds.has(id));
  if (withRows.size > 0 && heldRowCount > 0 && forces.size === 0 && moments.size === 0) {
    return Object.freeze({
      state: "held", reason: REACTION_HELD_REASON, case: overlayCase, forces: new Map(), moments: new Map(), maxForce: null, maxMoment: null,
      withheld, withoutRows, frameNotDrawn: [...frameNotDrawn], heldRowCount, drawnCount: 0
    });
  }
  const reasons: string[] = [];
  const vectors = (triples: Map<string, Triple>, kind: "force" | "moment", maxLength: number) => {
    const complete = [...triples].flatMap(([supportId, triple]) => {
      const item = completeTriple(triple);
      return item ? [{ supportId, ...item, length: Math.hypot(item.components.x, item.components.y, item.components.z) }] : [];
    });
    const units = [...new Set(complete.map((item) => item.unit))].sort();
    if (units.length > 1) {
      reasons.push(`${kind === "force" ? "Force" : "Moment"} units mixed (${units.join(", ")})`);
      return { vectors: new Map<string, ReactionVector>(), maximum: null };
    }
    const longest = complete.reduce((best, item) => Math.max(best, item.length), 0);
    const output = new Map<string, ReactionVector>();
    for (const item of complete) {
      const nodeId = supportNodes.get(item.supportId)!;
      const scale = item.length > 0 ? 1 / item.length : 0;
      output.set(item.supportId, Object.freeze({
        supportId: item.supportId,
        nodeId,
        components: Object.freeze(item.components),
        length: item.length,
        unit: item.unit,
        direction: Object.freeze({ x: item.components.x * scale, y: item.components.y * scale, z: item.components.z * scale }),
        displayLength: longest > 0 ? (item.length / longest) * maxLength : 0,
        emittedMagnitude: magnitudes.get(item.supportId)?.[kind] ?? null
      }));
    }
    return { vectors: output, maximum: complete.length ? Object.freeze({ value: longest, unit: units[0] }) : null };
  };
  const force = vectors(forces, "force", REACTION_FORCE_MAX_DISPLAY_LENGTH);
  const moment = vectors(moments, "moment", REACTION_MOMENT_MAX_DISPLAY_LENGTH);
  const drawnCount = [...force.vectors.values(), ...moment.vectors.values()].filter((item) => item.displayLength > 0).length;
  const any = force.vectors.size + moment.vectors.size > 0;
  return Object.freeze({
    state: any ? "available" : "unavailable",
    reason: any ? (reasons.length ? reasons.join("; ") : null) : reasons.length ? reasons.join("; ") : `No support reaction rows for ${overlayCase.label}`,
    case: overlayCase,
    forces: force.vectors,
    moments: moment.vectors,
    maxForce: force.maximum,
    maxMoment: moment.maximum,
    withheld,
    withoutRows,
    frameNotDrawn: [...frameNotDrawn],
    heldRowCount,
    drawnCount
  });
}
