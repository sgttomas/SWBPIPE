import * as THREE from "three";
import { OrbitControls } from "three/addons/controls/OrbitControls.js";
import type { Vec3 } from "../../types";
import type { EntityRef } from "../../types";
import type { ModelIndex } from "../workspace/modelIndex";
import type { EntityKey } from "../workspace/selectionState";
import {
  authoredToLocal,
  localToAuthored,
  pickPointPrimitive,
  type PointPickPrimitive
} from "./viewportSelection";

export type FrameRequest = (callback: FrameRequestCallback) => number;
export type FrameCancel = (handle: number) => void;

export class ViewportInvalidationScheduler {
  private pendingHandle: number | null = null;
  private pendingOpportunityHandle: number | null = null;
  private paused = false;
  private disposed = false;

  constructor(
    private readonly renderFrame: (time: number) => boolean,
    private readonly requestFrame: FrameRequest = (callback) => window.requestAnimationFrame(callback),
    private readonly cancelFrame: FrameCancel = (handle) => window.cancelAnimationFrame(handle),
    private readonly onPendingChange: (count: number) => void = () => {}
  ) {}

  get pendingCount(): number {
    return (this.pendingHandle === null ? 0 : 1) + (this.pendingOpportunityHandle === null ? 0 : 1);
  }

  invalidate(): void {
    if (this.disposed || this.paused || this.pendingHandle !== null) return;
    this.pendingHandle = this.requestFrame((time) => {
      this.pendingHandle = null;
      this.onPendingChange(this.pendingCount);
      if (this.disposed || this.paused) return;
      if (this.renderFrame(time)) this.invalidate();
    });
    this.onPendingChange(this.pendingCount);
  }

  schedulePaintOpportunity(callback: FrameRequestCallback): void {
    if (this.disposed || this.paused) return;
    if (this.pendingOpportunityHandle !== null) this.cancelFrame(this.pendingOpportunityHandle);
    this.pendingOpportunityHandle = this.requestFrame((time) => {
      this.pendingOpportunityHandle = null;
      this.onPendingChange(this.pendingCount);
      if (!this.disposed && !this.paused) callback(time);
    });
    this.onPendingChange(this.pendingCount);
  }

  pause(): void {
    this.paused = true;
    this.cancelPending();
  }

  resume(): void {
    if (this.disposed) return;
    this.paused = false;
    this.invalidate();
  }

  dispose(): void {
    if (this.disposed) return;
    this.disposed = true;
    this.cancelPending();
  }

  private cancelPending(): void {
    if (this.pendingHandle !== null) this.cancelFrame(this.pendingHandle);
    if (this.pendingOpportunityHandle !== null) this.cancelFrame(this.pendingOpportunityHandle);
    this.pendingHandle = null;
    this.pendingOpportunityHandle = null;
    this.onPendingChange(this.pendingCount);
  }
}

export class ViewportResourceRegistry {
  private readonly resources = new Set<{ dispose(): void }>();
  private disposed = false;

  own<T extends { dispose(): void }>(resource: T): T {
    if (this.disposed) {
      resource.dispose();
      throw new Error("Cannot own a viewport resource after disposal.");
    }
    this.resources.add(resource);
    return resource;
  }

  release(resource: { dispose(): void }): void {
    if (!this.resources.delete(resource)) return;
    resource.dispose();
  }

  dispose(): void {
    if (this.disposed) return;
    this.disposed = true;
    for (const resource of this.resources) resource.dispose();
    this.resources.clear();
  }
}

export type ViewportOwnedCounts = Readonly<{
  pipeMeshes: number;
  nodeMeshes: number;
  supportMeshes: number;
  componentMeshes: number;
  geometries: number;
  materials: number;
  textures: number;
  instanceMatrices: number;
  instanceColors: number;
  controls: number;
  eventBindings: number;
  resizeObservers: number;
}>;

export type ViewportOwnershipSnapshot = Readonly<{
  generation: number;
  live: ViewportOwnedCounts;
  created: ViewportOwnedCounts;
  disposed: ViewportOwnedCounts;
}>;

