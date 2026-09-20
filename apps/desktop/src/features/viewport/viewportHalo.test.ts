import { describe, expect, it, vi } from "vitest";
import * as THREE from "three";
import { entityKey, type EntityKey } from "../workspace/selectionState";
import {
  HALO_HOVER_RENDER_ORDER,
  HALO_MASK_RENDER_ORDER,
  HALO_SELECTION_RENDER_ORDER,
  haloEdgeGeometryFor,
  haloShellFormFor,
  ViewportHaloPresentation
} from "./viewportHalo";
import {
  registerInstancedRolePresentation,
  ViewportOwnershipLedger,
  ViewportResource
} from "./viewportResource";
import { createFigureMaterial, figureEdgeOutlineFor } from "./viewportFigureMaterial";
import { viewportRoleHex, viewportTokenColour } from "./viewportPalette";

const pipeKey = (id: number | string): EntityKey => entityKey({ type: "pipe", id: `p:${id}` });
const nodeKey = (id: number | string): EntityKey => entityKey({ type: "node", id: `n:${id}` });

const instanceHex = (mesh: THREE.InstancedMesh, index: number) =>
  new THREE.Color().fromBufferAttribute(mesh.instanceColor!, index).getHex();

function pipes(keys: readonly EntityKey[], geometry = new THREE.CylinderGeometry(1, 1, 1, 10, 1, false)) {
  const mesh = new THREE.InstancedMesh(geometry, createFigureMaterial({ edge: figureEdgeOutlineFor(geometry) }), keys.length);
  keys.forEach((_, index) => mesh.setMatrixAt(index, new THREE.Matrix4().makeTranslation(index + 1, 2, 3)));
  registerInstancedRolePresentation(mesh, keys, "pipe");
  return mesh;
}

function nodes(keys: readonly EntityKey[], geometry = new THREE.SphereGeometry(0.095, 12, 8)) {
  const mesh = new THREE.InstancedMesh(geometry, createFigureMaterial(), keys.length);
  keys.forEach((_, index) => mesh.setMatrixAt(index, new THREE.Matrix4().makeTranslation(index, 0, 0)));
  registerInstancedRolePresentation(mesh, keys, "node");
  return mesh;
}

/** A resource on the real prototype with the fields its presentation paths read, and a real halo. */
function haloResource(theme: "light" | "dark" = "light", pixelRatio = 1) {
  const layers = {
    modelLayer: new THREE.Group(),
    authoredLoadLayer: new THREE.Group(),
    resultLayer: new THREE.Group(),
    diagnosticLayer: new THREE.Group(),
    routingLayer: new THREE.Group()
  };
  const invalidate = vi.fn();
  const ownership = new ViewportOwnershipLedger(31);
  const halo = new ViewportHaloPresentation({
    created: (counts) => ownership.createLifecycle(counts),
    disposed: (counts) => ownership.disposeLifecycle(counts)
  });
  halo.setTheme(theme);
  halo.setViewport(800 * pixelRatio, 600 * pixelRatio, pixelRatio);
  const scene = new THREE.Scene();
  scene.add(...Object.values(layers), halo.group);
  const resource = Object.assign(Object.create(ViewportResource.prototype), {
    ownership,
    resourceGeneration: 31,
    contextLostCount: 0,
    contextRestoredCount: 0,
    scheduler: { pendingCount: 0 },
    renderer: { domElement: { isConnected: true }, getPixelRatio: () => pixelRatio },
    options: {},
    invalidate,
    scene,
    gizmoScene: new THREE.Scene(),
    selectionPresentation: null,
    halo,
    ...layers,
    selectedKeys: new Set<EntityKey>(),
    hiddenKeys: new Set<EntityKey>(),
    hoveredKey: null,
    pickables: [],
    pointPrimitives: [],
    modelIndex: null,
    themePresentation: theme,
    gridVisible: true,
    authoredLoadsVisible: true
  }) as ViewportResource;
  return { resource, ownership, invalidate, halo, scene, ...layers };
}

function haloMatrix(mesh: THREE.InstancedMesh, index: number): number[] {
  return Array.from((mesh.instanceMatrix.array as Float32Array).subarray(index * 16, index * 16 + 16));
}

