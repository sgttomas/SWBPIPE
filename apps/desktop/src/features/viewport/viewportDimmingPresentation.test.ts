import { describe, expect, it, vi } from "vitest";
import * as THREE from "three";
import { entityKey, type EntityKey } from "../workspace/selectionState";
import { prepareViewportDimming } from "./viewportDimmingPresentation";
import {
  applyPalettePresentation, applySelectionPresentation, applyVisibilityPresentation,
  disposeObjectChildren, registerInstancedRolePresentation, registerSelectionPresentation,
  ViewportOwnershipLedger, ViewportResource
} from "./viewportResource";
import { createFigureMaterial, figureEdgeOutlineFor, isFigureMaterial } from "./viewportFigureMaterial";
import { ViewportHaloPresentation, HALO_MASK_RENDER_ORDER } from "./viewportHalo";
import { viewportRoleHex } from "./viewportPalette";

const a = entityKey({ type: "pipe", id: "a" });
const b = entityKey({ type: "pipe", id: "b" });
const c = entityKey({ type: "pipe", id: "c" });
const empty = new Set<EntityKey>();
function mesh(keys = [a, b, c], opacity = 1) {
  const geometry = new THREE.CylinderGeometry(1, 1, 1, 10);
  const source = new THREE.InstancedMesh(geometry, createFigureMaterial({
    edge: figureEdgeOutlineFor(geometry), opacity, transparent: opacity < 1
  }), keys.length);
  keys.forEach((_, index) => source.setMatrixAt(index, new THREE.Matrix4().makeTranslation(index * 100, 0, 0)));
  registerInstancedRolePresentation(source, keys, "pipe");
  return source;
}
const companion = (source: THREE.InstancedMesh) => source.children[0] as THREE.InstancedMesh;
const material = (source: THREE.Mesh) => source.material as THREE.Material;
const matrix = (source: THREE.InstancedMesh, index: number) => new THREE.Matrix4().fromArray(source.instanceMatrix.array, index * 16);

function resourceFixture() {
  const layers = { modelLayer: new THREE.Group(), authoredLoadLayer: new THREE.Group(), resultLayer: new THREE.Group(), diagnosticLayer: new THREE.Group(), routingLayer: new THREE.Group() };
  const ownership = new ViewportOwnershipLedger(700);
  const halo = new ViewportHaloPresentation({
    created: (counts) => ownership.createLifecycle(counts), disposed: (counts) => ownership.disposeLifecycle(counts)
  });
  const scene = new THREE.Scene();
  scene.add(...Object.values(layers), halo.group);
  const resource = Object.assign(Object.create(ViewportResource.prototype), {
    ...layers, scene, gizmoScene: new THREE.Scene(), ownership, halo, resourceGeneration: 700,
    contextLostCount: 0, contextRestoredCount: 0, contextStatus: "ready", scheduler: { pendingCount: 0 },
    renderer: { domElement: { isConnected: true } }, options: {}, invalidate: vi.fn(),
    selectedKeys: empty, hiddenKeys: empty, dimmedKeys: empty, hoveredKey: null,
    selectionPresentation: null, themePresentation: "light", gridVisible: true, authoredLoadsVisible: true
  }) as ViewportResource;
  return { resource, ownership, halo, ...layers };
}