export type OwnedViewportResourceSnapshot = Readonly<{
  ownedPendingRafCount: number;
  owned: ViewportOwnershipSnapshot;
  context: Readonly<{
    generation: number;
    canvasConnected: boolean;
    lostCount: number;
    restoredCount: number;
  }>;
}>;

let latestOwnedViewportResourceSnapshot: OwnedViewportResourceSnapshot | null = null;

/** Bounded source-only owner state for component teardown witnesses. */
export function currentOwnedViewportResourceSnapshot(): OwnedViewportResourceSnapshot | null {
  return latestOwnedViewportResourceSnapshot;
}

export class ViewportOwnershipLedger {
  private readonly created = emptyOwnedCounts();
  private readonly disposed = emptyOwnedCounts();

  constructor(readonly generation: number) {}

  createObjects(objects: readonly THREE.Object3D[]): void {
    addOwnedCounts(this.created, countOwnedObjects(objects));
  }

  disposeObjects(objects: readonly THREE.Object3D[]): void {
    addOwnedCounts(this.disposed, countOwnedObjects(objects));
  }

  createLifecycle(counts: Partial<ViewportOwnedCounts>): void {
    addOwnedCounts(this.created, counts);
  }

  disposeLifecycle(counts: Partial<ViewportOwnedCounts>): void {
    addOwnedCounts(this.disposed, counts);
  }

  snapshot(): ViewportOwnershipSnapshot {
    const live = emptyOwnedCounts();
    for (const key of OWNED_COUNT_KEYS) live[key] = this.created[key] - this.disposed[key];
    return Object.freeze({
      generation: this.generation,
      live: Object.freeze({ ...live }),
      created: Object.freeze({ ...this.created }),
      disposed: Object.freeze({ ...this.disposed })
    });
  }
}

export type ViewportContextStatus = "ready" | "lost" | "restoring";
export type ViewportThemePresentation = "light" | "dark";
export type ViewportRendererInfo = Readonly<{
  geometries: number;
  textures: number;
  calls: number;
  triangles: number;
  points: number;
  lines: number;
}>;

const SELECTED_COLOR = 0xf08c22;
const SELECTED_EMISSIVE = 0x4c2500;

export function registerSelectionPresentation(object: THREE.Object3D, key: EntityKey): void {
  const ownershipKind = ownershipKindFromKey(key);
  object.traverse((child) => {
    child.userData.selectionEntityKey = key;
    if (ownershipKind && child instanceof THREE.Mesh) child.userData.viewportOwnershipKind = ownershipKind;
    for (const material of objectMaterials(child)) {
      if (!("color" in material) || !(material.color instanceof THREE.Color)) continue;
      if (material.userData.viewportBaseColor === undefined) {
        material.userData.viewportBaseColor = material.color.getHex();
      }
      if ("emissive" in material && material.emissive instanceof THREE.Color &&
          material.userData.viewportBaseEmissive === undefined) {
        material.userData.viewportBaseEmissive = material.emissive.getHex();
      }
    }
  });
}

export function registerInstancedSelectionPresentation(
  mesh: THREE.InstancedMesh,
  keys: readonly EntityKey[],
  baseColor: number
): void {
  mesh.userData.instanceEntityKeys = Object.freeze([...keys]);
  mesh.userData.viewportBaseColor = baseColor;
  const baseMatrices: THREE.Matrix4[] = [];
  for (let index = 0; index < keys.length; index += 1) {
    const matrix = new THREE.Matrix4();
    mesh.getMatrixAt(index, matrix);
    baseMatrices.push(matrix.clone());
  }
  mesh.userData.instanceBaseMatrices = Object.freeze(baseMatrices);
  const ownershipKind = keys[0] ? ownershipKindFromKey(keys[0]) : null;
  if (ownershipKind) mesh.userData.viewportOwnershipKind = ownershipKind;
  applyInstancedSelection(mesh, new Set());
}

