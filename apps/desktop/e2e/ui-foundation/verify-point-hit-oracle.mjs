import assert from "node:assert/strict";
import { expectedHitForProbe, fixtureCamera, projectPointToNdc, rayForNdc } from "./point-hit-oracle.mjs";

function nodeModel(points) {
  return {
    nodes: points.map((position, index) => ({ id: `node:${index}`, position: { x: position[0], y: position[1], z: position[2] } })),
    pipe_segments: [],
    supports: [],
    components: []
  };
}

const offAxisCamera = {
  position: [0, 0, 0], target: [0, 0, -1], up: [0, 1, 0], fovDegrees: 90,
  near: 1, far: 10, aspect: 1
};
const offAxisRay = rayForNdc(offAxisCamera, [0.8, 0]);
assert.equal(offAxisRay.eligible, true);
assert(Math.abs(offAxisRay.tNear - offAxisCamera.near / offAxisRay.cosTheta) < 1e-12);
assert(Math.abs(offAxisRay.tFar - offAxisCamera.far / offAxisRay.cosTheta) < 1e-12);
assert(offAxisRay.tNear > offAxisCamera.near);
assert(offAxisRay.tFar > offAxisCamera.far);

const beforeNear = nodeModel([[offAxisRay.direction[0] * 1.1, 0, offAxisRay.direction[2] * 1.1]]);
assert.equal(expectedHitForProbe(beforeNear, offAxisCamera, 1000, [0.8, 0]).status, "NO_HIT");
const crossingNear = nodeModel([[offAxisRay.direction[0] * 1.25, 0, offAxisRay.direction[2] * 1.25]]);
assert.equal(expectedHitForProbe(crossingNear, offAxisCamera, 1000, [0.8, 0]).expectedHitRef.id, "node:0");
const crossingFar = nodeModel([[offAxisRay.direction[0] * 12.75, 0, offAxisRay.direction[2] * 12.75]]);
assert.equal(expectedHitForProbe(crossingFar, offAxisCamera, 1000, [0.8, 0]).expectedHitRef.id, "node:0");
const beyondFar = nodeModel([[offAxisRay.direction[0] * 13.0, 0, offAxisRay.direction[2] * 13.0]]);
assert.equal(expectedHitForProbe(beyondFar, offAxisCamera, 1000, [0.8, 0]).status, "NO_HIT");

const pipeModel = {
  nodes: [
    { id: "node:a", position: { x: -1, y: 0, z: -5 } },
    { id: "node:b", position: { x: 1, y: 0, z: -5 } }
  ],
  pipe_segments: [{ id: "pipe:p", from: "node:a", to: "node:b", section_ref: "section:s" }],
  supports: [], components: []
};
const centreCamera = { ...offAxisCamera, near: 0.1, far: 100 };
assert.equal(expectedHitForProbe(pipeModel, centreCamera, 920, [0, 0]).expectedHitRef.id, "pipe:p");

const translated = structuredClone(pipeModel);
for (const node of translated.nodes) {
  node.position.x += 1_000_000;
  node.position.y += 2_000_000;
  node.position.z -= 3_000_000;
}
const translatedCamera = {
  ...centreCamera,
  position: [1_000_000, 2_000_000, -3_000_000],
  target: [1_000_000, 2_000_000, -3_000_001]
};
const baseHit = expectedHitForProbe(pipeModel, centreCamera, 920, [0, 0]);
const translatedHit = expectedHitForProbe(translated, translatedCamera, 920, [0, 0]);
assert.deepEqual(translatedHit.expectedHitRef, baseHit.expectedHitRef);
assert(Math.abs(translatedHit.winner.clippedEntry - baseHit.winner.clippedEntry) < 1e-9);

const boundsRuleModel = {
  nodes: [
    { id: "node:min", position: { x: 0, y: -2, z: -6 } },
    { id: "node:max", position: { x: 2, y: 4, z: -4 } }
  ],
  pipe_segments: [], supports: [], components: [],
  sections: [{ properties: { outside_diameter: { value: 1_000_000 } } }],
  load_cases: [{ primitive_loads: [{ target: { position: { x: 9_000_000, y: 9_000_000, z: 9_000_000 } } }] }]
};
const boundsRuleCamera = { ...centreCamera, position: [1, 1, 0], target: [1, 1, -5] };
assert.deepEqual(expectedHitForProbe(boundsRuleModel, boundsRuleCamera, 920, [0, 0]).origin, [1, 1, -5]);

const fixtureCameraValue = fixtureCamera("baseline_iso", 1248 / 854);
const projected = projectPointToNdc(fixtureCameraValue, [3.8, 1.2, 0.7]);
assert(Math.abs(projected.ndc[0]) < 1e-12 && Math.abs(projected.ndc[1]) < 1e-12);

process.stdout.write(`${JSON.stringify({
  status: "PASS",
  policy: "POINT_HIT_POLICY_V3",
  checks: [
    "off-axis near/cosTheta exclusion",
    "off-axis physical near-plane crossing inclusion",
    "off-axis physical far-plane crossing inclusion",
    "off-axis beyond-far exclusion",
    "capsule hit",
    "bounds-centred million-unit translation invariance",
    "render origin uses authored node bounds only",
    "camera target projection"
  ]
}, null, 2)}\n`);
