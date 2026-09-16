import { describe, expect, it, vi } from "vitest";
import * as THREE from "three";
import { entityKey } from "../workspace/selectionState";
import type { PointPickPrimitive } from "./viewportSelection";
import { ViewportSelectionPresentation } from "./viewportSelectionPresentation";
import { disposeObjectChildren, relativeContrastRatio, ViewportOwnershipLedger, ViewportResource } from "./viewportResource";

const key = (id: string) => entityKey({ type: "pipe", id });
const primitive = (id: string, x = 0): PointPickPrimitive => ({
  key: key(id), ref: { type: "pipe", id }, kind: "pipe", shape: "capsule",
  center: { x: x + 0.05, y: 2, z: 3 },
  start: { x, y: 2, z: 3 }, end: { x: x + 0.1, y: 2, z: 3 },
  radius: 0.052, height: 0, displayOrder: 0
});

function update(cue: ViewportSelectionPresentation, ids: string[], hidden: string[] = []) {
  cue.update(new Set(ids.map(key)), new Set(hidden.map(key)), 0xf08c22, 0x0c1114, 2);
}

describe("selected authored primitive cue", () => {
  it("anchors a concealed short pipe to its actual local endpoints and midpoint without changing the primitive", () => {
    // Diagnosed authored endpoints, rebased around a nonzero render origin.
    const origin = { x: 6, y: 0.2, z: -0.01 };
    const start = { x: 6.3 - origin.x, y: 0.279451 - origin.y, z: -0.003159 - origin.z };
    const end = { x: 6.4 - origin.x, y: 0.282949 - origin.y, z: -0.005347 - origin.z };
    const selected: PointPickPrimitive = {
      ...primitive("UIF-00266"), start, end,
      center: { x: (start.x + end.x) / 2, y: (start.y + end.y) / 2, z: (start.z + end.z) / 2 }
    };
    const before = JSON.stringify(selected);
    const cue = new ViewportSelectionPresentation([primitive("other", 20), selected]);
    update(cue, ["UIF-00266"]);
    const spans = cue.spans.geometry.getAttribute("position");
    const markers = cue.markers.geometry.getAttribute("position");
    expect(cue.spans.geometry.drawRange.count).toBe(2);
    for (const [index, point] of [start, end].entries()) {
      expect(spans.getX(index)).toBeCloseTo(point.x, 7);
      expect(spans.getY(index)).toBeCloseTo(point.y, 7);
      expect(spans.getZ(index)).toBeCloseTo(point.z, 7);
    }
    expect(markers.getX(0)).toBeCloseTo(0.35, 7);
    expect(markers.getY(0)).toBeCloseTo(0.0812, 7);
    expect(markers.getZ(0)).toBeCloseTo(0.005747, 7);
    expect(JSON.stringify(selected)).toBe(before);
    for (const object of [cue.spans, cue.markers]) {
      expect(object.material.depthTest).toBe(false);
      expect(object.material.depthWrite).toBe(false);
      expect(object.material.transparent).toBe(true);
      expect(object.renderOrder).toBeGreaterThan(3);
      expect(object.userData.entityRef).toBeUndefined();
    }
    disposeObjectChildren(cue.group);
  });

  it("keeps explicit hidden masks authoritative and clears stale/deselected markers", () => {
    const cue = new ViewportSelectionPresentation([primitive("a"), primitive("b", 1)]);
    cue.update(new Set([key("a")]), new Set([entityKey({ type: "node", id: "endpoint" })]), 0xf08c22, 0x0c1114, 1);
    expect(cue.spans.geometry.drawRange.count).toBe(2); // Node masks do not hide an independently visible span.
    update(cue, ["a", "b"], ["a"]);
    expect(cue.markers.geometry.drawRange.count).toBe(1);
    expect(cue.markers.geometry.getAttribute("position").getX(0)).toBeCloseTo(1.05);
    update(cue, ["a", "b"], ["a", "b"]);
    expect(cue.group.visible).toBe(false);
    expect(cue.spans.geometry.drawRange.count).toBe(0);
    update(cue, ["missing"]);
    expect(cue.markers.geometry.drawRange.count).toBe(0);
    update(cue, ["b"]);
    update(cue, []);
    expect(cue.group.visible).toBe(false);
    disposeObjectChildren(cue.group);
  });

  it("retains bounded reusable buffers for all 10,000 pipes and accounts disposal once", () => {
    const primitives = Array.from({ length: 10_000 }, (_, index) => primitive(String(index), index));
    const cue = new ViewportSelectionPresentation(primitives);
    const ledger = new ViewportOwnershipLedger(1);
    ledger.createObjects([cue.group]);
    const spans = cue.spans.geometry.getAttribute("position");
    const markers = cue.markers.geometry.getAttribute("position");
    const disposals = [cue.spans.geometry, cue.markers.geometry, cue.spans.material, cue.markers.material].map((resource) => vi.spyOn(resource, "dispose"));
    update(cue, primitives.map((entry) => entry.ref.id));
    expect(cue.spans.geometry.drawRange.count).toBe(20_000);
    expect(cue.markers.geometry.drawRange.count).toBe(10_000);
    update(cue, ["9"]);
    update(cue, []);
    expect(cue.spans.geometry.getAttribute("position")).toBe(spans);
    expect(cue.markers.geometry.getAttribute("position")).toBe(markers);
    expect(cue.group.children).toHaveLength(2);
    expect(ledger.snapshot().live).toMatchObject({ geometries: 2, materials: 2, textures: 0, instanceMatrices: 0, instanceColors: 0 });
    ledger.disposeObjects([cue.group]);
    disposeObjectChildren(cue.group);
    expect(Object.values(ledger.snapshot().live).every((count) => count === 0)).toBe(true);
    for (const dispose of disposals) expect(dispose).toHaveBeenCalledTimes(1);
  });

  it("keeps exact theme colors with contrasting rims and renderer pixel ratio", () => {
    const cue = new ViewportSelectionPresentation([primitive("a")]);
    for (const [color, rim] of [[0xf08c22, 0x0c1114], [0xa34400, 0xffffff]]) {
      cue.update(new Set([key("a")]), new Set(), color, rim, 2);
      expect(cue.spans.material.color.getHex()).toBe(color);
      expect(cue.markers.material.uniforms.selectedColor.value.getHex()).toBe(color);
      expect(cue.markers.material.uniforms.rimColor.value.getHex()).toBe(rim);
      expect(relativeContrastRatio(color, rim)).toBeGreaterThanOrEqual(3);
      expect(cue.markers.material.uniforms.pixelRatio.value).toBe(2);
    }
    disposeObjectChildren(cue.group);
  });

  it("replaces the resource-owned batch and retires the old geometry on model replacement", () => {
    const ledger = new ViewportOwnershipLedger(1);
    const fake = Object.assign(Object.create(ViewportResource.prototype), {
      scene: new THREE.Scene(), ownership: ledger, selectionPresentation: null,
      selectedKeys: new Set([key("a")]), hiddenKeys: new Set(), themePresentation: "dark",
      renderer: { getPixelRatio: () => 2 }, notifyResourceStateChange: vi.fn(), invalidate: vi.fn(),
      renderOrigin: { x: 0, y: 0, z: 0 }, camera: new THREE.PerspectiveCamera(),
      controls: { target: new THREE.Vector3() }, cancelNavigation: vi.fn(), markCameraProjectionChanged: vi.fn()
    });
    ViewportResource.prototype.setPointPrimitives.call(fake, {} as never, [primitive("a")]);
    const first = fake.selectionPresentation as ViewportSelectionPresentation;
    const dispose = vi.spyOn(first.markers.geometry, "dispose");
    ViewportResource.prototype.setOrigin.call(fake, { x: 100, y: 0, z: 0 });
    expect(first.group.position.x).toBe(-100);
    ViewportResource.prototype.setPointPrimitives.call(fake, {} as never, [primitive("b")]);
    expect(dispose).toHaveBeenCalledOnce();
    expect(fake.scene.children).toHaveLength(1);
    expect(fake.selectionPresentation.group.visible).toBe(false);
    expect(fake.selectionPresentation.group.position.x).toBe(0);
    expect(ledger.snapshot().live).toMatchObject({ geometries: 2, materials: 2 });
    ledger.disposeObjects(fake.scene.children);
    disposeObjectChildren(fake.scene);
    expect(ledger.snapshot().live.geometries).toBe(0);
  });
});
