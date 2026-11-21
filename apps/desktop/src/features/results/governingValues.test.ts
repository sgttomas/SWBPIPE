// The governing values (DEL-07-05 OUT-001): each is the row of largest magnitude among the rows its
// table already derives, for all cases or the chosen case, with its entity, case and unit; nothing is
// formed. Invented data (`keyResultsFixtures`); no value has engineering meaning. The expected values
// are written beside the rows by hand.
import { describe, expect, it } from "vitest";
import { healthStanding, reaction, row } from "../../test-support/resultOverlayFixtures";
import { L1, L2, keyModel, keyResult, keyRows } from "../../test-support/keyResultsFixtures";
import { ALL_CASES } from "./resultsNavigator";
import { GOVERNING_NO_RESULT, governingValues, isAbsent, type GoverningDisplacement, type GoverningReaction, type GoverningStress } from "./governingValues";

const PASSING = healthStanding({ eligible: true });
const GLOBAL_NODE = { coordinate_system: "global", location: "node", basis: "solved_from_global_linear_system", sign_convention: "global axes" };
const rows = () => [
  ...keyRows(),
  // Reactions: Fx at S-A is 100 N (L1) and -300 N (L2); at S-C 200 N (L1).
  reaction("support:S-A", "Fx", 100, "N"),
  reaction("support:S-A", "Fx", -300, "N", { id: "r:S-A:Fx:L2", basis_ref: L2 }),
  reaction("support:S-C", "Fx", 200, "N", { id: "r:S-C:Fx:L1" }),
  // Translations: uy at B is -5 mm (L1); at C 4 mm (L2).
  row("u:B:y", "global_nodal_displacement_y", "node:B", -5, "mm", { metadata: { component: "uy", ...GLOBAL_NODE } }),
  row("u:C:y:L2", "global_nodal_displacement_y", "node:C", 4, "mm", { basis_ref: L2, metadata: { component: "uy", ...GLOBAL_NODE } })
];
const key = (basis: { ref_type: string; ref_id: string }) => `${basis.ref_type}:${basis.ref_id}`;

describe("governing values", () => {
  it("selects the largest magnitude over every case, naming its entity, case, component and unit", () => {
    const values = governingValues({ model: keyModel(), result: keyResult(rows()), standing: PASSING, caseKey: ALL_CASES });
    const displacement = values.displacement as GoverningDisplacement;
    expect(isAbsent(displacement)).toBe(false);
    expect([displacement.nodeId, displacement.component, displacement.value, displacement.unit, displacement.caseKey]).toEqual(["node:B", "uy", -5, "mm", key(L1)]);
    const fx = values.reactions.find((item) => item.column === "Fx") as GoverningReaction;
    expect([fx.supportId, fx.value, fx.unit, fx.caseKey]).toEqual(["support:S-A", -300, "N", key(L2)]);
    const stresses = values.stresses as readonly GoverningStress[];
    // Stress rows 40 and 80 MPa (L1), 50 MPa (L2); the largest ranked one is 80 at P-2, or the range
    // envelope's where the table ranks it.
    expect(stresses.length).toBeGreaterThan(0);
    expect(Math.abs(stresses[0].value)).toBeGreaterThanOrEqual(80);
  });

  it("restricts every selection to the chosen case", () => {
    const values = governingValues({ model: keyModel(), result: keyResult(rows()), standing: PASSING, caseKey: key(L2) });
    const displacement = values.displacement as GoverningDisplacement;
    expect([displacement.nodeId, displacement.value, displacement.caseKey]).toEqual(["node:C", 4, key(L2)]);
    const fx = values.reactions.find((item) => item.column === "Fx") as GoverningReaction;
    expect([fx.supportId, fx.value]).toEqual(["support:S-A", -300]);
    const stresses = values.stresses as readonly GoverningStress[];
    expect(stresses.map((item) => [item.value, item.unit, item.caseKey])).toEqual([[50, "MPa", key(L2)]]);
    const l1 = governingValues({ model: keyModel(), result: keyResult(rows()), standing: PASSING, caseKey: key(L1) });
    expect((l1.reactions.find((item) => item.column === "Fx") as GoverningReaction).supportId).toBe("support:S-C");
    expect((l1.stresses as readonly GoverningStress[]).map((item) => [item.value, item.entityRef])).toEqual([[80, "pipe:P-2"]]);
  });

  it("states a quantity the result does not have in a few words, never as zero", () => {
    const values = governingValues({ model: keyModel(), result: keyResult(rows()), standing: PASSING, caseKey: ALL_CASES });
    expect(isAbsent(values.ratios) && values.ratios.absent).toBe("No rule pack");
    expect(isAbsent(values.reactions.find((item) => item.column === "Mz"))).toBe(true);
    const none = governingValues({ model: keyModel(), result: null, standing: PASSING, caseKey: ALL_CASES });
    expect(isAbsent(none.displacement) && none.displacement.absent).toBe(GOVERNING_NO_RESULT);
  });

  it("selects nothing the standing withholds", () => {
    const withheld = governingValues({ model: keyModel(), result: keyResult(rows()), standing: healthStanding({ eligible: false }), caseKey: ALL_CASES });
    expect(isAbsent(withheld.displacement)).toBe(true);
    expect(withheld.reactions.every((item) => isAbsent(item))).toBe(true);
  });
});
