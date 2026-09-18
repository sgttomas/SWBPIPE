const EPS = 1e-12;

const add = (a, b) => a.map((value, index) => value + b[index]);
const sub = (a, b) => a.map((value, index) => value - b[index]);
const scale = (a, factor) => a.map((value) => value * factor);
const dot = (a, b) => a.reduce((sum, value, index) => sum + value * b[index], 0);
const length = (a) => Math.sqrt(dot(a, a));
const cross = (a, b) => [
  a[1] * b[2] - a[2] * b[1],
  a[2] * b[0] - a[0] * b[2],
  a[0] * b[1] - a[1] * b[0]
];
const normalize = (a) => {
  const magnitude = length(a);
  if (!(magnitude > 0) || !Number.isFinite(magnitude)) throw new Error("cannot normalize non-finite or zero vector");
  return scale(a, 1 / magnitude);
};
const xyz = (value) => [Number(value.x), Number(value.y), Number(value.z)];
const finiteVec = (value) => value.length === 3 && value.every(Number.isFinite);
const clamp = (value, low, high) => Math.max(low, Math.min(high, value));

export function cameraBasis(camera) {
  const position = camera.position.map(Number);
  const target = camera.target.map(Number);
  const declaredUp = camera.up.map(Number);
  if (![position, target, declaredUp].every(finiteVec)) throw new Error("camera vectors must be finite triples");
  const forward = normalize(sub(target, position));
  const right = normalize(cross(forward, declaredUp));
  const up = normalize(cross(right, forward));
  const verticalFovRadians = Number(camera.fovDegrees) * Math.PI / 180;
  const aspect = Number(camera.aspect);
  const near = Number(camera.near);
  const far = Number(camera.far);
  if (!(verticalFovRadians > 0 && verticalFovRadians < Math.PI) || !(aspect > 0) || !(near > 0) || !(far > near)) {
    throw new Error("camera fov/aspect/clip values are invalid");
  }
  return { position, forward, right, up, verticalFovRadians, aspect, near, far };
}

export function projectPointToNdc(camera, point) {
  const basis = cameraBasis(camera);
  const offset = sub(point, basis.position);
  const axialDepth = dot(offset, basis.forward);
  const halfHeight = axialDepth * Math.tan(basis.verticalFovRadians / 2);
  return {
    ndc: [dot(offset, basis.right) / (halfHeight * basis.aspect), dot(offset, basis.up) / halfHeight],
    axialDepth
  };
}

export function rayForNdc(camera, ndc) {
  const basis = cameraBasis(camera);
  if (!finiteVec([ndc[0], ndc[1], 0]) || ndc[0] < -1 || ndc[0] > 1 || ndc[1] < -1 || ndc[1] > 1) {
    return { eligible: false, reason: "NDC_OUTSIDE_CLOSED_SQUARE", ndc: [...ndc] };
  }
  const tangent = Math.tan(basis.verticalFovRadians / 2);
  const direction = normalize(add(basis.forward, add(
    scale(basis.right, ndc[0] * tangent * basis.aspect),
    scale(basis.up, ndc[1] * tangent)
  )));
  const cosTheta = dot(direction, basis.forward);
  if (!(cosTheta > 0) || !Number.isFinite(cosTheta)) return { eligible: false, reason: "RAY_COS_THETA_INELIGIBLE", ndc: [...ndc] };
  return {
    eligible: true,
    origin: basis.position,
    direction,
    forward: basis.forward,
    cosTheta,
    tNear: basis.near / cosTheta,
    tFar: basis.far / cosTheta,
    verticalFovRadians: basis.verticalFovRadians
  };
}

function boundsOrigin(model) {
  const points = model.nodes.map((node) => xyz(node.position)).filter(finiteVec);
  if (!points.length) return [0, 0, 0];
  const minimum = [0, 1, 2].map((axis) => Math.min(...points.map((point) => point[axis])));
  const maximum = [0, 1, 2].map((axis) => Math.max(...points.map((point) => point[axis])));
  return minimum.map((value, axis) => (value + maximum[axis]) / 2);
}

