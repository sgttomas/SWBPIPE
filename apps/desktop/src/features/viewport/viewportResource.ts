import { applyViewportDimming, prepareViewportDimming } from "./viewportDimmingPresentation";
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

import { ViewportSelectionPresentation } from "./viewportSelectionPresentation";
import { ViewportHaloPresentation } from "./viewportHalo";
import { isFigureMaterial, setFigureEdge, setFigureShadeRatio } from "./viewportFigureMaterial";
import {
  DEFAULT_VIEWPORT_PALETTE_THEME,
  isViewportPaletteRole,
  viewportRoleHex,
  viewportShadeRatio,
  viewportTokenColour,
  type ViewportPaletteRole
} from "./viewportPalette";

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

// The scene's ground, the selected colour and the selection cue's rim are design tokens like
// every other canvas colour: `canvas.bg`, `canvas.selection`, and `canvas.bg` again for the rim,
// which separates the cue from whatever it lies over. The selected colour paints the cue here and
// the halo in `viewportHalo.ts`; no element is recoloured by selection. The stylesheet's `--ui-canvas` and
// `--ui-viewport-selection-geometry` mirror the first two. No colour literal belongs in this folder.
function sceneBackground(theme: ViewportThemePresentation): number {
  return viewportTokenColour(theme, "canvas.bg").hex;
}

function selectedColour(theme: ViewportThemePresentation): number {
  return viewportTokenColour(theme, "canvas.selection").hex;
}

function selectionCueRim(theme: ViewportThemePresentation): number {
  return viewportTokenColour(theme, "canvas.bg").hex;
}

function gizmoThemePalette(theme: ViewportThemePresentation) {
  return Object.freeze({
    // The scene's ground.
    canvas: sceneBackground(theme),
    badge: viewportRoleHex(theme, "gizmoBadge"),
    axes: Object.freeze([
      viewportRoleHex(theme, "gizmoAxisX"),
      viewportRoleHex(theme, "gizmoAxisY"),
      viewportRoleHex(theme, "gizmoAxisZ")
    ] as const)
  });
}

export const GIZMO_THEME_PALETTES = Object.freeze({
  light: gizmoThemePalette("light"),
  dark: gizmoThemePalette("dark")
});

export const GIZMO_MAX_CSS_SIZE = 96;
export const GIZMO_CAMERA_DISTANCE = 5;
export const GIZMO_AXIS_LABEL_WORLD_SIZE = 0.72;
export const GIZMO_AXIS_LABEL_FONT_PX = 46;

export function relativeContrastRatio(first: number, second: number): number {
  const luminance = (hex: number) => {
    const channel = (shift: number) => {
      const value = ((hex >> shift) & 0xff) / 255;
      return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
    };
    return 0.2126 * channel(16) + 0.7152 * channel(8) + 0.0722 * channel(0);
  };
  const a = luminance(first);
  const b = luminance(second);
  return (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05);
}
export type ViewportRendererInfo = Readonly<{
  geometries: number;
  textures: number;
  calls: number;
  triangles: number;
  points: number;
  lines: number;
}>;

// Scratch colours for repaint: no colour object is allocated per instance.
const scratchBaseColor = new THREE.Color();
const scratchSecondColor = new THREE.Color();

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
  // Cache conservative full-geometry bounds before any visibility mask can
  // zero instance matrices. Restoring a distant chunk must not retain masked bounds.
  mesh.computeBoundingBox();
  mesh.computeBoundingSphere();
  const ownershipKind = keys[0] ? ownershipKindFromKey(keys[0]) : null;
  if (ownershipKind) mesh.userData.viewportOwnershipKind = ownershipKind;
  paintInstancedBase(mesh);
}

/**
 * Registers an instanced mesh whose base colour is a palette role. The role is kept on
 * `userData`, so the mesh's `viewportBaseColor` is rewritten from the palette whenever it is
 * painted for a theme. A mesh registered with `registerInstancedSelectionPresentation` alone
 * has no role and keeps the number it was given.
 */
export function registerInstancedRolePresentation(
  mesh: THREE.InstancedMesh,
  keys: readonly EntityKey[],
  role: ViewportPaletteRole,
  theme: ViewportThemePresentation = DEFAULT_VIEWPORT_PALETTE_THEME
): void {
  mesh.userData.viewportPaletteRole = role;
  registerInstancedSelectionPresentation(mesh, keys, viewportRoleHex(theme, role));
  paintFigureShade(mesh, theme);
}

