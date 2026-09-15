import * as THREE from "three";
import { OrbitControls } from "three/addons/controls/OrbitControls.js";
import type { Vec3 } from "../../types";
import type { EntityRef } from "../../types";
import type { ModelIndex } from "../workspace/modelIndex";
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
  private paused = false;
  private disposed = false;

  constructor(
    private readonly renderFrame: (time: number) => boolean,
    private readonly requestFrame: FrameRequest = (callback) => window.requestAnimationFrame(callback),
    private readonly cancelFrame: FrameCancel = (handle) => window.cancelAnimationFrame(handle),
    private readonly onPendingChange: (count: number) => void = () => {}
  ) {}

  get pendingCount(): number {
    return this.pendingHandle === null ? 0 : 1;
  }

  invalidate(): void {
    if (this.disposed || this.paused || this.pendingHandle !== null) return;
    this.pendingHandle = this.requestFrame((time) => {
      this.pendingHandle = null;
      this.onPendingChange(0);
      if (this.disposed || this.paused) return;
      if (this.renderFrame(time)) this.invalidate();
    });
    this.onPendingChange(1);
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
    if (this.pendingHandle === null) return;
    this.cancelFrame(this.pendingHandle);
    this.pendingHandle = null;
    this.onPendingChange(0);
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

export type ViewportContextStatus = "ready" | "lost" | "restoring";

export type ViewportResourceOptions = {
  onContextStatus?: (status: ViewportContextStatus) => void;
  onAfterMainFrame?: (resource: ViewportResource, submittedAt: number) => void;
  onNextPaintOpportunity?: (resource: ViewportResource, opportunityAt: number) => void;
  shouldObservePaintOpportunity?: () => boolean;
  onRestore?: () => void;
};

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

  private readonly gizmoScene = new THREE.Scene();
  private readonly gizmoCamera = new THREE.PerspectiveCamera(45, 1, 0.1, 100);
  private readonly resizeObserver: ResizeObserver | null;
  private readonly raycaster = new THREE.Raycaster();
  private pickables: THREE.Object3D[] = [];
  private pointPrimitives: readonly PointPickPrimitive[] = [];
  private modelIndex: ModelIndex | null = null;
  private renderOrigin: Readonly<Vec3> = Object.freeze({ x: 0, y: 0, z: 0 });
  private labelUpdater: (() => void) | null = null;
  private disposed = false;
  private contextStatus: ViewportContextStatus = "ready";
  private frameSubmissionSequence = 0;

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

    this.scheduler = new ViewportInvalidationScheduler((time) => this.render(time));
    this.controls.addEventListener("change", this.invalidate);
    this.controls.addEventListener("start", this.invalidate);
    this.controls.addEventListener("end", this.invalidate);
    this.renderer.domElement.addEventListener("webglcontextlost", this.handleContextLost);
    this.renderer.domElement.addEventListener("webglcontextrestored", this.handleContextRestored);
    window.addEventListener("resize", this.resize);
    this.resizeObserver = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(this.resize);
    this.resizeObserver?.observe(host);
    this.resize();
    this.options.onContextStatus?.("ready");
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
    this.invalidate();
  }

  setPickables(pickables: readonly THREE.Object3D[]): void {
    this.pickables = [...pickables];
  }

  setPointPrimitives(index: ModelIndex, primitives: readonly PointPickPrimitive[]): void {
    this.modelIndex = index;
    this.pointPrimitives = primitives;
  }

  setLabelUpdater(updater: (() => void) | null): void {
    this.labelUpdater = updater;
  }

  replaceLayer(layer: THREE.Group, objects: readonly THREE.Object3D[]): void {
    disposeObjectChildren(layer);
    layer.clear();
    layer.add(...objects);
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
        clientY: event.clientY
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
    this.controls.removeEventListener("change", this.invalidate);
    this.controls.removeEventListener("start", this.invalidate);
    this.controls.removeEventListener("end", this.invalidate);
    this.renderer.domElement.removeEventListener("webglcontextlost", this.handleContextLost);
    this.renderer.domElement.removeEventListener("webglcontextrestored", this.handleContextRestored);
    this.controls.dispose();
    disposeObjectChildren(this.scene);
    disposeObjectChildren(this.gizmoScene);
    this.renderer.renderLists.dispose();
    this.renderer.dispose();
    this.renderer.forceContextLoss();
    this.pickables = [];
    this.pointPrimitives = [];
    this.modelIndex = null;
    this.labelUpdater = null;
    this.host.replaceChildren();
  }

  private render(time: number): boolean {
    if (this.disposed || this.contextStatus !== "ready") return false;
    const dampingActive = this.controls.update();
    this.renderer.setScissorTest(false);
    this.renderer.setViewport(0, 0, Math.max(1, this.host.clientWidth), Math.max(1, this.host.clientHeight));
    this.renderer.render(this.scene, this.camera);
    this.frameSubmissionSequence += 1;
    this.options.onAfterMainFrame?.(this, time);
    this.renderGizmo();
    this.labelUpdater?.();
    if (this.options.shouldObservePaintOpportunity?.()) {
      window.requestAnimationFrame((opportunityAt) => {
        if (!this.disposed && this.contextStatus === "ready") {
          this.options.onNextPaintOpportunity?.(this, opportunityAt);
        }
      });
    }
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
    this.invalidate();
  };

  private handleContextLost = (event: Event): void => {
    event.preventDefault();
    this.contextStatus = "lost";
    this.scheduler.pause();
    this.options.onContextStatus?.("lost");
  };

  private handleContextRestored = (): void => {
    this.contextStatus = "restoring";
    this.options.onContextStatus?.("restoring");
    this.options.onRestore?.();
    this.contextStatus = "ready";
    this.options.onContextStatus?.("ready");
    this.scheduler.resume();
  };
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

function disposeObjectChildren(root: THREE.Object3D): void {
  const geometries = new Set<THREE.BufferGeometry>();
  const materials = new Set<THREE.Material>();
  const textures = new Set<THREE.Texture>();
  root.traverse((object) => {
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
  for (const texture of textures) texture.dispose();
  for (const geometry of geometries) geometry.dispose();
  for (const material of materials) material.dispose();
}
