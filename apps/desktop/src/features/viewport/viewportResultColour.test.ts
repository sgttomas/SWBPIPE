import * as THREE from "three";
import { describe, expect, it, vi } from "vitest";
import { entityKey } from "../workspace/selectionState";
import { createFigureMaterial } from "./viewportFigureMaterial";
import { viewportRoleHex } from "./viewportPalette";
import {
  applyResultColourPresentation,
  registerInstancedRolePresentation,
  ViewportOwnershipLedger,
  ViewportResource,
  type ResultColourTable
} from "./viewportResource";

// Result colour (design system 6.7, OD-5) is a per-instance colour of the model's tubes, written in
// place when the table changes. These tests drive the resource's own paint paths (selection,
// theme, visibility, layer replacement) and check that each keeps the result colour and that
// taking the table away restores the role colour. Invented keys and colours only.

const instanceHex = (mesh: THREE.InstancedMesh, index: number) =>
  new THREE.Color().fromBufferAttribute(mesh.instanceColor!, index).getHex();

function paletteResource(theme: "light" | "dark") {
  const layers = {
    modelLayer: new THREE.Group(),
    authoredLoadLayer: new THREE.Group(),
    resultLayer: new THREE.Group(),
    diagnosticLayer: new THREE.Group(),
    routingLayer: new THREE.Group()
  };
  const ownership = new ViewportOwnershipLedger(31);
  const resource = Object.assign(Object.create(ViewportResource.prototype), {
    ownership,
    resourceGeneration: 31,
    contextLostCount: 0,
    contextRestoredCount: 0,
    scheduler: { pendingCount: 0 },
    renderer: { domElement: { isConnected: true }, getPixelRatio: () => 1 },
    options: {},
    invalidate: vi.fn(),
    scene: new THREE.Scene(),
    gizmoScene: new THREE.Scene(),
    selectionPresentation: null,
    ...layers,
    selectedKeys: new Set(),
    hiddenKeys: new Set(),
    dimmedKeys: new Set(),
    themePresentation: theme,
    gridVisible: true,
    authoredLoadsVisible: true,
    resultColours: null
  }) as ViewportResource;
  return { resource, ownership, ...layers };
}

function rolePipes(keys: readonly ReturnType<typeof entityKey>[], role: "pipe" | "deformedShape" = "pipe") {
  const mesh = new THREE.InstancedMesh(new THREE.CylinderGeometry(1, 1, 1, 8), createFigureMaterial(), keys.length);
  keys.forEach((_, index) => mesh.setMatrixAt(index, new THREE.Matrix4().makeTranslation(index, 0, 0)));
  registerInstancedRolePresentation(mesh, keys, role);
  return mesh;
}

const P1 = entityKey({ type: "pipe", id: "pipe:P-1" });
const P2 = entityKey({ type: "pipe", id: "pipe:P-2" });
const P3 = entityKey({ type: "pipe", id: "pipe:P-3" });
// Arbitrary distinct test numbers: the table is data here, not a palette.
const TABLE: ResultColourTable = new Map([
  [P1, { light: 0x112233, dark: 0x445566 }],
  [P3, { light: 0x778899, dark: 0x99aabb }]
]);

describe("result colour on the model's tubes", () => {
  it("colours each instance from the table by its element, and every other instance by its role", () => {
    const { resource, modelLayer } = paletteResource("light");
    const mesh = rolePipes([P1, P2, P3]);
    resource.replaceLayer(modelLayer, [mesh]);
    resource.setResultColourPresentation(TABLE);
    expect([0, 1, 2].map((index) => instanceHex(mesh, index))).toEqual([0x112233, viewportRoleHex("light", "pipe"), 0x778899]);
  });

  it("keeps the result colour through a selection repaint and repaints it for the theme", () => {
    const { resource, modelLayer } = paletteResource("light");
    const mesh = rolePipes([P1, P2, P3]);
    resource.replaceLayer(modelLayer, [mesh]);
    resource.setResultColourPresentation(TABLE);
    resource.setSelectionPresentation([P1]);
    expect(instanceHex(mesh, 0)).toBe(0x112233);
    resource.setThemePresentation("dark");
    expect([0, 1, 2].map((index) => instanceHex(mesh, index))).toEqual([0x445566, viewportRoleHex("dark", "pipe"), 0x99aabb]);
    resource.setSelectionPresentation([]);
    expect(instanceHex(mesh, 2)).toBe(0x99aabb);
  });

  it("restores the role colour when the table is taken away, creating and disposing nothing", () => {
    const { resource, ownership, modelLayer } = paletteResource("light");
    const mesh = rolePipes([P1, P2, P3]);
    resource.replaceLayer(modelLayer, [mesh]);
    const before = ownership.snapshot();
    const geometry = mesh.geometry;
    const material = mesh.material;
    const colours = mesh.instanceColor;
    resource.setResultColourPresentation(TABLE);
    resource.setResultColourPresentation(null);
    expect([0, 1, 2].map((index) => instanceHex(mesh, index))).toEqual([0, 1, 2].map(() => viewportRoleHex("light", "pipe")));
    expect(mesh.userData.viewportResultColours).toBeUndefined();
    expect(ownership.snapshot()).toEqual(before);
    expect(mesh.geometry).toBe(geometry);
    expect(mesh.material).toBe(material);
    expect(mesh.instanceColor).toBe(colours);
  });

  it("applies the kept table to a rebuilt model layer, and not to other layers or roles", () => {
    const { resource, modelLayer, resultLayer } = paletteResource("light");
    resource.setResultColourPresentation(TABLE);
    const rebuilt = rolePipes([P1, P2]);
    resource.replaceLayer(modelLayer, [rebuilt]);
    expect(instanceHex(rebuilt, 0)).toBe(0x112233);
    // The deformed overlay carries pipe keys too, and keeps its own role colour.
    const deformed = rolePipes([P1], "deformedShape");
    resource.replaceLayer(resultLayer, [deformed]);
    expect(instanceHex(deformed, 0)).toBe(viewportRoleHex("light", "deformedShape"));
    applyResultColourPresentation([resultLayer], TABLE);
    expect(instanceHex(deformed, 0)).toBe(viewportRoleHex("light", "deformedShape"));
  });

  it("keeps each element's result colour when isolation moves it into the dimmed batch and back", () => {
    const { resource, modelLayer } = paletteResource("light");
    const mesh = rolePipes([P1, P2, P3]);
    resource.replaceLayer(modelLayer, [mesh]);
    resource.setResultColourPresentation(TABLE);
    resource.setVisibilityPresentation(new Set(), new Set([P1]));
    const dimmed = mesh.children.find((child) => child instanceof THREE.InstancedMesh) as THREE.InstancedMesh;
    expect(dimmed.count).toBe(1);
    expect(instanceHex(dimmed, 0)).toBe(0x112233);
    expect(mesh.userData.instanceEntityKeys).toEqual([P2, P3]);
    expect(instanceHex(mesh, 1)).toBe(0x778899);
    // A theme change while dimmed repaints both batches by their own keys.
    resource.setThemePresentation("dark");
    expect(instanceHex(dimmed, 0)).toBe(0x445566);
    expect(instanceHex(mesh, 0)).toBe(viewportRoleHex("dark", "pipe"));
    resource.setVisibilityPresentation(new Set(), new Set());
    expect([0, 1, 2].map((index) => instanceHex(mesh, index))).toEqual([0x445566, viewportRoleHex("dark", "pipe"), 0x99aabb]);
  });
});