describe("selection halo", () => {
  it("leaves a selected instance at its base colour and gives it one halo instance at its own matrix", () => {
    const { resource, halo, modelLayer } = haloResource();
    const mesh = pipes([pipeKey(1), pipeKey(2), pipeKey(3)]);
    resource.replaceLayer(modelLayer, [mesh]);
    expect(halo.count("selection")).toBe(0);

    resource.setSelectionPresentation([pipeKey(2)]);
    for (const index of [0, 1, 2]) expect(instanceHex(mesh, index)).toBe(viewportRoleHex("light", "pipe"));
    const pair = halo.pairFor("selection", mesh.geometry)!;
    expect(pair.count).toBe(1);
    expect(pair.mask.count).toBe(1);
    expect(pair.shell.count).toBe(1);
    expect(pair.mask.visible && pair.shell.visible).toBe(true);
    const expected = new THREE.Matrix4().makeTranslation(2, 2, 3).toArray();
    expect(haloMatrix(pair.mask, 0)).toEqual(expected);
    expect(haloMatrix(pair.shell, 0)).toEqual(expected);
    // The mask is the element's own geometry; the shell of a prism is its edges.
    expect(pair.mask.geometry).toBe(mesh.geometry);
    expect(pair.shell.geometry).not.toBe(mesh.geometry);
  });

  it("halos every selected element, of every shape, and removes the halo on deselection", () => {
    const { resource, halo, modelLayer } = haloResource();
    const tubeA = pipes([pipeKey(1), pipeKey(2)]);
    // A second spatial chunk of the same shape shares the geometry, as the builders do.
    const tubeB = pipes([pipeKey(3)], tubeA.geometry);
    const spheres = nodes([nodeKey(1), nodeKey(2)]);
    resource.replaceLayer(modelLayer, [tubeA, tubeB, spheres]);

    resource.setSelectionPresentation([pipeKey(1), pipeKey(3), nodeKey(2)]);
    expect(halo.pairFor("selection", tubeA.geometry)!.count).toBe(2);
    expect(halo.pairFor("selection", spheres.geometry)!.count).toBe(1);
    expect(halo.count("selection")).toBe(3);
    // A sphere's shell is its own geometry, grown along its normals.
    expect(halo.pairFor("selection", spheres.geometry)!.shell.geometry).toBe(spheres.geometry);

    resource.setSelectionPresentation([]);
    expect(halo.count("selection")).toBe(0);
    for (const geometry of [tubeA.geometry, spheres.geometry]) {
      const pair = halo.pairFor("selection", geometry)!;
      expect(pair.mask.count).toBe(0);
      expect(pair.mask.visible || pair.shell.visible).toBe(false);
    }
  });

  it("gives a hidden selected element no halo, and gives it back when shown", () => {
    const { resource, halo, modelLayer } = haloResource();
    const mesh = pipes([pipeKey(1), pipeKey(2)]);
    resource.replaceLayer(modelLayer, [mesh]);
    resource.setSelectionPresentation([pipeKey(1), pipeKey(2)]);
    expect(halo.count("selection")).toBe(2);

    resource.setVisibilityPresentation(new Set([pipeKey(1)]));
    const pair = halo.pairFor("selection", mesh.geometry)!;
    expect(pair.count).toBe(1);
    // The one left is the visible element, at its base matrix and not at the zero of a hidden one.
    expect(haloMatrix(pair.mask, 0)).toEqual(new THREE.Matrix4().makeTranslation(2, 2, 3).toArray());

    resource.setVisibilityPresentation(new Set());
    expect(halo.count("selection")).toBe(2);
    expect(haloMatrix(pair.mask, 0)).toEqual(new THREE.Matrix4().makeTranslation(1, 2, 3).toArray());
  });

  it("halos from the model layer alone: a load arrow or an overlay that carries the key is neither recoloured nor haloed", () => {
    const { resource, halo, modelLayer, authoredLoadLayer } = haloResource();
    const mesh = pipes([pipeKey(1)]);
    const arrows = new THREE.InstancedMesh(new THREE.CylinderGeometry(0.1, 0.1, 1, 6), new THREE.MeshBasicMaterial(), 1);
    registerInstancedRolePresentation(arrows, [pipeKey(1)], "loadForce");
    resource.replaceLayer(modelLayer, [mesh]);
    resource.replaceLayer(authoredLoadLayer, [arrows]);
    resource.setSelectionPresentation([pipeKey(1)]);
    expect(instanceHex(arrows, 0)).toBe(viewportRoleHex("light", "loadForce"));
    expect(halo.pairFor("selection", arrows.geometry)).toBeNull();
    expect(halo.count("selection")).toBe(1);
  });
});