function closestRaySegment(origin, direction, start, end) {
  const segment = sub(end, start);
  const segmentLengthSquared = dot(segment, segment);
  if (!(segmentLengthSquared > 0)) {
    const t = Math.max(0, -dot(direction, sub(origin, start)));
    const rayPoint = add(origin, scale(direction, t));
    return { t, segmentFraction: 0, rayPoint, segmentPoint: start, distance: length(sub(rayPoint, start)) };
  }
  const offset = sub(origin, start);
  const raySegmentDot = dot(direction, segment);
  const rayOffsetDot = dot(direction, offset);
  const segmentOffsetDot = dot(segment, offset);
  const candidates = [];
  const denominator = segmentLengthSquared - raySegmentDot * raySegmentDot;
  if (Math.abs(denominator) > EPS) {
    const t = (raySegmentDot * segmentOffsetDot - segmentLengthSquared * rayOffsetDot) / denominator;
    const fraction = (segmentOffsetDot - raySegmentDot * rayOffsetDot) / denominator;
    if (t >= 0 && fraction >= 0 && fraction <= 1) candidates.push([t, fraction]);
  }
  candidates.push([0, clamp(segmentOffsetDot / segmentLengthSquared, 0, 1)]);
  candidates.push([Math.max(0, -rayOffsetDot), 0]);
  const endOffset = sub(origin, end);
  candidates.push([Math.max(0, -dot(direction, endOffset)), 1]);
  return candidates.map(([t, segmentFraction]) => {
    const rayPoint = add(origin, scale(direction, t));
    const segmentPoint = add(start, scale(segment, segmentFraction));
    return { t, segmentFraction, rayPoint, segmentPoint, distance: length(sub(rayPoint, segmentPoint)) };
  }).sort((a, b) => a.distance - b.distance || a.t - b.t || a.segmentFraction - b.segmentFraction)[0];
}

function intersectDomain(interval, domain) {
  const entry = Math.max(interval[0], domain[0]);
  const exit = Math.min(interval[1], domain[1]);
  return entry <= exit + EPS ? [entry, exit] : null;
}

function quadraticLessEqual(a, b, c, domain) {
  if (Math.abs(a) <= EPS) {
    if (Math.abs(b) <= EPS) return c <= EPS ? [domain] : [];
    const root = -c / b;
    const candidate = b > 0 ? [-Infinity, root] : [root, Infinity];
    const clipped = intersectDomain(candidate, domain);
    return clipped ? [clipped] : [];
  }
  const discriminant = b * b - 4 * a * c;
  if (discriminant < -EPS) return [];
  const squareRoot = Math.sqrt(Math.max(0, discriminant));
  const roots = [(-b - squareRoot) / (2 * a), (-b + squareRoot) / (2 * a)].sort((x, y) => x - y);
  const candidate = a > 0 ? roots : [-Infinity, roots[0], roots[1], Infinity];
  if (a > 0) {
    const clipped = intersectDomain(candidate, domain);
    return clipped ? [clipped] : [];
  }
  return [[candidate[0], candidate[1]], [candidate[2], candidate[3]]]
    .map((interval) => intersectDomain(interval, domain)).filter(Boolean);
}

function mergeIntervals(intervals) {
  const sorted = intervals.filter(Boolean).sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const merged = [];
  for (const interval of sorted) {
    const previous = merged.at(-1);
    if (previous && interval[0] <= previous[1] + EPS) previous[1] = Math.max(previous[1], interval[1]);
    else merged.push([...interval]);
  }
  return merged;
}

function sphereInterval(origin, direction, centre, radius) {
  const offset = sub(origin, centre);
  const projection = dot(direction, offset);
  const discriminant = projection * projection - (dot(offset, offset) - radius * radius);
  if (discriminant < -EPS) return null;
  const root = Math.sqrt(Math.max(0, discriminant));
  return [-projection - root, -projection + root];
}

function capsuleInterval(origin, direction, start, end, radius) {
  const segment = sub(end, start);
  const segmentLengthSquared = dot(segment, segment);
  if (!(segmentLengthSquared > EPS)) return sphereInterval(origin, direction, start, radius);
  const offset = sub(origin, start);
  const q0 = dot(offset, segment) / segmentLengthSquared;
  const q1 = dot(direction, segment) / segmentLengthSquared;
  const breakpoints = [0];
  if (Math.abs(q1) > EPS) {
    for (const target of [0, 1]) {
      const t = (target - q0) / q1;
      if (t > 0 && Number.isFinite(t)) breakpoints.push(t);
    }
  }
  breakpoints.sort((a, b) => a - b);
  const domains = breakpoints.map((low, index) => [low, breakpoints[index + 1] ?? Infinity]);
  const intervals = [];
  for (const domain of domains) {
    const sampleT = Number.isFinite(domain[1]) ? (domain[0] + domain[1]) / 2 : domain[0] + 1;
    const q = q0 + q1 * sampleT;
    let constant;
    let slope;
    if (q <= 0) {
      constant = offset;
      slope = direction;
    } else if (q >= 1) {
      constant = sub(origin, end);
      slope = direction;
    } else {
      constant = sub(offset, scale(segment, q0));
      slope = sub(direction, scale(segment, q1));
    }
    intervals.push(...quadraticLessEqual(dot(slope, slope), 2 * dot(constant, slope), dot(constant, constant) - radius * radius, domain));
  }
  const merged = mergeIntervals(intervals);
  if (!merged.length) return null;
  return [merged[0][0], merged.at(-1)[1]];
}