export function applySelectionPresentation(
  roots: readonly THREE.Object3D[],
  selectedKeys: ReadonlySet<EntityKey>
): void {
  for (const root of roots) {
    root.traverse((object) => {
      if (object instanceof THREE.InstancedMesh && Array.isArray(object.userData.instanceEntityKeys)) {
        applyInstancedSelection(object, selectedKeys);
        return;
      }
      const key = object.userData.selectionEntityKey as EntityKey | undefined;
      if (!key) return;
      const selected = selectedKeys.has(key);
      for (const material of objectMaterials(object)) {
        if ("color" in material && material.color instanceof THREE.Color &&
            typeof material.userData.viewportBaseColor === "number") {
          material.color.setHex(selected ? SELECTED_COLOR : material.userData.viewportBaseColor);
        }
        if ("emissive" in material && material.emissive instanceof THREE.Color &&
            typeof material.userData.viewportBaseEmissive === "number") {
          material.emissive.setHex(selected ? SELECTED_EMISSIVE : material.userData.viewportBaseEmissive);
        }
        material.needsUpdate = true;
      }
    });
  }
}

export function applyThemePresentation(scene: THREE.Scene, theme: ViewportThemePresentation): void {
  scene.background = new THREE.Color(theme === "dark" ? 0x111820 : 0xf6f7f4);
}

export function applyVisibilityPresentation(
  roots: readonly THREE.Object3D[],
  hiddenKeys: ReadonlySet<EntityKey>
): void {
  const hiddenMatrix = new THREE.Matrix4().makeScale(0, 0, 0);
  for (const root of roots) {
    root.traverse((object) => {
      if (object instanceof THREE.InstancedMesh && Array.isArray(object.userData.instanceEntityKeys)) {
        const keys = object.userData.instanceEntityKeys as readonly EntityKey[];
        const matrices = object.userData.instanceBaseMatrices as readonly THREE.Matrix4[] | undefined;
        if (!matrices) return;
        keys.forEach((key, index) => object.setMatrixAt(index, hiddenKeys.has(key) ? hiddenMatrix : matrices[index]));
        object.instanceMatrix.needsUpdate = true;
        return;
      }
      const key = object.userData.selectionEntityKey as EntityKey | undefined;
      if (key) object.visible = !hiddenKeys.has(key);
    });
  }
}

export type ViewportResourceOptions = {
  onContextStatus?: (status: ViewportContextStatus) => void;
  onAfterMainFrame?: (
    resource: ViewportResource,
    submittedAt: number,
    submissionSequence: number,
    rendererInfo: ViewportRendererInfo
  ) => void;
  onNextPaintOpportunity?: (resource: ViewportResource, opportunityAt: number, submissionSequence: number) => void;
  onOwnedRafCountChange?: (resource: ViewportResource, pendingCount: number) => void;
  onResourceStateChange?: (resource: ViewportResource, retired: boolean) => void;
  shouldObservePaintOpportunity?: () => boolean;
  onCameraChange?: (resource: ViewportResource) => void;
  onRestore?: () => void;
};

let nextViewportResourceGeneration = 0;
let nextViewportCameraSequence = 0;

export class ViewportResource {
  readonly scene = new THREE.Scene();
  readonly camera: THREE.PerspectiveCamera;
  readonly renderer: THREE.WebGLRenderer;
  readonly controls: OrbitControls;
  readonly modelLayer = new THREE.Group();
  readonly authoredLoadLayer = new THREE.Group();
  readonly resultLayer = new THREE.Group();
  readonly diagnosticLayer = new THREE.Group();
  readonly routingLayer = new THREE.Group();
  readonly scheduler: ViewportInvalidationScheduler;
  readonly resourceGeneration = ++nextViewportResourceGeneration;