describe("persistent viewport dimming", () => {
  it("partitions ownership once, keeps full bounds and restores exact source matrices", () => {
    const source = mesh();
    const original = [0, 1, 2].map((i) => matrix(source, i));
    const bounds = source.boundingBox!.clone();
    source.position.set(7, 8, 9);
    prepareViewportDimming([source]);
    const dim = companion(source);
    prepareViewportDimming([source]);
    expect(source.children).toEqual([dim]);
    expect(dim.position.toArray()).toEqual([0, 0, 0]);
    expect(dim.geometry).toBe(source.geometry);
    expect(dim.boundingBox).toEqual(bounds);
    applyVisibilityPresentation([source], new Set([c]), new Set([a, c]));
    expect(source.userData.instanceEntityKeys).toEqual([b, c]);
    expect(dim.userData.instanceEntityKeys).toEqual([a]);
    expect(matrix(source, 0)).toEqual(original[1]);
    expect(matrix(source, 1)).toEqual(new THREE.Matrix4().makeScale(0, 0, 0));
    expect(matrix(dim, 0)).toEqual(original[0]);
    expect(material(dim).opacity).toBe(0.2);
    expect(material(dim).depthWrite).toBe(false);
    expect(material(dim).depthTest).toBe(material(source).depthTest);
    expect(dim.renderOrder).toBeLessThan(HALO_MASK_RENDER_ORDER);
    expect(source.boundingBox).toEqual(bounds);
    applyVisibilityPresentation([source], empty, new Set([a, b, c]));
    expect(source.count).toBe(0);
    expect(source.visible).toBe(true);
    expect(dim.count).toBe(3);
    applyVisibilityPresentation([source], empty, empty);
    expect(dim.count).toBe(0);
    expect(source.userData.instanceEntityKeys).toEqual([a, b, c]);
    expect([0, 1, 2].map((i) => matrix(source, i))).toEqual(original);
    disposeObjectChildren(source);
  });

  it("uses literal 0.2 for opaque and translucent owned graphics and preserves normal material flags", () => {
    for (const opacity of [1, 0.82, 0.86]) {
      const source = mesh([a], opacity);
      const normal = material(source);
      normal.depthTest = false;
      const flags = [normal.opacity, normal.transparent, normal.depthWrite, normal.depthTest];
      prepareViewportDimming([source]);
      const dim = material(companion(source));
      applyVisibilityPresentation([source], empty, new Set([a]));
      expect([dim.opacity, dim.transparent, dim.depthWrite, dim.depthTest]).toEqual([0.2, true, false, false]);
      expect(isFigureMaterial(dim) && dim.uniforms.opacity.value).toBe(0.2);
      applyVisibilityPresentation([source], empty, empty);
      expect(material(source)).toBe(normal);
      expect([normal.opacity, normal.transparent, normal.depthWrite, normal.depthTest]).toEqual(flags);
      disposeObjectChildren(source);
    }
  });

  it("repaints both batches without restoring alpha or corrupting newly moved instance colours", () => {
    const source = mesh();
    prepareViewportDimming([source]);
    const dim = companion(source);
    applyVisibilityPresentation([source], empty, new Set([a, b]));
    for (const theme of ["dark", "light"] as const) {
      applyPalettePresentation([source], theme, 2);
      applySelectionPresentation([source], new Set([a]), theme);
      applyVisibilityPresentation([source], empty, new Set([c]));
      for (const batch of [source, dim]) {
        for (let i = 0; i < batch.count; i++) {
          expect(new THREE.Color().fromBufferAttribute(batch.instanceColor!, i).getHex()).toBe(viewportRoleHex(theme, "pipe"));
        }
      }
      expect(material(dim).opacity).toBe(0.2);
      expect(isFigureMaterial(material(dim)) && (material(dim) as ReturnType<typeof createFigureMaterial>).uniforms.edgeWidth.value).toBe(2);
    }
    disposeObjectChildren(source);
  });

  it("preserves distinct per-instance colours through compaction, Hide and Show All", () => {
    const source = mesh();
    const colors = [0xff0000, 0x00ff00, 0x0000ff];
    colors.forEach((hex, index) => source.setColorAt(index, new THREE.Color(hex)));
    prepareViewportDimming([source]);
    const dim = companion(source);
    const expected = new Map([[a, colors[0]], [b, colors[1]], [c, colors[2]]]);
    for (const [hidden, dimmed] of [
      [empty, new Set([a, c])], [new Set([b]), new Set([b, c])],
      [empty, new Set([a, b, c])], [empty, empty]
    ]) {
      applyVisibilityPresentation([source], hidden, dimmed);
      for (const batch of [source, dim]) {
        (batch.userData.instanceEntityKeys as EntityKey[]).forEach((key, index) => {
          expect(new THREE.Color().fromBufferAttribute(batch.instanceColor!, index).getHex()).toBe(expected.get(key));
        });
      }
    }
    disposeObjectChildren(source);
  });

  it("separates shared noninstanced material owners without losing originals on disposal", () => {
    const shared = new THREE.MeshBasicMaterial({ opacity: 0.86, transparent: true, depthTest: false });
    const group = new THREE.Group();
    const first = new THREE.Mesh(new THREE.BoxGeometry(), shared);
    const second = new THREE.Mesh(first.geometry, shared);
    const unowned = new THREE.Mesh(first.geometry, shared);
    registerSelectionPresentation(first, a);
    registerSelectionPresentation(second, b);
    group.add(first, second, unowned);
    prepareViewportDimming([group]);
    const originalDispose = vi.spyOn(shared, "dispose");
    const firstDispose = vi.spyOn(material(first), "dispose");
    const secondDispose = vi.spyOn(material(second), "dispose");
    const ownership = new ViewportOwnershipLedger(701);
    ownership.createObjects([group]);
    applyVisibilityPresentation([group], empty, new Set([a]));
    expect(material(first).opacity).toBe(0.2);
    expect(material(second).opacity).toBe(0.86);
    expect(shared.opacity).toBe(0.86);
    applyVisibilityPresentation([group], new Set([a]), new Set([a]));
    expect(first.visible).toBe(false);
    applyVisibilityPresentation([group], empty, empty);
    expect(first.visible).toBe(true);
    expect([material(first).opacity, material(first).transparent, material(first).depthWrite, material(first).depthTest]).toEqual([0.86, true, true, false]);
    ownership.disposeObjects([group]);
    disposeObjectChildren(group);
    for (const spy of [originalDispose, firstDispose, secondDispose]) expect(spy).toHaveBeenCalledTimes(1);
    expect(Object.values(ownership.snapshot().live).every((count) => count === 0)).toBe(true);
  });

  it("keeps unique full-strength halos, stable ledger through toggles, and returns resources on replacement", () => {
    const { resource, ownership, halo, modelLayer, resultLayer, authoredLoadLayer } = resourceFixture();
    const pipes = mesh();
    resource.replaceLayer(modelLayer, [pipes]);
    resource.replaceLayer(resultLayer, [mesh([a, b], 0.82)]);
    const arrow = new THREE.InstancedMesh(new THREE.ConeGeometry(), new THREE.MeshBasicMaterial(), 2);
    arrow.setMatrixAt(0, new THREE.Matrix4()); arrow.setMatrixAt(1, new THREE.Matrix4());
    registerInstancedRolePresentation(arrow, [a, b], "loadForce");
    resource.replaceLayer(authoredLoadLayer, [arrow]);
    resource.setSelectionPresentation([a, b]);
    resource.setHoverPresentation(c);
    const stable = ownership.snapshot();
    for (let iteration = 0; iteration < 5; iteration++) {
      resource.setVisibilityPresentation(empty, new Set([a, c]));
      const halos = halo.group.children.filter((child) => child.name === "viewport-halo-selection-shell") as THREE.InstancedMesh[];
      expect(halos.reduce((count, child) => count + child.count, 0)).toBe(2);
      expect(halos.every((child) => material(child).opacity === 1)).toBe(true);
      const hover = halo.group.children.filter((child) => child.name === "viewport-halo-hover-shell") as THREE.InstancedMesh[];
      expect(hover.reduce((count, child) => count + child.count, 0)).toBe(1);
      expect(material(companion(arrow)).opacity).toBe(0.2);
      resource.setThemePresentation(iteration % 2 ? "light" : "dark");
      resource.setVisibilityPresentation(new Set([a]), new Set([a, b, c]));
      expect(halos.reduce((count, child) => count + child.count, 0)).toBe(1);
      resource.setVisibilityPresentation(empty, empty);
      expect(ownership.snapshot()).toEqual(stable);
    }
    // An empty Historical result layer must stay absent under further isolation/theme changes.
    resource.replaceLayer(resultLayer, []);
    resource.setVisibilityPresentation(empty, new Set([b]));
    const next = mesh([a, b]);
    resource.replaceLayer(modelLayer, [next]);
    expect(companion(next).userData.instanceEntityKeys).toEqual([b]);
    resource.setThemePresentation("dark");
    expect(resultLayer.children).toHaveLength(0);
    resource.replaceLayer(modelLayer, []);
    resource.replaceLayer(authoredLoadLayer, []);
    halo.dispose();
    expect(Object.values(ownership.snapshot().live).every((count) => count === 0)).toBe(true);
  });
});
