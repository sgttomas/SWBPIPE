import { describe, expect, it } from "vitest";
import { loadBundledMechanicsReference, loadPreviewModel } from "../../services/previewService";
import type { MechanicsResult } from "../../types";
import { buildDeformationOverlay, buildRotationOverlay, ROTATION_GLYPH_MAX_DISPLAY_LENGTH } from "./deformationOverlay";

// All rows are invented display inputs; no engineering result is asserted.

type Row = MechanicsResult["results"][number];

function solved(results: Row[], mechanics = "MECHANICS_SOLVED"): MechanicsResult {
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.product_preview.mechanics_result",
    run_id: "run:rotation-overlay-unit-test",
    model_ref: "project:invented-demo-loop",
    status: { mechanics, rule_check: "RULE_INPUTS_INCOMPLETE", professional_acceptance: "NOT_PROVIDED" },
    summary: {},
    results,
    diagnostics: []
  };
}

function rotationRows(nodeId: string, vector: { x: number; y: number; z: number }, basis = "load:L-100", unit = "rad"): Row[] {
  return (["x", "y", "z"] as const).map((axis) => ({
    id: `result:rot:${nodeId}:${basis}:r${axis}`,
    kind: `global_nodal_rotation_${axis}`,
    value: vector[axis],
    unit,
    dimension: "angle" as const,
    entity_ref: nodeId,
    basis_ref: { ref_type: "load_case", ref_id: basis }
  }));
}

function magnitudeRow(nodeId: string, value: number, basis = "load:L-100"): Row {
  return {
    id: `result:disp:${nodeId}:${basis}`,
    kind: "displacement_magnitude",
    value,
    unit: "mm",
    dimension: "length",
    entity_ref: nodeId,
    basis_ref: { ref_type: "load_case", ref_id: basis }
  };
}

