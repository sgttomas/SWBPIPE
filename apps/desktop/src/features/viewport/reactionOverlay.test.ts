import { describe, expect, it } from "vitest";
import { healthStanding, overlayModel, overlayResult, reaction, row } from "../../test-support/resultOverlayFixtures";
import { loadBundledMechanicsReference } from "../../services/previewService";
import { resultOverlayCases } from "./resultColourOverlay";
import {
  buildReactionOverlay,
  REACTION_FORCE_MAX_DISPLAY_LENGTH,
  REACTION_HELD_REASON,
  REACTION_MOMENT_MAX_DISPLAY_LENGTH
} from "./reactionOverlay";

// Support reaction arrows (OD-5): emitted global support-on-pipe components only, sign kept,
// scaled to the largest drawn reaction of each kind; withheld attributions and rows in another
// frame draw nothing and are counted. All data are invented or the bundled invented reference.

const PASSING = healthStanding({ eligible: true });

function triple(support: string, prefix: "F" | "M", [x, y, z]: [number, number, number], unit: string) {
  return [reaction(support, `${prefix}x`, x, unit), reaction(support, `${prefix}y`, y, unit), reaction(support, `${prefix}z`, z, unit)];
}

function overlayOf(rows: ReturnType<typeof reaction>[], standing = PASSING, evidence?: Record<string, unknown>) {
  const model = overlayModel();
  const result = overlayResult([row("d:A", "displacement_magnitude", "node:A", 0, "mm"), ...rows], evidence);
  return buildReactionOverlay({ model, result, standing, overlayCase: resultOverlayCases(model, result)[0] });
}

describe("reaction arrows: frame, sign and scale", () => {
  it("draws the emitted global components with their sign, normalised to the largest force", () => {
    const overlay = overlayOf([
      ...triple("support:S-A", "F", [0, 0, -30], "N"),
      ...triple("support:S-C", "F", [30, 40, 0], "N"),
      reaction("support:S-C", "force_magnitude", 50, "N", { kind: "support_reaction_force_magnitude_v2" })
    ]);
    expect(overlay.state).toBe("available");
    const a = overlay.forces.get("support:S-A")!;
    expect(a.direction).toEqual({ x: 0, y: 0, z: -1 });
    expect(a.nodeId).toBe("node:A");
    expect(a.displayLength).toBeCloseTo((30 / 50) * REACTION_FORCE_MAX_DISPLAY_LENGTH, 12);
    expect(a.emittedMagnitude).toBeNull();
    const c = overlay.forces.get("support:S-C")!;
    expect(c.direction.x).toBeCloseTo(0.6, 12);
    expect(c.direction.y).toBeCloseTo(0.8, 12);
    expect(c.displayLength).toBeCloseTo(REACTION_FORCE_MAX_DISPLAY_LENGTH, 12);
    expect(c.emittedMagnitude).toBe(50);
    expect(overlay.maxForce).toEqual({ value: 50, unit: "N" });
    expect(overlay.drawnCount).toBe(2);
  });

  it("draws moments as their own kind, scaled to the largest moment", () => {
    const overlay = overlayOf([...triple("support:S-A", "M", [0, 2, 0], "N*m"), ...triple("support:S-C", "M", [-1, 0, 0], "N*m")]);
    expect(overlay.forces.size).toBe(0);
    expect(overlay.moments.get("support:S-A")!.displayLength).toBeCloseTo(REACTION_MOMENT_MAX_DISPLAY_LENGTH, 12);
    expect(overlay.moments.get("support:S-C")!.direction).toEqual({ x: -1, y: 0, z: 0 });
    expect(overlay.moments.get("support:S-C")!.displayLength).toBeCloseTo(REACTION_MOMENT_MAX_DISPLAY_LENGTH / 2, 12);
    expect(overlay.maxMoment).toEqual({ value: 2, unit: "N*m" });
  });

  it("draws nothing from rows that state another frame, or none, and counts their supports", () => {
    const local = triple("support:S-C", "F", [1, 0, 0], "N").map((item) => ({ ...item, metadata: { ...item.metadata!, coordinate_system: "support_local" } }));
    const overlay = overlayOf([...triple("support:S-A", "F", [0, 1, 0], "N"), ...local]);
    expect([...overlay.forces.keys()]).toEqual(["support:S-A"]);
    expect(overlay.frameNotDrawn).toEqual(["support:S-C"]);
  });

  it("draws no arrow from an incomplete triple, a zero vector or a range envelope", () => {
    const envelope = triple("support:S-D", "F", [5, 0, 0], "N").map((item) => ({ ...item, metadata: { ...item.metadata!, basis: "explicit_user_range_envelope" } }));
    const overlay = overlayOf([reaction("support:S-A", "Fx", 1, "N"), reaction("support:S-A", "Fy", 1, "N"), ...triple("support:S-C", "F", [0, 0, 0], "N"), ...envelope]);
    expect(overlay.forces.has("support:S-A")).toBe(false);
    expect(overlay.forces.get("support:S-C")!.displayLength).toBe(0);
    expect(overlay.forces.has("support:S-D")).toBe(false);
    expect(overlay.drawnCount).toBe(0);
    expect(overlay.withoutRows).toEqual(["support:S-D"]);
  });

  it("does not compare forces stated in different units", () => {
    const overlay = overlayOf([...triple("support:S-A", "F", [1, 0, 0], "N"), ...triple("support:S-C", "F", [1, 0, 0], "kN"), ...triple("support:S-C", "M", [1, 0, 0], "N*m")]);
    expect(overlay.forces.size).toBe(0);
    expect(overlay.moments.size).toBe(1);
    expect(overlay.reason).toBe("Force units mixed (N, kN)");
  });
});