function coneInterval(origin, direction, tip, axis, height, baseRadius) {
  const offset = sub(origin, tip);
  const slope = baseRadius / height;
  const directionAxis = dot(direction, axis);
  const offsetAxis = dot(offset, axis);
  const directionPerpendicular = sub(direction, scale(axis, directionAxis));
  const offsetPerpendicular = sub(offset, scale(axis, offsetAxis));
  const a = dot(directionPerpendicular, directionPerpendicular) - slope * slope * directionAxis * directionAxis;
  const b = 2 * (dot(directionPerpendicular, offsetPerpendicular) - slope * slope * directionAxis * offsetAxis);
  const c = dot(offsetPerpendicular, offsetPerpendicular) - slope * slope * offsetAxis * offsetAxis;
  const roots = [];
  if (Math.abs(a) <= EPS) {
    if (Math.abs(b) > EPS) roots.push(-c / b);
  } else {
    const discriminant = b * b - 4 * a * c;
    if (discriminant >= -EPS) {
      const squareRoot = Math.sqrt(Math.max(0, discriminant));
      roots.push((-b - squareRoot) / (2 * a), (-b + squareRoot) / (2 * a));
    }
  }
  const valid = roots.filter((t) => {
    const axial = offsetAxis + t * directionAxis;
    return t >= 0 && axial >= -EPS && axial <= height + EPS;
  });
  if (Math.abs(directionAxis) > EPS) {
    const tBase = (height - offsetAxis) / directionAxis;
    const basePoint = add(offset, scale(direction, tBase));
    const radial = sub(basePoint, scale(axis, height));
    if (tBase >= 0 && dot(radial, radial) <= baseRadius * baseRadius + EPS) valid.push(tBase);
  }
  const originAxial = offsetAxis;
  const originRadial = sub(offset, scale(axis, originAxial));
  const originInside = originAxial >= 0 && originAxial <= height &&
    length(originRadial) <= slope * originAxial + EPS;
  valid.sort((x, y) => x - y);
  if (originInside) valid.unshift(0);
  if (!valid.length) return null;
  return [valid[0], valid.at(-1)];
}

function rayAabbInterval(origin, direction, minimum, maximum) {
  let entry = -Infinity;
  let exit = Infinity;
  for (let axis = 0; axis < 3; axis += 1) {
    if (Math.abs(direction[axis]) <= EPS) {
      if (origin[axis] < minimum[axis] || origin[axis] > maximum[axis]) return null;
      continue;
    }
    const values = [(minimum[axis] - origin[axis]) / direction[axis], (maximum[axis] - origin[axis]) / direction[axis]].sort((a, b) => a - b);
    entry = Math.max(entry, values[0]);
    exit = Math.min(exit, values[1]);
    if (entry > exit) return null;
  }
  return [entry, exit];
}

function r6AtDepth(depth, verticalFovRadians, canvasCssHeight) {
  if (!(depth > 0) || !Number.isFinite(depth) || !(canvasCssHeight > 0)) return null;
  return 6 * (2 * depth * Math.tan(verticalFovRadians / 2) / canvasCssHeight);
}

function componentRadius(kind) {
  if (["bend", "elbow", "tee", "branch"].includes(kind)) return 0.30;
  if (["rigid", "valve", "flange", "reducer", "specialty", "expansion_joint"].includes(kind)) return 0.25;
  return Math.sqrt(0.12 ** 2 + 0.12 ** 2 + 0.12 ** 2);
}

