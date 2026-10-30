import { describe, expect, it, vi } from "vitest";
import type { PreviewModel, Vec3 } from "../../types";
import { buildModelIndex } from "../workspace/modelIndex";
import { entityKey } from "../workspace/selectionState";
import {
  authoredToLocal,
  fittedViewportDistance,
  applyMeasurementTargetCommand,
  broadPhaseEntityKeys,
  claimBoxSelectionPointerDown,
  boxGestureContextIsCurrent,
  composedVisibilityHiddenKeys,
  createRenderTransform,
  displayedBoundsForEntityKeys,
  effectiveHiddenEntityKeys,
  formatMeasurementDisplayValue,
  hideSelectionVisibility,
  isolateSelectionVisibility,
  localPointIsFloat32Representable,
  localToAuthored,
  pickPointPrimitive,
  pointPickPrimitives,
  prioritizedLabelKeys,
  spatialEntityKeyGroups,
  visibilityEligibleSelectionKeys
} from "./viewportSelection";
import * as THREE from "three";
import sharedEndpointFixture from "./viewportSelection.sharedEndpoint.fixture.json";

function largeModel(pipeCount = 2): PreviewModel {
  const nodes = Array.from({ length: pipeCount + 1 }, (_, index) => ({
    id: `n:${index}`,
    label: `Node ${index}`,
    position: { x: 9_000_000_000 + index * 4, y: -8_000_000_000, z: 7_000_000_000 },
    provenance: "test"
  }));
  return {
    schema_version: "test",
    document_kind: "test",
    data_boundary: {},
    project: { id: "p", name: "Large", description: "", units: { length: "m" } },
    analysis_status: { mechanics: "TBD", rule_check: "TBD", professional_acceptance: "TBD" },
    nodes,
    pipe_segments: Array.from({ length: pipeCount }, (_, index) => ({
      id: `pipe:${index}`,
      label: `Pipe ${index}`,
      from: nodes[index].id,
      to: nodes[index + 1].id,
      section: {},
      material: "m",
      provenance: "test"
    })),
    supports: [],
    components: [],
    load_cases: [],
    diagnostics: []
  };
}

