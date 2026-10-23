// Instanced rotation glyphs (DEL-07-05:1): one double-headed arrow per node,
// along that node's rotation vector. By the right-hand rule, the thumb points
// along the arrow and the fingers curl in the sense of rotation. The double
// head tells a rotation glyph apart from the single-headed load arrows. This is
// a display of emitted rx/ry/rz rows only.

import * as THREE from "three";
import type { Vec3 } from "../../types";
import type { EntityKey } from "../workspace/selectionState";
import { createFigureMaterial } from "./viewportFigureMaterial";
import type { ViewportPaletteRole } from "./viewportPalette";
import { registerInstancedRolePresentation } from "./viewportResource";

export type RotationGlyphPlacement = {
  ownerKey: EntityKey;
  /** Glyph origin in render-local coordinates. */
  anchor: Vec3;
  /** Unit rotation-vector direction. */
  axis: Vec3;
  /** Display length in render-local units. */
  displayLength: number;
};

const SHAFT_RADIUS = 0.016;
const HEAD_RADIUS = 0.055;
const HEAD_LENGTH_LIMIT = 0.09;

/** Shaft, outer head and inner head instance matrices for one glyph. */
export function rotationGlyphMatrices(glyph: RotationGlyphPlacement): { shaft: THREE.Matrix4; outerHead: THREE.Matrix4; innerHead: THREE.Matrix4 } {
  const axis = new THREE.Vector3(glyph.axis.x, glyph.axis.y, glyph.axis.z).normalize();
  const anchor = new THREE.Vector3(glyph.anchor.x, glyph.anchor.y, glyph.anchor.z);
  const length = glyph.displayLength;
  const head = Math.min(HEAD_LENGTH_LIMIT, length * 0.3);
  const quaternion = new THREE.Quaternion().setFromUnitVectors(new THREE.Vector3(0, 1, 0), axis);
  const at = (distance: number) => anchor.clone().addScaledVector(axis, distance);
  return {
    shaft: new THREE.Matrix4().compose(at((length - head) / 2), quaternion, new THREE.Vector3(1, length - head, 1)),
    outerHead: new THREE.Matrix4().compose(at(length - head / 2), quaternion, new THREE.Vector3(1, head, 1)),
    innerHead: new THREE.Matrix4().compose(at(length - head * 1.5), quaternion, new THREE.Vector3(1, head, 1))
  };
}

/** Builds instanced glyph meshes, one shaft mesh and one head mesh (two heads per
 * glyph) per spatial group. Each instance belongs to its owner for selection
 * presentation, under the deformed-shape palette role unless another role is
 * given. The support reaction moments use the same glyph under their own role
 * and name (`viewportReactionArrows.ts`). */
export function instancedRotationGlyphMeshes(
  groups: readonly (readonly RotationGlyphPlacement[])[],
  options: Readonly<{ role?: ViewportPaletteRole; name?: string; radiusScale?: number }> = {}
): THREE.InstancedMesh[] {
  const role = options.role ?? "deformedShape";
  const name = options.name ?? "viewport-rotation-glyph";
  const radiusScale = options.radiusScale ?? 1;
  const drawable = groups
    .map((group) => group.filter((glyph) => Number.isFinite(glyph.displayLength) && glyph.displayLength > 0 &&
      Number.isFinite(Math.hypot(glyph.axis.x, glyph.axis.y, glyph.axis.z)) && Math.hypot(glyph.axis.x, glyph.axis.y, glyph.axis.z) > 0))
    .filter((group) => group.length > 0);
  if (!drawable.length) return [];
  const shaftGeometry = new THREE.CylinderGeometry(SHAFT_RADIUS * radiusScale, SHAFT_RADIUS * radiusScale, 1, 8);
  const headGeometry = new THREE.ConeGeometry(HEAD_RADIUS * radiusScale, 1, 10);
  const material = createFigureMaterial({ opacity: 0.9, transparent: true });
  const output: THREE.InstancedMesh[] = [];
  for (const group of drawable) {
    const shaft = new THREE.InstancedMesh(shaftGeometry, material, group.length);
    const heads = new THREE.InstancedMesh(headGeometry, material, group.length * 2);
    group.forEach((glyph, instance) => {
      const matrices = rotationGlyphMatrices(glyph);
      shaft.setMatrixAt(instance, matrices.shaft);
      heads.setMatrixAt(instance * 2, matrices.outerHead);
      heads.setMatrixAt(instance * 2 + 1, matrices.innerHead);
    });
    shaft.instanceMatrix.needsUpdate = true;
    heads.instanceMatrix.needsUpdate = true;
    shaft.name = `${name}-shaft`;
    heads.name = `${name}-head`;
    registerInstancedRolePresentation(shaft, group.map((glyph) => glyph.ownerKey), role);
    registerInstancedRolePresentation(heads, group.flatMap((glyph) => [glyph.ownerKey, glyph.ownerKey]), role);
    output.push(shaft, heads);
  }
  return output;
}
