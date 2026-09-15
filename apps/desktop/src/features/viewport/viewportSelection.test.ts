import { describe, expect, it } from "vitest";
import type { PreviewModel } from "../../types";
import { buildModelIndex } from "../workspace/modelIndex";
import { entityKey } from "../workspace/selectionState";
import {
  authoredToLocal,
  broadPhaseEntityKeys,
  createRenderTransform,
  localPointIsFloat32Representable,
  localToAuthored,
  pickPointPrimitive,
  pointPickPrimitives,
  prioritizedLabelKeys
} from "./viewportSelection";
import * as THREE from "three";

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

  it("uses chunk bounds before entity bounds", () => {
    const index = buildModelIndex(largeModel(300), 1, 1);
    const keys = broadPhaseEntityKeys(index, {
      min: { x: 9_000_000_000, y: -8_000_000_001, z: 6_999_999_999 },
      max: { x: 9_000_000_006, y: -7_999_999_999, z: 7_000_000_001 }
    });
    expect(keys).toContain(entityKey({ type: "pipe", id: "pipe:0" }));
    expect(keys).not.toContain(entityKey({ type: "pipe", id: "pipe:299" }));
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
});
