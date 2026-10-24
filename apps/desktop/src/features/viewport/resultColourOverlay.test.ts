import { describe, expect, it } from "vitest";
import { entityKey } from "../workspace/selectionState";
import { healthStanding, overlayModel, overlayResult, row, standardRows } from "../../test-support/resultOverlayFixtures";
import { loadBundledMechanicsReference } from "../../services/previewService";
import {
  availableResultColourQuantities,
  buildResultColourOverlay,
  RESULT_COLOUR_HELD_REASON,
  RESULT_COLOUR_QUANTITIES,
  resultColourTable,
  resultColourTicks,
  resultOverlayCases
} from "./resultColourOverlay";
import { resultNoValueHex, resultScaleHex } from "./viewportPalette";

// Result colour (OD-5; design system 2.6 and 6.7): one scalar per member, the largest absolute
// value over its rows for one case, shown only where the numerical standing admits the row.
// All data are invented or the bundled invented reference.

const quantity = (id: string) => RESULT_COLOUR_QUANTITIES.find((item) => item.id === id)!;
const PASSING = healthStanding({ eligible: true });

function overlayFor(id: string, standing = PASSING, rows = standardRows(), caseIndex = 0) {
  const model = overlayModel();
  const result = overlayResult(rows);
  const overlayCase = resultOverlayCases(model, result)[caseIndex];
  return { model, overlay: buildResultColourOverlay({ model, result, standing, quantity: quantity(id), overlayCase }) };
}

describe("result colour: quantities and cases", () => {
  it("offers only the catalogued kinds the result carries for a member", () => {
    const model = overlayModel();
    expect(availableResultColourQuantities(model, overlayResult(standardRows())).map((item) => item.id))
      .toEqual(["displacement", "normal-stress-extreme", "intensified-bending"]);
    expect(availableResultColourQuantities(model, null)).toEqual([]);
  });

  it("lists load cases then combinations, labelled from the model, and no envelope-only case", () => {
    const model = overlayModel();
    const rows = [
      row("c", "element_local_axial_force", "pipe:P-1", 1, "N", { basis_ref: { ref_type: "combination", ref_id: "combination:C-1" } }),
      ...standardRows(),
      row("e", "element_local_axial_force", "pipe:P-1", 1, "N", {
        basis_ref: { ref_type: "combination", ref_id: "combination:ENV" },
        metadata: { component: "axial_force", coordinate_system: "element_local", location: "end_i", basis: "explicit_user_range_envelope", sign_convention: "invented" }
      })
    ];
    expect(resultOverlayCases(model, overlayResult(rows)).map((item) => [item.key, item.label])).toEqual([
      ["load_case:load:L-1", "Invented one (load:L-1)"],
      ["load_case:load:L-2", "Invented two (load:L-2)"],
      ["combination:combination:C-1", "combination:C-1"]
    ]);
  });

  it("offers the bundled invented reference's member quantities on both solver modes", async () => {
    for (const mode of ["sparse_interactive", "dense_scrutiny"] as const) {
      const { model, source } = await loadBundledMechanicsReference(mode);
      const ids = availableResultColourQuantities(model, source).map((item) => item.id);
      expect(ids, mode).toEqual(RESULT_COLOUR_QUANTITIES.map((item) => item.id));
      expect(resultOverlayCases(model, source).map((item) => item.basis.ref_id), mode).toContain("load:L-100");
    }
  });
});

