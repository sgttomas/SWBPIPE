import assert from "node:assert/strict";
import { clipAuthoredSegmentToClosedFrustum, expectedBoxSelection } from "./box-selection-oracle.mjs";

const camera = {
  position: [0, 0, 0], target: [0, 0, -1], up: [0, 1, 0],
  fovDegrees: 90, near: 1, far: 10, aspect: 1
};
const model = {
  nodes: [
    { id: "node:left", position: { x: -2, y: 0, z: -5 } },
    { id: "node:right", position: { x: 2, y: 0, z: -5 } },
    { id: "node:center", position: { x: 0, y: 0, z: -5 } },
    { id: "node:behind-near", position: { x: 0, y: 0, z: -0.5 } }
  ],
  pipe_segments: [
    { id: "pipe:crossing", from: "node:left", to: "node:right" },
    { id: "pipe:near-crossing", from: "node:behind-near", to: "node:center" }
  ],
  supports: [{ id: "support:center", node: "node:center" }],
  components: [{ id: "component:center", node: "node:center" }]
};

const clipped = clipAuthoredSegmentToClosedFrustum(camera, [0, 0, -0.5], [0, 0, -5]);
assert(clipped);
assert(Math.abs(clipped.start[2] - 1) < 1e-12);

const contained = expectedBoxSelection(model, camera, {
  sample: 1, direction: "left_to_right_contained", filter: "all",
  start_normalized: [0.45, 0.45], end_normalized: [0.55, 0.55]
});
assert.deepEqual(contained.orderedRefs, [
  { type: "node", id: "node:center" },
  { type: "pipe", id: "pipe:near-crossing" },
  { type: "support", id: "support:center" },
  { type: "component", id: "component:center" }
]);
assert.deepEqual(contained.primaryRef, { type: "component", id: "component:center" });

const crossing = expectedBoxSelection(model, camera, {
  sample: 2, direction: "right_to_left_intersecting", filter: "pipes",
  start_normalized: [0.55, 0.45], end_normalized: [0.45, 0.55]
});
assert.deepEqual(crossing.orderedRefs, [
  { type: "pipe", id: "pipe:crossing" },
  { type: "pipe", id: "pipe:near-crossing" }
]);

const hidden = expectedBoxSelection(model, camera, {
  sample: 3, direction: "left_to_right_contained", filter: "supports",
  start_normalized: [0.45, 0.45], end_normalized: [0.55, 0.55]
}, [{ type: "support", id: "support:center" }]);
assert.deepEqual(hidden.orderedRefs, []);

process.stdout.write(`${JSON.stringify({
  status: "PASS",
  policy: "BOX_SELECTION_POLICY_V1",
  checks: [
    "closed near-plane clipping",
    "left-to-right clipped containment",
    "right-to-left clipped intersection",
    "typed treeOrder and last-hit primary",
    "type filter",
    "hidden exclusion"
  ]
}, null, 2)}\n`);
