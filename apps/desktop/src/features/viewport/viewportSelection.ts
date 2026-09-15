import * as THREE from "three";
import type { EntityRef, PreviewComponent, PreviewModel, Vec3 } from "../../types";
import type { Bounds3, ModelIndex } from "../workspace/modelIndex";
import { boundsIntersect } from "../workspace/modelIndex";
import type { EntityKey } from "../workspace/selectionState";

export type RenderTransform = Readonly<{
  origin: Readonly<Vec3>;
  localBounds: Bounds3 | null;
}>;

export type LabelPriorityInput = Readonly<{
  primaryKey: EntityKey | null;
  hoverKey?: EntityKey | null;
  selectedKeys?: readonly EntityKey[];
  diagnosticKeys?: readonly EntityKey[];
  hiddenKeys?: ReadonlySet<EntityKey>;
  cameraTarget?: Readonly<Vec3>;
  limit?: number;
}>;

export type BoxSelectionFilter = "all" | "pipes" | "nodes" | "supports" | "components";
export type BoxSelectionDirection = "left-to-right" | "right-to-left";
export type BoxSelectionRect = Readonly<{ left: number; top: number; right: number; bottom: number }>;

export function boxSelectEntityKeys(
  index: ModelIndex,
  model: PreviewModel,
  options: Readonly<{
    camera: THREE.PerspectiveCamera;
    canvas: HTMLCanvasElement;
    renderOrigin: Readonly<Vec3>;
    rect: BoxSelectionRect;
    direction: BoxSelectionDirection;
    filter: BoxSelectionFilter;
    hiddenKeys?: ReadonlySet<EntityKey>;
  }>
): readonly EntityKey[] {
  const canvasRect = options.canvas.getBoundingClientRect();
  if (!(canvasRect.width > 0) || !(canvasRect.height > 0)) return Object.freeze([]);
  options.camera.updateMatrixWorld();
  const viewProjection = new THREE.Matrix4().multiplyMatrices(
    options.camera.projectionMatrix,
    options.camera.matrixWorldInverse
  );
  const frustum = new THREE.Frustum().setFromProjectionMatrix(viewProjection);
  const candidateKeys = new Set<EntityKey>();
  for (const chunk of index.spatialChunks) {
    const local = localBounds(chunk.bounds, options.renderOrigin);
    const box = new THREE.Box3(
      new THREE.Vector3(local.min.x, local.min.y, local.min.z),
      new THREE.Vector3(local.max.x, local.max.y, local.max.z)
    );
    if (!frustum.intersectsBox(box) && !frustum.containsPoint(box.getCenter(new THREE.Vector3()))) continue;
    for (const key of chunk.entityKeys) candidateKeys.add(key);
  }
  const nodes = new Map(model.nodes.map((node) => [node.id, node.position] as const));
  const pipes = new Map(model.pipe_segments.map((pipe) => [pipe.id, pipe] as const));
  const normalized = normalizeBoxRect(options.rect);
  const acceptsType = (type: EntityRef["type"]) =>
    options.filter === "all" || options.filter === `${type}s`;
  const hits: EntityKey[] = [];

  for (const key of index.treeOrder) {
    if (!candidateKeys.has(key) || options.hiddenKeys?.has(key) || index.invalidGeometry.has(key)) continue;
    const indexed = index.entities.get(key);
    if (!indexed || !acceptsType(indexed.ref.type)) continue;
    if (indexed.ref.type === "pipe") {
      const pipe = pipes.get(indexed.ref.id);
      const from = pipe ? nodes.get(pipe.from) : null;
      const to = pipe ? nodes.get(pipe.to) : null;
      if (!from || !to) continue;
      const clipped = clipSegmentToClosedNdc(
        clipPoint(authoredToLocal(from, options.renderOrigin), viewProjection),
        clipPoint(authoredToLocal(to, options.renderOrigin), viewProjection)
      );
      if (!clipped) continue;
      const a = clipToCanvasPoint(clipped[0], canvasRect.width, canvasRect.height);
      const b = clipToCanvasPoint(clipped[1], canvasRect.width, canvasRect.height);
      const hit = options.direction === "left-to-right"
        ? pointInClosedRect(a, normalized) && pointInClosedRect(b, normalized)
        : segmentIntersectsClosedRect(a, b, normalized);
      if (hit) hits.push(key);
      continue;
    }
    if (!["node", "support", "component"].includes(indexed.ref.type) || !indexed.anchor) continue;
    const clip = clipPoint(authoredToLocal(indexed.anchor, options.renderOrigin), viewProjection);
    if (!insideClosedClip(clip)) continue;
    if (pointInClosedRect(clipToCanvasPoint(clip, canvasRect.width, canvasRect.height), normalized)) hits.push(key);
  }
  return Object.freeze(hits);
}

