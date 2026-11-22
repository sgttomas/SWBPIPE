import * as THREE from "three";
import { describe, expect, it } from "vitest";
import {
  alongViewPose,
  fitApplied,
  fitDirection,
  initialCameraRuleState,
  personMovedCamera,
  refitForCanvas,
  standardViewApplied,
  standardViewPose,
  type StandardView
} from "./viewportCameraRule";
import type { Bounds3 } from "../workspace/modelIndex";
import { fittedViewportDistance } from "./viewportSelection";

// The original fit, copied verbatim from viewportSelection.ts and PipeViewport.tsx at 5d7a28e8e (the
// branch's base): the pose the timed instrument's frozen camera was taken from. Every standard view the
// base had (front, top, iso) must give this pose bit for bit.
function baseFittedViewportDistance(bounds: Bounds3 | null, preset: "front" | "top" | "iso", fovDegrees: number, aspect: number): number {
  const extent = bounds ? Math.max(bounds.max.x - bounds.min.x, bounds.max.y - bounds.min.y, bounds.max.z - bounds.min.z, 1) : 4;
  let distance = Math.max(4, extent * 1.8);
  if (!bounds) return distance;
  const orientation = new THREE.PerspectiveCamera();
  orientation.position.set(...(preset === "front" ? [0, 0, 1] : preset === "top" ? [0, 1, 0] : [1, 1, 1]) as [number, number, number]);
  orientation.lookAt(0, 0, 0);
  const inverseRotation = orientation.quaternion.clone().invert();
  const center = new THREE.Vector3(bounds.min.x / 2 + bounds.max.x / 2, bounds.min.y / 2 + bounds.max.y / 2, bounds.min.z / 2 + bounds.max.z / 2);
  const verticalSlope = Math.tan(THREE.MathUtils.degToRad(fovDegrees) / 2) * 0.9;
  const horizontalSlope = verticalSlope * aspect;
  for (const x of [bounds.min.x, bounds.max.x]) for (const y of [bounds.min.y, bounds.max.y]) for (const z of [bounds.min.z, bounds.max.z]) {
    const corner = new THREE.Vector3(x, y, z).sub(center).applyQuaternion(inverseRotation);
    distance = Math.max(distance, corner.z + Math.abs(corner.x) / horizontalSlope, corner.z + Math.abs(corner.y) / verticalSlope);
  }
  return distance;
}

function baseFitPose(preset: "front" | "top" | "iso", bounds: Bounds3 | null, fov: number, aspect: number) {
  const center = bounds
    ? { x: bounds.min.x / 2 + bounds.max.x / 2, y: bounds.min.y / 2 + bounds.max.y / 2, z: bounds.min.z / 2 + bounds.max.z / 2 }
    : { x: 0, y: 0, z: 0 };
  const extent = bounds ? Math.max(bounds.max.x - bounds.min.x, bounds.max.y - bounds.min.y, bounds.max.z - bounds.min.z, 1) : 4;
  const distance = baseFittedViewportDistance(bounds, preset, fov, aspect);
  const camera = new THREE.PerspectiveCamera(fov, aspect, 0.1, 10_000);
  if (preset === "front") camera.position.set(center.x, center.y, center.z + distance);
  else if (preset === "top") camera.position.set(center.x, center.y + distance, center.z);
  else {
    const diagonal = distance / Math.sqrt(3);
    camera.position.set(center.x + diagonal, center.y + diagonal, center.z + diagonal);
  }
  return { position: camera.position.clone(), target: new THREE.Vector3(center.x, center.y, center.z), near: Math.max(0.001, distance / 100_000), far: Math.max(1_000, distance + extent * 8) };
}

// The camera's field of view (viewportResource.ts) and the canvases: REQ-012's stated sizes, and the
// sizes measured at this head in the 1440 by 920 browser window (Both and Model).
const FOV = 42;
const CANVASES = [[603, 828], [1000, 828], [603, 743], [1000, 463]] as const;
const MODELS: Bounds3[] = [
  { min: { x: -20, y: -3, z: -11 }, max: { x: 20, y: 3, z: 11 } },
  { min: { x: 0.25, y: -1.5, z: 3 }, max: { x: 6.75, y: 11, z: 7.5 } },
  { min: { x: -500, y: 0, z: -2 }, max: { x: 500, y: 40, z: 2 } }
];

function expectSamePose(actual: ReturnType<typeof standardViewPose>, expected: ReturnType<typeof baseFitPose>) {
  expect([actual.position.x, actual.position.y, actual.position.z]).toEqual([expected.position.x, expected.position.y, expected.position.z]);
  expect([actual.target.x, actual.target.y, actual.target.z]).toEqual([expected.target.x, expected.target.y, expected.target.z]);
  expect([actual.near, actual.far]).toEqual([expected.near, expected.far]);
}

