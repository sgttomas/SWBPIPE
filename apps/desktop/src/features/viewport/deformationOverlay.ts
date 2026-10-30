// Result overlays for the viewport: the translational deformed shape (ux/uy/uz)
// and the rotation glyphs (rx/ry/rz). Both are display transforms of emitted
// result rows. Neither computes an engineering quantity, and neither makes a
// professional claim.

import type { MechanicsResult, PreviewModel, Vec3 } from "../../types";
import { unitRatio } from "../../services/unitConversion";
import { statusDisplayWithToken } from "../workspace/statusLabels";

type ResultRow = MechanicsResult["results"][number];

export type DeformationOverlay = {
  state: "not_started" | "available" | "blocked" | "unavailable";
  summary: string;
  maximum?: { value: number; unit: string };
  nodeCount?: number;
  boundary: string;
  /** Each node's drawn position at full deflection (phase 1). */
  nodePositions: Map<string, Vec3>;
  /** Each node's drawn offset at full deflection, in model length units of the
   * display. `nodePositions` is the node plus this offset. */
  offsets: Map<string, Vec3>;
  /** The load case or combination drawn (`ref_type:ref_id`), or null for each
   * node's governing row over every case. */
  basisKey: string | null;
  /** How the offsets are scaled. A factor multiplies the physical displacement;
   * the normalized scale draws the largest one at a fixed display length. */
  scale: { mode: "normalized" } | { mode: "factor"; factor: number; isDefault: boolean };
  /** The default factor for the drawn case: the 1-2-5 value nearest to drawing
   * the largest displacement at one tenth of the model extent. */
  defaultFactor?: number;
  /** Why a factor scale is not available, when one was asked for or cannot be. */
  factorUnavailable?: string;
  /** A factor can be drawn for this choice: one case with complete, convertible rows. */
  factorAccepted: boolean;
  /** Why the asked factor, or the default, is not the one drawn (no default without a
   * positive model extent; a factor whose shape cannot be represented). */
  factorNote?: string;
  /** The fraction of each offset drawn, from 0 (undeformed) to 1 (the stated scale). */
  phase: number;
};

/** A deflected-shape view setting. Never model data: it changes how emitted
 * displacement rows are drawn, never a result value. */
export type DeflectionScale = Readonly<{ mode: "normalized" } | { mode: "factor"; factor: number | null }>;

export type DeformationOptions = Readonly<{
  /** Draw only the rows of this load case or combination (`ref_type:ref_id`). */
  basisKey?: string | null;
  scale?: DeflectionScale;
  /** The model's length unit: node coordinates are in it. A factor needs it. */
  modelLengthUnit?: string | null;
  /** The model's largest bounding-box side, in model length units, for the default factor. */
  modelExtent?: number | null;
}>;

/** The normalized scale's display length of the largest displacement, in model length units. */
export const NORMALIZED_DEFORMATION_DISPLAY_OFFSET = 0.65;

/** The largest drawn offset a factor may produce, in model length units. The viewport's instance
 * buffers are 32-bit floats (largest finite 3.4e38) and the GPU multiplies them by the camera
 * matrices; at 1e15 the drawn coordinates and their products stay many orders inside that range. */
export const MAX_DRAWN_DEFLECTION_OFFSET = 1e15;

const NOT_GENERATED = "scale=not_generated; professional_claim=false";

function emptyDeformation(state: DeformationOverlay["state"], summary: string, basisKey: string | null): DeformationOverlay {
  return { state, summary, boundary: NOT_GENERATED, nodePositions: new Map(), offsets: new Map(), basisKey, scale: { mode: "normalized" }, factorAccepted: false, phase: 1 };
}

