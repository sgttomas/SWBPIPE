import { afterEach, describe, expect, it, vi } from "vitest";
import {
  UI_DIAGNOSTICS_VERSION,
  currentUiDiagnosticsSnapshot,
  installUiDiagnosticsObserver,
  projectAuthoredAnchor,
  publishUiDiagnostics,
  type UiDiagnosticsSnapshot
} from "./uiDiagnostics";

const snapshot = (): UiDiagnosticsSnapshot => ({
  version: UI_DIAGNOSTICS_VERSION,
  sessionGeneration: 2,
  modelGeneration: "2:4",
  assignmentStartedAt: 1,
  mainFrameSubmissionSequence: 3,
  mainFrameSubmittedAt: 10,
  nextPaintOpportunityAt: 11,
  selectionKeys: [JSON.stringify(["node", "n"]) as never],
  primaryKey: JSON.stringify(["node", "n"]) as never,
  boxSelectionMode: false,
  entityFilter: "all",
  camera: { position: [0, 0, 10], target: [0, 0, 0], up: [0, 1, 0] },
  renderOrigin: [1_000_000_000, 0, 0],
  rendererInfo: { geometries: 2, textures: 0, calls: 1, triangles: 4, points: 0, lines: 0 },
  pendingAppOwnedRafCount: 0
});

afterEach(() => installUiDiagnosticsObserver(null));

describe("UI diagnostics", () => {
  it("does not allocate or sample when no observer is installed", () => {
    const factory = vi.fn(snapshot);
    publishUiDiagnostics(factory);
    expect(factory).not.toHaveBeenCalled();
    expect(currentUiDiagnosticsSnapshot()).toBeNull();
  });

  it("replaces a frozen bounded current snapshot", () => {
    const observer = vi.fn();
    installUiDiagnosticsObserver(observer);
    publishUiDiagnostics(snapshot);
    const first = currentUiDiagnosticsSnapshot();
    publishUiDiagnostics(() => ({ ...snapshot(), mainFrameSubmissionSequence: 4 }));
    expect(observer).toHaveBeenCalledTimes(2);
    expect(currentUiDiagnosticsSnapshot()?.mainFrameSubmissionSequence).toBe(4);
    expect(Object.isFrozen(first)).toBe(true);
    expect(Object.isFrozen(first?.selectionKeys)).toBe(true);
    expect(Object.values(currentUiDiagnosticsSnapshot() ?? {}).some((value) => typeof value === "function")).toBe(false);
  });

  it("projects large authored anchors through a copied local-origin matrix and rejects stale generations", () => {
    installUiDiagnosticsObserver(vi.fn());
    publishUiDiagnostics(snapshot, () => ({
      generation: "2:4",
      renderOrigin: { x: 1_000_000_000, y: 0, z: 0 },
      viewProjectionMatrix: [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
      canvasRect: { left: 10, top: 20, width: 200, height: 100 }
    }));
    expect(projectAuthoredAnchor({ x: 1_000_000_000, y: 0, z: 0 }, "2:4")).toMatchObject({
      state: "available",
      cssX: 110,
      cssY: 70,
      inFrustum: true
    });
    expect(projectAuthoredAnchor({ x: 1_000_000_000, y: 0, z: 0 }, "older")).toMatchObject({
      state: "unavailable",
      reason: "stale_generation"
    });
  });
});
