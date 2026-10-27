// The contour tube drawn over a member (survey item A4; DEL-07-05 Design `result-contour.md`):
// a sleeve slightly wider than the member's own tube, from its first stated station to its last,
// with a ring of vertices at each of `contourRings` and a vertex colour per ring for each theme.
// The GPU blends colour linearly between two rings; the rings are close enough on the scale that
// the blend follows it. One merged mesh per spatial chunk, built when the overlay, threshold,
// geometry or visibility changes and never per frame. Display only: it is not picked, not haloed
// and not registered for selection; the member's own tube stays underneath, unchanged.

import * as THREE from "three";
import type { PreviewModel, Vec3 } from "../../types";
import { entityKey, type EntityKey } from "../workspace/selectionState";
import { type ResultColourOverlay } from "./resultColourOverlay";
import { contourRings, type ContourRing } from "./resultContour";
import { createFigureMaterial } from "./viewportFigureMaterial";
import { resultNoValueHex, resultScaleHex, viewportRoleHex, type ViewportPaletteTheme } from "./viewportPalette";

/** The sleeve's radius over the member's tube radius: wide enough that its inscribed polygon
 * clears the tube's ten-sided circumcircle, so the sleeve hides the tube along its length. */
export const CONTOUR_RADIUS_FACTOR = 1.12;
export const CONTOUR_RADIAL_SEGMENTS = 8;

export type ContourMeshMember = Readonly<{
  key: EntityKey;
  from: Readonly<Vec3>;
  to: Readonly<Vec3>;
  /** The member's own tube radius in the drawing. */
  radius: number;
  rings: readonly ContourRing[];
}>;

/** The members drawn along their length: those with two or more stated stations and a drawable
 * axis, minus `excluded` (hidden or dimmed members, which keep their own tube's presentation). */
export function contourMeshMembers(
  model: PreviewModel,
  overlay: ResultColourOverlay | null,
  options: {
    nodes: ReadonlyMap<string, Readonly<Vec3>>;
    radii: ReadonlyMap<EntityKey, number>;
    defaultRadius: number;
    threshold?: number | null;
    excluded?: ReadonlySet<EntityKey>;
  }
): Map<EntityKey, ContourMeshMember> {
  const members = new Map<EntityKey, ContourMeshMember>();
  if (!overlay || overlay.state !== "available" || overlay.resolution !== "station" || overlay.maximum === null) return members;
  for (const pipe of model.pipe_segments) {
    const member = overlay.members.get(pipe.id);
    const key = entityKey({ type: "pipe", id: pipe.id });
    if (!member || member.stations.length < 2 || options.excluded?.has(key)) continue;
    const from = options.nodes.get(pipe.from);
    const to = options.nodes.get(pipe.to);
    if (!from || !to) continue;
    const length = Math.hypot(to.x - from.x, to.y - from.y, to.z - from.z);
    if (!Number.isFinite(length) || length <= 0) continue;
    const rings = contourRings(member.stations, { maximum: overlay.maximum, threshold: options.threshold ?? null });
    if (rings.length < 2) continue;
    members.set(key, Object.freeze({ key, from, to, radius: options.radii.get(key) ?? options.defaultRadius, rings }));
  }
  return members;
}

/** A ring's colour in one theme: its position on the result scale, the plain pipe colour under the
 * threshold, or the no-value colour. */
export function contourRingHex(theme: ViewportPaletteTheme, ring: ContourRing): number {
  if (ring.kind === "value") return resultScaleHex(theme, ring.position);
  if (ring.kind === "below") return viewportRoleHex(theme, "pipe");
  return resultNoValueHex(theme);
}

type ContourColours = Readonly<Record<ViewportPaletteTheme, Float32Array>>;
const scratchColour = new THREE.Color();

/**
 * One merged sleeve mesh for a group of members, painted for `theme`. The mesh keeps both themes'
 * vertex colours, so a theme change rewrites the colour attribute in place (`paintResultContourTheme`).
 */