export function buildDeformationOverlay(
  model: PreviewModel,
  result: MechanicsResult | null,
  options: DeformationOptions = {}
): DeformationOverlay {
  const basisKey = options.basisKey ?? null;
  if (!result) return emptyDeformation("not_started", "not started; result rows=0", basisKey);
  if (result.status.mechanics !== "MECHANICS_SOLVED") {
    return emptyDeformation(
      "blocked",
      `blocked; mechanics=${statusDisplayWithToken(result.status.mechanics)}; rows=${result.results.length}`,
      basisKey
    );
  }

  // One case drawn: only its rows, and never a range envelope's (an envelope picks each
  // component separately, so its rows are not one solved state).
  const drawn = (row: ResultRow) => basisKey === null ||
    (rowBasisKey(row) === basisKey && row.metadata?.basis !== RANGE_ENVELOPE_BASIS);
  const nodeIds = new Set(model.nodes.map((node) => node.id));
  const nodeValues = new Map<string, { value: number; unit: string; basisKey: string }>();
  for (const row of result.results) {
    if (row.kind !== "displacement_magnitude" || !nodeIds.has(row.entity_ref) || !Number.isFinite(row.value) || !drawn(row)) continue;
    const current = nodeValues.get(row.entity_ref);
    if (!current || Math.abs(row.value) > Math.abs(current.value)) {
      nodeValues.set(row.entity_ref, {
        value: row.value,
        unit: row.unit,
        basisKey: rowBasisKey(row)
      });
    }
  }
  if (nodeValues.size === 0) return emptyDeformation("unavailable", "unavailable; displacement rows=0", basisKey);

  // Signed global-cartesian translation components (ux/uy/uz rows) grouped by
  // node and result basis, so each node's direction comes from the same load
  // case or combination as its governing magnitude row.
  const nodeComponentVectors = new Map<string, Map<string, Partial<Record<"x" | "y" | "z", number>>>>();
  for (const row of result.results) {
    const axis = DISPLACEMENT_COMPONENT_AXES[row.kind];
    if (!axis || !nodeIds.has(row.entity_ref) || !Number.isFinite(row.value) || !drawn(row)) continue;
    const perBasis: Map<string, Partial<Record<"x" | "y" | "z", number>>> = nodeComponentVectors.get(row.entity_ref) ??
    new Map();
    const vector = perBasis.get(rowBasisKey(row)) ?? {};
    vector[axis] = row.value;
    perBasis.set(rowBasisKey(row), vector);
    nodeComponentVectors.set(row.entity_ref, perBasis);
  }
  const nodeDirections = new Map<string, Vec3>();
  let directional = true;
  for (const [nodeId, info] of nodeValues) {
    const vector = nodeComponentVectors.get(nodeId)?.get(info.basisKey);
    if (!vector || vector.x === undefined || vector.y === undefined || vector.z === undefined) {
      directional = false;
      break;
    }
    nodeDirections.set(nodeId, unitVector({ x: vector.x, y: vector.y, z: vector.z }));
  }

  const values = Array.from(nodeValues.values());
  const maxValue = Math.max(...values.map((item) => Math.abs(item.value)));
  const units = Array.from(new Set(values.map((item) => item.unit))).sort();
  const unit = units.length === 1 ? units[0] : "mixed";

  // A factor multiplies the physical displacement, so it needs one case, a direction from
  // each node's own components, and displacement and model length units that convert.
  const lengthRatio = units.length === 1 && options.modelLengthUnit ? unitRatio(unit, options.modelLengthUnit) : null;
  const factorUnavailable = basisKey === null
    ? "choose one load case or combination; the governing row per node mixes cases"
    : !directional
      ? "displacement component rows are incomplete, so no direction is drawn to scale"
      : lengthRatio === null
        ? `displacement unit ${unit} does not convert to the model length unit ${options.modelLengthUnit || "(not declared)"}`
        : !(maxValue > 0)
          ? "every drawn displacement is zero"
          : undefined;
  const factorAccepted = factorUnavailable === undefined;
  const fallbackDirection: Vec3 = { x: 0, y: 1, z: 0 };
  // The offsets and positions for a factor (or the normalized scale), or null when any is not
  // a finite number: a finite factor can still be too large to draw.
  const draw = (factor: number | undefined) => {
    const offsets = new Map<string, Vec3>();
    const nodePositions = new Map<string, Vec3>();
    for (const node of model.nodes) {
      const value = nodeValues.get(node.id)?.value ?? 0;
      const length = factor !== undefined
        ? Math.abs(value) * lengthRatio! * factor
        : maxValue > 0 ? (Math.abs(value) / maxValue) * NORMALIZED_DEFORMATION_DISPLAY_OFFSET : 0;
      const direction = directional ? (nodeDirections.get(node.id) ?? fallbackDirection) : fallbackDirection;
      const offset = { x: direction.x * length, y: direction.y * length, z: direction.z * length };
      const position = { x: node.position.x + offset.x, y: node.position.y + offset.y, z: node.position.z + offset.z };
      // The normalized scale is bounded; only a factor is checked (a node already off the
      // finite range is the model's own issue, reported by the model index). The renderer keeps
      // geometry in 32-bit floats and multiplies it on the GPU, so a drawn offset is held within
      // MAX_DRAWN_DEFLECTION_OFFSET and every position must stay finite in 32 bits.
      if (factor !== undefined && Number.isFinite(node.position.x + node.position.y + node.position.z) &&
          (![offset.x, offset.y, offset.z].every((value) => Math.abs(value) <= MAX_DRAWN_DEFLECTION_OFFSET) ||
           ![position.x, position.y, position.z].every((value) => Number.isFinite(Math.fround(value))))) return null;
      offsets.set(node.id, offset);
      nodePositions.set(node.id, position);
    }
    return { offsets, nodePositions };
  };

  // The default draws the largest displacement at a tenth of the model's largest side, as the
  // nearest 1-2-5 value; it needs a positive, finite extent and a representable ratio.
  const extent = options.modelExtent;
  const rawDefault = factorAccepted && extent !== null && extent !== undefined && extent > 0 && Number.isFinite(extent)
    ? extent / 10 / (maxValue * lengthRatio!)
    : undefined;
  const defaultCandidate = rawDefault !== undefined && Number.isFinite(rawDefault) && rawDefault > 0 ? roundFactor125(rawDefault) : undefined;
  const defaultDrawing = defaultCandidate !== undefined ? draw(defaultCandidate) : null;
  const defaultFactor = defaultDrawing ? defaultCandidate : undefined;
  const noDefaultReason = !(extent !== null && extent !== undefined && extent > 0 && Number.isFinite(extent))
    ? "the model has no positive extent (for example a single node), so there is no default factor; enter one"
    : "the largest displacement is too small for a default factor; enter one";

  const asked = options.scale?.mode === "factor" ? options.scale.factor : undefined;
  let factor: number | undefined;
  let isDefault = false;
  let factorNote: string | undefined;
  let drawing: ReturnType<typeof draw> = null;
  if (factorAccepted && asked !== undefined && asked !== null && Number.isFinite(asked) && asked > 0) {
    drawing = draw(asked);
    if (drawing) factor = asked;
    else factorNote = `×${formatNumber(asked)} cannot be drawn: a drawn offset would exceed 1e15 model length units, beyond what the viewport represents`;
  }
  if (factorAccepted && asked !== undefined && factor === undefined) {
    if (defaultFactor !== undefined) {
      factor = defaultFactor;
      isDefault = true;
      drawing = defaultDrawing;
      if (factorNote) factorNote = `${factorNote}; the default ×${formatNumber(defaultFactor)} is drawn`;
    } else {
      factorNote = factorNote ? `${factorNote}, and ${noDefaultReason}` : noDefaultReason;
    }
  }
  const scale: DeformationOverlay["scale"] = factor !== undefined
    ? { mode: "factor", factor, isDefault }
    : { mode: "normalized" };
  const { offsets, nodePositions } = (factor !== undefined ? drawing : null) ?? draw(undefined)!;

  const vectorDirection = directional
    ? "global_cartesian_displacement_components"
    : "vertical_display_axis_fallback_component_rows_unavailable";
  const scaleText = factor !== undefined
    ? `scale=factor_times_physical_displacement; factor=${formatNumber(factor)}; model_length_unit=${options.modelLengthUnit}`
    : "scale=normalized_display_offset_not_physical_length";
  const caseText = basisKey === null ? "" : `; case=${basisKey}`;
  return {
    state: "available",
    summary: `available; nodes=${nodeValues.size}; max=${formatNumber(maxValue)} ${unit}`,
    maximum: { value: maxValue, unit },
    nodeCount: nodeValues.size,
    boundary: `${scaleText}${caseText}; vector_direction=${vectorDirection}; unit_basis=${unit}; professional_claim=false`,
    nodePositions,
    offsets,
    basisKey,
    scale,
    ...(defaultFactor !== undefined ? { defaultFactor } : {}),
    ...(factorUnavailable !== undefined && (asked !== undefined || basisKey !== null) ? { factorUnavailable } : {}),
    factorAccepted,
    ...(factorNote !== undefined ? { factorNote } : {}),
    phase: 1
  };
}