describe("viewport coordinates and broad phase", () => {
  it("claims only an active primary left canvas pointer for Box Select", () => {
    const preventDefault = vi.fn();
    const stopPropagation = vi.fn();
    const event = { button: 0, isPrimary: true, canvasTarget: true, preventDefault, stopPropagation };

    expect(claimBoxSelectionPointerDown(event, { active: true, authoringActive: false })).toBe(true);
    expect(preventDefault).toHaveBeenCalledOnce();
    expect(stopPropagation).toHaveBeenCalledOnce();

    for (const rejected of [
      { ...event, button: 2 },
      { ...event, isPrimary: false },
      { ...event, canvasTarget: false }
    ]) {
      preventDefault.mockClear();
      stopPropagation.mockClear();
      expect(claimBoxSelectionPointerDown(rejected, { active: true, authoringActive: false })).toBe(false);
      expect(preventDefault).not.toHaveBeenCalled();
      expect(stopPropagation).not.toHaveBeenCalled();
    }
    expect(claimBoxSelectionPointerDown(event, { active: false, authoringActive: false })).toBe(false);
    expect(claimBoxSelectionPointerDown(event, { active: true, authoringActive: true })).toBe(false);
  });

  it("formats viewport measurements within the six-decimal publication allowance", () => {
    const ordinaryCases = [
      [0.0067800000000000002, "0.006780"],
      [0.10025736528056192, "0.100257"],
      [1234.123456789, "1234.123457"],
      [-9876.123456789, "-9876.123457"]
    ] as const;
    const publicationAllowance = 0.5e-6 + 1e-10;

    for (const [value, expected] of ordinaryCases) {
      const formatted = formatMeasurementDisplayValue(value);
      expect(formatted).toBe(expected);
      expect(Math.abs(Number(formatted) - value)).toBeLessThanOrEqual(publicationAllowance);
    }
    expect(formatMeasurementDisplayValue(-0)).toBe("0.000000");
    expect(formatMeasurementDisplayValue(-4.32198765e-9)).toBe("-4.321988e-9");
    expect(Math.abs(Number(formatMeasurementDisplayValue(-4.32198765e-9)) + 4.32198765e-9))
      .toBeLessThanOrEqual(publicationAllowance);
  });

  it("uses one typed measurement-target command for keyboard labels and canvas picks", () => {
    const index = buildModelIndex(largeModel(1), 1, 1);
    const first = applyMeasurementTargetCommand(index, null, { type: "node", id: "n:0" });
    const second = applyMeasurementTargetCommand(index, first.source, { type: "node", id: "n:1" });
    const pipe = applyMeasurementTargetCommand(index, null, { type: "pipe", id: "pipe:0" });

    expect(first).toMatchObject({ accepted: true, source: { kind: "nodes", keys: [entityKey({ type: "node", id: "n:0" })] } });
    expect(second).toMatchObject({
      accepted: true,
      source: { kind: "nodes", keys: [entityKey({ type: "node", id: "n:0" }), entityKey({ type: "node", id: "n:1" })] }
    });
    expect(pipe).toMatchObject({ accepted: true, source: { kind: "pipe", key: entityKey({ type: "pipe", id: "pipe:0" }) } });
    expect(applyMeasurementTargetCommand(index, second.source, { type: "support", id: "missing" })).toMatchObject({
      accepted: false,
      source: second.source
    });
  });

  it("centres industrial offsets and maps local picks back exactly", () => {
    const index = buildModelIndex(largeModel(), 4, 9);
    const transform = createRenderTransform(index.geometryBounds);
    const authored = { x: 9_000_000_004, y: -8_000_000_000, z: 7_000_000_000 };
    const local = authoredToLocal(authored, transform.origin);
    expect(localPointIsFloat32Representable(local)).toBe(true);
    expect(localToAuthored(local, transform.origin)).toEqual(authored);
  });

  it("rejects non-finite transform inputs", () => {
    expect(() => authoredToLocal({ x: Number.NaN, y: 0, z: 0 }, { x: 0, y: 0, z: 0 })).toThrow(
      "cannot be represented"
    );
  });

  it("keeps valid geometry renderable while excluding a finite Float32-range outlier", () => {
    const model = largeModel(1);
    model.nodes.push({ id: "outlier", label: "Outlier", position: { x: 1e308, y: 0, z: 0 }, provenance: "test" });
    const index = buildModelIndex(model, 4, 10);
    expect(index.invalidGeometry.get(entityKey({ type: "node", id: "outlier" }))).toContain("Float32");
    expect(index.geometryBounds?.max.x).toBeLessThan(1e12);
    expect(() => createRenderTransform(index.geometryBounds)).not.toThrow();
    expect(() => createRenderTransform({
      min: { x: -1e308, y: 0, z: 0 },
      max: { x: 1e308, y: 0, z: 0 }
    })).toThrow("Float32 presentation range");
  });

  it("uses chunk bounds before entity bounds", () => {
    const index = buildModelIndex(largeModel(300), 1, 1);
    const keys = broadPhaseEntityKeys(index, {
      min: { x: 9_000_000_000, y: -8_000_000_001, z: 6_999_999_999 },
      max: { x: 9_000_000_006, y: -7_999_999_999, z: 7_000_000_001 }
    });
    expect(keys).toContain(entityKey({ type: "pipe", id: "pipe:0" }));
    expect(keys).not.toContain(entityKey({ type: "pipe", id: "pipe:299" }));
  });

  it("partitions typed repeated geometry by stable spatial chunks", () => {
    const index = buildModelIndex(largeModel(300), 1, 1);
    const pipeGroups = spatialEntityKeyGroups(index, "pipe");
    expect(pipeGroups.length).toBeGreaterThan(1);
    expect(pipeGroups.every((group) => group.length <= 128)).toBe(true);
    expect(new Set(pipeGroups.flat()).size).toBe(300);
  });

  it("keeps the generic component render cube aligned with its analytic half diagonal", () => {
    const model = largeModel(0);
    model.nodes = [{ id: "anchor", label: "Anchor", position: { x: 0, y: 0, z: 0 }, provenance: "test" }];
    model.components = [{ id: "generic", label: "Generic", node: "anchor", kind: "generic", provenance: "test" }];
    const index = buildModelIndex(model, 1, 1);
    const primitive = pointPickPrimitives(index, model, { x: 0, y: 0, z: 0 })
      .find((candidate) => candidate.ref.type === "component");
    expect(primitive?.radius).toBeCloseTo(Math.hypot(0.12, 0.12, 0.12), 12);
  });

  it("shares one effective attachment mask while preserving explicit hide and bounded isolate context", () => {
    const model = largeModel(2);
    model.supports.push({ id: "support:0", label: "Support", node: "n:0", restraints: [], provenance: "test" });
    model.components.push({ id: "component:0", label: "Valve", node: "n:0", kind: "valve", provenance: "test" });
    const index = buildModelIndex(model, 2, 1);
    const nodeKey = entityKey({ type: "node", id: "n:0" });
    const supportKey = entityKey({ type: "support", id: "support:0" });
    const componentKey = entityKey({ type: "component", id: "component:0" });
    const adjacentPipe = entityKey({ type: "pipe", id: "pipe:0" });
    const unrelatedPipe = entityKey({ type: "pipe", id: "pipe:1" });

    const explicit = hideSelectionVisibility(new Set(), [nodeKey]);
    const effective = effectiveHiddenEntityKeys(index, explicit);
    expect(explicit).toEqual(new Set([nodeKey]));
    expect(effective).toEqual(new Set([nodeKey, supportKey, componentKey]));

    const isolate = isolateSelectionVisibility(index, [nodeKey]);
    expect(isolate.has(nodeKey)).toBe(false);
    expect(isolate.has(supportKey)).toBe(false);
    expect(isolate.has(componentKey)).toBe(false);
    expect(isolate.has(adjacentPipe)).toBe(true);
    expect(isolate.has(unrelatedPipe)).toBe(true);
  });

  it("composes explicit hide and isolate masks without expanding non-geometry selections", () => {
    const index = buildModelIndex(largeModel(2), 2, 1);
    const first = entityKey({ type: "pipe", id: "pipe:0" });
    const second = entityKey({ type: "pipe", id: "pipe:1" });
    const isolate = isolateSelectionVisibility(index, [first, second]);
    const composed = composedVisibilityHiddenKeys(index, new Set([first]), isolate);
    expect(composed.has(first)).toBe(true);
    expect(composed.has(second)).toBe(false);
    expect(visibilityEligibleSelectionKeys(index, [entityKey({ type: "project", id: "p" }), second])).toEqual([second]);
  });

  it("fits only visible selected rendered envelopes including large OD and marker-only entities", () => {
    const model = largeModel(1);
    model.supports.push({ id: "support:0", label: "Support", node: "n:0", restraints: [], provenance: "test" });
    model.components.push({ id: "component:0", label: "Component", node: "n:1", kind: "generic", provenance: "test" });
    const index = buildModelIndex(model, 1, 2);
    const pipeKey = entityKey({ type: "pipe", id: "pipe:0" });
    const supportKey = entityKey({ type: "support", id: "support:0" });
    const componentKey = entityKey({ type: "component", id: "component:0" });
    const pipeBounds = displayedBoundsForEntityKeys(index, [pipeKey], new Set(), new Map([[pipeKey, 8]]));
    expect(pipeBounds!.max.y - pipeBounds!.min.y).toBe(16);

    const visible = displayedBoundsForEntityKeys(
      index,
      [pipeKey, supportKey, componentKey],
      new Set([pipeKey, componentKey]),
      new Map([[pipeKey, 8]])
    );
    expect(visible).not.toBeNull();
    expect(visible!.max.x - visible!.min.x).toBeCloseTo(0.86);
    expect(displayedBoundsForEntityKeys(index, [pipeKey], new Set([pipeKey]), new Map([[pipeKey, 8]]))).toBeNull();
  });

  it("structurally caps labels with primary, hover, selected, and diagnostics first", () => {
    const index = buildModelIndex(largeModel(100), 1, 1);
    const primary = entityKey({ type: "pipe", id: "pipe:99" });
    const hover = entityKey({ type: "node", id: "n:98" });
    const selected = entityKey({ type: "pipe", id: "pipe:97" });
    const diagnostic = entityKey({ type: "node", id: "n:96" });
    const keys = prioritizedLabelKeys(index, {
      primaryKey: primary,
      hoverKey: hover,
      selectedKeys: [selected],
      diagnosticKeys: [diagnostic]
    });
    expect(keys).toHaveLength(80);
    expect(keys.slice(0, 4)).toEqual([primary, hover, selected, diagnostic]);
    expect(prioritizedLabelKeys(index, {
      primaryKey: primary,
      hoverKey: hover,
      selectedKeys: [selected],
      hiddenKeys: new Set([hover])
    })).not.toContain(hover);
  });

  it("uses physical axial near and far planes for an off-axis unit ray", () => {
    const camera = new THREE.PerspectiveCamera(60, 1, 1, 10);
    camera.position.set(0, 0, 0);
    camera.lookAt(0, 0, -1);
    camera.updateProjectionMatrix();
    camera.updateMatrixWorld();
    const canvas = document.createElement("canvas");
    Object.defineProperty(canvas, "clientWidth", { value: 100 });
    Object.defineProperty(canvas, "clientHeight", { value: 100 });
    canvas.getBoundingClientRect = () => ({
      x: 0, y: 0, left: 0, top: 0, right: 100, bottom: 100, width: 100, height: 100,
      toJSON: () => ({})
    });
    const ray = new THREE.Raycaster();
    ray.setFromCamera(new THREE.Vector2(0.8, 0), camera);
    const forward = camera.getWorldDirection(new THREE.Vector3());
    const cosTheta = ray.ray.direction.dot(forward);
    const tNear = camera.near / cosTheta;
    const tFar = camera.far / cosTheta;

    for (const [id, distance] of [["near", tNear + 0.02], ["far", tFar + 0.2]] as const) {
      const center = ray.ray.at(distance, new THREE.Vector3());
      const model = largeModel(0);
      model.nodes = [
        { id, label: id, position: { x: center.x, y: center.y, z: center.z }, provenance: "test" },
        { id: "mirror", label: "mirror", position: { x: -center.x, y: -center.y, z: -center.z }, provenance: "test" }
      ];
      const index = buildModelIndex(model, 1, 1);
      const transform = createRenderTransform(index.geometryBounds);
      const primitives = pointPickPrimitives(index, model, transform.origin).filter((primitive) => primitive.ref.id === id);
      expect(pickPointPrimitive(primitives, {
        index,
        renderOrigin: transform.origin,
        camera,
        canvas,
        clientX: 90,
        clientY: 50
      })).toEqual({ type: "node", id });
    }
  });

  it("expands broad-phase bounds by the zoomed-out six-pixel tolerance", () => {
    const model = largeModel(0);
    model.nodes = [
      { id: "offset", label: "offset", position: { x: 0.5, y: 0, z: -100 }, provenance: "test" },
      { id: "balance", label: "balance", position: { x: -0.5, y: 0, z: 100 }, provenance: "test" }
    ];
    const index = buildModelIndex(model, 1, 1);
    const transform = createRenderTransform(index.geometryBounds);
    const camera = new THREE.PerspectiveCamera(60, 1, 0.1, 200);
    camera.position.set(0, 0, 0);
    camera.lookAt(0, 0, -1);
    camera.updateProjectionMatrix();
    camera.updateMatrixWorld();
    const canvas = document.createElement("canvas");
    Object.defineProperty(canvas, "clientWidth", { value: 100 });
    Object.defineProperty(canvas, "clientHeight", { value: 100 });
    canvas.getBoundingClientRect = () => ({
      x: 0, y: 0, left: 0, top: 0, right: 100, bottom: 100, width: 100, height: 100,
      toJSON: () => ({})
    });
    const primitives = pointPickPrimitives(index, model, transform.origin).filter((primitive) => primitive.ref.id === "offset");
    expect(pickPointPrimitive(primitives, {
      index,
      renderOrigin: transform.origin,
      camera,
      canvas,
      clientX: 50,
      clientY: 50
    })).toEqual({ type: "node", id: "offset" });
  });

  it("keeps a large Actual-OD pipe eligible outside its centerline chunk bounds", () => {
    const model = largeModel(0);
    model.nodes = [
      { id: "a", label: "a", position: { x: 5, y: -1, z: -100 }, provenance: "test" },
      { id: "b", label: "b", position: { x: 5, y: 1, z: -100 }, provenance: "test" },
      { id: "balance", label: "balance", position: { x: -5, y: 0, z: 100 }, provenance: "test" }
    ];
    model.pipe_segments = [{ id: "large-od", label: "large", from: "a", to: "b", section: {}, material: "m", provenance: "test" }];
    const index = buildModelIndex(model, 1, 1);
    const transform = createRenderTransform(index.geometryBounds);
    const camera = new THREE.PerspectiveCamera(60, 1, 0.1, 200);
    camera.position.set(0, 0, 0);
    camera.lookAt(0, 0, -1);
    camera.updateProjectionMatrix();
    camera.updateMatrixWorld();
    const canvas = document.createElement("canvas");
    Object.defineProperty(canvas, "clientWidth", { value: 100 });
    Object.defineProperty(canvas, "clientHeight", { value: 100 });
    canvas.getBoundingClientRect = () => ({
      x: 0, y: 0, left: 0, top: 0, right: 100, bottom: 100, width: 100, height: 100,
      toJSON: () => ({})
    });
    const pipeKey = entityKey({ type: "pipe", id: "large-od" });
    expect(pickPointPrimitive(pointPickPrimitives(index, model, transform.origin), {
      index,
      renderOrigin: transform.origin,
      camera,
      canvas,
      clientX: 50,
      clientY: 50,
      actualOdRadiusByPipe: new Map([[pipeKey, 6]])
    })).toEqual({ type: "pipe", id: "large-od" });
  });

  it.each([
    [[1, 0, 1], [-1, 0, -3]],
    [[-1, 0, -3], [1, 0, 1]]
  ] as const)("uses the visible closest segment point for near-plane depth in either endpoint order", (from, to) => {
    const model = largeModel(0);
    model.nodes = [
      { id: "from", label: "From", position: { x: from[0], y: from[1], z: from[2] }, provenance: "test" },
      { id: "to", label: "To", position: { x: to[0], y: to[1], z: to[2] }, provenance: "test" }
    ];
    model.pipe_segments = [{ id: "crossing", label: "Crossing", from: "from", to: "to", section: {}, material: "m", provenance: "test" }];
    const index = buildModelIndex(model, 1, 1);
    const camera = new THREE.PerspectiveCamera(60, 1, 0.1, 100);
    camera.position.set(0, 0, 0);
    camera.lookAt(0, 0, -1);
    camera.updateProjectionMatrix();
    camera.updateMatrixWorld();
    const canvas = document.createElement("canvas");
    Object.defineProperty(canvas, "clientWidth", { value: 100 });
    Object.defineProperty(canvas, "clientHeight", { value: 100 });
    canvas.getBoundingClientRect = () => ({
      x: 0, y: 0, left: 0, top: 0, right: 100, bottom: 100, width: 100, height: 100,
      toJSON: () => ({})
    });

    expect(pickPointPrimitive(pointPickPrimitives(index, model, { x: 0, y: 0, z: 0 }), {
      index,
      renderOrigin: { x: 0, y: 0, z: 0 },
      camera,
      canvas,
      clientX: 50,
      clientY: 50
    })).toEqual({ type: "pipe", id: "crossing" });
  });
});

