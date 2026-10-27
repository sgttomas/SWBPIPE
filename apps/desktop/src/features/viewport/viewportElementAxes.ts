// Instanced element axis triads (A7): three single-headed arrows per member at its midpoint,
// x, y and z in the triad's axis colours. Each instance is owned by its member, so it hides
// and dims with it. Built when the frame, the model or the transform changes, never per frame.

import * as THREE from "three";
import type { Vec3 } from "../../types";
import type { EntityKey } from "../workspace/selectionState";
import { createFigureMaterial } from "./viewportFigureMaterial";
import type { ViewportPaletteRole } from "./viewportPalette";
import { registerInstancedRolePresentation } from "./viewportResource";

export type ElementAxisPlacement = Readonly<{
  ownerKey: EntityKey;
  /** The member's midpoint in render-local coordinates. */
  anchor: Vec3;
  /** Unit x, y and z directions. */
  axes: readonly [Vec3, Vec3, Vec3];
  /** Arrow length in render-local units. */
  displayLength: number;
}>;

const SHAFT_RADIUS = 0.012;
const HEAD_RADIUS = 0.035;
const HEAD_LENGTH_LIMIT = 0.07;
/** Arrow length: a share of the member's length, capped so a long member's triad stays a glyph. */
export const ELEMENT_AXIS_LENGTH_SHARE = 0.35;
export const ELEMENT_AXIS_MAX_LENGTH = 0.6;

const AXIS_ROLES: readonly ViewportPaletteRole[] = ["elementAxisX", "elementAxisY", "elementAxisZ"];
const AXIS_NAMES = ["x", "y", "z"] as const;

export function elementAxisDisplayLength(memberLength: number): number {
  return Math.min(memberLength * ELEMENT_AXIS_LENGTH_SHARE, ELEMENT_AXIS_MAX_LENGTH);
}

/** Shaft and head matrices of one axis arrow from the anchor. */
export function elementAxisArrowMatrices(anchor: Vec3, axis: Vec3, length: number): { shaft: THREE.Matrix4; head: THREE.Matrix4 } {
  const direction = new THREE.Vector3(axis.x, axis.y, axis.z).normalize();
  const origin = new THREE.Vector3(anchor.x, anchor.y, anchor.z);
  const head = Math.min(HEAD_LENGTH_LIMIT, length * 0.3);
  const quaternion = new THREE.Quaternion().setFromUnitVectors(new THREE.Vector3(0, 1, 0), direction);
  return {
    shaft: new THREE.Matrix4().compose(origin.clone().addScaledVector(direction, (length - head) / 2), quaternion, new THREE.Vector3(1, length - head, 1)),
    head: new THREE.Matrix4().compose(origin.clone().addScaledVector(direction, length - head / 2), quaternion, new THREE.Vector3(1, head, 1))
  };
}

/** Per spatial group, one shaft mesh and one head mesh for each of x, y and z. */
export function instancedElementAxisMeshes(groups: readonly (readonly ElementAxisPlacement[])[]): THREE.InstancedMesh[] {
  const kept = groups
    .map((group) => group.filter((placement) => Number.isFinite(placement.displayLength) && placement.displayLength > 0))
    .filter((group) => group.length > 0);
  if (!kept.length) return [];
  const shaftGeometry = new THREE.CylinderGeometry(SHAFT_RADIUS, SHAFT_RADIUS, 1, 8);
  const headGeometry = new THREE.ConeGeometry(HEAD_RADIUS, 1, 10);
  const material = createFigureMaterial();
  const output: THREE.InstancedMesh[] = [];
  for (const group of kept) {
    const keys = group.map((placement) => placement.ownerKey);
    AXIS_ROLES.forEach((role, axisIndex) => {
      const shaft = new THREE.InstancedMesh(shaftGeometry, material, group.length);
      const head = new THREE.InstancedMesh(headGeometry, material, group.length);
      group.forEach((placement, instance) => {
        const matrices = elementAxisArrowMatrices(placement.anchor, placement.axes[axisIndex], placement.displayLength);
        shaft.setMatrixAt(instance, matrices.shaft);
        head.setMatrixAt(instance, matrices.head);
      });
      shaft.instanceMatrix.needsUpdate = true;
      head.instanceMatrix.needsUpdate = true;
      shaft.name = `viewport-element-axis-${AXIS_NAMES[axisIndex]}-shaft`;
      head.name = `viewport-element-axis-${AXIS_NAMES[axisIndex]}-head`;
      registerInstancedRolePresentation(shaft, keys, role);
      registerInstancedRolePresentation(head, keys, role);
      output.push(shaft, head);
    });
  }
  return output;
}