/**
 * The deformed shape drawn part of the way from the undeformed shape: each node at
 * `phase` (0 to 1) of its offset. Display only; the overlay's readout is unchanged.
 */
export function deformationAtPhase(model: PreviewModel, overlay: DeformationOverlay, phase: number): DeformationOverlay {
  const t = Number.isFinite(phase) ? Math.min(1, Math.max(0, phase)) : 1;
  if (overlay.state !== "available" || t === 1) return overlay;
  const nodePositions = new Map<string, Vec3>();
  for (const node of model.nodes) {
    const offset = overlay.offsets.get(node.id);
    if (!offset) continue;
    nodePositions.set(node.id, { x: node.position.x + offset.x * t, y: node.position.y + offset.y * t, z: node.position.z + offset.z * t });
  }
  const drawn = overlay.scale.mode === "factor" ? `; drawn_factor=${formatNumber(overlay.scale.factor * t)}` : "";
  return { ...overlay, nodePositions, phase: t, boundary: overlay.boundary.replace("; professional_claim=false", `; phase=${formatNumber(t)}${drawn}; professional_claim=false`) };
}

const FACTOR_MANTISSAS = [1, 2, 5] as const;

/** The 1-2-5 value (…, 0.5, 1, 2, 5, 10, 20, …) nearest to `value` on a log scale. */
export function roundFactor125(value: number): number {
  if (!Number.isFinite(value) || value <= 0) return 1;
  const exponent = Math.floor(Math.log10(value));
  let best = 1;
  let bestDistance = Infinity;
  for (const power of [exponent - 1, exponent, exponent + 1]) {
    for (const mantissa of FACTOR_MANTISSAS) {
      const candidate = cleanFactor(mantissa * 10 ** power);
      const distance = Math.abs(Math.log10(candidate) - Math.log10(value));
      if (distance < bestDistance) { best = candidate; bestDistance = distance; }
    }
  }
  return best;
}