export type PointPickPrimitive = Readonly<{
  key: EntityKey;
  ref: EntityRef;
  kind: "pipe" | "node" | "support" | "component";
  shape: "capsule" | "sphere" | "cone";
  center: Readonly<Vec3>;
  start: Readonly<Vec3> | null;
  end: Readonly<Vec3> | null;
  radius: number;
  height: number;
  displayOrder: number;
}>;

export type PointPickOptions = Readonly<{
  index: ModelIndex;
  renderOrigin: Readonly<Vec3>;
  camera: THREE.PerspectiveCamera;
  canvas: HTMLCanvasElement;
  clientX: number;
  clientY: number;
  hiddenKeys?: ReadonlySet<EntityKey>;
  actualOdRadiusByPipe?: ReadonlyMap<EntityKey, number>;
}>;

export function createRenderTransform(bounds: Bounds3 | null): RenderTransform {
  if (!bounds) return Object.freeze({ origin: freezeVec({ x: 0, y: 0, z: 0 }), localBounds: null });
  if (!finiteVec(bounds.min) || !finiteVec(bounds.max)) {
    throw new Error("Viewport bounds contain a non-finite authored coordinate.");
  }
  const origin = freezeVec({
    x: safeMidpoint(bounds.min.x, bounds.max.x),
    y: safeMidpoint(bounds.min.y, bounds.max.y),
    z: safeMidpoint(bounds.min.z, bounds.max.z)
  });
  const localMin = authoredToLocal(bounds.min, origin);
  const localMax = authoredToLocal(bounds.max, origin);
  if (!finiteVec(localMin) || !finiteVec(localMax)) {
    throw new Error("Viewport bounds cannot be represented relative to their render origin.");
  }
  return Object.freeze({
    origin,
    localBounds: Object.freeze({ min: freezeVec(localMin), max: freezeVec(localMax) })
  });
}

export function authoredToLocal(authored: Readonly<Vec3>, origin: Readonly<Vec3>): Vec3 {
  const local = {
    x: authored.x - origin.x,
    y: authored.y - origin.y,
    z: authored.z - origin.z
  };
  if (!finiteVec(authored) || !finiteVec(origin) || !finiteVec(local)) {
    throw new Error("Authored coordinate cannot be represented in the viewport.");
  }
  return local;
}

export function localToAuthored(local: Readonly<Vec3>, origin: Readonly<Vec3>): Vec3 {
  const authored = {
    x: local.x + origin.x,
    y: local.y + origin.y,
    z: local.z + origin.z
  };
  if (!finiteVec(local) || !finiteVec(origin) || !finiteVec(authored)) {
    throw new Error("Viewport coordinate cannot be mapped back to an authored coordinate.");
  }
  return authored;
}

export function localPointIsFloat32Representable(local: Readonly<Vec3>): boolean {
  if (!finiteVec(local)) return false;
  return [local.x, local.y, local.z].every((value) => {
    const presented = Math.fround(value);
    if (!Number.isFinite(presented)) return false;
    const tolerance = Math.max(1e-6, Math.abs(value) * 1e-6);
    return Math.abs(presented - value) <= tolerance;
  });
}

export function broadPhaseEntityKeys(index: ModelIndex, query: Bounds3): readonly EntityKey[] {
  if (!finiteVec(query.min) || !finiteVec(query.max)) return Object.freeze([]);
  const keys: EntityKey[] = [];
  for (const chunk of index.spatialChunks) {
    if (!boundsIntersect(chunk.bounds, query)) continue;
    for (const key of chunk.entityKeys) {
      const bounds = index.entities.get(key)?.geometryBounds;
      if (bounds && boundsIntersect(bounds, query)) keys.push(key);
    }
  }
  return Object.freeze(keys);
}

