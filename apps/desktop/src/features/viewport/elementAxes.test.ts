// Element axis triads (A7). The local frame must be the solver's: the strongest check here
// rotates the bundled invented reference's published element-local end rows with the drawn
// axes and recovers the solver's own published global end rows (W-7: F = R^T f), member by
// member, case by case, end by end. The other tests pin the convention's cases.
import { describe, expect, it } from "vitest";
import * as THREE from "three";
import { loadBundledMechanicsReference } from "../../services/previewService";
import type { MechanicsResult, PreviewModel, Vec3 } from "../../types";
import { entityKey } from "../workspace/selectionState";
import { buildElementAxesView, straightMemberLocalAxes, WITHHELD_REASONS } from "./elementAxes";
import { elementAxisArrowMatrices, elementAxisDisplayLength, ELEMENT_AXIS_MAX_LENGTH, instancedElementAxisMeshes } from "./viewportElementAxes";

type Row = MechanicsResult["results"][number];

function find(rows: readonly Row[], kind: string, pipe: string, end: string, basis: string, component?: string): Row | undefined {
  return rows.find((row) => row.kind === kind && row.entity_ref === pipe && row.metadata?.location === end &&
    row.basis_ref?.ref_id === basis && (component === undefined || row.metadata?.component === component));
}

function rotate(axes: { x: Vec3; y: Vec3; z: Vec3 }, local: [number, number, number]): [number, number, number] {
  const [a, b, c] = local;
  return [
    a * axes.x.x + b * axes.y.x + c * axes.z.x,
    a * axes.x.y + b * axes.y.y + c * axes.z.y,
    a * axes.x.z + b * axes.y.z + c * axes.z.z
  ];
}

function expectAxes(actual: ReturnType<typeof straightMemberLocalAxes>, expected: { x: Vec3; y: Vec3; z: Vec3 }) {
  expect("reason" in actual).toBe(false);
  if ("reason" in actual) return;
  for (const axis of ["x", "y", "z"] as const) {
    for (const component of ["x", "y", "z"] as const) expect(actual[axis][component], `${axis}.${component}`).toBeCloseTo(expected[axis][component], 15);
  }
}

