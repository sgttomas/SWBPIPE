import * as THREE from "three";
import type { Vec3 } from "../../types";
import type { EntityKey } from "./selectionState";

export const UI_DIAGNOSTICS_VERSION = 1 as const;

export type UiDiagnosticsSnapshot = Readonly<{
  version: typeof UI_DIAGNOSTICS_VERSION;
  sessionGeneration: number;
  modelGeneration: string;
  assignmentStartedAt: number | null;
  mainFrameSubmissionSequence: number;
  mainFrameSubmittedAt: number | null;
  nextPaintOpportunityAt: number | null;
  selectionKeys: readonly EntityKey[];
  primaryKey: EntityKey | null;
  boxSelectionMode: boolean;
  entityFilter: "all" | "pipes" | "nodes" | "supports" | "components";
  camera: Readonly<{
    position: readonly [number, number, number];
    target: readonly [number, number, number];
    up: readonly [number, number, number];
  }>;
  renderOrigin: readonly [number, number, number];
  rendererInfo: Readonly<{
    geometries: number;
    textures: number;
    calls: number;
    triangles: number;
    points: number;
    lines: number;
  }>;
  pendingAppOwnedRafCount: number;
}>;

export type UiProjectionContext = Readonly<{
  generation: string;
  renderOrigin: Readonly<Vec3>;
  viewProjectionMatrix: readonly number[];
  canvasRect: Readonly<{ left: number; top: number; width: number; height: number }>;
}>;

export type UiProjectionResult =
  | Readonly<{
      state: "available";
      generation: string;
      cssX: number;
      cssY: number;
      inFrustum: boolean;
      renderOrigin: readonly [number, number, number];
    }>
  | Readonly<{ state: "unavailable"; generation: string | null; reason: "non_finite" | "stale_generation" | "no_projection" }>;

export type UiDiagnosticsObserver = (snapshot: UiDiagnosticsSnapshot) => void;

let observer: UiDiagnosticsObserver | null = null;
let currentSnapshot: UiDiagnosticsSnapshot | null = null;
let currentProjection: UiProjectionContext | null = null;

export function installUiDiagnosticsObserver(observerToInstall: UiDiagnosticsObserver | null): () => void {
  observer = observerToInstall;
  if (!observer) {
    currentSnapshot = null;
    currentProjection = null;
  }
  return () => {
    if (observer === observerToInstall) {
      observer = null;
      currentSnapshot = null;
      currentProjection = null;
    }
  };
}

export function hasUiDiagnosticsObserver(): boolean {
  return observer !== null;
}

/** The factory is deliberately lazy so normal product rendering allocates and samples nothing. */
export function publishUiDiagnostics(
  snapshotFactory: () => UiDiagnosticsSnapshot,
  projectionFactory?: () => UiProjectionContext
): void {
  if (!observer) return;
  const snapshot = freezeSnapshot(snapshotFactory());
  currentSnapshot = snapshot;
  currentProjection = projectionFactory ? freezeProjection(projectionFactory()) : null;
  observer(snapshot);
}

export function currentUiDiagnosticsSnapshot(): UiDiagnosticsSnapshot | null {
  return currentSnapshot;
}

export function projectAuthoredAnchor(
  authored: Readonly<Vec3>,
  expectedGeneration: string
): UiProjectionResult {
  const projection = currentProjection;
  if (!projection) return Object.freeze({ state: "unavailable", generation: null, reason: "no_projection" });
  if (projection.generation !== expectedGeneration) {
    return Object.freeze({ state: "unavailable", generation: projection.generation, reason: "stale_generation" });
  }
  if (!finiteVec(authored) || !finiteVec(projection.renderOrigin) ||
    projection.viewProjectionMatrix.length !== 16 ||
    projection.viewProjectionMatrix.some((value) => !Number.isFinite(value)) ||
    !finiteRect(projection.canvasRect)) {
    return Object.freeze({ state: "unavailable", generation: projection.generation, reason: "non_finite" });
  }

  const local = new THREE.Vector3(
    authored.x - projection.renderOrigin.x,
    authored.y - projection.renderOrigin.y,
    authored.z - projection.renderOrigin.z
  );
  const clip = local.applyMatrix4(new THREE.Matrix4().fromArray([...projection.viewProjectionMatrix]));
  if (![clip.x, clip.y, clip.z].every(Number.isFinite)) {
    return Object.freeze({ state: "unavailable", generation: projection.generation, reason: "non_finite" });
  }
  return Object.freeze({
    state: "available",
    generation: projection.generation,
    cssX: projection.canvasRect.left + (clip.x * 0.5 + 0.5) * projection.canvasRect.width,
    cssY: projection.canvasRect.top + (-clip.y * 0.5 + 0.5) * projection.canvasRect.height,
    inFrustum: clip.x >= -1 && clip.x <= 1 && clip.y >= -1 && clip.y <= 1 && clip.z >= -1 && clip.z <= 1,
    renderOrigin: freezeTuple(vecTuple(projection.renderOrigin))
  });
}

function freezeSnapshot(snapshot: UiDiagnosticsSnapshot): UiDiagnosticsSnapshot {
  return Object.freeze({
    ...snapshot,
    version: UI_DIAGNOSTICS_VERSION,
    selectionKeys: Object.freeze([...snapshot.selectionKeys]),
    camera: Object.freeze({
      position: freezeTuple(snapshot.camera.position),
      target: freezeTuple(snapshot.camera.target),
      up: freezeTuple(snapshot.camera.up)
    }),
    renderOrigin: freezeTuple(snapshot.renderOrigin),
    rendererInfo: Object.freeze({ ...snapshot.rendererInfo })
  });
}

function freezeProjection(projection: UiProjectionContext): UiProjectionContext {
  return Object.freeze({
    generation: projection.generation,
    renderOrigin: Object.freeze({ ...projection.renderOrigin }),
    viewProjectionMatrix: Object.freeze([...projection.viewProjectionMatrix]),
    canvasRect: Object.freeze({ ...projection.canvasRect })
  });
}

function vecTuple(value: Readonly<Vec3>): [number, number, number] {
  return [value.x, value.y, value.z];
}

function freezeTuple(value: readonly [number, number, number]): readonly [number, number, number] {
  return Object.freeze([value[0], value[1], value[2]]) as readonly [number, number, number];
}

function finiteVec(value: Readonly<Vec3>): boolean {
  return Number.isFinite(value.x) && Number.isFinite(value.y) && Number.isFinite(value.z);
}

function finiteRect(rect: UiProjectionContext["canvasRect"]): boolean {
  return Number.isFinite(rect.left) && Number.isFinite(rect.top) &&
    Number.isFinite(rect.width) && rect.width > 0 &&
    Number.isFinite(rect.height) && rect.height > 0;
}