describe("hover halo", () => {
  it("gives an unselected hovered element exactly one hover halo and a selected one none", () => {
    const { resource, halo, modelLayer } = haloResource();
    const mesh = pipes([pipeKey(1), pipeKey(2)]);
    resource.replaceLayer(modelLayer, [mesh]);
    resource.setSelectionPresentation([pipeKey(1)]);

    resource.setHoverPresentation(pipeKey(2));
    expect(halo.count("hover")).toBe(1);
    expect(haloMatrix(halo.pairFor("hover", mesh.geometry)!.mask, 0)).toEqual(new THREE.Matrix4().makeTranslation(2, 2, 3).toArray());
    expect(halo.count("selection")).toBe(1);

    resource.setHoverPresentation(pipeKey(1));
    expect(halo.count("hover")).toBe(0);

    // Selecting the hovered element takes its hover halo away; deselecting brings it back.
    resource.setHoverPresentation(pipeKey(2));
    resource.setSelectionPresentation([pipeKey(1), pipeKey(2)]);
    expect(halo.count("hover")).toBe(0);
    resource.setSelectionPresentation([pipeKey(1)]);
    expect(halo.count("hover")).toBe(1);

    resource.setVisibilityPresentation(new Set([pipeKey(2)]));
    expect(halo.count("hover")).toBe(0);
    resource.setVisibilityPresentation(new Set());
    resource.setHoverPresentation(null);
    expect(halo.count("hover")).toBe(0);
  });

  it("costs one invalidation per change of hovered element and nothing for the same element again", () => {
    const { resource, halo, invalidate, ownership, modelLayer } = haloResource();
    const mesh = pipes([pipeKey(1), pipeKey(2)]);
    resource.replaceLayer(modelLayer, [mesh]);
    resource.setHoverPresentation(pipeKey(1));
    const ledger = ownership.snapshot();
    const replaced = vi.spyOn(resource, "replaceLayer");
    const sync = vi.spyOn(halo, "sync");
    const syncHover = vi.spyOn(halo, "syncHover");
    invalidate.mockClear();

    resource.setHoverPresentation(pipeKey(1));
    expect(invalidate).not.toHaveBeenCalled();
    expect(syncHover).not.toHaveBeenCalled();

    resource.setHoverPresentation(pipeKey(2));
    resource.setHoverPresentation(pipeKey(2));
    resource.setHoverPresentation(null);
    expect(invalidate).toHaveBeenCalledTimes(2);
    expect(syncHover).toHaveBeenCalledTimes(2);
    expect(sync).not.toHaveBeenCalled();
    expect(replaced).not.toHaveBeenCalled();
    // Moving between elements of one shape creates and disposes nothing.
    expect(ownership.snapshot()).toEqual(ledger);
  });
});