export function pointPickPrimitives(
  index: ModelIndex,
  model: PreviewModel,
  origin: Readonly<Vec3>
): readonly PointPickPrimitive[] {
  const order = new Map(index.treeOrder.map((key, position) => [key, position] as const));
  const nodes = new Map(model.nodes.map((node) => [node.id, node.position] as const));
  const primitives: PointPickPrimitive[] = [];
  for (const pipe of model.pipe_segments) {
    const key = entityKeyFor("pipe", pipe.id);
    const from = nodes.get(pipe.from);
    const to = nodes.get(pipe.to);
    if (!from || !to || index.invalidGeometry.has(key)) continue;
    primitives.push(freezePrimitive({
      key,
      ref: { type: "pipe", id: pipe.id },
      kind: "pipe",
      shape: "capsule",
      center: authoredToLocal(midpoint(from, to), origin),
      start: authoredToLocal(from, origin),
      end: authoredToLocal(to, origin),
      radius: 0.052,
      height: 0,
      displayOrder: order.get(key) ?? Number.MAX_SAFE_INTEGER
    }));
  }
  for (const node of model.nodes) {
    const key = entityKeyFor("node", node.id);
    if (index.invalidGeometry.has(key)) continue;
    primitives.push(freezePrimitive({
      key,
      ref: { type: "node", id: node.id },
      kind: "node",
      shape: "sphere",
      center: authoredToLocal(node.position, origin),
      start: null,
      end: null,
      radius: 0.095,
      height: 0,
      displayOrder: order.get(key) ?? Number.MAX_SAFE_INTEGER
    }));
  }
  for (const support of model.supports) {
    const key = entityKeyFor("support", support.id);
    const node = nodes.get(support.node);
    if (!node || index.invalidGeometry.has(key)) continue;
    primitives.push(freezePrimitive({
      key,
      ref: { type: "support", id: support.id },
      kind: "support",
      shape: "cone",
      center: authoredToLocal({ x: node.x, y: node.y - 0.26, z: node.z }, origin),
      start: null,
      end: null,
      radius: 0.18,
      height: 0.34,
      displayOrder: order.get(key) ?? Number.MAX_SAFE_INTEGER
    }));
  }
  for (const component of model.components) {
    const key = entityKeyFor("component", component.id);
    const node = nodes.get(component.node);
    if (!node || index.invalidGeometry.has(key)) continue;
    primitives.push(freezePrimitive({
      key,
      ref: { type: "component", id: component.id },
      kind: "component",
      shape: "sphere",
      center: authoredToLocal({ x: node.x, y: node.y + 0.2, z: node.z }, origin),
      start: null,
      end: null,
      radius: componentPickExtent(component),
      height: 0,
      displayOrder: order.get(key) ?? Number.MAX_SAFE_INTEGER
    }));
  }
  return Object.freeze(primitives);
}

export function pickPointPrimitive(
  primitives: readonly PointPickPrimitive[],
  options: PointPickOptions
): EntityRef | null {
  const rect = options.canvas.getBoundingClientRect();
  const width = rect.width || options.canvas.clientWidth;
  const height = rect.height || options.canvas.clientHeight;
  if (!(width > 0) || !(height > 0)) return null;
  const pointer = new THREE.Vector2(
    ((options.clientX - rect.left) / width) * 2 - 1,
    -(((options.clientY - rect.top) / height) * 2 - 1)
  );
  if (!Number.isFinite(pointer.x) || !Number.isFinite(pointer.y) || Math.abs(pointer.x) > 1 || Math.abs(pointer.y) > 1) {
    return null;
  }
  const raycaster = new THREE.Raycaster();
  options.camera.updateMatrixWorld();
  raycaster.setFromCamera(pointer, options.camera);
  const cameraForward = options.camera.getWorldDirection(new THREE.Vector3()).normalize();
  const cosTheta = raycaster.ray.direction.dot(cameraForward);
  if (!(cosTheta > 0) || !Number.isFinite(cosTheta)) return null;
  const tNear = options.camera.near / cosTheta;
  const tFar = options.camera.far / cosTheta;
  if (![tNear, tFar].every(Number.isFinite) || tNear < 0 || tFar < tNear) return null;
  const farTolerance = cssPixelsInWorld(6, options.camera.far, options.camera, height);
  let largestActualOdRadius = 0;
  for (const radius of options.actualOdRadiusByPipe?.values() ?? []) {
    if (Number.isFinite(radius)) largestActualOdRadius = Math.max(largestActualOdRadius, radius);
  }
  const broadPhaseKeys = pointRayBroadPhaseKeys(
    options.index,
    options.renderOrigin,
    raycaster.ray,
    tNear,
    tFar,
    Math.max(0.5, farTolerance, largestActualOdRadius)
  );
  const candidates: Array<{ primitive: PointPickPrimitive; entry: number; miss: number }> = [];
  for (const primitive of primitives) {
    if (!broadPhaseKeys.has(primitive.key)) continue;
    if (options.hiddenKeys?.has(primitive.key)) continue;
    const depth = primitiveDepth(primitive, raycaster.ray, options.camera);
    if (!(depth > 0) || !Number.isFinite(depth)) continue;
    const pixelRadius = cssPixelsInWorld(6, depth, options.camera, height);
    const requestedRadius = primitive.kind === "pipe"
      ? Math.max(primitive.radius, options.actualOdRadiusByPipe?.get(primitive.key) ?? 0, pixelRadius)
      : Math.max(primitive.radius, pixelRadius);
    if (!primitiveBoundsIntersectRay(primitive, requestedRadius, raycaster.ray, tNear, tFar)) continue;
    const hit = primitive.shape === "capsule" && primitive.start && primitive.end
      ? intersectCapsule(raycaster.ray, primitive.start, primitive.end, requestedRadius)
      : primitive.shape === "cone"
        ? intersectVerticalCone(raycaster.ray, primitive.center, requestedRadius, primitive.height)
        : intersectSphere(raycaster.ray, primitive.center, requestedRadius);
    if (!hit || hit.exit < tNear || hit.entry > tFar) continue;
    const clippedEntry = Math.max(hit.entry, tNear);
    const miss = primitive.shape === "cone"
      ? coneNormalizedMissAt(raycaster.ray, primitive.center, requestedRadius, clippedEntry)
      : hit.miss;
    candidates.push({ primitive, entry: clippedEntry, miss });
  }
  if (candidates.length === 0) return null;
  const tMin = Math.min(...candidates.map((candidate) => candidate.entry));
  const front = candidates.filter((candidate) =>
    Math.abs(candidate.entry - tMin) <= 1e-9 * Math.max(1, Math.abs(tMin), Math.abs(candidate.entry))
  );
  const mMin = Math.min(...front.map((candidate) => candidate.miss));
  const missGroup = front.filter((candidate) => Math.abs(candidate.miss - mMin) <= 1e-12);
  missGroup.sort(compareStablePointHit);
  return missGroup[0]?.primitive.ref ?? null;
}

