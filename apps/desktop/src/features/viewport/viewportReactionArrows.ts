// Instanced support reaction arrows (FR-GUI-001; owner ruling OD-5). A force is a single-headed
// arrow whose head ends just short of the support's node, pointing along the emitted force: the
// action of the support on the pipe, drawn arriving at the pipe. A moment is the rotation glyph's
// double-headed arrow from the node along the emitted moment vector (right-hand rule), drawn a
// little heavier than a rotation glyph so the two can be told apart at one node. Both are owned by
// their support, so they hide, dim and repaint with it. Built once per change of case or result,
// never per frame.

import * as THREE from "three";
import type { Vec3 } from "../../types";
import type { EntityKey } from "../workspace/selectionState";
import { createFigureMaterial } from "./viewportFigureMaterial";
import { registerInstancedRolePresentation } from "./viewportResource";
import { instancedRotationGlyphMeshes, type RotationGlyphPlacement } from "./viewportRotationGlyph";

export type ReactionArrowPlacement = Readonly<{
  ownerKey: EntityKey;
  /** The support's node in render-local coordinates. */
  anchor: Vec3;
  /** Unit direction of the vector in global (and render-local) axes. */
  direction: Vec3;
  /** Display length in render-local units. */
  displayLength: number;
}>;

const FORCE_SHAFT_RADIUS = 0.022;
const FORCE_HEAD_RADIUS = 0.075;
const FORCE_HEAD_LENGTH_LIMIT = 0.14;
/** The head stops this far short of the node centre, clear of the node marker (radius 0.095). */
export const REACTION_FORCE_TIP_GAP = 0.11;
/** Moment glyphs are the rotation glyph scaled up in radius by this factor. */
export const REACTION_MOMENT_RADIUS_SCALE = 1.5;

function drawable(placement: ReactionArrowPlacement): boolean {
  const length = Math.hypot(placement.direction.x, placement.direction.y, placement.direction.z);
  return Number.isFinite(placement.displayLength) && placement.displayLength > 0 && Number.isFinite(length) && length > 0;
}

/** Shaft and head instance matrices of one force arrow, whose tip lies `REACTION_FORCE_TIP_GAP`
 * before the anchor along the direction. */
export function reactionForceArrowMatrices(placement: ReactionArrowPlacement): { shaft: THREE.Matrix4; head: THREE.Matrix4; tip: THREE.Vector3; tail: THREE.Vector3 } {
  const direction = new THREE.Vector3(placement.direction.x, placement.direction.y, placement.direction.z).normalize();
  const length = placement.displayLength;
  const head = Math.min(FORCE_HEAD_LENGTH_LIMIT, length * 0.35);
  const tip = new THREE.Vector3(placement.anchor.x, placement.anchor.y, placement.anchor.z).addScaledVector(direction, -REACTION_FORCE_TIP_GAP);
  const tail = tip.clone().addScaledVector(direction, -length);
  const quaternion = new THREE.Quaternion().setFromUnitVectors(new THREE.Vector3(0, 1, 0), direction);
  return {
    shaft: new THREE.Matrix4().compose(tail.clone().addScaledVector(direction, (length - head) / 2), quaternion, new THREE.Vector3(1, length - head, 1)),
    head: new THREE.Matrix4().compose(tip.clone().addScaledVector(direction, -head / 2), quaternion, new THREE.Vector3(1, head, 1)),
    tip,
    tail
  };
}

/** One shaft mesh and one head mesh per spatial group of force arrows. */
export function instancedReactionForceMeshes(groups: readonly (readonly ReactionArrowPlacement[])[]): THREE.InstancedMesh[] {
  const kept = groups.map((group) => group.filter(drawable)).filter((group) => group.length > 0);
  if (!kept.length) return [];
  const shaftGeometry = new THREE.CylinderGeometry(FORCE_SHAFT_RADIUS, FORCE_SHAFT_RADIUS, 1, 8);
  const headGeometry = new THREE.ConeGeometry(FORCE_HEAD_RADIUS, 1, 10);
  const material = createFigureMaterial();
  const output: THREE.InstancedMesh[] = [];
  for (const group of kept) {
    const shaft = new THREE.InstancedMesh(shaftGeometry, material, group.length);
    const head = new THREE.InstancedMesh(headGeometry, material, group.length);
    group.forEach((placement, instance) => {
      const matrices = reactionForceArrowMatrices(placement);
      shaft.setMatrixAt(instance, matrices.shaft);
      head.setMatrixAt(instance, matrices.head);
    });
    shaft.instanceMatrix.needsUpdate = true;
    head.instanceMatrix.needsUpdate = true;
    shaft.name = "viewport-reaction-force-shaft";
    head.name = "viewport-reaction-force-head";
    const keys = group.map((placement) => placement.ownerKey);
    registerInstancedRolePresentation(shaft, keys, "reactionForce");
    registerInstancedRolePresentation(head, keys, "reactionForce");
    output.push(shaft, head);
  }
  return output;
}

/** Moment glyphs: the rotation glyph's geometry and placement rule, under the reaction role. */
export function instancedReactionMomentMeshes(groups: readonly (readonly ReactionArrowPlacement[])[]): THREE.InstancedMesh[] {
  const placements = groups.map((group) => group.filter(drawable).map((placement): RotationGlyphPlacement => ({
    ownerKey: placement.ownerKey,
    anchor: placement.anchor,
    axis: placement.direction,
    displayLength: placement.displayLength
  })));
  return instancedRotationGlyphMeshes(placements, { role: "reactionMoment", name: "viewport-reaction-moment", radiusScale: REACTION_MOMENT_RADIUS_SCALE });
}
