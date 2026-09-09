import { describe, expect, it } from "vitest";
import {
  applicableRoutingAxes,
  cancelPointerGesture,
  constrainRoutingPoint,
  convertedRoutingPoint,
  normalizeRoutingAxis,
  pointerGestureMayAuthor,
  resolveExistingRouteGhost,
  resolveNewRouteGhost,
  RoutingPlacementGate,
  routingPlaneDefinition,
  routingConversionRequests,
  startPointerGesture,
  updatePointerGesture
} from "./viewportRouting";

describe("viewport routing geometry", () => {
  const anchor = { x: 1.25, y: 2.4, z: -3.5 };

  it.each([
    ["XY", "z", -3.5, { x: 0, y: 0, z: 1 }],
    ["XZ", "y", 2.4, { x: 0, y: 1, z: 0 }],
    ["YZ", "x", 1.25, { x: 1, y: 0, z: 0 }]
  ] as const)("defines %s through the From anchor", (plane, fixedAxis, fixedValue, normal) => {
    expect(routingPlaneDefinition(plane, anchor)).toEqual({ anchor, fixedAxis, fixedValue, normal, plane });
  });

  it.each([
    ["XY", ["Free", "X", "Y"]],
    ["XZ", ["Free", "X", "Z"]],
    ["YZ", ["Free", "Y", "Z"]]
  ] as const)("limits %s constraints to its in-plane axes", (plane, axes) => {
    expect(applicableRoutingAxes(plane)).toEqual(axes);
  });

  it("projects to the plane before applying each axis constraint", () => {
    const point = { x: 9, y: 8, z: 7 };
    expect(constrainRoutingPoint(point, anchor, "XY", "Free")).toEqual({ x: 9, y: 8, z: -3.5 });
    expect(constrainRoutingPoint(point, anchor, "XY", "X")).toEqual({ x: 9, y: 2.4, z: -3.5 });
    expect(constrainRoutingPoint(point, anchor, "XY", "Y")).toEqual({ x: 1.25, y: 8, z: -3.5 });
    expect(constrainRoutingPoint(point, anchor, "XZ", "X")).toEqual({ x: 9, y: 2.4, z: -3.5 });
    expect(constrainRoutingPoint(point, anchor, "XZ", "Z")).toEqual({ x: 1.25, y: 2.4, z: 7 });
    expect(constrainRoutingPoint(point, anchor, "YZ", "Y")).toEqual({ x: 1.25, y: 8, z: -3.5 });
    expect(constrainRoutingPoint(point, anchor, "YZ", "Z")).toEqual({ x: 1.25, y: 2.4, z: 7 });
  });

  it("atomically resets an incompatible axis to Free", () => {
    expect(normalizeRoutingAxis("YZ", "X")).toBe("Free");
    expect(normalizeRoutingAxis("XY", "Z")).toBe("Free");
    expect(normalizeRoutingAxis("XZ", "X")).toBe("X");
  });
});

describe("viewport pointer gesture gate", () => {
  it("accepts exactly four CSS pixels and rejects any greater displacement", () => {
    const gesture = startPointerGesture(7, 10, 10);
    expect(pointerGestureMayAuthor(gesture, 7, 14, 10)).toBe(true);
    expect(pointerGestureMayAuthor(gesture, 7, 14.01, 10)).toBe(false);
    expect(updatePointerGesture(gesture, 7, 13, 13).cancelled).toBe(true);
  });

  it("rejects pointer-ID mismatch, cancellation, and movement that later returns", () => {
    const gesture = startPointerGesture(7, 10, 10);
    expect(pointerGestureMayAuthor(gesture, 8, 10, 10)).toBe(false);
    expect(pointerGestureMayAuthor(cancelPointerGesture(gesture), 7, 10, 10)).toBe(false);
    const moved = updatePointerGesture(gesture, 7, 15, 10);
    expect(pointerGestureMayAuthor(moved, 7, 10, 10)).toBe(false);
  });
});

