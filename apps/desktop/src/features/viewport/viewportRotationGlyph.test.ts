import * as THREE from "three";
import { describe, expect, it } from "vitest";
import { entityKey } from "../workspace/selectionState";
import { instancedRotationGlyphMeshes, rotationGlyphMatrices, type RotationGlyphPlacement } from "./viewportRotationGlyph";

const placement = (id: string, axis: { x: number; y: number; z: number }, displayLength = 0.5, anchor = { x: 1, y: 2, z: 3 }): RotationGlyphPlacement => ({
  ownerKey: entityKey({ type: "node", id }),
  anchor,
  axis,
  displayLength
});

function decompose(matrix: THREE.Matrix4) {
  const position = new THREE.Vector3();
  const quaternion = new THREE.Quaternion();
  const scale = new THREE.Vector3();
  matrix.decompose(position, quaternion, scale);
  return { position, direction: new THREE.Vector3(0, 1, 0).applyQuaternion(quaternion), scale };
}

describe("rotation glyph geometry", () => {
  it("draws the shaft and both heads along the rotation axis from the anchor", () => {
    const { shaft, outerHead, innerHead } = rotationGlyphMatrices(placement("node:N-1", { x: 0, y: 0, z: -1 }));
    const head = Math.min(0.09, 0.5 * 0.3);
    for (const matrix of [shaft, outerHead, innerHead]) {
      const { direction } = decompose(matrix);
      expect(direction.x).toBeCloseTo(0, 12);
      expect(direction.y).toBeCloseTo(0, 12);
      expect(direction.z).toBeCloseTo(-1, 12);
    }
    const shaftParts = decompose(shaft);
    expect(shaftParts.scale.y).toBeCloseTo(0.5 - head, 12);
    expect(shaftParts.position.toArray().map((value) => Number(value.toFixed(12)))).toEqual([1, 2, 3 - (0.5 - head) / 2]);
    expect(decompose(outerHead).position.z).toBeCloseTo(3 - (0.5 - head / 2), 12);
    expect(decompose(innerHead).position.z).toBeCloseTo(3 - (0.5 - head * 1.5), 12);
    // The two heads are distinct: a double-headed arrow, not a load arrow.
    expect(decompose(outerHead).position.distanceTo(decompose(innerHead).position)).toBeCloseTo(head, 12);
  });

  it("scales the heads down for short glyphs", () => {
    const { outerHead } = rotationGlyphMatrices(placement("node:N-1", { x: 1, y: 0, z: 0 }, 0.1));
    expect(decompose(outerHead).scale.y).toBeCloseTo(0.03, 12);
  });

  it("builds one shaft and two head instances per glyph, owned by its node", () => {
    const meshes = instancedRotationGlyphMeshes([
      [placement("node:N-1", { x: 1, y: 0, z: 0 }), placement("node:N-2", { x: 0, y: 1, z: 0 }, 0.2)],
      [placement("node:N-3", { x: 0, y: 0, z: 1 }, 0.3)]
    ]);
    expect(meshes.map((mesh) => [mesh.name, mesh.count])).toEqual([
      ["viewport-rotation-glyph-shaft", 2],
      ["viewport-rotation-glyph-head", 4],
      ["viewport-rotation-glyph-shaft", 1],
      ["viewport-rotation-glyph-head", 2]
    ]);
    expect(meshes[0].userData.instanceEntityKeys).toEqual([entityKey({ type: "node", id: "node:N-1" }), entityKey({ type: "node", id: "node:N-2" })]);
    expect(meshes[1].userData.instanceEntityKeys).toEqual([
      entityKey({ type: "node", id: "node:N-1" }), entityKey({ type: "node", id: "node:N-1" }),
      entityKey({ type: "node", id: "node:N-2" }), entityKey({ type: "node", id: "node:N-2" })
    ]);
    expect(meshes.every((mesh) => mesh.userData.viewportPaletteRole === "deformedShape")).toBe(true);
  });

  it("draws nothing for zero, non-finite or empty glyphs", () => {
    expect(instancedRotationGlyphMeshes([])).toEqual([]);
    expect(instancedRotationGlyphMeshes([[
      placement("node:N-1", { x: 1, y: 0, z: 0 }, 0),
      placement("node:N-2", { x: 0, y: 0, z: 0 }),
      placement("node:N-3", { x: Number.NaN, y: 0, z: 0 }),
      placement("node:N-4", { x: 1, y: 0, z: 0 }, Number.POSITIVE_INFINITY)
    ]])).toEqual([]);
  });
});