/** The next 1-2-5 value above (`direction` 1) or below (-1) `value`. */
export function stepFactor125(value: number, direction: 1 | -1): number {
  if (!Number.isFinite(value) || value <= 0) return 1;
  const exponent = Math.floor(Math.log10(value));
  const candidates = [exponent - 1, exponent, exponent + 1, exponent + 2]
    .flatMap((power) => FACTOR_MANTISSAS.map((mantissa) => cleanFactor(mantissa * 10 ** power)));
  const relative = 1e-9;
  return direction === 1
    ? candidates.find((candidate) => candidate > value * (1 + relative)) ?? candidates.at(-1)!
    : [...candidates].reverse().find((candidate) => candidate < value * (1 - relative)) ?? candidates[0];
}

function cleanFactor(value: number): number {
  return Number(value.toPrecision(12));
}

const DISPLACEMENT_COMPONENT_AXES: Record<string, "x" | "y" | "z" | undefined> = {
  global_nodal_displacement_x: "x",
  global_nodal_displacement_y: "y",
  global_nodal_displacement_z: "z"
};

function rowBasisKey(row: ResultRow): string {
  return row.basis_ref ? `${row.basis_ref.ref_type}:${row.basis_ref.ref_id}` : "unspecified";
}

function unitVector(vector: Vec3): Vec3 {
  const length = Math.hypot(vector.x, vector.y, vector.z);
  if (!Number.isFinite(length) || length === 0) {
    return { x: 0, y: 0, z: 0 };
  }
  return { x: vector.x / length, y: vector.y / length, z: vector.z / length };
}

function formatNumber(value: number): string {
  const fixed = value.toFixed(6);
  // toFixed gives exponent notation from 1e21; its digits are not trailing decimals.
  if (/e/iu.test(fixed)) return String(value);
  return fixed.replace(/0+$/u, "").replace(/\.$/u, "");
}

// ---------------------------------------------------------------------------
// Rotation glyphs (DEL-07-05:1).
// ---------------------------------------------------------------------------

/** One node's rotation glyph: a double-headed arrow along the node's rotation
 * vector (right-hand rule) drawn from its emitted rx/ry/rz rows. */
export type RotationGlyph = {
  /** Unit direction of the rotation vector in global cartesian axes. */
  axis: Vec3;
  /** Display length: proportional to the rotation vector's length relative to
   * the largest displayed one. Not a physical length. */
  displayLength: number;
  /** The emitted component values the glyph draws, in their own unit. */
  components: { x: number; y: number; z: number };
  unit: string;
  basisKey: string;
};

