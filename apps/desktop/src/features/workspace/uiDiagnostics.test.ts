import { describe, expect, it, vi } from "vitest";
import {
  clearUiDiagnosticsPublisher,
  UI_DIAGNOSTICS_SCHEMA,
  publishUiDiagnostics,
  publishUiModelAssignmentStarted,
  refreshUiDiagnostics,
  type UiDiagnosticsPublication
} from "./uiDiagnostics";

const publication = (): UiDiagnosticsPublication => ({
  model: {
    projectId: "project:p",
    generation: 4,
    indexGeneration: "2:4",
    projectSessionGeneration: 2,
    identityHash: "sha256:test",
    assignment: { status: "committed", generation: 4, startedAt: 1, committedAt: 2 }
  },
  tree: { generation: 4, publicationSequence: 1, query: "", visibleCount: 3, publishedAt: 3 },
  viewport: {
    generation: 4,
    canvas: {
      cssLeft: 10, cssTop: 20, cssWidth: 200, cssHeight: 100,
      dpr: 2, bufferWidth: 400, bufferHeight: 200
    },
    camera: {
      sequence: 3,
      kind: "perspective",
      position: [1_000_000_000, 0, 10],
      target: [1_000_000_000, 0, 0],
      up: [0, 1, 0],
      fovDegrees: 42,
      near: 0.1,
      far: 1_000,
      aspect: 2,
      localRenderOrigin: [1_000_000_000, 0, 0]
    },
    mainRender: {
      submissionSequence: 7,
      generation: 4,
      reason: "model-assignment",
      submittedAt: 10,
      nextPaintOpportunity: { submissionSequence: 7, generation: 4, at: 11 }
    },
    selection: {
      actionSequence: 1,
      generation: 4,
      inputKind: "pointer",
      pointerDownAt: 8,
      orderedRefs: [{ type: "node", id: "n" }],
      primaryRef: { type: "node", id: "n" },
      publishedAt: 9,
      renderSubmissionSequence: 7
    },
    inspector: { generation: 4, ref: { type: "node", id: "n" }, publicationSequence: 1, publishedAt: 9 },
    filter: { actionSequence: 0, generation: 4, query: "", visibleCount: 3, inputAt: null, inputEventTimeStamp: null, publishedAt: 3, renderSubmissionSequence: 7 },
    box: {
      actionSequence: 0,
      generation: 4,
      direction: null,
      filter: "all",
      orderedRefs: [],
      primaryRef: null,
      publishedAt: null,
      renderSubmissionSequence: 0
    },
    labels: { enabled: true, renderedCount: 3, budget: 80 },
    geometry: { mode: "schematic", odGeneration: 0, odStatus: "not-requested" },
    resources: {
      rendererInfo: { geometries: 2, textures: 0, calls: 1, triangles: 4, points: 0, lines: 0 },
      ownedPendingRafCount: 0,
      owned: {
        generation: 1,
        live: ownedCounts({ geometries: 2, materials: 2, controls: 1, eventBindings: 7, resizeObservers: 1 }),
        created: ownedCounts({ geometries: 2, materials: 2, controls: 1, eventBindings: 7, resizeObservers: 1 }),
        disposed: ownedCounts()
      },
      context: { generation: 1, canvasConnected: true, lostCount: 0, restoredCount: 0 }
    }
  }
});

function ownedCounts(overrides: Partial<Record<string, number>> = {}) {
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
    resizeObservers: 0,
    ...overrides
  };
}

