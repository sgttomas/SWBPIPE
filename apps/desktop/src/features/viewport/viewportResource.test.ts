import { describe, expect, it, vi } from "vitest";
import * as THREE from "three";
import { entityKey } from "../workspace/selectionState";
import {
  applyPalettePresentation,
  applySelectionPresentation,
  applyVisibilityPresentation,
  applyThemePresentation,
  applyGizmoThemePresentation,
  GIZMO_AXIS_LABEL_FONT_PX,
  GIZMO_AXIS_LABEL_WORLD_SIZE,
  GIZMO_CAMERA_DISTANCE,
  GIZMO_MAX_CSS_SIZE,
  GIZMO_THEME_PALETTES,
  relativeContrastRatio,
  currentOwnedViewportResourceSnapshot,
  disposeObjectChildren,
  registerGridPaletteRoles,
  registerInstancedRolePresentation,
  registerInstancedSelectionPresentation,
  registerPaletteRole,
  registerSelectionPresentation,
  ViewportInvalidationScheduler,
  ViewportOwnershipLedger,
  ViewportResource,
  ViewportResourceRegistry
} from "./viewportResource";
import { createFigureMaterial } from "./viewportFigureMaterial";
import { viewportRoleHex, viewportShadeRatio } from "./viewportPalette";

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

  it("publishes bounded post-render diagnostics without scheduling an observation RAF", () => {
    const schedulePaintOpportunity = vi.fn();
    const nextPaint = vi.fn();
    const afterMainFrame = vi.fn();
    const now = vi.spyOn(performance, "now").mockReturnValueOnce(40);
    const fake = Object.assign(Object.create(ViewportResource.prototype), {
      disposed: false,
      contextStatus: "ready",
      controls: { update: vi.fn(() => false) },
      renderer: {
        setScissorTest: vi.fn(), setViewport: vi.fn(), render: vi.fn(), info: { memory: {}, render: {} }
      },
      host: { clientWidth: 100, clientHeight: 100 },
      scene: new THREE.Scene(), camera: new THREE.PerspectiveCamera(),
      frameSubmissionSequence: 0,
      scheduler: { schedulePaintOpportunity },
      options: {
        onNextPaintOpportunity: nextPaint,
        onAfterMainFrame: afterMainFrame
      },
      renderGizmo: vi.fn(), labelUpdater: null
    }) as ViewportResource;
    (fake as unknown as { render(time: number): boolean }).render(7);
    expect(fake.controls.update).not.toHaveBeenCalled();
    expect(schedulePaintOpportunity).not.toHaveBeenCalled();
    expect(nextPaint).not.toHaveBeenCalled();
    expect(afterMainFrame).toHaveBeenCalledWith(fake, 40, 1, expect.any(Object));
    expect(now).toHaveBeenCalledTimes(1);
    now.mockRestore();
  });

  it("advances controls only for active navigation and finitely retires a settled damping tail", () => {
    const camera = new THREE.PerspectiveCamera();
    const target = new THREE.Vector3();
    const controls = {
      target,
      update: vi.fn()
        .mockImplementationOnce(() => { camera.position.x += 1; return true; })
        .mockImplementationOnce(() => { camera.position.x += 0.000001; return false; })
    };
    let fake!: ViewportResource;
    const cancelNavigation = vi.fn(() => {
      (fake as unknown as { navigationAdvancing: boolean }).navigationAdvancing = false;
    });
    const markCameraProjectionChanged = vi.fn();
    fake = Object.assign(Object.create(ViewportResource.prototype), {
      disposed: false,
      contextStatus: "ready",
      navigationAdvancing: true,
      suppressControlsChange: false,
      controls,
      camera,
      renderer: {
        setScissorTest: vi.fn(), setViewport: vi.fn(), render: vi.fn(), info: { memory: {}, render: {} }
      },
      host: { clientWidth: 100, clientHeight: 100 },
      scene: new THREE.Scene(),
      frameSubmissionSequence: 0,
      options: {},
      cancelNavigation,
      markCameraProjectionChanged,
      renderGizmo: vi.fn(), labelUpdater: null
    }) as ViewportResource;
    const render = (fake as unknown as { render(time: number): boolean }).render.bind(fake);

    expect(render(1)).toBe(true);
    expect(cancelNavigation).not.toHaveBeenCalled();
    expect(render(2)).toBe(false);
    expect(controls.update).toHaveBeenCalledTimes(2);
    expect(markCameraProjectionChanged).toHaveBeenCalledTimes(2);
    expect(cancelNavigation).toHaveBeenCalledTimes(1);
  });

  it("clears retained controls movement while preserving the exact public camera pose and input mapping", () => {
    const camera = new THREE.PerspectiveCamera();
    camera.position.set(4, 3, 7);
    camera.lookAt(1, 2, 3);
    camera.zoom = 1.25;
    const position = camera.position.clone();
    const quaternion = camera.quaternion.clone();
    const target = new THREE.Vector3(1, 2, 3);
    const originalTarget = target.clone();
    const mouseButtons = { LEFT: THREE.MOUSE.ROTATE, MIDDLE: THREE.MOUSE.DOLLY, RIGHT: THREE.MOUSE.PAN };
    const updateStates: Array<{ damping: boolean; autoRotate: boolean }> = [];
    const controls = {
      autoRotate: true,
      enableDamping: true,
      mouseButtons,
      target,
      update: vi.fn(() => {
        updateStates.push({ damping: controls.enableDamping, autoRotate: controls.autoRotate });
        camera.position.addScalar(5);
        target.addScalar(2);
        return true;
      })
    };
    const fake = Object.assign(Object.create(ViewportResource.prototype), {
      disposed: false,
      navigationAdvancing: true,
      suppressControlsChange: false,
      camera,
      controls
    }) as ViewportResource;

    fake.cancelNavigation();

    expect(updateStates).toEqual([{ damping: false, autoRotate: false }]);
    expect(camera.position.equals(position)).toBe(true);
    expect(camera.quaternion.equals(quaternion)).toBe(true);
    expect(camera.zoom).toBe(1.25);
    expect(target.equals(originalTarget)).toBe(true);
    expect(controls.enableDamping).toBe(true);
    expect(controls.autoRotate).toBe(true);
    expect(controls.mouseButtons).toEqual({
      LEFT: THREE.MOUSE.ROTATE,
      MIDDLE: THREE.MOUSE.DOLLY,
      RIGHT: THREE.MOUSE.PAN
    });
    expect((fake as unknown as { navigationAdvancing: boolean }).navigationAdvancing).toBe(false);
  });

  it("exposes projection context only while the owned ready canvas is connected", () => {
    const canvas = { isConnected: true };
    const fake = Object.assign(Object.create(ViewportResource.prototype), {
      disposed: false,
      contextStatus: "ready",
      renderer: { domElement: canvas }
    }) as ViewportResource;
    expect(fake.projectionAvailable).toBe(true);
    (fake as unknown as { contextStatus: string }).contextStatus = "lost";
    expect(fake.projectionAvailable).toBe(false);
    (fake as unknown as { contextStatus: string }).contextStatus = "ready";
    canvas.isConnected = false;
    expect(fake.projectionAvailable).toBe(false);
    (fake as unknown as { disposed: boolean }).disposed = true;
    canvas.isConnected = true;
    expect(fake.projectionAvailable).toBe(false);
  });

  it("restores renderer viewport, scissor and auto-clear after the owned gizmo pass", () => {
    const viewport = new THREE.Vector4(1, 2, 300, 200);
    const scissor = new THREE.Vector4(3, 4, 280, 180);
    const renderer = {
      autoClear: true,
      getViewport: vi.fn((target: THREE.Vector4) => target.copy(viewport)),
      getScissor: vi.fn((target: THREE.Vector4) => target.copy(scissor)),
      getScissorTest: vi.fn(() => false),
      setScissorTest: vi.fn(), setScissor: vi.fn(), setViewport: vi.fn(), clearDepth: vi.fn(), render: vi.fn()
    };
    const fake = Object.assign(Object.create(ViewportResource.prototype), {
      host: { clientWidth: 300, clientHeight: 200 }, renderer,
      camera: new THREE.PerspectiveCamera(), controls: { target: new THREE.Vector3() },
      gizmoCamera: new THREE.PerspectiveCamera(), gizmoScene: new THREE.Scene()
    }) as ViewportResource;
    fake.camera.position.set(3, 2, 4);
    (fake as unknown as { renderGizmo(): void }).renderGizmo();
    expect(renderer.autoClear).toBe(true);
    expect(renderer.setScissor).toHaveBeenLastCalledWith(scissor);
    expect(renderer.setViewport).toHaveBeenLastCalledWith(viewport);
    expect(renderer.setScissorTest).toHaveBeenLastCalledWith(false);
  });

  it("uses the full gizmo size whenever it fits and shrinks only for a smaller host", () => {
    const renderer = {
      autoClear: true,
      getViewport: vi.fn((target: THREE.Vector4) => target.set(0, 0, 300, 200)),
      getScissor: vi.fn((target: THREE.Vector4) => target.set(0, 0, 300, 200)),
      getScissorTest: vi.fn(() => false),
      setScissorTest: vi.fn(), setScissor: vi.fn(), setViewport: vi.fn(), clearDepth: vi.fn(), render: vi.fn()
    };
    const host = { clientWidth: 300, clientHeight: 200 };
    const fake = Object.assign(Object.create(ViewportResource.prototype), {
      host, renderer,
      camera: new THREE.PerspectiveCamera(), controls: { target: new THREE.Vector3() },
      gizmoCamera: new THREE.PerspectiveCamera(45, 1, 0.1, 100), gizmoScene: new THREE.Scene()
    }) as ViewportResource;
    fake.camera.position.set(3, 2, 4);

    (fake as unknown as { renderGizmo(): void }).renderGizmo();
    expect(renderer.setScissor).toHaveBeenNthCalledWith(1, 8, 8, GIZMO_MAX_CSS_SIZE, GIZMO_MAX_CSS_SIZE);
    expect(renderer.setViewport).toHaveBeenNthCalledWith(1, 8, 8, GIZMO_MAX_CSS_SIZE, GIZMO_MAX_CSS_SIZE);

    host.clientWidth = 72;
    host.clientHeight = 60;
    renderer.setScissor.mockClear();
    renderer.setViewport.mockClear();
    (fake as unknown as { renderGizmo(): void }).renderGizmo();
    expect(renderer.setScissor).toHaveBeenNthCalledWith(1, 8, 0, 60, 60);
    expect(renderer.setViewport).toHaveBeenNthCalledWith(1, 8, 0, 60, 60);
  });

  it("keeps complete XYZ badge bounds inside the gizmo scissor across orbit orientations", () => {
    const renderer = {
      autoClear: true,
      getViewport: vi.fn((target: THREE.Vector4) => target.set(0, 0, 300, 200)),
      getScissor: vi.fn((target: THREE.Vector4) => target.set(0, 0, 300, 200)),
      getScissorTest: vi.fn(() => false),
      setScissorTest: vi.fn(), setScissor: vi.fn(), setViewport: vi.fn(), clearDepth: vi.fn(), render: vi.fn()
    };
    const gizmoScene = new THREE.Scene();
    const labels = [
      new THREE.Vector3(1.48, 0, 0),
      new THREE.Vector3(0, 1.48, 0),
      new THREE.Vector3(0, 0, 1.48)
    ].map((position) => {
      const sprite = new THREE.Sprite();
      sprite.position.copy(position);
      sprite.scale.setScalar(GIZMO_AXIS_LABEL_WORLD_SIZE);
      gizmoScene.add(sprite);
      return sprite;
    });
    const fake = Object.assign(Object.create(ViewportResource.prototype), {
      host: { clientWidth: 300, clientHeight: 200 }, renderer,
      camera: new THREE.PerspectiveCamera(), controls: { target: new THREE.Vector3() },
      gizmoCamera: new THREE.PerspectiveCamera(45, 1, 0.1, 100), gizmoScene
    }) as ViewportResource;
    const orientations: THREE.Vector3[] = [];
    for (const elevation of [-1.2, -0.6, 0, 0.6, 1.2]) {
      for (let step = 0; step < 8; step += 1) {
        const yaw = step * Math.PI / 4;
        orientations.push(new THREE.Vector3(
          Math.cos(elevation) * Math.cos(yaw),
          Math.sin(elevation),
          Math.cos(elevation) * Math.sin(yaw)
        ));
      }
    }
    for (const direction of orientations) {
      fake.camera.position.copy(direction.multiplyScalar(10));
      (fake as unknown as { renderGizmo(): void }).renderGizmo();
      const gizmoCamera = (fake as unknown as { gizmoCamera: THREE.PerspectiveCamera }).gizmoCamera;
      gizmoCamera.updateMatrixWorld(true);
      expect(gizmoCamera.position.length()).toBeCloseTo(GIZMO_CAMERA_DISTANCE);
      const right = new THREE.Vector3().setFromMatrixColumn(gizmoCamera.matrixWorld, 0);
      const up = new THREE.Vector3().setFromMatrixColumn(gizmoCamera.matrixWorld, 1);
      for (const label of labels) {
        const half = label.scale.x / 2;
        for (const horizontal of [-half, half]) {
          for (const vertical of [-half, half]) {
            const corner = label.position.clone()
              .addScaledVector(right, horizontal)
              .addScaledVector(up, vertical)
              .project(gizmoCamera);
            expect(Math.abs(corner.x)).toBeLessThanOrEqual(1);
            expect(Math.abs(corner.y)).toBeLessThanOrEqual(1);
          }
        }
      }
    }
    const projectedBadgePixels = GIZMO_AXIS_LABEL_WORLD_SIZE * GIZMO_MAX_CSS_SIZE /
      (2 * GIZMO_CAMERA_DISTANCE * Math.tan(THREE.MathUtils.degToRad(45 / 2)));
    const projectedGlyphEmPixels = projectedBadgePixels * GIZMO_AXIS_LABEL_FONT_PX / 64;
    expect(projectedGlyphEmPixels).toBeGreaterThanOrEqual(10);
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

  it("keeps theme-aware gizmo axes and glyphs contrast-safe without replacing the scene", () => {
    for (const palette of Object.values(GIZMO_THEME_PALETTES)) {
      for (const axis of palette.axes) {
        expect(relativeContrastRatio(axis, palette.canvas)).toBeGreaterThanOrEqual(3);
        expect(relativeContrastRatio(axis, palette.badge)).toBeGreaterThanOrEqual(4.5);
      }
    }
    const scene = new THREE.Scene();
    const axes = new THREE.AxesHelper(1);
    axes.userData.gizmoAxes = true;
    scene.add(axes);
    applyGizmoThemePresentation(scene, "dark");
    const colors = axes.geometry.getAttribute("color") as THREE.BufferAttribute;
    const darkX = new THREE.Color(GIZMO_THEME_PALETTES.dark.axes[0]);
    expect(colors.getX(0)).toBeCloseTo(darkX.r);
    applyGizmoThemePresentation(scene, "light");
    expect(scene.children[0]).toBe(axes);
    expect(colors.getX(0)).not.toBeCloseTo(darkX.r);
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
      invalidate: vi.fn(),
      scene: new THREE.Scene(),
      modelLayer: layer,
      authoredLoadLayer: new THREE.Group(),
      resultLayer: new THREE.Group(),
      diagnosticLayer: new THREE.Group(),
      selectedKeys: new Set(),
      hiddenKeys: new Set(),
      themePresentation: "light",
      gridVisible: true,
      authoredLoadsVisible: true
    }) as ViewportResource;

    resource.replaceLayer(layer, [next]);
    expect(layer.children).toEqual([next]);
    expect(instanceDispose).toHaveBeenCalledTimes(1);
    expect(geometryDispose).toHaveBeenCalledTimes(1);
    expect(materialDispose).toHaveBeenCalledTimes(1);
    expect(ownership.snapshot().live.instanceMatrices).toBe(0);
    expect(ownership.snapshot().live.instanceColors).toBe(0);
  });

  it("reapplies hidden, selected, theme, grid, and load presentation after layer replacement", () => {
    const hidden = entityKey({ type: "pipe", id: "p:hidden" });
    const selected = entityKey({ type: "pipe", id: "p:selected" });
    const modelLayer = new THREE.Group();
    const authoredLoadLayer = new THREE.Group();
    const resultLayer = new THREE.Group();
    const diagnosticLayer = new THREE.Group();
    const resource = Object.assign(Object.create(ViewportResource.prototype), {
      ownership: new ViewportOwnershipLedger(12),
      resourceGeneration: 12,
      contextLostCount: 0,
      contextRestoredCount: 0,
      scheduler: { pendingCount: 0 },
      renderer: { domElement: { isConnected: true } },
      options: {},
      invalidate: vi.fn(),
      scene: new THREE.Scene(),
      modelLayer,
      authoredLoadLayer,
      resultLayer,
      diagnosticLayer,
      selectedKeys: new Set([selected]),
      hiddenKeys: new Set([hidden]),
      themePresentation: "light",
      gridVisible: false,
      authoredLoadsVisible: false
    }) as ViewportResource;
    const geometry = new THREE.BoxGeometry(1, 1, 1);
    const material = new THREE.MeshStandardMaterial({ color: 0xffffff });
    const mesh = new THREE.InstancedMesh(geometry, material, 2);
    mesh.setMatrixAt(0, new THREE.Matrix4().makeTranslation(1, 0, 0));
    mesh.setMatrixAt(1, new THREE.Matrix4().makeTranslation(2, 0, 0));
    registerInstancedSelectionPresentation(mesh, [hidden, selected], 0x4f6f73);
    const ground = new THREE.GridHelper(2, 2);
    ground.name = "viewport-reference-ground";

    resource.replaceLayer(modelLayer, [mesh, ground]);
    const hiddenMatrix = new THREE.Matrix4();
    mesh.getMatrixAt(0, hiddenMatrix);
    expect(hiddenMatrix.determinant()).toBe(0);
    expect(new THREE.Color().fromBufferAttribute(mesh.instanceColor!, 1).getHex()).toBe(0xa34400);
    expect(ground.visible).toBe(false);

    const load = new THREE.Mesh(new THREE.BoxGeometry(1, 1, 1), new THREE.MeshStandardMaterial());
    registerSelectionPresentation(load, hidden);
    resource.replaceLayer(authoredLoadLayer, [load]);
    expect(load.visible).toBe(false);
    expect(authoredLoadLayer.visible).toBe(false);
  });

  it("disposes spatial chunk instances and their intentionally shared assets exactly once", () => {
    const root = new THREE.Group();
    const geometry = new THREE.BoxGeometry(1, 1, 1);
    const material = new THREE.MeshStandardMaterial();
    const first = new THREE.InstancedMesh(geometry, material, 1);
    const second = new THREE.InstancedMesh(geometry, material, 1);
    const geometryDispose = vi.spyOn(geometry, "dispose");
    const materialDispose = vi.spyOn(material, "dispose");
    const firstDispose = vi.spyOn(first, "dispose");
    const secondDispose = vi.spyOn(second, "dispose");
    root.add(first, second);

    disposeObjectChildren(root);
    expect(firstDispose).toHaveBeenCalledTimes(1);
    expect(secondDispose).toHaveBeenCalledTimes(1);
    expect(geometryDispose).toHaveBeenCalledTimes(1);
    expect(materialDispose).toHaveBeenCalledTimes(1);
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
    const teardownGeneration = (currentOwnedViewportResourceSnapshot()?.context.generation ?? 0) + 1_000_000;
    const ownership = new ViewportOwnershipLedger(teardownGeneration);
    ownership.createLifecycle({ controls: 1, eventBindings: 7, resizeObservers: 1 });
    const removeWindowListener = vi.spyOn(window, "removeEventListener");
    const removeDocumentListener = vi.spyOn(document, "removeEventListener");
    const resource = Object.assign(Object.create(ViewportResource.prototype), {
      disposed: false,
      resourceGeneration: teardownGeneration,
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
      owned: { generation: teardownGeneration, live: zeroOwnedCounts() },
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

describe("masked instance rebuild bounds", () => {
  it.each([1, 5])("restores and picks a distant rebuilt chunk with radius %s", (radius) => {
    const key = entityKey({ type: "pipe", id: "distant" });
    const mesh = new THREE.InstancedMesh(new THREE.BoxGeometry(2 * radius, 2 * radius, 2 * radius), new THREE.MeshBasicMaterial(), 1);
    mesh.setMatrixAt(0, new THREE.Matrix4().makeTranslation(10000, 0, 0));
    registerInstancedSelectionPresentation(mesh, [key], 0xffffff);
    applyVisibilityPresentation([mesh], new Set([key]));
    const camera = new THREE.PerspectiveCamera(42, 1, 0.1, 100);
    camera.position.set(10000, 0, 20); camera.lookAt(10000, 0, 0); camera.updateMatrixWorld();
    const frustum = new THREE.Frustum().setFromProjectionMatrix(new THREE.Matrix4().multiplyMatrices(camera.projectionMatrix, camera.matrixWorldInverse));
    expect(mesh.frustumCulled).toBe(true);
    expect(frustum.intersectsObject(mesh)).toBe(true);
    applyVisibilityPresentation([mesh], new Set());
    expect(frustum.intersectsObject(mesh)).toBe(true);
    expect(mesh.boundingBox!.containsPoint(new THREE.Vector3(10000 + radius, 0, 0))).toBe(true);
    const ray = new THREE.Raycaster(); ray.setFromCamera(new THREE.Vector2(0, 0), camera);
    const hits = ray.intersectObject(mesh);
    expect(hits.length).toBeGreaterThan(0);
    expect(hits[0].instanceId).toBe(0);
    expect(mesh.userData.instanceEntityKeys[hits[0].instanceId!]).toBe(key);
    mesh.geometry.dispose(); (mesh.material as THREE.Material).dispose(); mesh.dispose();
  });
});

describe("viewport palette repaint", () => {
  const instanceHex = (mesh: THREE.InstancedMesh, index: number) =>
    new THREE.Color().fromBufferAttribute(mesh.instanceColor!, index).getHex();

  function paletteResource(theme: "light" | "dark", selectedKeys: ReadonlySet<string> = new Set()) {
    const layers = {
      modelLayer: new THREE.Group(),
      authoredLoadLayer: new THREE.Group(),
      resultLayer: new THREE.Group(),
      diagnosticLayer: new THREE.Group(),
      routingLayer: new THREE.Group()
    };
    const invalidate = vi.fn();
    const ownership = new ViewportOwnershipLedger(21);
    const resource = Object.assign(Object.create(ViewportResource.prototype), {
      ownership,
      resourceGeneration: 21,
      contextLostCount: 0,
      contextRestoredCount: 0,
      scheduler: { pendingCount: 0 },
      renderer: { domElement: { isConnected: true } },
      options: {},
      invalidate,
      scene: new THREE.Scene(),
      gizmoScene: new THREE.Scene(),
      selectionPresentation: null,
      ...layers,
      selectedKeys: new Set(selectedKeys),
      hiddenKeys: new Set(),
      themePresentation: theme,
      gridVisible: true,
      authoredLoadsVisible: true
    }) as ViewportResource;
    return { resource, ownership, invalidate, ...layers };
  }

  function rolePipes(keys: readonly ReturnType<typeof entityKey>[]) {
    const mesh = new THREE.InstancedMesh(new THREE.CylinderGeometry(1, 1, 1, 8), createFigureMaterial(), keys.length);
    registerInstancedRolePresentation(mesh, keys, "pipe");
    return mesh;
  }

  it("paints a role-registered instanced mesh with the dark token after a dark repaint and the light token after a light one", () => {
    const { resource, modelLayer } = paletteResource("light");
    const mesh = rolePipes([entityKey({ type: "pipe", id: "p:1" }), entityKey({ type: "pipe", id: "p:2" })]);
    const material = mesh.material as ReturnType<typeof createFigureMaterial>;
    resource.replaceLayer(modelLayer, [mesh]);
    expect(viewportRoleHex("dark", "pipe")).not.toBe(viewportRoleHex("light", "pipe"));

    resource.setThemePresentation("dark");
    expect(instanceHex(mesh, 0)).toBe(viewportRoleHex("dark", "pipe"));
    expect(instanceHex(mesh, 1)).toBe(viewportRoleHex("dark", "pipe"));
    expect(mesh.userData.viewportBaseColor).toBe(viewportRoleHex("dark", "pipe"));
    expect((material.uniforms.shadeRatio.value as THREE.Color).toArray()).toEqual([...viewportShadeRatio("dark")]);

    resource.setThemePresentation("light");
    expect(instanceHex(mesh, 0)).toBe(viewportRoleHex("light", "pipe"));
    expect(instanceHex(mesh, 1)).toBe(viewportRoleHex("light", "pipe"));
    expect(mesh.userData.viewportBaseColor).toBe(viewportRoleHex("light", "pipe"));
    expect((material.uniforms.shadeRatio.value as THREE.Color).toArray()).toEqual([...viewportShadeRatio("light")]);
  });

  it("keeps the held selected colour on a selected instance across a repaint while an unselected one takes the new base", () => {
    const unselected = entityKey({ type: "pipe", id: "p:plain" });
    const selected = entityKey({ type: "pipe", id: "p:selected" });
    const { resource, modelLayer } = paletteResource("light", new Set([selected]));
    const mesh = rolePipes([unselected, selected]);
    resource.replaceLayer(modelLayer, [mesh]);
    expect(instanceHex(mesh, 0)).toBe(viewportRoleHex("light", "pipe"));
    expect(instanceHex(mesh, 1)).toBe(0xa34400);

    resource.setThemePresentation("dark");
    expect(instanceHex(mesh, 0)).toBe(viewportRoleHex("dark", "pipe"));
    expect(instanceHex(mesh, 1)).toBe(0xf08c22);

    resource.setThemePresentation("light");
    expect(instanceHex(mesh, 0)).toBe(viewportRoleHex("light", "pipe"));
    expect(instanceHex(mesh, 1)).toBe(0xa34400);

    // Deselecting after a repaint returns the instance to the repainted base, not to a stale one.
    resource.setThemePresentation("dark");
    resource.setSelectionPresentation([]);
    expect(instanceHex(mesh, 1)).toBe(viewportRoleHex("dark", "pipe"));
  });

  it("carries the two grid tokens in a painted GridHelper's colour attribute, rewritten in place", () => {
    const grid = new THREE.GridHelper(8, 8);
    const colors = grid.geometry.getAttribute("color") as THREE.BufferAttribute;
    const geometry = grid.geometry;
    registerGridPaletteRoles(grid, "groundGridMajor", "groundGridMinor");
    expect(colors.count).toBe(4 * 9);
    const expectGridColours = (theme: "light" | "dark") => {
      const major = new THREE.Color(viewportRoleHex(theme, "groundGridMajor"));
      const minor = new THREE.Color(viewportRoleHex(theme, "groundGridMinor"));
      for (let vertex = 0; vertex < colors.count; vertex += 1) {
        // Line index 4 of 0 to 8 is the helper's centre line in each direction.
        const expected = Math.floor(vertex / 4) === 4 ? major : minor;
        expect(colors.getX(vertex)).toBeCloseTo(expected.r, 6);
        expect(colors.getY(vertex)).toBeCloseTo(expected.g, 6);
        expect(colors.getZ(vertex)).toBeCloseTo(expected.b, 6);
      }
    };
    expectGridColours("light");

    const versionBefore = colors.version;
    applyPalettePresentation([grid], "dark");
    expectGridColours("dark");
    expect(colors.version).toBeGreaterThan(versionBefore);
    applyPalettePresentation([grid], "light");
    expectGridColours("light");
    expect(grid.geometry).toBe(geometry);
    expect(grid.geometry.getAttribute("color")).toBe(colors);
  });

  it("paints incoming objects for the resource's current theme on layer replacement", () => {
    const { resource, modelLayer, routingLayer } = paletteResource("dark");
    const mesh = rolePipes([entityKey({ type: "pipe", id: "p:incoming" })]);
    expect(instanceHex(mesh, 0)).toBe(viewportRoleHex("light", "pipe"));
    resource.replaceLayer(modelLayer, [mesh]);
    expect(instanceHex(mesh, 0)).toBe(viewportRoleHex("dark", "pipe"));

    const marker = new THREE.Mesh(new THREE.SphereGeometry(0.1, 8, 6), createFigureMaterial());
    registerPaletteRole(marker, "routeDraft");
    resource.replaceLayer(routingLayer, [marker]);
    expect((marker.material as ReturnType<typeof createFigureMaterial>).color.getHex()).toBe(viewportRoleHex("dark", "routeDraft"));
  });

  it("replaces a layer with an empty list without a console error, leaving the layer empty", () => {
    const { resource, ownership, invalidate, modelLayer } = paletteResource("light");
    const ledgerEmpty = ownership.snapshot();
    const mesh = rolePipes([entityKey({ type: "pipe", id: "p:leaving" })]);
    const disposeGeometry = vi.spyOn(mesh.geometry, "dispose");
    resource.replaceLayer(modelLayer, [mesh]);
    expect(modelLayer.children).toEqual([mesh]);
    invalidate.mockClear();
    const consoleError = vi.spyOn(console, "error").mockImplementation(() => undefined);
    try {
      // three's Object3D.add reports an error when it is called with no object at all.
      resource.replaceLayer(modelLayer, []);
      expect(consoleError).not.toHaveBeenCalled();
    } finally {
      consoleError.mockRestore();
    }
    expect(modelLayer.children).toEqual([]);
    expect(disposeGeometry).toHaveBeenCalledTimes(1);
    expect(ownership.snapshot().live).toEqual(ledgerEmpty.live);
    expect(invalidate).toHaveBeenCalledTimes(1);
  });

  it("repaints every role colour in all five layers without changing the ownership ledger or any resource identity", () => {
    const selected = entityKey({ type: "pipe", id: "p:kept" });
    const { resource, ownership, invalidate, modelLayer, authoredLoadLayer, resultLayer, diagnosticLayer, routingLayer } =
      paletteResource("light", new Set([selected]));

    const pipes = rolePipes([selected, entityKey({ type: "pipe", id: "p:other" })]);
    const ground = new THREE.GridHelper(4, 4);
    ground.name = "viewport-reference-ground";
    registerGridPaletteRoles(ground, "groundGridMajor", "groundGridMinor");
    const arrows = new THREE.InstancedMesh(new THREE.CylinderGeometry(0.1, 0.1, 1, 6), new THREE.MeshBasicMaterial(), 1);
    registerInstancedRolePresentation(arrows, [entityKey({ type: "load", id: "l:force" })], "loadForce");
    const deformed = new THREE.InstancedMesh(
      new THREE.CylinderGeometry(1, 1, 1, 8), createFigureMaterial({ opacity: 0.82, transparent: true }), 1
    );
    registerInstancedRolePresentation(deformed, [entityKey({ type: "pipe", id: "p:deformed" })], "deformedShape");
    const diagnostic = new THREE.Mesh(new THREE.SphereGeometry(0.1, 8, 6), createFigureMaterial());
    registerPaletteRole(diagnostic, "node");
    const ghost = new THREE.Line(
      new THREE.BufferGeometry().setFromPoints([new THREE.Vector3(), new THREE.Vector3(1, 0, 0)]),
      new THREE.LineDashedMaterial({ dashSize: 0.18, gapSize: 0.1 })
    );
    registerPaletteRole(ghost, "routeDraft");
    const routeGrid = new THREE.GridHelper(2, 2);
    registerGridPaletteRoles(routeGrid, "routeGridAxis", "routeGridLine");

    resource.replaceLayer(modelLayer, [pipes, ground]);
    resource.replaceLayer(authoredLoadLayer, [arrows]);
    resource.replaceLayer(resultLayer, [deformed]);
    resource.replaceLayer(diagnosticLayer, [diagnostic]);
    resource.replaceLayer(routingLayer, [ghost, routeGrid]);

    const renderables = [pipes, ground, arrows, deformed, diagnostic, ghost, routeGrid];
    const identities = renderables.map((object) => ({
      geometry: object.geometry,
      material: object.material,
      colors: object.geometry.getAttribute("color") ?? null,
      instanceColor: object instanceof THREE.InstancedMesh ? object.instanceColor : null
    }));
    const disposals = renderables.flatMap((object) => [
      vi.spyOn(object.geometry, "dispose"),
      vi.spyOn(object.material as THREE.Material, "dispose")
    ]);
    const ledgerBefore = ownership.snapshot();
    const resourceSnapshotBefore = currentOwnedViewportResourceSnapshot();
    invalidate.mockClear();

    resource.setThemePresentation("dark");
    expect(invalidate).toHaveBeenCalledTimes(1);
    expect(instanceHex(pipes, 0)).toBe(0xf08c22);
    expect(instanceHex(pipes, 1)).toBe(viewportRoleHex("dark", "pipe"));
    expect(instanceHex(arrows, 0)).toBe(viewportRoleHex("dark", "loadForce"));
    expect(instanceHex(deformed, 0)).toBe(viewportRoleHex("dark", "deformedShape"));
    expect((diagnostic.material as ReturnType<typeof createFigureMaterial>).color.getHex()).toBe(viewportRoleHex("dark", "node"));
    expect((ghost.material as THREE.LineDashedMaterial).color.getHex()).toBe(viewportRoleHex("dark", "routeDraft"));
    const routeColors = routeGrid.geometry.getAttribute("color") as THREE.BufferAttribute;
    // A two-division helper has line indices 0, 1, 2: index 1 is the centre line.
    expect(new THREE.Color().fromBufferAttribute(routeColors, 4).getHex()).toBe(viewportRoleHex("dark", "routeGridAxis"));
    expect(new THREE.Color().fromBufferAttribute(routeColors, 0).getHex()).toBe(viewportRoleHex("dark", "routeGridLine"));
    const deformedMaterial = deformed.material as ReturnType<typeof createFigureMaterial>;
    expect(deformedMaterial.transparent).toBe(true);
    expect(deformedMaterial.uniforms.opacity.value).toBe(0.82);

    resource.setThemePresentation("light");
    expect(invalidate).toHaveBeenCalledTimes(2);
    expect(instanceHex(pipes, 0)).toBe(0xa34400);
    expect(instanceHex(arrows, 0)).toBe(viewportRoleHex("light", "loadForce"));
    expect((ghost.material as THREE.LineDashedMaterial).color.getHex()).toBe(viewportRoleHex("light", "routeDraft"));

    expect(ownership.snapshot()).toEqual(ledgerBefore);
    expect(currentOwnedViewportResourceSnapshot()).toBe(resourceSnapshotBefore);
    renderables.forEach((object, index) => {
      expect(object.geometry).toBe(identities[index].geometry);
      expect(object.material).toBe(identities[index].material);
      expect(object.geometry.getAttribute("color") ?? null).toBe(identities[index].colors);
      if (object instanceof THREE.InstancedMesh) expect(object.instanceColor).toBe(identities[index].instanceColor);
    });
    for (const dispose of disposals) expect(dispose).not.toHaveBeenCalled();
    expect(modelLayer.children).toEqual([pipes, ground]);
    expect(routingLayer.children).toEqual([ghost, routeGrid]);
  });

  it("leaves a mesh registered without a role at the number it was given", () => {
    const { resource, modelLayer } = paletteResource("light");
    const mesh = new THREE.InstancedMesh(new THREE.BoxGeometry(1, 1, 1), new THREE.MeshBasicMaterial(), 1);
    registerInstancedSelectionPresentation(mesh, [entityKey({ type: "pipe", id: "p:unroled" })], 0x4f6f73);
    resource.replaceLayer(modelLayer, [mesh]);
    resource.setThemePresentation("dark");
    expect(instanceHex(mesh, 0)).toBe(0x4f6f73);
    expect(mesh.userData.viewportBaseColor).toBe(0x4f6f73);
  });
});