describe("result colour: one scalar per member", () => {
  it("takes the largest absolute value over a member's rows of the case, never an envelope or another case", () => {
    const { overlay } = overlayFor("normal-stress-extreme");
    expect(overlay.state).toBe("available");
    expect(overlay.unit).toBe("MPa");
    expect([...overlay.members].map(([id, member]) => [id, member.value, member.rowId, member.position])).toEqual([
      ["pipe:P-1", 4, "s:P-1:i", 0.5],
      ["pipe:P-2", 8, "s:P-2:i", 1]
    ]);
    expect(overlay.minimum).toBe(4);
    expect(overlay.maximum).toBe(8);
    expect(overlay.memberCount).toBe(3);
    // P-3's only row is not finite: it has no value.
    expect(overlay.noValueCount).toBe(1);
    expect(resultColourTicks(overlay)).toEqual([0, 2, 4, 6, 8]);
  });

  it("gives a member the larger of its end nodes for a nodal quantity", () => {
    const { overlay } = overlayFor("displacement");
    expect([...overlay.members].map(([id, member]) => [id, member.value])).toEqual([["pipe:P-1", 2], ["pipe:P-2", 2], ["pipe:P-3", 1]]);
    expect(overlay.unit).toBe("mm");
    expect(overlay.noValueCount).toBe(0);
  });

  it("reaches a member through the emitted source rows of a component's intensified row", () => {
    const { overlay } = overlayFor("intensified-bending");
    expect([...overlay.members].map(([id, member]) => [id, member.value])).toEqual([["pipe:P-3", 7e6]]);
    expect(overlay.noValueCount).toBe(2);
  });

  it("selects the case: the second case colours from its own rows only", () => {
    const { overlay } = overlayFor("normal-stress-extreme", PASSING, standardRows(), 1);
    expect([...overlay.members].map(([id, member]) => [id, member.value])).toEqual([["pipe:P-2", 50]]);
  });

  it("does not compare rows of one quantity and case stated in different units", () => {
    const rows = [...standardRows(), row("s:P-3:Pa", "element_local_normal_stress_extreme", "pipe:P-3", 1e6, "Pa")];
    const { overlay } = overlayFor("normal-stress-extreme", PASSING, rows);
    expect(overlay.state).toBe("unavailable");
    expect(overlay.reason).toBe("Units mixed (MPa, Pa)");
    expect(overlay.members.size).toBe(0);
  });

  it("is unavailable without a solved result", () => {
    const model = overlayModel();
    const unsolved = { ...overlayResult(standardRows()), status: { mechanics: "MECHANICS_BLOCKED", rule_check: "not_run", professional_acceptance: "not_claimed" } };
    const overlay = buildResultColourOverlay({ model, result: unsolved, standing: PASSING, quantity: quantity("displacement"), overlayCase: resultOverlayCases(model, unsolved)[0] });
    expect(overlay.state).toBe("unavailable");
    expect(resultColourTable(model, overlay)).toBeNull();
  });
});

describe("result colour: numerical standing", () => {
  it("never colours a row the standing withholds: no standing at all", () => {
    const { model, overlay } = overlayFor("normal-stress-extreme", null as never);
    expect(overlay.state).toBe("held");
    expect(overlay.reason).toBe(RESULT_COLOUR_HELD_REASON);
    expect(overlay.members.size).toBe(0);
    expect(overlay.noValueCount).toBe(3);
    expect(overlay.heldRowCount).toBe(3);
    expect(resultColourTable(model, overlay)).toBeNull();
  });

  it("never colours when the reader's standing is not eligible, though the case passed", () => {
    const { overlay } = overlayFor("normal-stress-extreme", healthStanding({ eligible: false, states: { "load:L-1": "checks_passed", "load:L-2": "sensitive" } }));
    expect(overlay.state).toBe("held");
    expect(overlay.members.size).toBe(0);
  });

  it("never colours a row the results table badges other than checks passed", () => {
    // Eligible with a sensitive case cannot come from the generic reader; the row rule still holds alone.
    const { overlay } = overlayFor("normal-stress-extreme", healthStanding({ eligible: true, states: { "load:L-1": "sensitive", "load:L-2": "checks_passed" } }));
    expect(overlay.state).toBe("held");
    const second = overlayFor("normal-stress-extreme", healthStanding({ eligible: true, states: { "load:L-1": "sensitive", "load:L-2": "checks_passed" } }), standardRows(), 1);
    expect(second.overlay.state).toBe("available");
  });

  it("colours a pressure-source result whose recovery standing is eligible: its rows carry no badge", () => {
    const { overlay } = overlayFor("normal-stress-extreme", healthStanding({ eligible: true, route: "pressure_source", states: { "load:L-1": "sensitive" } }));
    expect(overlay.state).toBe("available");
    const held = overlayFor("normal-stress-extreme", healthStanding({ eligible: false, route: "pressure_source", states: { "load:L-1": "sensitive" } }));
    expect(held.overlay.state).toBe("held");
  });
});

describe("result colour: the drawn colours", () => {
  it("draws each member at its position on the scale and every other member in the no-value colour", () => {
    const { model, overlay } = overlayFor("normal-stress-extreme");
    const table = resultColourTable(model, overlay)!;
    expect(table.get(entityKey({ type: "pipe", id: "pipe:P-1" }))).toEqual({ light: resultScaleHex("light", 0.5), dark: resultScaleHex("dark", 0.5) });
    expect(table.get(entityKey({ type: "pipe", id: "pipe:P-2" }))).toEqual({ light: resultScaleHex("light", 1), dark: resultScaleHex("dark", 1) });
    expect(table.get(entityKey({ type: "pipe", id: "pipe:P-3" }))).toEqual({ light: resultNoValueHex("light"), dark: resultNoValueHex("dark") });
    expect(table.size).toBe(3);
  });
});