describe("halo colours and width", () => {
  const shellUniforms = (halo: ViewportHaloPresentation, kind: "selection" | "hover", geometry: THREE.BufferGeometry) =>
    (halo.pairFor(kind, geometry)!.shell.material as THREE.ShaderMaterial).uniforms;

  it("paints each halo with its token in each theme, in place across two theme switches", () => {
    const { resource, halo, ownership, invalidate, modelLayer } = haloResource("light");
    const mesh = pipes([pipeKey(1), pipeKey(2)]);
    const spheres = nodes([nodeKey(1)]);
    resource.replaceLayer(modelLayer, [mesh, spheres]);
    resource.setSelectionPresentation([pipeKey(1), nodeKey(1)]);
    resource.setHoverPresentation(pipeKey(2));
    const hex = (kind: "selection" | "hover", geometry: THREE.BufferGeometry) =>
      (shellUniforms(halo, kind, geometry).haloColour.value as THREE.Color).getHex();
    expect(hex("selection", mesh.geometry)).toBe(viewportTokenColour("light", "canvas.selection").hex);
    expect(hex("selection", spheres.geometry)).toBe(viewportTokenColour("light", "canvas.selection").hex);
    expect(hex("hover", mesh.geometry)).toBe(viewportTokenColour("light", "canvas.hover").hex);

    const objects = halo.group.children.slice() as THREE.InstancedMesh[];
    const identities = objects.map((mesh) => ({ geometry: mesh.geometry, material: mesh.material, matrix: mesh.instanceMatrix }));
    const disposals = objects.flatMap((object) => [
      vi.spyOn(object, "dispose"), vi.spyOn(object.geometry, "dispose"), vi.spyOn(object.material as THREE.Material, "dispose")
    ]);
    const ledger = ownership.snapshot();
    invalidate.mockClear();

    resource.setThemePresentation("dark");
    expect(invalidate).toHaveBeenCalledTimes(1);
    expect(hex("selection", mesh.geometry)).toBe(viewportTokenColour("dark", "canvas.selection").hex);
    expect(hex("selection", spheres.geometry)).toBe(viewportTokenColour("dark", "canvas.selection").hex);
    expect(hex("hover", mesh.geometry)).toBe(viewportTokenColour("dark", "canvas.hover").hex);
    expect(instanceHex(mesh, 0)).toBe(viewportRoleHex("dark", "pipe"));

    resource.setThemePresentation("light");
    expect(invalidate).toHaveBeenCalledTimes(2);
    expect(hex("selection", mesh.geometry)).toBe(viewportTokenColour("light", "canvas.selection").hex);
    expect(hex("hover", mesh.geometry)).toBe(viewportTokenColour("light", "canvas.hover").hex);

    expect(ownership.snapshot()).toEqual(ledger);
    expect(halo.group.children).toEqual(objects);
    objects.forEach((object, index) => {
      expect(object.geometry).toBe(identities[index].geometry);
      expect(object.material).toBe(identities[index].material);
      expect(object.instanceMatrix).toBe(identities[index].matrix);
    });
    for (const dispose of disposals) expect(dispose).not.toHaveBeenCalled();
    expect(halo.count("selection")).toBe(2);
    expect(halo.count("hover")).toBe(1);
  });

  it.each([1, 2])("draws selection at 2 CSS px and hover at 1 CSS px at pixel ratio %s", (pixelRatio) => {
    const { resource, halo, modelLayer } = haloResource("light", pixelRatio);
    const mesh = pipes([pipeKey(1), pipeKey(2)]);
    resource.replaceLayer(modelLayer, [mesh]);
    resource.setSelectionPresentation([pipeKey(1)]);
    resource.setHoverPresentation(pipeKey(2));
    expect(shellUniforms(halo, "selection", mesh.geometry).haloWidth.value).toBe(2 * pixelRatio);
    expect(shellUniforms(halo, "hover", mesh.geometry).haloWidth.value).toBe(1 * pixelRatio);
    expect((shellUniforms(halo, "selection", mesh.geometry).haloViewport.value as THREE.Vector2).toArray())
      .toEqual([800 * pixelRatio, 600 * pixelRatio]);
  });

  it("reads the width's pixel ratio and the buffer size from the renderer where it is sized", () => {
    const halo = new ViewportHaloPresentation();
    const mesh = pipes([pipeKey(1)]);
    halo.sync([mesh], new Set([pipeKey(1)]), new Set(), null);
    halo.setViewportFromRenderer({
      getDrawingBufferSize: (target: THREE.Vector2) => target.set(1280, 720),
      getPixelRatio: () => 2
    });
    const uniforms = shellUniforms(halo, "selection", mesh.geometry);
    expect(uniforms.haloWidth.value).toBe(4);
    expect((uniforms.haloViewport.value as THREE.Vector2).toArray()).toEqual([1280, 720]);
  });

  it("draws unlit and untoned, masks before shells, hover under selection, all before the first profile's cue", () => {
    const halo = new ViewportHaloPresentation();
    const mesh = pipes([pipeKey(1), pipeKey(2)]);
    halo.sync([mesh], new Set([pipeKey(1)]), new Set(), pipeKey(2));
    const selection = halo.pairFor("selection", mesh.geometry)!;
    const hover = halo.pairFor("hover", mesh.geometry)!;
    expect(selection.mask.renderOrder).toBe(HALO_MASK_RENDER_ORDER);
    expect(hover.mask.renderOrder).toBe(HALO_MASK_RENDER_ORDER);
    expect(hover.shell.renderOrder).toBe(HALO_HOVER_RENDER_ORDER);
    expect(selection.shell.renderOrder).toBe(HALO_SELECTION_RENDER_ORDER);
    expect(HALO_MASK_RENDER_ORDER).toBeLessThan(HALO_HOVER_RENDER_ORDER);
    expect(HALO_HOVER_RENDER_ORDER).toBeLessThan(HALO_SELECTION_RENDER_ORDER);
    expect(HALO_SELECTION_RENDER_ORDER).toBeLessThan(10_000);
    const mask = selection.mask.material as THREE.ShaderMaterial;
    expect(mask).toBe(hover.mask.material);
    expect(mask.colorWrite).toBe(false);
    expect(mask.depthWrite).toBe(true);
    expect(mask.depthFunc).toBe(THREE.AlwaysDepth);
    for (const pair of [selection, hover]) {
      const shell = pair.shell.material as THREE.ShaderMaterial;
      expect(shell.toneMapped).toBe(false);
      expect(shell.lights).toBe(false);
      expect(shell.depthWrite).toBe(false);
      expect(shell.depthTest).toBe(true);
      expect(shell.transparent).toBe(true);
      // No texture: nothing for the ledger's texture rule to find.
      for (const uniform of Object.values(shell.uniforms)) expect(uniform.value instanceof THREE.Texture).toBe(false);
    }
  });
});