describe("viewport routing conversion and transient ghosts", () => {
  it("rejects a delayed placement generation after manual or lifecycle invalidation", () => {
    const gate = new RoutingPlacementGate();
    const pending = gate.invalidate();
    expect(gate.isCurrent(pending)).toBe(true);
    gate.invalidate();
    expect(gate.isCurrent(pending)).toBe(false);
  });

  it("requests all three model axes in one mixed-unit engine batch", () => {
    expect(routingConversionRequests({ x: 3.2, y: 2.4, z: 0 }, "m", "mm")).toEqual([
      { id: "viewport-routing-x", value: 3.2, from_unit: "m", to_unit: "mm", dimension_id: "length" },
      { id: "viewport-routing-y", value: 2.4, from_unit: "m", to_unit: "mm", dimension_id: "length" },
      { id: "viewport-routing-z", value: 0, from_unit: "m", to_unit: "mm", dimension_id: "length" }
    ]);
  });

  it("accepts one finite, ID-matched result per axis in the requested unit", () => {
    expect(convertedRoutingPoint([
      { id: "viewport-routing-z", status: "converted", value: 0, unit: "mm" },
      { id: "viewport-routing-x", status: "converted", value: 3200, unit: "mm" },
      { id: "viewport-routing-y", status: "converted", value: 2400, unit: "mm" }
    ], "mm")).toEqual({ x: 3200, y: 2400, z: 0 });
  });

  it.each([
    [[{ id: "viewport-routing-x", status: "converted", value: 1, unit: "mm" }]],
    [[
      { id: "viewport-routing-x", status: "converted", value: 1, unit: "mm" },
      { id: "viewport-routing-x", status: "converted", value: 2, unit: "mm" },
      { id: "viewport-routing-z", status: "converted", value: 3, unit: "mm" }
    ]],
    [[
      { id: "viewport-routing-x", status: "converted", value: 1, unit: "m" },
      { id: "viewport-routing-y", status: "converted", value: 2, unit: "mm" },
      { id: "viewport-routing-z", status: "converted", value: 3, unit: "mm" }
    ]],
    [[
      { id: "viewport-routing-x", status: "converted", value: Number.NaN, unit: "mm" },
      { id: "viewport-routing-y", status: "converted", value: 2, unit: "mm" },
      { id: "viewport-routing-z", status: "converted", value: 3, unit: "mm" }
    ]],
    [[
      { id: null, status: "unavailable", message: "catalog unavailable" },
      { id: "viewport-routing-y", status: "converted", value: 2, unit: "mm" },
      { id: "viewport-routing-z", status: "converted", value: 3, unit: "mm" }
    ]]
  ] as const)("rejects malformed or unavailable engine conversion results %#", (results) => {
    expect(convertedRoutingPoint(results as never, "mm")).toBeNull();
  });

  it("shows existing ghosts only for two distinct resolved IDs", () => {
    const nodes = new Map([["node:A", { x: 0, y: 0, z: 0 }], ["node:B", { x: 1, y: 2, z: 3 }]]);
    expect(resolveExistingRouteGhost(nodes, "node:A", "node:B")?.provenance).toBe("existing");
    expect(resolveExistingRouteGhost(nodes, "node:A", "node:A")).toBeNull();
    expect(resolveExistingRouteGhost(nodes, "node:A", "node:missing")).toBeNull();
  });

  it("requires a resolved From and explicit pointer provenance for a new ghost", () => {
    const from = { x: 0, y: 0, z: 0 };
    const to = { x: 1, y: 2, z: 3 };
    expect(resolveNewRouteGhost(from, to, "captured")).toEqual({ from, to, provenance: "captured" });
    expect(resolveNewRouteGhost(null, to, "hover")).toBeNull();
    expect(resolveNewRouteGhost(from, to, null)).toBeNull();
  });
});