describe("rotation glyph overlay", () => {
  it("is not started without a result and blocked without solved mechanics", async () => {
    const model = await loadPreviewModel();
    expect(buildRotationOverlay(model, null)).toMatchObject({ state: "not_started", glyphs: new Map() });
    const blocked = buildRotationOverlay(model, solved(rotationRows("node:N-140", { x: 1, y: 0, z: 0 }), "MECHANICS_BLOCKED"));
    expect(blocked.state).toBe("blocked");
    expect(blocked.glyphs.size).toBe(0);
    expect(blocked.boundary).toContain("professional_claim=false");
  });

  it("draws each node's rotation vector along its rx/ry/rz components, normalized to the longest", async () => {
    const model = await loadPreviewModel();
    const overlay = buildRotationOverlay(model, solved([
      ...rotationRows("node:N-140", { x: 0, y: 0, z: -0.004 }),
      ...rotationRows("node:N-130", { x: 0.001, y: 0.001, z: 0 })
    ]));

    expect(overlay.state).toBe("available");
    expect(overlay.nodeCount).toBe(2);
    expect(overlay.maximum).toEqual({ value: 0.004, unit: "rad" });
    expect(overlay.boundary).toContain("scale=normalized_display_length_not_physical");
    expect(overlay.boundary).toContain("glyph=double_arrow_right_hand_rotation_vector");
    expect(overlay.boundary).toContain("unit_basis=rad");
    expect(overlay.boundary).toContain("professional_claim=false");

    const governing = overlay.glyphs.get("node:N-140")!;
    expect(governing.axis).toEqual({ x: 0, y: 0, z: -1 });
    expect(governing.displayLength).toBeCloseTo(ROTATION_GLYPH_MAX_DISPLAY_LENGTH, 12);
    expect(governing.components).toEqual({ x: 0, y: 0, z: -0.004 });

    const smaller = overlay.glyphs.get("node:N-130")!;
    expect(smaller.axis.x).toBeCloseTo(Math.SQRT1_2, 12);
    expect(smaller.axis.y).toBeCloseTo(Math.SQRT1_2, 12);
    expect(smaller.axis.z).toBe(0);
    expect(smaller.displayLength).toBeCloseTo((Math.hypot(0.001, 0.001) / 0.004) * ROTATION_GLYPH_MAX_DISPLAY_LENGTH, 12);
  });

  it("uses the basis that governs the node's deformed shape, else its longest rotation", async () => {
    const model = await loadPreviewModel();
    const overlay = buildRotationOverlay(model, solved([
      magnitudeRow("node:N-140", 3, "load:L-100"),
      magnitudeRow("node:N-140", 9, "load:L-200"),
      ...rotationRows("node:N-140", { x: 0.005, y: 0, z: 0 }, "load:L-100"),
      ...rotationRows("node:N-140", { x: 0, y: 0.001, z: 0 }, "load:L-200"),
      ...rotationRows("node:N-130", { x: 0, y: 0, z: 0.001 }, "load:L-100"),
      ...rotationRows("node:N-130", { x: 0, y: 0, z: -0.002 }, "load:L-200")
    ]));
    // N-140's deformed shape is governed by L-200 (9 mm), so its glyph is L-200's.
    expect(overlay.glyphs.get("node:N-140")).toMatchObject({ basisKey: "load_case:load:L-200", axis: { x: 0, y: 1, z: 0 } });
    // N-130 has no displacement row: its longer rotation (L-200) is drawn.
    expect(overlay.glyphs.get("node:N-130")).toMatchObject({ basisKey: "load_case:load:L-200", axis: { x: 0, y: 0, z: -1 } });
    // The readout's maximum covers only the drawn bases.
    expect(overlay.maximum).toEqual({ value: 0.002, unit: "rad" });
  });

  it("chooses the same governing displacement basis as the deformed shape on equal magnitudes", async () => {
    const model = await loadPreviewModel();
    const rows = [
      magnitudeRow("node:N-140", 4, "load:L-100"),
      magnitudeRow("node:N-140", -4, "load:L-200"),
      ...rotationRows("node:N-140", { x: 0.001, y: 0, z: 0 }, "load:L-100"),
      ...rotationRows("node:N-140", { x: 0, y: 0.003, z: 0 }, "load:L-200")
    ];
    expect(buildRotationOverlay(model, solved(rows)).glyphs.get("node:N-140")?.basisKey).toBe("load_case:load:L-100");
  });

  it("skips incomplete, non-finite, non-angle and unit-split triples and is unavailable without a complete one", async () => {
    const model = await loadPreviewModel();
    const partial = rotationRows("node:N-140", { x: 0.001, y: 0.002, z: 0.003 }).slice(0, 2);
    const nonFinite = rotationRows("node:N-130", { x: Number.NaN, y: 0, z: 0 });
    const notAngle = rotationRows("node:N-120", { x: 0.001, y: 0, z: 0 }).map((row) => ({ ...row, dimension: "length" as const }));
    const split = rotationRows("node:N-110", { x: 0.001, y: 0, z: 0 }).map((row, index) => (index === 2 ? { ...row, unit: "deg" } : row));
    const unknownNode = rotationRows("node:NOT-IN-MODEL", { x: 1, y: 0, z: 0 });
    const overlay = buildRotationOverlay(model, solved([...partial, ...nonFinite, ...notAngle, ...split, ...unknownNode]));
    expect(overlay.state).toBe("unavailable");
    expect(overlay.summary).toBe("unavailable; complete rotation component rows=0");
    expect(overlay.glyphs.size).toBe(0);
  });

  it("refuses to compare mixed rotation units across nodes", async () => {
    const model = await loadPreviewModel();
    const overlay = buildRotationOverlay(model, solved([
      ...rotationRows("node:N-140", { x: 0.001, y: 0, z: 0 }, "load:L-100", "rad"),
      ...rotationRows("node:N-130", { x: 1, y: 0, z: 0 }, "load:L-100", "deg")
    ]));
    expect(overlay.state).toBe("unavailable");
    expect(overlay.summary).toBe("unavailable; rotation units mixed (deg, rad); nodes=2");
    expect(overlay.glyphs.size).toBe(0);
  });

  it("counts zero rotations but draws no glyph for them", async () => {
    const model = await loadPreviewModel();
    const allZero = buildRotationOverlay(model, solved(rotationRows("node:N-140", { x: 0, y: 0, z: 0 })));
    expect(allZero).toMatchObject({ state: "available", nodeCount: 1, maximum: { value: 0, unit: "rad" } });
    expect(allZero.glyphs.size).toBe(0);
    const mixed = buildRotationOverlay(model, solved([
      ...rotationRows("node:N-140", { x: 0, y: 0, z: 0 }),
      ...rotationRows("node:N-130", { x: 0, y: 0.002, z: 0 })
    ]));
    expect([...mixed.glyphs.keys()]).toEqual(["node:N-130"]);
    expect(mixed.summary).toBe("available; nodes=2; drawn=1; max_component=0.002 rad");
  });

  it("does not change the translational deformed shape", async () => {
    const model = await loadPreviewModel();
    const rows = [magnitudeRow("node:N-140", 5), ...rotationRows("node:N-140", { x: 0.01, y: 0, z: 0 })];
    const withRotation = buildDeformationOverlay(model, solved(rows));
    const without = buildDeformationOverlay(model, solved(rows.slice(0, 1)));
    expect(withRotation).toEqual(without);
  });

  it("draws the bundled reference's emitted rotation rows", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    const rotationKinds = new Set(["global_nodal_rotation_x", "global_nodal_rotation_y", "global_nodal_rotation_z"]);
    const emitted = source.results.filter((row) => rotationKinds.has(row.kind));
    expect(emitted.length).toBeGreaterThan(0);
    const overlay = buildRotationOverlay(model, source);
    expect(overlay.state).toBe("available");
    for (const [nodeId, glyph] of overlay.glyphs) {
      // Each drawn component is an emitted row for that node and basis.
      for (const axis of ["x", "y", "z"] as const) {
        expect(emitted.some((row) => row.entity_ref === nodeId && row.kind === `global_nodal_rotation_${axis}` &&
          row.value === glyph.components[axis] && `${row.basis_ref?.ref_type}:${row.basis_ref?.ref_id}` === glyph.basisKey)).toBe(true);
      }
      expect(glyph.displayLength).toBeGreaterThan(0);
      expect(glyph.displayLength).toBeLessThanOrEqual(ROTATION_GLYPH_MAX_DISPLAY_LENGTH + 1e-12);
      expect(Math.hypot(glyph.axis.x, glyph.axis.y, glyph.axis.z)).toBeCloseTo(1, 12);
    }
  });
});