/** Gives a line or a mesh that is not instanced a palette role, and paints it for a theme. */
export function registerPaletteRole(
  object: THREE.Object3D,
  role: ViewportPaletteRole,
  theme: ViewportThemePresentation = DEFAULT_VIEWPORT_PALETTE_THEME
): void {
  object.userData.viewportPaletteRole = role;
  paintObjectForTheme(object, theme);
}

/**
 * Gives a `GridHelper` its two palette roles, the centre lines and every other line, and
 * paints it for a theme. Painting rewrites the values of the helper's colour attribute; the
 * helper is never rebuilt.
 */
export function registerGridPaletteRoles(
  grid: THREE.GridHelper,
  centreLineRole: ViewportPaletteRole,
  lineRole: ViewportPaletteRole,
  theme: ViewportThemePresentation = DEFAULT_VIEWPORT_PALETTE_THEME
): void {
  grid.userData.viewportPaletteGridRoles = Object.freeze({ centreLine: centreLineRole, line: lineRole });
  paintObjectForTheme(grid, theme);
}

/**
 * Paints every role-coloured object under the roots for a theme, in place: instance base
 * colours, material tints, the figure materials' shade ratio and edge line, line-material
 * colours and the colour attribute of each grid helper. It creates and disposes nothing.
 * Instances are left at their base colour, which a selected instance keeps: selection is a halo
 * (`viewportHalo.ts`) and never a colour of the element. `edgeWidth` is the edge line's width in device pixels: the renderer's pixel ratio,
 * for a line of one CSS pixel.
 */
export function applyPalettePresentation(
  roots: readonly THREE.Object3D[],
  theme: ViewportThemePresentation,
  edgeWidth = 1
): void {
  for (const root of roots) root.traverse((object) => paintObjectForTheme(object, theme, edgeWidth));
}

/**
 * Returns every selectable object under the roots to its own colour: each instance to its mesh's
 * base colour, each registered material to its base colour and base emissive. A selected element
 * is painted exactly as an unselected one, because selection is drawn as a halo outside the
 * element's silhouette (`viewportHalo.ts`, design system 6.6) and the element keeps its colour.
 * The keys and the theme stay in the signature for the callers that pass them; neither changes a
 * colour.
 */
export function applySelectionPresentation(
  roots: readonly THREE.Object3D[],
  _selectedKeys: ReadonlySet<EntityKey>,
  _theme: ViewportThemePresentation = "dark"
): void {
  for (const root of roots) {
    root.traverse((object) => {
      if (object instanceof THREE.InstancedMesh && Array.isArray(object.userData.instanceEntityKeys)) {
        paintInstancedBase(object);
        return;
      }
      const key = object.userData.selectionEntityKey as EntityKey | undefined;
      if (!key) return;
      for (const material of objectMaterials(object)) {
        if ("color" in material && material.color instanceof THREE.Color &&
            typeof material.userData.viewportBaseColor === "number") {
          material.color.setHex(material.userData.viewportBaseColor);
        }
        if ("emissive" in material && material.emissive instanceof THREE.Color &&
            typeof material.userData.viewportBaseEmissive === "number") {
          material.emissive.setHex(material.userData.viewportBaseEmissive);
        }
        material.needsUpdate = true;
      }
    });
  }
}

export function applyThemePresentation(scene: THREE.Scene, theme: ViewportThemePresentation): void {
  if (scene.background instanceof THREE.Color) scene.background.setHex(sceneBackground(theme));
  else scene.background = new THREE.Color(sceneBackground(theme));
}

