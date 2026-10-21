// Result overlays for the viewport: the translational deformed shape (ux/uy/uz)
// and the rotation glyphs (rx/ry/rz). Both are display transforms of emitted
// result rows. Neither computes an engineering quantity, and neither makes a
// professional claim.

import type { MechanicsResult, PreviewModel, Vec3 } from "../../types";
import { statusDisplayWithToken } from "../workspace/statusLabels";

type ResultRow = MechanicsResult["results"][number];

export type DeformationOverlay = {
  state: "not_started" | "available" | "blocked" | "unavailable";
  summary: string;
  maximum?: { value: number; unit: string };
  nodeCount?: number;
  boundary: string;
  nodePositions: Map<string, Vec3>;
};

export function buildDeformationOverlay(model: PreviewModel, result: MechanicsResult | null): DeformationOverlay {
  if (!result) {
    return {
      state: "not_started",
      summary: "not started; result rows=0",
      boundary: "scale=not_generated; professional_claim=false",
      nodePositions: new Map()
    };
  }
  if (result.status.mechanics !== "MECHANICS_SOLVED") {
    return {
      state: "blocked",
      summary: `blocked; mechanics=${statusDisplayWithToken(result.status.mechanics)}; rows=${result.results.length}`,
      boundary: "scale=not_generated; professional_claim=false",
      nodePositions: new Map()
    };
  }

  const nodeIds = new Set(model.nodes.map((node) => node.id));
  const nodeValues = new Map<string, { value: number; unit: string; basisKey: string }>();
  for (const row of result.results) {
    if (row.kind !== "displacement_magnitude" || !nodeIds.has(row.entity_ref) || !Number.isFinite(row.value)) continue;
    const current = nodeValues.get(row.entity_ref);
    if (!current || Math.abs(row.value) > Math.abs(current.value)) {
      nodeValues.set(row.entity_ref, {
        value: row.value,
        unit: row.unit,
        basisKey: rowBasisKey(row)
      });
    }
  }
  if (nodeValues.size === 0) {
    return {
      state: "unavailable",
      summary: "unavailable; displacement rows=0",
      boundary: "scale=not_generated; professional_claim=false",
      nodePositions: new Map()
    };
  }

  // Signed global-cartesian translation components (ux/uy/uz rows) grouped by
  // node and result basis, so each node's direction comes from the same load
  // case or combination as its governing magnitude row.
  const nodeComponentVectors = new Map<string, Map<string, Partial<Record<"x" | "y" | "z", number>>>>();
  for (const row of result.results) {
    const axis = DISPLACEMENT_COMPONENT_AXES[row.kind];
    if (!axis || !nodeIds.has(row.entity_ref) || !Number.isFinite(row.value)) continue;
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
  const displayOffset = 0.65;
  const fallbackDirection: Vec3 = { x: 0, y: 1, z: 0 };
  const nodePositions = new Map(
    model.nodes.map((node) => {
      const value = nodeValues.get(node.id)?.value ?? 0;
      const normalizedOffset = maxValue > 0 ? (Math.abs(value) / maxValue) * displayOffset : 0;
      const direction = directional ? (nodeDirections.get(node.id) ?? fallbackDirection) : fallbackDirection;
      return [
        node.id,
        {
          x: node.position.x + direction.x * normalizedOffset,
          y: node.position.y + direction.y * normalizedOffset,
          z: node.position.z + direction.z * normalizedOffset
        }
      ] as const;
    })
  );

  const vectorDirection = directional
    ? "global_cartesian_displacement_components"
    : "vertical_display_axis_fallback_component_rows_unavailable";
  return {
    state: "available",
    summary: `available; nodes=${nodeValues.size}; max=${formatNumber(maxValue)} ${unit}`,
    maximum: { value: maxValue, unit },
    nodeCount: nodeValues.size,
    boundary: `scale=normalized_display_offset_not_physical_length; vector_direction=${vectorDirection}; unit_basis=${unit}; professional_claim=false`,
    nodePositions
  };
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
  return value.toFixed(6).replace(/0+$/u, "").replace(/\.$/u, "");
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
 * comparing them.
 */
export function buildRotationOverlay(model: PreviewModel, result: MechanicsResult | null): RotationOverlay {
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
    const perBasis = triples.get(row.entity_ref) ?? new Map<string, Triple>();
    const triple = perBasis.get(rowBasisKey(row)) ?? {};
    triple[axis] = row;
    perBasis.set(rowBasisKey(row), triple);
    triples.set(row.entity_ref, perBasis);
  }

  const governingBasis = governingDisplacementBases(nodeIds, result);
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
function governingDisplacementBases(nodeIds: ReadonlySet<string>, result: MechanicsResult): Map<string, string> {
  const governing = new Map<string, { value: number; basisKey: string }>();
  for (const row of result.results) {
    if (row.kind !== "displacement_magnitude" || !nodeIds.has(row.entity_ref) || !Number.isFinite(row.value)) continue;
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

