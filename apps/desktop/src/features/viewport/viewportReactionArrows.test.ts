import * as THREE from "three";
import { describe, expect, it } from "vitest";
import { entityKey } from "../workspace/selectionState";
import { viewportRoleHex } from "./viewportPalette";
import { disposeObjectChildren, ViewportOwnershipLedger } from "./viewportResource";
import {
  instancedReactionForceMeshes,
  instancedReactionMomentMeshes,
  REACTION_FORCE_TIP_GAP,
  reactionForceArrowMatrices,
  type ReactionArrowPlacement
} from "./viewportReactionArrows";

// Reaction arrow geometry (OD-5): a force arrives at its support's node along the emitted force;
// a moment is the rotation glyph's double-headed arrow. Invented placements only.

const placement = (id: string, direction: { x: number; y: number; z: number }, displayLength = 0.6, anchor = { x: 1, y: 2, z: 3 }): ReactionArrowPlacement => ({
  ownerKey: entityKey({ type: "support", id }),
  anchor,
  direction,
  displayLength
});

function decompose(matrix: THREE.Matrix4) {
  const position = new THREE.Vector3();
  const quaternion = new THREE.Quaternion();
  const scale = new THREE.Vector3();
  matrix.decompose(position, quaternion, scale);
  return { position, direction: new THREE.Vector3(0, 1, 0).applyQuaternion(quaternion), scale };
}

const instanceHex = (mesh: THREE.InstancedMesh, index: number) => new THREE.Color().fromBufferAttribute(mesh.instanceColor!, index).getHex();

describe("reaction force arrow geometry", () => {
  it("points along the force and ends its head just short of the node", () => {
    const { shaft, head, tip, tail } = reactionForceArrowMatrices(placement("support:S-1", { x: 0, y: 0, z: -1 }));
    expect(tip.toArray().map((value) => Number(value.toFixed(12)))).toEqual([1, 2, 3 + REACTION_FORCE_TIP_GAP]);
    expect(tail.toArray().map((value) => Number(value.toFixed(12)))).toEqual([1, 2, Number((3 + REACTION_FORCE_TIP_GAP + 0.6).toFixed(12))]);
    for (const matrix of [shaft, head]) {
      const { direction } = decompose(matrix);
      expect(direction.z).toBeCloseTo(-1, 12);
    }
    // The cone's apex (+y/2 in its own frame) is the tip.
    const apex = new THREE.Vector3(0, 0.5, 0).applyMatrix4(head);
    expect(apex.distanceTo(tip)).toBeCloseTo(0, 12);
    // Shaft and head together span the display length.
    expect(decompose(shaft).scale.y + decompose(head).scale.y).toBeCloseTo(0.6, 12);
  });

  it("reverses with the sign of the force", () => {
    const up = reactionForceArrowMatrices(placement("support:S-1", { x: 0, y: 1, z: 0 }));
    const down = reactionForceArrowMatrices(placement("support:S-1", { x: 0, y: -1, z: 0 }));
    expect(decompose(up.head).direction.y).toBeCloseTo(1, 12);
    expect(decompose(down.head).direction.y).toBeCloseTo(-1, 12);
    expect(up.tail.y).toBeLessThan(2);
    expect(down.tail.y).toBeGreaterThan(2);
  });
});

describe("reaction meshes", () => {
  it("builds one shaft and one head per spatial group of forces, owned by the support, in the reaction role", () => {
    const meshes = instancedReactionForceMeshes([
      [placement("support:S-1", { x: 1, y: 0, z: 0 }), placement("support:S-2", { x: 0, y: 1, z: 0 }, 0)],
      [placement("support:S-3", { x: 0, y: 0, z: 1 })]
    ]);
    expect(meshes.map((mesh) => [mesh.name, mesh.count])).toEqual([
      ["viewport-reaction-force-shaft", 1],
      ["viewport-reaction-force-head", 1],
      ["viewport-reaction-force-shaft", 1],
      ["viewport-reaction-force-head", 1]
    ]);
    expect(meshes[0].userData.instanceEntityKeys).toEqual([entityKey({ type: "support", id: "support:S-1" })]);
    expect(meshes[0].userData.viewportPaletteRole).toBe("reactionForce");
    expect(instanceHex(meshes[0], 0)).toBe(viewportRoleHex("light", "reactionForce"));
    expect(instancedReactionForceMeshes([[placement("support:S-2", { x: 0, y: 0, z: 0 })]])).toEqual([]);
  });

  it("draws moments as heavier double-headed rotation glyphs in the reaction role", () => {
    const meshes = instancedReactionMomentMeshes([[placement("support:S-1", { x: 0, y: 0, z: 1 }, 0.4)]]);
    expect(meshes.map((mesh) => [mesh.name, mesh.count])).toEqual([["viewport-reaction-moment-shaft", 1], ["viewport-reaction-moment-head", 2]]);
    expect(meshes[1].userData.viewportPaletteRole).toBe("reactionMoment");
    const shaftRadius = (meshes[0].geometry as THREE.CylinderGeometry).parameters.radiusTop;
    expect(shaftRadius).toBeCloseTo(0.016 * 1.5, 12);
  });

  it("enters the ownership ledger and leaves it exactly on disposal", () => {
    const ledger = new ViewportOwnershipLedger(41);
    const root = new THREE.Group();
    root.add(
      ...instancedReactionForceMeshes([[placement("support:S-1", { x: 1, y: 0, z: 0 })]]),
      ...instancedReactionMomentMeshes([[placement("support:S-1", { x: 0, y: 1, z: 0 })]])
    );
    ledger.createObjects(root.children);
    // Two geometries and one material per kind; four instanced meshes with colours.
    expect(ledger.snapshot().live).toMatchObject({ geometries: 4, materials: 2, instanceMatrices: 4, instanceColors: 4 });
    ledger.disposeObjects(root.children);
    disposeObjectChildren(root);
    expect(ledger.snapshot().created).toEqual(ledger.snapshot().disposed);
  });
});