export function applyGizmoThemePresentation(scene: THREE.Scene, theme: ViewportThemePresentation): void {
  const palette = GIZMO_THEME_PALETTES[theme];
  scene.traverse((object) => {
    if (object.userData.gizmoAxes && object instanceof THREE.AxesHelper) {
      const colors = object.geometry.getAttribute("color") as THREE.BufferAttribute;
      palette.axes.forEach((hex, axis) => {
        const color = scratchBaseColor.setHex(hex);
        colors.setXYZ(axis * 2, color.r, color.g, color.b);
        colors.setXYZ(axis * 2 + 1, color.r, color.g, color.b);
      });
      colors.needsUpdate = true;
    }
    if (object.userData.gizmoAxisLabel && object instanceof THREE.Sprite) {
      paintGizmoAxisLabel(
        object.userData.gizmoCanvas as HTMLCanvasElement,
        object.userData.gizmoAxisLabel as "X" | "Y" | "Z",
        palette.axes[object.userData.gizmoAxisIndex as 0 | 1 | 2],
        palette.badge
      );
      const material = object.material as THREE.SpriteMaterial;
      if (material.map) material.map.needsUpdate = true;
      material.needsUpdate = true;
    }
  });
}

export function applyVisibilityPresentation(
  roots: readonly THREE.Object3D[],
  hiddenKeys: ReadonlySet<EntityKey>,
  dimmedKeys: ReadonlySet<EntityKey> = new Set()
): void {
  applyViewportDimming(roots, hiddenKeys, dimmedKeys);
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
  private selectionPresentation: ViewportSelectionPresentation | null = null;
  private modelIndex: ModelIndex | null = null;
  private hiddenKeys: ReadonlySet<EntityKey> = new Set();
  private dimmedKeys: ReadonlySet<EntityKey> = new Set();
  private selectedKeys: ReadonlySet<EntityKey> = new Set();
  private hoveredKey: EntityKey | null = null;
  // Selection and hover halos. Display-only: never given to either picking path. Absent only on a
  // resource that was not built by this constructor.
  private readonly halo: ViewportHaloPresentation | null;
  private themePresentation: ViewportThemePresentation = "light";
  private gridVisible = true;
  private authoredLoadsVisible = true;
  private actualOdRadiusByPipe: ReadonlyMap<EntityKey, number> = new Map();
  private renderOrigin: Readonly<Vec3> = Object.freeze({ x: 0, y: 0, z: 0 });
  private labelUpdater: (() => void) | null = null;
  private disposed = false;
  private contextStatus: ViewportContextStatus = "ready";
  private contextLostCount = 0;
  private contextRestoredCount = 0;
  private frameSubmissionSequence = 0;
  private cameraChangeSequence = ++nextViewportCameraSequence;
  private navigationAdvancing = false;
  private suppressControlsChange = false;
  private presentationBottomInsetPx = 0;
  private readonly ownership = new ViewportOwnershipLedger(this.resourceGeneration);
  // The edge line is one CSS pixel wide, which is this many device pixels. It is the renderer's
  // pixel ratio, read where that ratio is set, once, and never per frame.
  private readonly figureEdgeWidth: number;

  constructor(
    readonly host: HTMLDivElement,
    private readonly options: ViewportResourceOptions = {}
  ) {
    this.scene.background = new THREE.Color(sceneBackground(this.themePresentation));
    this.camera = new THREE.PerspectiveCamera(42, safeAspect(host), 0.1, 10_000);
    this.renderer = new THREE.WebGLRenderer({ antialias: true });
    this.renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    this.figureEdgeWidth = this.renderer.getPixelRatio();
    this.renderer.setSize(Math.max(1, host.clientWidth), Math.max(1, host.clientHeight));
    host.replaceChildren(this.renderer.domElement);

    this.controls = new OrbitControls(this.camera, this.renderer.domElement);
    this.controls.enableDamping = true;
    this.controls.dampingFactor = 0.08;
    this.scene.add(this.modelLayer, this.authoredLoadLayer, this.resultLayer, this.diagnosticLayer, this.routingLayer);
    // The halo accounts to the ledger for exactly what it creates, and disposes it itself: its
    // masks share the figure's geometries, which the layers own.
    this.halo = new ViewportHaloPresentation({
      created: (counts) => this.ownership.createLifecycle(counts),
      disposed: (counts) => this.ownership.disposeLifecycle(counts)
    });
    this.halo.setTheme(this.themePresentation);
    this.scene.add(this.halo.group);
    // No light: every material in the scene is unlit (the matte figure, the load arrows, the
    // lines, the selection cue), so a drawn colour is its token.
    const gizmoAxes = new THREE.AxesHelper(1.25);
    gizmoAxes.userData.gizmoAxes = true;
    this.gizmoScene.add(gizmoAxes);
    this.gizmoScene.add(
      gizmoAxisLabel("X", 0, new THREE.Vector3(1.48, 0, 0)),
      gizmoAxisLabel("Y", 1, new THREE.Vector3(0, 1.48, 0)),
      gizmoAxisLabel("Z", 2, new THREE.Vector3(0, 0, 1.48))
    );
    applyGizmoThemePresentation(this.gizmoScene, "light");
    this.ownership.createObjects(this.gizmoScene.children);

    this.scheduler = new ViewportInvalidationScheduler(
      (time) => this.render(time),
      undefined,
      undefined,
      (pendingCount) => this.options.onOwnedRafCountChange?.(this, pendingCount)
    );
    this.controls.addEventListener("change", this.handleControlsChange);
    this.controls.addEventListener("start", this.handleControlsStart);
    this.controls.addEventListener("end", this.handleControlsEnd);
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

  /** Moves only the painted orientation frame clear of shell furniture. */
  setPresentationBottomInsetPx(value: number): void {
    const next = Number.isFinite(value) ? Math.max(0, Math.round(value)) : 0;
    if (next === this.presentationBottomInsetPx) return;
    this.presentationBottomInsetPx = next;
    this.invalidate();
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

  get projectionAvailable(): boolean {
    return !this.disposed && this.contextStatus === "ready" && this.renderer.domElement.isConnected;
  }

  setOrigin(origin: Readonly<Vec3>): void {
    if (![origin.x, origin.y, origin.z].every(Number.isFinite)) {
      throw new Error("Viewport render origin must be finite.");
    }
    this.cancelNavigation();
    const localShift = new THREE.Vector3(
      this.renderOrigin.x - origin.x,
      this.renderOrigin.y - origin.y,
      this.renderOrigin.z - origin.z
    );
    this.camera.position.add(localShift);
    this.controls.target.add(localShift);
    this.renderOrigin = Object.freeze({ x: origin.x, y: origin.y, z: origin.z });
    // The next primitive inventory replaces this temporary origin translation.
    this.selectionPresentation?.group.position.add(localShift);
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
    if (this.selectionPresentation) {
      this.ownership.disposeObjects([this.selectionPresentation.group]);
      disposeObjectChildren(this.selectionPresentation.group);
      this.scene.remove(this.selectionPresentation.group);
    }
    this.selectionPresentation = new ViewportSelectionPresentation(primitives);
    this.scene.add(this.selectionPresentation.group);
    this.ownership.createObjects([this.selectionPresentation.group]);
    this.updateSelectionCue();
    this.notifyResourceStateChange(false);
    this.invalidate();
  }

  setActualOdRadiusByPipe(radii: ReadonlyMap<EntityKey, number>): void {
    this.actualOdRadiusByPipe = new Map(radii);
  }

  setLabelUpdater(updater: (() => void) | null): void {
    this.labelUpdater = updater;
  }

  setSelectionPresentation(selectedKeys: readonly EntityKey[]): void {
    this.selectedKeys = new Set(selectedKeys);
    applySelectionPresentation(
      [this.modelLayer, this.authoredLoadLayer, this.resultLayer, this.diagnosticLayer],
      this.selectedKeys,
      this.themePresentation
    );
    this.syncHalo();
    this.updateSelectionCue();
    this.invalidate();
  }

  /**
   * The hovered element, or none. Display-only. A change costs one scan for the key, one halo
   * instance and one invalidation; the same key again costs nothing. A selected or hidden element
   * shows no hover halo.
   */
  setHoverPresentation(hoveredKey: EntityKey | null): void {
    if (hoveredKey === this.hoveredKey) return;
    this.hoveredKey = hoveredKey;
    this.halo?.syncHover([this.modelLayer], this.selectedKeys, this.hiddenKeys, this.hoveredKey);
    this.invalidate();
  }

  setThemePresentation(theme: ViewportThemePresentation): void {
    this.themePresentation = theme;
    applyThemePresentation(this.scene, theme);
    applyGizmoThemePresentation(this.gizmoScene, theme);
    // All five layers: the routing layer holds role colours too, though never a selection.
    applyPalettePresentation(
      [this.modelLayer, this.authoredLoadLayer, this.resultLayer, this.diagnosticLayer, this.routingLayer],
      theme,
      this.figureEdgeWidth
    );
    applySelectionPresentation(
      [this.modelLayer, this.authoredLoadLayer, this.resultLayer, this.diagnosticLayer],
      this.selectedKeys,
      this.themePresentation
    );
    // In place: two colour uniforms, nothing created or disposed.
    this.halo?.setTheme(theme);
    this.updateSelectionCue();
    this.invalidate();
  }

  setAuxiliaryVisibility(gridVisible: boolean, authoredLoadsVisible: boolean): void {
    this.gridVisible = gridVisible;
    this.authoredLoadsVisible = authoredLoadsVisible;
    this.applyAuxiliaryVisibility();
    this.invalidate();
  }

  setVisibilityPresentation(hiddenKeys: ReadonlySet<EntityKey>, dimmedKeys: ReadonlySet<EntityKey> = new Set()): void {
    this.hiddenKeys = new Set(hiddenKeys);
    this.dimmedKeys = new Set(dimmedKeys);
    applyVisibilityPresentation(
      [this.modelLayer, this.authoredLoadLayer, this.resultLayer, this.diagnosticLayer],
      this.hiddenKeys,
      this.dimmedKeys
    );
    this.syncHalo();
    this.updateSelectionCue();
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
    // three reports an error when `add` is called with no object; an empty layer adds nothing.
    if (objects.length > 0) layer.add(...objects);
    prepareViewportDimming(objects);
    this.ownership.createObjects(objects);
    applyThemePresentation(this.scene, this.themePresentation);
    // The incoming objects were built without a theme; paint them for the current one so a
    // rebuilt layer is right without a theme change.
    applyPalettePresentation(objects, this.themePresentation, this.figureEdgeWidth);
    applySelectionPresentation(
      [this.modelLayer, this.authoredLoadLayer, this.resultLayer, this.diagnosticLayer],
      this.selectedKeys,
      this.themePresentation
    );
    applyVisibilityPresentation(
      [this.modelLayer, this.authoredLoadLayer, this.resultLayer, this.diagnosticLayer],
      this.hiddenKeys,
      this.dimmedKeys
    );
    this.applyAuxiliaryVisibility();
    // After the layer holds its new objects: halos of geometries that left with the old layer are
    // disposed, and the selected and hovered elements are haloed from the new ones.
    this.syncHalo();
    this.notifyResourceStateChange(false);
    this.invalidate();
  }

  /**
   * Halos follow the model layer alone: the elements the product draws and picks. A load arrow and
   * a deformed overlay carry their owner's key for visibility, and are not the element.
   */
  private syncHalo(): void {
    this.halo?.sync([this.modelLayer], this.selectedKeys, this.hiddenKeys, this.hoveredKey);
  }

  private updateSelectionCue(): void {
    this.selectionPresentation?.update(
      this.selectedKeys, this.hiddenKeys,
      selectedColour(this.themePresentation),
      selectionCueRim(this.themePresentation),
      this.renderer.getPixelRatio()
    );
  }

  private applyAuxiliaryVisibility(): void {
    const ground = this.modelLayer.getObjectByName("viewport-reference-ground");
    if (ground) ground.visible = this.gridVisible;
    this.authoredLoadLayer.visible = this.authoredLoadsVisible;
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

  /**
   * Clear OrbitControls' retained damping offsets without moving the visible
   * camera. OrbitControls has no public stop-inertia method, but disabling
   * damping for one supported update clears its pending rotation/pan state.
   * The exact public camera/target pose is restored before returning.
   */
  cancelNavigation(): void {
    if (this.disposed || !this.navigationAdvancing) return;
    // The local Three addon declaration intentionally exposes only the
    // controls surface used by the app. OrbitControls' runtime API includes
    // autoRotate, which must be suspended alongside damping while its retained
    // deltas are consumed.
    const navigationControls = this.controls as typeof this.controls & { autoRotate: boolean };
    const position = this.camera.position.clone();
    const quaternion = this.camera.quaternion.clone();
    const target = navigationControls.target.clone();
    const zoom = this.camera.zoom;
    const dampingEnabled = navigationControls.enableDamping;
    const autoRotateEnabled = navigationControls.autoRotate;
    this.suppressControlsChange = true;
    try {
      navigationControls.enableDamping = false;
      navigationControls.autoRotate = false;
      navigationControls.update();
    } finally {
      navigationControls.enableDamping = dampingEnabled;
      navigationControls.autoRotate = autoRotateEnabled;
      navigationControls.target.copy(target);
      this.camera.position.copy(position);
      this.camera.quaternion.copy(quaternion);
      this.camera.zoom = zoom;
      this.camera.updateProjectionMatrix();
      this.camera.updateMatrixWorld(true);
      this.navigationAdvancing = false;
      this.suppressControlsChange = false;
    }
  }

  dispose(): void {
    if (this.disposed) return;
    this.disposed = true;
    this.scheduler.dispose();
    this.resizeObserver?.disconnect();
    window.removeEventListener("resize", this.resize);
    document.removeEventListener("visibilitychange", this.handleVisibilityChange);
    this.controls.removeEventListener("change", this.handleControlsChange);
    this.controls.removeEventListener("start", this.handleControlsStart);
    this.controls.removeEventListener("end", this.handleControlsEnd);
    this.renderer.domElement.removeEventListener("webglcontextlost", this.handleContextLost);
    this.renderer.domElement.removeEventListener("webglcontextrestored", this.handleContextRestored);
    // The halo first: it leaves the scene, counts out what it created and disposes it, and never
    // touches the geometries it shares with the layers, which the walk below counts and disposes.
    this.halo?.dispose();
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
    this.selectionPresentation = null;
    this.hiddenKeys = new Set();
    this.dimmedKeys = new Set();
    this.actualOdRadiusByPipe = new Map();
    this.labelUpdater = null;
    this.host.replaceChildren();
    this.notifyResourceStateChange(true);
  }

  private render(_time: number): boolean {
    if (this.disposed || this.contextStatus !== "ready") return false;
    let navigationContinues = false;
    if (this.navigationAdvancing) {
      const position = this.camera.position.clone();
      const quaternion = this.camera.quaternion.clone();
      const target = this.controls.target.clone();
      const zoom = this.camera.zoom;
      this.suppressControlsChange = true;
      try {
        navigationContinues = this.controls.update();
      } finally {
        this.suppressControlsChange = false;
      }
      const cameraChanged = !position.equals(this.camera.position) ||
        !quaternion.equals(this.camera.quaternion) ||
        !target.equals(this.controls.target) ||
        zoom !== this.camera.zoom;
      if (cameraChanged) this.markCameraProjectionChanged();
      // OrbitControls retains a sub-EPS damping tail when update() returns
      // false. Clear it now so a later presentation-only frame cannot revive
      // the old gesture.
      if (!navigationContinues) this.cancelNavigation();
    }
    this.renderer.setScissorTest(false);
    this.renderer.setViewport(0, 0, Math.max(1, this.host.clientWidth), Math.max(1, this.host.clientHeight));
    this.renderer.render(this.scene, this.camera);
    const submittedAt = performance.now();
    const submissionSequence = ++this.frameSubmissionSequence;
    // Keep this bounded six-scalar record as ordinary render bookkeeping so a
    // later diagnostics read never has to cause another frame.
    const rendererInfo = copyRendererInfo(this.renderer.info);
    this.renderGizmo();
    this.labelUpdater?.();
    // Renderer diagnostics are captured in the actual main-render completion
    // path above. Presentation timing comes from the external compositor
    // witness; diagnostics must not schedule an additional app-owned RAF.
    this.options.onAfterMainFrame?.(this, submittedAt, submissionSequence, rendererInfo);
    return navigationContinues;
  }

  private renderGizmo(): void {
    const width = Math.max(1, this.host.clientWidth);
    const height = Math.max(1, this.host.clientHeight);
    const configuredBottomInset = Number.isFinite(this.presentationBottomInsetPx)
      ? Math.max(0, Math.round(this.presentationBottomInsetPx))
      : 0;
    const bottomInset = Math.min(configuredBottomInset, Math.max(0, height - 1));
    const availableHeight = Math.max(1, height - bottomInset);
    const size = Math.min(GIZMO_MAX_CSS_SIZE, Math.floor(Math.min(width, availableHeight)));
    const insetX = Math.min(8, Math.max(0, width - size));
    const insetY = bottomInset + Math.min(8, Math.max(0, availableHeight - size));
    const offset = this.camera.position.clone().sub(this.controls.target);
    if (offset.lengthSq() === 0) offset.set(0, 0, 1);
    // Fit the full enlarged axis-label sprite envelope inside the square
    // scissor across the accepted orbit envelope while projecting legible
    // XYZ glyphs.
    this.gizmoCamera.position.copy(offset.normalize().multiplyScalar(GIZMO_CAMERA_DISTANCE));
    this.gizmoCamera.up.copy(this.camera.up);
    this.gizmoCamera.lookAt(0, 0, 0);
    const previousViewport = this.renderer.getViewport(new THREE.Vector4());
    const previousScissor = this.renderer.getScissor(new THREE.Vector4());
    const previousScissorTest = this.renderer.getScissorTest();
    const previousAutoClear = this.renderer.autoClear;
    this.renderer.setScissorTest(true);
    this.renderer.setScissor(insetX, insetY, size, size);
    this.renderer.setViewport(insetX, insetY, size, size);
    this.renderer.clearDepth();
    this.renderer.autoClear = false;
    this.renderer.render(this.gizmoScene, this.gizmoCamera);
    this.renderer.autoClear = previousAutoClear;
    this.renderer.setScissor(previousScissor);
    this.renderer.setViewport(previousViewport);
    this.renderer.setScissorTest(previousScissorTest);
  }

  private resize = (): void => {
    if (this.disposed) return;
    const width = Math.max(1, this.host.clientWidth);
    const height = Math.max(1, this.host.clientHeight);
    this.camera.aspect = width / height;
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(width, height);
    // The halo's width and its pixel arithmetic: read here, where the renderer is sized, never per frame.
    this.halo?.setViewportFromRenderer(this.renderer);
    this.markCameraProjectionChanged();
    this.invalidate();
  };

  private handleContextLost = (event: Event): void => {
    event.preventDefault();
    this.contextLostCount += 1;
    this.contextStatus = "lost";
    this.cancelNavigation();
    this.scheduler.pause();
    this.options.onContextStatus?.("lost");
    this.notifyResourceStateChange(false);
  };

  private handleContextRestored = (): void => {
    this.contextRestoredCount += 1;
    this.contextStatus = "restoring";
    this.options.onContextStatus?.("restoring");
    this.options.onRestore?.();
    this.contextStatus = "ready";
    this.options.onContextStatus?.("ready");
    this.notifyResourceStateChange(false);
    if (document.visibilityState === "hidden") this.scheduler.pause();
    else this.scheduler.resume();
  };

  private handleVisibilityChange = (): void => {
    if (document.visibilityState === "hidden") {
      this.cancelNavigation();
      this.scheduler.pause();
      return;
    }
    if (this.contextStatus === "ready") this.scheduler.resume();
  };

  private handleControlsChange = (): void => {
    if (this.suppressControlsChange) return;
    this.navigationAdvancing = true;
    this.markCameraProjectionChanged();
    this.invalidate();
  };

  private handleControlsStart = (): void => {
    this.navigationAdvancing = true;
    this.invalidate();
  };

  private handleControlsEnd = (): void => {
    this.navigationAdvancing = true;
    this.invalidate();
  };

  private notifyResourceStateChange(retired: boolean): void {
    this.recordOwnedResourceSnapshot();
    this.options.onResourceStateChange?.(this, retired);
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

function gizmoAxisLabel(label: "X" | "Y" | "Z", axisIndex: 0 | 1 | 2, position: THREE.Vector3): THREE.Sprite {
  const canvas = document.createElement("canvas");
  canvas.width = 64;
  canvas.height = 64;
  const texture = new THREE.CanvasTexture(canvas);
  texture.colorSpace = THREE.SRGBColorSpace;
  texture.needsUpdate = true;
  const sprite = new THREE.Sprite(new THREE.SpriteMaterial({ map: texture, depthTest: false, transparent: true }));
  sprite.userData.gizmoAxisLabel = label;
  sprite.userData.gizmoAxisIndex = axisIndex;
  sprite.userData.gizmoCanvas = canvas;
  sprite.position.copy(position);
  sprite.scale.setScalar(GIZMO_AXIS_LABEL_WORLD_SIZE);
  return sprite;
}

function paintGizmoAxisLabel(canvas: HTMLCanvasElement, label: "X" | "Y" | "Z", color: number, badge: number): void {
  const context = canvas.getContext("2d");
  if (context) {
    context.clearRect(0, 0, 64, 64);
    context.fillStyle = `#${badge.toString(16).padStart(6, "0")}`;
    context.beginPath();
    context.arc(32, 32, 28, 0, Math.PI * 2);
    context.fill();
    context.fillStyle = `#${color.toString(16).padStart(6, "0")}`;
    context.font = `700 ${GIZMO_AXIS_LABEL_FONT_PX}px sans-serif`;
    context.textAlign = "center";
    context.textBaseline = "middle";
    context.fillText(label, 32, 34);
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

/** Every instance at its mesh's base colour. Selection does not enter: it is a halo. */
function paintInstancedBase(mesh: THREE.InstancedMesh): void {
  const keys = mesh.userData.instanceEntityKeys as readonly EntityKey[];
  const base = scratchBaseColor.setHex(mesh.userData.viewportBaseColor as number);
  for (let index = 0; index < keys.length; index += 1) mesh.setColorAt(index, base);
  if (mesh.instanceColor) mesh.instanceColor.needsUpdate = true;
}

type GridPaletteRoles = Readonly<{ centreLine: ViewportPaletteRole; line: ViewportPaletteRole }>;

/** The figure's own theme values: the shade ratio, and the edge line where the material draws one. */
function paintFigureShade(object: THREE.Object3D, theme: ViewportThemePresentation, edgeWidth = 1): void {
  for (const material of objectMaterials(object)) {
    if (!isFigureMaterial(material)) continue;
    setFigureShadeRatio(material, viewportShadeRatio(theme));
    setFigureEdge(material, viewportTokenColour(theme, "canvas.edge").hex, edgeWidth);
  }
}

function paintObjectForTheme(object: THREE.Object3D, theme: ViewportThemePresentation, edgeWidth = 1): void {
  paintFigureShade(object, theme, edgeWidth);
  const gridRoles = object.userData.viewportPaletteGridRoles as GridPaletteRoles | undefined;
  if (gridRoles && object instanceof THREE.LineSegments) paintGridColors(object, gridRoles, theme);
  const role: unknown = object.userData.viewportPaletteRole;
  if (!isViewportPaletteRole(role)) return;
  const hex = viewportRoleHex(theme, role);
  if (object instanceof THREE.InstancedMesh) {
    object.userData.viewportBaseColor = hex;
    const base = scratchBaseColor.setHex(hex);
    for (let index = 0; index < object.count; index += 1) object.setColorAt(index, base);
    if (object.instanceColor) object.instanceColor.needsUpdate = true;
    return;
  }
  for (const material of objectMaterials(object)) {
    if (!("color" in material) || !(material.color instanceof THREE.Color)) continue;
    material.color.setHex(hex);
    // A material registered for selection returns to its base colour when deselected.
    if (typeof material.userData.viewportBaseColor === "number") material.userData.viewportBaseColor = hex;
  }
}

/**
 * Rewrites a grid helper's colour attribute in three's own layout: four vertices per line
 * index, the centre index (when the division count is even) in the first colour and every
 * other index in the second.
 */
function paintGridColors(grid: THREE.LineSegments, roles: GridPaletteRoles, theme: ViewportThemePresentation): void {
  const colors = grid.geometry.getAttribute("color") as THREE.BufferAttribute | undefined;
  if (!colors) return;
  const lineIndices = colors.count / 4;
  const centreIndex = (lineIndices - 1) / 2;
  const centre = scratchBaseColor.setHex(viewportRoleHex(theme, roles.centreLine));
  const line = scratchSecondColor.setHex(viewportRoleHex(theme, roles.line));
  for (let index = 0; index < lineIndices; index += 1) {
    const color = index === centreIndex ? centre : line;
    for (let vertex = index * 4; vertex < index * 4 + 4; vertex += 1) colors.setXYZ(vertex, color.r, color.g, color.b);
  }
  colors.needsUpdate = true;
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