export function prioritizedLabelKeys(index: ModelIndex, input: LabelPriorityInput): readonly EntityKey[] {
  const limit = Math.max(0, Math.min(80, Math.floor(input.limit ?? 80)));
  if (limit === 0) return Object.freeze([]);
  const hidden = input.hiddenKeys ?? EMPTY_KEYS;
  const output: EntityKey[] = [];
  const seen = new Set<EntityKey>();
  const include = (key: EntityKey | null | undefined) => {
    if (output.length >= limit || !key || seen.has(key) || hidden.has(key)) return;
    const entity = index.entities.get(key);
    if (!entity?.anchor || entity.geometryIssue) return;
    seen.add(key);
    output.push(key);
  };

  include(input.primaryKey);
  include(input.hoverKey);
  for (const key of input.selectedKeys ?? []) {
    if (output.length >= limit) break;
    include(key);
  }
  for (const key of input.diagnosticKeys ?? []) {
    if (output.length >= limit) break;
    include(key);
  }

  const cameraTarget = input.cameraTarget ?? boundsCenter(index.geometryBounds);
  const context = input.cameraTarget
    ? sortedLabelContext(index, cameraTarget)
    : defaultLabelContext(index);
  for (const key of context) {
    if (output.length >= limit) break;
    include(key);
  }
  return Object.freeze(output.slice(0, limit));
}

const DEFAULT_LABEL_CONTEXT = new WeakMap<ModelIndex, readonly EntityKey[]>();

function defaultLabelContext(index: ModelIndex): readonly EntityKey[] {
  const cached = DEFAULT_LABEL_CONTEXT.get(index);
  if (cached) return cached;
  const sorted = Object.freeze(sortedLabelContext(index, boundsCenter(index.geometryBounds)));
  DEFAULT_LABEL_CONTEXT.set(index, sorted);
  return sorted;
}

function sortedLabelContext(index: ModelIndex, cameraTarget: Readonly<Vec3>): EntityKey[] {
  return [...index.visibilityEligibleKeys]
    .filter((key) => Boolean(index.entities.get(key)?.anchor))
    .sort((left, right) => {
      const leftAnchor = index.entities.get(left)?.anchor;
      const rightAnchor = index.entities.get(right)?.anchor;
      if (!leftAnchor || !rightAnchor) return left.localeCompare(right);
      const distanceDelta = distanceSquared(leftAnchor, cameraTarget) - distanceSquared(rightAnchor, cameraTarget);
      return distanceDelta || left.localeCompare(right);
    });
}