describe("envelope camera fitting", () => {
  it.each(["front", "top", "iso"] as const)("contains width, height and depth in %s at narrow and ordinary aspects", (preset) => {
    for (const aspect of [0.25, 0.5, 1, 2]) for (const fov of [30, 42, 65]) {
      const bounds = { min: { x: -20, y: -3, z: -11 }, max: { x: 20, y: 3, z: 11 } };
      const distance = fittedViewportDistance(bounds, preset, fov, aspect);
      const camera = new THREE.PerspectiveCamera(fov, aspect, 0.001, 10000);
      if (preset === "front") camera.position.set(0, 0, distance);
      else if (preset === "top") camera.position.set(0, distance, 0);
      else camera.position.setScalar(distance / Math.sqrt(3));
      camera.lookAt(0, 0, 0); camera.updateMatrixWorld();
      for (const x of [-20, 20]) for (const y of [-3, 3]) for (const z of [-11, 11]) {
        const point = new THREE.Vector3(x, y, z).project(camera);
        expect(Math.abs(point.x)).toBeLessThanOrEqual(0.900001);
        expect(Math.abs(point.y)).toBeLessThanOrEqual(0.900001);
        expect(Math.abs(point.z)).toBeLessThan(1);
      }
    }
  });
  it("preserves the legacy ordinary-aspect pose when it already fits", () => {
    const bounds = { min: { x: -10, y: -1, z: -1 }, max: { x: 10, y: 1, z: 1 } };
    for (const preset of ["front", "top", "iso"] as const) expect(fittedViewportDistance(bounds, preset, 42, 2)).toBe(36);
  });
});


