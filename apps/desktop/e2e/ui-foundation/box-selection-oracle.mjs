import { cameraBasis } from "./point-hit-oracle.mjs";

const EPS = 1e-12;
const dot = (a, b) => a.reduce((sum, value, index) => sum + value * b[index], 0);
const sub = (a, b) => a.map((value, index) => value - b[index]);
const add = (a, b) => a.map((value, index) => value + b[index]);
const scale = (a, factor) => a.map((value) => value * factor);
const xyz = (value) => [Number(value.x), Number(value.y), Number(value.z)];
const finite = (value) => value.length === 3 && value.every(Number.isFinite);

function cameraPoint(basis, point) {
  const offset = sub(point, basis.position);
  return [dot(offset, basis.right), dot(offset, basis.up), dot(offset, basis.forward)];
}

function planeValues(point, tangent, aspect, near, far) {
  const [x, y, z] = point;
  return [z - near, far - z, x + z * tangent * aspect, z * tangent * aspect - x, y + z * tangent, z * tangent - y];
}

export function clipAuthoredSegmentToClosedFrustum(camera, authoredStart, authoredEnd) {
  const basis = cameraBasis(camera);
  const start = cameraPoint(basis, authoredStart);
  const end = cameraPoint(basis, authoredEnd);
  const tangent = Math.tan(basis.verticalFovRadians / 2);
  const startPlanes = planeValues(start, tangent, basis.aspect, basis.near, basis.far);
  const endPlanes = planeValues(end, tangent, basis.aspect, basis.near, basis.far);
  let low = 0;
  let high = 1;
  for (let index = 0; index < startPlanes.length; index += 1) {
    const atStart = startPlanes[index];
    const delta = endPlanes[index] - atStart;
    if (Math.abs(delta) <= EPS) {
      if (atStart < -EPS) return null;
      continue;
    }
    const crossing = -atStart / delta;
    if (delta > 0) low = Math.max(low, crossing);
    else high = Math.min(high, crossing);
    if (low > high + EPS) return null;
  }
  const interpolate = (fraction) => add(start, scale(sub(end, start), fraction));
  return { start: interpolate(Math.max(0, low)), end: interpolate(Math.min(1, high)), interval: [Math.max(0, low), Math.min(1, high)] };
}

function cameraPointToNormalized(point, camera) {
  const tangent = Math.tan(Number(camera.fovDegrees) * Math.PI / 360);
  const ndcX = point[0] / (point[2] * tangent * Number(camera.aspect));
  const ndcY = point[1] / (point[2] * tangent);
  return [(ndcX + 1) / 2, (1 - ndcY) / 2];
}

function pointInside(point, rectangle) {
  return point[0] >= rectangle.left - EPS && point[0] <= rectangle.right + EPS &&
    point[1] >= rectangle.top - EPS && point[1] <= rectangle.bottom + EPS;
}

function segmentIntersectsRectangle(start, end, rectangle) {
  if (pointInside(start, rectangle) || pointInside(end, rectangle)) return true;
  const delta = sub(end, start);
  let low = 0;
  let high = 1;
  for (const [p, q] of [
    [-delta[0], start[0] - rectangle.left],
    [delta[0], rectangle.right - start[0]],
    [-delta[1], start[1] - rectangle.top],
    [delta[1], rectangle.bottom - start[1]]
  ]) {
    if (Math.abs(p) <= EPS) {
      if (q < -EPS) return false;
      continue;
    }
    const ratio = q / p;
    if (p < 0) low = Math.max(low, ratio);
    else high = Math.min(high, ratio);
    if (low > high + EPS) return false;
  }
  return true;
}

function pointInClosedFrustum(camera, authoredPoint) {
  const basis = cameraBasis(camera);
  const cameraSpace = cameraPoint(basis, authoredPoint);
  const tangent = Math.tan(basis.verticalFovRadians / 2);
  return planeValues(cameraSpace, tangent, basis.aspect, basis.near, basis.far).every((value) => value >= -EPS)
    ? cameraSpace : null;
}

function candidateEntries(model) {
  const nodesById = new Map(model.nodes.map((node) => [node.id, node]));
  const entries = [];
  for (const node of model.nodes) {
    const point = xyz(node.position);
    entries.push({ ref: { type: "node", id: node.id }, kind: "point", valid: finite(point), point });
  }
  for (const pipe of model.pipe_segments) {
    const from = xyz(nodesById.get(pipe.from)?.position ?? {});
    const to = xyz(nodesById.get(pipe.to)?.position ?? {});
    const valid = finite(from) && finite(to) && Math.hypot(...sub(to, from)) > 0;
    entries.push({ ref: { type: "pipe", id: pipe.id }, kind: "segment", valid, from, to });
  }
  for (const support of model.supports) {
    const point = xyz(nodesById.get(support.node)?.position ?? {});
    entries.push({ ref: { type: "support", id: support.id }, kind: "point", valid: finite(point), point });
  }
  for (const component of model.components) {
    const point = xyz(nodesById.get(component.node)?.position ?? {});
    entries.push({ ref: { type: "component", id: component.id }, kind: "point", valid: finite(point), point });
  }
  return entries;
}

export function expectedBoxSelection(model, camera, sample, hiddenRefs = []) {
  const hidden = new Set(hiddenRefs.map((ref) => JSON.stringify([ref.type, ref.id])));
  const rectangle = {
    left: Math.min(sample.start_normalized[0], sample.end_normalized[0]),
    right: Math.max(sample.start_normalized[0], sample.end_normalized[0]),
    top: Math.min(sample.start_normalized[1], sample.end_normalized[1]),
    bottom: Math.max(sample.start_normalized[1], sample.end_normalized[1])
  };
  const allowedType = sample.filter === "all" ? null : sample.filter.slice(0, -1);
  const direction = sample.end_normalized[0] >= sample.start_normalized[0] ? "left-to-right" : "right-to-left";
  const hits = [];
  for (const entry of candidateEntries(model)) {
    if (!entry.valid || (allowedType && entry.ref.type !== allowedType) || hidden.has(JSON.stringify([entry.ref.type, entry.ref.id]))) continue;
    if (entry.kind === "point") {
      const cameraSpace = pointInClosedFrustum(camera, entry.point);
      if (cameraSpace && pointInside(cameraPointToNormalized(cameraSpace, camera), rectangle)) hits.push(entry.ref);
      continue;
    }
    const clipped = clipAuthoredSegmentToClosedFrustum(camera, entry.from, entry.to);
    if (!clipped) continue;
    const screenStart = cameraPointToNormalized(clipped.start, camera);
    const screenEnd = cameraPointToNormalized(clipped.end, camera);
    const selected = direction === "left-to-right"
      ? pointInside(screenStart, rectangle) && pointInside(screenEnd, rectangle)
      : segmentIntersectsRectangle(screenStart, screenEnd, rectangle);
    if (selected) hits.push(entry.ref);
  }
  return {
    sample: sample.sample,
    direction,
    filter: sample.filter,
    rectangle,
    orderedRefs: hits,
    primaryRef: hits.at(-1) ?? null,
    membershipCount: hits.length
  };
}