export function localBounds(authored: Bounds3, origin: Readonly<Vec3>): Bounds3 {
  return Object.freeze({
    min: freezeVec(authoredToLocal(authored.min, origin)),
    max: freezeVec(authoredToLocal(authored.max, origin))
  });
}

function boundsCenter(bounds: Bounds3 | null): Vec3 {
  if (!bounds) return { x: 0, y: 0, z: 0 };
  return {
    x: safeMidpoint(bounds.min.x, bounds.max.x),
    y: safeMidpoint(bounds.min.y, bounds.max.y),
    z: safeMidpoint(bounds.min.z, bounds.max.z)
  };
}

function safeMidpoint(min: number, max: number): number {
  return min + (max - min) / 2;
}

function distanceSquared(a: Readonly<Vec3>, b: Readonly<Vec3>): number {
  const x = a.x - b.x;
  const y = a.y - b.y;
  const z = a.z - b.z;
  return x * x + y * y + z * z;
}

function midpoint(a: Readonly<Vec3>, b: Readonly<Vec3>): Vec3 {
  return {
    x: safeMidpoint(a.x, b.x),
    y: safeMidpoint(a.y, b.y),
    z: safeMidpoint(a.z, b.z)
  };
}

function finiteVec(value: Readonly<Vec3>): boolean {
  return Number.isFinite(value.x) && Number.isFinite(value.y) && Number.isFinite(value.z);
}

function freezeVec(value: Readonly<Vec3>): Readonly<Vec3> {
  return Object.freeze({ x: value.x, y: value.y, z: value.z });
}

const EMPTY_KEYS: ReadonlySet<EntityKey> = new Set();

function freezePrimitive(primitive: PointPickPrimitive): PointPickPrimitive {
  return Object.freeze({
    ...primitive,
    ref: Object.freeze({ ...primitive.ref }),
    center: freezeVec(primitive.center),
    start: primitive.start ? freezeVec(primitive.start) : null,
    end: primitive.end ? freezeVec(primitive.end) : null
  });
}

function entityKeyFor(type: EntityRef["type"], id: string): EntityKey {
  return JSON.stringify([type, id]) as EntityKey;
}

function componentPickExtent(component: PreviewComponent): number {
  if (["bend", "elbow", "branch", "tee", "branch_connection"].includes(component.kind)) return 0.3;
  if (["valve", "flange", "reducer", "rigid", "specialty", "expansion_joint"].includes(component.kind)) return 0.25;
  return Math.hypot(0.12, 0.12, 0.12);
}

function cssPixelsInWorld(
  pixels: number,
  depth: number,
  camera: THREE.PerspectiveCamera,
  viewportHeight: number
): number {
  return pixels * (2 * depth * Math.tan(THREE.MathUtils.degToRad(camera.fov) / 2)) / viewportHeight;
}

function intersectSphere(
  ray: THREE.Ray,
  center: Readonly<Vec3>,
  radius: number
): { entry: number; exit: number; miss: number } | null {
  const toCenter = new THREE.Vector3(center.x, center.y, center.z).sub(ray.origin);
  const along = toCenter.dot(ray.direction);
  const missSquared = Math.max(0, toCenter.lengthSq() - along * along);
  if (missSquared > radius * radius) return null;
  const halfSpan = Math.sqrt(radius * radius - missSquared);
  return { entry: along - halfSpan, exit: along + halfSpan, miss: Math.sqrt(missSquared) / radius };
}

function intersectCapsule(
  ray: THREE.Ray,
  start: Readonly<Vec3>,
  end: Readonly<Vec3>,
  radius: number
): { entry: number; exit: number; miss: number } | null {
  const a = new THREE.Vector3(start.x, start.y, start.z);
  const b = new THREE.Vector3(end.x, end.y, end.z);
  const axis = b.clone().sub(a);
  const axisLengthSquared = axis.lengthSq();
  if (!(axisLengthSquared > 0)) return intersectSphere(ray, start, radius);
  const originFromA = ray.origin.clone().sub(a);
  const axisRay = axis.dot(ray.direction);
  const axisOrigin = axis.dot(originFromA);
  const qa = axisLengthSquared - axisRay * axisRay;
  const qb = axisLengthSquared * originFromA.dot(ray.direction) - axisOrigin * axisRay;
  const qc = axisLengthSquared * originFromA.lengthSq() - axisOrigin * axisOrigin - radius * radius * axisLengthSquared;
  const intersections: number[] = [];
  if (Math.abs(qa) > Number.EPSILON) {
    const discriminant = qb * qb - qa * qc;
    if (discriminant >= 0) {
      const root = Math.sqrt(discriminant);
      for (const value of [(-qb - root) / qa, (-qb + root) / qa]) {
        const axial = axisOrigin + value * axisRay;
        if (axial >= 0 && axial <= axisLengthSquared) intersections.push(value);
      }
    }
  }
  for (const center of [a, b]) {
    const cap = sphereInterval(ray, center, radius);
    if (cap) intersections.push(cap.entry, cap.exit);
  }
  if (pointInsideCapsule(ray.origin, a, b, radius)) intersections.push(0);
  const finite = intersections.filter(Number.isFinite).sort((left, right) => left - right);
  if (finite.length === 0) return null;
  const pointOnRay = new THREE.Vector3();
  const pointOnSegment = new THREE.Vector3();
  const missSquared = ray.distanceSqToSegment(a, b, pointOnRay, pointOnSegment);
  return {
    entry: finite[0],
    exit: finite.at(-1)!,
    miss: Math.sqrt(Math.max(0, missSquared)) / radius
  };
}

