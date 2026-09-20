import * as THREE from "three";
import type { EntityKey } from "../workspace/selectionState";

const DIMMED_OPACITY = 0.2;
const hiddenMatrix = new THREE.Matrix4().makeScale(0, 0, 0);
const scratchColor = new THREE.Color();
type Renderable = THREE.Object3D & { material?: THREE.Material | THREE.Material[] };
type Partition = {
  dimmed: THREE.InstancedMesh;
  keys: readonly EntityKey[];
  matrices: readonly THREE.Matrix4[];
  colors: Float32Array;
  normalIndices: number[];
  dimmedIndices: number[];
};
const partitions = new WeakMap<THREE.InstancedMesh, Partition>();
const companions = new WeakSet<THREE.Object3D>();
const baseMaterialState = new WeakMap<THREE.Material, {
  opacity: number; transparent: boolean; depthWrite: boolean; depthTest: boolean;
}>();

function materials(object: Renderable): THREE.Material[] {
  return Array.isArray(object.material) ? object.material : object.material ? [object.material] : [];
}

/**
 * Prepare persistent dim batches before the layer enters the ownership ledger. A companion
 * shares its parent's geometry, owns its material and instance buffers, and has identity local
 * transform. Full bounds stay conservative through every partition. No resource is created
 * by a visibility toggle. Halos see disjoint live owner lists, not two masked copies of a list.
 */
export function prepareViewportDimming(roots: readonly THREE.Object3D[]): void {
  const objects: Renderable[] = [];
  for (const root of roots) root.traverse((object) => objects.push(object));
  // Existing noninstanced graphics can share materials. Split only across different owners
  // (including unowned graphics), retaining the original on one group so nothing is orphaned.
  const uses = new Map<THREE.Material, Map<EntityKey | null, Renderable[]>>();
  for (const object of objects) {
    const owner = object instanceof THREE.InstancedMesh ? null
      : object.userData.selectionEntityKey as EntityKey | undefined ?? null;
    for (const material of materials(object)) {
      const groups = uses.get(material) ?? new Map<EntityKey | null, Renderable[]>();
      const group = groups.get(owner) ?? [];
      group.push(object);
      groups.set(owner, group);
      uses.set(material, groups);
    }
  }
  for (const [material, groups] of uses) {
    const owners = [...groups.keys()].sort((a, b) => a === null ? -1 : b === null ? 1 : 0);
    owners.forEach((owner, index) => {
      const owned = index === 0 ? material : material.clone();
      if (owner !== null && !baseMaterialState.has(owned)) {
        baseMaterialState.set(owned, {
          opacity: owned.opacity, transparent: owned.transparent, depthWrite: owned.depthWrite, depthTest: owned.depthTest
        });
      }
      if (owned === material) return;
      for (const object of groups.get(owner)!) {
        object.material = Array.isArray(object.material)
          ? object.material.map((entry) => entry === material ? owned : entry) : owned;
      }
    });
  }
  for (const object of objects) {
    if (!(object instanceof THREE.InstancedMesh) || companions.has(object) || partitions.has(object)) continue;
    const keys = object.userData.instanceEntityKeys as readonly EntityKey[] | undefined;
    const matrices = object.userData.instanceBaseMatrices as readonly THREE.Matrix4[] | undefined;
    if (!keys || !matrices || keys.length === 0) continue;
    if (!object.instanceColor) {
      scratchColor.setHex(object.userData.viewportBaseColor as number);
      keys.forEach((_, index) => object.setColorAt(index, scratchColor));
    }
    const dimMaterials = materials(object).map((material) => {
      const dimmed = material.clone();
      dimmed.opacity = DIMMED_OPACITY;
      dimmed.transparent = true;
      // Dim context must not write a solid occluder into the depth buffer. Depth testing
      // remains as authored; opaque figure draws first and both batches precede the halo.
      dimmed.depthWrite = false;
      return dimmed;
    });
    const dimmed = new THREE.InstancedMesh(object.geometry,
      Array.isArray(object.material) ? dimMaterials : dimMaterials[0], keys.length);
    dimmed.name = `${object.name || "viewport"}-dimmed`;
    dimmed.userData = { ...object.userData, instanceEntityKeys: [], instanceBaseMatrices: [] };
    dimmed.instanceMatrix.setUsage(object.instanceMatrix.usage);
    dimmed.instanceColor = object.instanceColor
      ? new THREE.InstancedBufferAttribute(object.instanceColor.array.slice(), object.instanceColor.itemSize, object.instanceColor.normalized) : null;
    dimmed.boundingBox = object.boundingBox?.clone() ?? null;
    dimmed.boundingSphere = object.boundingSphere?.clone() ?? null;
    dimmed.frustumCulled = object.frustumCulled;
    dimmed.renderOrder = object.renderOrder;
    dimmed.count = 0;
    companions.add(dimmed);
    partitions.set(object, {
      dimmed, keys: [...keys], matrices: [...matrices],
      colors: new Float32Array(object.instanceColor!.array),
      normalIndices: keys.map((_, index) => index), dimmedIndices: []
    });
    object.add(dimmed);
  }
}