describe("reaction arrows: withheld attribution and standing", () => {
  it("counts the producer's withheld attributions for the case and draws no arrow for them", () => {
    const evidence = { preview_cases: [{ load_case_id: "load:L-1", support_attribution: { attributed_support_ids: ["support:S-A"], withheld: [{ support_id: "support:S-C", reason: "CONSTANT_EFFORT_NOT_CONSUMED" }] } }] };
    const overlay = overlayOf(triple("support:S-A", "F", [0, 1, 0], "N"), PASSING, evidence);
    expect(overlay.withheld).toEqual([{ supportId: "support:S-C", reason: "CONSTANT_EFFORT_NOT_CONSUMED" }]);
    expect(overlay.forces.has("support:S-C")).toBe(false);
    expect(overlay.withoutRows).toEqual(["support:S-D"]);
  });

  it("draws nothing the numerical standing withholds", () => {
    const rows = triple("support:S-A", "F", [0, 1, 0], "N");
    for (const standing of [null, healthStanding({ eligible: false })]) {
      const overlay = overlayOf(rows, standing as never);
      expect(overlay.state).toBe("held");
      expect(overlay.reason).toBe(REACTION_HELD_REASON);
      expect(overlay.forces.size).toBe(0);
      expect(overlay.heldRowCount).toBe(3);
    }
  });

  it("reads the bundled invented reference: global rows, CE-120 withheld, the anchor's downward force kept", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    const overlayCase = resultOverlayCases(model, source).find((item) => item.basis.ref_id === "load:L-100")!;
    const overlay = buildReactionOverlay({ model, result: source, standing: PASSING_REFERENCE, overlayCase });
    expect(overlay.state).toBe("available");
    expect(overlay.withheld).toEqual([{ supportId: "support:CE-120", reason: "CONSTANT_EFFORT_NOT_CONSUMED" }]);
    const anchor = overlay.forces.get("support:S-100")!;
    const fz = source.results.find((item) => item.id === "result:support-action:10:load:L-100:13:support:S-100:Fz")!;
    expect(Math.sign(anchor.direction.z)).toBe(Math.sign(fz.value));
    expect(anchor.emittedMagnitude).toBe(source.results.find((item) => item.id === "result:support-action:10:load:L-100:13:support:S-100:force_magnitude")!.value);
    expect(overlay.frameNotDrawn).toEqual([]);
  });
});

const PASSING_REFERENCE = healthStanding({ eligible: true, states: { "load:L-100": "checks_passed", "load:L-200": "checks_passed" } });
