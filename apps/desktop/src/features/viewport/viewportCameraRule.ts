// Where the view's camera goes, and when the view moves it by itself (DEL-07-01 REQ-012: a view or
// stage switch keeps the camera where the person set it; a fitted view is refitted to the new canvas).
//
// The rule:
// - A standard view button (Front, Top, Side, Isometric) always applies its view: the model fitted
//   from that direction, by the original fit (fittedViewportDistance, 90 % of the frame). That view is
//   then the current one, shown pressed.
// - The person sets the camera only when their own navigation changes its pose (a drag, a wheel, a
//   pan). A press that does not move it (a click on an entity, a right-click, a press released where
//   it began) does not. Once the person has moved it, no standard view is current, none is pressed.
// - Fit Model, Fit Visible and Fit Selection frame what they name. While a standard view is current,
//   they run that view's original fit unchanged, so a standard view and then Fit Model gives the same
//   camera as it always did. After the person has moved the camera, Fit keeps the direction they look
//   from and reframes along it.
// - The view is fitted after a standard view or a Fit, until the person moves the camera. A fitted
//   view is fitted again when the canvas changes size, as its last fit was: the same view, or the same
//   command for the same entities (a Fit Selection refits the entities it fitted; when none of them is
//   still shown, it fits the model). A view the person has moved is left where they set it.

import * as THREE from "three";
import type { Bounds3 } from "../workspace/modelIndex";
import { fittedViewportDistance, fittedViewportDistanceAlong } from "./viewportSelection";

export type StandardView = "front" | "top" | "side" | "iso";
export type FitCommandKind = "fit-model" | "fit-visible" | "fit-selection";

export type FitRecord = Readonly<{
  kind: "standard-view" | FitCommandKind;
  /** For a Fit Selection, the entities it fitted. */
  keys?: readonly string[];
  width: number;
  height: number;
}>;

export type CameraRuleState = Readonly<{
  /** The standard view the camera is in, or null once the person has moved it off one. */
  standardView: StandardView | null;
  /** The last standard view applied: where a new model opens when the person has moved the camera. */
  lastStandardView: StandardView;
  /** The last fit, or null when the person has moved the camera since. */
  fitted: FitRecord | null;
}>;

export function initialCameraRuleState(view: StandardView = "iso"): CameraRuleState {
  return { standardView: view, lastStandardView: view, fitted: null };
}

/** A standard view button, or a model opened: the view applied, current and fitted. */
export function standardViewApplied(_state: CameraRuleState, view: StandardView, width: number, height: number): CameraRuleState {
  return { standardView: view, lastStandardView: view, fitted: { kind: "standard-view", width, height } };
}

/** A Fit command: fitted; the current standard view, if any, stays current. */
export function fitApplied(state: CameraRuleState, kind: FitCommandKind, width: number, height: number, keys?: readonly string[]): CameraRuleState {
  return { ...state, fitted: { kind, keys: kind === "fit-selection" ? [...(keys ?? [])] : undefined, width, height } };
}

/** The person's own navigation changed the camera's pose. */
export function personMovedCamera(state: CameraRuleState): CameraRuleState {
  return { ...state, standardView: null, fitted: null };
}

/** How a Fit frames: a current standard view's original fit, or along the person's direction. */
export function fitDirection(state: CameraRuleState): Readonly<{ kind: "standard-view"; view: StandardView } | { kind: "along" }> {
  return state.standardView ? { kind: "standard-view", view: state.standardView } : { kind: "along" };
}

/** What to fit again when the canvas is now width by height, or null when nothing is refitted. */
export function refitForCanvas(state: CameraRuleState, width: number, height: number): FitRecord | null {
  const last = state.fitted;
  if (!last || !(width >= 2) || !(height >= 2)) return null;
  if (Math.abs(last.width - width) < 1 && Math.abs(last.height - height) < 1) return null;
  return last;
}

export type CameraPose = Readonly<{
  position: Readonly<{ x: number; y: number; z: number }>;
  target: Readonly<{ x: number; y: number; z: number }>;
  near: number;
  far: number;
}>;

function centerAndExtent(bounds: Bounds3 | null) {
  const center = bounds
    ? {
        x: bounds.min.x / 2 + bounds.max.x / 2,
        y: bounds.min.y / 2 + bounds.max.y / 2,
        z: bounds.min.z / 2 + bounds.max.z / 2
      }
    : { x: 0, y: 0, z: 0 };
  const extent = bounds
    ? Math.max(bounds.max.x - bounds.min.x, bounds.max.y - bounds.min.y, bounds.max.z - bounds.min.z, 1)
    : 4;
  return { center, extent };
}

/** A standard view's pose: the original fit, in the original arithmetic (Side added). */
export function standardViewPose(view: StandardView, bounds: Bounds3 | null, fovDegrees: number, aspect: number): CameraPose {
  const { center, extent } = centerAndExtent(bounds);
  const distance = fittedViewportDistance(bounds, view, fovDegrees, aspect);
  let position: { x: number; y: number; z: number };
  if (view === "front") {
    position = { x: center.x, y: center.y, z: center.z + distance };
  } else if (view === "top") {
    position = { x: center.x, y: center.y + distance, z: center.z };
  } else if (view === "side") {
    position = { x: center.x + distance, y: center.y, z: center.z };
  } else {
    const diagonal = distance / Math.sqrt(3);
    position = { x: center.x + diagonal, y: center.y + diagonal, z: center.z + diagonal };
  }
  return { position, target: center, near: Math.max(0.001, distance / 100_000), far: Math.max(1_000, distance + extent * 8) };
}

/** A fit along the direction the camera looks from now (its quaternion), framing the bounds. */
export function alongViewPose(orientation: THREE.Quaternion, bounds: Bounds3 | null, fovDegrees: number, aspect: number): CameraPose {
  const { center, extent } = centerAndExtent(bounds);
  const distance = fittedViewportDistanceAlong(bounds, orientation, fovDegrees, aspect);
  const back = new THREE.Vector3(0, 0, 1).applyQuaternion(orientation);
  return {
    position: { x: center.x + back.x * distance, y: center.y + back.y * distance, z: center.z + back.z * distance },
    target: center,
    near: Math.max(0.001, distance / 100_000),
    far: Math.max(1_000, distance + extent * 8)
  };
}