describe("the local frame is the solver's", () => {
  it("rotates the published local end rows onto the published global end rows of every member", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    const view = buildElementAxesView(model, "local");
    expect(view.withheld.size).toBe(0);
    const bases = [...new Set(source.results.flatMap((row) => row.basis_ref?.ref_type === "load_case" ? [row.basis_ref.ref_id] : []))];
    let compared = 0;
    for (const pipe of model.pipe_segments) {
      const axes = view.triads.get(pipe.id)!;
      for (const basis of bases) {
        for (const end of ["end_i", "end_j"]) {
          const groups = [
            { local: ["element_local_axial_force", "element_local_shear_force_y", "element_local_shear_force_z"], global: "element_global_end_force_v1", components: ["Fx", "Fy", "Fz"] },
            { local: ["element_local_torsional_moment", "element_local_bending_moment_y", "element_local_bending_moment_z"], global: "element_global_end_moment_v1", components: ["Mx", "My", "Mz"] }
          ];
          for (const group of groups) {
            const local = group.local.map((kind) => find(source.results, kind, pipe.id, end, basis)?.value);
            const published = group.components.map((component) => find(source.results, group.global, pipe.id, end, basis, component)?.value);
            expect(local.every((value) => typeof value === "number"), `${pipe.id} ${basis} ${end} local`).toBe(true);
            expect(published.every((value) => typeof value === "number"), `${pipe.id} ${basis} ${end} global`).toBe(true);
            const rotated = rotate(axes, local as [number, number, number]);
            const scale = Math.max(1, ...published.map((value) => Math.abs(value!)));
            rotated.forEach((value, index) => expect(Math.abs(value - published[index]!), `${pipe.id} ${basis} ${end} ${group.components[index]}`).toBeLessThanOrEqual(1e-9 * scale));
            compared += 1;
          }
        }
      }
    }
    expect(compared).toBe(model.pipe_segments.length * bases.length * 2 * 2);
  });

  it("forms x along the member, y from y_reference without its x part, and z = x cross y", () => {
    const along = straightMemberLocalAxes({ x: 0, y: 0, z: 0 }, { x: 3, y: 0, z: 0 }, { x: 0, y: 0, z: 1 });
    expectAxes(along, { x: { x: 1, y: 0, z: 0 }, y: { x: 0, y: 0, z: 1 }, z: { x: 0, y: -1, z: 0 } });
    const oblique = straightMemberLocalAxes({ x: 0, y: 0, z: 0 }, { x: 2, y: 0, z: 0 }, { x: 5, y: 1, z: 1 });
    expect("reason" in oblique).toBe(false);
    if (!("reason" in oblique)) {
      expect(oblique.y.x).toBe(0);
      expect(oblique.y.y).toBeCloseTo(Math.SQRT1_2, 15);
      expect(oblique.y.z).toBeCloseTo(Math.SQRT1_2, 15);
    }
    // Reversing the member reverses x and z, and keeps y.
    const reversed = straightMemberLocalAxes({ x: 3, y: 0, z: 0 }, { x: 0, y: 0, z: 0 }, { x: 0, y: 0, z: 1 });
    expectAxes(reversed, { x: { x: -1, y: 0, z: 0 }, y: { x: 0, y: 0, z: 1 }, z: { x: 0, y: 1, z: 0 } });
  });

  it("applies no default orientation and draws no straight triad on a realized curved bend", async () => {
    const { model } = await loadBundledMechanicsReference();
    const [first, second, third] = model.pipe_segments;
    const edited: PreviewModel = {
      ...model,
      pipe_segments: model.pipe_segments.map((pipe) =>
        pipe.id === first.id ? { ...pipe, y_reference: undefined }
          : pipe.id === second.id ? { ...pipe, y_reference: { x: 0, y: 0, z: 0 } }
            : pipe),
      components: [
        ...(model.components ?? []),
        { id: "component:invented-arc", label: "Invented arc", kind: "bend", node: third.to, provenance: "invented",
          geometry: { bend_pipe_ref: third.id }, mechanics_interface: { solver_consumption: "curved_bend_macro_element" } }
      ]
    };
    const local = buildElementAxesView(edited, "local");
    expect(local.withheld.get(first.id)).toBe(WITHHELD_REASONS.noYReference);
    expect(local.withheld.get(second.id)).toBe(WITHHELD_REASONS.parallelYReference);
    expect(local.withheld.get(third.id)).toBe(WITHHELD_REASONS.curvedBend);
    // A geometry-only bend leaves its span a straight member (the demo's C-110).
    expect(model.components?.some((component) => component.kind === "bend")).toBe(true);
    expect(buildElementAxesView(model, "local").withheld.size).toBe(0);
    // The global frame is the model's axes at every member.
    const global = buildElementAxesView(edited, "global");
    expect(global.withheld.size).toBe(0);
    expect(global.triads.get(first.id)).toMatchObject({ x: { x: 1, y: 0, z: 0 }, y: { x: 0, y: 1, z: 0 }, z: { x: 0, y: 0, z: 1 } });
  });

  it("never reads or changes a result", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    const before = structuredClone(source);
    buildElementAxesView(model, "local");
    expect(source).toEqual(before);
  });
});

describe("element axis meshes", () => {
  it("draws three owned arrows per member along its axes", () => {
    const key = entityKey({ type: "pipe", id: "pipe:A" });
    const meshes = instancedElementAxisMeshes([[{ ownerKey: key, anchor: { x: 1, y: 2, z: 3 }, axes: [{ x: 1, y: 0, z: 0 }, { x: 0, y: 0, z: 1 }, { x: 0, y: -1, z: 0 }], displayLength: 0.5 }]]);
    expect(meshes.map((mesh) => mesh.name)).toEqual([
      "viewport-element-axis-x-shaft", "viewport-element-axis-x-head",
      "viewport-element-axis-y-shaft", "viewport-element-axis-y-head",
      "viewport-element-axis-z-shaft", "viewport-element-axis-z-head"
    ]);
    expect(meshes.map((mesh) => mesh.userData.viewportPaletteRole)).toEqual(["elementAxisX", "elementAxisX", "elementAxisY", "elementAxisY", "elementAxisZ", "elementAxisZ"]);
    for (const mesh of meshes) expect(mesh.userData.instanceEntityKeys).toEqual([key]);
    // The y head sits along +Z from the anchor, at the arrow's far end.
    const head = new THREE.Vector3().setFromMatrixPosition(elementAxisArrowMatrices({ x: 1, y: 2, z: 3 }, { x: 0, y: 0, z: 1 }, 0.5).head);
    expect(head.x).toBeCloseTo(1, 12);
    expect(head.y).toBeCloseTo(2, 12);
    expect(head.z).toBeGreaterThan(3.4);
    expect(head.z).toBeLessThan(3.5);
    expect(elementAxisDisplayLength(100)).toBe(ELEMENT_AXIS_MAX_LENGTH);
    expect(elementAxisDisplayLength(1)).toBeCloseTo(0.35, 12);
  });
});