describe("halo resources", () => {
  it("owns nothing while nothing is selected or hovered: the ledger equals a resource's with no halo", () => {
    const withHalo = haloResource();
    const without = haloResource();
    (without.resource as unknown as { halo: null }).halo = null;
    for (const { resource, modelLayer } of [withHalo, without]) {
      resource.replaceLayer(modelLayer, [pipes([pipeKey(1), pipeKey(2)]), nodes([nodeKey(1)])]);
      resource.setSelectionPresentation([]);
      resource.setHoverPresentation(null);
      resource.setThemePresentation("dark");
      resource.setVisibilityPresentation(new Set([pipeKey(1)]));
    }
    expect(withHalo.ownership.snapshot()).toEqual(without.ownership.snapshot());
    expect(withHalo.halo.group.children).toEqual([]);
  });

  it("counts exactly what a halo creates: two instance buffers a pair, one edge geometry a prism, one material a program", () => {
    const { resource, ownership, modelLayer } = haloResource();
    const mesh = pipes([pipeKey(1), pipeKey(2)]);
    const spheres = nodes([nodeKey(1)]);
    resource.replaceLayer(modelLayer, [mesh, spheres]);
    const before = ownership.snapshot().live;

    resource.setSelectionPresentation([pipeKey(1)]);
    let live = ownership.snapshot().live;
    // Mask and edge-shell meshes; the edge geometry; the mask material and the selection edge material.
    expect(live.instanceMatrices - before.instanceMatrices).toBe(2);
    expect(live.geometries - before.geometries).toBe(1);
    expect(live.materials - before.materials).toBe(2);
    expect(live.textures).toBe(before.textures);
    expect(live.instanceColors).toBe(before.instanceColors);
    expect(live.pipeMeshes).toBe(before.pipeMeshes);

    resource.setSelectionPresentation([pipeKey(1), nodeKey(1)]);
    live = ownership.snapshot().live;
    // The sphere's pair shares the sphere's geometry and the mask material; one new shell material.
    expect(live.instanceMatrices - before.instanceMatrices).toBe(4);
    expect(live.geometries - before.geometries).toBe(1);
    expect(live.materials - before.materials).toBe(3);

    resource.setHoverPresentation(pipeKey(2));
    live = ownership.snapshot().live;
    // The hover pair shares the prism's edge geometry and the mask material; one new shell material.
    expect(live.instanceMatrices - before.instanceMatrices).toBe(6);
    expect(live.geometries - before.geometries).toBe(1);
    expect(live.materials - before.materials).toBe(4);
  });

  it("grows capacity geometrically, disposes what it replaces, and keeps created minus disposed equal to live", () => {
    const { resource, halo, ownership, modelLayer } = haloResource();
    const keys = Array.from({ length: 40 }, (_, index) => pipeKey(index));
    const mesh = pipes(keys);
    resource.replaceLayer(modelLayer, [mesh]);
    const base = ownership.snapshot();
    const capacities: number[] = [];
    const replaced: THREE.InstancedMesh[] = [];
    for (const size of [1, 2, 3, 5, 9, 17, 33, 40, 12, 40]) {
      const before = halo.pairFor("selection", mesh.geometry);
      const disposal = before ? vi.spyOn(before.mask, "dispose") : null;
      resource.setSelectionPresentation(keys.slice(0, size));
      const pair = halo.pairFor("selection", mesh.geometry)!;
      expect(pair.count).toBe(size);
      expect(pair.capacity).toBeGreaterThanOrEqual(size);
      if (before && pair.mask !== before.mask) {
        expect(disposal).toHaveBeenCalledTimes(1);
        replaced.push(before.mask);
      } else if (disposal) {
        expect(disposal).not.toHaveBeenCalled();
      }
      if (capacities.at(-1) !== pair.capacity) capacities.push(pair.capacity);
    }
    expect(capacities).toEqual([1, 2, 4, 8, 16, 32, 64]);
    expect(replaced).toHaveLength(6);
    for (const mask of replaced) expect(mask.parent).toBeNull();
    const snapshot = ownership.snapshot();
    // Seven pairs made, six disposed; one edge geometry; two materials.
    expect(snapshot.created.instanceMatrices - base.created.instanceMatrices).toBe(14);
    expect(snapshot.disposed.instanceMatrices - base.disposed.instanceMatrices).toBe(12);
    expect(snapshot.live.instanceMatrices - base.live.instanceMatrices).toBe(2);
    for (const key of Object.keys(snapshot.live) as (keyof typeof snapshot.live)[]) {
      expect(snapshot.created[key] - snapshot.disposed[key]).toBe(snapshot.live[key]);
    }
    expect(halo.group.children).toHaveLength(2);
  });

  it("follows a replaced layer: the old shape's halo objects are disposed, the shared geometry is not, and the new elements are haloed", () => {
    const { resource, halo, ownership, modelLayer } = haloResource();
    const first = pipes([pipeKey(1), pipeKey(2)]);
    resource.replaceLayer(modelLayer, [first]);
    resource.setSelectionPresentation([pipeKey(2)]);
    resource.setHoverPresentation(pipeKey(1));
    const oldPair = halo.pairFor("selection", first.geometry)!;
    const oldEdges = oldPair.shell.geometry;
    const edgesDisposed = vi.spyOn(oldEdges, "dispose");
    const maskDisposed = vi.spyOn(oldPair.mask, "dispose");
    // The layer disposes the figure's geometry; the halo must not.
    const figureGeometryDisposed = vi.spyOn(first.geometry, "dispose");

    const second = pipes([pipeKey(2), pipeKey(3)]);
    second.setMatrixAt(0, new THREE.Matrix4().makeTranslation(7, 8, 9));
    registerInstancedRolePresentation(second, [pipeKey(2), pipeKey(3)], "pipe");
    resource.replaceLayer(modelLayer, [second]);

    expect(figureGeometryDisposed).toHaveBeenCalledTimes(1);
    expect(edgesDisposed).toHaveBeenCalledTimes(1);
    expect(maskDisposed).toHaveBeenCalledTimes(1);
    expect(halo.pairFor("selection", first.geometry)).toBeNull();
    expect(halo.pairFor("hover", first.geometry)).toBeNull();
    const pair = halo.pairFor("selection", second.geometry)!;
    expect(pair.count).toBe(1);
    expect(haloMatrix(pair.mask, 0)).toEqual(new THREE.Matrix4().makeTranslation(7, 8, 9).toArray());
    expect(instanceHex(second, 0)).toBe(viewportRoleHex("light", "pipe"));
    // The hovered key is gone from the model, so there is no hover halo.
    expect(halo.count("hover")).toBe(0);
    for (const child of halo.group.children) expect((child as THREE.Mesh).geometry).not.toBe(first.geometry);

    const snapshot = ownership.snapshot();
    for (const key of Object.keys(snapshot.live) as (keyof typeof snapshot.live)[]) {
      expect(snapshot.created[key] - snapshot.disposed[key]).toBe(snapshot.live[key]);
    }
    // Normal and dimmed figure batches share one geometry and own two materials; one selection pair with the new shape's edge
    // geometry; and the three halo materials made so far (mask, selection edge, hover edge), which
    // live as long as the presentation does.
    expect(snapshot.live).toMatchObject({ pipeMeshes: 2, geometries: 2, materials: 5, instanceMatrices: 4, instanceColors: 2, textures: 0 });
  });

  it("disposes everything it created and nothing it shares, and returns the ledger to the figure's own counts", () => {
    const { resource, halo, ownership, modelLayer } = haloResource();
    const mesh = pipes([pipeKey(1), pipeKey(2)]);
    const spheres = nodes([nodeKey(1)]);
    resource.replaceLayer(modelLayer, [mesh, spheres]);
    const before = ownership.snapshot().live;
    resource.setSelectionPresentation([pipeKey(1), nodeKey(1)]);
    resource.setHoverPresentation(pipeKey(2));
    const shared = [vi.spyOn(mesh.geometry, "dispose"), vi.spyOn(spheres.geometry, "dispose")];
    const owned = (halo.group.children as THREE.InstancedMesh[]).flatMap((object) => [
      vi.spyOn(object, "dispose"),
      vi.spyOn(object.material as THREE.Material, "dispose"),
      ...(object.geometry === mesh.geometry || object.geometry === spheres.geometry ? [] : [vi.spyOn(object.geometry, "dispose")])
    ]);

    halo.dispose();
    halo.dispose();
    for (const dispose of shared) expect(dispose).not.toHaveBeenCalled();
    for (const dispose of owned) expect(dispose).toHaveBeenCalled();
    expect(ownership.snapshot().live).toEqual(before);
    expect(halo.group.children).toEqual([]);
    expect(halo.group.parent).toBeNull();
    // A disposed presentation does nothing more.
    halo.sync([modelLayer], new Set([pipeKey(1)]), new Set(), null);
    expect(halo.count("selection")).toBe(0);
  });

  it("updates 10,000 selected keys in bounded time, allocating per shape and not per key", () => {
    const { resource, halo, modelLayer } = haloResource();
    const chunk = 500;
    const keys = Array.from({ length: 10_000 }, (_, index) => pipeKey(index));
    const geometry = new THREE.CylinderGeometry(1, 1, 1, 10, 1, false);
    const chunks = Array.from({ length: keys.length / chunk }, (_, index) => pipes(keys.slice(index * chunk, (index + 1) * chunk), geometry));
    const nodeKeys = Array.from({ length: 10_001 }, (_, index) => nodeKey(index));
    resource.replaceLayer(modelLayer, [...chunks, nodes(nodeKeys)]);
    const selected = new Set(keys);
    const hidden = new Set<EntityKey>();
    halo.sync([modelLayer], selected, hidden, null);
    const pair = halo.pairFor("selection", geometry)!;
    const matrixArray = pair.mask.instanceMatrix.array;
    const shellArray = pair.shell.instanceMatrix.array;
    const children = halo.group.children.slice();
    const runs: number[] = [];
    for (let run = 0; run < 5; run += 1) {
      const started = performance.now();
      halo.sync([modelLayer], selected, hidden, null);
      runs.push(performance.now() - started);
    }
    runs.sort((a, b) => a - b);
    // An observation, printed for the return; the bound is generous and only catches a blow-up.
    console.info(`halo sync, 10,000 selected of 20,001 elements: median ${runs[2].toFixed(2)} ms, worst ${runs[4].toFixed(2)} ms`);
    expect(runs[2]).toBeLessThan(250);
    expect(pair.count).toBe(10_000);
    expect(pair.capacity).toBe(16_384);
    // The same buffers and the same meshes: a repeat allocates no instance storage.
    expect(halo.pairFor("selection", geometry)).toBe(pair);
    expect(pair.mask.instanceMatrix.array).toBe(matrixArray);
    expect(pair.shell.instanceMatrix.array).toBe(shellArray);
    expect(halo.group.children).toEqual(children);
    expect(halo.group.children).toHaveLength(2);
  });
});