export function buildResultContourMesh(members: readonly ContourMeshMember[], theme: ViewportPaletteTheme): THREE.Mesh | null {
  const segments = CONTOUR_RADIAL_SEGMENTS;
  const ringCount = members.reduce((count, member) => count + member.rings.length, 0);
  if (members.length === 0 || ringCount === 0) return null;
  const positions = new Float32Array(ringCount * segments * 3);
  const normals = new Float32Array(ringCount * segments * 3);
  const colours: ContourColours = { light: new Float32Array(ringCount * segments * 3), dark: new Float32Array(ringCount * segments * 3) };
  const indices: number[] = [];
  const axis = new THREE.Vector3();
  const u = new THREE.Vector3();
  const v = new THREE.Vector3();
  const helper = new THREE.Vector3();
  const radial = new THREE.Vector3();
  let vertex = 0;
  for (const member of members) {
    axis.set(member.to.x - member.from.x, member.to.y - member.from.y, member.to.z - member.from.z);
    const length = axis.length();
    axis.divideScalar(length);
    helper.set(Math.abs(axis.y) < 0.9 ? 0 : 1, Math.abs(axis.y) < 0.9 ? 1 : 0, 0);
    u.crossVectors(axis, helper).normalize();
    v.crossVectors(axis, u).normalize();
    const radius = member.radius * CONTOUR_RADIUS_FACTOR;
    const first = vertex;
    member.rings.forEach((ring) => {
      const light = scratchColour.setHex(contourRingHex("light", ring));
      const [lr, lg, lb] = [light.r, light.g, light.b];
      const dark = scratchColour.setHex(contourRingHex("dark", ring));
      const [dr, dg, db] = [dark.r, dark.g, dark.b];
      for (let segment = 0; segment < segments; segment += 1) {
        const angle = (segment / segments) * Math.PI * 2;
        radial.copy(u).multiplyScalar(Math.cos(angle)).addScaledVector(v, Math.sin(angle));
        const at = vertex * 3;
        positions[at] = member.from.x + axis.x * length * ring.fraction + radial.x * radius;
        positions[at + 1] = member.from.y + axis.y * length * ring.fraction + radial.y * radius;
        positions[at + 2] = member.from.z + axis.z * length * ring.fraction + radial.z * radius;
        normals[at] = radial.x;
        normals[at + 1] = radial.y;
        normals[at + 2] = radial.z;
        colours.light[at] = lr; colours.light[at + 1] = lg; colours.light[at + 2] = lb;
        colours.dark[at] = dr; colours.dark[at + 1] = dg; colours.dark[at + 2] = db;
        vertex += 1;
      }
    });
    for (let ring = 0; ring + 1 < member.rings.length; ring += 1) {
      for (let segment = 0; segment < segments; segment += 1) {
        const a = first + ring * segments + segment;
        const b = first + ring * segments + ((segment + 1) % segments);
        const c = a + segments;
        const d = b + segments;
        // Counter-clockwise seen from outside: (u, v, axis) is right-handed, so the faces point out.
        indices.push(a, b, c, b, d, c);
      }
    }
  }
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute("position", new THREE.BufferAttribute(positions, 3));
  geometry.setAttribute("normal", new THREE.BufferAttribute(normals, 3));
  geometry.setAttribute("color", new THREE.BufferAttribute(colours[theme].slice(), 3));
  geometry.setIndex(indices);
  geometry.computeBoundingSphere();
  const material = createFigureMaterial();
  material.vertexColors = true;
  const mesh = new THREE.Mesh(geometry, material);
  mesh.name = "viewport-result-contour";
  mesh.userData.viewportContourColours = colours;
  mesh.userData.viewportContourMembers = Object.freeze(members.map((member) => member.key));
  // The sleeve is a display of the members beneath it; it is never a pick target.
  mesh.raycast = () => {};
  return mesh;
}

/** Rewrites a contour sleeve's vertex colours for a theme, in place. Any other object is untouched. */
export function paintResultContourTheme(object: THREE.Object3D, theme: ViewportPaletteTheme): void {
  const colours = object.userData.viewportContourColours as ContourColours | undefined;
  if (!colours || !(object instanceof THREE.Mesh)) return;
  const attribute = object.geometry.getAttribute("color") as THREE.BufferAttribute | undefined;
  if (!attribute) return;
  (attribute.array as Float32Array).set(colours[theme]);
  attribute.needsUpdate = true;
}
