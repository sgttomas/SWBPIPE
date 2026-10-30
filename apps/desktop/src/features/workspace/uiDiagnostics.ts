import * as THREE from "three";
import type { EntityRef, Vec3 } from "../../types";

export const UI_DIAGNOSTICS_SCHEMA = "openpipestress.ui-diagnostics/v1" as const;

type Unavailable = Readonly<{ status: "unavailable" }>;
type FrozenRef = Readonly<Pick<EntityRef, "type" | "id">>;

export type UiRendererInfo = Readonly<{
  geometries: number;
  textures: number;
  calls: number;
  triangles: number;
  points: number;
  lines: number;
}>;

export type UiOwnedResourceCounts = Readonly<{
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

export type UiDiagnosticsSnapshot = Readonly<{
  schema: typeof UI_DIAGNOSTICS_SCHEMA;
  snapshotSequence: number;
  capturedAt: number;
  model: Readonly<{
    projectId: string | null;
    generation: number | null;
    indexGeneration: string | null;
    projectSessionGeneration: number | null;
    identityHash: string | null;
    assignment: Readonly<{
      status: "idle" | "started" | "committed";
      generation: number | null;
      startedAt: number | null;
      committedAt: number | null;
    }>;
  }>;
  tree: Readonly<{
    generation: number;
    publicationSequence: number;
    query: string;
    visibleCount: number;
    publishedAt: number;
  }> | Unavailable;
  viewport: Readonly<{
    generation: number;
    canvas: Readonly<{
      cssLeft: number;
      cssTop: number;
      cssWidth: number;
      cssHeight: number;
      dpr: number;
      bufferWidth: number;
      bufferHeight: number;
    }>;
    camera: Readonly<{
      sequence: number;
      kind: "perspective";
      position: readonly [number, number, number];
      target: readonly [number, number, number];
      up: readonly [number, number, number];
      fovDegrees: number;
      near: number;
      far: number;
      aspect: number;
      localRenderOrigin: readonly [number, number, number];
    }>;
    mainRender: Readonly<{
      submissionSequence: number;
      generation: number;
      reason: string;
      submittedAt: number | null;
      selectionPresentation?: Readonly<{
        resourceGeneration: number;
        modelGeneration: number;
        revision: number;
        appliedAfterSubmissionSequence: number;
        renderedSubmissionSequence: number;
        orderedRefs: readonly FrozenRef[];
      }> | null;
      nextPaintOpportunity: Readonly<{
        submissionSequence: number;
        generation: number;
        at: number;
      }> | null;
    }>;
    selection: Readonly<{
      actionSequence: number;
      generation: number;
      inputKind: string;
      pointerDownAt: number | null;
      orderedRefs: readonly FrozenRef[];
      primaryRef: FrozenRef | null;
      publishedAt: number;
      renderSubmissionSequence: number;
    }>;
    inspector: Readonly<{
      generation: number;
      ref: FrozenRef | null;
      publicationSequence: number;
      publishedAt: number;
    }>;
    filter: Readonly<{
      actionSequence: number;
      generation: number;
      query: string;
      visibleCount: number;
      inputAt: number | null;
      inputEventTimeStamp: number | null;
      publishedAt: number;
      renderSubmissionSequence: number;
    }> | Unavailable;
    box: Readonly<{
      actionSequence: number;
      generation: number;
      direction: "left-to-right" | "right-to-left" | null;
      filter: string;
      orderedRefs: readonly FrozenRef[];
      primaryRef: FrozenRef | null;
      publishedAt: number | null;
      renderSubmissionSequence: number;
    }> | Unavailable;
    labels: Readonly<{ enabled: boolean; renderedCount: number; budget: number }>;
    geometry: Readonly<{ mode: "schematic" | "actual-od"; odGeneration: number; odStatus: string }>;
    resources: Readonly<{
      rendererInfo: UiRendererInfo;
      ownedPendingRafCount: number;
      owned: Readonly<{
        generation: number;
        live: UiOwnedResourceCounts;
        created: UiOwnedResourceCounts;
        disposed: UiOwnedResourceCounts;
      }>;
      context: Readonly<{
        generation: number;
        canvasConnected: boolean;
        lostCount: number;
        restoredCount: number;
      }>;
    }>;
  }> | Unavailable;
}>;

export type UiDiagnosticsPublication = Omit<UiDiagnosticsSnapshot, "schema" | "snapshotSequence" | "capturedAt">;

export type UiProjectionContext = Readonly<{
  modelGeneration: number;
  cameraSequence: number;
  renderOrigin: Readonly<Vec3>;
  viewProjectionMatrix: readonly number[];
  canvasCss: Readonly<{ left: number; top: number; width: number; height: number }>;
  canvasDevice: Readonly<{ width: number; height: number }>;
}>;

export type ProjectionRequest = Readonly<{
  modelGeneration: number;
  cameraSequence: number;
  authoredPoint: Readonly<Vec3>;
}>;

export type ProjectionResult =
  | Readonly<{
      status: "available";
      modelGeneration: number;
      cameraSequence: number;
      canvasCss: Readonly<{ width: number; height: number }>;
      canvasDevice: Readonly<{ width: number; height: number }>;
      localPoint: Readonly<Vec3>;
      clip: Readonly<{ x: number; y: number; z: number; w: number }>;
      ndc: Readonly<Vec3>;
      canvasCssPoint: Readonly<{ x: number; y: number }>;
      insideClosedNdc: boolean;
      insideCanvasCss: boolean;
    }>
  | Readonly<{
      status: "stale";
      requested: Readonly<{ modelGeneration: number; cameraSequence: number }>;
      current: Readonly<{ modelGeneration: number | null; cameraSequence: number | null }>;
    }>
  | Readonly<{
      status: "invalid";
      reason: "NON_FINITE_AUTHORED_POINT" | "NON_FINITE_PROJECTION" | "ZERO_CLIP_W" |
        "NO_CURRENT_MODEL" | "NO_CURRENT_CAMERA";
    }>;

export type UiDiagnosticsGlobal = Readonly<{
  schema: typeof UI_DIAGNOSTICS_SCHEMA;
  readCurrent(): UiDiagnosticsSnapshot;
  projectAuthoredPoint(request: ProjectionRequest): ProjectionResult;
}>;

let snapshotSequence = 0;
let currentSnapshot: UiDiagnosticsSnapshot = idleSnapshot();
let currentProjection: UiProjectionContext | null = null;
let latestSnapshotFactory: (() => UiDiagnosticsPublication) | null = null;
let latestProjectionFactory: (() => UiProjectionContext) | null = null;
let projectionAvailable = false;
let pendingAssignment: UiDiagnosticsSnapshot["model"]["assignment"] | null = null;
let lastAcceptedModelGeneration: number | null = null;

/** Publish assignment entry without exposing the pending model or changing projection. */
export function publishUiModelAssignmentStarted(generation: number, startedAt: number): void {
  pendingAssignment = deepFreeze({
    status: "started",
    generation,
    startedAt,
    committedAt: null
  });
}

/** Factories stay lazy until VERIFY first reads the fixed read-only global. */
export function publishUiDiagnostics(
  snapshotFactory: () => UiDiagnosticsPublication,
  projectionFactory?: () => UiProjectionContext
): void {
  latestSnapshotFactory = snapshotFactory;
  latestProjectionFactory = projectionFactory ?? null;
  projectionAvailable = Boolean(projectionFactory);
}

export function refreshUiDiagnostics(includeProjection = true, modelGeneration?: number | null): void {
  projectionAvailable = includeProjection;
  if (modelGeneration !== undefined) lastAcceptedModelGeneration = modelGeneration;
}

/** Clears only the current product-owned read publisher during viewport teardown. */
export function clearUiDiagnosticsPublisher(): void {
  latestSnapshotFactory = null;
  latestProjectionFactory = null;
  projectionAvailable = false;
  currentProjection = null;
  pendingAssignment = null;
  lastAcceptedModelGeneration = null;
  const idle = idleSnapshot();
  currentSnapshot = deepFreeze({ ...idle, snapshotSequence: ++snapshotSequence, capturedAt: performance.now() });
}

function materializeCurrentPublisher(): UiDiagnosticsSnapshot {
  const publication = latestSnapshotFactory?.();
  const basis = publication ?? currentSnapshot;
  if (pendingAssignment && publication?.model.assignment.status === "committed" &&
      publication.model.assignment.generation === pendingAssignment.generation) {
    pendingAssignment = null;
  }
  currentSnapshot = deepFreeze({
    ...basis,
    schema: UI_DIAGNOSTICS_SCHEMA,
    snapshotSequence: ++snapshotSequence,
    capturedAt: performance.now(),
    model: pendingAssignment
      ? { ...basis.model, assignment: pendingAssignment }
      : basis.model
  });
  lastAcceptedModelGeneration = currentSnapshot.model.generation;
  return currentSnapshot;
}

function readCurrent(): UiDiagnosticsSnapshot {
  // The factory builds an owned, recursively frozen snapshot only for this
  // explicit pull. No publication/render callback performs this work.
  return materializeCurrentPublisher();
}

function projectAuthoredPoint(request: ProjectionRequest): ProjectionResult {
  // Projection is its own explicit pull. It never pays to materialize the
  // larger snapshot and never depends on a preceding readCurrent() call.
  if (!projectionAvailable || !latestProjectionFactory) {
    return invalid(lastAcceptedModelGeneration === null ? "NO_CURRENT_MODEL" : "NO_CURRENT_CAMERA");
  }
  try {
    currentProjection = deepFreeze(latestProjectionFactory());
    lastAcceptedModelGeneration = currentProjection.modelGeneration;
  } catch {
    currentProjection = null;
  }
  const projection = currentProjection;
  if (!projection) {
    return invalid(lastAcceptedModelGeneration === null ? "NO_CURRENT_MODEL" : "NO_CURRENT_CAMERA");
  }
  if (request.modelGeneration !== projection.modelGeneration || request.cameraSequence !== projection.cameraSequence) {
    return deepFreeze({
      status: "stale",
      requested: { modelGeneration: request.modelGeneration, cameraSequence: request.cameraSequence },
      current: { modelGeneration: projection.modelGeneration, cameraSequence: projection.cameraSequence }
    });
  }
  if (!finiteVec(request.authoredPoint)) return invalid("NON_FINITE_AUTHORED_POINT");
  if (!finiteVec(projection.renderOrigin) || projection.viewProjectionMatrix.length !== 16 ||
      projection.viewProjectionMatrix.some((value) => !Number.isFinite(value)) ||
      !finiteCanvas(projection.canvasCss, projection.canvasDevice)) {
    return invalid("NON_FINITE_PROJECTION");
  }
  const localPoint = {
    x: request.authoredPoint.x - projection.renderOrigin.x,
    y: request.authoredPoint.y - projection.renderOrigin.y,
    z: request.authoredPoint.z - projection.renderOrigin.z
  };
  const clipVector = new THREE.Vector4(localPoint.x, localPoint.y, localPoint.z, 1)
    .applyMatrix4(new THREE.Matrix4().fromArray([...projection.viewProjectionMatrix]));
  if (![clipVector.x, clipVector.y, clipVector.z, clipVector.w].every(Number.isFinite)) {
    return invalid("NON_FINITE_PROJECTION");
  }
  if (clipVector.w === 0) return invalid("ZERO_CLIP_W");
  const ndc = { x: clipVector.x / clipVector.w, y: clipVector.y / clipVector.w, z: clipVector.z / clipVector.w };
  if (!finiteVec(ndc)) return invalid("NON_FINITE_PROJECTION");
  const canvasCssPoint = {
    x: (ndc.x * 0.5 + 0.5) * projection.canvasCss.width,
    y: (-ndc.y * 0.5 + 0.5) * projection.canvasCss.height
  };
  return deepFreeze({
    status: "available",
    modelGeneration: projection.modelGeneration,
    cameraSequence: projection.cameraSequence,
    canvasCss: { width: projection.canvasCss.width, height: projection.canvasCss.height },
    canvasDevice: { ...projection.canvasDevice },
    localPoint,
    clip: { x: clipVector.x, y: clipVector.y, z: clipVector.z, w: clipVector.w },
    ndc,
    canvasCssPoint,
    insideClosedNdc: ndc.x >= -1 && ndc.x <= 1 && ndc.y >= -1 && ndc.y <= 1 && ndc.z >= -1 && ndc.z <= 1,
    insideCanvasCss: canvasCssPoint.x >= 0 && canvasCssPoint.x <= projection.canvasCss.width &&
      canvasCssPoint.y >= 0 && canvasCssPoint.y <= projection.canvasCss.height
  });
}

function idleSnapshot(): UiDiagnosticsSnapshot {
  return deepFreeze({
    schema: UI_DIAGNOSTICS_SCHEMA,
    snapshotSequence: 0,
    capturedAt: performance.now(),
    model: {
      projectId: null,
      generation: null,
      indexGeneration: null,
      projectSessionGeneration: null,
      identityHash: null,
      assignment: { status: "idle", generation: null, startedAt: null, committedAt: null }
    },
    tree: { status: "unavailable" },
    viewport: { status: "unavailable" }
  });
}

function invalid(reason: Extract<ProjectionResult, { status: "invalid" }>["reason"]): ProjectionResult {
  return deepFreeze({ status: "invalid", reason });
}

function finiteVec(value: Readonly<Vec3>): boolean {
  return Number.isFinite(value.x) && Number.isFinite(value.y) && Number.isFinite(value.z);
}

function finiteCanvas(
  css: UiProjectionContext["canvasCss"],
  device: UiProjectionContext["canvasDevice"]
): boolean {
  return [css.left, css.top, css.width, css.height, device.width, device.height].every(Number.isFinite) &&
    css.width > 0 && css.height > 0 && device.width > 0 && device.height > 0;
}

function deepFreeze<T>(value: T): T {
  if (!value || (typeof value !== "object" && typeof value !== "function") || Object.isFrozen(value)) return value;
  for (const nested of Object.values(value as Record<string, unknown>)) deepFreeze(nested);
  return Object.freeze(value);
}

const diagnosticsGlobal: UiDiagnosticsGlobal = deepFreeze({
  schema: UI_DIAGNOSTICS_SCHEMA,
  readCurrent,
  projectAuthoredPoint
});

if (!Object.prototype.hasOwnProperty.call(globalThis, "__openPipeStressUiDiagnosticsV1")) {
  Object.defineProperty(globalThis, "__openPipeStressUiDiagnosticsV1", {
    value: diagnosticsGlobal,
    writable: false,
    configurable: false,
    enumerable: false
  });
}

declare global {
  // Test-only/internal read-only production surface frozen by OBSERVABILITY_ATTACHMENT_V1.
  var __openPipeStressUiDiagnosticsV1: UiDiagnosticsGlobal;
}