function intersectVerticalCone(
  ray: THREE.Ray,
  center: Readonly<Vec3>,
  radius: number,
  height: number
): { entry: number; exit: number; miss: number } | null {
  const tip = new THREE.Vector3(center.x, center.y + height / 2, center.z);
  const q = ray.origin.clone().sub(tip);
  const axis = new THREE.Vector3(0, -1, 0);
  const directionAlong = ray.direction.dot(axis);
  const originAlong = q.dot(axis);
  const directionPerpendicular = ray.direction.clone().addScaledVector(axis, -directionAlong);
  const originPerpendicular = q.clone().addScaledVector(axis, -originAlong);
  const slopeSquared = (radius / height) ** 2;
  const a = directionPerpendicular.lengthSq() - slopeSquared * directionAlong * directionAlong;
  const b = 2 * (directionPerpendicular.dot(originPerpendicular) - slopeSquared * directionAlong * originAlong);
  const c = originPerpendicular.lengthSq() - slopeSquared * originAlong * originAlong;
  const roots = quadraticRoots(a, b, c);
  const sides = roots.filter((value) => {
    const axial = originAlong + value * directionAlong;
    return value >= 0 && axial >= 0 && axial <= height;
  });
  const baseY = center.y - height / 2;
  const baseT = Math.abs(ray.direction.y) <= Number.EPSILON ? null : (baseY - ray.origin.y) / ray.direction.y;
  const base = baseT !== null && baseT >= 0 &&
    Math.hypot(ray.origin.x + ray.direction.x * baseT - center.x, ray.origin.z + ray.direction.z * baseT - center.z) <= radius
    ? baseT
    : null;
  const intersections = [...sides, base]
    .filter((value): value is number => value !== null && Number.isFinite(value))
    .sort((x, y) => x - y);
  const entry = intersections[0];
  const exit = intersections.at(-1);
  if (entry === undefined || exit === undefined) return null;
  const point = ray.at(Math.max(0, entry), new THREE.Vector3());
  const miss = Math.hypot(point.x - center.x, point.z - center.z) / radius;
  return { entry, exit, miss };
}

function quadraticRoots(a: number, b: number, c: number): number[] {
  if (Math.abs(a) <= Number.EPSILON) return Math.abs(b) <= Number.EPSILON ? [] : [-c / b];
  const discriminant = b * b - 4 * a * c;
  if (discriminant < 0) return [];
  const root = Math.sqrt(discriminant);
  return [(-b - root) / (2 * a), (-b + root) / (2 * a)].sort((x, y) => x - y);
}

function compareStablePointHit(
  left: { primitive: PointPickPrimitive },
  right: { primitive: PointPickPrimitive }
): number {
  const typeDelta = PICK_TYPE_ORDER[left.primitive.kind] - PICK_TYPE_ORDER[right.primitive.kind];
  if (typeDelta) return typeDelta;
  const orderDelta = left.primitive.displayOrder - right.primitive.displayOrder;
  return orderDelta || left.primitive.key.localeCompare(right.primitive.key);
}

const PICK_TYPE_ORDER: Readonly<Record<PointPickPrimitive["kind"], number>> = Object.freeze({
  component: 0,
  support: 1,
  node: 2,
  pipe: 3
});

function primitiveDepth(
  primitive: PointPickPrimitive,
  ray: THREE.Ray,
  camera: THREE.PerspectiveCamera
): number {
  const point = primitive.shape === "capsule" && primitive.start && primitive.end
    ? closestPointOnSegmentToRay(primitive.start, primitive.end, ray)
    : new THREE.Vector3(primitive.center.x, primitive.center.y, primitive.center.z);
  return -point.applyMatrix4(camera.matrixWorldInverse).z;
}

