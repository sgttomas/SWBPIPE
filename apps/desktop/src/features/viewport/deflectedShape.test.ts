// Deflected-shape view settings (A5; design system 6.8): one case drawn, a magnification
// factor on the physical displacement, the phase between the undeformed and the deformed
// shape, and the undeformed ghost. All rows are invented display inputs; no engineering result
// is asserted, and every setting is shown to leave the result unchanged.
import { describe, expect, it, vi } from "vitest";
import * as THREE from "three";
import { loadPreviewModel } from "../../services/previewService";
import type { MechanicsResult } from "../../types";
import { entityKey, type EntityKey } from "../workspace/selectionState";
import {
  buildDeformationOverlay,
  buildRotationOverlay,
  deformationAtPhase,
  NORMALIZED_DEFORMATION_DISPLAY_OFFSET,
  roundFactor125,
  stepFactor125
} from "./deformationOverlay";
import { createFigureMaterial } from "./viewportFigureMaterial";
import { registerInstancedRolePresentation, ViewportOwnershipLedger, ViewportResource } from "./viewportResource";
import { deflectionLabel } from "./DeflectedShapeControls";

type Row = MechanicsResult["results"][number];

function solved(results: Row[]): MechanicsResult {
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.product_preview.mechanics_result",
    run_id: "run:deflected-shape-unit-test",
    model_ref: "project:invented-demo-loop",
    status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE", professional_acceptance: "NOT_PROVIDED" },
    summary: {},
    results,
    diagnostics: []
  };
}

function displacement(nodeId: string, vector: { x: number; y: number; z: number }, loadCase: string, unit = "mm"): Row[] {
  const basis_ref = { ref_type: "load_case" as const, ref_id: loadCase };
  return [
    { id: `d:${nodeId}:${loadCase}`, kind: "displacement_magnitude", value: Math.hypot(vector.x, vector.y, vector.z), unit, dimension: "length", entity_ref: nodeId, basis_ref },
    ...(["x", "y", "z"] as const).map((axis) => ({
      id: `d:${nodeId}:${loadCase}:${axis}`, kind: `global_nodal_displacement_${axis}`, value: vector[axis], unit, dimension: "length" as const, entity_ref: nodeId, basis_ref
    }))
  ];
}

function rotation(nodeId: string, vector: { x: number; y: number; z: number }, loadCase: string): Row[] {
  return (["x", "y", "z"] as const).map((axis) => ({
    id: `r:${nodeId}:${loadCase}:${axis}`, kind: `global_nodal_rotation_${axis}`, value: vector[axis], unit: "rad", dimension: "angle" as const,
    entity_ref: nodeId, basis_ref: { ref_type: "load_case" as const, ref_id: loadCase }
  }));
}

const L100 = "load_case:load:L-100";
const L200 = "load_case:load:L-200";

async function fixture() {
  const model = await loadPreviewModel();
  const node = model.nodes.find((candidate) => candidate.id === "node:N-140")!;
  const other = model.nodes.find((candidate) => candidate.id === "node:N-130")!;
  const result = solved([
    ...displacement("node:N-140", { x: 0, y: 0, z: -20 }, "load:L-100"),
    ...displacement("node:N-130", { x: 10, y: 0, z: 0 }, "load:L-100"),
    // The other case moves N-130 further, so it governs there over every case.
    ...displacement("node:N-130", { x: 0, y: 40, z: 0 }, "load:L-200")
  ]);
  return { model, node, other, result };
}