describe("halo and picking", () => {
  it("registers no halo object with either picking path and answers no raycast", () => {
    const { resource, halo, scene, modelLayer } = haloResource();
    const mesh = pipes([pipeKey(1)]);
    mesh.setMatrixAt(0, new THREE.Matrix4());
    registerInstancedRolePresentation(mesh, [pipeKey(1)], "pipe");
    resource.replaceLayer(modelLayer, [mesh]);
    const setPickables = vi.spyOn(resource, "setPickables");
    const setPointPrimitives = vi.spyOn(resource, "setPointPrimitives");
    resource.setSelectionPresentation([pipeKey(1)]);
    resource.setHoverPresentation(pipeKey(1));
    expect(setPickables).not.toHaveBeenCalled();
    expect(setPointPrimitives).not.toHaveBeenCalled();
    const internals = resource as unknown as { pickables: THREE.Object3D[]; pointPrimitives: readonly unknown[] };
    expect(internals.pickables).toEqual([]);
    expect(internals.pointPrimitives).toEqual([]);

    expect(halo.group.children.length).toBeGreaterThan(0);
    scene.updateMatrixWorld(true);
    const raycaster = new THREE.Raycaster(new THREE.Vector3(0, 0, 10), new THREE.Vector3(0, 0, -1));
    // The ray meets the figure; a halo mesh on the same matrix answers nothing.
    expect(raycaster.intersectObject(mesh, true).length).toBeGreaterThan(0);
    expect(raycaster.intersectObject(halo.group, true)).toEqual([]);
    for (const child of halo.group.children) {
      expect(child.userData.entityRef).toBeUndefined();
      expect(child.userData.selectionEntityKey).toBeUndefined();
      expect(child.userData.instanceEntityKeys).toBeUndefined();
    }
  });
});