describe("the fitted pose of a standard view", () => {
  it("is the base's, bit for bit, at the reference and measured canvases, with the frame filled to exactly 90 %", () => {
    for (const [width, height] of CANVASES) for (const bounds of MODELS) for (const view of ["front", "top", "iso"] as const) {
      const aspect = width / height;
      expect(fittedViewportDistance(bounds, view, FOV, aspect)).toBe(baseFittedViewportDistance(bounds, view, FOV, aspect));
      expectSamePose(standardViewPose(view, bounds, FOV, aspect), baseFitPose(view, bounds, FOV, aspect));
      // The fill: the farthest corner lands at 90 % of the half-frame on the binding axis, no less.
      const pose = standardViewPose(view, bounds, FOV, aspect);
      const camera = new THREE.PerspectiveCamera(FOV, aspect, pose.near, pose.far);
      camera.position.set(pose.position.x, pose.position.y, pose.position.z);
      camera.lookAt(pose.target.x, pose.target.y, pose.target.z);
      camera.updateMatrixWorld();
      let reach = 0;
      for (const x of [bounds.min.x, bounds.max.x]) for (const y of [bounds.min.y, bounds.max.y]) for (const z of [bounds.min.z, bounds.max.z]) {
        const ndc = new THREE.Vector3(x, y, z).project(camera);
        reach = Math.max(reach, Math.abs(ndc.x), Math.abs(ndc.y));
      }
      expect(reach).toBeLessThanOrEqual(0.9 + 1e-7);
    }
  });

  it("with no model, is the base's default pose", () => {
    for (const [width, height] of CANVASES) expectSamePose(standardViewPose("iso", null, FOV, width / height), baseFitPose("iso", null, FOV, width / height));
  });
});

describe("the camera rule", () => {
  const bounds = MODELS[1];
  const aspect = 603 / 743;

  it("orbit, then Isometric, then Fit Model: the base's Isometric fit of the model, bit for bit", () => {
    // Opened on Isometric and fitted.
    let state = standardViewApplied(initialCameraRuleState("iso"), "iso", 603, 743);
    // The person orbits: no standard view is current, nothing is fitted.
    state = personMovedCamera(state);
    expect(state.standardView).toBeNull();
    expect(fitDirection(state)).toEqual({ kind: "along" });
    // Isometric applies its view every time, even though it was the last one applied.
    state = standardViewApplied(state, "iso", 603, 743);
    expect(state.standardView).toBe("iso");
    // Fit Model on the unmoved Isometric view runs Isometric's original fit.
    const direction = fitDirection(state);
    expect(direction).toEqual({ kind: "standard-view", view: "iso" });
    state = fitApplied(state, "fit-model", 603, 743);
    expect(state.standardView).toBe("iso");
    if (direction.kind !== "standard-view") throw new Error("unreachable");
    expectSamePose(standardViewPose(direction.view, bounds, FOV, aspect), baseFitPose("iso", bounds, FOV, aspect));
  });

  it("refits a fitted view on a new canvas size, and leaves a view the person moved", () => {
    const fitted = standardViewApplied(initialCameraRuleState(), "front", 603, 743);
    expect(refitForCanvas(fitted, 603, 743)).toBeNull();
    expect(refitForCanvas(fitted, 603.4, 742.6)).toBeNull();
    expect(refitForCanvas(fitted, 1000, 463)).toMatchObject({ kind: "standard-view" });
    const moved = personMovedCamera(fitted);
    expect(refitForCanvas(moved, 1000, 463)).toBeNull();
    // Each Fit command and each standard view returns it to fitted.
    for (const kind of ["fit-model", "fit-visible", "fit-selection"] as const) {
      expect(refitForCanvas(fitApplied(moved, kind, 603, 743), 1000, 463)).toMatchObject({ kind });
    }
    for (const view of ["front", "top", "side", "iso"] as const satisfies readonly StandardView[]) {
      const state = standardViewApplied(moved, view, 603, 743);
      expect(state.standardView).toBe(view);
      expect(refitForCanvas(state, 1000, 463)).toMatchObject({ kind: "standard-view" });
    }
  });

  it("refits a Fit Selection on the entities it fitted, not on a later selection", () => {
    const state = fitApplied(initialCameraRuleState(), "fit-selection", 603, 743, ["node:N-110", "pipe:P-100"]);
    expect(refitForCanvas(state, 1000, 463)?.keys).toEqual(["node:N-110", "pipe:P-100"]);
  });

  it("keeps the person's direction when they have moved the camera, and reframes along it", () => {
    const orientation = new THREE.Quaternion().setFromEuler(new THREE.Euler(-0.4, 0.9, 0, "YXZ"));
    const pose = alongViewPose(orientation, bounds, FOV, aspect);
    const camera = new THREE.PerspectiveCamera(FOV, aspect, pose.near, pose.far);
    camera.position.set(pose.position.x, pose.position.y, pose.position.z);
    camera.lookAt(pose.target.x, pose.target.y, pose.target.z);
    camera.updateMatrixWorld();
    // The same direction: the camera's forward axis is the one it had.
    const before = new THREE.Vector3(0, 0, -1).applyQuaternion(orientation);
    const after = new THREE.Vector3(0, 0, -1).applyQuaternion(camera.quaternion);
    expect(before.angleTo(after)).toBeLessThan(1e-6);
    // Framed: every corner inside 90 % of the frame, and one on it.
    let reach = 0;
    for (const x of [bounds.min.x, bounds.max.x]) for (const y of [bounds.min.y, bounds.max.y]) for (const z of [bounds.min.z, bounds.max.z]) {
      const ndc = new THREE.Vector3(x, y, z).project(camera);
      reach = Math.max(reach, Math.abs(ndc.x), Math.abs(ndc.y));
    }
    expect(reach).toBeLessThanOrEqual(0.9 + 1e-7);
    expect(reach).toBeGreaterThan(0.85);
  });
});