describe("one case at a magnification factor", () => {
  it("draws each node at its physical displacement times the factor, in model length units", async () => {
    const { model, node, other, result } = await fixture();
    expect(model.project.units.length).toBe("m");
    const overlay = buildDeformationOverlay(model, result, { basisKey: L100, scale: { mode: "factor", factor: 100 }, modelLengthUnit: "m", modelExtent: 10 });

    expect(overlay.state).toBe("available");
    expect(overlay.scale).toEqual({ mode: "factor", factor: 100, isDefault: false });
    // 20 mm = 0.02 m, times 100.
    expect(overlay.nodePositions.get("node:N-140")!.z).toBeCloseTo(node.position.z - 2, 12);
    expect(overlay.nodePositions.get("node:N-130")!.x).toBeCloseTo(other.position.x + 1, 12);
    // Only load:L-100's rows: L-200's larger movement of N-130 is not drawn.
    expect(overlay.nodePositions.get("node:N-130")!.y).toBeCloseTo(other.position.y, 12);
    expect(overlay.maximum).toEqual({ value: 20, unit: "mm" });
    expect(overlay.boundary).toContain("scale=factor_times_physical_displacement; factor=100; model_length_unit=m");
    expect(overlay.boundary).toContain(`case=${L100}`);
    expect(overlay.boundary).toContain("professional_claim=false");
  });

  it("defaults the factor to the 1-2-5 value drawing the largest displacement at a tenth of the model extent", async () => {
    const { model, result } = await fixture();
    // 10 m / 10 = 1 m, over 20 mm: ×50.
    const overlay = buildDeformationOverlay(model, result, { basisKey: L100, scale: { mode: "factor", factor: null }, modelLengthUnit: "m", modelExtent: 10 });
    expect(overlay.defaultFactor).toBe(50);
    expect(overlay.scale).toEqual({ mode: "factor", factor: 50, isDefault: true });
    expect(deflectionLabel(overlay, "Thermal (load:L-100)")).toBe("Deformed ×50 (default) · Thermal (load:L-100)");
  });

  it("keeps the normalized drawing, with the reason, when a factor cannot apply", async () => {
    const { model, result } = await fixture();
    const governing = buildDeformationOverlay(model, result, { scale: { mode: "factor", factor: 10 }, modelLengthUnit: "m", modelExtent: 10 });
    expect(governing.scale).toEqual({ mode: "normalized" });
    expect(governing.factorUnavailable).toContain("mixes cases");
    const noUnit = buildDeformationOverlay(model, result, { basisKey: L100, scale: { mode: "factor", factor: 10 }, modelLengthUnit: null, modelExtent: 10 });
    expect(noUnit.scale).toEqual({ mode: "normalized" });
    expect(noUnit.factorUnavailable).toContain("does not convert");
    expect(noUnit.boundary).toContain("scale=normalized_display_offset_not_physical_length");
    expect(deflectionLabel(noUnit, null)).toContain("normalized, not to scale");
  });

  it("leaves the governing drawing unchanged without options", async () => {
    const { model, other, result } = await fixture();
    const overlay = buildDeformationOverlay(model, result);
    expect(overlay.basisKey).toBeNull();
    expect(overlay.scale).toEqual({ mode: "normalized" });
    expect(overlay.factorUnavailable).toBeUndefined();
    // N-130 governs at 40 mm under L-200, drawn at the full normalized offset along +Y.
    expect(overlay.nodePositions.get("node:N-130")).toEqual({ ...other.position, y: other.position.y + NORMALIZED_DEFORMATION_DISPLAY_OFFSET });
  });

  it("never changes the result it draws", async () => {
    const { model, result } = await fixture();
    const before = structuredClone(result);
    const overlay = buildDeformationOverlay(model, result, { basisKey: L200, scale: { mode: "factor", factor: 7 }, modelLengthUnit: "m", modelExtent: 10 });
    deformationAtPhase(model, overlay, 0.3);
    buildRotationOverlay(model, result, { basisKey: L200 });
    expect(result).toEqual(before);
  });
});

describe("the phase between the undeformed and the deformed shape", () => {
  it("draws the undeformed shape at 0, half the offset at 0.5 and the deformed shape at 1", async () => {
    const { model, node, result } = await fixture();
    const overlay = buildDeformationOverlay(model, result, { basisKey: L100, scale: { mode: "factor", factor: 100 }, modelLengthUnit: "m", modelExtent: 10 });
    expect(deformationAtPhase(model, overlay, 0).nodePositions.get("node:N-140")).toEqual(node.position);
    expect(deformationAtPhase(model, overlay, 0.5).nodePositions.get("node:N-140")!.z).toBeCloseTo(node.position.z - 1, 12);
    expect(deformationAtPhase(model, overlay, 1)).toBe(overlay);
    // The readout is the emitted maximum at every phase.
    expect(deformationAtPhase(model, overlay, 0.25).maximum).toEqual({ value: 20, unit: "mm" });
  });
});

