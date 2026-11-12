import { describe, expect, it } from "vitest";
import { buildCreateComponentIntent, CENTER_OF_GRAVITY_CONVENTION, componentDraftForKind, defaultComponentDraft, isComponentDraftValid, keepComponentSources, rigidInterfacesValid, tieRodIdsDistinct, type ComponentDraft } from "./componentIntent";
import { componentDraftMissingUnitKeys } from "../project-units/missingProjectUnit";
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
    connectionEndAReference: "a", connectionEndBReference: "b", stiffnessBehaviorReference: "s",
    geometrySourceReference: "user geometry source", provenance: "user source", ...extra
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

  it.each(["valve", "flange", "reducer"] as const)("keeps each %s quantity's entered value and unit independent", (kind) => {
    const draft = valve({ kind, lengthUnit: "m", endASize: "270", endASizeUnit: "mm", endBSize: "10", endBSizeUnit: "in", centerOfGravityUnit: "cm" });
    expect(isComponentDraftValid(model, draft)).toBe(true);
    expect(payload(draft).geometry).toMatchObject({
      rigid_body_length: { value: 0.3, unit: "m" },
      end_a_size: { value: 270, unit: "mm" },
      end_b_size: { value: 10, unit: "in" },
      center_of_gravity: { x: 0.12, y: 0.02, z: 0, unit: "cm" }
    });
    for (const key of ["lengthUnit", "endASizeUnit", "endBSizeUnit", "centerOfGravityUnit"] as const) {
      expect(isComponentDraftValid(model, { ...draft, [key]: "TBD" })).toBe(false);
    }
  });

  it("keeps the missing-length notice until every rigid quantity has a chosen unit", () => {
    const missing = structuredClone(model);
    delete (missing.project.units as Record<string, string>).length;
    const draft = defaultComponentDraft(missing, null, [], "flange");
    expect([draft.lengthUnit, draft.endASizeUnit, draft.endBSizeUnit, draft.centerOfGravityUnit]).toEqual(["TBD", "TBD", "TBD", "TBD"]);
    const chosen = { ...draft, lengthUnit: "m", endASizeUnit: "mm", endBSizeUnit: "mm" };
    expect(componentDraftMissingUnitKeys(missing, chosen)).toContain("length");
    expect(componentDraftMissingUnitKeys(missing, { ...chosen, centerOfGravityUnit: "m" })).not.toContain("length");
  });
});

// PRD §5.5; DEL-07-03 AC-001: a component's sources are the user's own statements.
describe("component sources start empty and the next record keeps the user's own", () => {
  const model = {
    schema_version: "0.9.0", project: { id: "p", units: { length: "m", force: "N" } },
    nodes: [{ id: "n1", position: { x: 0, y: 0, z: 0 } }, { id: "n2", position: { x: 1, y: 0, z: 0 } }],
    pipe_segments: [{ id: "p1", from: "n1", to: "n2" }], components: [], supports: [], load_cases: []
  } as unknown as PreviewModel;
  const filled = (extra: Partial<ComponentDraft> = {}): ComponentDraft => ({
    ...defaultComponentDraft(model, { type: "node", id: "n2" }, [], "valve"),
    primaryPipeRef: "p1", rigidBodyLength: "0.3", endASize: "0.1", endBSize: "0.1", weight: "420",
    centerOfGravityX: "0", centerOfGravityY: "0", centerOfGravityZ: "0",
    connectionEndAReference: "a", connectionEndBReference: "b", stiffnessBehaviorReference: "s", ...extra
  });

  it.each(["bend", "tee", "valve", "flange", "reducer", "expansion_joint"] as const)("a new %s draft supplies no source", (kind) => {
    const draft = defaultComponentDraft(model, null, [], kind);
    expect([draft.geometrySourceReference, draft.provenance]).toEqual(["", ""]);
  });

  it("an untouched record is refused; one with the user's sources is not", () => {
    expect(isComponentDraftValid(model, filled())).toBe(false);
    expect(isComponentDraftValid(model, filled({ geometrySourceReference: "geometry A" }))).toBe(false);
    expect(isComponentDraftValid(model, filled({ geometrySourceReference: "geometry A", provenance: "source A" }))).toBe(true);
  });

  it("the next draft shows the user's last sources and nothing else from the previous record", () => {
    const previous = filled({ geometrySourceReference: "geometry A", provenance: "source A" });
    const fresh = defaultComponentDraft(model, { type: "node", id: "n2" }, [], "valve");
    expect(keepComponentSources(fresh, previous)).toEqual({ ...fresh, geometrySourceReference: "geometry A", provenance: "source A" });
  });

  it("leaving the expansion joint supplies no source", () => {
    const joint = defaultComponentDraft(model, { type: "node", id: "n2" }, [], "expansion_joint");
    const bend = componentDraftForKind(model, joint, "bend");
    expect([bend.geometrySourceReference, bend.provenance]).toEqual(["", ""]);
  });
});