  private readonly gizmoScene = new THREE.Scene();
  private readonly gizmoCamera = new THREE.PerspectiveCamera(45, 1, 0.1, 100);
  private readonly resizeObserver: ResizeObserver | null;
  private readonly raycaster = new THREE.Raycaster();
  private pickables: THREE.Object3D[] = [];
  private pointPrimitives: readonly PointPickPrimitive[] = [];
  private modelIndex: ModelIndex | null = null;
  private hiddenKeys: ReadonlySet<EntityKey> = new Set();
  private actualOdRadiusByPipe: ReadonlyMap<EntityKey, number> = new Map();
  private renderOrigin: Readonly<Vec3> = Object.freeze({ x: 0, y: 0, z: 0 });
  private labelUpdater: (() => void) | null = null;
  private disposed = false;
  private contextStatus: ViewportContextStatus = "ready";
  private contextLostCount = 0;
  private contextRestoredCount = 0;
  private frameSubmissionSequence = 0;
  private cameraChangeSequence = ++nextViewportCameraSequence;
  private readonly ownership = new ViewportOwnershipLedger(this.resourceGeneration);

  constructor(
    readonly host: HTMLDivElement,
    private readonly options: ViewportResourceOptions = {}
  ) {
    this.scene.background = new THREE.Color(0xf6f7f4);
    this.camera = new THREE.PerspectiveCamera(42, safeAspect(host), 0.1, 10_000);
    this.renderer = new THREE.WebGLRenderer({ antialias: true });
    this.renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    this.renderer.setSize(Math.max(1, host.clientWidth), Math.max(1, host.clientHeight));
    host.replaceChildren(this.renderer.domElement);

    this.controls = new OrbitControls(this.camera, this.renderer.domElement);
    this.controls.enableDamping = true;
    this.controls.dampingFactor = 0.08;
    this.scene.add(this.modelLayer, this.authoredLoadLayer, this.resultLayer, this.diagnosticLayer, this.routingLayer);
    this.scene.add(new THREE.AmbientLight(0xffffff, 0.72));
    const key = new THREE.DirectionalLight(0xffffff, 1.2);
    key.position.set(4, 9, 7);
    this.scene.add(key);
    this.gizmoScene.add(new THREE.AxesHelper(1.25));
    this.ownership.createObjects(this.gizmoScene.children);

    this.scheduler = new ViewportInvalidationScheduler(
      (time) => this.render(time),
      undefined,
      undefined,
      (pendingCount) => this.options.onOwnedRafCountChange?.(this, pendingCount)
    );
    this.controls.addEventListener("change", this.handleControlsChange);
    this.controls.addEventListener("start", this.invalidate);
    this.controls.addEventListener("end", this.invalidate);
    this.renderer.domElement.addEventListener("webglcontextlost", this.handleContextLost);
    this.renderer.domElement.addEventListener("webglcontextrestored", this.handleContextRestored);
    window.addEventListener("resize", this.resize);
    document.addEventListener("visibilitychange", this.handleVisibilityChange);
    this.resizeObserver = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(this.resize);
    this.resizeObserver?.observe(host);
    this.ownership.createLifecycle({
      controls: 1,
      eventBindings: 7,
      resizeObservers: this.resizeObserver ? 1 : 0
    });
    this.resize();
    if (document.visibilityState === "hidden") this.scheduler.pause();
    this.options.onContextStatus?.("ready");
    this.notifyResourceStateChange(false);
  }

  get origin(): Readonly<Vec3> {
    return this.renderOrigin;
  }

  get pendingAppOwnedRafCount(): number {
    return this.scheduler.pendingCount;
  }

  get submissionSequence(): number {
    return this.frameSubmissionSequence;
  }

  get cameraSequence(): number {
    return this.cameraChangeSequence;
  }

  get ownershipSnapshot(): ViewportOwnershipSnapshot {
    return this.ownership.snapshot();
  }

  get contextSnapshot(): Readonly<{
    generation: number;
    canvasConnected: boolean;
    lostCount: number;
    restoredCount: number;
  }> {
    return Object.freeze({
      generation: this.resourceGeneration,
      canvasConnected: this.renderer.domElement.isConnected,
      lostCount: this.contextLostCount,
      restoredCount: this.contextRestoredCount
    });
  }