function primitiveCandidates(model, originShift) {
  const nodeMap = new Map(model.nodes.map((node) => [node.id, sub(xyz(node.position), originShift)]));
  const values = [];
  model.components.forEach((component, order) => {
    const node = nodeMap.get(component.node);
    if (node) values.push({ type: "component", id: component.id, order, shape: "sphere", centre: add(node, [0, 0.20, 0]), baseRadius: componentRadius(component.kind), componentKind: component.kind });
  });
  model.supports.forEach((support, order) => {
    const node = nodeMap.get(support.node);
    if (node) values.push({ type: "support", id: support.id, order, shape: "cone", centre: add(node, [0, -0.26, 0]), height: 0.34, baseRadius: 0.18 });
  });
  model.nodes.forEach((node, order) => values.push({ type: "node", id: node.id, order, shape: "sphere", centre: sub(xyz(node.position), originShift), baseRadius: 0.095 }));
  model.pipe_segments.forEach((pipe, order) => {
    const start = nodeMap.get(pipe.from);
    const end = nodeMap.get(pipe.to);
    if (start && end) values.push({ type: "pipe", id: pipe.id, order, shape: "capsule", start, end, baseRadius: 0.052, pipe });
  });
  return values;
}

// Visual centers share the independent primitive construction used by ray picking.
// Build once per model; lookup remains typed so colliding IDs cannot alias.
export function primitiveCenterIndex(model, originShift = [0, 0, 0]) {
  const index = new Map();
  for (const primitive of primitiveCandidates(model, originShift)) {
    const center = primitive.shape === "capsule"
      ? primitive.start.map((value, axis) => (value + primitive.end[axis]) / 2)
      : [...primitive.centre];
    if (center.length !== 3 || !center.every(Number.isFinite)) throw new Error("nonfinite primitive center");
    const key = `${primitive.type}\0${primitive.id}`;
    if (index.has(key)) throw new Error("duplicate typed primitive");
    index.set(key, center);
  }
  return index;
}

function exactCandidate(candidate, ray, cameraPosition, cameraForward, canvasCssHeight, mode, convertedOdBySection) {
  let depth;
  if (candidate.shape === "capsule") {
    const closest = closestRaySegment(ray.origin, ray.direction, candidate.start, candidate.end);
    depth = dot(sub(closest.segmentPoint, cameraPosition), cameraForward);
  } else {
    depth = dot(sub(candidate.centre, cameraPosition), cameraForward);
  }
  const sixPixelRadius = r6AtDepth(depth, ray.verticalFovRadians, canvasCssHeight);
  if (sixPixelRadius === null) return null;
  let radius = Math.max(candidate.baseRadius, sixPixelRadius);
  if (candidate.type === "pipe" && mode === "actual_od") {
    const converted = Number(convertedOdBySection?.[candidate.pipe.section_ref]);
    if (Number.isFinite(converted) && converted > 0) radius = Math.max(radius, converted / 2);
  }
  let minimum;
  let maximum;
  if (candidate.shape === "capsule") {
    minimum = [0, 1, 2].map((axis) => Math.min(candidate.start[axis], candidate.end[axis]) - radius);
    maximum = [0, 1, 2].map((axis) => Math.max(candidate.start[axis], candidate.end[axis]) + radius);
  } else if (candidate.shape === "cone") {
    const tip = add(candidate.centre, [0, candidate.height / 2, 0]);
    const base = sub(candidate.centre, [0, candidate.height / 2, 0]);
    minimum = [candidate.centre[0] - radius, base[1], candidate.centre[2] - radius];
    maximum = [candidate.centre[0] + radius, tip[1], candidate.centre[2] + radius];
  } else {
    minimum = candidate.centre.map((value) => value - radius);
    maximum = candidate.centre.map((value) => value + radius);
  }
  const broad = rayAabbInterval(ray.origin, ray.direction, minimum, maximum);
  if (!broad || !intersectDomain(broad, [ray.tNear, ray.tFar])) return null;
  let interval;
  if (candidate.shape === "capsule") interval = capsuleInterval(ray.origin, ray.direction, candidate.start, candidate.end, radius);
  else if (candidate.shape === "cone") {
    const tip = add(candidate.centre, [0, candidate.height / 2, 0]);
    interval = coneInterval(ray.origin, ray.direction, tip, [0, -1, 0], candidate.height, radius);
  } else interval = sphereInterval(ray.origin, ray.direction, candidate.centre, radius);
  if (!interval) return null;
  const clipped = intersectDomain(interval, [ray.tNear, ray.tFar]);
  if (!clipped) return null;
  let miss;
  if (candidate.shape === "capsule") miss = closestRaySegment(ray.origin, ray.direction, candidate.start, candidate.end).distance / radius;
  else if (candidate.shape === "sphere") {
    const toCentre = sub(candidate.centre, ray.origin);
    const closestT = Math.max(0, dot(toCentre, ray.direction));
    miss = length(sub(add(ray.origin, scale(ray.direction, closestT)), candidate.centre)) / radius;
  } else {
    const point = add(ray.origin, scale(ray.direction, Math.max(clipped[0], ray.tNear)));
    miss = Math.hypot(point[0] - candidate.centre[0], point[2] - candidate.centre[2]) / radius;
  }
  return { ...candidate, radius, depth, broadInterval: broad, exactInterval: interval, clippedEntry: Math.max(interval[0], ray.tNear), normalizedMiss: miss };
}