describe("factor rounding and stepping", () => {
  it("rounds to the nearest 1-2-5 value and steps through the sequence", () => {
    expect(roundFactor125(50)).toBe(50);
    expect(roundFactor125(70)).toBe(50);
    expect(roundFactor125(80)).toBe(100);
    expect(roundFactor125(0.03)).toBe(0.02);
    expect(stepFactor125(50, 1)).toBe(100);
    expect(stepFactor125(50, -1)).toBe(20);
    expect(stepFactor125(1, -1)).toBe(0.5);
    expect(stepFactor125(37, 1)).toBe(50);
    expect(stepFactor125(37, -1)).toBe(20);
  });
});

describe("rotation glyphs follow the drawn case", () => {
  it("reads only the chosen case's rotation rows", async () => {
    const { model, result } = await fixture();
    const withRotations = solved([...result.results, ...rotation("node:N-130", { x: 0.001, y: 0, z: 0 }, "load:L-100"), ...rotation("node:N-130", { x: 0, y: 0, z: 0.003 }, "load:L-200")]);
    expect(buildRotationOverlay(model, withRotations, { basisKey: L100 }).glyphs.get("node:N-130")!.axis).toEqual({ x: 1, y: 0, z: 0 });
    expect(buildRotationOverlay(model, withRotations, { basisKey: L200 }).glyphs.get("node:N-130")!.axis).toEqual({ x: 0, y: 0, z: 1 });
    // Without a case, N-130's governing displacement (L-200) picks its glyph, as before.
    expect(buildRotationOverlay(model, withRotations).glyphs.get("node:N-130")!.basisKey).toBe(L200);
  });
});

describe("the undeformed ghost", () => {
  function fakeResource() {
    const layers = { modelLayer: new THREE.Group(), authoredLoadLayer: new THREE.Group(), resultLayer: new THREE.Group(), diagnosticLayer: new THREE.Group(), routingLayer: new THREE.Group() };
    const empty = new Set<EntityKey>();
    return Object.assign(Object.create(ViewportResource.prototype), {
      ...layers, scene: new THREE.Scene(), gizmoScene: new THREE.Scene(), ownership: new ViewportOwnershipLedger(910), halo: null,
      resourceGeneration: 910, contextLostCount: 0, contextRestoredCount: 0, contextStatus: "ready", scheduler: { pendingCount: 0 },
      renderer: { domElement: { isConnected: true } }, options: {}, invalidate: vi.fn(),
      selectedKeys: empty, hiddenKeys: empty, dimmedKeys: empty, hoveredKey: null,
      selectionPresentation: null, themePresentation: "light", gridVisible: true, authoredLoadsVisible: true
    }) as ViewportResource & typeof layers;
  }
  function instanced(keys: EntityKey[]) {
    const mesh = new THREE.InstancedMesh(new THREE.CylinderGeometry(1, 1, 1, 6), createFigureMaterial(), keys.length);
    keys.forEach((_, index) => mesh.setMatrixAt(index, new THREE.Matrix4().makeTranslation(index, 0, 0)));
    registerInstancedRolePresentation(mesh, keys, "pipe");
    return mesh;
  }

  it("dims the undeformed model and not the deformed shape, and clears without residue", () => {
    const pipe = entityKey({ type: "pipe", id: "pipe:A" });
    const resource = fakeResource();
    const undeformed = instanced([pipe]);
    const deformed = instanced([pipe]);
    resource.replaceLayer(resource.modelLayer, [undeformed]);
    resource.replaceLayer(resource.resultLayer, [deformed]);

    resource.setUndeformedGhostPresentation(new Set([pipe]));
    const modelDim = undeformed.children[0] as THREE.InstancedMesh;
    const resultDim = deformed.children[0] as THREE.InstancedMesh;
    expect(undeformed.count).toBe(0);
    expect(modelDim.count).toBe(1);
    expect((modelDim.material as THREE.Material).opacity).toBe(0.2);
    expect(deformed.count).toBe(1);
    expect(resultDim.count).toBe(0);

    // A rebuilt model layer keeps the ghost.
    const rebuilt = instanced([pipe]);
    resource.replaceLayer(resource.modelLayer, [rebuilt]);
    expect(rebuilt.count).toBe(0);
    expect((rebuilt.children[0] as THREE.InstancedMesh).count).toBe(1);

    resource.setUndeformedGhostPresentation(new Set());
    expect(rebuilt.count).toBe(1);
    expect((rebuilt.children[0] as THREE.InstancedMesh).count).toBe(0);
  });
});