  setOrigin(origin: Readonly<Vec3>): void {
    if (![origin.x, origin.y, origin.z].every(Number.isFinite)) {
      throw new Error("Viewport render origin must be finite.");
    }
    const localShift = new THREE.Vector3(
      this.renderOrigin.x - origin.x,
      this.renderOrigin.y - origin.y,
      this.renderOrigin.z - origin.z
    );
    this.camera.position.add(localShift);
    this.controls.target.add(localShift);
    this.renderOrigin = Object.freeze({ x: origin.x, y: origin.y, z: origin.z });
    this.camera.updateMatrixWorld();
    this.markCameraProjectionChanged();
    this.invalidate();
  }

  setPickables(pickables: readonly THREE.Object3D[]): void {
    this.pickables = [...pickables];
  }

  setPointPrimitives(index: ModelIndex, primitives: readonly PointPickPrimitive[]): void {
    this.modelIndex = index;
    this.pointPrimitives = primitives;
  }

  setActualOdRadiusByPipe(radii: ReadonlyMap<EntityKey, number>): void {
    this.actualOdRadiusByPipe = new Map(radii);
  }

  setLabelUpdater(updater: (() => void) | null): void {
    this.labelUpdater = updater;
  }

  setSelectionPresentation(selectedKeys: readonly EntityKey[]): void {
    applySelectionPresentation(
      [this.modelLayer, this.authoredLoadLayer, this.resultLayer, this.diagnosticLayer],
      new Set(selectedKeys)
    );
    this.invalidate();
  }

  setThemePresentation(theme: ViewportThemePresentation): void {
    applyThemePresentation(this.scene, theme);
    this.invalidate();
  }

  setVisibilityPresentation(hiddenKeys: ReadonlySet<EntityKey>): void {
    this.hiddenKeys = new Set(hiddenKeys);
    applyVisibilityPresentation(
      [this.modelLayer, this.authoredLoadLayer, this.resultLayer, this.diagnosticLayer],
      this.hiddenKeys
    );
    this.invalidate();
  }

  markCameraProjectionChanged(): void {
    this.cameraChangeSequence = ++nextViewportCameraSequence;
    this.options.onCameraChange?.(this);
  }

  replaceLayer(layer: THREE.Group, objects: readonly THREE.Object3D[]): void {
    this.ownership.disposeObjects(layer.children);
    disposeObjectChildren(layer);
    layer.clear();
    layer.add(...objects);
    this.ownership.createObjects(objects);
    this.notifyResourceStateChange(false);
    this.invalidate();
  }

  pick(event: { clientX: number; clientY: number }): EntityRef | null {
    if (this.modelIndex) {
      return pickPointPrimitive(this.pointPrimitives, {
        index: this.modelIndex,
        renderOrigin: this.renderOrigin,
        camera: this.camera,
        canvas: this.renderer.domElement,
        clientX: event.clientX,
        clientY: event.clientY,
        hiddenKeys: this.hiddenKeys,
        actualOdRadiusByPipe: this.actualOdRadiusByPipe
      });
    }
    const pointer = eventNdc(this.renderer.domElement, event);
    this.raycaster.setFromCamera(pointer, this.camera);
    for (const hit of this.raycaster.intersectObjects(this.pickables, true)) {
      let object: THREE.Object3D | null = hit.object;
      while (object) {
        if (object.userData?.entityRef) return object.userData.entityRef as EntityRef;
        object = object.parent;
      }
    }
    return null;
  }

  authoredToLocal(authored: Readonly<Vec3>): Vec3 {
    return authoredToLocal(authored, this.renderOrigin);
  }

  localToAuthored(local: Readonly<Vec3>): Vec3 {
    return localToAuthored(local, this.renderOrigin);
  }