export function expectedHitForProbe(model, camera, canvasCssHeight, ndc, options = {}) {
  const originShift = options.origin ?? boundsOrigin(model);
  const shiftedCamera = { ...camera, position: sub(camera.position, originShift), target: sub(camera.target, originShift) };
  const ray = rayForNdc(shiftedCamera, ndc);
  if (!ray.eligible) return { status: "NO_HIT", reason: ray.reason, origin: originShift, ndc: [...ndc], expectedHitRef: null, candidates: [] };
  const basis = cameraBasis(shiftedCamera);
  const candidates = primitiveCandidates(model, originShift)
    .map((candidate) => exactCandidate(candidate, ray, basis.position, basis.forward, canvasCssHeight, options.mode ?? "schematic", options.convertedOdBySection ?? {}))
    .filter(Boolean);
  if (!candidates.length) return { status: "NO_HIT", reason: "NO_ELIGIBLE_PRIMITIVE", origin: originShift, ndc: [...ndc], expectedHitRef: null, candidates: [] };
  const tMin = Math.min(...candidates.map((candidate) => candidate.clippedEntry));
  const frontGroup = candidates.filter((candidate) => Math.abs(candidate.clippedEntry - tMin) <=
    1e-9 * Math.max(1, Math.abs(tMin), Math.abs(candidate.clippedEntry)));
  const missMin = Math.min(...frontGroup.map((candidate) => candidate.normalizedMiss));
  const missGroup = frontGroup.filter((candidate) => Math.abs(candidate.normalizedMiss - missMin) <= 1e-12);
  const typeOrder = new Map([["component", 0], ["support", 1], ["node", 2], ["pipe", 3]]);
  missGroup.sort((a, b) => typeOrder.get(a.type) - typeOrder.get(b.type) || a.order - b.order ||
    JSON.stringify([a.type, a.id]).localeCompare(JSON.stringify([b.type, b.id])));
  const winner = missGroup[0];
  return {
    status: "HIT",
    origin: originShift,
    ndc: [...ndc],
    ray: { direction: ray.direction, cosTheta: ray.cosTheta, tNear: ray.tNear, tFar: ray.tFar },
    expectedHitRef: { type: winner.type, id: winner.id },
    details: { candidates: candidates.map(c=>({type:c.type,id:c.id,entry:c.clippedEntry,miss:c.normalizedMiss})), front: frontGroup.map(c=>({type:c.type,id:c.id,entry:c.clippedEntry,miss:c.normalizedMiss})), missGroup: missGroup.map(c=>({type:c.type,id:c.id,entry:c.clippedEntry,miss:c.normalizedMiss})) }, winner: { clippedEntry: winner.clippedEntry, normalizedMiss: winner.normalizedMiss, radius: winner.radius, depth: winner.depth, displayedModelOrder: winner.order },
    candidateCount: candidates.length,
    frontGroupCount: frontGroup.length,
    missGroupCount: missGroup.length
  };
}

export function fixtureCamera(recipe, aspect) {
  if (recipe === "baseline_iso") return { position: [7.6, 7, 8], target: [3.8, 1.2, 0.7], up: [0, 1, 0], fovDegrees: 42, near: 0.1, far: 1000, aspect };
  if (recipe === "candidate_overview_iso") return { position: [12.4, 8.1, 13.7], target: [5, 0.65, 0.5], up: [0, 1, 0], fovDegrees: 42, near: 0.1, far: 1000, aspect };
  throw new Error(`unknown fixture camera recipe: ${recipe}`);
}
