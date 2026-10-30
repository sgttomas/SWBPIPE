import type { DisplayQuantityRequest, DisplayQuantityResult } from "../../services/displayQuantityService";
import type { Vec3 } from "../../types";

export const ROUTING_GESTURE_SLOP_PX = 4;

export type RoutingPlane = "XY" | "XZ" | "YZ";
export type RoutingAxisConstraint = "Free" | "X" | "Y" | "Z";

export type RoutingPlaneDefinition = {
  anchor: Vec3;
  fixedAxis: "x" | "y" | "z";
  fixedValue: number;
  normal: Vec3;
  plane: RoutingPlane;
};

export type PointerGesture = {
  cancelled: boolean;
  maxDisplacement: number;
  pointerId: number;
  startX: number;
  startY: number;
};

export type RouteGhost = {
  from: Vec3;
  provenance: "captured" | "existing" | "hover";
  to: Vec3;
};

export class RoutingPlacementGate {
  private generation = 0;

  invalidate(): number {
    this.generation += 1;
    return this.generation;
  }

  isCurrent(generation: number): boolean {
    return generation === this.generation;
  }
}

const AXES_BY_PLANE: Record<RoutingPlane, readonly RoutingAxisConstraint[]> = {
  XY: ["Free", "X", "Y"],
  XZ: ["Free", "X", "Z"],
  YZ: ["Free", "Y", "Z"]
};

export function routingPlaneDefinition(plane: RoutingPlane, anchor: Vec3): RoutingPlaneDefinition {
  if (plane === "XY") {
    return { anchor, fixedAxis: "z", fixedValue: anchor.z, normal: { x: 0, y: 0, z: 1 }, plane };
  }
  if (plane === "YZ") {
    return { anchor, fixedAxis: "x", fixedValue: anchor.x, normal: { x: 1, y: 0, z: 0 }, plane };
  }
  return { anchor, fixedAxis: "y", fixedValue: anchor.y, normal: { x: 0, y: 1, z: 0 }, plane };
}

export function applicableRoutingAxes(plane: RoutingPlane): readonly RoutingAxisConstraint[] {
  return AXES_BY_PLANE[plane];
}

export function normalizeRoutingAxis(
  plane: RoutingPlane,
  constraint: RoutingAxisConstraint
): RoutingAxisConstraint {
  return AXES_BY_PLANE[plane].includes(constraint) ? constraint : "Free";
}

export function constrainRoutingPoint(
  pointOnPlane: Vec3,
  anchor: Vec3,
  plane: RoutingPlane,
  constraint: RoutingAxisConstraint
): Vec3 {
  const definition = routingPlaneDefinition(plane, anchor);
  const projected = { ...pointOnPlane, [definition.fixedAxis]: definition.fixedValue };
  const normalized = normalizeRoutingAxis(plane, constraint);
  if (normalized === "Free") return projected;
  const axis = normalized.toLowerCase() as "x" | "y" | "z";
  return {
    x: axis === "x" ? projected.x : anchor.x,
    y: axis === "y" ? projected.y : anchor.y,
    z: axis === "z" ? projected.z : anchor.z
  };
}

export function startPointerGesture(pointerId: number, clientX: number, clientY: number): PointerGesture {
  return { cancelled: false, maxDisplacement: 0, pointerId, startX: clientX, startY: clientY };
}

export function updatePointerGesture(
  gesture: PointerGesture,
  pointerId: number,
  clientX: number,
  clientY: number
): PointerGesture {
  if (pointerId !== gesture.pointerId) return gesture;
  const displacement = Math.hypot(clientX - gesture.startX, clientY - gesture.startY);
  return {
    ...gesture,
    cancelled: gesture.cancelled || displacement > ROUTING_GESTURE_SLOP_PX,
    maxDisplacement: Math.max(gesture.maxDisplacement, displacement)
  };
}

export function cancelPointerGesture(gesture: PointerGesture): PointerGesture {
  return { ...gesture, cancelled: true };
}

export function pointerGestureMayAuthor(
  gesture: PointerGesture | null,
  pointerId: number,
  clientX: number,
  clientY: number
): boolean {
  if (!gesture || gesture.pointerId !== pointerId || gesture.cancelled) return false;
  return updatePointerGesture(gesture, pointerId, clientX, clientY).maxDisplacement <= ROUTING_GESTURE_SLOP_PX;
}

export function convertedRoutingPoint(
  results: DisplayQuantityResult[],
  requestedUnit: string
): Vec3 | null {
  if (results.length !== 3) return null;
  const expected = new Set(["viewport-routing-x", "viewport-routing-y", "viewport-routing-z"]);
  const values = new Map<string, number>();
  for (const result of results) {
    if (
      result.status !== "converted" ||
      !expected.has(result.id) ||
      result.unit !== requestedUnit ||
      !Number.isFinite(result.value) ||
      values.has(result.id)
    ) return null;
    values.set(result.id, result.value);
  }
  if (values.size !== expected.size) return null;
  return {
    x: values.get("viewport-routing-x")!,
    y: values.get("viewport-routing-y")!,
    z: values.get("viewport-routing-z")!
  };
}

export function routingConversionRequests(
  point: Vec3,
  modelUnit: string,
  requestedUnit: string
): DisplayQuantityRequest[] {
  return (["x", "y", "z"] as const).map((axis) => ({
    id: `viewport-routing-${axis}`,
    value: point[axis],
    from_unit: modelUnit,
    to_unit: requestedUnit,
    dimension_id: "length"
  }));
}

export function resolveExistingRouteGhost(
  nodes: ReadonlyMap<string, Vec3>,
  fromId: string,
  toId: string
): RouteGhost | null {
  if (!fromId || !toId || fromId === toId) return null;
  const from = nodes.get(fromId);
  const to = nodes.get(toId);
  return from && to ? { from, to, provenance: "existing" } : null;
}

export function resolveNewRouteGhost(
  from: Vec3 | null,
  point: Vec3 | null,
  provenance: "captured" | "hover" | null
): RouteGhost | null {
  return from && point && provenance ? { from, to: point, provenance } : null;
}
