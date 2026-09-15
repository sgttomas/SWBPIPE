import { describe, expect, it, vi } from "vitest";
import * as THREE from "three";
import { entityKey } from "../workspace/selectionState";
import {
  applySelectionPresentation,
  applyThemePresentation,
  currentOwnedViewportResourceSnapshot,
  disposeObjectChildren,
  registerInstancedSelectionPresentation,
  registerSelectionPresentation,
  ViewportInvalidationScheduler,
  ViewportOwnershipLedger,
  ViewportResource,
  ViewportResourceRegistry
} from "./viewportResource";

describe("viewport resource primitives", () => {
  it("coalesces invalidations and returns to zero owned RAF callbacks", () => {
    let callback: FrameRequestCallback | null = null;
    const request = vi.fn((next: FrameRequestCallback) => { callback = next; return 7; });
    const cancel = vi.fn();
    const pending = vi.fn();
    const render = vi.fn(() => false);
    const scheduler = new ViewportInvalidationScheduler(render, request, cancel, pending);
    scheduler.invalidate();
    scheduler.invalidate();
    expect(request).toHaveBeenCalledTimes(1);
    expect(scheduler.pendingCount).toBe(1);
    const scheduled = callback as unknown as FrameRequestCallback;
    scheduled(12);
    expect(render).toHaveBeenCalledWith(12);
    expect(scheduler.pendingCount).toBe(0);
    expect(pending).toHaveBeenLastCalledWith(0);
  });

  it("cancels pending work on context pause and disposal", () => {
    const request = vi.fn(() => 9);
    const cancel = vi.fn();
    const scheduler = new ViewportInvalidationScheduler(() => false, request, cancel);
    scheduler.invalidate();
    scheduler.pause();
    expect(cancel).toHaveBeenCalledWith(9);
    expect(scheduler.pendingCount).toBe(0);
    scheduler.resume();
    scheduler.dispose();
    expect(scheduler.pendingCount).toBe(0);
  });

  it("tracks and cancels the single latest paint opportunity with render RAFs", () => {
    let nextHandle = 0;
    const callbacks = new Map<number, FrameRequestCallback>();
    const request = vi.fn((callback: FrameRequestCallback) => {
      const handle = ++nextHandle;
      callbacks.set(handle, callback);
      return handle;
    });
    const cancel = vi.fn((handle: number) => callbacks.delete(handle));
    const scheduler = new ViewportInvalidationScheduler(() => false, request, cancel);
    scheduler.invalidate();
    scheduler.schedulePaintOpportunity(vi.fn());
    const firstOpportunityHandle = nextHandle;
    scheduler.schedulePaintOpportunity(vi.fn());
    expect(cancel).toHaveBeenCalledWith(firstOpportunityHandle);
    expect(scheduler.pendingCount).toBe(2);
    scheduler.pause();
    expect(scheduler.pendingCount).toBe(0);
    expect(callbacks.size).toBe(0);
  });

  it("disposes each registered GPU-style resource exactly once", () => {
    const registry = new ViewportResourceRegistry();
    const first = { dispose: vi.fn() };
    const second = { dispose: vi.fn() };
    registry.own(first);
    registry.own(second);
    registry.dispose();
    registry.dispose();
    expect(first.dispose).toHaveBeenCalledTimes(1);
    expect(second.dispose).toHaveBeenCalledTimes(1);
  });

  it("updates selection and theme presentation without replacing stable resource objects", () => {
    const scene = new THREE.Scene();
    const layer = new THREE.Group();
    const geometry = new THREE.BoxGeometry(1, 1, 1);
    const material = new THREE.MeshStandardMaterial({ color: 0x24705a });
    const mesh = new THREE.Mesh(geometry, material);
    const key = entityKey({ type: "component", id: "component:stable" });
    registerSelectionPresentation(mesh, key);
    layer.add(mesh);
    scene.add(layer);
    const resourceIdentity = { scene, layer, mesh, geometry, material };

    applySelectionPresentation([layer], new Set([key]));
    applyThemePresentation(scene, "dark");
    expect(material.color.getHex()).toBe(0xf08c22);
    expect(resourceIdentity).toEqual({ scene, layer, mesh, geometry, material });
    expect(layer.children[0]).toBe(mesh);
    expect(mesh.geometry).toBe(geometry);

    applySelectionPresentation([layer], new Set());
    applyThemePresentation(scene, "light");
    expect(material.color.getHex()).toBe(0x24705a);
    expect(layer.children[0]).toBe(mesh);
    expect(mesh.geometry).toBe(geometry);
  });

  it("updates instanced selection colors without replacing matrices or geometry", () => {
    const geometry = new THREE.SphereGeometry(1, 8, 6);
    const material = new THREE.MeshStandardMaterial({ color: 0xffffff });
    const mesh = new THREE.InstancedMesh(geometry, material, 2);
    const first = entityKey({ type: "node", id: "n:1" });
    const second = entityKey({ type: "node", id: "n:2" });
    registerInstancedSelectionPresentation(mesh, [first, second], 0x2f6f73);
    const matrix = mesh.instanceMatrix;

    applySelectionPresentation([mesh], new Set([second]));
    expect(mesh.geometry).toBe(geometry);
    expect(mesh.instanceMatrix).toBe(matrix);
    expect(mesh.instanceColor).not.toBeNull();
    expect(new THREE.Color().fromBufferAttribute(mesh.instanceColor!, 0).getHex()).toBe(0x2f6f73);
    expect(new THREE.Color().fromBufferAttribute(mesh.instanceColor!, 1).getHex()).toBe(0xf08c22);
  });

  it("retires each instanced mesh and its owned assets exactly once", () => {
    const root = new THREE.Group();
    const geometry = new THREE.BoxGeometry(1, 1, 1);
    const material = new THREE.MeshStandardMaterial();
    const mesh = new THREE.InstancedMesh(geometry, material, 2);
    const instanceDispose = vi.spyOn(mesh, "dispose");
    const geometryDispose = vi.spyOn(geometry, "dispose");
    const materialDispose = vi.spyOn(material, "dispose");
    root.add(mesh);

    disposeObjectChildren(root);
    expect(instanceDispose).toHaveBeenCalledTimes(1);
    expect(geometryDispose).toHaveBeenCalledTimes(1);
    expect(materialDispose).toHaveBeenCalledTimes(1);
  });

  it("uses the real layer replacement path to retire instance buffers and assets", () => {
    const layer = new THREE.Group();
    const geometry = new THREE.BoxGeometry(1, 1, 1);
    const material = new THREE.MeshStandardMaterial();
    const retired = new THREE.InstancedMesh(geometry, material, 2);
    registerInstancedSelectionPresentation(
      retired,
      [entityKey({ type: "pipe", id: "p:1" }), entityKey({ type: "pipe", id: "p:2" })],
      0x4f6f73
    );
    layer.add(retired);
    const next = new THREE.Mesh(new THREE.BoxGeometry(1, 1, 1), new THREE.MeshStandardMaterial());
    const instanceDispose = vi.spyOn(retired, "dispose");
    const geometryDispose = vi.spyOn(geometry, "dispose");
    const materialDispose = vi.spyOn(material, "dispose");
    const ownership = new ViewportOwnershipLedger(8);
    ownership.createObjects([retired]);
    const resource = Object.assign(Object.create(ViewportResource.prototype), {
      ownership,
      resourceGeneration: 8,
      contextLostCount: 0,
      contextRestoredCount: 0,
      scheduler: { pendingCount: 0 },
      renderer: { domElement: { isConnected: true } },
      options: {},
      invalidate: vi.fn()
    }) as ViewportResource;

    resource.replaceLayer(layer, [next]);
    expect(layer.children).toEqual([next]);
    expect(instanceDispose).toHaveBeenCalledTimes(1);
    expect(geometryDispose).toHaveBeenCalledTimes(1);
    expect(materialDispose).toHaveBeenCalledTimes(1);
    expect(ownership.snapshot().live.instanceMatrices).toBe(0);
    expect(ownership.snapshot().live.instanceColors).toBe(0);
  });

  it("uses the real owner teardown path for controls, listeners, observer, and RAFs", () => {
    const scheduler = { dispose: vi.fn(), pendingCount: 0 };
    const resizeObserver = { disconnect: vi.fn() };
    const controls = {
      removeEventListener: vi.fn(),
      dispose: vi.fn()
    };
    const canvas = { removeEventListener: vi.fn(), isConnected: false };
    const renderer = {
      domElement: canvas,
      renderLists: { dispose: vi.fn() },
      dispose: vi.fn(),
      forceContextLoss: vi.fn()
    };
    const host = { replaceChildren: vi.fn() };
    const ownership = new ViewportOwnershipLedger(9);
    ownership.createLifecycle({ controls: 1, eventBindings: 7, resizeObservers: 1 });
    const removeWindowListener = vi.spyOn(window, "removeEventListener");
    const removeDocumentListener = vi.spyOn(document, "removeEventListener");
    const resource = Object.assign(Object.create(ViewportResource.prototype), {
      disposed: false,
      resourceGeneration: 9,
      contextLostCount: 0,
      contextRestoredCount: 0,
      scheduler,
      resizeObserver,
      controls,
      renderer,
      host,
      ownership,
      scene: new THREE.Scene(),
      gizmoScene: new THREE.Scene(),
      options: {},
      resize: vi.fn(),
      handleVisibilityChange: vi.fn(),
      handleControlsChange: vi.fn(),
      handleContextLost: vi.fn(),
      handleContextRestored: vi.fn(),
      invalidate: vi.fn(),
      pickables: [],
      pointPrimitives: [],
      modelIndex: null,
      labelUpdater: null
    }) as ViewportResource;

    resource.dispose();
    resource.dispose();
    expect(scheduler.dispose).toHaveBeenCalledTimes(1);
    expect(resizeObserver.disconnect).toHaveBeenCalledTimes(1);
    expect(controls.removeEventListener).toHaveBeenCalledTimes(3);
    expect(controls.dispose).toHaveBeenCalledTimes(1);
    expect(canvas.removeEventListener).toHaveBeenCalledTimes(2);
    expect(removeWindowListener).toHaveBeenCalledWith("resize", expect.any(Function));
    expect(removeDocumentListener).toHaveBeenCalledWith("visibilitychange", expect.any(Function));
    expect(host.replaceChildren).toHaveBeenCalledTimes(1);
    expect(ownership.snapshot().live.controls).toBe(0);
    expect(ownership.snapshot().live.eventBindings).toBe(0);
    expect(ownership.snapshot().live.resizeObservers).toBe(0);
    expect(currentOwnedViewportResourceSnapshot()).toMatchObject({
      ownedPendingRafCount: 0,
      owned: { generation: 9, live: zeroOwnedCounts() },
      context: { canvasConnected: false }
    });
  });

  it("keeps camera sequences monotonic across resource identities", () => {
    const first = Object.assign(Object.create(ViewportResource.prototype), {
      cameraChangeSequence: 0,
      options: { onCameraChange: vi.fn() }
    }) as ViewportResource;
    const second = Object.assign(Object.create(ViewportResource.prototype), {
      cameraChangeSequence: 0,
      options: { onCameraChange: vi.fn() }
    }) as ViewportResource;
    first.markCameraProjectionChanged();
    const firstSequence = first.cameraSequence;
    second.markCameraProjectionChanged();
    expect(second.cameraSequence).toBeGreaterThan(firstSequence);
  });

  it("keeps exact created minus disposed ownership balances", () => {
    const ledger = new ViewportOwnershipLedger(7);
    const geometry = new THREE.BoxGeometry(1, 1, 1);
    const material = new THREE.MeshStandardMaterial();
    const mesh = new THREE.InstancedMesh(geometry, material, 1);
    registerInstancedSelectionPresentation(mesh, [entityKey({ type: "pipe", id: "p" })], 0x4f6f73);
    ledger.createLifecycle({ controls: 1, eventBindings: 7, resizeObservers: 1 });
    ledger.createObjects([mesh]);
    expect(ledger.snapshot().live).toMatchObject({
      pipeMeshes: 1,
      geometries: 1,
      materials: 1,
      instanceMatrices: 1,
      instanceColors: 1,
      controls: 1,
      eventBindings: 7,
      resizeObservers: 1
    });
    ledger.disposeObjects([mesh]);
    ledger.disposeLifecycle({ controls: 1, eventBindings: 7, resizeObservers: 1 });
    expect(ledger.snapshot().live).toEqual({
      pipeMeshes: 0,
      nodeMeshes: 0,
      supportMeshes: 0,
      componentMeshes: 0,
      geometries: 0,
      materials: 0,
      textures: 0,
      instanceMatrices: 0,
      instanceColors: 0,
      controls: 0,
      eventBindings: 0,
      resizeObservers: 0
    });
    expect(ledger.snapshot().created).toEqual(ledger.snapshot().disposed);
  });
});

function zeroOwnedCounts() {
  return {
    pipeMeshes: 0,
    nodeMeshes: 0,
    supportMeshes: 0,
    componentMeshes: 0,
    geometries: 0,
    materials: 0,
    textures: 0,
    instanceMatrices: 0,
    instanceColors: 0,
    controls: 0,
    eventBindings: 0,
    resizeObservers: 0
  };
}