  projectToAuthoredPlane(
    event: { clientX: number; clientY: number },
    normal: Readonly<Vec3>,
    authoredConstant: number
  ): Vec3 | null {
    if (![normal.x, normal.y, normal.z, authoredConstant].every(Number.isFinite)) return null;
    const localConstant =
      normal.x * this.renderOrigin.x + normal.y * this.renderOrigin.y + normal.z * this.renderOrigin.z - authoredConstant;
    const plane = new THREE.Plane(new THREE.Vector3(normal.x, normal.y, normal.z), localConstant);
    this.raycaster.setFromCamera(eventNdc(this.renderer.domElement, event), this.camera);
    const intersection = new THREE.Vector3();
    if (!this.raycaster.ray.intersectPlane(plane, intersection)) return null;
    return this.localToAuthored(intersection);
  }

  invalidate = (): void => {
    this.scheduler.invalidate();
  };

  dispose(): void {
    if (this.disposed) return;
    this.disposed = true;
    this.scheduler.dispose();
    this.resizeObserver?.disconnect();
    window.removeEventListener("resize", this.resize);
    document.removeEventListener("visibilitychange", this.handleVisibilityChange);
    this.controls.removeEventListener("change", this.handleControlsChange);
    this.controls.removeEventListener("start", this.invalidate);
    this.controls.removeEventListener("end", this.invalidate);
    this.renderer.domElement.removeEventListener("webglcontextlost", this.handleContextLost);
    this.renderer.domElement.removeEventListener("webglcontextrestored", this.handleContextRestored);
    this.ownership.disposeObjects(this.scene.children);
    this.ownership.disposeObjects(this.gizmoScene.children);
    this.ownership.disposeLifecycle({
      controls: 1,
      eventBindings: 7,
      resizeObservers: this.resizeObserver ? 1 : 0
    });
    this.controls.dispose();
    disposeObjectChildren(this.scene);
    disposeObjectChildren(this.gizmoScene);
    this.renderer.renderLists.dispose();
    this.renderer.dispose();
    this.renderer.forceContextLoss();
    this.pickables = [];
    this.pointPrimitives = [];
    this.modelIndex = null;
    this.hiddenKeys = new Set();
    this.actualOdRadiusByPipe = new Map();
    this.labelUpdater = null;
    this.host.replaceChildren();
    this.notifyResourceStateChange(true);
  }

  private render(_time: number): boolean {
    if (this.disposed || this.contextStatus !== "ready") return false;
    const dampingActive = this.controls.update();
    this.renderer.setScissorTest(false);
    this.renderer.setViewport(0, 0, Math.max(1, this.host.clientWidth), Math.max(1, this.host.clientHeight));
    this.renderer.render(this.scene, this.camera);
    const submittedAt = performance.now();
    const submissionSequence = ++this.frameSubmissionSequence;
    const rendererInfo = copyRendererInfo(this.renderer.info);
    this.renderGizmo();
    this.labelUpdater?.();
    if (this.options.shouldObservePaintOpportunity?.()) {
      this.scheduler.schedulePaintOpportunity((opportunityAt) => {
        if (!this.disposed && this.contextStatus === "ready" && this.frameSubmissionSequence === submissionSequence) {
          this.options.onNextPaintOpportunity?.(this, opportunityAt, submissionSequence);
        }
      });
    }
    this.options.onAfterMainFrame?.(this, submittedAt, submissionSequence, rendererInfo);
    return dampingActive;
  }

  private renderGizmo(): void {
    const width = Math.max(1, this.host.clientWidth);
    const size = Math.min(96, Math.max(48, Math.floor(Math.min(width, this.host.clientHeight) * 0.18)));
    const offset = this.camera.position.clone().sub(this.controls.target);
    if (offset.lengthSq() === 0) offset.set(0, 0, 1);
    this.gizmoCamera.position.copy(offset.normalize().multiplyScalar(3.2));
    this.gizmoCamera.up.copy(this.camera.up);
    this.gizmoCamera.lookAt(0, 0, 0);
    this.renderer.clearDepth();
    this.renderer.setScissorTest(true);
    this.renderer.setScissor(8, 8, size, size);
    this.renderer.setViewport(8, 8, size, size);
    this.renderer.render(this.gizmoScene, this.gizmoCamera);
    this.renderer.setScissorTest(false);
    this.renderer.setViewport(0, 0, width, Math.max(1, this.host.clientHeight));
  }