describe("halo shell geometry", () => {
  it("grows a sphere by its normals and everything else by its edges", () => {
    expect(haloShellFormFor(new THREE.SphereGeometry(0.095, 12, 8))).toBe("normal");
    for (const geometry of [
      new THREE.CylinderGeometry(1, 1, 1, 10, 1, false),
      new THREE.ConeGeometry(0.18, 0.34, 4),
      new THREE.BoxGeometry(0.24, 0.24, 0.24),
      new THREE.TorusGeometry(0.24, 0.027, 8, 18, Math.PI * 0.75)
    ]) expect(haloShellFormFor(geometry)).toBe("edge");
  });

  it.each([
    ["the pipe's prism", () => new THREE.CylinderGeometry(1, 1, 1, 10, 1, false), 30],
    ["the expansion joint's prism", () => new THREE.CylinderGeometry(0.11, 0.11, 0.34, 12), 36],
    ["the support's cone", () => new THREE.ConeGeometry(0.18, 0.34, 4), 8],
    ["the rigid's box", () => new THREE.BoxGeometry(0.24, 0.24, 0.24), 12],
    // 18 sweeps of 8 lengthwise edges, and 19 rings of 8, the two end rings being borders.
    ["the bend's torus arc", () => new THREE.TorusGeometry(0.24, 0.027, 8, 18, Math.PI * 0.75), 18 * 8 + 19 * 8]
  ] as const)("builds %s as one quad per crease or border edge", (_name, build, edges) => {
    const geometry = haloEdgeGeometryFor(build());
    expect(geometry.getAttribute("position").count).toBe(edges * 4);
    expect(geometry.getAttribute("haloOther").count).toBe(edges * 4);
    expect(geometry.getAttribute("haloSide").count).toBe(edges * 4);
    expect(geometry.getIndex()!.count).toBe(edges * 6);
    const position = geometry.getAttribute("position");
    const other = geometry.getAttribute("haloOther");
    const side = geometry.getAttribute("haloSide");
    for (let edge = 0; edge < edges; edge += 1) {
      const at = edge * 4;
      const point = (attribute: THREE.BufferAttribute | THREE.InterleavedBufferAttribute, index: number) =>
        [attribute.getX(index), attribute.getY(index), attribute.getZ(index)];
      // Two vertices at either end, each knowing the other end, one on either side.
      expect(point(position, at)).toEqual(point(position, at + 1));
      expect(point(position, at + 2)).toEqual(point(position, at + 3));
      expect(point(other, at)).toEqual(point(position, at + 2));
      expect(point(other, at + 2)).toEqual(point(position, at));
      expect([side.getX(at), side.getX(at + 1), side.getX(at + 2), side.getX(at + 3)]).toEqual([1, -1, 1, -1]);
      expect([side.getY(at), side.getY(at + 1), side.getY(at + 2), side.getY(at + 3)]).toEqual([1, 1, -1, -1]);
    }
  });
});