describe("UI diagnostics attachment", () => {
  it("installs the exact immutable global and materializes only on explicit pulls", () => {
    const factory = vi.fn(publication);
    const projectionFactory = vi.fn(() => ({
      modelGeneration: 4,
      cameraSequence: 3,
      renderOrigin: { x: 1_000_000_000, y: 0, z: 0 },
      viewProjectionMatrix: [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
      canvasCss: { left: 10, top: 20, width: 200, height: 100 },
      canvasDevice: { width: 400, height: 200 }
    }));
    publishUiDiagnostics(factory, projectionFactory);
    expect(factory).not.toHaveBeenCalled();
    expect(projectionFactory).not.toHaveBeenCalled();
    const descriptor = Object.getOwnPropertyDescriptor(globalThis, "__openPipeStressUiDiagnosticsV1");
    expect(descriptor).toMatchObject({ writable: false, configurable: false });
    expect(Object.keys(globalThis.__openPipeStressUiDiagnosticsV1).sort()).toEqual([
      "projectAuthoredPoint", "readCurrent", "schema"
    ]);
    expect(globalThis.__openPipeStressUiDiagnosticsV1.schema).toBe(UI_DIAGNOSTICS_SCHEMA);
    expect(Object.isFrozen(globalThis.__openPipeStressUiDiagnosticsV1)).toBe(true);
    const first = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    expect(factory).toHaveBeenCalledTimes(1);
    expect(projectionFactory).not.toHaveBeenCalled();
    expect(first.model.generation).toBe(4);
    expect(first.viewport).not.toEqual({ status: "unavailable" });
    const activeViewport = "status" in first.viewport ? null : first.viewport;
    expect(activeViewport).not.toBeNull();
    expect(Object.isFrozen(activeViewport!.selection)).toBe(true);
    expect(Object.isFrozen(activeViewport!.selection.orderedRefs)).toBe(true);
    expect(Object.isFrozen(activeViewport!.selection.orderedRefs[0])).toBe(true);
    publishUiDiagnostics(factory, projectionFactory);
    refreshUiDiagnostics();
    expect(factory).toHaveBeenCalledTimes(1);
    expect(projectionFactory).not.toHaveBeenCalled();
    const second = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    expect(factory).toHaveBeenCalledTimes(2);
    expect(second).not.toBe(first);
    expect(Object.isFrozen(second.viewport)).toBe(true);
    clearUiDiagnosticsPublisher();
  });

  it("returns a recursively frozen idle snapshot, then replaces one bounded current snapshot", () => {
    const idle = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    expect(idle.model.generation).toBeNull();
    expect(idle.model.assignment.status).toBe("idle");
    expect(Object.isFrozen(idle)).toBe(true);
    expect(Object.isFrozen(idle.model.assignment)).toBe(true);

    publishUiModelAssignmentStarted(4, 0.5);
    const assigning = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    expect(assigning.model.generation).toBeNull();
    expect(assigning.model.assignment).toEqual({
      status: "started",
      generation: 4,
      startedAt: 0.5,
      committedAt: null
    });
    expect(assigning.viewport).toEqual({ status: "unavailable" });

    publishUiDiagnostics(publication, () => ({
      modelGeneration: 4,
      cameraSequence: 3,
      renderOrigin: { x: 1_000_000_000, y: 0, z: 0 },
      viewProjectionMatrix: [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
      canvasCss: { left: 10, top: 20, width: 200, height: 100 },
      canvasDevice: { width: 400, height: 200 }
    }));
    const current = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    expect(current.model.generation).toBe(4);
    expect("status" in current.viewport ? current.viewport.status : "available").toBe("available");
    expect(Object.isFrozen(current.viewport)).toBe(true);
    expect(Object.values(current).some((value) => typeof value === "function")).toBe(false);
  });

  it("projects fresh exact large-offset literals without a preceding snapshot read", () => {
    clearUiDiagnosticsPublisher();
    const snapshotFactory = vi.fn(publication);
    const projectionFactory = vi.fn(() => ({
      modelGeneration: 4,
      cameraSequence: 3,
      renderOrigin: { x: 1_000_000_000, y: 0, z: 0 },
      viewProjectionMatrix: [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
      canvasCss: { left: 10, top: 20, width: 200, height: 100 },
      canvasDevice: { width: 400, height: 200 }
    }));
    publishUiDiagnostics(snapshotFactory, projectionFactory);
    const api = globalThis.__openPipeStressUiDiagnosticsV1;
    expect(api.projectAuthoredPoint({
      modelGeneration: 4,
      cameraSequence: 3,
      authoredPoint: { x: 1_000_000_000, y: 0, z: 0 }
    })).toEqual({
      status: "available",
      modelGeneration: 4,
      cameraSequence: 3,
      canvasCss: { width: 200, height: 100 },
      canvasDevice: { width: 400, height: 200 },
      localPoint: { x: 0, y: 0, z: 0 },
      clip: { x: 0, y: 0, z: 0, w: 1 },
      ndc: { x: 0, y: 0, z: 0 },
      canvasCssPoint: { x: 100, y: 50 },
      insideClosedNdc: true,
      insideCanvasCss: true
    });
    expect(snapshotFactory).not.toHaveBeenCalled();
    expect(projectionFactory).toHaveBeenCalledTimes(1);
    expect(api.projectAuthoredPoint({
      modelGeneration: 3,
      cameraSequence: 3,
      authoredPoint: { x: 0, y: 0, z: 0 }
    })).toEqual({
      status: "stale",
      requested: { modelGeneration: 3, cameraSequence: 3 },
      current: { modelGeneration: 4, cameraSequence: 3 }
    });
    expect(api.projectAuthoredPoint({
      modelGeneration: 4,
      cameraSequence: 2,
      authoredPoint: { x: 1_000_000_000, y: 0, z: 0 }
    })).toEqual({
      status: "stale",
      requested: { modelGeneration: 4, cameraSequence: 2 },
      current: { modelGeneration: 4, cameraSequence: 3 }
    });
    expect(api.projectAuthoredPoint({
      modelGeneration: 4,
      cameraSequence: 3,
      authoredPoint: { x: Number.NaN, y: 0, z: 0 }
    })).toEqual({ status: "invalid", reason: "NON_FINITE_AUTHORED_POINT" });
    clearUiDiagnosticsPublisher();
    expect(api.projectAuthoredPoint({
      modelGeneration: 4,
      cameraSequence: 3,
      authoredPoint: { x: 1_000_000_000, y: 0, z: 0 }
    })).toEqual({ status: "invalid", reason: "NO_CURRENT_MODEL" });
  });

  it("returns no-current-camera when a committed model loses projection context", () => {
    clearUiDiagnosticsPublisher();
    const projectionFactory = vi.fn(() => { throw new Error("context lost"); });
    publishUiDiagnostics(vi.fn(publication), projectionFactory);
    refreshUiDiagnostics(true, 4);
    expect(globalThis.__openPipeStressUiDiagnosticsV1.projectAuthoredPoint({
      modelGeneration: 4,
      cameraSequence: 3,
      authoredPoint: { x: 1_000_000_000, y: 0, z: 0 }
    })).toEqual({ status: "invalid", reason: "NO_CURRENT_CAMERA" });
  });
});