function closestPointOnSegmentToRay(
  start: Readonly<Vec3>,
  end: Readonly<Vec3>,
  ray: THREE.Ray
): THREE.Vector3 {
  const a = new THREE.Vector3(start.x, start.y, start.z);
  const segment = new THREE.Vector3(end.x - start.x, end.y - start.y, end.z - start.z);
  const lengthSquared = segment.lengthSq();
  if (!(lengthSquared > 0)) return a;
  const fromStart = ray.origin.clone().sub(a);
  const raySegment = ray.direction.dot(segment);
  const rayStart = ray.direction.dot(fromStart);
  const segmentStart = segment.dot(fromStart);
  const denominator = lengthSquared - raySegment * raySegment;
  const fraction = denominator > Number.EPSILON
    ? THREE.MathUtils.clamp((raySegment * rayStart - segmentStart) / denominator, 0, 1)
    : 0;
  return a.addScaledVector(segment, fraction);
}

function pointRayBroadPhaseKeys(
  index: ModelIndex,
  origin: Readonly<Vec3>,
  ray: THREE.Ray,
  near: number,
  far: number,
  tolerance: number
): ReadonlySet<EntityKey> {
  const keys = new Set<EntityKey>();
  for (const chunk of index.spatialChunks) {
    const local = localBounds(chunk.bounds, origin);
    const box = new THREE.Box3(
      new THREE.Vector3(local.min.x, local.min.y, local.min.z),
      new THREE.Vector3(local.max.x, local.max.y, local.max.z)
    ).expandByScalar(tolerance);
    const interval = rayBoxInterval(ray, box);
    if (!interval || interval.exit < near || interval.entry > far) continue;
    for (const key of chunk.entityKeys) keys.add(key);
  }
  return keys;
}

function primitiveBoundsIntersectRay(
  primitive: PointPickPrimitive,
  radius: number,
  ray: THREE.Ray,
  near: number,
  far: number
): boolean {
  let min: THREE.Vector3;
  let max: THREE.Vector3;
  if (primitive.shape === "capsule" && primitive.start && primitive.end) {
    min = new THREE.Vector3(
      Math.min(primitive.start.x, primitive.end.x) - radius,
      Math.min(primitive.start.y, primitive.end.y) - radius,
      Math.min(primitive.start.z, primitive.end.z) - radius
    );
    max = new THREE.Vector3(
      Math.max(primitive.start.x, primitive.end.x) + radius,
      Math.max(primitive.start.y, primitive.end.y) + radius,
      Math.max(primitive.start.z, primitive.end.z) + radius
    );
  } else if (primitive.shape === "cone") {
    min = new THREE.Vector3(primitive.center.x - radius, primitive.center.y - primitive.height / 2, primitive.center.z - radius);
    max = new THREE.Vector3(primitive.center.x + radius, primitive.center.y + primitive.height / 2, primitive.center.z + radius);
  } else {
    min = new THREE.Vector3(primitive.center.x - radius, primitive.center.y - radius, primitive.center.z - radius);
    max = new THREE.Vector3(primitive.center.x + radius, primitive.center.y + radius, primitive.center.z + radius);
  }
  const interval = rayBoxInterval(ray, new THREE.Box3(min, max));
  return Boolean(interval && interval.exit >= near && interval.entry <= far);
}

function rayBoxInterval(ray: THREE.Ray, box: THREE.Box3): { entry: number; exit: number } | null {
  let entry = Number.NEGATIVE_INFINITY;
  let exit = Number.POSITIVE_INFINITY;
  for (const axis of ["x", "y", "z"] as const) {
    const direction = ray.direction[axis];
    const origin = ray.origin[axis];
    if (Math.abs(direction) <= Number.EPSILON) {
      if (origin < box.min[axis] || origin > box.max[axis]) return null;
      continue;
    }
    const first = (box.min[axis] - origin) / direction;
    const second = (box.max[axis] - origin) / direction;
    entry = Math.max(entry, Math.min(first, second));
    exit = Math.min(exit, Math.max(first, second));
    if (exit < entry) return null;
  }
  return { entry, exit };
}

function sphereInterval(ray: THREE.Ray, center: THREE.Vector3, radius: number): { entry: number; exit: number } | null {
  const hit = intersectSphere(ray, center, radius);
  return hit ? { entry: hit.entry, exit: hit.exit } : null;
}