/** Applies only on visibility/model changes, never in the render loop. */
export function applyViewportDimming(
  roots: readonly THREE.Object3D[],
  hiddenKeys: ReadonlySet<EntityKey>,
  dimmedKeys: ReadonlySet<EntityKey>
): void {
  for (const root of roots) root.traverse((object) => {
    if (companions.has(object)) return;
    if (object instanceof THREE.InstancedMesh) {
      const partition = partitions.get(object);
      const keys = partition?.keys ?? object.userData.instanceEntityKeys as readonly EntityKey[] | undefined;
      const matrices = partition?.matrices ?? object.userData.instanceBaseMatrices as readonly THREE.Matrix4[] | undefined;
      if (!keys || !matrices) return;
      if (!partition) {
        keys.forEach((key, index) => object.setMatrixAt(index, hiddenKeys.has(key) ? hiddenMatrix : matrices[index]));
        object.instanceMatrix.needsUpdate = true;
        return;
      }
      // Capture both live partitions before writing either: slot compaction must not erase
      // a later source colour. Theme repaint may have changed these since the last partition.
      for (const [batch, indices] of [[object, partition.normalIndices], [partition.dimmed, partition.dimmedIndices]] as const) {
        indices.forEach((canonicalIndex, slot) => {
          batch.getColorAt(slot, scratchColor);
          scratchColor.toArray(partition.colors, canonicalIndex * 3);
        });
      }
      const normalIndices: number[] = [], dimmedIndices: number[] = [];
      const normalKeys: EntityKey[] = [], dimKeys: EntityKey[] = [];
      const normalMatrices: THREE.Matrix4[] = [], dimMatrices: THREE.Matrix4[] = [];
      keys.forEach((key, sourceIndex) => {
        const isDimmed = !hiddenKeys.has(key) && dimmedKeys.has(key);
        const target = isDimmed ? partition.dimmed : object;
        const targetKeys = isDimmed ? dimKeys : normalKeys;
        const targetMatrices = isDimmed ? dimMatrices : normalMatrices;
        const at = targetKeys.length;
        targetKeys.push(key);
        (isDimmed ? dimmedIndices : normalIndices).push(sourceIndex);
        targetMatrices.push(matrices[sourceIndex]);
        target.setMatrixAt(at, hiddenKeys.has(key) ? hiddenMatrix : matrices[sourceIndex]);
        target.setColorAt(at, scratchColor.fromArray(partition.colors, sourceIndex * 3));
      });
      for (const [target, targetKeys, targetMatrices] of [
        [object, normalKeys, normalMatrices], [partition.dimmed, dimKeys, dimMatrices]
      ] as const) {
        target.count = targetKeys.length;
        target.userData.instanceEntityKeys = targetKeys;
        target.userData.instanceBaseMatrices = targetMatrices;
        target.instanceMatrix.needsUpdate = true;
        if (target.instanceColor) target.instanceColor.needsUpdate = true;
      }
      partition.normalIndices = normalIndices;
      partition.dimmedIndices = dimmedIndices;
      // A zero-count normal mesh must stay visible: its dimmed child still draws.
      return;
    }
    const key = object.userData.selectionEntityKey as EntityKey | undefined;
    if (!key) return;
    object.visible = !hiddenKeys.has(key);
    for (const material of materials(object)) {
      const base = baseMaterialState.get(material);
      if (!base) continue;
      const dimmed = !hiddenKeys.has(key) && dimmedKeys.has(key);
      const transparent = dimmed ? true : base.transparent;
      if (material.transparent !== transparent) material.needsUpdate = true;
      material.opacity = dimmed ? DIMMED_OPACITY : base.opacity;
      material.transparent = transparent;
      material.depthWrite = dimmed ? false : base.depthWrite;
      material.depthTest = base.depthTest;
    }
  });
}