describe("captured Box model/session admission", () => {
  it("accepts its exact context and rejects same-ID model, index and session replacements", () => {
    const model = largeModel();
    const context = { model, indexGeneration: "2:7", sessionGeneration: 2 };
    expect(boxGestureContextIsCurrent(context, { ...context })).toBe(true);
    expect(boxGestureContextIsCurrent(context, { ...context, model: structuredClone(model) })).toBe(false);
    expect(boxGestureContextIsCurrent(context, { ...context, indexGeneration: "2:8" })).toBe(false);
    expect(boxGestureContextIsCurrent(context, { ...context, sessionGeneration: 3 })).toBe(false);
  });
});

// Maintained reduced geometry and input records; no dependency on dated run artifacts.

function pickingCanvas(left: number, top: number, width: number, height: number): HTMLCanvasElement {
  const canvas = document.createElement("canvas");
  canvas.getBoundingClientRect = () => ({
    x: left, y: top, left, top, right: left + width, bottom: top + height, width, height,
    toJSON: () => ({})
  });
  return canvas;
}

describe("stable shared-endpoint picking", () => {
  const cases = sharedEndpointFixture.records.flatMap((record) =>
    (["before", "after"] as const).flatMap((phase) =>
      (["intended", "delivered", "xOnly", "yOnly"] as const).map((pointKind) => ({
        name: `attempt ${record.attempt} ${phase} ${pointKind}`,
        record,
        cameraRecord: record[phase],
        point: {
          x: pointKind === "delivered" || pointKind === "xOnly" ? record.delivered.x : record.intended.x,
          y: pointKind === "delivered" || pointKind === "yOnly" ? record.delivered.y : record.intended.y
        }
      }))
    )
  );

  it.each(cases)("keeps the analytic endpoint tie for $name", ({ record, cameraRecord, point }) => {
    const model = largeModel(0);
    model.nodes = structuredClone(sharedEndpointFixture.nodes);
    model.pipe_segments = structuredClone(sharedEndpointFixture.pipe_segments);
    const index = buildModelIndex(model, 1, 1);
    const [x, y, z] = cameraRecord.localRenderOrigin;
    const renderOrigin = { x, y, z };
    const camera = new THREE.PerspectiveCamera(cameraRecord.fovDegrees, cameraRecord.aspect, cameraRecord.near, cameraRecord.far);
    camera.position.fromArray(cameraRecord.position.map((value, axis) => value - cameraRecord.localRenderOrigin[axis]));
    camera.up.fromArray(cameraRecord.up);
    camera.lookAt(new THREE.Vector3().fromArray(cameraRecord.target.map((value, axis) => value - cameraRecord.localRenderOrigin[axis])));
    const canvas = pickingCanvas(record.canvas.cssLeft, record.canvas.cssTop, record.canvas.cssWidth, record.canvas.cssHeight);
    const primitives = pointPickPrimitives(index, model, renderOrigin);
    const options = { index, renderOrigin, camera, canvas, clientX: point.x, clientY: point.y };
    expect(pickPointPrimitive(primitives, options)).toEqual({ type: "node", id: "node:UIF-08786" });
    expect(pickPointPrimitive([...primitives].reverse(), options)).toEqual({ type: "node", id: "node:UIF-08786" });
  });
});