  private resize = (): void => {
    if (this.disposed) return;
    const width = Math.max(1, this.host.clientWidth);
    const height = Math.max(1, this.host.clientHeight);
    this.camera.aspect = width / height;
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(width, height);
    this.markCameraProjectionChanged();
    this.invalidate();
  };

  private handleContextLost = (event: Event): void => {
    event.preventDefault();
    this.contextLostCount += 1;
    this.contextStatus = "lost";
    this.scheduler.pause();
    this.options.onContextStatus?.("lost");
  };

  private handleContextRestored = (): void => {
    this.contextRestoredCount += 1;
    this.contextStatus = "restoring";
    this.options.onContextStatus?.("restoring");
    this.options.onRestore?.();
    this.contextStatus = "ready";
    this.options.onContextStatus?.("ready");
    if (document.visibilityState === "hidden") this.scheduler.pause();
    else this.scheduler.resume();
  };

  private handleVisibilityChange = (): void => {
    if (document.visibilityState === "hidden") {
      this.scheduler.pause();
      return;
    }
    if (this.contextStatus === "ready") this.scheduler.resume();
  };

  private handleControlsChange = (): void => {
    this.markCameraProjectionChanged();
    this.invalidate();
  };

  private notifyResourceStateChange(retired: boolean): void {
    this.recordOwnedResourceSnapshot();
    if (this.options.shouldObservePaintOpportunity?.()) {
      this.options.onResourceStateChange?.(this, retired);
    }
  }

  private recordOwnedResourceSnapshot(): void {
    const snapshot = Object.freeze({
      ownedPendingRafCount: this.pendingAppOwnedRafCount,
      owned: this.ownershipSnapshot,
      context: this.contextSnapshot
    });
    if (!latestOwnedViewportResourceSnapshot ||
        snapshot.context.generation >= latestOwnedViewportResourceSnapshot.context.generation) {
      latestOwnedViewportResourceSnapshot = snapshot;
    }
  }
}

function safeAspect(host: HTMLElement): number {
  return Math.max(1, host.clientWidth) / Math.max(1, host.clientHeight);
}

function eventNdc(element: HTMLElement, event: { clientX: number; clientY: number }): THREE.Vector2 {
  const rect = element.getBoundingClientRect();
  const width = rect.width || element.clientWidth || 1;
  const height = rect.height || element.clientHeight || 1;
  const left = rect.width ? rect.left : 0;
  const top = rect.height ? rect.top : 0;
  return new THREE.Vector2(
    ((event.clientX - left) / width) * 2 - 1,
    -(((event.clientY - top) / height) * 2 - 1)
  );
}

export function disposeObjectChildren(root: THREE.Object3D): void {
  const geometries = new Set<THREE.BufferGeometry>();
  const materials = new Set<THREE.Material>();
  const textures = new Set<THREE.Texture>();
  const instances = new Set<THREE.InstancedMesh>();
  root.traverse((object) => {
    if (object instanceof THREE.InstancedMesh) instances.add(object);
    const renderable = object as THREE.Object3D & {
      geometry?: THREE.BufferGeometry;
      material?: THREE.Material | THREE.Material[];
    };
    if (renderable.geometry) geometries.add(renderable.geometry);
    const ownedMaterials = Array.isArray(renderable.material) ? renderable.material : renderable.material ? [renderable.material] : [];
    for (const material of ownedMaterials) {
      materials.add(material);
      for (const value of Object.values(material)) if (value instanceof THREE.Texture) textures.add(value);
    }
  });
  for (const instance of instances) instance.dispose();
  for (const texture of textures) texture.dispose();
  for (const geometry of geometries) geometry.dispose();
  for (const material of materials) material.dispose();
}