function pointInsideCapsule(point: THREE.Vector3, start: THREE.Vector3, end: THREE.Vector3, radius: number): boolean {
  const segment = end.clone().sub(start);
  const lengthSquared = segment.lengthSq();
  const fraction = lengthSquared > 0
    ? THREE.MathUtils.clamp(point.clone().sub(start).dot(segment) / lengthSquared, 0, 1)
    : 0;
  return point.distanceTo(start.clone().addScaledVector(segment, fraction)) <= radius;
}

function coneNormalizedMissAt(
  ray: THREE.Ray,
  center: Readonly<Vec3>,
  radius: number,
  entry: number
): number {
  const point = ray.at(entry, new THREE.Vector3());
  return Math.hypot(point.x - center.x, point.z - center.z) / radius;
}

type ClipPoint = Readonly<{ x: number; y: number; z: number; w: number }>;
type ScreenPoint = Readonly<{ x: number; y: number }>;

function clipPoint(point: Readonly<Vec3>, matrix: THREE.Matrix4): ClipPoint {
  const e = matrix.elements;
  const { x, y, z } = point;
  return {
    x: e[0] * x + e[4] * y + e[8] * z + e[12],
    y: e[1] * x + e[5] * y + e[9] * z + e[13],
    z: e[2] * x + e[6] * y + e[10] * z + e[14],
    w: e[3] * x + e[7] * y + e[11] * z + e[15]
  };
}

function insideClosedClip(point: ClipPoint): boolean {
  return Number.isFinite(point.w) && point.w > 0 &&
    point.x >= -point.w && point.x <= point.w && point.y >= -point.w && point.y <= point.w &&
    point.z >= -point.w && point.z <= point.w;
}

function clipSegmentToClosedNdc(a: ClipPoint, b: ClipPoint): readonly [ClipPoint, ClipPoint] | null {
  if (![a.x, a.y, a.z, a.w, b.x, b.y, b.z, b.w].every(Number.isFinite)) return null;
  let enter = 0;
  let exit = 1;
  const planes = [
    (p: ClipPoint) => p.x + p.w, (p: ClipPoint) => p.w - p.x,
    (p: ClipPoint) => p.y + p.w, (p: ClipPoint) => p.w - p.y,
    (p: ClipPoint) => p.z + p.w, (p: ClipPoint) => p.w - p.z
  ];
  for (const plane of planes) {
    const fa = plane(a);
    const fb = plane(b);
    if (fa < 0 && fb < 0) return null;
    if (fa >= 0 && fb >= 0) continue;
    const t = fa / (fa - fb);
    if (fa < 0) enter = Math.max(enter, t);
    else exit = Math.min(exit, t);
    if (enter > exit) return null;
  }
  const lerp = (t: number): ClipPoint => ({
    x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t,
    z: a.z + (b.z - a.z) * t, w: a.w + (b.w - a.w) * t
  });
  const start = lerp(enter);
  const end = lerp(exit);
  return start.w > 0 && end.w > 0 ? Object.freeze([start, end] as const) : null;
}

function clipToCanvasPoint(point: ClipPoint, width: number, height: number): ScreenPoint {
  return { x: (point.x / point.w * 0.5 + 0.5) * width, y: (-point.y / point.w * 0.5 + 0.5) * height };
}

function normalizeBoxRect(rect: BoxSelectionRect): BoxSelectionRect {
  return { left: Math.min(rect.left, rect.right), right: Math.max(rect.left, rect.right), top: Math.min(rect.top, rect.bottom), bottom: Math.max(rect.top, rect.bottom) };
}

function pointInClosedRect(point: ScreenPoint, rect: BoxSelectionRect): boolean {
  return point.x >= rect.left && point.x <= rect.right && point.y >= rect.top && point.y <= rect.bottom;
}

function segmentIntersectsClosedRect(a: ScreenPoint, b: ScreenPoint, rect: BoxSelectionRect): boolean {
  if (pointInClosedRect(a, rect) || pointInClosedRect(b, rect)) return true;
  let enter = 0;
  let exit = 1;
  const dx = b.x - a.x;
  const dy = b.y - a.y;
  for (const [p, q] of [[-dx, a.x - rect.left], [dx, rect.right - a.x], [-dy, a.y - rect.top], [dy, rect.bottom - a.y]] as const) {
    if (p === 0 && q < 0) return false;
    if (p === 0) continue;
    const t = q / p;
    if (p < 0) enter = Math.max(enter, t);
    else exit = Math.min(exit, t);
    if (enter > exit) return false;
  }
  return true;
}