export type RotationOverlay = {
  state: "not_started" | "available" | "blocked" | "unavailable";
  summary: string;
  /** The largest absolute value |rx|, |ry| or |rz| among the drawn bases'
   * emitted rows (the magnitude of an emitted value; its sign is not kept). */
  maximum?: { value: number; unit: string };
  nodeCount?: number;
  /** Nodes with a nonzero rotation, which get a glyph. */
  drawnCount?: number;
  boundary: string;
  glyphs: Map<string, RotationGlyph>;
};

/** Longest glyph, in model display units (cf. the deformed shape's 0.65 offset). */
export const ROTATION_GLYPH_MAX_DISPLAY_LENGTH = 0.5;

const ROTATION_COMPONENT_AXES: Record<string, "x" | "y" | "z" | undefined> = {
  global_nodal_rotation_x: "x",
  global_nodal_rotation_y: "y",
  global_nodal_rotation_z: "z"
};

const RANGE_ENVELOPE_BASIS = "explicit_user_range_envelope";

const ROTATION_BOUNDARY_NOT_GENERATED = "scale=not_generated; professional_claim=false";

/**
 * Build the rotation glyph overlay from emitted global nodal rotation rows.
 *
 * Per node, the drawn basis is the one whose displacement magnitude row governs
 * that node in the deformed shape, so translation and rotation show the same
 * load case or combination; if that basis has no drawable triple, the node has
 * no glyph. Only a node without a governing displacement row uses its basis
 * with the longest rotation vector (ties: first basis key). A basis is
 * drawable only with all three finite components in one unit. Range-envelope
 * rows are never drawn: an envelope picks each component separately, so the
 * three need not be one emitted rotation state. Glyph lengths
 * are normalized to the longest drawn rotation vector, so every drawn node must
 * share one unit; mixed units leave the overlay unavailable rather than
 * comparing them. With `basisKey`, only that load case's or combination's rows
 * are read, as the deformed shape reads them for the same option.
 */
export function buildRotationOverlay(
  model: PreviewModel,
  result: MechanicsResult | null,
  options: Readonly<{ basisKey?: string | null }> = {}
): RotationOverlay {
  const basisKey = options.basisKey ?? null;
  if (!result) {
    return { state: "not_started", summary: "not started; result rows=0", boundary: ROTATION_BOUNDARY_NOT_GENERATED, glyphs: new Map() };
  }
  if (result.status.mechanics !== "MECHANICS_SOLVED") {
    return {
      state: "blocked",
      summary: `blocked; mechanics=${statusDisplayWithToken(result.status.mechanics)}; rows=${result.results.length}`,
      boundary: ROTATION_BOUNDARY_NOT_GENERATED,
      glyphs: new Map()
    };
  }

  const nodeIds = new Set(model.nodes.map((node) => node.id));
  type Triple = { x?: ResultRow; y?: ResultRow; z?: ResultRow };
  const triples = new Map<string, Map<string, Triple>>();
  for (const row of result.results) {
    const axis = ROTATION_COMPONENT_AXES[row.kind];
    if (!axis || !nodeIds.has(row.entity_ref) || !Number.isFinite(row.value)) continue;
    if (row.dimension !== undefined && row.dimension !== "angle") continue;
    if (row.metadata?.basis === RANGE_ENVELOPE_BASIS) continue;
    if (basisKey !== null && rowBasisKey(row) !== basisKey) continue;
    const perBasis = triples.get(row.entity_ref) ?? new Map<string, Triple>();
    const triple = perBasis.get(rowBasisKey(row)) ?? {};
    triple[axis] = row;
    perBasis.set(rowBasisKey(row), triple);
    triples.set(row.entity_ref, perBasis);
  }

  const governingBasis = governingDisplacementBases(nodeIds, result, basisKey);
  const chosen = new Map<string, { components: { x: number; y: number; z: number }; unit: string; basisKey: string; length: number }>();
  for (const [nodeId, perBasis] of triples) {
    const complete = [...perBasis.entries()]
      .flatMap(([basisKey, triple]) => {
        const { x, y, z } = triple;
        if (!x || !y || !z || x.unit !== y.unit || y.unit !== z.unit) return [];
        const components = { x: x.value, y: y.value, z: z.value };
        return [{ components, unit: x.unit, basisKey, length: Math.hypot(components.x, components.y, components.z) }];
      })
      .sort((left, right) => (left.basisKey < right.basisKey ? -1 : left.basisKey > right.basisKey ? 1 : 0));
    if (!complete.length) continue;
    const governing = governingBasis.get(nodeId);
    const pick = governing !== undefined
      ? complete.find((candidate) => candidate.basisKey === governing)
      : complete.reduce((best, candidate) => (candidate.length > best.length ? candidate : best));
    if (pick) chosen.set(nodeId, pick);
  }

  if (chosen.size === 0) {
    return {
      state: "unavailable",
      summary: "unavailable; no complete rotation triple on a drawable basis",
      boundary: ROTATION_BOUNDARY_NOT_GENERATED,
      glyphs: new Map()
    };
  }
  const units = Array.from(new Set([...chosen.values()].map((item) => item.unit))).sort();
  if (units.length !== 1) {
    return {
      state: "unavailable",
      summary: `unavailable; rotation units mixed (${units.join(", ")}); nodes=${chosen.size}`,
      boundary: ROTATION_BOUNDARY_NOT_GENERATED,
      glyphs: new Map()
    };
  }

  const unit = units[0];
  const maxLength = Math.max(...[...chosen.values()].map((item) => item.length));
  const maxComponent = Math.max(
    ...[...chosen.values()].flatMap(({ components }) => [Math.abs(components.x), Math.abs(components.y), Math.abs(components.z)])
  );
  const glyphs = new Map<string, RotationGlyph>();
  for (const [nodeId, item] of chosen) {
    if (!(item.length > 0) || !Number.isFinite(item.length)) continue;
    glyphs.set(nodeId, {
      axis: unitVector(item.components),
      displayLength: (item.length / maxLength) * ROTATION_GLYPH_MAX_DISPLAY_LENGTH,
      components: item.components,
      unit: item.unit,
      basisKey: item.basisKey
    });
  }
  return {
    state: "available",
    summary: `available; nodes=${chosen.size}; drawn=${glyphs.size}; max_abs_component=${formatNumber(maxComponent)} ${unit}`,
    maximum: { value: maxComponent, unit },
    nodeCount: chosen.size,
    drawnCount: glyphs.size,
    boundary: `scale=normalized_display_length_not_physical; glyph=double_arrow_right_hand_rotation_vector; vector_direction=global_cartesian_rotation_components; basis=governing_displacement_else_longest; range_envelope_rows=not_drawn; unit_basis=${unit}; professional_claim=false`,
    glyphs
  };
}