function objectMaterials(object: THREE.Object3D): THREE.Material[] {
  const material = (object as THREE.Object3D & { material?: THREE.Material | THREE.Material[] }).material;
  return Array.isArray(material) ? material : material ? [material] : [];
}

function applyInstancedSelection(mesh: THREE.InstancedMesh, selectedKeys: ReadonlySet<EntityKey>): void {
  const keys = mesh.userData.instanceEntityKeys as readonly EntityKey[];
  const baseColor = mesh.userData.viewportBaseColor as number;
  for (let index = 0; index < keys.length; index += 1) {
    mesh.setColorAt(index, new THREE.Color(selectedKeys.has(keys[index]) ? SELECTED_COLOR : baseColor));
  }
  if (mesh.instanceColor) mesh.instanceColor.needsUpdate = true;
}

function copyRendererInfo(info: THREE.WebGLInfo): ViewportRendererInfo {
  return Object.freeze({
    geometries: info.memory.geometries,
    textures: info.memory.textures,
    calls: info.render.calls,
    triangles: info.render.triangles,
    points: info.render.points,
    lines: info.render.lines
  });
}

type MutableOwnedCounts = { -readonly [K in keyof ViewportOwnedCounts]: number };

const OWNED_COUNT_KEYS: readonly (keyof ViewportOwnedCounts)[] = Object.freeze([
  "pipeMeshes",
  "nodeMeshes",
  "supportMeshes",
  "componentMeshes",
  "geometries",
  "materials",
  "textures",
  "instanceMatrices",
  "instanceColors",
  "controls",
  "eventBindings",
  "resizeObservers"
]);

function emptyOwnedCounts(): MutableOwnedCounts {
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

function addOwnedCounts(target: MutableOwnedCounts, source: Partial<ViewportOwnedCounts>): void {
  for (const key of OWNED_COUNT_KEYS) target[key] += source[key] ?? 0;
}

function countOwnedObjects(objects: readonly THREE.Object3D[]): MutableOwnedCounts {
  const counts = emptyOwnedCounts();
  const geometries = new Set<THREE.BufferGeometry>();
  const materials = new Set<THREE.Material>();
  const textures = new Set<THREE.Texture>();
  const instances = new Set<THREE.InstancedMesh>();
  const meshes = new Set<THREE.Object3D>();
  for (const root of objects) {
    root.traverse((object) => {
      if ((object instanceof THREE.Mesh || object instanceof THREE.Line || object instanceof THREE.Points) &&
          !meshes.has(object)) {
        meshes.add(object);
        const kind = object.userData.viewportOwnershipKind as "pipe" | "node" | "support" | "component" | undefined;
        if (kind === "pipe") counts.pipeMeshes += 1;
        if (kind === "node") counts.nodeMeshes += 1;
        if (kind === "support") counts.supportMeshes += 1;
        if (kind === "component") counts.componentMeshes += 1;
      }
      if (object instanceof THREE.InstancedMesh) instances.add(object);
      const renderable = object as THREE.Object3D & {
        geometry?: THREE.BufferGeometry;
        material?: THREE.Material | THREE.Material[];
      };
      if (renderable.geometry) geometries.add(renderable.geometry);
      for (const material of objectMaterials(object)) {
        materials.add(material);
        for (const value of Object.values(material)) if (value instanceof THREE.Texture) textures.add(value);
      }
    });
  }
  counts.geometries = geometries.size;
  counts.materials = materials.size;
  counts.textures = textures.size;
  counts.instanceMatrices = instances.size;
  counts.instanceColors = [...instances].filter((instance) => instance.instanceColor !== null).length;
  return counts;
}

function ownershipKindFromKey(key: EntityKey): "pipe" | "node" | "support" | "component" | null {
  try {
    const parsed: unknown = JSON.parse(key);
    if (!Array.isArray(parsed)) return null;
    const type = parsed[0];
    return type === "pipe" || type === "node" || type === "support" || type === "component" ? type : null;
  } catch {
    return null;
  }
}
