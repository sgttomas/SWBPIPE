import { describe, expect, it } from "vitest";
import { buildCreateComponentIntent, CENTER_OF_GRAVITY_CONVENTION, defaultComponentDraft, rigidInterfacesValid, tieRodIdsDistinct, type ComponentDraft } from "./componentIntent";
import type { PreviewModel } from "../../types";

describe("tie-rod ids (T4-U5b round 4)", () => {
  it("compares ids as the producer does: raw and encoded (':' as '-')", () => {
    expect(tieRodIdsDistinct(["rod:a", "rod:b"])).toBe(true);
    expect(tieRodIdsDistinct(["rod:a", "rod:a"])).toBe(false);
    expect(tieRodIdsDistinct(["rod:a", " rod:a "])).toBe(false);
    expect(tieRodIdsDistinct(["rod:a", "rod-a"])).toBe(false);
    expect(tieRodIdsDistinct(["a:b:c", "a-b:c"])).toBe(false);
  });
});


describe("rigid component scale, consumption and centre of gravity (W-15)", () => {
  const model = {
    schema_version: "0.9.0", project: { id: "p", units: { length: "m", force: "N" } },
    nodes: [{ id: "n1", position: { x: 0, y: 0, z: 0 } }, { id: "n2", position: { x: 1, y: 0, z: 0 } }],
    pipe_segments: [{ id: "p1", from: "n1", to: "n2" }], components: [], supports: [], load_cases: []
  } as unknown as PreviewModel;
  const valve = (extra: Partial<ComponentDraft> = {}): ComponentDraft => ({
    ...defaultComponentDraft(model, { type: "node", id: "n2" }, [], "valve"),
    primaryPipeRef: "p1", rigidBodyLength: "0.3", endASize: "0.1", endBSize: "0.1", weight: "420",
    centerOfGravityX: "0.12", centerOfGravityY: "0.02", centerOfGravityZ: "0",
    connectionEndAReference: "a", connectionEndBReference: "b", stiffnessBehaviorReference: "s", ...extra
  });
  const payload = (draft: ComponentDraft) => JSON.parse(buildCreateComponentIntent({ draft, model, sourceRef: "test", sourceRole: "gui_editor", unitValidation: "test" }).change.after);
  it("enters the scale with its source and the consumption mode explicitly", () => {
    const draft = valve({ rigidStiffnessScale: "15", rigidScaleSourceReference: "user scale", rigidSolverConsumption: "mechanics_geometry_only" });
    expect(rigidInterfacesValid(draft)).toBe(true);
    const created = payload(draft);
    expect(created.modifiers).toEqual({ stiffness_scaling_user_value: { value: 15, unit: "none" }, source_reference: "user scale" });
    expect(created.mechanics_interface).toEqual({ solver_consumption: "mechanics_geometry_only" });
    expect(created.geometry.center_of_gravity).toEqual({ x: 0.12, y: 0.02, z: 0, unit: "m" });
  });
  it("defaults nothing and refuses a scale without its source", () => {
    const plain = payload(valve());
    expect(plain.modifiers).toBeUndefined();
    expect(plain.mechanics_interface).toBeUndefined();
    expect(rigidInterfacesValid(valve({ rigidStiffnessScale: "15" }))).toBe(false);
    expect(rigidInterfacesValid(valve({ rigidScaleSourceReference: "user scale" }))).toBe(false);
    expect(rigidInterfacesValid(valve({ rigidStiffnessScale: "0", rigidScaleSourceReference: "user scale" }))).toBe(false);
    expect(rigidInterfacesValid(valve({ rigidSolverConsumption: "objective_connector" }))).toBe(false);
    expect(rigidInterfacesValid(valve({ rigidSolverConsumption: "not_solver_consumed" }))).toBe(true);
  });
  it("states the owner's centre-of-gravity convention", () => {
    expect(CENTER_OF_GRAVITY_CONVENTION).toContain("global X/Y/Z from the component's node");
  });
});