/** The basis of each node's governing displacement magnitude row, chosen exactly
 * as the deformed shape chooses it (largest absolute value, first row wins ties). */
function governingDisplacementBases(nodeIds: ReadonlySet<string>, result: MechanicsResult, basisKey: string | null): Map<string, string> {
  const governing = new Map<string, { value: number; basisKey: string }>();
  for (const row of result.results) {
    if (row.kind !== "displacement_magnitude" || !nodeIds.has(row.entity_ref) || !Number.isFinite(row.value)) continue;
    if (basisKey !== null && (rowBasisKey(row) !== basisKey || row.metadata?.basis === RANGE_ENVELOPE_BASIS)) continue;
    const current = governing.get(row.entity_ref);
    if (!current || Math.abs(row.value) > Math.abs(current.value)) {
      governing.set(row.entity_ref, { value: row.value, basisKey: rowBasisKey(row) });
    }
  }
  return new Map([...governing].map(([nodeId, { basisKey }]) => [nodeId, basisKey]));
}

/** One glyph's placement before the render-local transform: its node, the
 * anchor in authored coordinates, and the glyph's axis and length. */
export type RotationGlyphAnchor = { nodeId: string; anchor: Vec3; axis: Vec3; displayLength: number };

/** Anchors each glyph at its node's displayed position: on the deformed shape
 * when it is drawn, otherwise at the undeformed node. Nodes in `excluded`
 * (invalid geometry) get no glyph. */
export function rotationGlyphAnchors(
  model: PreviewModel,
  rotation: RotationOverlay,
  deformation: DeformationOverlay,
  excluded: (nodeId: string) => boolean = () => false
): RotationGlyphAnchor[] {
  if (rotation.state !== "available") return [];
  return model.nodes.flatMap((node) => {
    const glyph = rotation.glyphs.get(node.id);
    if (!glyph || excluded(node.id)) return [];
    const anchor = (deformation.state === "available" ? deformation.nodePositions.get(node.id) : undefined) ?? node.position;
    return [{ nodeId: node.id, anchor, axis: glyph.axis, displayLength: glyph.displayLength }];
  });
}