describe("analytic ray and capsule picking controls", () => {
  const frontZ = (miss: number) => 10 + Math.sqrt(1 - miss * miss);
  const controls = [
    { name: "endpoint tie", node: [0.6, 0, 10.8], a: [0.6, 0, 10.8], b: [2.6, 0, 10.8], winner: "node" },
    { name: "distinct pipe", node: [0.6, 0, 10.8], a: [0.2, 0, frontZ(0.2)], b: [2.2, 0, frontZ(0.2)], winner: "pipe" },
    { name: "distinct node", node: [0.2, 0, frontZ(0.2)], a: [0.6, 0, 10.8], b: [2.6, 0, 10.8], winner: "node" },
    { name: "interior pipe", node: [0.6, 0, 10.8], a: [-2, 0.2, frontZ(0.2)], b: [2, 0.2, frontZ(0.2)], winner: "pipe" },
    { name: "front entry precedes miss", node: [0, 0, 12], a: [0.6, 0, 10.8], b: [2.6, 0, 10.8], winner: "pipe" },
    { name: "near-distinct pipe", node: [0.600001, 0, frontZ(0.600001)], a: [0.6, 0, 10.8], b: [2.6, 0, 10.8], winner: "pipe" },
    { name: "near-distinct node", node: [0.6, 0, 10.8], a: [0.600001, 0, frontZ(0.600001)], b: [2.600001, 0, frontZ(0.600001)], winner: "node" },
    { name: "parallel axis", node: [0.6, 0, 10.8], a: [0.6, 0, 10.8], b: [0.6, 0, 12.8], winner: "node" },
    { name: "nearly parallel axis", node: [0.6, 0, 10.8], a: [0.6, 0, 10.8], b: [0.600000000001, 0, 12.8], winner: "node" },
    { name: "behind ray", node: [0.6, 0, -10.8], a: [0.6, 0, -10.8], b: [2.6, 0, -10.8], winner: null }
  ] as const;

  function pickControl(control: { node: readonly number[]; a: readonly number[]; b: readonly number[] },
    reverse: boolean, shift: Vec3 = { x: 0, y: 0, z: 0 }, origin: Vec3 = shift, zeroLength = false) {
    const translated = (point: readonly number[]): Vec3 => ({ x: point[0] + shift.x, y: point[1] + shift.y, z: point[2] + shift.z });
    const model = largeModel(0);
    model.nodes = [
      { id: "node", label: "Node", position: translated(control.node), provenance: "invented analytic geometry" },
      { id: "a", label: "A", position: translated(control.a), provenance: "invented analytic geometry" },
      { id: "b", label: "B", position: translated(control.b), provenance: "invented analytic geometry" }
    ];
    model.pipe_segments = [{ id: "pipe", label: "Pipe", from: reverse ? "b" : "a", to: reverse ? "a" : "b", section: {}, material: "test", provenance: "invented analytic geometry" }];
    const index = buildModelIndex(model, 1, 1);
    const camera = new THREE.PerspectiveCamera(42, 1, 0.001, 100);
    camera.position.set(shift.x - origin.x, shift.y - origin.y, shift.z - origin.z);
    camera.lookAt(camera.position.clone().add(new THREE.Vector3(0, 0, 1)));
    const primitives = pointPickPrimitives(index, model, origin)
      .filter((primitive) => primitive.ref.id === "node" || primitive.ref.id === "pipe")
      .map((primitive) => ({ ...primitive, radius: 1,
        // Exercise the degenerate capsule fallback without admitting invalid model geometry.
        ...(zeroLength && primitive.kind === "pipe" ? { start: authoredToLocal(translated(control.a), origin), end: authoredToLocal(translated(control.a), origin) } : {})
      }));
    return pickPointPrimitive(primitives, {
      index, renderOrigin: origin, camera, canvas: pickingCanvas(0, 0, 1000, 1000), clientX: 500, clientY: 500,
      actualOdRadiusByPipe: new Map([[entityKey({ type: "pipe", id: "pipe" }), 1]])
    });
  }

  it.each(controls)("preserves $name with either endpoint order", (control) => {
    for (const reverse of [false, true]) {
      expect(pickControl(control, reverse)).toEqual(control.winner ? { type: control.winner, id: control.winner } : null);
    }
  });

  it("preserves exact common translations and alternate local origins", () => {
    // Binary-exact geometry keeps authored coordinates representable after a large shift.
    const control = { node: [0.5, 0, 10], a: [0.5, 0, 10], b: [2.5, 0, 10] };
    const shift = { x: 2 ** 30, y: -(2 ** 29), z: 2 ** 28 };
    for (const reverse of [false, true]) {
      expect(pickControl(control, reverse, shift)).toEqual({ type: "node", id: "node" });
      expect(pickControl(control, reverse, shift, { x: shift.x - 16, y: shift.y + 8, z: shift.z - 4 })).toEqual({ type: "node", id: "node" });
      expect(pickControl(control, reverse)).toEqual({ type: "node", id: "node" });
    }
  });

  it("treats a zero-length capsule as its endpoint sphere", () => {
    expect(pickControl(controls[0], false, undefined, undefined, true)).toEqual({ type: "node", id: "node" });
  });

  it("preserves positive-depth eligibility after reoptimizing against the ray origin", () => {
    // The infinite line crosses this segment behind O. Its forward-ray minimum
    // lies in the segment interior at the origin projection, rather than at that crossing.
    const control = { node: [0.8, 0, 0.1], a: [-0.5, 0, -0.5], b: [0.5, 0, 0.1] };
    // Its closest point has negative camera depth, so the existing eligibility
    // rule excludes the capsule; the visible node remains selectable.
    for (const reverse of [false, true]) expect(pickControl(control, reverse)).toEqual({ type: "node", id: "node" });
  });

  it("clips signed intervals for shapes containing the ray origin", () => {
    const control = { node: [0.8, 0, 0.1], a: [-0.5, 0, 0.1], b: [0.5, 0, 0.1] };
    for (const reverse of [false, true]) expect(pickControl(control, reverse)).toEqual({ type: "pipe", id: "pipe" });
  });
});
