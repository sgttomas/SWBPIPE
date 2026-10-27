import { act, fireEvent, render, screen } from "@testing-library/react";
import * as THREE from "three";
import { afterEach, describe, expect, it, vi } from "vitest";
import { DisplayUnitsProvider } from "../display-units";
import { loadBundledMechanicsReference } from "../../services/previewService";
import { numericalHealthView, readNumericalHealth, type ResultOverlayStanding } from "../results/numericalHealth";
import { numericalResultStanding } from "../results/resultStanding";
import type { MechanicsResult, PreviewModel } from "../../types";
import { entityKey } from "../workspace/selectionState";
import { PipeViewport } from "./PipeViewport";
import { resultNoValueHex } from "./viewportPalette";
import { ViewportResource } from "./viewportResource";

// The result contour in the viewport's own scene graph (A4; DEL-07-05 Design `result-contour.md`
// §3, §9). jsdom has no WebGL, so three's renderer is replaced by a stand-in that draws nothing;
// every other part of the viewport resource (layers, palette, instance colours, ownership and
// disposal) is the product's own. What is drawn on a GPU still needs a browser.

vi.mock("three", async (original) => {
  const actual = await original<typeof import("three")>();
  class StandInRenderer {
    domElement = document.createElement("canvas");
    autoClear = true;
    info = { memory: { geometries: 0, textures: 0 }, render: { calls: 0, triangles: 0, points: 0, lines: 0 } };
    renderLists = { dispose: () => {} };
    private pixelRatio = 1;
    setPixelRatio(value: number) { this.pixelRatio = value; }
    getPixelRatio() { return this.pixelRatio; }
    setSize() {}
    render() {}
    setViewport() {}
    setScissor() {}
    setScissorTest() {}
    getScissorTest() { return false; }
    getViewport(target: InstanceType<typeof actual.Vector4>) { return target.set(0, 0, 1, 1); }
    getScissor(target: InstanceType<typeof actual.Vector4>) { return target.set(0, 0, 1, 1); }
    getDrawingBufferSize(target: InstanceType<typeof actual.Vector2>) { return target.set(1, 1); }
    clearDepth() {}
    forceContextLoss() {}
    dispose() {}
  }
  return { ...actual, WebGLRenderer: StandInRenderer };
});

function sessionStanding(model: PreviewModel, result: MechanicsResult): ResultOverlayStanding {
  const standing = numericalResultStanding(result, model);
  return {
    eligible: standing.eligible,
    health: numericalHealthView(readNumericalHealth(result, standing.findings), { eligible: standing.eligible, verdictHold: false })
  };
}

// The resource the viewport builds, caught as it takes its first layer.
let resource: ViewportResource | null = null;
const replaceLayer = ViewportResource.prototype.replaceLayer;
afterEach(() => {
  resource = null;
  vi.restoreAllMocks();
});

function catchResource() {
  vi.spyOn(ViewportResource.prototype, "replaceLayer").mockImplementation(function (this: ViewportResource, layer, objects) {
    resource = this;
    return replaceLayer.call(this, layer, objects);
  });
}

const sleeves = () => {
  const found: THREE.Mesh[] = [];
  resource!.resultLayer.traverse((object) => { if (object.name === "viewport-result-contour") found.push(object as THREE.Mesh); });
  return found;
};

function pipeInstanceHex(pipeId: string): number | null {
  let hex: number | null = null;
  resource!.modelLayer.traverse((object) => {
    if (!(object instanceof THREE.InstancedMesh) || object.userData.viewportPaletteRole !== "pipe") return;
    const keys = object.userData.instanceEntityKeys as readonly string[];
    const index = keys.indexOf(entityKey({ type: "pipe", id: pipeId }));
    if (index >= 0 && object.instanceColor) hex = new THREE.Color().fromBufferAttribute(object.instanceColor, index).getHex();
  });
  return hex;
}

async function mount() {
  catchResource();
  const { model, source } = await loadBundledMechanicsReference();
  const view = render(
    <DisplayUnitsProvider>
      <PipeViewport model={model} result={source} resultStanding={sessionStanding(model, source)} selection={{ type: "project", id: model.project.id }} onSelect={vi.fn()} />
    </DisplayUnitsProvider>
  );
  expect(resource).not.toBeNull();
  fireEvent.click(screen.getByTestId("toggle-viewport-result-colour"));
  return { model, view };
}

describe("PipeViewport result contour in the scene graph", () => {
  it("adds one sleeve mesh per chunk only along members, and takes it away with one value per member", async () => {
    const { model } = await mount();
    expect(sleeves()).toHaveLength(0);
    fireEvent.change(screen.getByTestId("viewport-overlay-resolution"), { target: { value: "station" } });
    const drawn = sleeves();
    expect(drawn.length).toBeGreaterThan(0);
    expect(drawn.flatMap((mesh) => mesh.userData.viewportContourMembers as string[]).sort())
      .toEqual(model.pipe_segments.map((pipe) => entityKey({ type: "pipe", id: pipe.id })).sort());
    fireEvent.change(screen.getByTestId("viewport-overlay-display"), { target: { value: "numbers" } });
    expect(sleeves()).toHaveLength(0);
    fireEvent.change(screen.getByTestId("viewport-overlay-display"), { target: { value: "colour" } });
    expect(sleeves().length).toBeGreaterThan(0);
    fireEvent.change(screen.getByTestId("viewport-overlay-resolution"), { target: { value: "member" } });
    expect(sleeves()).toHaveLength(0);
  });

  it("gives a member drawn along its length the no-value colour on its own tube, ends included", async () => {
    const { model } = await mount();
    const pipe = model.pipe_segments[0].id;
    const theme = "light";
    const before = pipeInstanceHex(pipe);
    expect(before).not.toBe(resultNoValueHex(theme));
    fireEvent.change(screen.getByTestId("viewport-overlay-resolution"), { target: { value: "station" } });
    for (const item of model.pipe_segments) expect(pipeInstanceHex(item.id), item.id).toBe(resultNoValueHex(theme));
  });

  it("disposes the sleeve geometry and material when the result layer is replaced and when the viewport unmounts", async () => {
    const { view } = await mount();
    fireEvent.change(screen.getByTestId("viewport-overlay-resolution"), { target: { value: "station" } });
    const first = sleeves();
    const disposed = { geometry: 0, material: 0 };
    for (const mesh of first) {
      mesh.geometry.addEventListener("dispose", () => { disposed.geometry += 1; });
      (mesh.material as THREE.Material).addEventListener("dispose", () => { disposed.material += 1; });
    }
    // A new threshold replaces the sleeves.
    fireEvent.click(screen.getByTestId("viewport-overlay-threshold-on"));
    fireEvent.change(screen.getByTestId("viewport-overlay-threshold"), { target: { value: "1" } });
    expect(disposed).toEqual({ geometry: first.length, material: first.length });
    const second = sleeves();
    expect(second.length).toBeGreaterThan(0);
    expect(second.some((mesh) => first.includes(mesh))).toBe(false);
    const ownership = resource!.ownershipSnapshot;
    for (const mesh of second) {
      mesh.geometry.addEventListener("dispose", () => { disposed.geometry += 1; });
      (mesh.material as THREE.Material).addEventListener("dispose", () => { disposed.material += 1; });
    }
    expect(ownership.live.geometries).toBeGreaterThan(0);
    act(() => view.unmount());
    expect(disposed).toEqual({ geometry: first.length + second.length, material: first.length + second.length });
  });
});
