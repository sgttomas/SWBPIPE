import { describe, expect, it } from "vitest";
import type { MechanicsResult } from "../../types";
import { buildDeformationOverlay } from "./deformationOverlay";
import { ABSOLUTE_SUM_SIGN, SRSS_SIGN, combinationNotes, derivedRowNote, isOneSolvedState } from "./resultColourOverlay";
import { overlayModel, overlayResult, row } from "../../test-support/resultOverlayFixtures";

// RT-A4 review r1 P4 (DEL-05-02 absolute-sum-srss-combinations.md section 4): a combination row that is not one
// solved state is recognized by its sign convention and its basis_ref to a combination, not only by its basis
// token. All rows are invented display inputs; no engineering result is asserted.

type Row = MechanicsResult["results"][number];
const comb = (id: string) => ({ ref_type: "combination", ref_id: id }) as Row["basis_ref"];
const STRESS = { component: "axial_normal_stress", coordinate_system: "element_local", location: "midspan" };

describe("derived combination rows", () => {
  it("recognizes an exact-route SRSS stress row that keeps its operand rows' basis", () => {
    const stress = row("s", "element_local_axial_normal_stress", "pipe:P-1", 3, "Pa", { basis_ref: comb("comb:SRSS"),
      metadata: { ...STRESS, basis: "recovered_from_open_mechanics_stress_components; section_basis=nominal_section", sign_convention: SRSS_SIGN } });
    expect(derivedRowNote(stress)).toBe("SRSS, unsigned");
    expect(isOneSolvedState(stress)).toBe(false);
    const abs = { ...stress, metadata: { ...stress.metadata!, sign_convention: ABSOLUTE_SUM_SIGN } };
    expect(derivedRowNote(abs)).toBe("absolute sum, unsigned");
    // The same sign text on a load case's row states no combination: it is not taken as one.
    expect(derivedRowNote({ ...stress, basis_ref: { ref_type: "load_case", ref_id: "load:L-1" } })).toBeNull();
  });

  it("recognizes the derivation metadata of an unsigned combination's displacement magnitude", () => {
    const magnitude = row("m", "displacement_magnitude", "node:B", 2, "mm", { basis_ref: comb("comb:SRSS"),
      metadata: { component: "displacement_magnitude", coordinate_system: "global", location: "node", basis: "explicit_user_srss", sign_convention: SRSS_SIGN } });
    expect(derivedRowNote(magnitude)).toBe("SRSS, unsigned");
  });

  it("gives a combination row that states no derivation its combination's note", () => {
    // A flat range envelope's displacement magnitude carries no metadata (V-8); a sibling row states the envelope.
    const bare = row("m", "displacement_magnitude", "node:B", 9, "mm", { basis_ref: comb("comb:ENV") });
    const sibling = row("ux", "global_nodal_displacement_x", "node:B", 9, "mm", { basis_ref: comb("comb:ENV"),
      metadata: { component: "nodal_displacement_x", coordinate_system: "global", location: "node", basis: "explicit_user_range_envelope",
        sign_convention: "value is the max_abs mode-selected source result across explicit user range-envelope operands and keeps that source result sign convention" } });
    const rows = [bare, sibling];
    expect(derivedRowNote(bare)).toBeNull();
    expect(combinationNotes(rows).get("combination:comb:ENV")).toBe("range envelope");
    expect(isOneSolvedState(bare, rows)).toBe(false);
    expect(isOneSolvedState(row("c", "displacement_magnitude", "node:B", 1, "mm"), rows)).toBe(true);
  });

  it("never draws an SRSS displacement magnitude as the deformed shape of its case", () => {
    const model = overlayModel();
    const rows: Row[] = [
      row("m1", "displacement_magnitude", "node:B", 1, "mm"),
      row("m2", "displacement_magnitude", "node:B", 5, "mm", { basis_ref: comb("comb:SRSS"),
        metadata: { component: "displacement_magnitude", coordinate_system: "global", location: "node", basis: "explicit_user_srss", sign_convention: SRSS_SIGN } }),
      row("ux", "global_nodal_displacement_x", "node:B", 5, "mm", { basis_ref: comb("comb:SRSS"),
        metadata: { component: "nodal_displacement_x", coordinate_system: "global", location: "node", basis: "explicit_user_srss", sign_convention: SRSS_SIGN } })
    ];
    const overlay = buildDeformationOverlay(model, overlayResult(rows), { basisKey: "combination:comb:SRSS" });
    expect(overlay.state).not.toBe("available");
  });
});
